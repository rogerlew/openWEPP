# CAL-07 Prospective Scientific Review B

Evidence class: `Static`

Review scope: package intent and prespecified methods only. No result-bearing
CAL-07 execution was reviewed.

Recommendation: `HOLD BEFORE RESULT EXECUTION`

The two sites are a useful, prospectively frozen Southern Hemisphere
evaluation pair, and the package correctly preserves provisional status,
rejects Northern Hemisphere retuning, and limits camera greenness to a
relative proxy. Result execution should wait until the closure and
observation-operator gaps below are corrected and independently re-reviewed.

## Ranked findings

### P0-1 — The package cannot satisfy the current roadmap advancement gate

Roadmap Order 7 requires timing, phase, **amplitude**, mass closure, and
consumer chronology, while section 7.4 requires an observed persistent
evergreen or mixed floor. The package excludes absolute canopy amplitude and
instantiates `evergreen_fraction=1` using assumed contract-test canopy-scale
operands. The Alerce `EN` site classification is useful ecological metadata,
but neither that label nor GCC color variation is a quantitative observation
of persistent foliar mass, LAI, canopy cover, or a winter canopy floor.

Before execution, the package must choose and record one truthful route:

1. admit a quantitative Southern Hemisphere amplitude/floor observation with
   a unit-, scale-, and uncertainty-defined operator; or
2. declare that CAL-07 is a bounded phase/timing characterization that cannot
   pass the Order 7 amplitude/floor advancement cell, keep that roadmap cell
   `NOT_EVALUATED`, and forbid a completed-CAL-07 or broad Southern Hemisphere
   robustness claim.

The second route can still produce valuable evidence, but it is not the
roadmap advancement currently named by the package title.

### P0-2 — The planned tests do not prove the required phase transform through real consumers

`full_wrapped_nh_climate_phase_flip_preserves_sh_canopy_and_limb_order`
tests the GSI/canopy state plus allocation and litter events. It does not run
snow, ET, WB15 interception, residue/depth/frost, or erosion consumers.
`native_forest_yaml_executes_through_the_direct_production_consumer` proves
that those consumers read one post-phenology state in a two-day, latitude-zero
fixture. It does not compare phase-transformed Northern and Southern consumer
trajectories.

Running the two tests separately is strong compositional ordering evidence,
but it does not meet section 7.4's literal complete cyclic phase-transform
gate for “all real consumers.” The package must either add a complete
phase-transformed direct-consumer execution, with consumer-specific
chronology comparisons, or explicitly classify consumer **ordering** as
proved and consumer **phase invariance** as `NOT_EVALUATED`. A static lineage
argument must not be reported as a consumer trajectory run.

### P1-3 — The daily shape score is not reproducibly prespecified

“Daily normalized-shape correlation and RMSE” does not define:

- whether GCC90 or `smooth_gcc_90` is scored;
- whether normalization is global, annual, seasonal, or event-local;
- which source and model extrema define the transform;
- how leap days, gaps, unequal daily support, and partial 2023/2026 seasons
  are handled; or
- whether scores are pooled across years or retained per season.

These choices can materially change correlation and RMSE and may conceal
amplitude or gap effects. Freeze the exact equation and scoring rows before
execution. Any smoothed series must still be restricted to dates with
`image_count > 0` and an accepted GCC90 quality flag; the pre-camera smoothed
extension visible in the daily product is not observed evidence. Report
season-level values before any pooled summary.

### P1-4 — Crossing extraction and transition pairing are incomplete

Pairing by calendar year and direction does not specify what happens when a
member has zero, one, or multiple upward/downward GSI 0.5 crossings. It also
does not protect the austral sequence that crosses the calendar boundary
(rising in one year followed by falling in the next).

Freeze a deterministic one-to-one event rule before execution, including:

- the strict/non-strict crossing equation and interpolation convention;
- treatment of threshold plateaus and adjacent recrossings;
- event eligibility at partial-series boundaries;
- unmatched observed and modeled events; and
- chronology checks on the full time-ordered austral sequence.

Do not select the nearest crossing after viewing residuals. A PhenoCam 50%
GCC transition and a GSI 0.5 crossing are only analogous relative midpoints;
their residual is descriptive and cannot establish physiological leaf-on or
leaf-off accuracy without mapping authority.

### P1-5 — Site independence and ROI authority need narrower wording and stronger binding

Beza Mahafaly and Alerce Costero are geographically and climatically
independent of each other and were not used in CAL-04B fitting. Both
observational products nevertheless come from the same PhenoCam processing
system, so their processing errors are not methodologically independent.
Describe them as independent sites/lane assignments, not independent
measurement methods.

The frozen inputs currently identify site metadata and `DB_1000`/`EN_1000`
daily products, but the package must also bind the exact ROI definition or
mask, ROI description, product version/methods DOI, download URL, retrieval
time, and digest. Without the ROI binding, the broad site vegetation class
does not by itself prove what pixels the scored series represents. Retain
site-specific acknowledgments. Also disposition the stale license text
embedded in downloaded metadata against the separately frozen live CC BY 4.0
fair-use page rather than silently choosing one.

### P1-6 — NASA POWER forcing is gridded model/assimilation evidence, not an observed site series

The package objective says “observed daily minimum temperature.” The frozen
POWER response identifies `GEOSIT`, `MERRA2`, and `POWER` sources and is a
point extraction of gridded products. Replace “observed” with
“source-provided gridded daily forcing” throughout the claim chain. Record
the returned coordinates, LST time basis, units, API version, source list,
and `-999` fill semantics.

The VPD reconstruction is contract-legitimate if it uses the exact production
saturation-vapor-pressure equation and Pa conversion. Preserve raw
`T2M_MAX`, `T2M_MIN`, and `T2MDEW`, independently reconstruct VPD from them,
and fail on missing, non-finite, negative, or date-misaligned values. Passing
this gate supports faithful forcing translation, not meteorological
representativeness at either camera footprint.

### P2-7 — Ensemble retention needs an exact parameter-level custody gate

The accepted-ensemble ledger names 37 IDs, while parameter execution depends
on the CAL-04B candidate-configuration join. Before result execution, freeze
the hashes of both predecessor tables and produce a 37-row parameter manifest
showing an exact one-to-one ID join, no duplicate or missing member, and no
member-specific modification. Require both sites to emit every member over
the complete forcing interval. Aggregate bands must not obscure individual
crossing failures, unmatched events, or contrary seasons.

## Conditions to lift this hold

1. Reconcile the amplitude/evergreen-floor and real-consumer phase gates with
   the roadmap, using explicit `SUPPORTED`, `BOUNDED`, or `NOT_EVALUATED`
   claim ceilings.
2. Freeze the exact daily-shape and crossing/pairing algorithms, including all
   missing-data and partial-season behavior.
3. Complete the ROI, PhenoCam method/license, POWER forcing, and
   parameter-level ensemble custody bindings.
4. Obtain renewed independent prospective review before any result-bearing
   executor or analyzer run.

## Amended-protocol re-review

Evidence class: `Static`

Re-review scope: amended `package.md`, frozen intent, source-authority record,
source manifest, ROI records, ensemble custody, `prepare_inputs.py`, and
`analyze.py`. No result-bearing execution was reviewed.

Recommendation: `HOLD BEFORE RESULT EXECUTION`

The amended protocol resolves the two original scope findings. It now
truthfully defines a bounded evaluation, explicitly withholds Order 7, keeps
absolute amplitude and quantitative evergreen-floor agreement
`NOT_EVALUATED`, and separates the cyclic producer-state test from the
real-consumer ordering test. The raw-GCC filter, 2024/2025 calendar-window
normalization, minimum paired-day support, crossing inequalities,
interpolation, non-nearest bracket assignment, edge-event exclusion, and
unmatched-event retention are now reproducibly specified.

The source bindings also materially improve:

- all 13 retained source objects match their manifest size and SHA-256 digest;
- ROI records bind the source-assigned descriptions and mask-version
  chronology for both ROI IDs;
- PhenoCam method, provisional status, shared-method limitation,
  acknowledgments, and license-text conflict are explicit;
- POWER is correctly classified as gridded/reanalysis forcing, including its
  LST basis, API version, sources, fill value, coordinates, and material
  elevation mismatch; and
- the current custody artifact contains 37 unique IDs and matches the exact
  predecessor ledger and candidate-table digests.

The executor's saturation-vapor-pressure equation exactly matches the
production runner helper, and the current normalized forcing contains both
sites from 2022-01-01 through 2026-07-24 with finite raw temperature/dew-point
values and no retained fill value.

### Remaining P0-1 — Verdicts are hard-coded independently of result agreement

`analyze.py` always writes `SUPPORTED` for signed-latitude calendar/seasonal
direction and consumer ordering, and `BOUNDED` for transition chronology,
relative shape, and persistent-evergreen realization. Those rows do not
depend on:

- unmatched or extra modeled transitions;
- the sign or magnitude of shape agreement;
- whether the external producer-phase and consumer-ordering test commands
  actually pass; or
- whether the observational seasonal direction agrees with the modeled
  direction.

This can publish support even after contrary evidence. Before execution,
separate result-independent evidence ceilings from terminal verdicts.
Generate terminal statuses only after consuming explicit gate receipts and
prespecified result predicates. Because no defensible observational accuracy
threshold exists, the observation rows may remain `BOUNDED` or become
`CONTRADICTED` when direction is reversed, but they cannot be hard-coded.
The signed-latitude implementation claim may cite the contract phase test;
it must not cite “two independent SH lanes” unless their retained results
actually agree. Preserve every contrary site/year/member.

The `persistent evergreen realization` row should be renamed to
`evergreen-class compatibility boundary` or otherwise made unmistakably
distinct from the quantitative evergreen-floor cell, which the amended
protocol correctly keeps `NOT_EVALUATED`.

### Remaining P0-2 — Required inventory and closure gates are calculated but not enforced

The analyzer checks only 37 IDs globally and the two global site names. It
does not require every site/member pair to contain the same complete daily
date set. A member missing at one site could pass the current inventory check.
Likewise, maximum mass-closure residual is calculated and labeled
`SUPPORTED`, but no failure occurs when it exceeds `1e-12 kg m-2`.

Before execution, require:

- exactly 37 unique member IDs at each site;
- exactly one row per site/member/date over the frozen forcing dates;
- no duplicate or missing dates and chronological order per trajectory;
- finite bounded kernel fields; and
- a hard failure when absolute mass-closure residual exceeds `1e-12 kg m-2`.

The terminal verdict must consume those checks rather than infer passage from
artifact presence.

### Remaining P1-3 — Custody generation should enforce the one-to-one join it claims

The current custody output is correct: 37 rows, 37 unique IDs, one parameter
row per accepted ID, and correct predecessor hashes. `prepare_inputs.py`
nevertheless converts the accepted ledger directly to a set and checks only
the number of selected candidate rows. It does not reject duplicate accepted
IDs, duplicate candidate IDs, an ID-set mismatch, or unexpected accepted
state.

Add explicit uniqueness, `ACCEPTED_FROZEN` state, exact ID-set equality, and
one-to-one join checks. The executor should also reject duplicate member IDs,
not merely require 37 rows. This makes deterministic regeneration as strong
as the currently inspected artifact.

### Remaining P1-4 — Small schema and support-wording mismatches need correction

- `transition-residuals.csv` writes a full calendar ordinal into
  `observed_doy_50`; write the source DOY in that field or rename it
  `observed_ordinal`.
- The protocol says to retain the count of **extra** same-direction crossings,
  while the output field counts all crossings in the bracket. Either emit
  `extra_crossings=max(count-1, 0)` or rename and document the current count.
- Alerce 2024 admitted observations begin on 2024-01-23 because the ROI record
  has no active mask through 2024-01-22. Call 2024 and 2025 fixed
  calendar-year scoring **windows**, not complete observational years, and
  retain the exact paired-day count. The current joined-date rule and
  180-day floor are otherwise clear.
- `prepare_inputs.py` should explicitly parse and reject non-finite GCC and
  forcing values rather than relying on the present source files being clean.

## Conditions to lift the amended hold

1. Make verdict generation conditional on retained observations and explicit
   external test receipts, with `CONTRADICTED` available for directional
   disagreement.
2. Enforce complete per-site/member/date inventory, finite/bounded state, and
   the declared mass-closure tolerance.
3. Strengthen the reproducible one-to-one custody checks and correct the
   transition/support schema wording.
4. Renew both prospective reviews on the corrected frozen protocol before
   any result-bearing run.

## Final corrected-tree prospective check

Evidence class: `Static`

Recommendation: `HOLD BEFORE RESULT EXECUTION`

The current tree resolves most amended-review findings:

- both focused gate receipts are consumed and must be `PASS`;
- closure operands use round-trip-safe serialization and both analyzer and
  independent validator enforce `1e-12 kg m-2`;
- VPD is independently reconstructed from retained raw forcing;
- the current 37-member custody join is unique and bound to both predecessor
  digests;
- source normalization rejects non-finite forcing and admitted GCC90;
- transition DOY and crossing-count names are now truthful; and
- the scoring-window wording no longer implies gap-free camera coverage.

Three exact-tree issues remain.

### CAL07-PRB-F1 — The independent inventory gate still counts rows, not dates

Both `analyze.py` and `validate.py` require 74 site/member groups with 1,666
rows, but neither compares each group's unique, ordered date sequence with
that site's 1,666 forcing dates. A duplicated date replacing a missing date
passes both independent inventory checks. The producer's chronological
kernel guard makes that unlikely in an untampered run, but it does not fulfill
the independently verified date-inventory condition from the prior review.

Require, for every site/member, exact date-set equality, no duplicate dates,
and strictly increasing chronology against `forcing.csv`. Also assert all
published VPD, GSI, foliar-activity, biomass, allocation, litter, and closure
fields are finite; the current validator explicitly bounds GSI and closure
but can allow `NaN` to evade `max()` checks in other numeric columns.

### CAL07-PRB-F2 — Quantitative evergreen-floor agreement is not visibly `NOT_EVALUATED`

The package prespecifies a visible `NOT_EVALUATED` cell for quantitative
evergreen-floor agreement. `verdict-matrix.csv` instead contains only
`persistent evergreen realization = BOUNDED`. That is a useful
execution/class-compatibility boundary, but it is not the missing
observational floor cell and can be mistaken for it.

Retain the bounded compatibility row under an unambiguous name and add a
separate `quantitative evergreen-floor agreement = NOT_EVALUATED` row.
Require that status in `validate.py`, as is already done for absolute
amplitude and phase-transformed consumers.

### CAL07-PRB-F3 — Observational verdicts remain unconditional

The producer-phase, mass-closure, and consumer-ordering statuses now consume
their actual gates. The signed-latitude seasonal-direction, transition, and
shape rows remain fixed as `BOUNDED` regardless of unmatched events or
directional disagreement. A bounded ceiling is appropriate without an
accuracy threshold, but the result synthesis must still retain an explicit
`CONTRADICTED` site/year/member cell when observed and modeled seasonal
directions reverse. At minimum, publish and gate a directional-agreement
inventory; do not let the existence of observations alone satisfy the
package's stated “when both lanes ... agree” condition.

The source-authority wording also still calls V3 the provisional
“product/method authority.” Treat V3 and Young et al. explicitly as method
lineage, not as the file identity of these later provisional products. This
wording correction is non-blocking by itself.

Once F1--F3 are corrected and independently checked, Review B's prospective
recommendation is `GO` for the bounded evaluation only. Order 7 advancement,
absolute amplitude, quantitative evergreen floor, and phase-transformed real
consumer chronology remain withheld.

## Superseding final prospective decision

Evidence class: `Static`

Recommendation: `GO FOR BOUNDED RESULT EXECUTION`

This decision supersedes Review B's earlier prospective `HOLD`
recommendations. No result-bearing CAL-07 execution was inspected or used to
set this decision.

The exact current protocol resolves `CAL07-PRB-F1` through
`CAL07-PRB-F3`:

- the independent validator compares every site/member trajectory with the
  exact ordered 1,666-date forcing sequence, so duplicate, missing, or
  out-of-order dates fail;
- every published numeric daily field is required to be finite, GSI remains
  bounded, VPD is independently reconstructed, and both published and
  independently reconstructed mass closure retain their declared tolerances;
- the verdict matrix now separately retains
  `quantitative evergreen-floor agreement = NOT_EVALUATED`;
- the package prospectively defines directional shape agreement as a strictly
  positive across-member median Pearson correlation for every site/year
  scoring cell;
- the analyzer conditionally marks signed-latitude seasonal direction and
  relative shape `CONTRADICTED` if that rule fails;
- deciduous transition chronology is `CONTRADICTED` when any prespecified
  eligible event/member match is absent; and
- producer phase, closure, and consumer ordering consume their actual gates
  while phase-transformed consumer chronology, absolute amplitude,
  quantitative evergreen floor, and source-incomplete downstream physics
  remain `NOT_EVALUATED`.

The V3 wording now correctly identifies method lineage and distinguishes the
later provisional products from the curated release. Source custody,
parameter custody, observation filters, forcing classification, score
equations, crossing rules, evidence ceilings, and Order 7 withholding remain
unchanged and acceptable.

Review B therefore authorizes result-bearing execution of this frozen
bounded-evaluation protocol. This is not authorization to claim completed
Southern Hemisphere robustness, pass Order 7, refit the ensemble, introduce
an amplitude/floor mapping, or treat separate producer-phase and
consumer-ordering tests as a phase-transformed consumer run. Any change to
the frozen sites, inputs, ensemble, filters, score predicates, crossing
assignment, or advancement ceiling requires an incident record and renewed
prospective review.
