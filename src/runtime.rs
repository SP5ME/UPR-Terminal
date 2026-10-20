//! Embedded Terminal engine lifecycle. No standalone server entrypoint.
use crate::{http, station, storage, web_config};
use std::{
    env, fs,
    net::{SocketAddr, TcpListener},
    path::PathBuf,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

const INITIAL: &str = "# UltimatePR Station - configure callsign and TNC before radio operation\n# callsign=N0CALL-1\nmyname=\nlanguage=en\naccept_incoming=true\nax25_autonegotiation=true\nax25_modulo_128=true\npaclen=128\nt1_ms=5000\nt3_ms=180000\nretries=10\ndigipeat_enabled=false\nbeacon_enabled=false\nbeacon_seconds=600\nbeacon_to=BEACON\nwelcome_enabled=true\ngoodbye_enabled=true\nremote_commands=true\nmailbox_enabled=false\nyapp_enabled=false\nyapp_auto_receive=false\nyapp_max_files=1000\nyapp_auto_cleanup=false\n";

#[derive(Clone)]
pub struct RuntimeOptions {
    pub config: PathBuf,
    pub web_config: PathBuf,
    pub web_config_explicit: bool,
    /// Optional bearer token override for embedded API callers and isolated tests.
    pub token_override: Option<String>,
}

pub struct StationRuntime {
    pub address: SocketAddr,
    stop: Arc<AtomicBool>,
    coordinator: Option<thread::JoinHandle<Result<(), String>>>,
}
impl StationRuntime {
    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
    pub fn is_finished(&self) -> bool {
        self.coordinator.as_ref().is_none_or(|t| t.is_finished())
    }
    pub fn wait(mut self) -> Result<(), String> {
        self.coordinator
            .take()
            .unwrap()
            .join()
            .map_err(|_| "Station coordinator panicked".to_owned())?
    }
}
impl Drop for StationRuntime {
    fn drop(&mut self) {
        self.request_stop();
        if let Some(worker) = self.coordinator.take() {
            let _ = worker.join();
        }
    }
}
struct StopOnDrop(Arc<AtomicBool>);
impl Drop for StopOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Relaxed);
    }
}

pub fn start(
    options: RuntimeOptions,
    events: Option<mpsc::SyncSender<crate::station_events::StationEvent>>,
) -> Result<StationRuntime, String> {
    let RuntimeOptions {
        config: mut path,
        web_config: web_config_path,
        web_config_explicit,
        token_override,
    } = options;
    if let Some(parent) = web_config_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mut web_config = web_config::WebConfig::load(&web_config_path)?;
    if !web_config_path.exists() {
        storage::write_atomic(&web_config_path, web_config.encode().as_bytes())
            .map_err(|e| format!("Cannot create {}: {e}", web_config_path.display()))?;
    }
    // The HTTP transport is private to the native host. Legacy public bind
    // settings must never expose a Terminal profile to the network.
    let address: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let token = token_override.clone().unwrap_or_default();
    // Validate an optional private host/test bearer token.
    if !token.is_ascii()
        || token
            .bytes()
            .any(|b| b.is_ascii_control() || b.is_ascii_whitespace())
    {
        return Err("STATION_TOKEN must be printable ASCII without spaces".into());
    }
    let listener = TcpListener::bind(address).map_err(|e| format!("HTTP bind: {e}"))?;
    let address = listener.local_addr().map_err(|e| e.to_string())?;
    if !path.is_absolute() {
        path = env::current_dir().map_err(|e| e.to_string())?.join(path);
    }
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    let _lock = DataLock::acquire(path.parent().unwrap().join(".station.lock"))?;
    if !path.exists() {
        storage::write_atomic(&path, INITIAL.as_bytes()).map_err(|e| e.to_string())?;
    }
    let station_web_path = if web_config_explicit {
        web_config_path.clone()
    } else {
        path.with_file_name("web.conf")
    };
    let auth_path = station_web_path.with_file_name("operator.account");
    if !web_config.auth_configured {
        // Preserve the local profile migration; public server authentication
        // policy never controls the loopback-only native transport.
        web_config.auth_enabled = token_override.is_some();
        storage::write_atomic(&web_config_path, web_config.encode().as_bytes())
            .map_err(|e| format!("Cannot migrate {}: {e}", web_config_path.display()))?;
    }
    let auth = std::sync::Arc::new(std::sync::Mutex::new(crate::auth::Auth::load(auth_path)?));
    let mut station =
        station::Station::new_with_web_config(path.clone(), Some(station_web_path.clone()))?;
    let files_directory = station.files_directory();
    let stop = Arc::new(AtomicBool::new(false));

    let snapshot = Arc::new(RwLock::new(station.snapshot()));
    let (tx, rx) = mpsc::sync_channel::<http::Envelope>(8);
    let runtime_stop = stop.clone();
    let runtime_snapshot = snapshot.clone();
    station.set_events(events);
    station.notify_application_started();
    let runtime = thread::spawn(move || {
        let _stop_on_exit = StopOnDrop(runtime_stop.clone());
        let mut published = Instant::now();
        let mut shutdown = None;
        loop {
            if runtime_stop.load(Ordering::Relaxed) && shutdown.is_none() {
                station.shutdown();
                shutdown = Some(Instant::now());
            }
            for _ in 0..8 {
                match rx.try_recv() {
                    Ok((request, reply)) => {
                        let response = station.command(request);
                        let _ = reply.send(response);
                    }
                    Err(_) => break,
                }
            }
            station.tick();

            if published.elapsed() >= Duration::from_millis(500) {
                station.notify_mail();
                *runtime_snapshot.write().unwrap() = station.snapshot();
                published = Instant::now();
            }

            if let Some(start) = shutdown
                && (!station.active() || start.elapsed() > Duration::from_secs(12))
            {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    });
    let server_stop = stop.clone();
    let coordinator = thread::spawn(move || {
        let _lock = _lock;
        let stop = server_stop;
        let result = http::Server {
            address,
            token,
            auth_enabled: token_override.is_some(),
            web_config: web_config_path.clone(),
            snapshot,
            commands: tx,
            stop: stop.clone(),
            files: files_directory,
            auth,
        }
        .run(listener);
        stop.store(true, Ordering::Relaxed);
        let runtime_result = runtime.join();

        runtime_result.map_err(|_| "Station runtime panicked".to_owned())?;
        result.map_err(|e| e.to_string())
    });
    Ok(StationRuntime {
        address,
        stop,
        coordinator: Some(coordinator),
    })
}
struct DataLock {
    _file: fs::File,
}
impl DataLock {
    fn acquire(path: PathBuf) -> Result<Self, String> {
        let mut options = fs::OpenOptions::new();
        options.write(true).create(true).truncate(false);
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            options.share_mode(0);
        }
        let file = options.open(&path).map_err(|e| {
            format!(
                "Cannot lock configuration {} (another Station may be running): {e}",
                path.display()
            )
        })?;
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } < 0 {
                return Err("Another Station is using this configuration".into());
            }
        }
        Ok(Self { _file: file })
    }
}
