# CAL-07D Terminal Scientific Review B

Evidence class: `Static + Ran`

Review scope: remediated CAL-07D package, exact retained dependencies,
independent validation implementation, result tables, decision reduction,
science summary, four figure/sidecar pairs, and roadmap/catalog edits.

Verdict: `PASS / ORDER 7 HOLD RETAINED`

## Remediation disposition

All three findings from the initial Terminal Review B are resolved.

### `CAL07D-TRB-001` — Resolved

`tools/validate.py` now independently reconstructs the five 37-member
scenarios and the single canonical generalized-default trajectory directly
from frozen forcing and parameters. It does not import analyzer helpers. The
independent path:

- recomputes daily indicators, instantaneous products, continuous 21-day
  histories, and all 9,996 published scenario/day summaries;
- applies fixed event-year thresholds, direction inequalities, fractional-day
  interpolation, adjacent-event windows, and first-chronological selection;
- exactly reproduces all 148 absolute rows, 1,628 model-level rows, 444
  source-level rows, 1,488 scenario rows, and all 34,480 global crossing rows;
- independently reconstructs the 12 observation-support and 12
  event-attribution rows from the comment-prefixed source objects; and
- independently reduces all seven decision predicates, including contributor
  counts, same-operator match increase, and the bounded missing-process
  screen.

The remediated validator ran successfully and reported every event/crossing
row reproduced.

### `CAL07D-TRB-002` — Resolved

Every SVG now embeds a machine-checkable metadata record containing exact
source CSV hashes and row counts, plotted fields, plotted-record count, and a
canonical plotted-data SHA-256. The independent validator rebuilds those
records from the bound CSVs and verifies the fingerprints.

All four sidecars now state:

- the exact table key and row/sample count;
- the relevant event-window and first-crossing rule;
- explicit blank/unmatched encoding, or why matching does not apply;
- exact result digests;
- execution assumptions and evidence ceiling; and
- limitations and accessible interpretation.

The plot-to-validation pipeline, XML parsing, and raster rendering all pass.

### `CAL07D-TRB-003` — Resolved for reviewer-owned gates

Both independent terminal review/verification pairs now pass the scientific
and artifact surfaces. Package integration still needs to record the already
executed Markdown lint and diff checks, disposition terminal findings, and
author exact-diff/final-disposition artifacts. Those are final assembly steps,
not an open scientific or validation defect.

## Scientific and authority assessment

- Source custody remains exact. The ten dependency paths are bound to commit
  `11b1faab37b5d365b0c0c7051ed32a4762821239`, and direct commit-object checks
  confirmed all ten recorded SHA-256 values.
- CAL-07C is reproduced exactly: all 61,642 BASE member-days have maximum
  equation residual `0.000e+00`, and the same 11 of 148 absolute transitions
  match.
- Observation scale is a supported mathematical contributor because 262
  previously absolute-unmatched member/event/source-level rows become
  matched. It does not establish GCC/GSI equivalence or solve chronology.
- Temperature, VPD, and photoperiod substitutions change 98, 248, and 296
  member/event/operator rows under the prespecified predicate. These are
  mathematical sensitivity results, not biological cause attribution.
- Fixed calendar-year thresholds are explicitly retrospective and
  `ASSUMED_FOR_EXECUTION`. No level is selected or calibrated.
- POWER forcing bias, ecotype/threshold transfer, observation semantics, and
  a missing seasonal or water cue remain non-identifiable with current
  evidence.
- The roadmap and catalog correctly state that CAL-07D explains the
  contradiction without resolving it. No production parameter, forcing,
  process equation, science-contract amendment, or Order 7 advancement is
  authorized.

## Gate non-deferral

All prespecified CAL-07D scientific diagnostics and independent validation
requirements now have direct current-package evidence. The named needs for
on-site meteorology, field/image corroboration, tropical dry-forest parameter
authority, water-status evidence, and a reserved independent deciduous lane
are successor evidence/authority boundaries, not deferred CAL-07D acceptance
gates.

## Final review disposition

`PASS / ORDER 7 HOLD RETAINED`.

No closure-blocking scientific, reproducibility, figure-integrity, source
custody, scenario-isolation, decision-predicate, or claim-calibration defect
remains. Final package integration must truthfully record the remaining
terminal documentation and exact-diff steps before declaring package closure.
