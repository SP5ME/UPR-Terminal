#!/bin/sh
# UPR Terminal installer/update bootstrap, Debian/Ubuntu x86-64 only.
set -eu
fail() { printf 'ERROR: %s\n' "$*" >&2; exit 1; }
detect_platform() {
    release=${1:-/etc/os-release}; machine=${2:-$(uname -m)}
    os_id=$(awk -F= '$1=="ID" {gsub(/["\047]/,"",$2); print $2; exit}' "$release")
    os_like=$(awk -F= '$1=="ID_LIKE" {gsub(/["\047]/,"",$2); print $2; exit}' "$release")
    case "$os_id $os_like" in debian*|ubuntu*|*' debian'*|*' ubuntu'*) ;; *) fail 'Debian/Ubuntu derivatives required.' ;; esac
    case "$machine" in x86_64|amd64) ;; *) fail 'UPR Terminal requires x86-64.' ;; esac
    printf 'systemd x86_64\n'
}
download_package() {
    script_root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
    python3 "$script_root/scripts/download-release.py" "$WORK" "$VERSION" "$ARCH" "$MODE"
}
as_root() {
    command -v sudo >/dev/null 2>&1 || fail 'sudo required for package installation.'
    sudo "$@"
}
cleanup() {
    case "${WORK:-}" in /tmp/upr-terminal.*) rm -rf -- "$WORK" ;; esac
}
main() {
    VERSION=latest;LAUNCH=1;DRY_RUN=0;WORK='';MODE=terminal;ARCH=x86_64
    while [ "$#" -gt 0 ]; do
        case "$1" in
            --version) [ "$#" -ge 2 ] || fail 'Missing version';VERSION=$2;shift 2 ;;
            --no-launch) LAUNCH=0;shift ;;
            --dry-run) DRY_RUN=1;shift ;;
            --help) printf 'Usage: sh install.sh [--version TAG] [--no-launch] [--dry-run]\nInstall/update UPR Terminal only; defaults to newest published release.\n';return ;;
            *) fail "Unknown option: $1" ;;
        esac
    done
    case "$VERSION" in ''|[!A-Za-z0-9]*|*[!A-Za-z0-9._-]*) fail 'Invalid release tag.' ;; esac
    detect_platform
    [ "$DRY_RUN" = 0 ] || return 0
    [ "$(id -u)" -ne 0 ] || fail 'Run as your normal desktop user, not root.'
    command -v apt-get >/dev/null 2>&1 || fail 'apt-get required.'
    if ! command -v python3 >/dev/null 2>&1; then
        as_root apt-get update
        as_root apt-get install -y python3 ca-certificates
    fi
        python3 - <<'UPR_PROCESS' || fail 'Cannot continue: UPR Terminal must be stopped and process checks must succeed.'
import os
from pathlib import Path
for item in Path('/proc').iterdir():
    if not item.name.isdecimal():
        continue
    try:
        target = os.readlink(item/'exe')
        if target.endswith(' (deleted)'):
            target = target[:-len(' (deleted)')]
    except OSError:
        continue
    if Path(target).name == 'ultimatepr-station-desktop':
        raise SystemExit('Close UPR Terminal, including its tray process, before updating.')
UPR_PROCESS
    set +x
    GITHUB_TOKEN=${GITHUB_TOKEN:-${GH_TOKEN:-}};export GITHUB_TOKEN
    WORK=$(mktemp -d /tmp/upr-terminal.XXXXXX)
    trap cleanup EXIT;trap 'exit 1' HUP INT TERM
    PACKAGE=$(download_package)
    VERSION=$(cat "$WORK/release-tag.txt")
    unset GITHUB_TOKEN GH_TOKEN
    as_root apt-get install -y --reinstall "$PACKAGE"
    printf 'UPR Terminal installed/updated to %s; user configuration preserved.\n' "$VERSION"
    if [ "$LAUNCH" = 1 ] && { [ -n "${DISPLAY:-}" ] || [ -n "${WAYLAND_DISPLAY:-}" ]; }; then
        command -v ultimatepr-station-desktop >/dev/null 2>&1 || fail 'Installed executable not found.'
        nohup ultimatepr-station-desktop </dev/null >/dev/null 2>&1 &
    fi
}
if [ "${UPR_BOOTSTRAP_LIBRARY:-0}" != 1 ]; then main "$@";fi
