# Review Agent B

Status: complete
Evidence mode: Static

Static: reviewed the listed uncommitted HPHYS0290 code, tests, unit-registry changes, and package artifacts. I did not run tests or gates in this review. User-reported local execution is noted, but gate/disposition findings below are based on repository artifacts.

## Findings

### HIGH - Work package is not closeable while required gate, review, verification, and disposition artifacts remain queued

Paths:

- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/package.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/package.md:17`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/package.md:154`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/gate-results.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/disposition.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/review_agent_a.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/review-disposition.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/verification_agent_a.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/verification_agent_b.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/kernel-profile-compliance-checklist.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/worker-handoff.md:3`
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/owned-file-manifest.md:3`

The package evidence is materially ahead of package closure bookkeeping. Contract implementation, contract-test evidence, pre-implementation gate evidence, focused implementation evidence, targeted traces, and full H1..H39 metrics are present, but the package itself still says `Status: queued`, its progress checklist leaves all execution phases unchecked after scaffolding, and `Outcomes & Retrospective` is still pending. More importantly, closure-critical artifacts for gate results, review A, review disposition, both verification artifacts, kernel-profile compliance, owned-file manifest, worker handoff, and final disposition remain `queued` / `not-run`.

This also leaves the required Rust workflow under-recorded. The implementation evidence records `cargo fmt` and focused/adjacent tests, but the package `gate-results.md` does not record the root-required `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `cargo deny check` statuses. The user-reported focused tests and 39/39 runtime completion are useful evidence, but they do not replace package gate recording or dual-review disposition requirements.

Disposition recommendation: accepted, blocking for closure. Keep HPHYS0290 out of final `complete` disposition until the package status/progress/outcomes are updated, required gates are either run and recorded or truthfully marked not-run with rationale, both reviews and verifications are completed, every finding is explicitly dispositioned, and final disposition/handoff artifacts are completed. No production-code rollback is implied by this finding.

### MEDIUM - Stale daily snow publication reset is implemented but not directly regression-tested at the lifecycle boundary

Paths:

- `crates/openwepp-runner/src/hillslope/mod.rs:1714`
- `crates/openwepp-runner/src/hillslope/mod.rs:3853`
- `crates/openwepp-runner/src/hillslope/mod.rs:7786`
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs:8`

The implementation resets `snow.routed_melt_m` and `snow.post_winter_rain_m` before each scheduler run, which is the right lifecycle location after daily climate overlay and before kernel writeback. The WB13 unit tests cover explicit post-winter-rain consumption, missing/negative guards, and flux-over-state precedence. They do not directly prove that a prior day flux is removed before the next day, and the source-level integration test only checks for symbol/call presence.

If `reset_daily_snow_publication_fluxes` were removed or moved after scheduler execution, the HPHYS0290 source-string test and the direct WB13 row-builder tests could still pass. The runtime trace evidence reduces practical risk, but stale-flux behavior is important enough to deserve a direct regression: a two-day lifecycle/surface test or a focused private-unit test that seeds stale fluxes, invokes the reset path, and asserts flux removal plus zero state seeding before scheduler writeback.

Disposition recommendation: follow-up before final closure if stale-flux prevention is claimed as a package deliverable; otherwise defer as targeted test-hardening debt. I do not recommend production-code changes based on the static implementation shape.

### LOW - Unit-registry entry is present, but tests do not explicitly assert the new symbol's typed/domain posture

Paths:

- `crates/openwepp-sim-contract/src/units.rs:1049`
- `crates/openwepp-sim-contract/src/units.rs:1868`
- `tests/integration/sim_contract_boundary_unit_registry.rs:117`
- `tests/integration/sim_contract_boundary_unit_registry.rs:195`
- `tests/integration/sim_contract_boundary_unit_registry.rs:275`
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs:39`

The registry implementation declares `snow.post_winter_rain_m` as a depth, `NonNegativeFinite`, `TypedRequired` boundary and includes it in the required alias list. The integration test now verifies alias resolution and depth units, and the required-alias gate covers presence. However, the typed-posture test's migrated-alias list does not include `snow.post_winter_rain_m`, and the HPHYS0290 source-level test only checks for the symbol string plus the contract citation.

Disposition recommendation: non-blocking follow-up. Add an explicit registry assertion for `snow.post_winter_rain_m` covering canonical symbol, unit, depth dimension, non-negative finite domain, typed-required posture, and contract anchor. Current implementation coverage is adequate to proceed once closure artifacts/gates are fixed.

## Non-Blocking Debt / Follow-Ups

- `crates/openwepp-runner/src/hillslope/mod.rs:6142` retains `_runtime_swe_before_m` in `build_simulation_owned_wb13_row` after HPHYS0290 removes the prior SWE-active inference. This is harmless but leaves a private API seam with a dead argument; remove it or document why the signature is intentionally retained.
- `tests/integration/hphys0290_post_winter_rain_publication_contract.rs` is useful as an anti-regression source check, but it is string-sensitive. Prefer pairing it with behavior tests for lifecycle publication and stale-flux prevention rather than relying on source presence alone.
- `docs/work-packages/20260605-hphys0290-post-winter-rain-publication-lineage-closure-001/artifacts/full-39-suite-metrics.md` records runtime `39/39` and semantic pass `0/39`; closure language should explicitly classify this as scoped publication-lineage closure only, not semantic parity closure.

## QA Pass Statement

No blocking production-code maintainability, stale-flux implementation, or unit-registry implementation defects were found in this static review. The code change is small and cohesive: WB12/WB14 publish a named post-winter rain surface, WB13 consumes it fail-closed with flux precedence, daily publication fluxes are reset before scheduler execution, and the unit registry covers the new boundary symbol.

Package closure is not acceptable yet because required gates, dual review/disposition, verification, kernel-profile checklist, final disposition, and handoff artifacts remain incomplete or unrecorded.
