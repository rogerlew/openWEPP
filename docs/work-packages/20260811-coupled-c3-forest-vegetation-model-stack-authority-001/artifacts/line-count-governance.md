# Line Count Governance

Status: `PASS`

Evidence mode: `Ran`

Ran `wc -l` on every changed Rust and package oracle source:

| File | Lines | Disposition |
|---|---:|---|
| `tests/integration/vegetation_boundary_authority_contract.rs` | 721 | `PASS`, below 2,000 warning threshold |
| `artifacts/reference_calculator.py` | 754 | `PASS`, below 2,000 warning threshold |

No changed source file reaches the warning or blocking threshold. The canonical
vegetation contract is 918 lines and the BGC contract 220 lines; documentation
is governed by markdown lint rather than the Rust-source threshold.
