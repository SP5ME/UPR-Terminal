# UPR Terminal

Niezależna aplikacja packet radio (AX.25/KISS), wersja `0.1.3`.
Repozytorium: `SP5ME/UPR-Terminal`. Desktop Windows/Linux z interfejsem Tauri, zasobnikiem, autostartem, dymkami i dźwiękiem.

To nie jest APRS. Funkcje obejmują terminal radiowy, digipeater, MailBox,
YAPP, beacon, monitor i MHeard. Winlink i demo nie są częścią projektu.
Zanim uruchomisz radio, ustaw znak i właściwy interfejs TNC.

## Budowanie i testy

Wymagany Rust zgodny z manifestem i zależności zapisane w Cargo.lock.
Projekt jest samodzielny; nie pobiera źródeł z sąsiedniego folderu.

```powershell
python scripts/desktop-licenses.py
cargo test --locked --manifest-path desktop/Cargo.toml
PowerShell -ExecutionPolicy Bypass -File scripts/build-desktop.ps1
```

Linux: `bash scripts/build-desktop-linux.sh` (wymaga bibliotek GTK/WebKitGTK).


Testy protokołów: `cargo test --locked --tests`.
Testy JS: pliki `tests/browser/*.cjs` uruchamiane przez Node.
Kontrola publikacji: `python scripts/verify-publication.py`.
Testy źródłowe pozostają w repo; dane operatora i wyniki są ignorowane.

## Instalacja i wydania

[Instalacja Linux](docs/LINUX-INSTALL.md). Workflow wydania jest uruchamiany
ręcznie i weryfikuje komplet paczek tylko tego produktu przed publikacją.
Podział lokalny nie wykonał push ani release.

## Zgodność i bezpieczeństwo

[Opis podziału](docs/PROJECT-SPLIT.md), [publikacja](docs/PUBLISHING.md),
[materiały osób trzecich](docs/THIRD-PARTY.md), [SECURITY.md](SECURITY.md).
Zachowano stare identyfikatory instalacji i ścieżki danych. Wykonaj kopię
profilu przed ręcznym wdrożeniem; nie dołączaj sekretów do Git ani zgłoszeń.
Licencja własnego kodu: Apache-2.0; licencje zależności pozostają niezależne.
Audyt pochodzenia inspiracji nie jest uznany za zakończony.
