# Owned File Manifest

Evidence mode: Static.

Package files:

- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/**`

Expected implementation files:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs`
- `docs/work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/artifacts/worker-handoff.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Actual implementation files stayed within the declared write set. The
orchestrator direct-runtime crate did not need changes because the required
publication row and report types were already public.
