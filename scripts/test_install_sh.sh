#!/usr/bin/env bash
# Exercise the real installer against local archives without network access.
set -euo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/bin" "$work/assets" "$work/stage"
export FIXTURE_DIR="$work/assets"
export PATH="$work/bin:$PATH"

cat > "$work/bin/uname" <<'EOF'
#!/bin/sh
case "$1" in
  -s) echo "$TEST_OS" ;;
  -m) echo "$TEST_ARCH" ;;
  *) exit 1 ;;
esac
EOF
cat > "$work/bin/curl" <<'EOF'
#!/bin/sh
output=''
previous=''
for arg do
  if [ "$previous" = '--output' ]; then output=$arg; fi
  previous=$arg
done
echo "$arg" >> "$FIXTURE_DIR/downloads"
cp "$FIXTURE_DIR/${arg##*/}" "$output"
EOF
cat > "$work/stage/specbind" <<'EOF'
#!/bin/sh
echo 'specbind 0.0.0'
EOF
chmod +x "$work/bin/"* "$work/stage/specbind"

for target in x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu aarch64-apple-darwin; do
  tar -C "$work/stage" -czf "$work/assets/specbind-v0.0.0-$target.tar.gz" specbind
done
(
  cd "$work/assets"
  sha256sum --text specbind-*.tar.gz > SHA256SUMS
)

for combination in Linux:x86_64:x86_64-unknown-linux-gnu \
  Linux:aarch64:aarch64-unknown-linux-gnu Linux:arm64:aarch64-unknown-linux-gnu \
  Darwin:arm64:aarch64-apple-darwin Darwin:aarch64:aarch64-apple-darwin; do
  IFS=: read -r TEST_OS TEST_ARCH target <<< "$combination"
  export TEST_OS TEST_ARCH
  : > "$work/assets/downloads"
  destination="$work/installed-$TEST_OS-$TEST_ARCH"
  sh "$root/install.sh" --version 0.0.0 --install-dir "$destination"
  grep -Fx "https://github.com/Huruikagi/specbind/releases/download/v0.0.0/specbind-v0.0.0-$target.tar.gz" "$work/assets/downloads"
  test "$("$destination/specbind" --version)" = 'specbind 0.0.0'
done

for combination in Linux:armv7l Darwin:x86_64 FreeBSD:aarch64; do
  IFS=: read -r TEST_OS TEST_ARCH <<< "$combination"
  export TEST_OS TEST_ARCH
  : > "$work/assets/downloads"
  if sh "$root/install.sh" --version 0.0.0 --install-dir "$work/unsupported" > "$work/output" 2>&1; then
    echo "Unexpected installer success: $combination" >&2
    exit 1
  fi
  grep -F 'does not support' "$work/output"
  test ! -s "$work/assets/downloads"
  test ! -e "$work/unsupported"
done

export TEST_OS=Linux TEST_ARCH=aarch64
printf 'corrupt archive\n' >> "$work/assets/specbind-v0.0.0-aarch64-unknown-linux-gnu.tar.gz"
if sh "$root/install.sh" --version 0.0.0 --install-dir "$work/corrupt" > "$work/output" 2>&1; then
  echo 'Unexpected success with a corrupt archive' >&2
  exit 1
fi
grep -F 'Checksum verification failed' "$work/output"
test ! -e "$work/corrupt"
echo 'Installer regression checks passed.'
