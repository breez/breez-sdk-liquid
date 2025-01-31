#!/bin/bash

run_in_container() {
    docker exec -it boltz-scripts bash -c "source /etc/profile.d/utils.sh && $(printf '%q ' "$@")"
}

alias bitcoin-cli-sim-client='run_in_container bitcoin-cli-sim-client'
alias bitcoin-cli-sim-server='run_in_container bitcoin-cli-sim-server'
alias elements-cli-sim-client='run_in_container elements-cli-sim-client'
alias elements-cli-sim-server='run_in_container elements-cli-sim-server'
alias boltzcli-sim='run_in_container boltzcli-sim'
alias mine-block='bitcoin-cli-sim-client -generate 1 && elements-cli-sim-client -generate 1'

lightning-cli-sim() {
    run_in_container lightning-cli-sim "$@"
}

lncli-sim() {
    run_in_container lncli-sim "$@"
}
