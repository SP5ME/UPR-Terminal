# Testy UPR Terminal

`cargo test --locked --tests`: protokoły radiowe, trwałość danych, konfiguracja,
powiadomienia i cykl życia procesu. Lokalny HTTP/KISS używa tylko loopback.
Testy tworzą własne katalogi, nie wymagają rzeczywistego TNC.
`node tests/browser/NAZWA.cjs`: pomocniki MHeard, dźwięku, nazw i ustawień produktu.
`python tests/packaging/release-download.py`: pobieranie testowane bez sieci z atrapami.
`bash tests/installers/bootstrap.sh`: wygenerowany instalator bez rzeczywistej instalacji.
`cargo test --locked --manifest-path desktop/Cargo.toml`: ustawienia i migracja desktopu.
`python tests/packaging/desktop-release.py` oraz `installer-hooks.py`: formaty paczek i NSIS.
Ręczny `tests/manual/windows-desktop-smoke.ps1` uruchamia własny wyizolowany profil
z wyłączonym TNC. Wymaga zbudowanego desktopu i Node; nie uruchamiaj na profilu operatora.

Nie są to testy instalacji na każdej dystrybucji ani testy prawdziwego radia.
