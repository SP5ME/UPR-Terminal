from pathlib import Path
import shutil
import sys
platform,tag,destination=sys.argv[1:]
root=Path(__file__).resolve().parents[1]
out=Path(destination);out.mkdir(parents=True,exist_ok=True)
formats={
    'windows-x86_64': ('x86_64-pc-windows-msvc', [('nsis/*-setup.exe',f'UPR-Terminal-{tag}-windows-x86_64-setup.exe')]),
    'windows-i686': ('i686-pc-windows-msvc', [('nsis/*-setup.exe',f'UPR-Terminal-{tag}-windows-i686-setup.exe')]),
    'linux-x86_64': ('x86_64-unknown-linux-gnu', [
        ('deb/*_amd64.deb',f'UPR-Terminal-{tag}-linux-x86_64.deb'),
        ('appimage/*.AppImage',f'UPR-Terminal-{tag}-linux-x86_64.AppImage'),
    ]),
    'macos-universal': ('universal-apple-darwin', [('dmg/*.dmg',f'UPR-Terminal-{tag}-macos-universal.dmg')]),
}
if platform not in formats: raise SystemExit('Unsupported release platform: '+platform)
target,assets=formats[platform]
for pattern,name in assets:
    files=list((root/'desktop/target'/target/'release/bundle').glob(pattern))
    if len(files)!=1: raise SystemExit('Expected exactly one fresh installer matching '+pattern)
    shutil.copy2(files[0],out/name)
