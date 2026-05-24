# Gate Results

Status: `completed`
Evidence mode: `Ran`

## Required Repository Gates
| Command | Result |
| --- | --- |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass (warning-only allowlist drift notices) |

## WB19-Targeted Verification
| Command | Result |
| --- | --- |
| `cargo test --test wb19_lateral_drainage_physics_kernel_contract` | Pass |
| WB11/WB12/WB14/WB15/WB16/WB17/IRRIG10/CLIM05/CLIM06 targeted bundle | Pass |

## Overall Gate Outcome
`PASS`
