# Artifacts

Status: queued (Codex to execute — Stage 1: registry + invariant proofs, no
storage-authority flip).

Expected deliverables (ADR-0022 Stage 1):

- `perfidx01-registry-and-invariants.md` — sorted-id, completeness (0 post-freeze
  unknowns on H2637 + ladder; observed symbol counts), and equality evidence.
- `perfidx01-bit-identity-evidence.md` — `anchor_mismatches = 0` if runtime touched.
- `perfidx01-gate-results.md`, `perfidx01-line-count-governance.md`.
- `perfidx01_disposition.md` + worker-handoff (naming Stage 2,
  `PERFIDX02-indexed-shadow-runtime-surface-001`).

The completeness proof (every runtime symbol pre-registered, fail-closed otherwise)
is the make-or-break gate for the whole migration.
