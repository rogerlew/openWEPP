# R6G Independent Reconstruction

Status: executed-held.

Record independent WAT reconstruction that does not restate the direct producer
formula with the same operands.

| Field group | Reconstruction source | Writer source | Agreement evidence | Status |
|---|---|---|---|---|
| Identity/calendar | First direct publication day and simulation-year helper, independent of row writer payload | Direct WAT row builder | Reduced field set excludes `wepp_id` and `year`; marker reservation test rejects unrelated fields. Full non-trivial WAT id authority remains follow-up. | Pass-Hold |
| First-day ET `Es` | Direct ET compute input from private seed surface plus direct runtime ET output | Direct WAT row builder consumes direct publication row | First WAT row `Es=1.0115699107918512` for direct and compatibility. | Pass |
| First-day storage/profile | Direct layer state after ET, residual theta contribution, and profile inputs from parsed layer/profile symbols | Direct hydrology projection and WAT row builder | First WAT row `Total-Soil=103.76254155138196` and `SoilWaterTotal=103.76254155138196`; `r4pqz_projection_includes_residual_water_in_layer_storage` protects residual water. | Pass |
| Day-2 ET/storage | Recomputed direct day should use direct-carried day-1 layer state before PMET operand construction | Direct WAT row builder currently receives precomputed day-2 PMET inputs | Day-2 direct `Es=0.8341925321233935`, compatibility `Es=0.7677601843722608`; storage delta matches the ET delta. | Hold |

## Anti-Tautology Check

The focused evidence does not accept direct publication merely because the
writer repeats its own operands. It compares the direct output family against
the compatibility output family only after direct HBP/WAT artifacts are built,
and it separately asserts:

- HBP byte identity is green for the inherited near-zero runoff fixture.
- The first WAT row is equal after identity/profile/residual storage fixes.
- The remaining reduced WAT field list is exactly
  `Es`, `Total-Soil`, `SoilWaterTotal`.
- The R6G hold marker does not fire for unrelated field sets.
