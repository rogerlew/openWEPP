# Gate Results

| Gate | Status | Evidence |
|---|---|---|
| Cover-first module coverage | PASS/ATTEMPT | `89.615%` lines, `90.937%` regions before decomposition. |
| Cover-first per-function floor | FAIL/LOCAL HOLD | `parse_single_i32` logical body/closure was `20/30` (`66.667%`) at decomposition time; repaired only later in the provisional attempt. |
| Focused nextest | PASS/ATTEMPT | Strongest provisional suite `32/32`. |
| Focused CRAP | PASS/ATTEMPT | Zero attempted target rows above `30`; maximum `15.000`. |
| Focused clippy | PASS/ATTEMPT | `cargo clippy -p openwepp-input-contract --lib -- -D warnings`, exit `0`. |
| Contract-obligation closure | FAIL/LOCAL HOLD | Extra rating row requires `CHN-E-006`; runtime emits `CHN-E-002`. |
| Rollback identity | PASS | Target/test hashes match scaffold `a7d07708`; scoped Git diff exit `0`. |
| Post-rollback focused nextest | PASS | Baseline suite `21/21`; run `a4c1b8be-b3e2-422c-af48-588fa7b1274f`. |
| `cargo fmt --check` | NOT RUN FINAL | Not required after implementation/test rollback; baseline unchanged. |
| Workspace clippy | NOT RUN FINAL | Not required after implementation/test rollback. |
| Full workspace nextest | NOT RUN FINAL | Not required after implementation/test rollback. |
| `cargo deny check` | NOT RUN FINAL | Not required after implementation/test rollback. |
| `git diff --check` | PASS | Exit `0` after hold reconciliation. |
| Package/catalog docs lint | PASS | `markdown-doc lint --path`: package `22` files and catalog `1` file; zero errors/warnings. |

Heavy closure gates are not closure evidence and were not launched after the
local hold decision. No external-authority suite posture, cohort fixture, or
required-case binding is landed; anti-evasion gates are not applicable.
