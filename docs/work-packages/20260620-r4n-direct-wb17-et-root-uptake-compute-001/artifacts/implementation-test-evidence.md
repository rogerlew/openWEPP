# Implementation Test Evidence

Status: complete.

Static: R4N added focused direct-runtime tests in
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r4n.rs`
and updated aggregate/counter tests in the existing direct-runtime and runner
test modules.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r4n -- --nocapture` passed:
  5 tests.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`
  passed: 36 tests.
- `cargo test -p openwepp-hillslope-orchestrator r4 -- --nocapture`
  passed: 26 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture` passed: 2 tests.
- `cargo test -p openwepp-hillslope-orchestrator
  r2a_direct_skeleton_runs_noop -- --nocapture` passed: 1 test.
- `cargo check -p openwepp-hillslope-orchestrator` passed during iteration.

Focused fixtures:

- `r4n_surface_et_matches_wb17_soil_evaporation_layer_mutation_fixture` covers
  direct surface ET component computation and layer mutation.
- `r4n_root_uptake_matches_swu_fixture_and_finalizes_aggregate_et` covers
  post-WB19 SWU/root-uptake vectors and aggregate ET finalization.
- `r4o_consumes_r4n_surface_et_mutated_layer_state_when_present` proves WB19
  consumes ET-mutated layer state when the R4N surface span has run.
- `r4n_feeds_r4b_final_et_not_handoff_or_publication_aliases` proves R4B
  consumes R4N final ET rather than the R4E-H handoff or publication-side
  reconstruction.
- `r4n_fails_closed_on_missing_upstream_and_invalid_domain` covers missing
  producer and invalid-domain typed failure behavior.

Counter and span evidence:

- Aggregate direct-runtime tests now account for R4N surface and root/final
  spans in phase entries, direct operations, state mutations, downstream
  operands, and shadow projections.
- Runner opt-in counter tests now include R4N spans in the explicit
  direct-runtime lower bound while the default-disabled runner fixture remains
  zero-counter.
