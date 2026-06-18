#!/usr/bin/env bash
set -euo pipefail

repository="${SUBSTRATE_REPOSITORY:-longmaba/substrate}"
release_tag="${SUBSTRATE_RELEASE_TAG:-}"
base_url="${SUBSTRATE_BASE_URL:-}"
install_dir="scripts/bin"

usage() {
  cat <<'EOF'
Usage: scripts/install-substrate.sh [--repo <owner/repo>] [--tag <release-tag>] [--base-url <url-or-path>] [--install-dir <path>]

Installs the platform-specific Substrate binary into the current repository.
Environment overrides: SUBSTRATE_REPOSITORY, SUBSTRATE_RELEASE_TAG,
SUBSTRATE_BASE_URL.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo)
      repository="${2:-}"
      [[ -n "$repository" ]] || { echo "error: --repo requires a value" >&2; exit 2; }
      shift 2
      ;;
    --tag|--release-tag)
      release_tag="${2:-}"
      [[ -n "$release_tag" ]] || { echo "error: --tag requires a value" >&2; exit 2; }
      shift 2
      ;;
    --base-url)
      base_url="${2:-}"
      [[ -n "$base_url" ]] || { echo "error: --base-url requires a value" >&2; exit 2; }
      shift 2
      ;;
    --install-dir)
      install_dir="${2:-}"
      [[ -n "$install_dir" ]] || { echo "error: --install-dir requires a value" >&2; exit 2; }
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

detect_label() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"
  case "$os:$arch" in
    Linux:x86_64|Linux:amd64) echo "linux-x64" ;;
    Linux:aarch64|Linux:arm64) echo "linux-arm64" ;;
    Darwin:x86_64|Darwin:amd64) echo "macos-x64" ;;
    Darwin:arm64|Darwin:aarch64) echo "macos-arm64" ;;
    *)
      echo "error: unsupported platform: $os/$arch" >&2
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

download() {
  local source="$1"
  local destination="$2"

  if [[ -f "$source" ]]; then
    cp "$source" "$destination"
    return
  fi

  case "$source" in
    file://*)
      cp "${source#file://}" "$destination"
      ;;
    http://*|https://*)
      if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$source" -o "$destination"
      elif command -v wget >/dev/null 2>&1; then
        wget -qO "$destination" "$source"
      else
        echo "error: install requires curl or wget" >&2
        exit 2
      fi
      ;;
    *)
      echo "error: unsupported download source: $source" >&2
      exit 2
      ;;
  esac
}

asset="substrate-$(detect_label)"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

if [[ -n "$base_url" ]]; then
  base_url="${base_url%/}"
  asset_url="$base_url/$asset"
  checksum_url="$base_url/$asset.sha256"
elif [[ -n "$release_tag" ]]; then
  asset_url="https://github.com/$repository/releases/download/$release_tag/$asset"
  checksum_url="$asset_url.sha256"
else
  asset_url="https://github.com/$repository/releases/latest/download/$asset"
  checksum_url="$asset_url.sha256"
fi

download "$asset_url" "$tmp_dir/$asset"
download "$checksum_url" "$tmp_dir/$asset.sha256"

expected="$(awk '{print tolower($1)}' "$tmp_dir/$asset.sha256")"
actual="$(sha256_value "$tmp_dir/$asset" | tr '[:upper:]' '[:lower:]')"
if [[ "$expected" != "$actual" ]]; then
  echo "error: checksum mismatch for $asset" >&2
  echo "expected: $expected" >&2
  echo "actual:   $actual" >&2
  exit 1
fi

mkdir -p "$install_dir"
install_path="$install_dir/substrate"
cp "$tmp_dir/$asset" "$install_path"
chmod +x "$install_path"

echo "installed: $install_path"
case "$install_path" in
  /*) echo "run: $install_path <command>" ;;
  *) echo "run: ./$install_path <command>" ;;
esac
