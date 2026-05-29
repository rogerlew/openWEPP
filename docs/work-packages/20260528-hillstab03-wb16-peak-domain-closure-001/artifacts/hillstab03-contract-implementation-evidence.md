# hillstab03-contract-implementation-evidence

Status: complete  
Evidence mode: Static

## Canonical Contract Amendments
- Updated `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  (WB16 section):
  - removed `timep` from WB16 closure-diagnostics required inputs,
  - aligned `tc` to baseline `appmth.for` lineage as a function of `vstar` for
    `vstar < 1`,
  - added explicit constant-excess branch for `vstar >= 1` with `tstar < 1`
    (`qpstar = 1`, branch id `4`),
  - revised WB16 domain posture (`m <= 0` invalid; `vstar > 1` no longer
    rejected by domain guard),
  - updated branch-selector test-vector obligations to include all four
    branch-authoritative selectors.
- Revision history entry added in `SC-WATBAL-001.md`:
  - date: `2026-05-28`
  - version: `39`
  - package: HILLSTAB03 WB16 amendment.

## Authority Notes
- Contract-first sequencing was preserved: canonical contract text amendments
  were made before production WB16 runtime edits.
- Canonical `SC-*` contracts remain the sole authority; package artifacts are
  evidence only.
