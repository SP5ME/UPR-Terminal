#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
fixture=$(mktemp -d)
trap 'rm -rf -- "$fixture"' EXIT
${UPR_TEST_PYTHON:-python3} scripts/emit-installer.py --bootstrap "$fixture/install.sh"
sh -n "$fixture/install.sh"
export UPR_BOOTSTRAP_LIBRARY=1
source "$fixture/install.sh"
printf 'ID=ubuntu\n' > "$fixture/ubuntu"
[[ "$(detect_platform "$fixture/ubuntu" x86_64 64)" == 'systemd x86_64' ]]
detect_platform() { printf 'systemd x86_64\n'; }
main --help >/dev/null
main --dry-run >/dev/null
if (main --version '../bad' --dry-run >/dev/null 2>&1); then exit 1;fi
if (main --station --dry-run >/dev/null 2>&1); then exit 1;fi
if (main --docker --dry-run >/dev/null 2>&1); then exit 1;fi
printf 'ID=alpine\n' > "$fixture/alpine"
# Invoke the original detection from the generated source in a subshell.
if (source "$fixture/install.sh";detect_platform "$fixture/alpine" x86_64 64 >/dev/null 2>&1); then exit 1;fi
(
 id() { printf '0\n'; }
 if (main --no-launch >/dev/null 2>&1); then exit 1;fi
)
export UPR_TEST_LOG="$fixture/log"
download_package() { printf vresolved > "$WORK/release-tag.txt";printf fixture > "$WORK/package.deb";printf '%s\n' "$WORK/package.deb"; }
(
 id() { printf '1000\n'; }
 python3() { return 0; }
 apt-get() { return 0; }
 as_root() { printf '%s\n' "$@" > "$UPR_TEST_LOG"; }
 main --no-launch >/dev/null
)
grep -Fx -- '--reinstall' "$fixture/log" >/dev/null
echo 'PASS: standalone Terminal bootstrap, detection, validation and product isolation.'
