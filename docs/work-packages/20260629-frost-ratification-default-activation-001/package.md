# FROST Ratification and Default Activation

Status: `EXECUTED-COMPLETE-PRODUCTION-DEFAULT-ACTIVATION`

Package type: contract-first governance plus production-default activation.

Primary gap: `GAP-SNOWFREEZE-002`.

## Objective

Ratify the exercised frost-observation validation invariants
`INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, and `INV-SNOWFREEZE-050`, then make
the direct-production frost path the no-env production default while preserving
explicit compatibility rollback.

## Scope

Included:

- Contract-first ratification of the frost-depth observation method, snow-depth
  control method, and forcing-robust snow/frost rubric.
- Deliberate disposition of the Step 1 diagnostic-local `>0.25`
  systematic-timing-fraction cutoff.
- Measurement-correspondence wording for `frdp` bottom extent versus `thdp`
  top-thaw cap after the H1b check.
- `GAP-SNOWFREEZE-002` re-disposition as open but attributed and bounded.
- Runtime default activation from `DefaultCandidate` to direct production on
  supported modern single-OFE runs, with explicit compatibility fallback for
  current unsupported multi-OFE/Wave-2 and legacy sidecar-discovery runs.
- Full no-regression gates and explicit rollback evidence.

Excluded:

- No Qwet implementation.
- No new frost physics.
- No fixture, public output-schema, or observation-harness default changes.
- No zero-residual claim.

## Required Reading

- `docs/planning/snow-frost-fidelity-strategy.md` section 11 and section 10.3.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`.
- `docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001/`.
- `docs/work-packages/20260629-frost-step2-sleepers-attribution-001/`.
- `docs/work-packages/20260629-frost-step3-residue-parameterization-001/`.
- `docs/work-packages/20260629-frost-residue-cover-implementation-001/`.
- `docs/work-packages/20260629-frost-thaw-residual-diagnostic-001/`.
- `docs/work-packages/20260629-frost-snow-persistence-decomposition-001/`.
- `docs/work-packages/20260629-frost-h1b-state-machine-thaw-asymmetry-check-001/`.
- ADR-0011 and ADR-0017.

## Intended Write Set

- `docs/work-packages/20260629-frost-ratification-default-activation-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `tests/integration/snowfreeze_observed_frost_depth_contract.rs`

## Phase Plan

1. Phase 4 contract ratification:
   - promote `INV-SNOWFREEZE-047/048/050` and `TOL-SNOWFREEZE-007..011` from
     draft/provisional to accepted evaluation authority;
   - document that the Step 1 `>0.25` fraction is not ratified as an invariant;
   - bind `frdp`/`thdp` measurement correspondence after the H1b package;
   - re-disposition `GAP-SNOWFREEZE-002` as open but attributed and bounded.
2. Phase 5 activation:
   - add the frost Policy B analog: attributed/bounded residuals plus
     full-surface no-regression, not zero residual closure;
   - flip no-env `DefaultCandidate` to direct production on supported modern
     single-OFE runs;
   - preserve compatibility fallback for unsupported multi-OFE/Wave-2 and
     legacy sidecar-discovery runs;
   - keep explicit compatibility rollback and fail-closed runtime selection.
3. Verify:
   - `cargo fmt --check`;
   - `cargo clippy --workspace --all-targets -- -D warnings`;
   - `cargo test --workspace`;
   - `cargo deny check`;
   - source-level anti-evasion guards;
   - focused runtime-selection and contract-marker tests.

## Exit Criteria

- Contract ratification lands before runtime default activation.
- The `>0.25` diagnostic-local fraction is adjudicated and not silently promoted.
- H1b `frdp`/`thdp` metric correspondence is accepted as bounded residual
  context, not a state-machine blocker.
- `GAP-SNOWFREEZE-002` records the bounded residual decomposition.
- Default no-env supported modern single-OFE runtime selects direct production;
  unsupported multi-OFE/Wave-2 defaults, legacy sidecar-discovery defaults, and
  explicit compatibility selection remain rollback.
- Full gates pass or the package closes `HOLD`.

## Disposition

`EXECUTED-COMPLETE-PRODUCTION-DEFAULT-ACTIVATION`.

The package ratified the frost observation invariants, re-dispositioned
`GAP-SNOWFREEZE-002` as open but attributed and bounded, and activated direct
production as the supported modern single-OFE no-env hillslope default. Explicit
compatibility rollback remains available. Current unsupported multi-OFE/Wave-2
and legacy sidecar-discovery no-env runs fall back to compatibility with explicit
reason strings until separate direct-surface promotion packages cover them.
