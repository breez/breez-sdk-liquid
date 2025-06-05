#!/bin/bash
set -xe

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$SCRIPT_DIR/boltz"
# Use boltz from https://github.com/BoltzExchange/boltz-backend/pull/959 while it isn't merged
export BOLTZ_BACKEND_IMAGE=danielgranhao/boltz:latest
./start.sh

cd "$SCRIPT_DIR"
docker compose down
docker compose up --remove-orphans -d

./swapproxy-db-tool.sh --migrate