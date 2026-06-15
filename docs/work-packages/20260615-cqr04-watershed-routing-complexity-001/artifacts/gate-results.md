# Gate Results

Ran:

| Gate | Result |
| --- | --- |
| `cargo test --test ws11_channel_routing_physics_equivalence_contract` before | exit 0, 44 passed |
| `cargo test --test ws10_watershed_kernel_contract` before | exit 0, 5 passed |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path .../lcov_before.info` | exit 0 |
| `cargo crap --workspace --lcov .../lcov_before.info --min 0 --format json --output .../crap_before.json` | exit 0 |
| `cargo check -p openwepp-watershed-orchestrator` | exit 0 |
| `cargo test --test ws11_channel_routing_physics_equivalence_contract` after | exit 0, 44 passed |
| `cargo test --test ws10_watershed_kernel_contract` after | exit 0, 5 passed |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path .../lcov_after.info` | exit 0 |
| `cargo crap --workspace --lcov .../lcov_after.info --min 0 --format json --output .../crap_after.json` | exit 0 |
| `cargo fmt --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo test --workspace` | exit 0 |
| `cargo deny check` | exit 0 |

Non-gate note:

- `cargo llvm-cov report --workspace --json --summary-only ...` exited 1
  because `report` does not accept `--workspace`. The compact after coverage
  summary was regenerated from the saved LCOV artifact instead.
