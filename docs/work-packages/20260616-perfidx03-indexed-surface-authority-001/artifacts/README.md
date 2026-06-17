# Artifacts

Status: executed-hold 2026-06-17.

PERFIDX03 found and fixed reachable-registry gaps, then attempted the indexed
authority flip. The flip preserved logical outputs on the exercised cases but
regressed OFE5 wall-clock from about 27s to about 38s because the live path still
exported the sparse authority back into full `BTreeMap` surfaces at the kernel
seam. The production authority activation was disabled before disposition, and
the package is held.

Deliverables:
- `perfidx03-preflip-registry-coverage.md` - 0 post-freeze unknowns across a diverse
  management cohort (grazing/multi-cut/irrigation), the precondition to flipping.
- `perfidx03-bit-identity-evidence.md` - partial exercised-case identity evidence;
  full H2637 + ladder anchor was not run because speed failed first.
- `perfidx03-realized-speedup.md` - active flip regression and no-flip rollback
  timing.
- `perfidx03-gate-results.md`, `perfidx03-line-count-governance.md`.
- `perfidx03-review-a.md`, `perfidx03-review-b.md`.
- `perfidx03-verification-a.md`, `perfidx03-verification-b.md`.
- `perfidx03_disposition.md` + `perfidx03-worker-handoff.md`.
