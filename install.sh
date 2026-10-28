#!/usr/bin/env sh
set -eu

if ! command -v cargo >/dev/null 2>&1; then
    printf '%s\n' "Error: Cargo not installed." >&2
    printf '%s\n' "Install Rust from https://rustup.rs/" >&2
    exit 1
fi

cargo install --path daemon
cargo install --path setup

GREEN='\033[0;32m'
BOLD='\033[1m'
RESET='\033[0m'

printf '\n'
printf '%b\n' "${GREEN}${BOLD}========================================"
printf '%b\n' "         Installation completed"
printf '%b\n' "========================================${RESET}"
printf '\n'
printf '%b\n' "  Last step: execute ${BOLD}floord_setup${RESET}"
printf '\n'

echo ""