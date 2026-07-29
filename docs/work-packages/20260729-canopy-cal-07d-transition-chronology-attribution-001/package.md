# CANOPY-CAL-07D Transition Chronology Attribution

Status: `complete / diagnostic attribution / Order 7 hold retained`

Evidence mode: `Static + Ran`

Intent: `diagnostic-only calibration-readiness investigation`

## Objective

Identify why the frozen CAL-04B GSI ensemble does not reproduce the
prespecified Beza Mahafaly deciduous transition chronology after CAL-07C
lifted the Alerce forcing-domain blocker. Separate observation-operator scale,
event pairing, transferred parameter thresholds, admitted forcing, and
potential missing-process explanations without refitting or changing
production science.

## Scientific question

CAL-07C found only 11 same-direction matches among 148 Beza member/event
comparisons. All matches were falling transitions and occurred early; the
prespecified rising transitions were unmatched. CAL-07D asks whether this
result is primarily caused by:

1. comparing an absolute `GSI21=0.5` crossing with a relative PhenoCam
   seasonal 50-percent transition;
2. event-window or calendar-year pairing;
3. one or more GSI constraint indicators suppressing the observed transition;
4. non-transferable Northern Hemisphere threshold combinations;
5. gridded forcing limitations; or
6. a process cue outside the current minimum-temperature, VPD, and
   photoperiod GSI structure.

## Frozen authority and evidence

- CAL-07C commit `11b1faab37b5d365b0c0c7051ed32a4762821239`.
- CAL-07C package-local forcing, observations, transition table, frozen
  37-member ensemble, and 123,284 daily kernel rows.
- CAL-07 prospective event eligibility and deterministic adjacent-event
  midpoint windows.
- `SC-PLANT-001` CP-GSI01 equations, units, 21-day window, signed-latitude
  photoperiod, guards, and explicit parameter meaning.
- ADR-0042 separation of implemented science, measured-data authority, and
  calibration readiness.

All PhenoCam observations and transition products are assigned
`DIAGNOSTIC_ONLY`. They are not calibration or independent-validation data in
this package.

## Prespecified diagnostics

### Observation-level and model-level audit

For every frozen member and the four internally bracketed Beza events:

- reproduce the CAL-07C absolute `GSI21=0.5` crossing;
- enumerate all absolute crossings, including crossings outside the event
  window;
- retain the source `date_10`, `date_25`, and `date_50` normalized-transition
  dates, source-smoothed GCC90 and confidence-width context, and quality-filtered
  raw GCC90 support within 21 days on either side of each source date;
- evaluate a prospectively frozen model-relative threshold grid
  `0.10,0.20,0.25,0.30,0.40,0.50,0.60,0.70,0.75,0.80,0.90`; and
- retain unmatched, multiple, and out-of-window crossings explicitly.

For a member, event, and model-relative level `q`, freeze one constant
retrospective threshold over the event's complete adjacent-event window:

```text
threshold =
    min(GSI21 during the event calendar year)
    + q * (
        max(GSI21 during the event calendar year)
        - min(GSI21 during the event calendar year)
      )
```

The event-year extrema use the complete modeled calendar year, including dates
after the event, and are therefore retrospective diagnostics rather than
predictive operators. Zero annual range is explicitly undefined and unmatched.
The threshold is never recomputed at January 1 or on each crossing day.

PhenoCam normalized transition level and model state level remain different
quantities. For a bounded source-aligned analogy only, source level `p` maps to
model level `q=p` for both directions, while crossing inequalities retain the
source direction. This matches the retained falling-event order: the series
crosses `0.50`, then `0.25`, then `0.10` as greenness declines. The source's
10-to-50-percent date span is a normalized transition-level span, not a
confidence interval or physiological uncertainty.

The threshold grid and direction-aware analogy are
`ASSUMED_FOR_EXECUTION`. They are scale and pairing diagnostics, not
physiological authority, fitted observation operators, validation of GCC as
GSI/foliage/LAI, or candidate production thresholds.

### Crossing and event semantics

- Sort all six source Beza `date_50` events chronologically; evaluate only the
  four events with both a predecessor and successor.
- Define each event window from adjacent `date_50` midpoints and admit
  `lower_midpoint < crossing <= upper_midpoint`.
- Rising uses `old < threshold <= new`; falling uses
  `old >= threshold > new`.
- Interpolate a crossing linearly in fractional Gregorian ordinal days.
- A plateau at the threshold creates no crossing until one of the strict
  inequalities is satisfied.
- Enumerate every crossing over the complete 1,666-day trajectory. Within an
  event window, select the first chronological same-direction crossing. Never
  select the nearest residual.
- Retain total global, in-window, extra-in-window, and out-of-window crossing
  counts separately. Emit one event row even when no crossing matches.

Expected machine-readable inventories:

| Artifact surface | Unique key | Rows |
| --- | --- | ---: |
| Absolute CAL-07C reproduction | member, event | 148 |
| BASE model-level sensitivity | member, event, model level | 1,628 |
| Source-level analogy | member, event, source level | 444 |
| Scenario absolute and relative-midpoint screen for five 37-member scenarios | scenario, member, event, operator | 1,480 |
| Canonical generalized-default scenario | scenario, event, operator | 8 |
| BASE member-day reconstruction | member, date | 61,642 |
| Daily scenario ensemble summary | scenario, date | 9,996 |

The five 37-member scenarios are `BASE` and the four indicator-removal
scenarios. `SC_PLANT_GENERALIZED_DEFAULT` is emitted once because its
trajectory is parameter-independent under common forcing.

### Indicator attribution

Independently reconstruct daily `iTmin`, `iVPD`, `iPhoto`, instantaneous GSI,
and the exact 21-day trailing arithmetic mean for Beza from the frozen forcing
and parameter ensemble. Prove reconstructed `GSI21` agrees with CAL-07C output
within `1e-12`.

For each observed transition, publish:

- forcing values and photoperiod;
- ensemble quantiles for all three indicators and instantaneous/GSI21 values;
- the frequency with which each indicator is the smallest constraint;
- preceding 21-day indicator/product context; and
- comparison with all modeled crossing dates.

The reconstruction maps `inactive`/`unconstrained` columns exactly to the
contract's increasing temperature/photoperiod and decreasing VPD indicators.
FAO-56 uses signed latitude, Gregorian runtime day, the contract's `365`
denominator on leap day, and only the prescribed `acos`-argument clamp. Every
trajectory starts with an empty FIFO on 2022-01-01, admits one real sample,
maintains continuous chronology across year boundaries, and averages the
available trailing samples up to 21.

Smallest-constraint attribution retains the sorted tie set. A tied day is one
tie category and is not silently assigned to multiple individual indicators.

### Constraint-removal counterfactuals

Recompute the diagnostic GSI trajectory with:

- `BASE`;
- `TEMPERATURE_UNCONSTRAINED`;
- `VPD_UNCONSTRAINED`;
- `PHOTOPERIOD_UNCONSTRAINED`;
- `PHOTOPERIOD_AND_VPD_UNCONSTRAINED`; and
- `SC_PLANT_GENERALIZED_DEFAULT`.

An `UNCONSTRAINED` scenario sets the named daily indicator to one only for
attribution. These scenarios are `ASSUMED_FOR_EXECUTION`; they are not
authorized parameter values, process replacements, calibration candidates, or
production recommendations. The generalized default is canonical equation
test authority, not site calibration authority.

### Solution decision screen

Classify quantitative sensitivity without selecting a production correction.
No timing-accuracy tolerance or `DOMINANT` label is admitted.

- `OBSERVATION_SCALE_SUPPORTED_AS_CONTRIBUTOR` when at least one
  member/event unmatched under absolute `0.5` becomes matched under its
  source-level-aligned model level without changing the BASE trajectory;
  otherwise `NOT_SUPPORTED_BY_SCREEN`.
- Each indicator receives
  `SUPPORTED_AS_MATHEMATICAL_CONTRIBUTOR` when setting only that indicator to
  one changes event-date `GSI21`, event-window match availability, or selected
  crossing date for at least one member/event under the same operator;
  otherwise `NOT_SUPPORTED_BY_SCREEN`.
- `CURRENT_GSI_CONSTRAINT_SENSITIVE_UNRESOLVED` when any single-indicator
  scenario increases same-operator matched rows relative to BASE. It does not
  isolate transferred thresholds from forcing bias, physiology, or missing
  cues and never identifies a replacement threshold.
- `FORCING_LIMITATION_PLAUSIBLE_UNRESOLVED` remains true because Beza forcing
  is gridded POWER rather than admitted on-site meteorology.
- `MISSING_PROCESS_PLAUSIBLE_UNRESOLVED` when a source rising transition
  occurs while BASE median instantaneous GSI is below `0.5` and at least two
  median indicators are below `0.5`. This remains non-causal and compatible
  with forcing bias or transferred thresholds.

Every status, input statistic, and predicate result is machine-readable.
Multiple contributor/plausibility labels are expected; forcing bias,
physiology, and missing process remain non-identifiable with current evidence.

The additional-evidence inventory maps each solution route to its minimum
discriminating evidence:

- forcing bias: quality-controlled on-site minimum temperature, humidity/VPD,
  and precipitation over complete leaf-off and leaf-on seasons;
- observation semantics: raw image/ROI review, fit metadata, source method,
  and transition uncertainty or field corroboration;
- parameter/ecotype transfer: admitted tropical dry-forest threshold
  authority plus an independently reserved site or held-out years; and
- missing process: rainfall, soil-water or plant-water-status observations
  at defensible scales plus site/ecotype phenology literature establishing the
  cue before any contract-first process proposal.

## Required outputs

- `artifacts/dependency-manifest.csv`
- `artifacts/base-member-daily.csv`
- `artifacts/daily-scenario-ensemble.csv`
- `artifacts/all-crossings.csv`
- `artifacts/absolute-reproduction.csv`
- `artifacts/model-level-sensitivity.csv`
- `artifacts/source-level-audit.csv`
- `artifacts/scenario-event-screen.csv`
- `artifacts/event-indicator-attribution.csv`
- `artifacts/observation-support.csv`
- `artifacts/decision-screen.csv`
- `artifacts/additional-evidence-needed.csv`
- `artifacts/result-manifest.csv`

`all-crossings.csv` covers every operator evaluated by the event tables:
BASE absolute `0.5`, all eleven BASE event-year relative levels, absolute and
relative-midpoint operators for each indicator-removal scenario, and the same
two operators for the single generalized-default trajectory. Its unique key
is `scenario, member_or_default, event_id, operator, model_level, direction,
crossing_sequence`. It enumerates all complete-trajectory crossings before
event-window filtering; an operator with no crossing has no all-crossing row
but retains its mandatory unmatched event row in the applicable event table.

The expanded source audit reads the exact comment-prefixed CAL-07 source
objects directly:

- `bezamahafaly_DB_1000_1day.csv`, SHA-256
  `a490b29758ce0608428c6e794d8c803727b60fddc4e601c875564a26ed514f1f`;
- `bezamahafaly_DB_1000_simplified_transition_dates.csv`, SHA-256
  `db477b36731d0a8c072ac400dac3aa135e84234408d79a1a6a10eded739632cd`.

Comment lines are skipped before CSV parsing. CAL-07D may report raw support,
source-smoothed GCC90, and source confidence width, but must not regenerate,
refit, shift, or select replacement transition dates.

Minimum figures:

1. event-by-member crossing map with unmatched and out-of-window outcomes;
2. event-centered indicator/product chronology with ensemble spread and ties;
3. constraint-removal chronology/effect-size screen; and
4. model-relative threshold sensitivity screen.

Each figure sidecar binds the exact source CSV digest, keys, sample count,
event-window and unmatched rules, execution assumptions, evidence ceiling,
limitations, and accessible interpretation.

## Included scope

- Scaffold and execute this package-local investigation.
- Add package-local deterministic Python analysis, plotting, and validation.
- Publish machine-readable attribution, crossing, sensitivity, and decision
  artifacts.
- Produce accessible plot-only SVG figures with Markdown caption and
  ancillary-information sidecars.
- Update the canopy roadmap and work-package catalog.
- Complete two prospective reviews and two terminal reviews/verifications with
  finding disposition.

## Excluded scope

- No production Rust, runner, science-contract, fixture, or public-output edit.
- No mutation of CAL-07, CAL-07B, or CAL-07C evidence.
- No parameter refit, ranking, posterior, prior, physiological bound, or
  production default.
- No new phenology process equation, rainfall/soil-water surrogate, or
  heuristic correction.
- No claim that GCC is LAI, biomass, canopy cover, or physiological activity.
- No Order 7 advancement solely from diagnostic counterfactuals.

## Intended write set

- `docs/work-packages/20260729-canopy-cal-07d-transition-chronology-attribution-001/`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/README.md`

## Phase plan

1. Freeze scope, source identities, diagnostic operators, assumptions, and
   validation requirements.
2. Complete two independent prospective reviews before result-bearing
   analysis.
3. Execute crossing inventory, observation-operator, indicator, forcing, and
   constraint-removal diagnostics.
4. Generate figures and Markdown sidecars.
5. Independently validate equations, inventories, reconstructions, manifests,
   claims, and exact write set.
6. Complete dual terminal review/verification, finding disposition,
   roadmap/catalog updates, and final disposition.

## Validation requirements

- Dependency hashes bind exact CAL-07C inputs and results at commit
  `11b1faab`.
- Reconstructed daily Beza GSI matches all 61,642 CAL-07C Beza rows within
  `1e-12`.
- The four event by 37-member inventory contains exactly 148 rows for every
  absolute operator and the exact expanded inventories declared above.
- Absolute-crossing results reproduce every CAL-07C row, all 11 matched rows,
  all residuals, and all same-direction crossing counts.
- Every unmatched and out-of-window crossing remains explicit.
- Constraint-removal scenarios use the frozen forcing, member parameters, and
  21-day operator; only the declared indicator substitution may differ.
- All numeric outputs are finite where defined; undefined matches are blank,
  counted, and dispositioned rather than imputed.
- Independent validation reimplements equations, photoperiod, FIFO, crossing,
  event-window, scenario isolation, and decision predicates without importing
  analyzer helpers.
- Validation proves ordered equality of all 61,642 Beza member-dates,
  first-20-day available-sample semantics, later 21-day windows, no
  event-threshold discontinuity at year boundaries, unique declared keys, and
  one-row blank encoding for undefined/unmatched cases.
- Figures are valid/renderable SVG, contain no embedded prose panels, and each
  has a Markdown sidecar with caption, methods, source bindings, assumptions,
  limitations, and accessibility information.
- Package validator, Python syntax, Markdown lint, SVG XML/render checks,
  source-manifest validation, and exact diff hygiene pass.

## Review authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent prospective reviewer subagents and two
independent terminal reviewer/verifier subagents for protocol adequacy,
observation-operator integrity, scientific attribution, claim calibration,
figure integrity, and validation completeness; expected outputs are
package-local review and verification artifacts; write access is limited to
those named artifacts.

## Security-impact gate

No credentials, network acquisition, user-controlled command construction,
production interface, or runtime behavior is in scope. All inputs are retained
local evidence from committed predecessor packages.

## Exit criteria

- Every prespecified diagnostic executes or the package closes on hold with a
  named unavailable-evidence or authority boundary.
- The final disposition identifies which explanations are supported,
  contradicted, plausible, or unresolved without converting counterfactuals
  into science authority.
- The exact additional observations and source authority needed for each
  viable solution route are published.
- Order 7 remains held unless existing empirical contradictions are actually
  resolved by admitted evidence, not merely explained.
