# R6E Blocker Ledger

Evidence mode: Static + Ran.

Status: executed-held.

| ID | Output family / mechanism | Evidence | Authority | In envelope? | Action | Status |
|---|---|---|---|---:|---|---|
| R6E-B001 | R6D marker named broad parity-grade producer absence | R6D hold marker and R6E initial reproduction | Architecture ledger + R6D handoff | yes | Refine marker after reproducing fail-closed behavior. | SUPERSEDED by R6E-B003 |
| R6E-B002 | `00_runner_intake_and_lane_setup.rs` above 3000 lines | `wc -l` after split shows `2787` lines for `00_runner_intake_and_lane_setup.rs` and `376` lines for `04_direct_publication.rs` | `crates/AGENTS.md` | yes | Split direct-publication helpers into `04_direct_publication.rs`; preserve same-module `include!` visibility and focused behavior. | RESOLVED |
| R6E-B003 | Production direct-runtime input/state binding absent for direct publication producers | Initial CLI cutover failed with `HOLD-R6E-PRODUCTION-DIRECT-RUNTIME-INPUT-BINDING-ABSENT`; post-fix focused tests prove direct frame/executor/capture counters run and the old marker is absent | R6 publication operand ledger + R5 direct endpoint readiness + R6D retained-frame handoff | yes | Add `DirectPublicationDayInput`; bind parsed precipitation/effective temperature into direct day frames; build retained cutover execution through direct capture instead of hand-authored rows. | RESOLVED |
| R6E-B004 | PASS Arrow parity fixture missing in current CLI fixture | Static fixture scan finds only HBP/WAT/loss/plot/manifest outputs and no PASS Parquet target | R6 acceptance criteria | yes | Add or select a fixture that writes PASS Parquet before claiming PASS Arrow parity. | BLOCKED behind HBP process parity |
| R6E-B005 | HBP direct-process parity mismatch | Direct CLI cutover exits `1` with `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`; HBP byte lengths match (`1654`) but bytes differ; no outputs are written | R6 acceptance criteria and direct process contracts | yes, but not safely closable in R6E without process-family parity migration | Continue with contract-first direct process parity work before publication cutover can write HBP/WAT/PASS/loss/manifest. | HOLD |

## Hold Legitimacy

R6E-B003 was in-envelope and is resolved in this package. The remaining first
blocker is now HBP process parity: direct publication has typed input binding
and direct span execution, but current direct process formulas/operands are not
yet parity-grade for public HBP bytes. Closing that requires process-family
parity migration and contract-backed tests rather than wrapping compatibility
WB13 rows, runtime surfaces, or writeback payloads as direct authority.
