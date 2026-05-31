# HPHYS0216 Disposition

Status: completed
Evidence mode: Static + Ran

## Decision
- `HOLD`

## Closure measure evaluation
1. `MEASURE-HP216-001` (required workspace gates pass and are recorded):
   **pass**.
2. `MEASURE-HP216-002` (canonical contract authority explicitly maps FC to
   layer aggregation lineage): **pass**.
3. `MEASURE-HP216-003` (WB13 implementation consumes authoritative layer
   symbols for FC with typed guards): **pass**.
4. `MEASURE-HP216-004` (39-hillslope rerun reduces `ProfileFCStore` fail count
   vs HPHYS0214 `27/39`): **fail** (`39/39`).

## Interpretation
- HPHYS0216 contract-first implementation is complete and authoritative FC
  publication lineage is now explicit and enforced.
- Comparator outcome regressed for `ProfileFCStore` in the 39-hillslope cohort
  (`27/39 -> 39/39`), so closure condition for this package is not met.
- Disposition remains `HOLD` and requires follow-on remediation before
  integrated hold-lift.

## Hold-lift posture
- `ProfileFCStore`: open (regressed in HPHYS0216 semantic lane).
- `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`: unchanged open families.
