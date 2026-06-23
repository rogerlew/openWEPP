# Worker Handoff

Status: executed-held.

## Handoff

- R7D6 landed direct typed EROD13/EROD14/EROD15 producer authority and the
  direct WB16 peak-duration producer required by erosion publication. H2637
  direct production now exits `0` with `compatibility_edge_invocations = 0`.
- H2637 sediment publication parity is clean after removing the fabricated
  `erod14_lddend = 0.3` default; WAT is byte-identical.
- Current hold:
  `HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL`.
- Exact first action: execute
  `docs/work-packages/20260623-r7d7-direct-wb16-peak-publication-parity-001/`
  to adjudicate WB16 `peakro` publication authority. The package must decide,
  with `SC-HYDRAULICS-001`, `SC-SED-001`, and HBP/PASS serialization evidence,
  whether compatibility PASS/HBP `peakro = 0.0` is a missing-publication defect
  or whether direct publication needs a contract-authorized byte-identity
  serialization policy that does not discard typed WB16 state.
- Do not "fix" the residual by silently forcing direct WB16 peaks to zero.
  That would erase direct producer authority and violate the no-fallback/no-
  fabricated-publication posture.
- Secondary follow-up: split or otherwise disposition
  `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`, currently
  `3243` lines, before claiming full package/root closure under line-count
  governance.
