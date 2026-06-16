# Artifacts

Status: complete 2026-06-16 (design + feasibility, no production change).

Deliverables for PERFARCH01 (indexed runtime-surface design):

- `indexed-runtime-surface-design.md` — the registry + indexed-store design.
- `feasibility-and-projected-speedup.md` — prototype-measured clone/lookup savings
  and the projected speedup (can we reach ≤10×? ≤5×?), or the honest floor.
- `staged-implementation-plan.md` — incremental stages, each gated on
  `anchor_mismatches = 0` (bit-identity) + determinism.
- `risk-register.md` — hazards and required gates.
- `prototypes/indexed_surface_microbench.rs` — standalone dense-vs-BTreeMap prototype.
- proposed `docs/decisions/0022-indexed-runtime-surface-representation.md` (draft).
- `perfarch01_disposition.md` + worker-handoff (naming Stage-1).
- `perfarch01-review.md`, `perfarch01-verification.md`, and
  `perfarch01-gate-results.md`.
