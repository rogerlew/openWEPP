# Native-Forest Canopy Phenology in openWEPP

*Version 1.0 — 2026-07-29*

*Audience: hydrologists, forest managers, erosion modelers, and scientific
reviewers configuring or interpreting native-forest simulations.*

## Why Forest Needs Its Own Seasonal State

Historical WEPP forest applications commonly represented forest, shrub, and
grass areas through perennial cropland management records. That was a useful
compatibility route because the original native land-use branches were not
finished for production. It did not, however, make a mixed or deciduous forest
an agricultural field in ecological terms. A crop schedule has planting,
maturity, harvest, and residue-management events. A forest has persistent
woody structure, foliage that may be evergreen or seasonally renewed, gradual
leaf-on and leaf-off, and a forest floor that receives litter whenever leaves
are shed.

Those differences matter to a hillslope model. The amount and height of
foliage alter rainfall and snow interception. Leaf area affects transpiration.
Canopy and ground cover shelter soil from raindrop impact and overland flow.
Litter mass affects residue cover and the thermal boundary over the soil. A
fixed crop-like canopy can therefore impose the wrong seasonal timing on
several processes at once even when its annual-average cover looks plausible.

openWEPP's `native_forest` land use provides an explicit, continuously active
forest state. It separates persistent structure from evergreen and deciduous
foliage, derives the seasonal foliar signal from daily weather and latitude,
and transfers modeled leaf loss to the forest floor on the day it occurs. It
is a configured capability, not an inferred upgrade: a native forest requires
a complete management definition, while compatibility forest inputs retain
their historical behavior.

## From Weather to Leaf-On and Leaf-Off

The seasonal signal is the generalized growing season index (GSI) of Jolly,
Nemani, and Running (2005). GSI treats minimum temperature, atmospheric vapor
pressure deficit (VPD), and photoperiod as simultaneous constraints on foliar
activity. Each daily indicator varies from zero, where the constraint is fully
inactive, to one, where it is unconstrained. Their product is averaged over
the current and preceding 20 available days:

```text
daily GSI = temperature indicator × VPD indicator × photoperiod indicator
foliar GSI = mean of as many as 21 consecutive daily GSI values
```

Temperature and photoperiod indicators increase between their two thresholds.
The VPD indicator decreases between its thresholds because dry air increases
atmospheric demand. Photoperiod is calculated from the signed latitude and
calendar date using solar geometry from FAO-56 (Allen et al., 1998). The
moving mean makes leaf-on and leaf-off gradual rather than a one-day switch.
The first 20 days of a run are a warm-up period based only on weather actually
present in the run; openWEPP does not invent prior days.

GSI is a constraint model, not a complete theory of forest phenology. It does
not directly represent soil-water access, stored stem water, species
composition, hydraulic traits, disturbance, or reproductive phenology. Those
omissions are especially important in tropical dry forests, where rainfall,
groundwater, stored water, and species-specific strategies can alter leaf
timing (Chapotin et al., 2006; Méndez-Alonzo et al., 2013; Rivera et al.,
2002).

## Turning Foliar Activity into a Canopy

The model divides the full-leaf foliar pool into evergreen and deciduous
fractions. If `f_e` is the evergreen fraction and `GSI` is the 21-day foliar
signal, the realized foliar fraction is:

```text
f = f_e + (1 - f_e) × GSI
foliar biomass = summer foliar biomass × f
LAI = maximum LAI × f
```

An evergreen forest (`f_e = 1`) keeps its modeled foliar pool throughout the
year. A deciduous forest (`f_e = 0`) follows the full GSI cycle. A mixed forest
retains its evergreen floor while the deciduous share changes. This is an
aggregate community representation; it does not simulate individual species
or crown layers.

Canopy cover is the larger of a persistent structural-cover floor and the
foliar cover calculated from live foliar biomass. Canopy height is calculated
from persistent structural biomass plus current foliar biomass using the WEPP
community-height relation (Flanagan & Nearing, 1995). Persistent cover and
structural biomass are separate because a leafless branch network may retain
effective overhead cover and height without being part of the seasonal leaf
pool.

The change in foliar biomass closes a daily mass ledger. An increase is leaf
allocation; a decrease is leaf litter. The first modeled day establishes the
initial foliar state and creates neither allocation nor litter. Thereafter:

```text
current foliage =
    previous foliage + leaf allocation - leaf litter
```

Leaf litter is deposited into surface and ground-residue representations on
the same day, before decomposition, residue cover, residue depth, and frost
inputs are calculated. Native forest does not wait for a crop harvest date to
release this litter.

## How the Seasonal State Reaches Water and Sediment

All downstream processes receive the same post-phenology canopy state for a
given day. That ordering prevents snow, evapotranspiration, erosion, and
residue calculations from seeing different versions of the forest.

Interception and evapotranspiration respond to current foliar biomass, leaf
area, canopy cover, and height. A larger or denser canopy can retain more
precipitation and snow and can support greater transpiration demand, subject
to the rest of the water-balance state. The canopy also changes the path and
timing by which water reaches the ground.

Snow processes use current canopy attenuation and height, while frost uses
the current residue-depth boundary after litter deposition and decomposition.
Leaf-off can therefore reduce overhead interception while increasing the
organic material at the soil surface. Those two effects act in different
directions and should not be collapsed into a single “forest cover” value.
The broader winter-process context is described in
[Snow and frost in openWEPP](snow-frost-modeling-and-validation.md).

Erosion processes use canopy and residue cover to modify raindrop impact,
interrill detachment, rill behavior, and sediment delivery. Runoff routing
also consumes the daily vegetation state. The direction of a final runoff or
sediment response cannot be inferred from one canopy coefficient alone:
weather, soil water, snow, frozen soil, surface residue, hydraulic roughness,
and event intensity remain active causes.

## Configuring the Coefficients

A `native_forest` management record must explicitly provide the phenology,
growth, decomposition, and routing information required by the selected
runtime path. There are no hidden generalized-GSI defaults. The hard domains
below are input-validity rules, not recommended ecological ranges.

Every scalar in the three coefficient tables is a required input; openWEPP
does not ship a default native-forest vector. The two external litter
time-series fields are observed forcings rather than coefficients, and the
residue mass-to-depth ratio discussed below is derived at runtime rather than
entered by the user.

The “range evidence” column uses three distinctions:

- `HARD_DOMAIN` is only the parser or runtime validity domain.
- `CALIBRATION_ENSEMBLE` or `SOURCE_RANGE` is limited to the named evidence
  and scale.
- `NOT_ESTABLISHED` means openWEPP has no defensible general starting range;
  local observations must supply it.

The tables give the effect direction for every field. The following map binds
those rows to their equation location and minimum calibration evidence:

| Fields | Equation or process location | Minimum observation need and scale |
| --- | --- | --- |
| Six `phenology` weather and day-length thresholds | Piecewise temperature, VPD, and photoperiod indicators; their product; and the 21-day mean | At least one observed rising and falling canopy transition per year across two or more years, with complete daily forcing and correct signed latitude at the community or overland-flow-element scale |
| `summer_foliar_biomass_kg_m2` and `evergreen_fraction` | `foliar biomass = summer biomass × [evergreen fraction + deciduous fraction × GSI]` | Full-leaf dry foliar mass per horizontal area, a functional-type inventory, and matched leaf-on/leaf-off foliage |
| `structural_canopy_cover_fraction` and `growth.bb` | `cover = max(structural floor, 1 - exp(-bb × foliar biomass))` | Leaf-off effective overhead cover plus paired foliar dry mass and cover at two or more distinct foliar states |
| `structural_biomass_kg_m2`, `growth.bbb`, and `growth.hmax` | `height = [1 - exp(-bbb × total canopy biomass)] × hmax` | Matched structural/foliar biomass and canopy height; multiple biomass-height states are needed to separate the curve coefficient from its asymptote |
| `growth.xmxlai` | `LAI = maximum LAI × realized foliar fraction` | At least one representative mature full-leaf LAI measurement per horizontal ground area; repeated plots and dates are preferable |
| `decomposition.oratea` and `decomposition.orater` | Environmentally modified first-order surface- and root-residue recurrences | At least two stock measurements with intervening, material-separated inputs; a seasonal time series is preferable |
| `cf` | `residue cover = 1 - exp(-cf × ground-residue mass)` and the inverse initial-cover calculation | Paired dry residue mass and interrill/rill cover at two or more residue states |
| `diam` | Legacy residue material/depth classification input | A representative material description or diameter; the current native-forest branch is insensitive to this field, so it has no calibration target |

### Seasonal timing

| YAML field | Meaning and units | Hard domain | Range evidence and effect |
| --- | --- | --- | --- |
| `phenology.minimum_temperature_inactive_c` | Minimum temperature at complete cold constraint, °C | finite and less than the unconstrained threshold | `CALIBRATION_ENSEMBLE`: -27.52 to -4.67 °C in the retained Hubbard Brook ensemble only. Raising it generally delays release from cold constraint. |
| `phenology.minimum_temperature_unconstrained_c` | Minimum temperature at no cold constraint, °C | finite and greater than the inactive threshold | `CALIBRATION_ENSEMBLE`: 6.70 to 19.91 °C at Hubbard Brook only. Raising it extends partial cold limitation. |
| `phenology.vapor_pressure_deficit_unconstrained_pa` | VPD below which atmospheric dryness does not constrain foliage, Pa | finite, nonnegative, and less than the inactive threshold | `CALIBRATION_ENSEMBLE`: 655.90 to 1100.65 Pa at Hubbard Brook only. Raising it delays the onset of VPD limitation. |
| `phenology.vapor_pressure_deficit_inactive_pa` | VPD at complete dryness constraint, Pa | finite and greater than the unconstrained threshold | `CALIBRATION_ENSEMBLE`: 2155.33 Pa for every retained Hubbard Brook member. Raising it makes complete dryness shutdown require higher VPD. |
| `phenology.photoperiod_inactive_hours` | Day length at complete short-day constraint, hours | 0 to less than the unconstrained threshold | `CALIBRATION_ENSEMBLE`: 10.936 to 12.479 hours at Hubbard Brook only. Raising it lengthens the short-day-constrained season. |
| `phenology.photoperiod_unconstrained_hours` | Day length at no short-day constraint, hours | greater than the inactive threshold and no more than 24 | `CALIBRATION_ENSEMBLE`: 11.239 to 15.294 hours at Hubbard Brook only. Raising it extends partial photoperiod limitation. |

The six timing thresholds are a correlated set. Changing one threshold can
move both spring and autumn transitions, and compensating changes can produce
similar seasonal curves. The retained temperate ensemble is therefore
partially identifiable: it is evidence of multiple acceptable timing
configurations, not a menu of independent ranges. Calibrate the six fields
together against dated leaf-on and leaf-off observations.

### Foliar and persistent structure

| YAML field | Meaning and units | Hard domain | Range evidence and effect |
| --- | --- | --- | --- |
| `phenology.summer_foliar_biomass_kg_m2` | Full-leaf dry foliar biomass per horizontal hillslope area, kg m^-2 | finite and greater than 0 | `NOT_ESTABLISHED` generally. More mass raises foliar biomass and usually foliar canopy cover; it also increases the seasonal mass available for modeled leaf transfer. |
| `phenology.evergreen_fraction` | Evergreen share of the full-leaf foliar pool, fraction | 0 to 1 inclusive | `NOT_ESTABLISHED` generally. A larger value retains more winter foliage and reduces the amplitude of modeled seasonal leaf allocation and litter. |
| `phenology.structural_canopy_cover_fraction` | Persistent effective branch/stem canopy-cover floor, fraction | 0 to 0.999 inclusive | `NOT_ESTABLISHED` generally. It raises canopy cover only when foliar cover would otherwise be lower. It is not the evergreen leaf fraction. |
| `phenology.structural_biomass_kg_m2` | Persistent above-ground structural dry biomass per horizontal hillslope area, kg m^-2 | finite and at least 0 | `NOT_ESTABLISHED` generally. It raises the biomass used to calculate canopy height but is excluded from seasonal leaf transfer. |
| `growth.xmxlai` | Full-leaf maximum leaf area index, m^2 leaf m^-2 ground | finite and greater than 0 | `SOURCE_RANGE`: 3.5 to 8.0 m^2 m^-2 was retained as a Hubbard Brook mature-LAI observation interval, not a physiological bound. Larger values scale LAI throughout the year. |
| `growth.bb` | WEPP foliar-biomass-to-canopy-cover coefficient, m^2 kg^-1 | finite and greater than 0 | `NOT_ESTABLISHED` generally. Larger values make foliar cover approach closure with less foliar biomass. It can be equifinal with foliar biomass and the structural-cover floor. |
| `growth.bbb` | WEPP total-biomass-to-height coefficient, m^2 kg^-1 | finite and greater than 0 | `NOT_ESTABLISHED` generally. Larger values make height approach `hmax` with less total canopy biomass. |
| `growth.hmax` | Maximum community canopy height, m | finite and greater than 0 | `NOT_ESTABLISHED` generally. It sets the asymptote of the modeled canopy-height relation. |

“Structural biomass” here is a canopy-height operand, not a predictive branch
turnover pool. Likewise, the structural cover floor is effective overhead
cover, not a stem-area measurement. Observations must match the modeled
quantity and horizontal-area basis.

### Litter and decomposition

| YAML field | Meaning and units | Hard domain | Range evidence and effect |
| --- | --- | --- | --- |
| `decomposition.oratea` | Optimum above-ground residue decomposition-rate constant, d^-1 | finite and at least 0 | `NOT_ESTABLISHED` generally. Larger values accelerate environmentally modified surface-residue loss. For recurring native-forest litter, configured zero invokes the authorized forest-litter fallback of 0.5 yr^-1 rather than making seasonal litter inert. |
| `decomposition.orater` | Optimum root-residue decomposition-rate constant, d^-1 | finite and at least 0 | `NOT_ESTABLISHED` generally. Larger values accelerate environmentally modified root-residue loss; zero is a no-decay constant for this pool. |
| `cf` | Ground-residue mass-to-cover coefficient, m^2 kg^-1 | finite and greater than 0 on the native depth-seed path | `NOT_ESTABLISHED` generally. Larger values produce more cover from a given residue mass and imply less initial mass when the declared initial cover is inverted. |
| `diam` | Residue material or stem-diameter descriptor, m | finite and at least 0 | `NOT_ESTABLISHED` generally. It is carried into the legacy depth-classification function, but the current native-forest `landuse=3` branch uses the fixed non-cropland conversion class; changing `diam` therefore does not change current native-forest depth. |

Surface and root residue are separate pools. Each day's authorized litter is
added before a first-order exponential decay step whose effective rate is
modified by temperature and moisture. Current interrill and rill residue
covers are then calculated as `1 - exp(-cf × mass)`. The same current surface
mass is multiplied by a derived mass-to-depth ratio to form the frost-facing
residue depth. Thus decomposition can lower both erosion protection and
thermal resistance, but the cover and depth conversions are not
interchangeable.

There is no standalone native-forest YAML depth coefficient. The initial
condition supplies `initial_conditions[].inrcov` and `.rilcov` (declared
interrill and rill cover fractions) and `.sumsrm` (initial surface-residue dry
mass, kg m^-2). The referenced forest plant supplies `cf` and `diam`. openWEPP
inverts the declared covers with `cf` to establish initial ground-residue mass,
uses the native land-use conversion class to establish initial depth, and
derives the runtime
`residue_depth_conversion_m_per_kg_m2 = initial depth / initial sumsrm`.
The initial cover, mass, and material description must represent one coherent
surface state. The derived ratio is not independently tuned.

The configured-zero `oratea` fallback is a narrow runtime rule for recurring
native litter, not a calibrated value, a typical forest rate, or a recommended
user setting. It uses a first-order rate of 0.5 yr^-1, following the
contract-authorized moderate forest-litter class informed by Olson's stock and
turnover formulation and long-term forest-floor decomposition evidence (Olson,
1963; Qualls, 2016).

Modeled broadleaf leaf litter is only the day-to-day loss from the foliar pool.
Needle and fine-woody deposition are separate observed external forcings under
`surface_litter_forcing.needle` and
`surface_litter_forcing.fine_woody`. A complete forcing supplies exact daily
oven-dry or constant-dry mass in `kg m^-2 d^-1`, bound to its dates, material,
plot or overland-flow element (OFE), and authenticated source. Interval
collections cannot be divided among days without independent temporal
authority (Keane, 2008a, 2008b). Missing applicable material uses the YAML
status `not_represented`, not a numeric zero.

openWEPP does not currently predict recurring needle or fine-woody deposition
from evergreen fraction or structural biomass. Branch turnover, attached dead
wood, in-canopy loss, and material reaching the ground are distinct quantities
and require state that the present aggregate canopy does not carry (Lim et
al., 2024).

## A Calibration Sequence That Preserves Meaning

Begin with observations, not runoff or sediment residuals. First classify the
forest's functional composition and determine which structure persists through
the leaf-off season. Species or functional-type inventories, winter canopy
photography, and leaf-habit information constrain evergreen fraction and help
separate evergreen foliage from woody cover.

Next constrain full-leaf foliar biomass, mature LAI, canopy cover, and canopy
height at compatible spatial scales. Measurements of only total above-ground
biomass cannot uniquely separate summer foliage from persistent structure.
Similarly, canopy cover alone generally cannot distinguish `bb`, foliar
biomass, and the structural-cover floor.

Then fit seasonal timing jointly. Use multiple years of dated leaf emergence,
canopy development, senescence, and leaf fall if available. Preserve all
threshold combinations that meet the observation criterion; do not select one
coefficient at a time while fixing the others by convenience. Digital-camera
greenness can be useful, but it is not identical to leaf area or leaf mass and
can depend on canopy color and the camera region of interest (Donnelly et al.,
2022; Keenan et al., 2014).

Assess litter source and decomposition after the canopy transfer is
constrained. At minimum, use repeated dry-mass litter inputs separated by
material class and a time series of forest-floor or modeled-pool-equivalent
mass. A single terminal stock cannot identify annual input and decomposition
rate separately: a higher input paired with faster decay can yield the same
stock. Root data are needed before interpreting `orater`.

Finally, reserve sites or years for transfer evaluation without refitting.
The retained Hubbard Brook timing ensemble was only partially identifiable and
did not transfer successfully to the evaluated Harvard Forest chronology.
Evaluation in a tropical dry-forest lane produced seasonally incoherent
transitions; another timing calibration round was stopped because the present
GSI structure and available observations could not identify a defensible
solution. Apply the model outside northern temperate deciduous conditions only
with independent local evidence and an explicit limitation statement.

Do not tune canopy coefficients to conceal a residual in snow, frost, runoff,
erosion, sediment, litter source, or decomposition. Diagnose the responsible
process with its own observations. A better hydrograph obtained by giving the
forest an implausible LAI is compensation, not canopy calibration.

## Interpreting a Native-Forest Run

A native-forest run may be interpreted as a daily, mass-consistent projection
of the configured GSI foliar model into canopy, broadleaf litter, residue, and
their downstream process inputs. It can compare how explicitly defined forest
structures and seasonal states propagate through the model.

It may not be interpreted as a species-level phenology forecast, a prediction
of needle or fine-wood litterfall, or evidence that the Hubbard Brook timing
ensemble transfers to another ecosystem. Agreement in runoff or sediment
alone does not validate the canopy state, and disagreement does not by itself
identify canopy phenology as the cause. The defensible application is the one
whose composition, canopy, seasonal timing, and litter observations support
the configured quantities at the modeled scale.

## References

Allen, R. G., Pereira, L. S., Raes, D., & Smith, M. (1998). *Crop
evapotranspiration: Guidelines for computing crop water requirements* (FAO
Irrigation and Drainage Paper 56). Food and Agriculture Organization.

Chapotin, S. M., Razanameharizaka, J. H., & Holbrook, N. M. (2006). Baobab
trees (*Adansonia*) in Madagascar use stored water to flush new leaves but not
to support stomatal opening before the rainy season. *New Phytologist, 169*,
549–559. https://doi.org/10.1111/j.1469-8137.2005.01618.x

Donnelly, A., Yu, R., Jones, K., Belitz, M., Li, B., Duffy, K., Zhang, X.,
Wang, J., Seyednasrollah, B., Gerst, K. L., Li, D., Kaddoura, Y., Zhu, K.,
Morisette, J., Ramey, C., & Smith, K. (2022). Exploring discrepancies between
in situ phenology and remotely derived phenometrics at NEON sites. *Ecosphere,
13*, e3912. https://doi.org/10.1002/ecs2.3912

Flanagan, D. C., & Nearing, M. A. (Eds.). (1995). *USDA-Water Erosion
Prediction Project: Hillslope profile and watershed model documentation*
(NSERL Report No. 10). USDA Agricultural Research Service.

Jolly, W. M., Nemani, R., & Running, S. W. (2005). A generalized,
bioclimatic index to predict foliar phenology in response to climate. *Global
Change Biology, 11*, 619–632.
https://doi.org/10.1111/j.1365-2486.2005.00930.x

Keane, R. E. (2008a). Biophysical controls on surface fuel litterfall and
decomposition in the northern Rocky Mountains, USA. *Canadian Journal of
Forest Research, 38*, 1431–1445. https://doi.org/10.1139/X08-003

Keane, R. E. (2008b). *Surface fuel litterfall and decomposition in the
northern Rocky Mountains, USA* (RMRS-RP-70). USDA Forest Service.
https://doi.org/10.2737/RMRS-RP-70

Keenan, T. F., Darby, B., Felts, E., Sonnentag, O., Friedl, M. A., Hufkens,
K., O'Keefe, J., Klosterman, S., Munger, J. W., Toomey, M., & Richardson,
A. D. (2014). Tracking forest phenology and seasonal physiology using digital
repeat photography: A critical assessment. *Ecological Applications, 24*,
1478–1489. https://doi.org/10.1890/13-0652.1

Lim, H., Medvigy, D., Mäkelä, A., Kim, D., Albaugh, T. J., Knier, A., Blaško,
R., Campoe, O. C., Deshar, R., Franklin, O., Henriksson, N., Littke, K.,
Lutter, R., Maier, C. A., Palmroth, S., Rosenvald, K., Slesak, R. A., Tullus,
A., & Oren, R. (2024). Overlooked branch turnover creates a widespread bias in
forest carbon accounting. *Proceedings of the National Academy of Sciences,
121*, e2401035121. https://doi.org/10.1073/pnas.2401035121

Méndez-Alonzo, R., Pineda-García, F., Paz, H., Rosell, J. A., & Olson, M. E.
(2013). Leaf phenology is associated with soil water availability and xylem
traits in a tropical dry forest. *Trees, 27*, 745–754.
https://doi.org/10.1007/s00468-012-0829-x

Olson, J. S. (1963). Energy storage and the balance of producers and
decomposers in ecological systems. *Ecology, 44*, 322–331.
https://doi.org/10.2307/1932179

Qualls, R. G. (2016). Long-term (13 years) decomposition rates of forest floor
organic matter on paired coniferous and deciduous watersheds with contrasting
temperature regimes. *Forests, 7*, 231.
https://doi.org/10.3390/f7100231

Rivera, G., Elliott, S., Caldas, L. S., Nicolossi, G., Coradin, V. T. R., &
Borchert, R. (2002). Increasing day-length induces spring flushing of tropical
dry forest trees in the absence of rain. *Trees, 16*, 445–456.
https://doi.org/10.1007/s00468-002-0185-3

## Revision Log

| Version | Date | Changes |
| --- | --- | --- |
| 1.0 | 2026-07-29 | Initial native-forest canopy-phenology science narrative, coefficient guide, calibration sequence, and interpretation boundary. |
