# Kontrola podziału — UPR Terminal, 2026-10-20

Środowisko lokalne: Windows x86-64, Rust i Node. Brak testu instalacji na produkcji.

- Silnik: `cargo test --offline --locked --tests` — 114 testów.
- JavaScript: pięć testów `tests/browser/*.cjs` i sprawdzenie składni.
- Pakowanie i pobieranie: testy `tests/packaging/*.py` (dane syntetyczne, brak sieci).
- Instalator: `tests/installers/bootstrap.sh` — detekcja, walidacja i izolacja produktu.
- Kontrola publikacji: inwentarz Git; test odrzucania prywatnych ścieżek.
- Desktop Windows: kompilacja i 6 testów ustawień/migracji.
- Izolowany WebView: inicjalizacja, zapis ustawień, brak opcji Station,
  pobranie pliku, zasobnik, pojedyncza instancja i zakończenie silnika.

Nie wykonano kompilacji/instalacji Linux, Alpine, Raspberry Pi OS ani Dockera
w tym środowisku Windows. Odpowiednie workflow pozostają do wykonania na GitHub
po osobnej zgodzie na push/wydanie. Nie testowano rzeczywistego radia ani pełnej
aktualizacji profilu operatora. Paczki testów pakowania nie są binarkami do dystrybucji.
Nie opublikowano wydania i nie zmieniono jego numeru.
