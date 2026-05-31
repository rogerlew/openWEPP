# HPHYS0219 Contract Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Scope
- Canonical WB19 threshold-lineage authority amendments landed in:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`

## Contract deltas (Static)
1. WB19 threshold authority now requires `coca_####` (not `cpm_####`) as the
   coefficient family for `drfc` realization.
2. WB19 threshold lineage is explicit:
   `drfc_i = wb18_perc_fc_i + (1-coca_i)*dg_i`.
3. WB19 branch-table and symbol requirements were amended to remove `cpm` from
   threshold authority and codify `coca` as the required runtime symbol.
4. Missing/non-finite/domain-invalid `coca_####` remains typed hard-fail
   authority with no fallback behavior.

## Provenance anchors (Static)
- Baseline authority reference maintained:
  `/workdir/wepp-forest_260430_baseline/src/watbal.for`
  (`drfc(i) = fc(i) + ((1 - coca(i))*dg(i))` lineage).

## Validation tie-in (Ran)
- Contract amendments validated by targeted WB19 contract tests and full
  workspace gates (see `artifacts/gate-results.md`).
