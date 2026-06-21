# R6F Gate Results

Status: scaffolded.

Record commands exactly as run. A failed gate starts another iteration unless
`no-premature-stop-audit.md` proves a legitimate out-of-envelope hold.

## Focused Iteration Gates

| Date | Gate | Command | Result | Evidence |
|---|---|---|---|---|
| Pending | Reproduce cutover failure | Pending | Pending | `r6f-current-failure-reproduction.md` |
| Pending | HBP decode/diff | Pending | Pending | `r6f-hbp-byte-diff.md` |
| Pending | HBP parity | Pending | Pending | Pending |
| Pending | WAT parity | Pending | Pending | Pending |
| Pending | PASS parity | Pending | Pending | Pending |
| Pending | Loss parity | Pending | Pending | Pending |
| Pending | Manifest parity | Pending | Pending | Pending |
| Pending | No-compatibility scan | Pending | Pending | `r6f-no-compatibility-proof.md` |
| Pending | Anti-alias fixture | Pending | Pending | `r6f-anti-alias-fixtures.md` |
| Pending | Independent reconstruction | Pending | Pending | `r6f-independent-reconstruction.md` |

## Final Gates

| Date | Command | Result | Notes |
|---|---|---|---|
| Pending | `cargo fmt --check` | Pending |  |
| Pending | `cargo clippy --workspace --all-targets -- -D warnings` | Pending |  |
| Pending | `cargo test --workspace` | Pending |  |
| Pending | `cargo deny check` | Pending |  |
| Pending | `wctl doc-lint --path docs/work-packages` | Pending |  |
| Pending | `git diff --check` | Pending |  |
| Pending | Direct cutover fixture command | Pending | Must write public HBP/WAT/PASS/loss/manifest outputs. |

## Skipped Gates

Skipped gates are allowed only for a legitimate hold. Record why the gate could
not be reached and point to the hold audit.

| Gate | Reason skipped | Hold audit evidence |
|---|---|---|
|  |  |  |
