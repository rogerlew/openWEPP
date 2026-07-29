# CAL-07D Prospective Scientific Review B

Evidence class: `Static`

Review scope: the CAL-07D scaffold and prespecified protocol, frozen CAL-07
and CAL-07C evidence, `SC-PLANT-001` CP-GSI01, ADR-0042, and applicable
repository instructions. No CAL-07D result-bearing analysis was reviewed.

Recommendation: `HOLD BEFORE RESULT EXECUTION`

The package has an appropriate diagnostic-only boundary and correctly forbids
refitting, production changes, invented physiology, and Order 7 advancement
from counterfactuals. Result execution should wait until the reproducibility
and attribution gaps below are frozen in the protocol and independently
re-reviewed.

## Findings

### CAL07D-PRB-001 — The observation-operator audit omits admitted source information

The retained Beza transition product contains source-provided `date_10`,
`date_25`, and `date_50` values. CAL-07D currently audits only `date_50`, while
its model threshold grid uses 0.10 increments and therefore omits the exact
source 0.25 level. This is an avoidable loss of information in the diagnostic
most directly intended to test operator scale.

Before execution:

- bind the exact retained transition source digest and carry all 10%, 25%,
  and 50% dates for the four internally bracketed events;
- include model-relative thresholds 0.10, 0.25, and 0.50 explicitly, whether
  or not the broader sensitivity grid is retained;
- report matched, unmatched, multiple, and out-of-window results separately
  by observed transition level; and
- state that the 10--50% date span is a source transition-progression
  diagnostic, not a confidence interval or physiological uncertainty bound.

The raw quality-filtered GCC90 series, the source-smoothed series used by the
transition product, and the simplified transition dates must remain distinct.
CAL-07D may inspect their temporal support and visible curve consistency, but
must not regenerate a preferred observed transition after seeing model
residuals.

### CAL07D-PRB-002 — Calendar-year relative thresholds can create a false boundary crossing

The protocol does not say whether a calendar-year relative threshold is
recomputed on each modeled day or frozen for the observed event. An austral
adjacent-event midpoint window can span January 1. If the threshold changes
from the prior year's range to the event year's range while crossings are
enumerated, a discontinuity in the diagnostic threshold can appear as a model
transition even when `GSI21` itself does not cross a fixed level.

Freeze the relative operator before execution. For example, for each
member/event/level, calculate

```text
threshold = min(GSI21 in event calendar year)
          + level * (max(GSI21 in event calendar year)
                   - min(GSI21 in event calendar year))
```

and apply that one constant threshold throughout the event's full
adjacent-event midpoint window. Label its use of the complete calendar year,
including future dates relative to an event, as retrospective diagnostic
information rather than a predictive operator. A zero annual range must
produce an explicit undefined/unmatched result.

Also freeze the crossing inequalities, linear interpolation, plateau
behavior, lower/upper window inclusivity, chronological ordering, and
selection rule for every relative level. They should follow the CAL-07C
non-nearest rule unless a prospective justification says otherwise. Crossing
enumeration must never compare adjacent days evaluated against two different
threshold values.

### CAL07D-PRB-003 — Expected inventories and counterfactual score operators are underspecified

The requirement for “exactly 148 rows for every prespecified observation
operator/scenario where that inventory applies” is not sufficiently
machine-checkable. The threshold grid multiplies the event/member inventory,
and the constraint-removal section does not state which threshold operator,
window, or event-pairing rule determines its chronology result.

Before execution, publish an expected-inventory table that names the exact
row count and unique key for:

- the absolute 0.5 reproduction;
- every model-relative threshold level;
- each constraint-removal scenario; and
- the generalized-default scenario.

For every scenario, specify whether chronology uses absolute `GSI21=0.5`,
the event-year relative midpoint, the full relative grid, or more than one of
those operators. Require one row even when a crossing is absent, and retain
all additional crossings in a separate complete inventory.

`SC_PLANT_GENERALIZED_DEFAULT` also needs an exact definition. Replacing all
six thresholds with CP-GSI01 defaults yields one forcing trajectory repeated
37 times, not 37 independent ensemble members. Emit it once or label and
validate the intentional duplication. Each unconstrained scenario must
recompute instantaneous products and the 21-day FIFO from the first frozen
forcing date; it must not edit the committed kernel output or reuse a base
`GSI21` history.

### CAL07D-PRB-004 — “Dominant” solution labels lack prospective decision predicates

The current decision screen uses “restore chronology,” “dominant,” and
“explain the discrepancy” without a tolerance or deterministic reduction.
No admitted accuracy threshold exists. A smallest-indicator frequency is also
not, by itself, causal attribution: ties are common at zero, and the
multiplicative product can remain suppressed after the smallest constraint is
removed.

Before execution:

- define tie handling for the daily smallest-indicator inventory and retain
  the tied constraint set rather than silently choosing one;
- report counterfactual effect sizes on instantaneous GSI, `GSI21`, crossing
  availability, and crossing displacement, not only constraint ranks;
- replace `OBSERVATION_OPERATOR_DOMINANT` with an explicitly bounded
  sensitivity label, or define a deterministic predicate that requires all
  prespecified events and members and does not invent a timing tolerance;
- likewise use `TRANSFERRED_THRESHOLD_SENSITIVE` or retain
  threshold transfer as `PLAUSIBLE/UNRESOLVED` unless independent
  ecotype-threshold authority separates parameter transfer from forcing bias
  and missing process cues; and
- encode every decision label as a machine-readable predicate consumed by the
  validator, including the permitted multiple-label outcome.

Constraint removal can identify mathematical suppression under the current
equation. It cannot establish that the removed constraint is biologically
wrong, that the gridded forcing is unbiased, or that a replacement parameter
or process is authoritative.

### CAL07D-PRB-005 — Forcing and missing-process alternatives cannot be separated with current evidence

Beza forcing is a POWER gridded/reanalysis daily product, not on-site
meteorology. Its grid elevation differs from the camera site, and CAL-07D has
no rainfall, plant-available water, soil moisture, or physiological
observations. The planned event-period indicators can therefore show when the
implemented model is suppressed, but they cannot distinguish a forcing bias
from ecotype physiology or a missing water-availability cue.

The final evidence screen must preserve that non-identifiability. The required
additional-observation artifact should name, at minimum:

- quality-controlled on-site minimum temperature, humidity or VPD, and
  precipitation spanning complete leaf-off and leaf-on seasons;
- a soil-water or plant-water-status measure at a defensible rooting/canopy
  scale;
- image/ROI review or field phenology corroborating the source transitions;
- site/ecotype literature establishing the environmental cues for tropical
  dry-forest flushing and senescence; and
- another independently assigned Southern Hemisphere deciduous site or held
  out years for any later transferability test.

No rainfall or soil-water equation should be introduced by CAL-07D. If source
authority later establishes a missing cue, that is a separate contract-first
process package.

### CAL07D-PRB-006 — Figure and independent-validation obligations need exact bindings

The general figure/sidecar requirements are sound, but the package should
freeze a minimum visual inventory that prevents aggregate summaries from
hiding failures:

1. an event-by-member crossing map showing unmatched and out-of-window
   outcomes across absolute and source-aligned relative levels;
2. an event-centered 21-day indicator/product figure with ties and ensemble
   spread visible; and
3. a constraint-removal chronology/effect-size figure that distinguishes
   mathematical sensitivity from evidence-backed solutions.

Each Markdown sidecar should name the exact CSV binding and digest, row keys,
sample counts, event windows, unmatched encoding, source/operator level,
execution assumptions, evidence ceiling, and accessible interpretation.

Independent validation must reconstruct equations from retained forcing and
parameter rows rather than import analyzer helpers. It must verify:

- exact ordered date equality for all 61,642 Beza rows;
- first-20-day available-sample FIFO semantics and all later 21-day windows;
- unique event/member/operator/scenario keys and the prospectively declared
  inventory counts;
- CAL-07C reproduction of all 11 matches and every original residual, not
  only the total count;
- explicit blank/undefined encoding without NaN, imputation, or row loss;
- absence of threshold discontinuity at year boundaries;
- scenario isolation to the declared indicator or full default parameter
  substitution;
- figure values against bound result tables; and
- source/dependency digests against commit
  `11b1faab37b5d365b0c0c7051ed32a4762821239`, not merely the mutable
  predecessor paths in the working tree.

### CAL07D-PRB-007 — Scaffold readiness rows claim execution evidence prematurely

The calibration-readiness matrix currently marks deterministic execution,
observation-operator definition, and sensitivity/boundary reporting `PASS`
before the package-local scripts or result artifacts exist. That conflicts
with the scaffold's `Static + planned Ran` evidence mode.

Mark execution-dependent rows as prospective/pending during the scaffold
phase, then reduce every required final row to `PASS`, `BLOCKED`, or
`NOT_APPLICABLE` with an evidence path. Any required `BLOCKED` row must force
the package disposition required by `docs/work-packages/AGENTS.md`.
`NOT_APPLICABLE` for empirical calibration is acceptable only while the
package continues to state clearly that all measured evidence is
`DIAGNOSTIC_ONLY` and that CAL-07D does not claim calibration readiness for a
future optimization workflow.

## Conditions to lift this hold

1. Bind and prespecify the complete source-aligned 10%, 25%, and 50%
   observation-operator audit.
2. Freeze one constant event-specific relative threshold and exact crossing
   rules so austral year boundaries cannot manufacture events.
3. Define scenario operators, unique keys, expected inventories, default
   handling, and full-history recomputation.
4. Replace qualitative dominance language with deterministic,
   authority-bounded predicates and explicit non-identifiability.
5. Freeze the minimum figure/sidecar inventory and independent validation
   checks, and correct the premature readiness statuses.
6. Obtain renewed independent prospective review before any result-bearing
   analyzer execution.

After these changes, Review B can authorize bounded diagnostic execution.
Such authorization would not validate a production correction, identify a
site parameterization, resolve Order 7, or establish that threshold transfer,
forcing, or missing process physics is the biological cause.

## Amended-protocol re-review

Evidence class: `Static`

Re-review scope: amended `package.md` and
`artifacts/calibration-readiness-matrix.md`. No result-bearing CAL-07D
analysis was reviewed.

Superseding recommendation: `HOLD BEFORE RESULT EXECUTION`

The amendment resolves most of the original protocol gaps. One
observation-operator defect remains scientifically material because it
reverses the source-level analogy for falling events.

| Finding | Disposition | Amended-protocol evidence |
| --- | --- | --- |
| `CAL07D-PRB-001` | `PARTIALLY RESOLVED / BLOCKING` | The protocol now binds and retains source 10%, 25%, and 50% dates, includes model levels 0.10, 0.25, and 0.50, separates raw/smoothed/support evidence, and withholds physiological authority. The falling-event mapping is reversed, as detailed below. |
| `CAL07D-PRB-002` | `RESOLVED` | One event-year threshold is frozen over the full adjacent-event window; complete-year retrospective use, zero range, January 1 behavior, inequalities, plateaus, interpolation, window inclusivity, first-crossing selection, and global/in-window/out-of-window counts are explicit. |
| `CAL07D-PRB-003` | `RESOLVED` | Unique keys and exact row inventories are declared for absolute, model-level, source-progress, five ensemble scenarios, canonical default, BASE daily, and scenario daily surfaces. The canonical default is emitted once, and full FIFO recomputation is required. |
| `CAL07D-PRB-004` | `RESOLVED` | `DOMINANT` and accuracy language are removed. Tie sets are retained; contributor/plausibility predicates and their inputs are machine-readable; multiple labels and non-identifiability are explicit. |
| `CAL07D-PRB-005` | `RESOLVED` | POWER remains gridded evidence, forcing/physiology/missing process remain non-identifiable, and an exact additional-evidence artifact is required before closure. |
| `CAL07D-PRB-006` | `RESOLVED` | Four minimum figures, exact sidecar bindings, complete crossing visibility, independent equation/FIFO/event reconstruction, declared inventories, CAL-07C rowwise reproduction, scenario isolation, and year-boundary checks are now mandatory. Terminal verification must still prove plotted values agree with their digest-bound CSVs. |
| `CAL07D-PRB-007` | `RESOLVED` | Execution-dependent readiness rows are now `PENDING`; they must reduce to `PASS`, `BLOCKED`, or `NOT_APPLICABLE` with result evidence before disposition. |

### Remaining blocker — falling source levels are mapped in reverse

The amended protocol says that falling source progress `p` maps to model level
`q=1-p`. The retained Beza source does not support interpreting `date_10` and
`date_25` that way. Its falling rows are chronologically ordered:

```text
2024: date_50=05-21, date_25=06-13, date_10=06-27
2025: date_50=05-01, date_25=05-23, date_10=06-16
```

As the canopy declines, the source crosses 50% before 25% before 10%. The
source fields therefore behave as remaining normalized greenness levels, not
elapsed decline-progress fractions. Mapping them to `q=0.50`, `q=0.75`, and
`q=0.90` would reverse that observed chronology and could manufacture an
observation-scale conclusion.

Before execution, use the source-aligned state-level analogy `q=p` for both
rising and falling events, while retaining direction-specific crossing
inequalities. Alternatively, cite retained source-method authority that
establishes a different semantic mapping and reconcile it explicitly with the
actual falling date order. Update the terminology from “source progress” to
“source normalized transition level” unless that authority proves that
“progress” is the correct meaning.

The following dependent protocol surfaces must change together:

- the direction-aware analogy in `package.md`;
- the source-aligned observation-scale decision predicate;
- output field names and sidecar terminology;
- validator expectations for falling 10%, 25%, and 50% mappings; and
- any expected crossing-level inventories affected by the mapping.

After this correction, Review B's recommendation can become
`GO FOR BOUNDED DIAGNOSTIC EXECUTION`, provided the exact amended protocol is
re-reviewed before result generation. That GO would retain all existing
authority ceilings and the Order 7 hold.

## Final corrected-tree prospective decision

Evidence class: `Static`

Final recommendation: `GO FOR BOUNDED DIAGNOSTIC EXECUTION`

This decision supersedes Review B's earlier `HOLD` recommendations. No
result-bearing CAL-07D analysis was inspected or used to set this decision.

The exact corrected protocol resolves the remaining
`CAL07D-PRB-001` blocker:

- source normalized transition level `p` now maps to model level `q=p` for
  both rising and falling events;
- direction remains encoded only in the frozen rising/falling crossing
  inequalities;
- the retained falling order
  `date_50 < date_25 < date_10` is stated explicitly;
- the source-level audit, keys, and 444-row inventory are frozen;
- the broader model sensitivity grid contains all source levels as well as
  0.75, with an exact 1,628-row inventory; and
- the protocol forbids regenerated, refitted, shifted, or selectively
  replaced observed transition dates.

All seven Review B findings are now resolved prospectively:

| Finding | Final disposition |
| --- | --- |
| `CAL07D-PRB-001` | `RESOLVED` |
| `CAL07D-PRB-002` | `RESOLVED` |
| `CAL07D-PRB-003` | `RESOLVED` |
| `CAL07D-PRB-004` | `RESOLVED` |
| `CAL07D-PRB-005` | `RESOLVED` |
| `CAL07D-PRB-006` | `RESOLVED` |
| `CAL07D-PRB-007` | `RESOLVED` |

The final protocol also binds the exact source hashes and comment-prefixed
CSV parsing, freezes the complete all-crossing operator/key scope, removes a
threshold-transfer implication from the decision screen, and names the
minimum discriminating evidence for forcing, observation, ecotype-parameter,
and missing-process routes.

One non-blocking editorial cleanup remains: the section heading
“Observation-progress and model-level audit” and the decision predicate phrase
“source-progress-aligned model level” should say “source-level” for
terminology consistency. The operative `q=p` equation, source order, output
name, keys, and inventory are unambiguous, so this stale wording does not
alter the frozen analysis and does not withhold GO.

This GO authorizes only the prespecified package-local diagnostic execution.
It does not authorize parameter fitting, a production threshold or forcing
change, new process physics, empirical calibration, predictive validation,
causal attribution, Order 7 advancement, or replacement of CP-GSI01
authority. Execution must retain the package's `ASSUMED_FOR_EXECUTION`
labels, machine-readable non-identifiability, independent validation,
figure/sidecar bindings, terminal dual review/verification, and Order 7 hold.
