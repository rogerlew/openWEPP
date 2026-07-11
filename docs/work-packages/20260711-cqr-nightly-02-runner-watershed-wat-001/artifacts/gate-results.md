# Gate Results

| Gate | Status | Evidence |
|---|---|---|
| Focused nextest | PASS | `10/10` target tests. |
| Focused production coverage | PASS | `96.502%` lines, `90.854%` regions; science tier. |
| Focused CRAP | PASS | `0` rows above `30`; max `25.625`. |
| Focused lib clippy | PASS | Exit `0`, warnings denied. |
| Delegated closure round 1 | FAIL/FIXED | Workspace clippy caught `similar_names`; local renamed, focused clippy/tests pass. |
| Delegated closure round 2 | SUPERSEDED | Interrupted after reviewers required deterministic fixtures and stronger closure evidence; not accepted as final evidence. |
| `cargo fmt --check` | PASS | Delegated r3 exit `0`; `1.95s`. |
| Workspace clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings`; delegated r3 exit `0`; `6.66s`. |
| Full workspace nextest | PASS | `cargo nextest run --workspace --profile full`; delegated r3 exit `0`; `1700/1700` passed, `4` slow, `3` skipped; `595.56s`; run `01436a0f-cba6-4633-a213-d99eeaeb5454`. |
| `cargo deny check` | PASS | Delegated r3 exit `0`; advisories, bans, licenses, and sources all OK; `0.86s`. |
| `git diff --check` | PASS | Exit `0` after final reconciliation. |
| Package/catalog docs lint | PASS | `markdown-doc lint --path` on the package (`21` files) and catalog (`1` file); exit `0`, zero errors/warnings. |

No authority-suite posture, external cohort fixture, or required-case binding
changed; anti-evasion gates are not applicable.

Delegated r3 raw logs, exit files, and timings are under
`/tmp/openwepp-cqr-20260711-t02-closure-r3/`. The closure worker edited no
repository file.
