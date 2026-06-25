# SNOWFROST-FIDELITY-A Observation Residual Classification

Status: complete

Package type: characterization / external-observation adjudication gate.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: run the observed frost-depth comparison harness across all five
pilot sites and classify residuals before changing frost physics. Closure means
each valid site emits an exit-0 metric-bearing report and receives a
measurement-aware classification that respects `INV-SNOWFREEZE-047` and the
`TOL-SNOWFREEZE-009` snow-control gate.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`tools/snowfreeze_observed/README.md`, and
`tests/fixtures/snowfreeze_observed/README.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only science-review and verification subagents for
classification legitimacy, snow-control gate review, and final evidence review.
Expected outputs are compact findings summarized into
`artifacts/review-disposition.md` and `artifacts/verification.md`; subagents
may not edit files. Current execution used local reviews because the user did
not separately request subagent dispatch in this turn.

## Purpose

R7H is closed opt-in and frost-depth fidelity is no longer a
direct-vs-compatibility bit-parity target. The next governing question is
whether current openWEPP frost-depth behavior can be judged against historic
observations. This package is the first field-data adjudication step: run the
existing observation harness and classify residuals as snow-confounded,
heat-flow shaped, lower-boundary shaped, frozen-conductivity/infiltration
shaped, migration/fringe shaped, or inconclusive only where the current
evidence supports that classification.

## Non-Goals

- Do not change snow or frost physics.
- Do not enable or port `Qwet`.
- Do not tune coefficients or thresholds.
- Do not declare `OPENWEPP-DEFECTIVE` unless modeled snow depth, measurement
  correspondence, censoring, and residual magnitude gates all pass.
- Do not default-activate direct runtime or delete compatibility runtime.
- Do not require network access for normal validation.

## Authority Envelope

In scope:

- `SC-SNOWFREEZE-001#GAP-SNOWFREEZE-002`;
- `INV-SNOWFREEZE-047` measurement correspondence;
- provisional `TOL-SNOWFREEZE-007`, `TOL-SNOWFREEZE-008`, and
  `TOL-SNOWFREEZE-009`;
- `tests/fixtures/snowfreeze_observed/` pilot fixtures;
- `tools/snowfreeze_observed/observed_harness.py compare` output reports;
- a report classifier that labels current evidence and blocks unsupported
  physics attribution.

Out of scope:

- production runtime code and kernel physics;
- observation threshold changes;
- new data acquisition or normalization;
- compatibility frost bit-parity.

## Intended Write Set

- `docs/work-packages/20260625-snowfrost-fidelity-a-observation-residual-classification-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `tools/snowfreeze_observed/classify_residuals.py`

## Phase Plan

### Phase 0: Scaffold and Authority Lock

- Create the package structure and active kickoff prompt.
- Record the required reading and source authority.
- Define the classification vocabulary and gate table.

Exit criteria:

- Package artifacts exist.
- Classification cannot create a frost defect verdict when snow-control
  evidence is missing.

### Phase 1: Fresh Site Comparisons

- Build the current `openwepp-cli-hill` binary.
- Run direct observed comparisons for all five pilot sites.
- Record command, output directory, exit status, and report verdict.

Exit criteria:

- All five sites emit `comparison_report.json`.
- Valid site reports are metric-bearing; failures are classified as harness
  failures rather than physics defects.

### Phase 2: Residual Classification

- Add and run a classifier over the five reports and normalized observation
  manifest.
- Classify each site by measurement method, snow-control status, and residual
  family.
- Record a next-action recommendation that does not change physics before
  snow-control and benchmark gates.

Exit criteria:

- `artifacts/residual-classification.json` and
  `artifacts/residual-classification.md` exist.
- Every site has exactly one primary classification.
- No site is marked `OPENWEPP-DEFECTIVE` unless snow-control has passed.

### Phase 3: Verification, Review, and Disposition

- Run focused observation-harness validation and classifier checks.
- Run doc and diff hygiene checks.
- Complete review, disposition, line-count, and worker-handoff artifacts.

Exit criteria:

- Gate table has no unjustified `FAIL`, `BLOCKED`, or `NOT RUN` for current
  package scope.
- Final disposition truthfully states whether frost physics is ready for
  remediation or still blocked by classification gates.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate --observations-dir tests/fixtures/snowfreeze_observed/observations`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site <site_id> --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfrost_fidelity_a_observed_compare/<site_id>`
- `.venv/bin/python tools/snowfreeze_observed/classify_residuals.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-json docs/work-packages/20260625-snowfrost-fidelity-a-observation-residual-classification-001/artifacts/residual-classification.json --output-md docs/work-packages/20260625-snowfrost-fidelity-a-observation-residual-classification-001/artifacts/residual-classification.md target/snowfrost_fidelity_a_observed_compare/*/comparison_report.json`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/observed_harness.py tools/snowfreeze_observed/classify_residuals.py`
- `cargo test --test snowfreeze_observed_frost_depth_contract`
- `git diff --check`

## HOLD Boundaries

Close as `HOLD` only if the harness cannot produce metric-bearing reports for
valid fixtures, the classifier cannot distinguish snow-control absence from a
physics defect, the observation manifest is invalid, or current evidence
contradicts `INV-SNOWFREEZE-047`.
