# SNOWFROST-FIDELITY-E Snow-Depth Fidelity Adjudication

Status: completed

Package type: contract-first snow-depth correspondence and diagnostic
adjudication.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: adjudicate the snow-depth control failure exposed by
SNOWFROST-FIDELITY-D before any frost heat-flow, frozen-K, SFCC, impedance, or
`Qwet` work resumes. Closure requires a canonical snow-depth correspondence
invariant, signed snow-depth residual evidence across the observed pilot sites,
artifact/alias checks that rule out depth-vs-SWE and timing/stage mismatches as
the primary explanation, and a route decision for the next package.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`tests/AGENTS.md`, `tools/snowfreeze_observed/README.md`, and packages
SNOWFROST-FIDELITY-A/B/C/D.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only science-review, harness-review, and
verification subagents for snow-depth correspondence legitimacy review, audit
anti-alias review, and final evidence review. Expected outputs are compact
findings summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files. Current execution
uses local reviews unless the operator separately requests subagent dispatch.

## Purpose

SNOWFROST-FIDELITY-D proved the modeled snow-depth diagnostic exists but also
showed that sites with paired snow observations fail `TOL-SNOWFREEZE-009`.
That blocks frost-depth attribution because snow depth is the dominant
insulation control. E decides whether the failure is a correspondence artifact,
publication alias/timing artifact, fixture/input issue, or snow-physics issue.

## Non-Goals

- Do not change production snow/frost physics, constants, or runtime control
  flow.
- Do not tune snow depth, frost depth, heat flow, frozen conductivity, SFCC
  parameters, impedance factors, or migration/fringe heat.
- Do not enable, port, approximate, or promote `Qwet`.
- Do not classify frost residuals `OPENWEPP-DEFECTIVE`.
- Do not default-activate direct runtime or delete compatibility runtime.

## Authority Envelope

In scope:

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-047`;
- new canonical snow-depth correspondence invariant under `SC-SNOWFREEZE-001`;
- `TOL-SNOWFREEZE-009` as a frost-attribution confound gate, not a snow model
  pass/fail calibration target;
- WAT `Snow-Depth` diagnostic lineage from `snow.runtime_depth_m`;
- WAT `Snow-Water` SWE anti-alias evidence;
- observed snow-depth rows from the five pilot fixtures;
- all-site observed harness rerun and snow-depth signed residual audit.

Out of scope:

- production process-physics edits;
- new external observation acquisition;
- compatibility bit-parity;
- migration/fringe or frozen-K model selection.

## Intended Write Set

- `docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `tools/snowfreeze_observed/README.md`
- `tools/snowfreeze_observed/observed_harness.py`
- `tools/snowfreeze_observed/classify_residuals.py`
- `tools/snowfreeze_observed/snow_depth_audit.py`
- `tests/integration/snowfreeze_observed_frost_depth_contract.rs`

## Phase Plan

### Phase 0: Scaffold and Authority Lock

- Create package scaffold and required-reading evidence.
- Record SNOWFROST-FIDELITY-D results as the starting point.
- Lock the no-physics-change boundary.

Exit criteria:

- Package artifacts exist.
- Package scope forbids frost/snow physics changes and `Qwet`.

### Phase 1: Contract Correspondence Invariant

- Add a canonical invariant that defines modeled WAT `Snow-Depth` as physical
  snowpack depth and forbids using SWE (`Snow-Water`) as a snow-depth proxy.
- Require like-for-like snow-depth verdicts to prove source method, units,
  timing/stage, and publication lineage before classifying failures as a snow
  model issue.
- Add regression coverage that the harness/tooling carries the invariant.

Exit criteria:

- Contract version and change log are updated.
- Tests bind the invariant and anti-alias language.

### Phase 2: Signed Snow-Depth Audit Diagnostics

- Extend comparison/audit tooling to publish signed residual direction,
  over/under counts, mean signed residual, median signed residual, and
  depth-vs-SWE anti-alias comparisons.
- Summarize by site and water year where available.
- Preserve nullable/no-observed-snow classifications for SCAN and Reynolds.

Exit criteria:

- Audit JSON/Markdown artifacts exist under this package.
- The audit states whether current evidence is correspondence artifact,
  publication alias/timing artifact, fixture/input issue, snow-physics issue,
  insufficient data, or frost-ready.

### Phase 3: Rerun Observed Evidence and Close

- Build the runner as needed.
- Run all five observed comparisons with the current WAT publication.
- Run residual classification and the snow-depth audit over fresh reports.
- Record route decision and validation gates.

Exit criteria:

- Five fresh comparison reports exist.
- Classification and snow-depth audit artifacts exist under this package.
- Gate table has no unjustified `FAIL`, `BLOCKED`, or `NOT RUN`.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate --observations-dir tests/fixtures/snowfreeze_observed/observations`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site <site_id> --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfrost_fidelity_e_observed_compare/<site_id>`
- `.venv/bin/python tools/snowfreeze_observed/classify_residuals.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-json docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/artifacts/residual_classification.json --output-md docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/artifacts/residual_classification.md target/snowfrost_fidelity_e_observed_compare/*/comparison_report.json`
- `.venv/bin/python tools/snowfreeze_observed/snow_depth_audit.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-json docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/artifacts/snow_depth_audit.json --output-md docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/artifacts/snow_depth_audit.md target/snowfrost_fidelity_e_observed_compare/*/comparison_report.json`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/observed_harness.py tools/snowfreeze_observed/classify_residuals.py tools/snowfreeze_observed/snow_depth_audit.py`
- `cargo test --test snowfreeze_observed_frost_depth_contract`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`
- `rg -n "qwet|Qwet|frzftp" crates || true`

## HOLD Boundaries

Close as `HOLD` only if modeled/observed snow-depth correspondence cannot be
adjudicated from current source provenance, if WAT publication cannot expose the
required anti-alias operands without production physics changes, if observed
harness rerun cannot produce metric-bearing reports, or if source provenance
contradicts the assumed snow-depth measurement semantics.
