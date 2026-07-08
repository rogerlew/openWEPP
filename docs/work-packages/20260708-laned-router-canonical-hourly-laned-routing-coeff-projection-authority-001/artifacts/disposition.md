# Disposition

Status: complete.

## Package Decision

`EXECUTED-HOLD-PROJECTION-AUTHORITY`.

Legacy cropland projection did not close because the audited baseline sources do
not provide all five static Lane D operands or a bounded deterministic mapping
to them.

## Findings

| Source | Severity | Finding | Disposition |
|---|---|---|---|
| Review A | Medium | SC rev-48 source anchor named only `frcfac.for` while the gap row cited four baseline files. | Accepted; anchor now covers `frcfac.for`, `param.for`, `bigout.for`, and `watbal_hourly.for`, with package artifact pointer. |
| Review A / Verification A | Medium/High | Closure artifacts said pending while package/catalog were marked executed. | Accepted; closure artifacts finalized after review and gates. |
| Review A | Low | Current-authority audit attributed conditional default activation to rev 47 instead of rev 46. | Accepted; corrected. |
| Review A | Low | Runtime comments/errors still say native `routing_coefficients` although rev 48 uses broader explicit-producer terminology. | Accepted as handoff; Rust edits out of scope, future implementation must update wording/tests if another explicit producer becomes visible. |
| Verification A | Medium | BEI strict result must not be recorded as a clean pass. | Accepted; gate results record non-strict `PASS-DEFERRED` and strict deferred-nonzero truthfully. |
| Comparator A | Medium | Fidelity envelope needed rule-development vs. acceptance/holdout separation. | Accepted; envelope now requires frozen rules and separate untouched acceptance/holdout cohort when exploratory data are used. |
| Comparator A | Low | Reference hierarchy needed sharper wording. | Accepted; envelope distinguishes source-authorized candidate/reference comparisons, legacy/off compatibility controls, and non-negotiable closure gates. |
| Comparator A | Low | Future bridge cohort coverage needed stratification. | Accepted; envelope now requires stratification across slope, cover/residue, random roughness, storm class, OFE count, and low-mass sediment years. |

## Accepted Actions Already Taken

- `SC-OFEROUTE-001` rev 48 records the projection hold/rejection.
- `plant-file.spec.md` now states the end-user no-inference rule in concrete
  legacy field names.
- Package artifacts record current authority, source audit, projection hold,
  canonical policy, and future fidelity envelope.
- Final gates and review/verification artifacts are recorded.
