# Artifacts

Status: queued (Codex to execute — design + feasibility, no production change).

Expected deliverables for PERFARCH01 (indexed runtime-surface design):

- `indexed-runtime-surface-design.md` — the registry + indexed-store design.
- `feasibility-and-projected-speedup.md` — prototype-measured clone/lookup savings
  and the projected speedup (can we reach ≤10×? ≤5×?), or the honest floor.
- `staged-implementation-plan.md` — incremental stages, each gated on
  `anchor_mismatches = 0` (bit-identity) + determinism.
- proposed `docs/decisions/00NN-indexed-runtime-surface-representation.md` (draft).
- `perfarch01_disposition.md` + worker-handoff (naming Stage-1).
