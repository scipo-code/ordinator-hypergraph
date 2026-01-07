# Ordinator Hypergraph Scheduling Domain Model
This is a crate for the Hypergraph Domain Model that Ordinator that will use
for the storing and validation complex interactions in data for Ordinator.

The coordinator of the `ScheduleGraph` will be handled by sagas and orchestrators
in the code.


# Quick Start
This package handles dependencies using the `nix` package manager

To download the `nix` package manager on UNIX based systems
```bash
sh <(curl -L https://nixos.org/nix/install) --no-daemon
```

To download and install the required dependencies
```bash
cd ~/PATH/TO/PROJECT
nix develop
```

Run the large scale test that is referenced in the whitepaper
```bash
cargo test -- test_large_scale_hypergraph
```


# Contributing
Contribution guidelines and current missing features are found in
[CONTRIBUTING](/CONTRIBUTING.md)

