#!/usr/bin/env bash
set -euo pipefail

TARGET="aarch64-unknown-linux-gnu"
SSH_TARGET_DIR="~"
PACKAGE_NAME="dipstick"

ssh_host="${1:?}"

cargo zigbuild --release -p "$PACKAGE_NAME" --target "$TARGET"
ssh "${ssh_host}" "rm -rf $SSH_TARGET_DIR/$PACKAGE_NAME" &>/dev/null
scp "target/$TARGET/release/$PACKAGE_NAME" "${ssh_host}:~"
exec ssh -tt "${ssh_host}" "killall -9 $PACKAGE_NAME; $SSH_TARGET_DIR/$PACKAGE_NAME"
