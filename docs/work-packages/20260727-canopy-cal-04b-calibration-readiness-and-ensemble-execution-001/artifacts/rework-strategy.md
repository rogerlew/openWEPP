# CAL-04B Scaffold Rework Strategy

Status: `PROSPECTIVE / REVIEW REQUIRED`

Evidence class: `Static`

Harvard remains sealed. No result-bearing candidate, synthetic-recovery, or
holdout execution is authorized by this rework alone.

## Outcome

Replace the blocked monolithic executor with four independently gated programs
and an evidence-only disposition phase:

1. `intake-and-native-proof`: authenticate every input and prove representative
   threshold vectors pass from package-local configuration copies through the
   real production runfile/management consumer to the retained native research
   trace.
2. `hubbard-calibration`: execute all 9,261 vectors across CAL-04A's nine
   plot-specific Daymet lanes with the actual `openwepp-plant-phenology`
   kernel, retain plot-keyed daily/crossing/component evidence, independently
   reconstruct the equal-year objective, and freeze the accepted ensemble.
3. `readiness-stages`: execute prospectively frozen synthetic and later-stage
   designs, propagate accepted upstream membership, and report sensitivity,
   failures, boundaries, and equifinality without empirical overclaim.
4. `harvard-holdout`: require a nonempty immutable freeze manifest and two
   verification PASS records before a separate command may read Harvard. Score
   the first downward `previous > 0.5 && current <= 0.5` crossing once, with no
   refit path.

## Binding Controls

- `CTRL-NATIVE-01`, `CTRL-ID-01`, `CTRL-RECON-01`, `CTRL-HOLDOUT-01`, and
  `CTRL-SAT-01` are defined in `execution-control-contract.md`.
- `CTRL-RECOVERY-01` is the exact machine-readable design in
  `later-stage-design.csv`.
- `CTRL-UPSTREAM-01` carries the full parent membership Cartesian product into
  each ordered stage. Every child row retains all parent candidate IDs and the
  checksum of the stage-input membership ledger. Best-vector, range-endpoint,
  centroid, or convenience reduction is forbidden.

## Correction Rules

- The production path proof and population executor have different claims. The
  former proves configuration wiring and trace consumption; the latter proves
  complete deterministic kernel evaluation. Their outputs must agree exactly
  for representative interior, boundary, saturated, and invalid vectors.
- Python or another independent implementation may reconstruct objectives and
  diagnostics, but cannot produce the native-kernel claim.
- Climate parsers fail on malformed, duplicate, missing, or nonchronological
  daily rows. Calibration latitude is parsed from CAL-04A's checksum-bound
  source-EML plot geometry, never supplied as an unauthenticated literal. Each
  plot-year is a separate native cold start fed real Daymet yday 1–180; yday
  1–59 is warm-up, crossing eligibility begins at yday 60, and no synthetic
  prefill, missing leap date, or cross-year state carry is allowed.
- Saturation is calculated from CAL-04A's nine-plot Daymet population, restricted
  to yday 60 through each applicable observation interval upper bound, using
  the exact `1e-12` rule.
- Synthetic recovery generates observations from a hidden declared truth,
  executes the frozen search, and derives the recovered set from objective
  values. A prewritten PASS row is invalid evidence.
- Later-stage axes, operators, units, enumeration, and acceptance behavior are
  frozen before execution and every execution-only value is labeled
  `ASSUMED_FOR_EXECUTION`.
- Calibration output retains observation distances, species diagnostics, annual
  components, failed records/years, crossings, and aggregate equal-year RMSE.
  Acceptance is independently rebuilt from annual evidence rather than from the
  producer's aggregate scalar.

## Program Interfaces

| Program | Reads | Writes | Forbidden reads |
|---|---|---|---|
| native proof | copied Hubbard fixture/configs, production binary | native trace, injection ledger, invalid-case errors | Harvard |
| calibration producer | frozen grid, CAL-04A Daymet derivation and plot geometry | authenticated plot-keyed daily trace, typed producer failures, execution receipt | protected fixture as calibration forcing; Harvard; all observations/scores |
| primary reconstructor | immutable daily GSI trace, configurations, Hubbard observations | crossings, components, objectives, membership | Harvard and producer-derived scores |
| verification reconstructor | immutable daily GSI trace, configurations, Hubbard observations | independently implemented full derived ledger and membership diff | Harvard, producer-derived scores, and primary scoring code |
| holdout scorer | frozen manifest, accepted configs, Harvard member/observations | opening record, holdout components/results | calibration write paths |

## Holdout State Machine

`SEALED -> FREEZE_READY -> VERIFIED_BY_TWO -> OPENED_ONCE -> SCORED_NO_REFIT`

The atomic and crash behavior is binding under `CTRL-HOLDOUT-01`.

## Review Boundaries

- Two prospective scientific reviews and two scaffold verifications must PASS
  this scaffold before executor implementation begins.
- Executor implementation receives two code/science reviews before Hubbard
  execution.
- The freeze manifest receives two independent verification PASS records before
  Harvard opening.
- Terminal evidence receives dual scientific review and dual verification.

## Completion of This Rework

This scaffold rework is complete when all prior findings are mapped to explicit
control IDs, two reviewers and two verifiers accept the strategy without
unresolved blocking findings, Markdown/diff/scaffold-validator checks pass, and
the package remains `HOLD / REWORKED SCAFFOLD / NOT EXECUTED`.
