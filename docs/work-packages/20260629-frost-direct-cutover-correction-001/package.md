# FROST Direct Cutover Correction

Status: `EXECUTED-COMPLETE-DIRECT-CUTOVER-CORRECTION`

Package type: defect correction for default-runtime cutover scope.

## Objective

Correct the frost ratification/default-activation package so the no-env hillslope
`DefaultCandidate` selects direct production for all current hillslope runs
rather than falling back to the compatibility runtime for multi-OFE/Wave-2 or
legacy sidecar-discovery runs.

## Rationale

The prior package incorrectly preserved compatibility fallback for unsupported
surfaces. Operator correction on 2026-06-29 clarified that the frost validation
arc's purpose was to retire the compatibility runtime boundary, not to keep
compatibility as a default fallback. Any direct-surface failure must therefore
close as a blocker or be fixed, not be hidden behind compatibility selection.

## Scope

Included:

- Contract-first amendment superseding the fallback wording in
  `INV-SNOWFREEZE-084`.
- Runtime-selection correction so no-env `DefaultCandidate` resolves to direct
  production independent of OFE count or legacy sidecar-discovery mode.
- Tests proving legacy sidecar-discovery no-env runs select direct production.
- Focused gates for the previously failing legacy-discovery replay and runtime
  selection coverage.

Excluded:

- No new frost physics, Qwet, fixture fitting, or observation-threshold change.
- No parser compatibility-mode deletion in this package.
- Physical removal of every explicit compatibility test/helper may be a follow-on
  mechanical deletion package if source-wide callers remain after default
  cutover.

## Intended Write Set

- `docs/work-packages/20260629-frost-direct-cutover-correction-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/normalization.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r3c_r4b.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4pqz.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs`
- `tests/integration/snowfreeze_observed_frost_depth_contract.rs`

## Gates

- Focused direct-runtime selection tests pass.
- Previously compatibility-fallback legacy sidecar-discovery replay passes under
  direct production or closes with a named blocker.
- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo test --workspace` or `cargo nextest run --workspace --profile full`
  when available and compatible with the current suite.
- `cargo deny check`.
- Authority-suite anti-evasion guards.

## Disposition

Executed complete. `SC-SNOWFREEZE-001` v115 supersedes the v114 fallback
carve-out: no-env `DefaultCandidate` selection resolves to direct production for
current hillslope runs, including legacy sidecar-discovery and multi-OFE/Wave-2
surfaces. Explicit compatibility selection remains only as a deprecated
comparator/deletion seam pending a mechanical removal package; it is no longer
advertised as default rollback provenance.

The cutover exposed a real direct-runtime storage ledger defect in legacy
sidecar-discovery replay. PMET can return negative soil-evaporation demand to
soil storage (`soil_evaporation_storage_return_m`) without counting it as
evapotranspiration; direct storage reconciliation now carries that operand as an
explicit source. Focused PL14S replay, R7E selector, and R7H frost storage-source
tests pass under direct production.

The correction also closed two direct multi-OFE defects that compatibility had
masked. Direct execution now records Wave-2 provenance from the same seeded
direct surface used by the day-input builder, so no-env multi-OFE runs report
Wave-2 enabled without scheduler fallback. Direct normalization now applies the
upstream/downstream area ratio to transfer carry, so public WAT `UpStrmQ`
matches upstream `QOFE` scaled onto the receiving OFE. Release metadata sidecars
are written through an atomic temp-file rename so parallel `nextest` processes
cannot observe a partial JSON sidecar.

Full closure passed: `cargo fmt --check`, `cargo clippy --workspace
--all-targets -- -D warnings`, `cargo nextest run --workspace --profile full`
(`1865` passed, `1` skipped), `cargo deny check`, authority anti-evasion script,
and `auth11_required_suite_obligation_guards_contract`.
