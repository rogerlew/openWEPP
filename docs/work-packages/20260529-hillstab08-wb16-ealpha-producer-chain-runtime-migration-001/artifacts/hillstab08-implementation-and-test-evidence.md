# HILLSTAB08 Implementation and Test Evidence

Status: complete  
Evidence mode: mixed (`Static` + `Ran`)

## Implementation Evidence (Static)
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
  - Projects WB16 producer-chain controls and seeds onto runtime surfaces:
    `inrcov`, `rilcov`, `rrinit`, `rspace`, `width`, `rtyp`, `bb_seed`,
    `bbb_seed`, `flivmx_seed`, `hmax_seed` (OFE-scoped + primary aliases).
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`
  - Extended growth projection parameter coverage to include `bbb`, `flivmx`,
    and `hmax`.
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Added WB16 runtime producer
    (`frcfac -> rdat(alpha) -> alphay -> eplane`) with typed guard posture,
    OFE diagnostics publication (`ofe{n}_frcteq`, `ofe{n}_alpha`), and
    `ealpha` publication.
  - Updated WB11 seeding flow to prefer runtime-produced `ealpha` and use
    compatibility seeding only when producer inputs are unavailable.
- `tests/integration/cli03_runner_contract_derived_tests.rs`
  - Updated WB16 execution provenance expectation to runtime-producer path for
    canonical fixture run.

## Test Evidence (Ran)
- Targeted:
  - `cargo test -p openwepp-hillslope-orchestrator management_runtime_surfaces_project_required_pl_controls_and_seeds`
  - `cargo test -p openwepp-runner hillstab08_wb16_producer`
  - `cargo test -p openwepp --test cli03_runner_contract_derived_tests cli03_fixture_run_publishes_wb16_ealpha_runtime_seed_provenance`
- Full gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
