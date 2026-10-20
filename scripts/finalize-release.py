import hashlib
from pathlib import Path
import sys
tag,directory=sys.argv[1:]
root=Path(directory)
required=[f'UPR-Terminal-{tag}-windows-x86_64-setup.exe',f'UPR-Terminal-{tag}-linux-x86_64.deb',f'UPR-Terminal-{tag}-linux-x86_64.AppImage','install.sh']
unexpected=[p.name for p in root.iterdir() if p.is_file() and p.name not in required+['RELEASE-NOTES.md','SHA256SUMS.txt']]
if unexpected: raise SystemExit('Unexpected release assets: '+', '.join(unexpected))
for name in required:
    if not (root/name).is_file() or not (root/name).stat().st_size: raise SystemExit('Missing release asset: '+name)
notes=f'''# UPR Terminal — {tag}

Wydanie oficjalne. Wersja aplikacji: 0.1.3.
To wydanie zawiera wyłącznie UPR Terminal; drugi produkt ma osobne repozytorium.

Windows: instalator NSIS. Linux x86-64: DEB i AppImage (WebKitGTK 4.1).
Zamknij aplikację wraz z procesem zasobnika przed aktualizacją.
Zachowano konfigurację, identyfikator instalacji i ścieżki danych.
Instalator Windows nie jest podpisany certyfikatem wydawcy.

Instalacja/aktualizacja: pobierz install.sh, następnie `sh install.sh`.
Opcja `--version {tag}` przypina wydanie; domyślnie najnowszy opublikowany tag,
także pre-release. Skrypt pobiera tylko pliki z SP5ME/UPR-Terminal i sprawdza SHA-256.

Po upublicznieniu repozytorium:
`curl -fsSL https://github.com/SP5ME/UPR-Terminal/releases/download/{tag}/install.sh | sh`

Polecenie wykonuje pobrany kod. Repozytorium prywatne wymaga autoryzowanego
pobrania skryptu i GITHUB_TOKEN z odczytem repozytorium; nie wpisuj tokenu do argumentów.
Testy CI nie zastępują testów rzeczywistego radia i aktualizacji użytkownika.
Audyt pochodzenia kodu opisany w docs/PUBLISHING.md pozostaje otwarty.
'''
(root/'RELEASE-NOTES.md').write_text(notes, encoding='utf-8', newline='\n')
with (root/'SHA256SUMS.txt').open('w', encoding='utf-8', newline='\n') as output:
    for path in sorted(root.iterdir()):
        if path.is_file() and path.name!='SHA256SUMS.txt':
            digest=hashlib.sha256()
            with path.open('rb') as stream:
                while chunk:=stream.read(1024*1024): digest.update(chunk)
            output.write(f'{digest.hexdigest()}  {path.name}\n')
print(f'Validated {len(required)} required release assets.')
