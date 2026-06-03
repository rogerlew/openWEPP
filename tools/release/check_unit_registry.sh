#!/usr/bin/env bash
set -euo pipefail

cargo test --test sim_contract_boundary_unit_registry
cargo clippy --test sim_contract_boundary_unit_registry -- -D warnings
