#!/usr/bin/env bash
set -euo pipefail

PORT="28589"

ssh_host="${1:?}"

exec ssh -tt "${ssh_host}" "cd /opt/dipstick && sudo lldb-server platform --server --listen '*:$PORT'"
