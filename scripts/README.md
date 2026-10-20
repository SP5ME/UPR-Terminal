# Narzędzia UPR Terminal

- `verify-publication.py`: skan plików przeznaczonych do Git, bez ignorowanych danych lokalnych.
- `download-release.py`: pobieranie wyłącznie z SP5ME/UPR-Terminal, kontrola SHA-256 i przekierowań.
- `emit-installer.py`: generowanie samodzielnego install.sh z lokalnego szablonu.
- `finalize-release.py`: sprawdzenie pełnego zestawu plików tego produktu i sum kontrolnych.
- `create-release.py`: utworzenie draftu albo wydania stable/pre-release przez GitHub CLI; ponawia chwilowe błędy API i odmawia nadpisania istniejącego wydania.
- `desktop-licenses.py`: zebranie not zależności z Cargo (nazwa historyczna).

Budowanie desktopu: `build-desktop.ps1` lub `build-desktop-linux.sh`.
`collect-desktop-release.py` zbiera Windows x86-64/i686, Linux x86-64 (DEB/AppImage)
i macOS universal DMG dla workflow wydania.

[Budowanie](../README.md), [instalacja Linux](../docs/LINUX-INSTALL.md).
