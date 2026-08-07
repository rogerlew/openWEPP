# Line-Count Governance

Status: `pass`.

Evidence mode: `Ran`.

Compared package admission commit `3e9a62c4a` with the terminal tree:

| Rust test path | Before | Terminal | Disposition |
| --- | ---: | ---: | --- |
| `tests/integration/snow_stage3_turbulent_operator_reconciliation_contract.rs` | 400 | 733 | pass |
| `tests/integration/snow_stage3_legacy_predecessor_bridge_contract.rs` | 247 | 247 | pass |
| `tests/integration/snow_surface_eb03_contract.rs` | 277 | 277 | pass |

No touched Rust file approaches the 2,000-line warning or 3,000-line closure
blocker. No exception, owner, or sunset is required.
