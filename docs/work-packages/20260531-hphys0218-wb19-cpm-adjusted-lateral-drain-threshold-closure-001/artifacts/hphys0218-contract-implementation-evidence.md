# HPHYS0218 Contract Implementation Evidence

Status: completed
Evidence mode: Static + Ran

## Scope
- Canonical WB19 threshold-lineage authority amendments landed in:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`

## Contract deltas (Static)
1. Required WB19 runtime symbol family now explicitly includes `cpm_####`.
2. WB19 drain-threshold lineage is explicit:
   `drfc_i = wb18_perc_fc_i + (1-cpm_i)*dg_i`.
3. WB19 saturated-zone classification and lateral/drainage withdrawals are
   contract-authorized against `drfc_i` (not FC-only thresholds).
4. Missing/non-finite/domain-invalid `cpm_####` is explicit typed hard-fail
   authority with no fallback to FC-only behavior.

## Provenance anchors (Static)
- Baseline authority reference maintained:
  `/workdir/wepp-forest_260430_baseline/src/watbal.for`
  (`drfc` threshold lineage and layer withdrawal semantics).

## Validation tie-in (Ran)
- Contract amendments were validated by targeted WB19 contract tests and full
  workspace gates (see `artifacts/gate-results.md`).
