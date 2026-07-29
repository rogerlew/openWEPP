# CAL-07D Prospective Science Review A

Evidence class: `Static`

Review boundary: protocol and retained predecessor evidence only. No
result-bearing CAL-07D analyzer, counterfactual, or plot was run.

Initial verdict: `HOLD BEFORE RESULT-BEARING EXECUTION`

The proposed investigation is scientifically useful and correctly keeps all
counterfactuals diagnostic-only. The frozen ensemble, forcing, events, and
CP-GSI01 operator can support a defensible attribution screen. The following
items must be frozen before results are generated so the analysis cannot choose
operators or explanations after seeing outcomes.

## P0-1 — Separate observation progress from model state level

The package currently calls the `0.10,...,0.90` model annual-range grid a set
of relative observation operators, but the retained PhenoCam transition source
reports progress through a fitted seasonal transition. Those are not
automatically the same quantity. In particular, a falling PhenoCam
`date_10` is early in the decline and therefore corresponds, under the simple
annual-range analogy, to a high model state level (`0.90`), not `0.10`.

Before execution:

- rename the full model grid a `model-relative threshold sensitivity` screen;
- bind and retain the source `date_10`, `date_25`, and `date_50` fields rather
  than treating only `date_50` as the entire observation operator;
- define the direction-aware analogy explicitly: rising progress `p` maps to
  model level `p`, while falling progress `p` maps to model level `1-p`;
- either include `0.25` and `0.75` in the frozen model grid for the source
  25-percent dates or state why only the 10- and 50-percent source points are
  comparable;
- retain the source smoothed GCC and confidence-width context near every event,
  plus raw-GCC image support, gaps, and outlier status, without refitting the
  PhenoCam curve or inventing date uncertainty; and
- state that an annual model min/max is only a scale diagnostic and cannot
  validate GCC as GSI, foliage mass, LAI, or physiological activity.

Without this amendment, a change in match count across model levels cannot be
attributed to an observation-operator mismatch.

## P0-2 — Freeze crossing and event-pairing semantics completely

The event inventory and grid are not yet reproducible from the prose alone.
Before execution, specify:

- chronological sorting of all six Beza source events and the exact four
  internally bracketed events;
- the inherited non-overlapping bracket convention
  `lower_midpoint < crossing <= upper_midpoint`;
- the inherited crossing inequalities, linear fractional-day interpolation,
  first same-direction match rule, and prohibition on nearest-residual
  selection;
- whether relative thresholds are calculated from each
  member/scenario/calendar-year trajectory, and that 2024 and 2025 use complete
  modeled calendar years;
- the exact annual-level formula
  `annual_min + q * (annual_max - annual_min)`;
- zero-range handling as undefined/unmatched, never imputed;
- explicit rows for every matched, unmatched, multiple, and out-of-window
  crossing, with all-crossing and extra-in-bracket counts kept distinct; and
- exactly which operators apply to `BASE`, each constraint-removal scenario,
  and `SC_PLANT_GENERALIZED_DEFAULT`.

The validator should prove the BASE absolute-0.5 inventory reproduces the 148
CAL-07C rows, all 11 matches, their dates/residuals/counts, and the complete
out-of-window crossing inventory—not merely the aggregate number 11.

## P0-3 — Make the independent CP-GSI01 reconstruction exact

The reconstruction requirement needs enough detail to prevent an
almost-equivalent implementation. Freeze the following contract semantics:

- the ensemble's `*_inactive` and `*_unconstrained` columns map to the correct
  increasing temperature/photoperiod and decreasing VPD breakpoints;
- FAO-56 photoperiod uses signed latitude, runtime Gregorian day-of-year, the
  contract's `365` denominator even on day 366, and only the prescribed
  `acos`-argument clamp;
- each member starts with an empty FIFO on 2022-01-01, admits one real daily
  product at cold start, retains continuous chronology across year boundaries,
  and averages at most the trailing 21 real products;
- an unconstrained scenario evaluates the two unchanged BASE indicators and
  substitutes exactly `1` for only the named indicator before multiplication;
- the combined scenario substitutes exactly the two named indicators; and
- `SC_PLANT_GENERALIZED_DEFAULT` changes all six thresholds to the canonical
  defaults but otherwise uses the same forcing, dates, geometry, FIFO, and
  crossing rules.

Require unique complete forcing/member/date inventories, finite bounded
indicators, and rowwise agreement for all 61,642 BASE Beza rows within
`1e-12`. The independent validator must reconstruct the equations and FIFO
rather than validate analyzer output through a shared computational helper.

For “smallest constraint” frequencies, define tie handling prospectively.
Prefer retaining a tie-set/category so frequencies do not silently double
count tied minima.

## P0-4 — Replace qualitative dominance labels with frozen evidence predicates

Terms such as “restore chronology,” “dominant,” “explain,” and “cannot
distinguish” currently have no numerical or logical predicate. This permits
terminal statuses to be selected after results are visible. No defensible
timing-accuracy tolerance has been admitted, so an arbitrary residual cutoff
must not be introduced merely to close the screen.

Before execution, publish a machine-readable decision rule for every status.
At minimum:

- report match counts and signed residual distributions by event, direction,
  member, operator, and scenario;
- distinguish `SUPPORTED_AS_CONTRIBUTOR`, `PLAUSIBLE`, `CONTRADICTED`, and
  `UNRESOLVED` from causal or parameter-identification claims;
- do not label the observation operator `DOMINANT` solely because some
  relative level creates a crossing;
- do not label transferred thresholds `DOMINANT` solely because setting an
  indicator to the nonphysical attribution value one creates a crossing;
- keep forcing bias versus physiology unresolved in the absence of admitted
  site meteorology; and
- keep missing process only `PLAUSIBLE` when the same evidence is also
  consistent with forcing bias or transferred thresholds.

A defensible default is to reserve all `*_DOMINANT` labels for a later package
with an admitted discrimination rule and use contribution/plausibility
statuses here.

## P1-5 — Freeze required outputs and solution-evidence routes

Name the machine-readable outputs before execution. They should expose at
least:

- exact dependency/source hashes and schemas;
- daily BASE and scenario indicators/products/GSI21;
- every crossing and every event-pairing result;
- event-period forcing, indicator, and 21-day summaries;
- model-level/source-progress comparison results;
- scenario deltas and decision predicates; and
- an additional-evidence inventory.

The additional-evidence inventory should map each viable route to what would
actually discriminate it: on-site temperature/humidity for forcing bias;
raw images, fit metadata, and transition uncertainty for observation
semantics; an independently reserved tropical dry-forest site/year for
threshold transfer; and rainfall, soil-water, hydraulic-state, or
site-phenology authority for a missing-process route. Literature or new data
may motivate a contract-first follow-on, but cannot become production science
through this diagnostic package.

## Required gate before release

After the P0 amendments, both prospective reviewers should re-review the exact
protocol. Result-bearing analysis may proceed only on dual `GO`. Order 7 must
remain held regardless of diagnostic counterfactual improvement unless
admitted evidence resolves the existing contradiction.

## Amended-protocol re-review

Evidence class: `Static`

Re-review boundary: amended `package.md` and
`calibration-readiness-matrix.md`, checked against both prospective reviews.
No result-bearing CAL-07D analysis was run.

Current verdict: `HOLD BEFORE RESULT-BEARING EXECUTION`

The amendment resolves most of both reviewers' findings:

- source progress and model state level are now distinguished;
- the direction-aware analogy, retrospective event-year threshold, fixed
  event-window threshold, zero-range handling, crossing inequalities,
  interpolation, plateau behavior, and non-nearest pairing rule are frozen;
- expected event/scenario inventories and generalized-default deduplication
  are explicit;
- CP-GSI01 daylength, FIFO, scenario-isolation, tie, and independent
  reconstruction rules are explicit;
- qualitative `DOMINANT` labels were removed and deterministic,
  non-identifying sensitivity predicates were added;
- required output tables, minimum figures, sidecar bindings, and independent
  validation checks are named; and
- execution-dependent readiness rows are now truthfully `PENDING`.

Four narrow pre-execution corrections remain.

### Remaining P0-A — The falling 25-percent source analogy has no frozen model level

The direction-aware rule maps falling progress `p=0.25` to model state level
`q=0.75`, but the frozen model-level grid contains `0.25` and not `0.75`.
The declared 444-row source-progress inventory therefore cannot be derived
unambiguously from the frozen grid.

Add `0.75` to the grid, or state explicitly that the source-progress audit
computes direction-aligned levels independently of the broader grid and
includes `q=0.75` for falling 25-percent events. Validate all 444 rows against
the direction-aware mapping.

### Remaining P0-B — Bind the actual source objects needed by the expanded audit

CAL-07C's normalized `transitions.csv` retains only `date_50`, and its
normalized observations do not provide the full source-smoothed,
confidence-width, and image-support context now required. The amended audit
therefore depends on the predecessor Beza one-day product and simplified
transition product, not only the CAL-07C inputs/results named by the current
dependency-hash gate.

Name those two retained source paths explicitly and require their exact
SHA-256/size identities at commit
`11b1faab37b5d365b0c0c7051ed32a4762821239` in the dependency manifest.
Validation must parse the comment-prefixed one-day CSV deterministically and
prove the source `date_10`, `date_25`, and `date_50` values used by the audit.
No alternate observed transition may be regenerated or selected.

### Remaining P0-C — Complete the machine-readable crossing key

The event-screen keys and counts are now explicit, but `all-crossings.csv`
still lacks a declared unique key and expected scope. Freeze a key that
distinguishes scenario, member or generalized default, operator/level,
direction, and chronological crossing index. State whether it contains the
complete 1,666-day crossing inventory for every executed trajectory/operator
or only BASE absolute/model-level crossings. The validator must independently
reconstruct that exact scope and prove no crossing is dropped because it is
outside an event window.

### Remaining P1-D — Keep threshold-transfer wording commensurate with the experiment

Setting an entire indicator to one shows mathematical constraint sensitivity;
it does not isolate the transferred threshold values from the indicator's
functional form, forcing bias, or a missing cue. The proposed
`TRANSFERRED_THRESHOLD_PLAUSIBLE_UNRESOLVED` status is appropriately cautious,
but its predicate can be read as stronger than its evidence.

Either rename it
`TRANSFERRED_PARAMETERIZATION_OR_CUE_PLAUSIBLE_UNRESOLVED`, or add an explicit
machine-readable claim ceiling that the predicate does not separately support
threshold transfer. The additional-evidence table should retain the minimum
discriminating routes named by Review B: on-site temperature/humidity or VPD
and precipitation; soil-water or plant-water status; image/ROI or field
phenology corroboration; site/ecotype cue literature; and an independently
reserved Southern Hemisphere deciduous site or years.

Once these corrections are frozen and both prospective reviewers record
`GO`, bounded result execution is scientifically legitimate. That `GO` will
authorize only diagnostic execution; it will not identify a replacement
parameter, validate new process physics, resolve Order 7, or lift its hold.

## Final corrected-protocol review

Evidence class: `Static`

Final verdict: `GO FOR BOUNDED DIAGNOSTIC EXECUTION`

No result-bearing CAL-07D analysis had run when this verdict was recorded.

The final correction closes every prospective Review A blocker:

- retained falling source dates cross normalized levels in the order `0.50`,
  `0.25`, then `0.10` as greenness declines, so the corrected bounded analogy
  `q=p` for both directions is consistent with the actual source-level
  ordering;
- the frozen grid now includes `0.75` as part of the broader sensitivity
  inventory while the source-level audit explicitly uses the retained
  `0.10`, `0.25`, and `0.50` levels;
- the exact comment-prefixed Beza one-day and normalized-transition source
  objects are SHA-256 bound, their parsing is specified, and regeneration or
  replacement of observed transition dates is prohibited;
- `all-crossings.csv` now has a complete operator/scenario scope and unique
  key, while mandatory event rows preserve no-crossing and unmatched cases;
- the decision screen now reports current-GSI constraint sensitivity without
  implying that constraint removal identifies transferred thresholds; and
- the minimum evidence needed to discriminate forcing, observation,
  parameter/ecotype, and missing-process routes is frozen.

Disposition of earlier findings:

| Finding | Final disposition |
| --- | --- |
| P0-1 / P0-A: source level versus model level | `RESOLVED` |
| P0-2: crossing and event semantics | `RESOLVED` |
| P0-3: independent CP-GSI01 reconstruction | `RESOLVED` |
| P0-4 / P1-D: decision predicates and claim ceiling | `RESOLVED` |
| P1-5: outputs and discriminating evidence routes | `RESOLVED` |
| P0-B: expanded source custody | `RESOLVED` |
| P0-C: complete crossing inventory | `RESOLVED` |

Execution remains bounded by the protocol's explicit assumptions and
validation gates. A successful run may identify mathematical contributors,
sensitivity, and unresolved solution routes. It cannot establish causal
physiology, calibrate Beza parameters, authorize a rainfall or soil-water
process, validate GCC as canopy state, resolve the chronology contradiction,
or advance Order 7 by itself.
