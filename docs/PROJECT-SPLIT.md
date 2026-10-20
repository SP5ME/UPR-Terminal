# Rozdzielenie projektów — 2026-10-20

Projekt: **UPR Terminal**, docelowe repozytorium **SP5ME/UPR-Terminal**.
Baza historii: `eb42f2e0056c56f2ec0560330186e18641a83609`.
Wersja wydania oficjalnego to `0.1.3`. Brak migracji schematu danych.
W repozytorium są wcześniejsze wydania testowe; stabilne `0.1.3` jest przygotowywane.
Repozytorium Station nie jest tworzone automatycznie.

Oba produkty mają pełne własne źródła protokołów AX.25/KISS, poczty,
YAPP, MHeard i testów. Nie korzystają z katalogu sąsiedniego projektu.
Poprawki wspólnej logiki trzeba odtąd świadomie przenosić między repozytoriami.
Poprzednia wspólna historia pozostaje w Git; stare pliki mogą być widoczne w dawnych commitach.

Terminal nie zawiera samodzielnego serwera, klientów Telegram/MQTT ani
narzędzi wdrażania Station. HTTP na losowym porcie loopback służy wyłącznie
komunikacji Tauri z silnikiem; stary publiczny adres bind jest ignorowany.
Zachowano identyfikator `org.ultimatepr.station`, nazwę binarki
`ultimatepr-station-desktop` i katalogi danych `%APPDATA%/UltimatePR Station`
oraz `$XDG_DATA_HOME/ultimatepr-station` (domyślnie ~/.local/share).
Starsze klucze usług w konfiguracji są czytane w celu zachowania zgodności;
nie uruchamiają klientów sieciowych, a ich sekretów nie usuwamy.

Weryfikacja lokalna nie oznacza testu każdej platformy docelowej ani radia.
Linux/AppImage/DEB, Alpine i Docker wymagają weryfikacji w właściwym środowisku;
workflow wydania buduje tylko produkt własnego repozytorium.
