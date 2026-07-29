# CAL-07 Prospective Scientific Review A

Evidence class: `Static: frozen protocol, roadmap, contract, predecessor
dispositions, source products, and named production tests`

Status: `HOLD BEFORE RESULT-BEARING EXECUTION`

## Findings

### CAL07-PRA-001 — Blocker: consumer-phase evidence is not the roadmap gate

The two named existing tests prove different, narrower propositions:

- `full_wrapped_nh_climate_phase_flip_preserves_sh_canopy_and_limb_order`
  phase-transforms GSI, canopy, leaf-on allocation, and leaf-off litter in the
  phenology crate; it does not execute the downstream consumers.
- `native_forest_yaml_executes_through_the_direct_production_consumer` proves
  that one native fixture publishes a common post-phenology state to the real
  consumer path; it does not phase-transform that path and does not use either
  Southern Hemisphere observational lane.

Running those tests side by side therefore does not satisfy roadmap section
7.4's complete cyclic phase-transform gate for all real consumers or Order 7's
consumer-chronology criterion. Before execution, add a prespecified
production-path comparison that carries a complete phase-transformed cycle
through the named consumers, or explicitly limit CAL-07 to producer-state
characterization and withhold Order 7 advancement. A generic consumer-ordering
test may remain supporting evidence, but it cannot substitute for the missing
phase/chronology evidence.

### CAL07-PRA-002 — Blocker: observational score operators are not frozen

The protocol names normalized-shape correlation, RMSE, and calendar-year
crossing pairing without defining enough of the operator to prevent
post-result choices. It must specify, before reading results:

- whether raw `gcc_90` or source-smoothed `smooth_gcc_90` is scored;
- the normalization equation and its domain (whole site, year, or complete
  season), including treatment of partial seasons and zero range;
- the exact date join, minimum support, correlation definition, and whether
  scores are per season, per event, per member, or pooled;
- whether a modeled crossing is the first qualifying discrete day or an
  interpolated date, and how repeated crossings are resolved;
- how missing crossings, multiple crossings, incomplete boundary seasons,
  leap years, and source transitions outside admitted daily support are
  retained in inventory and scored.

The existing rule to publish residual days without an invented accuracy
threshold is scientifically sound. It does not remove the need to freeze the
measurement operator itself.

### CAL07-PRA-003 — Major: amplitude and evergreen-floor evidence cannot advance

The PhenoCam vegetation classes and GCC products are suitable for timing and
relative greenness. They are not absolute LAI, biomass, canopy cover, or
evergreen-foliar-fraction observations. Setting `evergreen_fraction=1` makes
model foliar persistence a prescribed boundary consequence of
INV-PLANT-034; it is not an empirical test of a persistent canopy floor.

Consequently, this design can characterize the independent evergreen lane and
can retain camera color dynamics that the constant foliar realization does
not represent. It cannot satisfy the roadmap's canopy-amplitude criterion or
turn the evergreen realization into `SUPPORTED` validation. The advancement
logic should say explicitly that CAL-07 may close as
`complete / bounded characterization / Order 7 advancement withheld`, while
absolute amplitude and quantitative evergreen-floor agreement remain
`NOT_EVALUATED`. No aggregate pass may erase this limitation.

### CAL07-PRA-004 — Major: NASA POWER is reanalysis forcing, not observed site meteorology

The package currently calls the daily minimum temperature “observed.” The
frozen POWER responses identify GEOSIT/MERRA2/POWER source lineage and are
gridded/reanalysis estimates. They are independent forcing data, but not
on-site measurements. Their grid elevations also differ materially from the
camera metadata:

- Alerce Costero: POWER geometry elevation `99.4 m`; site elevation `840 m`.
- Beza Mahafaly: POWER geometry elevation `277.79 m`; site elevation `165 m`.

The Alerce mismatch is especially large and can bias minimum temperature,
dew point, VPD, and hence GSI timing. Rename the forcing throughout, publish
the elevation and scale mismatch, and prohibit attribution of timing residuals
uniquely to hemisphere handling or GSI transferability. Without authoritative
site meteorology or an independently authorized downscaling operator, the
observational verdict remains bounded.

### CAL07-PRA-005 — Major: source admission is not yet fully reproducible

The retained files and digests are a good start, but the manifest names
PhenoCam ZIP files without their exact archive URLs and describes POWER points
without the complete API query. The retained site metadata also does not
contain ROI descriptions, so the evidence currently proves site vegetation
class plus `EN_1000`/`DB_1000` product identity, not why the selected pixels
represent the asserted canopy stratum.

Before execution, retain or cite the ROI metadata, exact source URLs/query
parameters, product/provisional status, processing-method citation, and both
site acknowledgments. Reconcile the stale noncommercial/share-alike wording
embedded in the generated metadata with the retained live fair-use page's
CC BY 4.0 statement, recording which source governs redistribution and why.
This is a provenance/admission requirement, not permission to reinterpret the
camera signal.

### CAL07-PRA-006 — Major: independent reconstruction must be operationally distinct

OBL-PLANT-P-013 requires

`1000 * (0.5 * (es(Tmax) + es(Tmin)) - es(Tdew))`

with `es(T)=0.6108*exp(17.27*T/(T+237.3))`, negative/non-finite rejection,
and no clamp. The proposed equation matches the current production lineage.
The package should pin those constants and source location, verify POWER units
are degrees Celsius, and fail on the response's `-999` fill value.

However, “independent VPD reconstruction” and foliar-mass reconstruction must
not merely call or restate the executor's implementation over its own derived
columns. Freeze a separate reconstruction over the raw forcing and daily
state operands, compare every row, and report maximum absolute residual and
inventory count. This is necessary for the planned closure claim.

### CAL07-PRA-007 — Note: lane independence is adequate but narrow

Beza Mahafaly and Alerce Costero are disjoint from the CAL-04B calibration
observations, occur on different continents, have signed negative latitudes,
span tropical dry (`Aw`) and cool wet temperate (`Cfb`) climates, and contain
multiple camera seasons. They therefore meet the roadmap's minimum spatial
and climate-region separation for an independent-validation assignment.

They share one camera-network method, use provisional products, omit high
southern and polar latitudes, and cover only one deciduous and one evergreen
site. The final report must not broaden them into biome-wide or global
Southern Hemisphere validation.

### CAL07-PRA-008 — Note: predecessor negative evidence is correctly binding

Preserving all 37 CAL-04B members without selection or weighting is required.
The poor Harvard transfer result remains contrary evidence and cannot be
offset by a favorable Southern Hemisphere aggregate. CAL-05's missing native
predictive needle/fine-woody authority and CAL-06's non-advanced residue,
frost, ET, runoff, and erosion cells also remain binding. CAL-07 may exercise
their runtime boundaries, but must keep the unavailable source terms and
downstream consequences `NOT_EVALUATED` or `NOT_ADVANCED`.

## Execution recommendation

Do not begin result-bearing execution under the current protocol. Correct
CAL07-PRA-001 and CAL07-PRA-002 and amend the protocol to bind the limitations
and provenance in CAL07-PRA-003 through CAL07-PRA-006. Then obtain a renewed
prospective concurrence and execute exactly once without changing sites,
members, observation filters, score operators, or advancement rules.

After those corrections, the design is fit for a bounded Southern Hemisphere
timing/phase evaluation. It is not, with these sources, fit to advance
absolute canopy amplitude, quantitative evergreen floor, global
transferability, or source-incomplete downstream physics.

## Prospective re-review after protocol amendment

Evidence class: `Static: amended package, frozen intent, source authority,
source manifest, ROI records, ensemble custody, input normalizer, executor,
and analyzer`

Re-review status: `HOLD BEFORE RESULT-BEARING EXECUTION`

No result-bearing CAL-07 artifact was present at re-review.

### Disposition of original findings

- `CAL07-PRA-001` is accepted and resolved at the claim boundary. The amended
  package treats the producer phase test and consumer-ordering test as
  separate compositional evidence and keeps phase-transformed real-consumer
  chronology `NOT_EVALUATED`. Order 7 advancement is explicitly withheld.
- `CAL07-PRA-002` is accepted and resolved in the written operator. Raw,
  quality-admitted GCC90 is joined on exact dates and independently min-max
  normalized per site, complete year, and member for 2024 and 2025. The
  minimum support, zero-range disposition, population Pearson correlation,
  RMSE, partial-year exclusion, threshold inequalities, linear interpolation,
  midpoint brackets, first-crossing rule, unmatched-event retention, and
  repeated-crossing count are now fixed.
- `CAL07-PRA-003` and `CAL07-PRA-004` are accepted and resolved at the claim
  boundary. Absolute amplitude and quantitative evergreen-floor agreement
  remain `NOT_EVALUATED`; POWER is correctly described as gridded/reanalysis
  forcing, both elevation mismatches are published, and no downscaling or
  site-representativeness claim is introduced.
- `CAL07-PRA-005` is substantially resolved. Exact archive/API endpoints,
  retrieval date, digests, ROI identity and descriptions, provisional status,
  acknowledgments, method citations, and the retained license-text conflict
  are present.
- `CAL07-PRA-006` is resolved in the written method but not yet in the
  executable evidence path; see `CAL07-PRA-R1` and `CAL07-PRA-R2`.
- `CAL07-PRA-007` and `CAL07-PRA-008` remain correctly binding limitations.

### CAL07-PRA-R1 — Blocker: verdicts are hard-coded before their gates run

`analyze.py` writes `SUPPORTED` for signed-latitude calendar/seasonal
direction, mass closure, and real-consumer ordering without ingesting phase or
consumer gate results. It also calculates the maximum closure residual but
does not fail when it exceeds `1e-12 kg m-2`. Thus a failed phase test,
consumer test, seasonal-direction comparison, or closure tolerance could
still produce a `SUPPORTED` row.

Before execution, make every result-dependent verdict conditional on its
actual evidence. At minimum:

- assert the daily closure tolerance before publishing closure support;
- ingest retained PASS evidence for the producer phase and consumer-ordering
  tests, or leave those cells pending until post-test synthesis;
- define and execute the claimed observed-seasonal-direction check rather
  than inferring it from the existence of two lanes; and
- prevent `verdict-matrix.csv` from containing `SUPPORTED` when any binding
  package gate failed or was not run.

### CAL07-PRA-R2 — Blocker: the closure output cannot safely carry the declared tolerance

The executor computes closure using full-precision values but serializes each
independent operand with 12 digits after the decimal. Reconstructing
`Bf_before + A_leaf - L_leaf - Bf_after` from those serialized operands can
accumulate rounding error above the declared `1e-12 kg m-2` tolerance. The
published residual computed before serialization is not an independent
reconstruction.

Serialize all closure and VPD operands with round-trip-safe precision and
have a distinct validator join raw forcing and daily output, reconstruct VPD
and mass closure, compare every row, and report inventory plus maximum
absolute residual. Do not use the executor's residual column as the sole
closure gate.

The transition table also writes an absolute Python ordinal into the field
named `observed_doy_50`. The residual calculation uses the absolute ordinal
correctly, but the published DOY field is false. Populate it from source
`doy_50` while retaining a separate internal ordinal for arithmetic.

### CAL07-PRA-R3 — Major: promised fail-closed input and inventory checks are incomplete

`prepare_inputs.py` rejects `NA`, flagged GCC, and POWER `-999`, but it does
not implement the prespecified finite-value checks for GCC or the three
forcing variables. Add explicit `math.isfinite` checks before normalized
inputs are written.

The analyzer confirms only the global set of 37 IDs and two site IDs. It does
not prove that every member-site pair contains every expected forcing day.
Add an exact per-member/site date-inventory comparison against
`forcing.csv`, including uniqueness and chronology, before scoring. The
37-row custody table and predecessor hashes otherwise provide the required
parameter-level binding.

### CAL07-PRA-R4 — Minor: describe V3 as method lineage, not file identity

The frozen 2024--2026 files are provisional archive products, whereas the
cited curated PhenoCam V3 dataset covers 2000--2023. Young et al. and the V3
DOI are appropriate processing/method lineage and the package already
preserves provisional status. Change “product/method authority is PhenoCam
Dataset V3” to distinguish the retained provisional products from the
curated V3 dataset itself.

### Re-review execution recommendation

`HOLD`. The narrowed scientific design, exact shape/crossing operators, lane
independence statement, proxy semantics, source custody, and advancement
ceiling are now fit for bounded evaluation. Result-bearing execution should
start only after `CAL07-PRA-R1` through `CAL07-PRA-R3` are corrected and
checked. `CAL07-PRA-R4` should be corrected with the same prospective
amendment.

## Final prospective executable re-review

Evidence class: `Static: current execute, normalize, kernel-executor,
analyzer, source-authority, custody, and artifact inventory`

Final prospective status: `HOLD BEFORE RESULT-BEARING EXECUTION`

No result-bearing CAL-07 artifact was present during this inspection.

### Corrected findings

- `CAL07-PRA-R1` is corrected for the two focused executable gates. The
  execution driver writes PASS only after each command returns successfully;
  the analyzer requires both named rows before publishing their supported
  cells. Signed-latitude/seasonal-direction evidence is conservatively
  `BOUNDED`, not hard-coded `SUPPORTED`.
- The executable inventory now requires exactly 74 member/site groups and
  1,666 daily rows in every group. The source forcing sequence is a
  date-keyed mapping, the executor emits one row after each chronological
  `GsiState` admission, and skipped/repeated dates fail in the kernel. This
  composition closes the requested complete-sequence gate.
- Rust daily values now use round-trip-safe precision, source DOY is
  published separately from the ordinal used for residual arithmetic, and
  GCC plus POWER forcing receive explicit finite checks. These correct the
  serialization, DOY, and fail-closed portions of `CAL07-PRA-R2` and
  `CAL07-PRA-R3`.
- `CAL07-PRA-R4` is corrected. The authority record now distinguishes V3
  processing/method lineage from the 2024--2026 provisional archive
  products.

### CAL07-PRA-F1 — Blocker: independent numeric reconstruction is still absent

The analyzer computes `max_closure` by reading the executor-produced
`mass_closure_residual_kg_m2` column. That residual was calculated inside the
same Rust producer from the same in-memory operands. Checking it against
`1e-12` is a producer self-check, not the independent reconstruction required
by the package or `CAL07-PRA-R2`.

No current tool independently:

- reconstructs each post-initial foliar state as previous serialized live
  foliar mass plus serialized allocation minus serialized litter;
- checks the first realized day's zero-transfer boundary;
- joins `forcing.csv` to every daily output row and independently recomputes
  VPD from raw `Tmax`, `Tmin`, and `Tdew`; or
- reports the maximum independent mass and VPD residuals with exact row
  inventory.

Round-trip serialization makes those checks possible but does not perform
them. Add them to `analyze.py` or a distinct prospective validator and make
failure binding before result execution. The executor's residual may remain
as a diagnostic comparison, but it cannot carry the independent closure
claim.

### Final execution recommendation

`HOLD` only for `CAL07-PRA-F1`. All other prospective scientific, source,
operator, evidence-ceiling, and executable findings are sufficiently
corrected for the bounded CAL-07 route. Once independent VPD and mass
reconstruction are implemented prospectively and re-inspected, result-bearing
execution may proceed without further scientific redesign.

## Independent-validator inspection and final prospective disposition

Evidence class: `Static: exact current tools/validate.py plus bound inputs and
executor schema`

Superseding final prospective status: `GO FOR RESULT-BEARING EXECUTION`

`CAL07-PRA-F1` is corrected. The operationally distinct Python validator:

- verifies every retained source object against manifest size and SHA-256;
- verifies the 37-row unique ensemble custody and execution order;
- joins each daily output row to raw `forcing.csv` by site and date;
- independently implements the saturation-vapor-pressure equation and
  reconstructs VPD from raw `Tmax`, `Tmin`, and `Tdew`;
- reconstructs post-initial foliar closure from the previous serialized live
  mass plus current serialized allocation minus current serialized litter;
- checks the executor's first-day and daily producer residual separately;
- requires exactly 123,284 daily rows, 74 member/site inventories, and 1,666
  rows in every inventory;
- binds maximum independent VPD and mass residuals to `1e-9 Pa` and
  `1e-12 kg m-2`, respectively; and
- preserves the three required `NOT_EVALUATED` ceilings before checking the
  figure/sidecar contract.

The separate implementation, raw-forcing join, sequential mass reconstruction,
round-trip serialization, complete inventory, and binding tolerances satisfy
the prospective independent-numeric gate. The validator necessarily runs
after execution and rendering because those are its inputs, but its algorithm
and thresholds are frozen before result creation.

No prospective blocker remains. CAL-07 may now execute exactly once under the
frozen operators. This `GO` authorizes only the package's bounded
Southern Hemisphere timing/phase characterization. Order 7 advancement,
absolute amplitude, quantitative evergreen floor, phase-transformed
real-consumer chronology, global transferability, and source-incomplete
downstream consequences remain expressly withheld.
