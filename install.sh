#!/bin/sh

set -eu

repository="Huruikagi/specbind"
version=""
install_dir="${HOME}/.local/bin"

usage() {
  cat <<'EOF'
Install the SpecBind Linux x64 binary from GitHub Releases.

Usage: install.sh [--version <VERSION>] [--install-dir <DIRECTORY>]

Without --version, the latest non-prerelease is installed. Prereleases require
an explicit version such as 1.0.0-rc.1. The script verifies SHA256SUMS and does
not modify PATH or shell profiles.
EOF
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --version)
      [ "$#" -ge 2 ] || { echo "--version requires a value" >&2; exit 2; }
      version=$2
      shift 2
      ;;
    --install-dir)
      [ "$#" -ge 2 ] || { echo "--install-dir requires a value" >&2; exit 2; }
      install_dir=$2
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

[ "$(uname -s)" = "Linux" ] || {
  echo "install.sh supports Linux only." >&2
  exit 1
}
[ "$(uname -m)" = "x86_64" ] || {
  echo "install.sh supports Linux x64 only." >&2
  exit 1
}
command -v curl >/dev/null 2>&1 || {
  echo "curl is required." >&2
  exit 1
}

if [ -z "$version" ]; then
  api_url="https://api.github.com/repos/${repository}/releases/latest"
  tag=$(curl --fail --silent --show-error --location \
    -H 'Accept: application/vnd.github+json' \
    -H 'X-GitHub-Api-Version: 2022-11-28' \
    "$api_url" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1)
  [ -n "$tag" ] || {
    echo "Could not resolve the latest stable SpecBind release." >&2
    exit 1
  }
else
  case "$version" in
    v*) tag=$version ;;
    *) tag="v$version" ;;
  esac
fi

if ! printf '%s\n' "$tag" | grep -Eq \
  '^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?$'; then
  echo "Unsupported release version: $tag" >&2
  exit 1
fi

archive="specbind-${tag}-x86_64-unknown-linux-gnu.tar.gz"
base_url="https://github.com/${repository}/releases/download/${tag}"
temporary_dir=$(mktemp -d)
trap 'rm -rf "$temporary_dir"' EXIT HUP INT TERM

curl --fail --silent --show-error --location \
  --output "$temporary_dir/$archive" "$base_url/$archive"
curl --fail --silent --show-error --location \
  --output "$temporary_dir/SHA256SUMS" "$base_url/SHA256SUMS"

expected=$(awk -v archive="$archive" '$2 == archive { print $1 }' \
  "$temporary_dir/SHA256SUMS")
[ -n "$expected" ] || {
  echo "SHA256SUMS has no entry for $archive." >&2
  exit 1
}

if command -v sha256sum >/dev/null 2>&1; then
  actual=$(sha256sum "$temporary_dir/$archive" | awk '{ print $1 }')
elif command -v shasum >/dev/null 2>&1; then
  actual=$(shasum -a 256 "$temporary_dir/$archive" | awk '{ print $1 }')
else
  echo "sha256sum or shasum is required." >&2
  exit 1
fi

[ "$actual" = "$expected" ] || {
  echo "Checksum verification failed for $archive." >&2
  exit 1
}

tar -C "$temporary_dir" -xzf "$temporary_dir/$archive" specbind
mkdir -p "$install_dir"
install -m 755 "$temporary_dir/specbind" "$install_dir/specbind"

actual_version=$("$install_dir/specbind" --version)
expected_version="specbind ${tag#v}"
[ "$actual_version" = "$expected_version" ] || {
  echo "Installed binary reports '$actual_version'; expected '$expected_version'." >&2
  exit 1
}

echo "Installed $actual_version to $install_dir/specbind"
case ":${PATH}:" in
  *:"$install_dir":*) ;;
  *)
    echo "Add it to this shell with:"
    echo "  export PATH=\"$install_dir:\$PATH\""
    ;;
esac
