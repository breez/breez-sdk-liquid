#!/bin/bash
set -xe

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$SCRIPT_DIR/boltz"
./start.sh

cd "$SCRIPT_DIR"
docker compose down
docker compose up --remove-orphans -d

./swapproxy-db-tool.sh --migrate