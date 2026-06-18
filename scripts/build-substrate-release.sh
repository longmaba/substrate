#!/usr/bin/env bash
set -euo pipefail

target=""
out_dir="dist"

usage() {
  cat <<'EOF'
Usage: scripts/build-substrate-release.sh [--target <cargo-target>] [--out-dir <path>]

Builds a release-mode Substrate binary and writes a platform-labeled artifact
plus a sibling .sha256 checksum file.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      target="${2:-}"
      if [[ -z "$target" ]]; then
        echo "error: --target requires a value" >&2
        exit 2
      fi
      shift 2
      ;;
    --out-dir)
      out_dir="${2:-}"
      if [[ -z "$out_dir" ]]; then
        echo "error: --out-dir requires a value" >&2
        exit 2
      fi
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

platform_label_for_target() {
  case "$1" in
    x86_64-unknown-linux-gnu) echo "linux-x64" ;;
    aarch64-unknown-linux-gnu) echo "linux-arm64" ;;
    x86_64-apple-darwin) echo "macos-x64" ;;
    aarch64-apple-darwin) echo "macos-arm64" ;;
    x86_64-pc-windows-msvc|x86_64-pc-windows-gnu) echo "windows-x64" ;;
    *)
      echo "error: unsupported cargo target: $1" >&2
      exit 2
      ;;
  esac
}

platform_label_for_host() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"
  case "$os:$arch" in
    Linux:x86_64|Linux:amd64) echo "linux-x64" ;;
    Linux:aarch64|Linux:arm64) echo "linux-arm64" ;;
    Darwin:x86_64|Darwin:amd64) echo "macos-x64" ;;
    Darwin:arm64|Darwin:aarch64) echo "macos-arm64" ;;
    *)
      echo "error: unsupported host platform: $os/$arch" >&2
      exit 2
      ;;
  esac
}

sha256_value() {
  local file="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
  elif command -v openssl >/dev/null 2>&1; then
    openssl dgst -sha256 -r "$file" | awk '{print $1}'
  else
    echo "error: no SHA-256 tool found; install sha256sum, shasum, or openssl" >&2
    exit 2
  fi
}

if [[ -n "$target" ]]; then
  platform_label="$(platform_label_for_target "$target")"
  cargo build --release --target "$target"
  binary_dir="target/$target/release"
else
  platform_label="$(platform_label_for_host)"
  cargo build --release
  binary_dir="target/release"
fi

binary_name="substrate"
artifact_name="substrate-$platform_label"
if [[ "$platform_label" == "windows-x64" ]]; then
  binary_name="substrate.exe"
  artifact_name="$artifact_name.exe"
fi

binary_path="$binary_dir/$binary_name"
if [[ ! -f "$binary_path" ]]; then
  echo "error: expected release binary not found: $binary_path" >&2
  exit 1
fi

mkdir -p "$out_dir"
artifact_path="$out_dir/$artifact_name"
cp "$binary_path" "$artifact_path"
chmod +x "$artifact_path" 2>/dev/null || true

hash="$(sha256_value "$artifact_path")"
printf '%s  %s\n' "$hash" "$artifact_name" > "$artifact_path.sha256"

echo "artifact: $artifact_path"
echo "checksum: $artifact_path.sha256"
