# Verification Agent A

Status: `PASS`

Evidence mode: **Ran + Static**.

## Verdict

All ten package acceptance criteria pass on the corrected, reviewed tree. No
closure-blocking, major, minor, or low finding remains. EB-04W2 may proceed to
the second terminal verifier and lifecycle closure.

## Independent Ran Evidence

- Recomputed the SHA-256 identities of the prospective freeze, frozen tool,
  release binary, transformation preflight, and all four bound EB-04W1
  predecessor objects. They match the freeze exactly. Source HEAD is
  `5037ff35278f6c07f5f7b824a503cab467ffe0cc`; the freeze precedes the first
  extension run by `25.395 s`.
- Reconstructed the exact extension inventory as four lanes times five
  multipliers (`1.6-2.0`): `20` unique cells, `20` provenance objects, and zero
  nonzero return codes. Rehashed all `20` provenance objects and all `120`
  receipt-bound new runtime outputs against their recorded hashes and sizes.
- Reconstructed the retained inventory as four lanes times six multipliers
  (`1.0-1.5`): `24` unique cells. Rehashed all `24` committed provenance
  objects and all `144` retained outputs against the immutable EB-04W1
  receipt. No anchor was rerun or altered.
- Independently compared every source/scaled fixture tree for all 20 extension
  cells. Each tree has identical file membership, every non-climate file is
  byte-identical, and the only changed file is the lane's `.cli` file. Parsing
  all `284,900` daily climate rows found only precipitation token 4 changed;
  all dates and protected duration, shape, temperature, radiation, wind, and
  dew-point tokens match. Maximum scaling residual is
  `5.684341886080802e-14 mm`; protected-token and non-daily-line mismatch
  counts are both zero.
- Reconstructed the combined inventory directly from the result object:
  `44` unique cells, exactly 11 multipliers per lane, partitioned into six
  retained and five extension cells. The result object is hash-bound to the
  unchanged tool and execution receipt.
- Reapplied the frozen eligibility, absolute-log magnitude ranking,
  chronology ranking, parity crossing, strict compensation thresholds, and
  `2.0` stop-loss without calling the producer selection routine. Results
  match exactly:

| Lane | Selected | Magnitude best | Chronology best | Parity bracket | Compensation | Classification |
|---|---:|---:|---:|---|---|---|
| Mica Creek | `1.4` | `1.4` | `2.0` | `1.4-1.5` | false | `TRADEOFF_BRACKETED` |
| Niwot | `1.7` | `1.7` | `1.9` | `1.6-1.7` | false | `TRADEOFF_BRACKETED` |
| Paradise | `1.8` | `1.8` | `1.8` | `1.8-1.9` | false | `BRACKETED_CANDIDATE` |
| Snowbird | `2.0` | `2.0` | `2.0` | none in grid | false | `EXPERIMENT_BUDGET_BOUNDARY` |

- Checked every stored cell closure maximum and the independently reviewed
  operand reconstruction. Mass, phase, accumulation, melt-component, and
  trace/WAT closure remain below `1e-12 m`; the combined maximum is
  `4.440892098500626e-15 m`.
- Ran the frozen transformer self-check. Parsed and rasterized all four SVGs,
  then visually inspected them at 1200-pixel width. Axes, units, calibration
  band, experiment boundary, compensation thresholds, labels, trajectories,
  and legends are readable and agree with the machine results. No clipping,
  hidden markers, or obstructed labels were found. Each SVG has a same-stem
  sidecar covering population, units, methods, provenance, uncertainty, and
  interpretation limits.
- Ran `markdown-doc lint` over the package and each of the three roadmap/catalog
  files: 31 files total, zero errors and zero warnings. `git diff --check` and
  SVG XML/render checks pass.

## Static Governance Evidence

- Observations were frozen as `CALIBRATION`; independent-validation count is
  zero and promotion is false. `EMPIRICALLY_CALIBRATED` is narrowly supported
  for the interior Mica Creek `1.4`, Niwot `1.7`, and Paradise `1.8`
  fixture/record pairs. Snowbird `2.0` is correctly boundary-censored.
  `PARTIALLY_IDENTIFIABLE` preserves forcing/process confounding; the package
  makes no gauge-undercatch, transferability, regional-default, uncertainty,
  independent-validation, or promotion claim.
- The archived execution prompt retains the exact task, constraints,
  stop-loss, and delegated-review authorization. No result-bearing prompt
  remains active.
- The exact write set is the new EB-04W2 package plus `docs/ROADMAP.md`, the
  snow-surface campaign roadmap, and the work-package catalog. Ignored output
  is confined to the declared target root. No Rust, contract, manifest, test,
  source fixture, observation, assurance, schema, selector, default, or
  historical-package path changed. Rust suites are therefore correctly
  `NOT_APPLICABLE` for this analysis-only increment.
- Roadmap and catalog consistently record EB-04W2 as executed/review, close
  the precipitation-forcing branch without EB-04W3, and identify EB-04X as
  next. The `2.0` ceiling is consistently described as an experiment-budget
  stop rather than a physical upper bound.

## Acceptance-Criterion Disposition

| Criterion | Result |
|---:|---|
| 1. prospective identities and freeze timing | `PASS` |
| 2. exact 20-cell extension execution | `PASS` |
| 3. exact 24-cell retained-anchor identity | `PASS` |
| 4. precipitation-only transformation | `PASS` |
| 5. exact 44-cell frozen-rule analysis | `PASS` |
| 6. inherited closures within `1e-12 m` | `PASS` |
| 7. brackets, rankings, warnings, and stop-loss | `PASS` |
| 8. figures, visual inspection, and sidecars | `PASS` |
| 9. protected write set | `PASS` |
| 10. gates, reviews, prompt, and terminal evidence | `PASS` |

## Residual Risks

Residual scientific risk is explicit and non-blocking for this package:
calibration and assessment use the same site records; interannual uncertainty
and coefficient covariance are not estimated; the inferred multiplier can
compensate for phase, retention, and pre-peak loss error; and Snowbird remains
23 days early at the final budget cell. These limits prevent validation,
transferability, default, and promotion claims, but do not invalidate the
bounded site-specific calibration experiment.

Final disposition: `PASS / ADMIT_CLOSURE`.
