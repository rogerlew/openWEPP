# Operand lineage and independent reconstruction

Status: preimplementation lineage complete
Evidence mode: Static

## Authority and scope

This artifact binds the current `totalwatsed3` producer before test expansion
or decomposition. Authority is:

- ADR-0019: openWEPP owns this native output surface; wepppyo3 interchange is
  a semantic reference only, not a runtime dependency.
- ADR-0020: `openwepp-cli-totalwatsed3` is a read-only, hillslope-output
  aggregation CLI, separate from watershed/channel routing.
- Current production code in `crates/openwepp-runner/src/totalwatsed3.rs` and
  output schema/value mapping in
  `crates/openwepp-watershed-output/src/writers.rs`.
- The accepted WSHED01 T-arc evidence, especially `totalwatsed3-cli-scope.md`
  and the supersession chain ending at T-B2-REDO2. PASS `runvol` is the
  authoritative runoff-volume input to this aggregator. It must not be
  reconstructed here from WAT `Q`, WAT `QOFE`, a publication area, or a
  balance residual.
- Unit lineage in
  `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`.

The current-scope acceptance surface is aggregation/output identity. It does
not authorize changing upstream runoff, water, sediment, soil, or element
physics.

## Keys, area, and accumulation order

| Surface | Exact key/order rule | Disposition |
| --- | --- | --- |
| PASS and WAT daily group | `DayKey = (year, julian, sim_day_index, month, day_of_month, water_year)`; `day` is a compatibility alias for `sim_day_index` | authoritative grouping and output row identity |
| PASS `wepp_id` | required, typed, and non-null, but deliberately not part of `DayKey`; every hillslope row for the same day is summed | authoritative validation/aggregation rule |
| WAT outlet selector | maximum available `ofe_id`/`OFE` separately for each `(DayKey, wepp_id)` | authoritative only for `latqcc` selection |
| Optional join | `(wepp_id, ofe_id, year, julian, month, day_of_month, water_year)` joined to the WAT area lookup; `sim_day_index` is not in this join key | current exact optional-join behavior; collision risk requiring a test |
| Output domain/order | WAT `DayKey` domain only; PASS-only days are omitted and a WAT day with no PASS match uses zero PASS values. `BTreeMap` iteration emits lexicographic `DayKey` order | authoritative current behavior |
| Floating order | configured paths in vector order, parquet batches in reader order, rows in batch order, and fields/classes in source array order; each accumulator uses sequential `+=` | protected behavior; decomposition must not sort, parallelize, regroup, or reassociate |

`Area` is the sum of every positive WAT row `Area` in the daily group. It is
the denominator for all published WAT aggregate depths and PASS-derived
`Runoff`. It is not an outlet-only area, one hillslope's publication area, a
PASS area, a maximum, or a distinct-area sum.

## Authoritative water operands

For a WAT depth column `x`, define the ordered daily numerator
`N_x = Σ_rows(x_mm * Area_m2)`. Except for `latqcc`, the sum includes every WAT
row. Published depth is `N_x / Area`; the writer's exact-volume column is
`N_x / 1000`. The implementation stores the intermediate in `mm*m^2` and does
not round.

| Published field(s) | Units | Source column/surface | Area/normalization and accumulation | Authority |
| --- | --- | --- | --- | --- |
| `Area` | `m^2` | WAT `Area` | ordered sum of every daily WAT row; every input must be finite and `>0` | authoritative denominator |
| `runvol`; `Runoff` | `m^3`; `mm` | PASS `runvol` | ordered daily PASS sum `V_r`; `Runoff = V_r / Area * 1000` | authoritative runoff/primary closure operand |
| `sbrunv` | `m^3` | PASS `sbrunv` | ordered daily PASS sum; finite and nonnegative | authoritative PASS publication; not substituted for `Lateral Flow` |
| `P`; `Precipitation` | `m^3`; `mm` | WAT `P` | `N_P/1000`; `N_P/Area` | authoritative primary closure input |
| `RM`; `Rain+Melt` | `m^3`; `mm` | WAT `RM` | `N_RM/1000`; `N_RM/Area` | alternate/diagnostic closure input, not primary precipitation |
| `Q`; internal `Q (mm)` | `m^3`; `mm` | WAT `Q` | `N_Q/1000`; `N_Q/Area` | diagnostic only; forbidden runoff alias |
| `Dp`; `Percolation` | `m^3`; `mm` | WAT `Dp` | `N_Dp/1000`; `N_Dp/Area` | authoritative closure outflow |
| `latqcc`; `Lateral Flow` | `m^3`; `mm` | WAT `latqcc` | numerator includes only the maximum OFE row for each `(day,wepp_id)`; denominator remains total daily WAT `Area` | authoritative closure outflow with outlet-only selector |
| `QOFE`; `QOFE (mm)` | `m^3`; `mm` | WAT `QOFE` | every WAT row contributes; `N_QOFE/1000`; `N_QOFE/Area` | diagnostic here; forbidden `runvol` alias in this aggregator |
| `Ep`; `Transpiration` | `m^3`; `mm` | WAT `Ep` | `N_Ep/1000`; `N_Ep/Area` | authoritative ET component |
| `Es`; `Es (mm)` | `m^3`; `mm` | WAT `Es` | `N_Es/1000`; `N_Es/Area` | authoritative ET component |
| `Er`; `Er (mm)` | `m^3`; `mm` | WAT `Er` | `N_Er/1000`; `N_Er/Area` | authoritative ET component |
| `Evaporation`; `ET` | `mm` | derived from normalized WAT components | `Evaporation = Es + Er`; `ET = Ep + Es + Er`, in that order | authoritative closure outflow; derived, not a new input |
| `Interception` | `mm` | optional WAT `Interception` | `N/Area`; absent or all-null column contributes exact zero; a mixed-null column fails closed | authoritative when present; explicit zero-default policy when absent |
| `Total-Soil Water` | `mm` | WAT `Total-Soil Water` or `Total-Soil` | `N/Area`; required finite values | authoritative basic-storage operand |
| `frozwt`; `Snow-Water` | `mm` | same-named WAT columns | each `N/Area`; required finite values | authoritative basic-storage operands |
| `SoilWaterTotal` | `mm` | optional WAT `SoilWaterTotal` | row-level fallback to that row's `Total-Soil Water` only when the whole optional column is absent/all-null; then `N/Area` | enriched-storage operand; fallback must remain explicit |
| `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore` | `mm` | same-named optional WAT columns | each `N/Area`; absent/all-null becomes zero; mixed null fails | diagnostic profile bounds, not closure terms |
| `InterceptionStorage` | `mm` | optional WAT `InterceptionStorage` | `N/Area`; absent/all-null becomes zero; mixed null fails | storage diagnostic; distinct from interception flux |
| `UpStrmQ`, `SubRIn`, `Tile`, `Irr` | `mm` | same-named WAT columns; `Tile`/`Irr` optional | each `N/Area`; optional absent/all-null becomes zero | diagnostic/coupling fields, excluded from primary closure identity |
| `Baseflow` | `mm` | no configured source in this CLI | inherited row-seed zero | diagnostic zero policy, not a closure operand |

Primary independent water closure is:

```text
residual_mm =
  Precipitation
  - (Runoff + Lateral Flow + ET + Percolation + Interception)
  - delta(Total-Soil Water + frozwt + Snow-Water)
```

The enriched diagnostic substitutes `SoilWaterTotal + Snow-Water` for basic
storage. `Rain+Melt`, WAT `Q`, `QOFE`, `sbrunv`, runon, tile, irrigation,
profile-capacity fields, and interception storage must not be silently inserted
into the primary identity.

## Authoritative sediment operands

| Published field | Units | Source column/surface | Area/normalization and accumulation | Authority |
| --- | --- | --- | --- | --- |
| `tdet` | `kg` | PASS `tdet` | ordered daily PASS sum; finite signed input is preserved | authoritative publication mass |
| `tdep` | `kg` | PASS `tdep` | ordered daily PASS sum; finite signed input is preserved | authoritative publication mass |
| `seddep_1..5` | `kg` | PASS `sedcon_1..5` (`kg/m^3`) and the same row's PASS `runvol` (`m^3`) | per input row and class compute `sedcon_i * runvol`, then ordered daily sum; both operands nonnegative | authoritative class delivery masses |
| `sed_del` | `kg` | accumulated `seddep_1..5` | fixed class-order sum 1 through 5 | authoritative total sediment delivery |
| `sed_vol_conc` | `m^3/m^3` | class masses, class densities, and PASS `runvol` | `Σ_i(seddep_i / rho_i) / V_r`, fixed class order; `rho=[2600,2650,1800,1600,2650] kg/m^3`; exact zero when `V_r<=0` | derived sediment diagnostic with authoritative operands |

`seddep_i` is not a sum of concentrations, an unweighted mean concentration,
`tdet`, `tdep`, or a WAT sediment surrogate. Reassociation to “aggregate
concentration times aggregate runoff” is prohibited because it can change both
meaning and floating order.

## Optional soil and element operands

| Published field | Units | Source column/surface | Area/normalization and accumulation | Authority |
| --- | --- | --- | --- | --- |
| `TSMF` | fraction | optional soil `TSMF` | exact optional key joins to WAT; ordered `Σ(TSMF * matched WAT Area) / Σ(matched WAT Area)` | authoritative optional soil diagnostic |
| `QRain` | `mm` | optional element `QRain` | exact optional key joins to WAT; ordered `Σ(QRain * matched WAT Area) / Σ(matched WAT Area)` | authoritative optional runoff-partition diagnostic |
| `QSnow` | `mm` | optional element `QSnow` | exact optional key joins to WAT; ordered `Σ(QSnow * matched WAT Area) / Σ(matched WAT Area)` | authoritative optional runoff-partition diagnostic |

No configured soil/element file, no matching key, a missing/all-null optional
element column, or zero matched area publishes zero. `TSMF` is required when a
soil file is configured. For optional element columns, mixed nulls fail closed;
an all-null column behaves as absent. The denominator is matched optional area,
not total daily WAT area. Unmatched optional rows are currently ignored; this
is a high-risk behavior that must be deliberately characterized before any
reader decomposition.

## Independent reconstruction oracle

The characterization test must reconstruct expected results from literal input
fixture cells without calling `totalwatsed3` helpers, row-seed accessors, or the
writer's depth-to-volume mapping. Use one day with these deliberately separated
rows:

- WAT rows `(wepp1,ofe1,Area=100)`, `(wepp1,ofe2,Area=300)`, and
  `(wepp2,ofe1,Area=600)`; total `Area=1000 m^2`. Every WAT field receives a
  distinct triple. `latqcc` uses `[100,2,3]` so the non-outlet sentinel `100`
  must be excluded while `2*300 + 3*600` is retained.
- PASS rows use `runvol=[7,13]`, `sbrunv=[1,2]`, `tdet=[17,19]`,
  `tdep=[23,29]`, and class concentrations `[1,2,3,4,5]` and
  `[2,3,5,7,11]`.
- Soil `TSMF=[0.1,0.5,0.9]`; element `QRain=[1,2,4]` and
  `QSnow=[8,5,1]`, joined to the three WAT rows.
- WAT `Q` and `QOFE` must be chosen so neither `Σ(Q*Area)/1000` nor
  `Σ(QOFE*Area)/1000` equals PASS `runvol=20 m^3`; use, for example,
  `Q=[101,103,107]` and `QOFE=[13,17,19]`.

The independent assertions are:

```text
Area = 100 + 300 + 600 = 1000 m^2
runvol = 7 + 13 = 20 m^3
Runoff = 20 / 1000 * 1000 = 20 mm
Q = (101*100 + 103*300 + 107*600) / 1000 = 105.2 m^3
QOFE = (13*100 + 17*300 + 19*600) / 1000 = 17.8 m^3
latqcc = (2*300 + 3*600) / 1000 = 2.4 m^3
Lateral Flow = (2*300 + 3*600) / 1000 = 2.4 mm
sbrunv = 3 m^3; tdet = 36 kg; tdep = 52 kg
seddep = [33,53,86,119,178] kg
sed_del = 469 kg
sed_vol_conc =
  (33/2600 + 53/2650 + 86/1800 + 119/1600 + 178/2650) / 20
TSMF = (0.1*100 + 0.5*300 + 0.9*600) / 1000 = 0.7
QRain = (1*100 + 2*300 + 4*600) / 1000 = 3.1 mm
QSnow = (8*100 + 5*300 + 1*600) / 1000 = 2.9 mm
```

For each remaining WAT field `x`, use a distinct non-proportional triple and
assert both `x_depth = Σ(x_i*A_i)/1000` and, where published, exact volume
`x_volume = Σ(x_i*A_i)/1000`. Here the first `/1000` in the depth expression is
division by total `Area=1000 m^2`; the volume expression's `/1000` is the
mm-to-m conversion. The equal numeric divisor is intentional but the test must
compute the two results from independently named formulas to avoid unit
aliasing. Add a second day with different areas so those denominators are not
numerically equal in the whole fixture.

The oracle must also independently compute daily storage deltas and the primary
closure residual from emitted columns, and must require a nonzero-at-noise
fixture/cohort result. Exact self-consistency between an output depth and its
own output volume is supplementary, not acceptance.

## Rejected alias candidates

| Accepted operand | Candidates the fixture must make unequal and reject |
| --- | --- |
| PASS `runvol` / `Runoff` | WAT `Q`; WAT `QOFE`; `Q*Area/1000`; `QOFE*Area/1000`; outlet-only WAT area; total publication area paired with one OFE; per-OFE runoff sum; a balance residual; output `Runoff*Area/1000` used as its own oracle |
| WAT `Area` | first/last/max row area; outlet-only area; one hillslope's area; unique-area sum; PASS/upstream publication area |
| ordinary WAT fields | arithmetic mean; raw depth sum; first/last row; adjacent column; outlet-only value; missing `/1000` or double `/1000`; division by optional matched area |
| `latqcc` | all-OFE sum; global maximum OFE rather than max per `wepp_id`; first/last OFE; `sbrunv`; `QOFE`; total-area numerator |
| `QOFE` | outlet-only collapse; `Q`; `runvol`; cloned aggregate value per OFE |
| storage/interception | `Total-Soil Water` substituted for present `SoilWaterTotal`; `InterceptionStorage` substituted for `Interception`; profile capacity inserted into storage closure |
| `seddep_i` / `sed_del` | `Σ sedcon_i`; unweighted mean concentration times total runoff; `tdet`; `tdep`; wrong class; common density; tonnes conversion |
| `sed_vol_conc` | mass concentration `sed_del/runvol`; one 2650 density for all classes; `tdet/runvol`; division by WAT `Q` volume; NaN/inf on zero runoff |
| `TSMF`, `QRain`, `QSnow` | unweighted mean; total-WAT-area denominator under partial coverage; soil/element row count; wrong WAT OFE area; join by day only; ignoring `wepp_id` or OFE; `TSMF` from WAT soil-water columns; QRain/QSnow swap; null-as-zero with area retained |

## Proposed exact A-H bindings and risks

These stable test names are required before cover-first closure. Existing tests
remain useful but do not satisfy the entire family alone.

| Family | Exact current/planned binding | Principal risk to close |
| --- | --- | --- |
| A nominal | existing `totalwatsed3_cli_uses_pass_runvol_and_outlet_lateral_flow`, `totalwatsed3_cli_reads_openwepp_per_hillslope_pass_and_wat_surfaces`; planned `totalwatsed3_independent_operand_oracle_covers_all_published_fields` and `totalwatsed3_optional_soil_element_oracle_uses_wat_area` | valid schema/value/order identity, all published operands, combined and per-hillslope discovery |
| B boundary | planned `totalwatsed3_zero_runoff_has_zero_finite_sediment_volume_concentration`, `totalwatsed3_outlet_lateral_is_selected_per_wepp_and_day`, `totalwatsed3_optional_partial_coverage_uses_matched_area`, `totalwatsed3_wat_domain_controls_missing_and_extra_pass_days` | zero denominator, max-OFE boundary, partial optional denominator, PASS/WAT day-domain mismatch |
| C branch | existing explicit/default discovery tests; planned `totalwatsed3_alias_columns_and_optional_presence_branches_are_exact` | `sim_day_index|day`, `ofe_id|OFE`, selector present/absent, optional absent/all-null/present/mixed-null, combined/per-hill filename override |
| D domain reject | planned `totalwatsed3_rejects_nonpositive_area_and_negative_nonnegative_operands`, `totalwatsed3_rejects_invalid_optional_values_without_partial_output` | area `<=0`, negative `runvol/sbrunv/sedcon`, non-finite signed masses/fluxes, output created before error |
| E missing/type/null | existing `totalwatsed3_cli_fails_closed_when_required_pass_input_is_missing`, `totalwatsed3_cli_rejects_missing_explicit_optional_inputs`; planned `totalwatsed3_required_column_type_and_null_errors_preserve_priority` | exact TW3/CLI error code, path, column, row index, required-versus-optional distinction, error order |
| F non-finite | planned `totalwatsed3_every_real_operand_rejects_nan_and_infinities` | every PASS/WAT/soil/element real family, including optional all-null versus mixed-null semantics |
| G conservation/continuity | planned `totalwatsed3_independent_daily_water_closure_reconstructs_from_source_rows`, `totalwatsed3_sediment_mass_and_volume_oracle_rejects_aliases`; retain real WSHED01 closure audit as cohort evidence | anti-tautology, independent PASS runoff, real daily/whole-run magnitude, sediment class/mass closure |
| H fail closed/order | existing CLI argument/error tests; planned `totalwatsed3_row_key_output_order_and_error_precedence_are_exact`, `totalwatsed3_optional_join_key_collisions_and_unmatched_rows_are_explicit` | BTreeMap key order, path/batch/row accumulation order, no partial output, optional key omits `sim_day_index`, duplicate WAT lookup overwrite, unmatched optional-row policy |

Highest-risk cover-first targets are `read_wat_batch`, optional soil/element
readers, `DateOfeKey`, mixed-null handling, and the independent oracle. No
CRAP-driven decomposition is authorized until these bindings pass at the
package's science-tier line/region and per-function floors.
