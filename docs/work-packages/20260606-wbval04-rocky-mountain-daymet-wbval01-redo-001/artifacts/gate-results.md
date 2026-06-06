# Gate Results

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

| Gate | Result | Evidence |
|---|---|---|
| Climate precondition audit | pass | `climate-precondition-audit.md`: zero CLI radiation bound exceedances. |
| Hillslope inventory complete | pass | `run-manifest.md`: `22` single-OFE, `1` multi-OFE observed-only. |
| Release binary recorded | pass | `run-manifest.md`: SHA-256 and source commit recorded. |
| Single-OFE validation complete | pass | `run-manifest.md`: all `22` single-OFE runs attempted. |
| Closure ledger complete | pass | `single-ofe-closure-ledger.md`: `18` WAT emitters classified, `4` blockers recorded. |
| WBVAL01 comparison complete | pass | `wbval01-redo-comparison.md`. |
| Review findings dispositioned | pass | `review-disposition.md`: no undispositioned findings. |
| Verification complete | pass | `verification_agent_a.md`, `verification_agent_b.md`. |
| Final disposition truthful | pass | `disposition.md`: executed-hold with defect-shaped follow-ons. |
| Rust full workspace gates | skipped | No Rust, contract, or test edits were made; validation package used release build and batch run gates. |

Ran:

- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: passed.
- WBVAL04 validation batch: all `22` single-OFE hillslopes attempted.
