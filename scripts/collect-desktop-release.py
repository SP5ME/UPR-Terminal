from pathlib import Path
import shutil
import sys
platform,tag,destination=sys.argv[1:]
root=Path(__file__).resolve().parents[1]
out=Path(destination);out.mkdir(parents=True,exist_ok=True)
formats={
    'windows-x86_64': [('nsis/*-setup.exe',f'UPR-Terminal-{tag}-windows-x86_64-setup.exe')],
    'windows-i686': [('nsis/*-setup.exe',f'UPR-Terminal-{tag}-windows-i686-setup.exe')],
    'linux-x86_64': [
        ('deb/*_amd64.deb',f'UPR-Terminal-{tag}-linux-x86_64.deb'),
        ('appimage/*.AppImage',f'UPR-Terminal-{tag}-linux-x86_64.AppImage'),
    ],
    'macos-universal': [('dmg/*.dmg',f'UPR-Terminal-{tag}-macos-universal.dmg')],
}
if platform not in formats: raise SystemExit('Unsupported release platform: '+platform)
for pattern,name in formats[platform]:
    files=list((root/'desktop/target/release/bundle').glob(pattern))
    if len(files)!=1: raise SystemExit('Expected exactly one fresh installer matching '+pattern)
    shutil.copy2(files[0],out/name)
