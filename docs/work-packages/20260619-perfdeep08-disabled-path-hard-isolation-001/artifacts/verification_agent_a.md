# Verification Agent A

Status: complete.
Evidence mode: Static/Ran.

## Verification

| Check | Result | Evidence |
|---|---|---|
| Candidate timing captured | PASS | `691.93 s`, RSS `229444 KB`. |
| Threshold math checked | PASS | `691.93 > 676.67`; gate fails. |
| Rejected candidate reverted | PASS | `git diff --stat` showed no production Rust diff after revert. |
| Focused tests passed | PASS | Runner, orchestrator writeback, and kernel-contract focused tests passed. |
| Gate evidence non-deferral checked | PASS | Failed timing gate prevents READY-FOR-R2. |

Final docs lint and whitespace checks are recorded in `gate-results.md`.
