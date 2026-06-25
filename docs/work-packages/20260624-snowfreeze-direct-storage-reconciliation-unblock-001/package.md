# SNOWFREEZE Direct Storage Reconciliation Unblock

Status: complete

Package type: Defect-Closure ExecPlan.

Primary defects:

- `SNOWFREEZE-DRSTOR-001`: direct observed-harness comparison for
  `site3_scan_mandan_nd` fails before comparison at lane 1 day 487 with
  `storage_reconciliation.frost_storage_projection_theta_m must be nonnegative`.
- `SNOWFREEZE-DRSTOR-002`: direct observed-harness comparison for
  `site4_ggd498_morris_mn` fails before comparison at lane 1 day 10727 with
  `storage_reconciliation.frost_storage_projection_theta_m must be nonnegative`.

Objective: diagnose and close the direct-runtime storage-reconciliation failures
that prevent observed frost-depth comparison for sites 3 and 4. Closure means
both site fixtures run through `tools/snowfreeze_observed/observed_harness.py
compare` with exit `0` and metric-bearing reports (`reason = null`, `metrics`
present), without loosening typed storage guards, changing snow/frost physics,
resuming frost-vs-compatibility bit-parity, or default-activating direct
runtime.

This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, `crates/AGENTS.md`, and
`tests/AGENTS.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only runtime-review and verification subagents for
final diff review, guard-integrity review, site3/site4 evidence review, and
line-count/gate legitimacy review. Expected outputs are compact findings
summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files.

## Purpose

The observation harness created the right validation direction for reopened
`GAP-SNOWFREEZE-002`, but two pilot sites cannot yet produce metric-bearing
comparisons because direct runtime fails closed while reconciling explicit frost
storage deltas back into the direct layer projection. After this package, all
five pilot sites can reach the observation comparison surface, making
frost-depth fidelity defects visible instead of being hidden behind a storage
projection implementation failure.

## Correction Authority Envelope

Observed violations:

- `site3_scan_mandan_nd` direct comparison fails before producing
  an exit-0 metric-bearing comparison report.
- `site4_ggd498_morris_mn` direct comparison fails before producing
  an exit-0 metric-bearing comparison report.
- Both failures report the same direct typed guard:
  `storage_reconciliation.frost_storage_projection_theta_m must be
  nonnegative`.

In-scope mechanism:

- R4B direct storage reconciliation in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs` when an
  explicit WB12 frost storage liquid delta is present and the aggregate
  reconciled storage must be projected back into
  `evapotranspiration_compute.layer_state_after_root_uptake`.

Allowed production-edit classes:

- Correct aggregate-to-layer projection bookkeeping so a negative aggregate
  delta can be distributed across available positive layer liquid storage
  without making any layer negative.
- Preserve fail-closed behavior when the requested debit exceeds available
  active layer liquid storage, when non-finite values appear, when layers are
  missing, or when layer depth/frozen-depth invariants are invalid.
- Update shadow projection state to match the corrected layer projection.

Protected boundaries:

- Do not tune, replace, or approximate snow/frost heat-flow physics.
- Do not change observation verdict thresholds or classify the frost model as
  valid/invalid in this package.
- Do not delete compatibility runtime, rollback, or shadow paths.
- Do not default-activate direct runtime.
- Do not silently clamp material negative storage or hide a true domain
  violation.

Acceptance criteria:

- Reproduce both failures before the fix and record the exact command/results.
- Add focused regression coverage proving that explicit frost storage
  projection can debit more than the first layer's `theta_m` by debiting across
  multiple layers, while still failing closed when total active liquid storage
  is insufficient.
- Rerun `compare` for `site3_scan_mandan_nd` and
  `site4_ggd498_morris_mn`; both must exit `0` and emit metric-bearing
  machine-readable comparison reports with `reason = null` rather than failing
  at the direct storage guard.
- Run focused Rust tests for the touched runtime path.
- Run final hygiene gates recorded in `artifacts/verification.md`; any skipped
  full workspace gate must be labeled `NOT RUN` with rationale and prevents a
  complete disposition unless explicitly accepted in review.

## Seven-Gate DC Bar

1. Reproduction: the site3/site4 direct compare failures are reproduced or
   statically tied to the current tree.
2. Mechanism: the failure is reduced to a named in-envelope mechanism.
3. Ownership: the mechanism lies in the declared write set.
4. Authority: expected behavior is aggregate-storage bookkeeping preserving
   nonnegative layer state, not physics invention.
5. Safety: the fix preserves typed fail-closed guards and does not canonicalize
   material negatives.
6. Testability: focused Rust regression tests distinguish multi-layer debit from
   insufficient-storage failure.
7. Validation: site3/site4 observed harness comparisons reach exit-0
   metric-bearing report emission.

If all seven gates are met, the package must land the fix and may not close as a
diagnostic `HOLD`.

## Intended Write Set

- `docs/work-packages/20260624-snowfreeze-direct-storage-reconciliation-unblock-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3c_r4b.rs`
  or `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs`
- `tests/integration/snowfreeze_observed_frost_depth_contract.rs` only for
  mechanical lint hygiene if package closure gates expose an adjacent
  snowfreeze harness test lint.
- `tools/snowfreeze_observed/**` only if the harness has a defect that blocks
  valid site3/site4 exit-0 metric-bearing report emission. This package does
  not currently expect harness changes.

## Phase Plan

### Phase 0: Scaffold and Reproduce

Create package artifacts, register the package, and run the two failing compare
commands against the current tree.

Exit criteria:

- Package artifacts exist and are truthfully initialized.
- Site3 and site4 failures are recorded with commands and error fields.

### Phase 1: Mechanism and Regression Tests

Trace R4B explicit frost storage projection, identify the layer projection
mechanism, and add focused tests that fail on the pre-fix behavior.

Exit criteria:

- `artifacts/pre-implementation-evidence.md` names the mechanism.
- Regression tests encode both the valid multi-layer debit case and the
  insufficient-storage fail-closed case.

### Phase 2: Production Correction

Implement the bounded projection correction in the direct runtime.

Exit criteria:

- Layer projection preserves aggregate storage within `WB11_ZERO_THRESHOLD`.
- Every updated layer remains finite and nonnegative.
- Material insufficient active storage still fails closed with a typed error.

### Phase 3: Validation and Review

Run focused tests, rerun site3/site4 comparisons, complete review/disposition,
line-count governance, and final package artifacts.

Exit criteria:

- Focused Rust tests pass.
- Site3/site4 comparisons exit `0` and metric-bearing reports are emitted.
- Review findings are dispositioned.
- Gate table has no unjustified `FAIL`, `BLOCKED`, or `NOT RUN` for current
  required gates.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `cargo test -p openwepp-hillslope-orchestrator r4b_explicit_frost_storage`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site3_scan_mandan_nd --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfreeze_observed_compare_site3_direct`
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site site4_ggd498_morris_mn --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfreeze_observed_compare_site4_direct`
- `cargo fmt --check`
- `git diff --check`
- Additional workspace gates as time permits and as recorded in
  `artifacts/verification.md`.

## HOLD Boundaries

The package may close `HOLD` only if the reproduced mechanism is outside the
declared R4B storage projection envelope, canonical authority contradicts the
needed correction, the site fixtures are proven invalid upstream, or required
evidence cannot be generated in this environment. A valid in-envelope
multi-layer projection defect must be corrected in this package.
