# Line-Count Governance

| File | Baseline lines | Final lines | Delta | Status |
|---|---:|---:|---:|---|
| `crates/openwepp-topology/src/lib.rs` | 898 | 1141 | +243 | PASS, below 2000 warning threshold |
| `tests/integration/topology_graph_validation_gate.rs` | not governed as production source | 445 | N/A | Test file, below concern threshold |

No `.rs` file reaches the 2000-line warning threshold or 3000-line blocking
threshold.
