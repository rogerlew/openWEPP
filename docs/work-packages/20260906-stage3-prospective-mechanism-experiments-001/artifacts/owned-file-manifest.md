Ran: provisional source inventory; no source edits by this artifact author.

Roots: A=/workdir/openWEPP; F=/tmp/openwepp-controlled-mechanisms-Hb6uS2/F;
R=/tmp/openwepp-controlled-mechanisms-Hb6uS2/R. F/R are detached at A commit
2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3 with isolated working deltas.
A is compared to e89befa4678eadec039b3e7f7fe0a176af8e9dc5; F/R to A.
Inventory recipe: union `git diff --name-only <base> -- '*.rs'` and
`git ls-files --others --exclude-standard -- '*.rs'`, excluding unrelated tmp/;
count newline bytes and hash each present file. No deletions were observed.
Terminal complete path/content custody remains source-and-build-manifest.json
and reproduction/ snapshots/patches; this provisional table is not a frozen
executable identity. R-PC1 can change after this inspection.

Paths below are relative to crates/, except explicit tests/ paths.

| Arm | Changed/new path | Lines |
|---|---|---:|
| A | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs | 2447 |
| A | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs | 1201 |
| A | openwepp-hillslope-orchestrator/src/lib.rs | 310 |
| A | openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs | 2910 |
| A | openwepp-hillslope-orchestrator/src/stage3_mechanism_experiment_audit.rs (new) | 453 |
| A | openwepp-land-surface-energy/src/lib.rs | 341 |
| A | openwepp-land-surface-energy/src/solver_covered_evaluation.rs | 2651 |
| A | openwepp-land-surface-energy/src/solver_covered_solve.rs | 1423 |
| A | openwepp-land-surface-energy/src/solver_mechanism_audit.rs (new) | 689 |
| A | openwepp-land-surface-energy/src/transaction.rs | 2952 |
| A | openwepp-land-surface-energy/src/transaction_v3_bridge.rs | 235 |
| A | openwepp-runner/src/hillslope/03_tests.rs | 2933 |
| A | openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs (new) | 478 |
| A | openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs | 2654 |
| A | ../tests/integration/land_surface_energy_balance_authority_contract.rs | 1833 |
| A | ../tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs | 2436 |
| F | openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs | 1559 |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/mod.rs | 32 |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs | 2699 |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs | 2954 |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs | 2298 |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs | 1260 |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_carrier_evaluation.rs (new) | 262 |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_feed_forward_tests.rs (new) | 439 |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/terminal_carrier_provider.rs (new) | 96 |
| F | openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs | 2533 |
| F | openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs | 2942 |
| F | openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_feed_forward_tests.rs (new) | 174 |
| F | openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs | 2944 |
| F | openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase/snow_boundary.rs | 276 |
| F | openwepp-hillslope-orchestrator/src/v11_covered/open_snow_tail_tests.rs | 295 |
| R | openwepp-land-surface-energy/src/solver.rs | 797 |
| R | openwepp-land-surface-energy/src/solver_component_dependency_graph.rs (new) | 384 |
| R | openwepp-land-surface-energy/src/solver_component_dependency_graph_tests.rs (new) | 316 |
| R | openwepp-land-surface-energy/src/solver_component_dependency_replay.rs (new) | 426 |
| R | openwepp-land-surface-energy/src/solver_component_dependency_replay_tests.rs (new) | 1266 |
| R | openwepp-land-surface-energy/src/solver_covered_evaluation.rs | 2700 |
| R | openwepp-land-surface-energy/src/solver_covered_solve.rs | 1548 |
| R | openwepp-land-surface-energy/src/solver_mechanism_audit.rs | 710 |
| R | openwepp-land-surface-energy/src/solver_tests.rs | 1894 |
| R | openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs | 494 |

Provisional SHA-256 anchors (full path resolved by table):

| Arm/file | SHA-256 |
|---|---|
| A controlled_mechanism_experiments.rs | b81f0bc5a26237bb16c9f0968f12ba0f2e55265054177dd6a631edde427d2bfe |
| F terminal_carrier_evaluation.rs | fd4c790255c0b76b2b599399c0e0b0e215fc56799b1929b314d928e0a2aa4bc1 |
| F snow_stage3_v11_terminal_execution.rs | ed090ee54d61a1a74e7ca392459466843a9828d9dabd64f182082e02ac972834 |
| R solver_component_dependency_replay.rs | 0c557403ded163aa67867a28d3eecdfd9444908da3666ab3469670f4befdb7ce |
| R solver_covered_solve.rs | fdb5ed651eabd847e18076cc74004282af29613a4d976dc29e7c28d48e6ec200 |

Instruction discovery for these three artifacts resolves root AGENTS.md and
docs/work-packages/AGENTS.md. Readiness additionally used the complete
Calibration and Identifiability Schema in science-contract-spec.md; line-count
disposition used crates/AGENTS.md's explicit threshold policy. These artifacts
do not modify scientific authority or source ownership.
