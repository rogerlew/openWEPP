# Gate Results

Status: executed-hold.
Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| R5E prerequisite or waiver | PASS | R5E complete at pushed commit `d8f6bbea`; package `20260621-r5e-full-ofe-day-endpoint-readiness-001/` records verdict `COMPLETE-R5E-FULL-OFE-DAY-ENDPOINT-READINESS`. |
| Publication ledger canonical promotion | PASS | `docs/architecture/array-native-runtime-specification.md` section `5.2.1 R6 Canonical Publication Operand Ledger` promotes the PERFDEEP06 ledger into canonical architecture authority. |
| Direct publication frame availability | BLOCKED | Static inspection found no production run-bound direct `PublicationFrame` carrying the promoted HBP/WAT/PASS/loss/manifest operands. Existing `DirectPublicationFrame` is a narrow skeleton frame; runner outputs still build from compatibility WB13 rows/runtime surfaces. |
| HBP byte identity | NOT RUN | Blocked by absent run-bound direct publication frame. |
| WAT Arrow/metadata parity | NOT RUN | Blocked by absent run-bound direct publication frame. |
| PASS Arrow/metadata parity | NOT RUN | Blocked by absent run-bound direct publication frame. |
| loss JSON parity | NOT RUN | Blocked by absent run-bound direct publication frame. |
| manifest parity | NOT RUN | Blocked by absent run-bound direct publication frame. |
| Anti-alias fixtures | NOT RUN | Blocked by absent run-bound direct publication frame. |
| Independent operand reconstruction | NOT RUN | Blocked by absent run-bound direct publication frame. |
| No-compatibility proof | BLOCKED | Static scan confirms current public output path still uses compatibility WB13 rows/runtime surfaces; no direct-publication path exists to prove. |
| Default-disabled H2637 gate | NOT RUN | No production Rust/output edit or R6 direct-publication candidate exists. |
| Endpoint/RSS evidence | NOT RUN | No R6 direct-publication candidate exists. |
| Full Rust gates | NOT RUN | No Rust edits occurred after the pre-implementation blocker. |
| Scoped markdown lint | PASS | `markdown-doc lint --path docs/architecture/array-native-runtime-specification.md --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/r5-burndown-execplan.md --path docs/work-packages/20260621-r6-direct-publication-cutover-001 --format json`: 28 files scanned, 0 errors, 0 warnings. |
| `git diff --check` | PASS | Ran successfully after final R6 artifact updates. |
| Dual review and verification | PASS | Local package review/verification artifacts accept the resumed HOLD as required by package gate. No delegated subagents were invoked. |

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` status blocks completion.
