# Adversarial Execution Matrix

Ran: executable Rust/Python cases cover the declared matrix; workflow source
inspection remains supplemental rather than sole closure evidence.

| Case | Expected disposition | Evidence |
| --- | --- | --- |
| FAIL exit | receipt `FAIL`, exact artifact-derived partial inventory | affected terminal end-to-end and partial-JUnit executor/verifier tests |
| Failed prerequisite | dependent `BLOCKED`, null exit, exact unavailable item | affected terminal end-to-end test |
| Spawn failure | `BLOCKED/PROCESS_SPAWN_FAILED` | executor process matrix |
| Timeout | process-group kill and `BLOCKED/TIMEOUT` | executor process matrix |
| Signal termination | `FAIL`, null exit, exact termination signal | executable `SIGTERM` process case |
| Source mutation outside prior roots | mutating attempt `INVALID`; later independent node `BLOCKED` and never spawned | `.github/probe.yml` plus absent marker end-to-end case |
| Unknown executor | typed preflight rejection | executor unit case |
| Inline shell string | reject `-c`, combined `-lc`, long `--command`, option-value-prefixed, BusyBox/env wrapped (including shell-like assignments), `*sh`, and PowerShell forms before a real script path | executable preflight variants |
| Path/output escape | typed rejection | executor path unit and integration contract |
| Disallowed/missing environment | exact allowlist behavior | executor environment unit and integration contract |
| Inventory drift | fail-closed live re-enumeration | executable preflight rejection |
| Output collision | fail before spawn | executable preflight rejection |
| Malformed identity | fail before reconstruction/spawn | executable identity rejection |
| Zero-work dispatch | no process attempts and empty aggregate | executable zero-node dispatch case |
| Rollback | remove only shadow workflow; conservative workflow bytes unchanged | executable filesystem rollback case |
| Reconstruction symlink | reject workspace and Cargo-target child symlinks | executable filesystem escape case |
| Rootless artifact provider | reject verification before reconstruction | executable verifier case |
| Unknown coverage contribution | one global CRAP node and `ESCALATED_GLOBAL` | executable critical-plan case |

The verifier independently parsed JUnit bytes for nonpass nodes, rejected
inventory outside the plan, accepted a truthful FAIL/BLOCKED receipt, and
verified that unavailable items exactly partition planned minus executed work.
