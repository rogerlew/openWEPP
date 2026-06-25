# SNOWFROST-FIDELITY-D Snow-Depth Publication and A Rerun

Status: completed

Package type: diagnostic publication implementation and observation
classification rerun.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: expose modeled snow depth as a diagnostic WAT publication surface
and rerun the observed frost-depth residual classifier from
SNOWFROST-FIDELITY-A. Closure requires `Snow-Depth` to publish from
`snow.runtime_depth_m` without changing snow/frost physics, the observed
harness to consume that depth rather than the old missing-diagnostic sentinel,
and fresh all-site comparison/classification evidence.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`tests/AGENTS.md`, `tools/snowfreeze_observed/README.md`, and packages
SNOWFROST-FIDELITY-A/B/C.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only publication-review, science-review, and
verification subagents for WAT publication lineage review, snow-control
classification legitimacy review, source-scan review, and final evidence
review. Expected outputs are compact findings summarized into
`artifacts/review-disposition.md` and `artifacts/verification.md`; subagents
may not edit files. Current execution uses local reviews unless the operator
separately requests subagent dispatch.

## Purpose

SNOWFROST-FIDELITY-A could not classify field frost-depth residuals because
modeled snow depth was absent. B and C added heat-flow benchmark and
SFCC/frozen-K diagnostic gates without production physics changes. D supplies
the missing modeled snow-depth diagnostic and reruns A so `TOL-SNOWFREEZE-009`
can either pass, fail as snow-confounded, or remain inconclusive with current
evidence.

## Non-Goals

- Do not change snow/frost physics, constants, or runtime control flow.
- Do not enable, port, approximate, or promote `Qwet`.
- Do not tune residuals, thresholds, SFCC/frozen-K models, or heat-flow terms.
- Do not classify `OPENWEPP-DEFECTIVE` unless the published modeled snow depth
  satisfies the `TOL-SNOWFREEZE-009` gate and the remaining
  `INV-SNOWFREEZE-047` correspondence gates pass.
- Do not default-activate direct runtime or delete compatibility runtime.

## Authority Envelope

In scope:

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-047`;
- `SC-SNOWFREEZE-001#TOL-SNOWFREEZE-009`;
- existing runtime `snow.runtime_depth_m` and `snow.runtime_density_kg_m3`
  diagnostic state surfaces;
- WAT publication as a diagnostic output surface;
- observed harness compare/classify rerun for all five pilot sites.

Out of scope:

- production process-physics edits;
- observation data acquisition or normalization changes;
- compatibility bit-parity;
- migration/fringe or frozen-K model selection.

## Intended Write Set

- `docs/work-packages/20260625-snowfrost-fidelity-d-snow-depth-publication-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/watershed_wat.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`
- `crates/openwepp-sim-contract/src/units_mod/boundary_catalog.rs`
- `tools/snowfreeze_observed/README.md`
- `tools/snowfreeze_observed/observed_harness.py`
- `tools/snowfreeze_observed/classify_residuals.py`
- `tests/integration/snowfreeze_observed_frost_depth_contract.rs`

## Phase Plan

### Phase 0: Scaffold and Authority Lock

- Create package scaffold and required-reading evidence.
- Record the prior A/B/C state and the diagnostic-only publication boundary.
- Update the roadmap so D means snow-depth publication and A rerun; defer
  conditional migration/fringe work to later scope.

Exit criteria:

- Package artifacts exist.
- Package scope forbids physics changes and Qwet.

### Phase 1: WAT Snow-Depth Publication

- Add nullable WAT `Snow-Depth` (`mm`) as a diagnostic output column.
- Populate compatibility/standard rows from `snow.runtime_depth_m * 1000`.
- Populate direct-publication rows from direct storage operands sourced from
  `snow.runtime_depth_m`.
- Add unit registry/boundary metadata.

Exit criteria:

- Focused WAT/schema tests prove `Snow-Depth` exists, has `mm` units, and is
  distinct from `Snow-Water`.
- Direct/compatibility optional mismatch checks include `Snow-Depth`.

### Phase 2: Observed Harness Consumption

- Load WAT `Snow-Depth` and compare it to paired observed snow-depth rows.
- Replace `UNRESOLVED_NO_MODELED_SNOW_DEPTH_DIAGNOSTIC` when modeled depth is
  present.
- Preserve the rule that `Snow-Water` is SWE and is not a snow-depth proxy.
- Extend classifier output with snow-control counts/residual summaries.

Exit criteria:

- Harness reports include modeled snow-depth metrics.
- Classifier no longer reports the missing-depth sentinel for sites with WAT
  `Snow-Depth` available.
- No site may become defect-eligible unless snow control passes.

### Phase 3: Rerun A and Close Evidence

- Build the runner.
- Run all five observed comparisons.
- Run the classifier over fresh reports.
- Record current classifications and next action.
- Run focused and workspace validation gates.

Exit criteria:

- Five reports exist and are metric-bearing where sources are valid.
- Classification artifacts exist under this package.
- Gate table has no unjustified `FAIL`, `BLOCKED`, or `NOT RUN`.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py validate --observations-dir tests/fixtures/snowfreeze_observed/observations`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site <site_id> --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfrost_fidelity_d_observed_compare/<site_id>`
- `.venv/bin/python tools/snowfreeze_observed/classify_residuals.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-json docs/work-packages/20260625-snowfrost-fidelity-d-snow-depth-publication-rerun-001/artifacts/residual_classification.json --output-md docs/work-packages/20260625-snowfrost-fidelity-d-snow-depth-publication-rerun-001/artifacts/residual_classification.md target/snowfrost_fidelity_d_observed_compare/*/comparison_report.json`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/observed_harness.py tools/snowfreeze_observed/classify_residuals.py`
- `cargo test -p openwepp-hillslope-output hillslope_wat`
- `cargo test --test snowfreeze_observed_frost_depth_contract`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`

## HOLD Boundaries

Close as `HOLD` only if WAT snow-depth publication requires a physics change,
if observed harness rerun cannot produce metric-bearing reports, if
snow-control classification cannot be made non-tautologically from published
depth, or if the added diagnostic surface breaks output/schema consumers.
