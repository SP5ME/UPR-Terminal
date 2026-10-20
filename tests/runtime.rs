use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use ultimatepr::runtime::{self, RuntimeOptions};

struct Temporary(PathBuf);
impl Temporary {
    fn new() -> Self {
        // Windows timestamps can be equal for tests starting on different threads.
        static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("test-tmp")
            .join(format!(
            "station-runtime-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}
#[test]
fn temporary_directories_are_distinct() {
    let first = Temporary::new();
    let second = Temporary::new();
    assert_ne!(first.0, second.0);
}
impl Drop for Temporary {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn request(address: std::net::SocketAddr, path: &str, headers: &str) -> String {
    let mut stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {address}\r\n{headers}\r\n"
    )
    .unwrap();
    let mut text = String::new();
    stream.read_to_string(&mut text).unwrap();
    text
}
fn post_request(address: std::net::SocketAddr, path: &str, headers: &str) -> String {
    let mut stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    write!(
        stream,
        "POST {path} HTTP/1.1\r\nHost: {address}\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 0\r\n{headers}\r\n"
    )
    .unwrap();
    let mut text = String::new();
    stream.read_to_string(&mut text).unwrap();
    text
}

#[test]
fn desktop_runtime_is_authenticated_locked_and_releases_resources_on_stop() {
    let dir = Temporary::new();
    let options = RuntimeOptions {
        config: dir.0.join("station.conf"),
        web_config: dir.0.join("web.conf"),
        web_config_explicit: true,
        token_override: Some("a".repeat(64)),

    };
    let station = runtime::start(options.clone(), None).unwrap();
    assert_ne!(station.address.port(), 0);
    assert!(request(station.address, "/healthz", "").starts_with("HTTP/1.1 200"));
    assert!(request(station.address, "/api/state", "").starts_with("HTTP/1.1 401"));
    let auth = format!(
        "X-Station-Request: 1\r\nAuthorization: Bearer {}\r\n",
        "a".repeat(64)
    );
    let web_details = post_request(station.address, "/api/web/details", &auth);
    assert!(web_details.contains("\"runtime\":\"desktop\""));
    assert!(web_details.contains("\"webPanel\":false"));
    let snapshot = request(station.address, "/api/state", &auth);
    assert!(snapshot.starts_with("HTTP/1.1 200"));
    assert!(!snapshot.contains("\"demo\":"));
    assert!(
        request(
            station.address,
            "/api/state",
            &(auth + "Origin: https://untrusted.example\r\n")
        )
        .starts_with("HTTP/1.1 403")
    );
    assert!(runtime::start(options.clone(), None).is_err());
    station.request_stop();
    station.wait().unwrap();
    let restarted = runtime::start(options, None).unwrap();
    let address = restarted.address;
    drop(restarted); // Drop must stop and join rather than leave a radio worker orphaned.
    assert!(TcpStream::connect(address).is_err());
}

#[test]
fn desktop_runtime_reports_desktop_profile() {
    let dir = Temporary::new();
    // A copied server configuration must not expose the embedded desktop API.
    fs::write(dir.0.join("web.conf"), "bind=0.0.0.0:8080\nauth_enabled=true\nmqtt_enabled=true\nmqtt_broker=mqtt://127.0.0.1:1883\n").unwrap();
    let options = RuntimeOptions {
        config: dir.0.join("station.conf"),
        web_config: dir.0.join("web.conf"),
        web_config_explicit: true,
        token_override: Some("b".repeat(64)),

    };
    let station = runtime::start(options, None).unwrap();
    assert!(station.address.ip().is_loopback());
    assert_ne!(station.address.port(), 8080);
    let auth = format!(
        "X-Station-Request: 1\r\nAuthorization: Bearer {}\r\n",
        "b".repeat(64)
    );
    let web_details = post_request(station.address, "/api/web/details", &auth);
    assert!(web_details.contains("\"runtime\":\"desktop\""));
    assert!(web_details.contains("\"webPanel\":false"));
    let telegram = post_request(station.address, "/api/telegram/status", &auth);
    assert!(telegram.starts_with("HTTP/1.1 400"));
    assert!(telegram.contains("UPR Station"));
    station.request_stop();
    station.wait().unwrap();
}
