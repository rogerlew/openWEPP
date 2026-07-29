# CAL-07D Terminal Verification A

Evidence class: `Ran + Static`

Verdict: `PASS / ORDER 7 HOLD RETAINED`

## Executed verification

The package validator ran successfully:

```text
CAL-07D validation PASS: 61,642 BASE rows independently reconstructed;
maximum equation residual=0.000e+00; 11 CAL-07C matches reproduced;
4 SVG/sidecar pairs verified
```

A separate terminal verifier, without importing `tools/analyze.py`, then
reimplemented event construction, fixed event-year thresholds, crossing
inequalities, fractional-day interpolation, adjacent-event windows,
first-crossing selection, named-indicator substitution, full-history FIFO
recomputation, scenario aggregation, and numeric decision predicates. It
reported:

```text
CAL-07D supplemental independent verification PASS:
event rows 148/1628/444/1488; scenario-days 9996;
crossings 34480; numeric predicates verified
```

## Verification matrix

| Surface | Result | Evidence |
| --- | --- | --- |
| Dependency and source identity | `PASS` | Ten paths, bytes, SHA-256 values, and source-commit bindings verified; retained CAL-07C transition digest also matches `git show 11b1faab`. |
| CP-GSI01 equations and chronology | `PASS` | All 61,642 ordered BASE member-days independently reconstructed; signed-latitude FAO-56, `365` leap-day denominator, empty-FIFO warm-up, and later 21-day windows agree within `1e-12`. |
| CAL-07C reproduction | `PASS` | All 148 event rows, 11 matches, prior residuals, crossing ordinals, and same-direction event-window counts agree. |
| Observation/operator inventories | `PASS` | Independently recomputed 148 absolute, 1,628 model-level, and 444 source-level rows, retaining blanks for undefined/unmatched cases. |
| Scenario isolation | `PASS` | Five 37-member scenarios and the one-member canonical default were rebuilt from day one; all 9,996 published daily aggregate rows agree. |
| Crossing inventory | `PASS` | All 34,480 published crossings were re-derived with matching direction, sequence ordinal, and event-window flag. |
| Decision predicates | `PASS` | Observation-scale recovery, three indicator-contributor counts, current-constraint sensitivity, and missing-process screen were independently reduced; forcing plausibility was checked against retained source authority. |
| Figure integrity | `PASS` | Four SVGs parsed and rendered. A temporary-copy rerun of `tools/plot.py` produced byte-identical SVGs and sidecars from the digest-bound CSVs; visual inspection found no clipped or misleading panel. |
| Exact write set | `PASS` | Worktree changes are limited to the declared CAL-07D directory, canopy roadmap, and work-package catalog. `git diff --check` passed at this snapshot. |
| Production/science authority | `PASS` | No production Rust, runner, predecessor package, or `SC-PLANT-001` edit is present; OBL-PLANT-P-013 remains unchanged. |

## Claim and closure check

The result artifacts support diagnostic attribution only. They do not identify
a correct parameter set, establish forcing error, validate a missing process,
or authorize production changes. Required successor evidence is explicitly
listed rather than treated as completed CAL-07D work.

Verifier A passes the executed scientific and artifact gates. Package-level
closure still requires the terminal documentation lint, final exact-diff
reconciliation, sibling review/verification, finding disposition, and final
disposition to be recorded as completed. Order 7 correctly remains on
scientific hold.
