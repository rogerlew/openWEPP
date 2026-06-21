# R6G Anti-Alias Fixtures

Status: executed-held.

| Fixture/test | Alias risk | Required distinction | Result |
|---|---|---|---|
| `r6g_cutover_candidate_hbp_identity_reduces_wat_to_pmet_day_state_carry_gap` | HBP identity hiding WAT mismatch | HBP must remain byte-identical while WAT residual is reduced separately | Pass: HBP identity green; WAT residual exactly `Es`, `Total-Soil`, `SoilWaterTotal`. |
| `r6g_cutover_candidate_hbp_identity_reduces_wat_to_pmet_day_state_carry_gap` | First-row ET/storage writer tautology | First WAT row must match compatibility after direct producer binding, not just self-consistency | Pass: first direct and compatibility WAT rows are equal. |
| `r6g_wat_hold_marker_is_reserved_for_exact_pmet_day_state_carry_fields` | Broad marker masking unrelated fields | Marker must fire only for `Es`, `Total-Soil`, `SoilWaterTotal` | Pass: unrelated fields and `Dp`/`P` do not qualify. |
| `r6_direct_publication_cutover_cli_flag_reaches_hbp_identity_then_fails_pmet_day_state_carry` | CLI could write partial outputs on cutover failure | Cutover must fail closed and name exact hold marker | Pass: CLI exits nonzero, names `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`, and keeps outputs absent. |
| `r4pqz_projection_includes_residual_water_in_layer_storage` | Storage aggregate could omit residual liquid water and still look internally consistent | Aggregate storage must include residual theta over unfrozen layer depth | Pass: direct projection storage, `total_soil_m`, and `soil_water_total_m` include residual water. |

## Remaining Anti-Alias Coverage

R6G anti-alias evidence is limited to the inherited current fixture. A follow-on
hold-lift package must add non-trivial multi-OFE/lane coverage before complete
R6 publication cutover can claim direct WAT id and lane-dimensional authority.
