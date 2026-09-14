# team-tmig — The Mark Intelligence Group

Content-addressed transformation engine with a stateless admit/route/attest control plane.

## Build
    cargo build --release

## Binaries
    tmig            — 4 roles, 5 methods, 3 operations, ledger
    athletic-swarm  — 34 modes, 9 verbs, 10K+ agents
    murmur          — hyper-fine dunbar swarm (10M–1B agents)
    jira-swarm      — tickets as live agents
    filelayer       — every file as a live agent
    plane           — parallel terminals in generative planes
    model-boundary  — local Apple Silicon model-catalog filter

## Run
    ./target/release/tmig init
    ./target/release/tmig audit
    ./target/release/tmig red-team
    ./target/release/athletic-swarm --mode adaptive --agents 10000 --duration 10
    ./target/release/murmur --mb 10 --ticks 10
    ./target/release/jira-swarm --tickets 1000 --ticks 10
    ./target/release/filelayer --scan . --ticks 10
    ./target/release/plane --terminals 1000 --ticks 100
    ./target/release/model-boundary

## Checkers
    ./bin/swarm-full.sh src/swarm/agent.rs
