"""Create a validated GitHub release, retrying transient API failures."""

import subprocess
import sys
import time
from pathlib import Path


TRANSIENT_ERRORS = (
    "http 502",
    "http 503",
    "http 504",
    "bad gateway",
    "service unavailable",
    "gateway timeout",
    "connection reset",
    "timed out",
    "temporary failure",
)
MAX_ATTEMPTS = 4


def run_gh(arguments: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["gh", *arguments],
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )


def release_exists(tag: str, repository: str) -> bool:
    result = run_gh(["release", "view", tag, "--repo", repository])
    if result.returncode == 0:
        return True
    if "http 404" in result.stdout.lower() or "release not found" in result.stdout.lower():
        return False
    raise RuntimeError(f"Nie można sprawdzić istniejącego wydania:\n{result.stdout.strip()}")


def main() -> int:
    if len(sys.argv) != 7:
        raise SystemExit(
            "Użycie: create-release.py TAG OWNER/REPO COMMIT PRERELEASE PUBLISH ASSET_DIRECTORY"
        )

    tag, repository, commit, prerelease, publish, asset_directory = sys.argv[1:]
    if prerelease.lower() not in {"true", "false"} or publish.lower() not in {"true", "false"}:
        raise SystemExit("PRERELEASE i PUBLISH muszą mieć wartość true albo false.")
    assets = Path(asset_directory)
    notes = assets / "RELEASE-NOTES.md"
    if not notes.is_file():
        raise SystemExit(f"Brak not wydania: {notes}")

    if release_exists(tag, repository):
        raise SystemExit(f"Wydanie {tag} już istnieje; odmawiam jego nadpisania.")

    prerelease_option = ["--prerelease"] if prerelease.lower() == "true" else []
    command = [
        "release", "create", tag,
        "--repo", repository,
        "--target", commit,
        "--draft",
        *prerelease_option,
        "--title", f"UPR Terminal {tag}",
        "--notes-file", str(notes),
        *[str(path) for path in sorted(assets.iterdir()) if path.is_file()],
    ]

    for attempt in range(1, MAX_ATTEMPTS + 1):
        result = run_gh(command)
        if result.returncode == 0:
            print(result.stdout.strip())
            break

        output = result.stdout.strip()
        transient = any(error in output.lower() for error in TRANSIENT_ERRORS)
        if not transient or attempt == MAX_ATTEMPTS:
            raise SystemExit(f"Tworzenie wydania nie powiodło się:\n{output}")

        # Do not repeat creation if GitHub accepted it but its response was lost.
        if release_exists(tag, repository):
            raise SystemExit(
                f"GitHub zgłosił błąd, ale wydanie {tag} już istnieje. "
                "Sprawdź je ręcznie przed ponowieniem."
            )
        delay = 2 ** attempt
        print(f"Chwilowy błąd GitHub API; ponawiam za {delay} s ({attempt}/{MAX_ATTEMPTS - 1}).")
        time.sleep(delay)
    else:
        raise SystemExit("Nie udało się utworzyć wydania.")

    if publish.lower() == "true":
        result = run_gh(["release", "edit", tag, "--repo", repository, "--draft=false"])
        if result.returncode != 0:
            raise SystemExit(f"Wydanie utworzono jako draft, ale publikacja nie powiodła się:\n{result.stdout.strip()}")
        print(result.stdout.strip())

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
