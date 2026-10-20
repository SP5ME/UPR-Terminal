# Instalacja i aktualizacja UPR Terminal na Linux

Pobierz `install.sh` z wydania **SP5ME/UPR-Terminal**. Skrypt z drugiego produktu
nie jest zamiennikiem. Uruchom: `sh install.sh`.
Domyślnie wybiera najnowsze opublikowane wydanie (także pre-release),
`--version TAG` przypina tag. Przed wykonaniem możesz przeczytać skrypt.

Obsługiwane: Debian/Ubuntu i pochodne x86-64, DEB. Uruchamiaj jako zwykły
użytkownik; sudo służy tylko do instalacji pakietu. Zamknij Terminal wraz
z procesem zasobnika przed aktualizacją. `--no-launch` wyłącza uruchomienie.
AppImage jest osobnym plikiem do pobrania, wymaga uprawnienia wykonywania
i bibliotek WebKitGTK 4.1; nie gwarantuje działania na każdej dystrybucji.

Po upublicznieniu repozytorium (zastąp TAG właściwym tagiem):

```sh
curl -fsSL https://github.com/SP5ME/UPR-Terminal/releases/download/TAG/install.sh | sh
```

To polecenie wykonuje pobrany kod. Prywatne repo wymaga zalogowanego GitHub CLI
lub uwierzytelnionego pobrania. Skrypt pobierania paczek odczytuje GITHUB_TOKEN
(lub GH_TOKEN) ze środowiska i sprawdza SHA-256; nie podawaj sekretu w argumentach.
Nowe repo Station nie zostało utworzone ani opublikowane w ramach podziału.
