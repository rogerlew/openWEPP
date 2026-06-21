# R6E Candidate Ledger

Evidence mode: Static + Ran.

Status: executed-held.

| Candidate | Blocker | Evidence | Result | Rationale |
|---|---|---|---|---|
| Split direct-publication helpers from `00_runner_intake_and_lane_setup.rs` | R6E-B002 | `wc -l` after split; focused tests pass | RETAINED | Resolves touched hard-threshold line-count issue without changing module visibility. |
| Rename fail-closed marker to production direct-runtime input binding absence | R6E-B003 | Initial focused tests and CLI reproduction | RETAINED as intermediate | Correctly identified the next actionable blocker but was not a legitimate terminal hold. |
| Add typed direct publication day inputs and retained direct execution capture | R6E-B003 | Focused tests prove direct capture counters execute and old marker is absent | RETAINED | Resolves production direct-runtime input binding for parsed climate. |
| Populate direct publication rows from `SimulationOwnedWb13Row`, `HillslopeWritebackSurface`, `KernelWritebackPayload`, or compatibility runtime publication symbols | R6E-B003/R6E-B005 | Static authority review | REJECTED | Prohibited compatibility authority sources for direct publication cutover. |
| Treat HBP byte mismatch as output-writer-only defect | R6E-B005 | Direct CLI reaches HBP comparison with equal byte lengths but mismatched bytes | REJECTED | Remaining mismatch is direct process parity, not a writer-only plumbing issue. |
