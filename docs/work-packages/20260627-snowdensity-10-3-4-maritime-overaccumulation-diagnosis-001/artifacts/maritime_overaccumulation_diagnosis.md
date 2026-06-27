# SNOWDENSITY-10.3.4 Maritime Over-Accumulation Diagnosis

Evidence mode: Ran.

- Schema: `snowdensity10-3-4-maritime-overaccumulation-diagnosis-v1`
- Contract: `SC-SNOWFREEZE-001 INV-SNOWFREEZE-047 INV-SNOWFREEZE-048 INV-SNOWFREEZE-050 INV-SNOWFREEZE-063`
- Runtime coupling: `diagnostic snowbench replay only; no production activation`
- No physics change: `True`
- No tuning: `True`
- Disposition: `PARTITION-THAW-FIRST`
- Next route: SNOWDENSITY-10.3.5 partition/thaw-window candidate package before rain-heat or longwave production changes.

## Mechanism Ranking

| Rank | Mechanism | Disposition | Evidence | Next action |
|---:|---|---|---|---|
| 1 | `snow_rain_partition_near_zero_c` | `DEFECT-ELIGIBLE` | Paired over-accumulating surfaces=4; phase-ambiguous precip over those surfaces=21.175 m; warm modeled snow input=6.939 m. | Author a partition-threshold/phase-confidence package before any melt coefficient tuning. |
| 2 | `winter_thaw_melt_response` | `DEFECT-ELIGIBLE` | Paired over-accumulating surfaces=4; positive-temperature snowpack hours over those surfaces=167815. | Decompose thaw-window melt operands and compare pack ablation across observed thaw periods. |
| 3 | `sub_canopy_longwave_or_forest_energy` | `DEFECT-ELIGIBLE` | Forested paired over-accumulating surfaces=2; current CoE path has canopy attenuation and temperature terms but no explicit sub-canopy longwave process. | Scope a forest-energy diagnostic only after partition/thaw windows are decomposed. |
| 4 | `rain_on_snow_heat` | `DEFECT-ELIGIBLE` | Warm-rain heat melt equivalent over paired over-accumulating surfaces=1.859 m water. The production formula already includes the CoE `dmelt` rain term. | Do not alter rain heat first unless event-window reconstruction shows the CoE `dmelt` term is numerically inactive during observed rain-on-snow failures. |
| 5 | `precipitation_bias` | `FORCING-LIMITED` | No independent precipitation-gauge or catch-correction authority is installed for these fixture hillslopes. | Report as uncertainty; do not tune shared precipitation to fix snow depth. |
| 6 | `representativeness` | `FORCING-LIMITED` | Observation-blocked surfaces=3; point/stratum versus hillslope correspondence remains load-bearing for HJ Andrews and Hubbard Brook. | Install paired observation tables before assigning defect labels at HJ Andrews or Hubbard Brook. |
| 7 | `wind_undercatch` | `NOT-SUPPORTED` | The observed paired failures are modeled-over-observed snow depth; increasing snow precipitation for undercatch would generally worsen that signal. | Keep as uncertainty only; do not prioritize as a corrective lever for over-accumulation. |

## Site Surfaces

| Surface | Site | Cover | Scope | Pairs | Mean depth residual m | Fail fraction | Ambiguous precip m | Warm snow m | Positive-temp snowpack h | Rain heat equiv m |
|---|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| `hjandrews_conifer` | `hjandrews` | `conifer` | `observation_blocked` | 0 | n/a | n/a | 17.3702 | 4.14333 | 33661 | 0.792615 |
| `sleepers_south_field` | `sleepers` | `open_field` | `paired_observation` | 384 | 0.410817 | 0.838542 | 4.19137 | 1.37 | 36923 | 0.350173 |
| `sleepers_w9_hardwood` | `sleepers` | `hardwood` | `paired_observation` | 193 | 0.325021 | 0.740933 | 5.44152 | 1.92102 | 45453 | 0.470001 |
| `harvard_hardwood` | `harvard` | `hardwood` | `paired_observation` | 448 | 0.383192 | 0.796875 | 5.85705 | 1.88835 | 38787 | 0.461733 |
| `harvard_open` | `harvard` | `open` | `paired_observation` | 390 | 0.391934 | 0.833333 | 5.68505 | 1.75998 | 46652 | 0.57703 |
| `hubbardbrook_deciduous` | `hubbardbrook` | `deciduous` | `observation_blocked` | 0 | n/a | n/a | 5.55817 | 1.82322 | 34125 | 0.382593 |
| `hubbardbrook_mixed` | `hubbardbrook` | `mixed` | `observation_blocked` | 0 | n/a | n/a | 5.12494 | 1.60319 | 35405 | 0.40902 |

## Observation-Blocked Surfaces

| Surface | Reason |
|---|---|
| `hjandrews_conifer` | Fixture exists, but EDI MS007 / SNOTEL paired snow observations are not installed. |
| `hubbardbrook_deciduous` | Fixture exists, but Hubbard Brook paired snow observations are not installed. |
| `hubbardbrook_mixed` | Fixture exists, but Hubbard Brook paired snow observations are not installed. |

Conclusion: partition and thaw-window diagnosis should precede any opt-in physics candidate. Rain heat is not first because the current CoE path already carries `dmelt`, and the diagnosed warm-rain heat magnitude is smaller than the broader phase/thaw signals.
