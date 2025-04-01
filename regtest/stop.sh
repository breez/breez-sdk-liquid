#!/bin/bash
set -xe

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$SCRIPT_DIR"
docker compose down --volumes

cd "$SCRIPT_DIR/boltz"
./stop.sh