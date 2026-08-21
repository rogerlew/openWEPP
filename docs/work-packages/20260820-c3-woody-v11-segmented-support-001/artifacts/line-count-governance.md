# Line Count Governance

Status: `PASS / implementation checkpoint b052158d0`.

| File | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-land-surface-energy/src/support.rs` | 250 | below block threshold |
| `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs` | 3150 | pre-existing >3000; +44 additive lines |
| `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_tests.rs` | 1812 | below block threshold |
| `crates/openwepp-persisted-restart-v1/src/vegetation_v11_v3.rs` | 562 | below block threshold |
| `crates/openwepp-vegetation/src/v11.rs` | 2482 | below block threshold |

The package WARN threshold is 2,000 and block threshold 3,000. The only
over-threshold file predates this package at 3,106 lines; this change adds no
new file-level block and remains bounded to the actual default-off consumer.
