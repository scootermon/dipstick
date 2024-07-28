#!/usr/bin/env bash
set -euo pipefail

TARGET="aarch64-unknown-linux-gnu"
SSH_TARGET_DIR="/home/pi"
PACKAGE_NAME="dipstick"

ssh_host="${1:?}"

cargo zigbuild --release -p "$PACKAGE_NAME" --target "$TARGET"
ssh "${ssh_host}" "killall -wq -9 '$PACKAGE_NAME'" || true
scp "target/$TARGET/release/$PACKAGE_NAME" "${ssh_host}:$SSH_TARGET_DIR/$PACKAGE_NAME"
exec ssh -tt "${ssh_host}" "cd '$SSH_TARGET_DIR' && sudo -E '$SSH_TARGET_DIR/$PACKAGE_NAME'"
