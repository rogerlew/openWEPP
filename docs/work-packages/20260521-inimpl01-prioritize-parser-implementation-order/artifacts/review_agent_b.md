# INIMPL01 Review Agent B

Evidence: `Static`

## Findings

### INIMPL-B-001 — Severity: High
- Issue: `SC-INFILE-PHOSPHORUS-001` is sequenced as a Wave-2 hillslope sidecar even though the contract binds the surface to watershed routing state and unresolved watershed-vs-hillslope applicability.
- Evidence:
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md:11`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md:31`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md:38`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-priority-matrix.csv:18`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/follow-on-parser-implementation-wp-queue.md:17`
  - `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:30`
  - `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:96`
  - `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:105`
  - `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:210`
- Why it matters: Current sequencing can force premature parser/runtime coupling assumptions before watershed core topology/routing contracts are implemented, which weakens dependency coherence and increases rework risk.
- Proposed disposition: `amend` — move phosphorus to watershed-sidecar sequencing (Wave 4), or explicitly split into `parse-only` Wave 2 plus `routing-integration` Wave 4 with formal dependency gates.

### INIMPL-B-002 — Severity: Medium
- Issue: The canonical ranked queue allows cross-wave ordering that conflicts with the stated wave-governance execution model and follow-on WP queue.
- Evidence:
  - `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md:21`
  - `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md:27`
  - `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md:29`
  - `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md:33`
  - `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md:45`
  - `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md:56`
  - `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md:65`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/follow-on-parser-implementation-wp-queue.md:14`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/follow-on-parser-implementation-wp-queue.md:21`
- Why it matters: This creates an execution-authority ambiguity (rank vs wave). Teams can rationally choose different orders and drift from dependency gates and risk controls.
- Proposed disposition: `amend` — make wave precedence authoritative and absolute, and constrain rank to intra-wave order only (or add explicit `execution_wave` + `intra_wave_rank` fields).

### INIMPL-B-003 — Severity: Medium
- Issue: `hold_gap_risk` scoring is not traceable to explicit contract gap IDs per surface, reducing reproducibility of prioritization decisions.
- Evidence:
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-prioritization-rubric.md:8`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-prioritization-rubric.md:41`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-priority-matrix.csv:1`
  - `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-priority-matrix.csv:2`
  - `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md:17`
- Why it matters: Without explicit gap-ID traceability, the risk component cannot be independently audited or updated when contract HOLD registers change.
- Proposed disposition: `amend` — add a `gap_trace`/`gap_ids` column in the matrix and cite the specific `*-GAP-*` entries used for each surface’s `hold_gap_risk` score.

## Final Recommendation

`HOLD` — sequencing and traceability amendments should be applied before this prioritization set is treated as implementation-authoritative.
