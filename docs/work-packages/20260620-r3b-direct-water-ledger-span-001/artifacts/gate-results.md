# R3B Gate Results

Status: complete.
Evidence mode: Static + Ran.

| Gate | Result | Evidence |
|---|---|---|
| R3B span contract selected before Rust edits | PASS | `r3b-span-contract.md`. |
| Implementation and focused tests | PASS | `implementation-test-evidence.md`. |
| No-compatibility proof | PASS | Forbidden-token scan no matches; `scheduler.rs` no diff; runtime counters. |
| Default-disabled H2637 median `<= 676.67 s` | PASS | `640.67 s <= 676.67 s`. |
| Protected output identity | PASS | HBP/loss/WAT/plot stable; PASS row/schema equivalence. |
| Full Rust closure gates | PASS | fmt, clippy, workspace test, deny all passed. |
| Markdown lint and diff hygiene | PASS | Scoped markdown lint and `git diff --check` passed. |
| Line-count governance | PASS | All touched Rust files below 2000 lines. |
| Review and verification | PASS | Review A/B and Verification A/B complete. |

No failing, blocked, or deferred gate remains.
