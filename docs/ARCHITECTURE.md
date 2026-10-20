# Architektura UPR Terminal

`src/` zawiera własny silnik AX.25/KISS, transport TCP/Serial, MailBox,
YAPP, MHeard, statystyki i trwałą konfigurację. `web/` zawiera własny interfejs
HTML/CSS/JS. `languages/` i `messages/` są zasobami tego projektu.
Nie ma wspólnego katalogu źródeł, submodułu ani zależności od drugiego produktu.

`desktop/` jest aplikacją Tauri, zależną od biblioteki `ultimatepr` przez
lokalny manifest `../Cargo.toml` wewnątrz tego samego projektu. Osobny lockfile
desktopu obejmuje GUI. Manifest główny nie udostępnia samodzielnej binarki.
Tauri startuje silnik i prywatny HTTP na losowym porcie 127.0.0.1.
Kod zasobnika/autostartu/notification host jest wyłącznie w tym projekcie.
Nie ma trybu --headless/--mode server ani klientów Telegram/MQTT.
Serwerowe klucze starej konfiguracji pozostają wyłącznie zgodnościowe.

Zapisy konfiguracji korzystają z istniejących parserów, walidacji i atomowego
zapisu z kopią. Ustawienia radiowe wymagające przebudowy nadal chronią aktywne
sesje; preferencje powiadomień nie przebudowują transportu.
Nie zmieniono schematów ani numeru wersji.

[Granice podziału i kompatybilność](PROJECT-SPLIT.md).
