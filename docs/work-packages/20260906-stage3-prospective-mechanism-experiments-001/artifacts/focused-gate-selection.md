# Remaining focused default-debug selection

Static: source, canonical strategy, nearest instructions and existing execution logs inspected. No builds/tests executed by this author. Assigned write set is this artifact only; find-agents returns root and work-package AGENTS.md. Sources: owned-file-manifest.md, full-validation-variant-review-b.md, validation-posture-review-a.md, gate-results.md and named raw logs below. Selection does not replace full-profile optimized correctness or disposition its failures.

## Existing execution and limits

- A-LSE-debug-01: owning LSE crate 146/146 PASS. R-PC1-SG1-focused-02: owning LSE crate 150/150 PASS, including R dependency replay and common LSE observer. These satisfy current focused LSE debug execution when source identity is unchanged.
- authority-orchestrator-observer-05: four common observer tests PASS in unoptimized test profile. runner-clock-test-05 records the common clock test; retain this named evidence subject to exact-source reconciliation. No reason to repeat unchanged observer/clock tests merely because F/R exist.
- F-focused-04: unoptimized test profile, four typed feed-forward tests and the separately selected ignored actual-carrier forced-reference oracle all PASS (5/5). The oracle executes 101 comparisons and is not a missing debug gate. Do not rerun it solely as part of this selection.
- A-full-release-01 executed 4202 tests: 4022 PASS, 180 FAIL; F-full-release-01 executed 4207: 4005 PASS, 202 FAIL. They are completed full-profile executions, not passing gates. R-full-release-01 has no terminal Summary in the inspected log; do not infer completion. Full-release execution already covers named nonignored common, ownership, restart and kernel tests when admitted by the recorded full inventory; a completed full command cannot imply every such test passed.
- In F full-release, actual failures include ordinary_physical_reuse_is_byte_identical_to_forced_double_evaluation, wb14_failure_preserves_every_resource_candidate_byte, duplicate_configured_mapping_rejects_real_stage3_day_without_owner_or_clock_mutation and before/after_snow_reappearance_round_trips_and_resumes_byte_identically. These require explicit defect/environment/source-authority disposition independently of this debug selection. A later unrelated PASS does not erase them.

## Minimal remaining commands

Use each arm's existing isolated target directory and recorded offline environment. Commands below intentionally omit --release, preserving default debug assertions and overflow checks. Prefix with `nix develop --offline -c env RUST_MIN_STACK=67108864 CARGO_NET_OFFLINE=true`. Execute serially. `--profile full` preserves canonical nextest posture while `-E` narrows only the separately named debug obligation. Record selected inventory/counts and no-match errors before claiming coverage.

1. Common A: actual runner bootstrap/publication and untouched dependency debug invariants, one invocation:

```text
cargo nextest run -p openwepp-runner -p openwepp-vegetation -p openwepp-kernel-contract --profile full --success-output final -E '(package(openwepp-runner) & (test(explicit_stage3_runner_fixture_bootstraps_before_day_execution) | test(accepted_stage3_real_runner_routes_lane_d_and_publishes_summary) | test(duplicate_configured_mapping_rejects_real_stage3_day_without_owner_or_clock_mutation))) | (package(openwepp-vegetation) & (test(occupancy_solver::potential::tests::) | test(occupancy_solver::constitutive::tests::))) | (package(openwepp-kernel-contract) & (test(symbol_registry_) | test(indexed_)))'
```

Runner cases use actual fixture/consumer paths; release-only controlled_mechanism_experiment explicitly refuses debug and must not be selected. Occupancy constitutive/potential tests execute the diagnostics.validate debug assertions; kernel registry/indexed tests exercise the symbol-index assertion and mutation ordering. These dependencies are unchanged between A/F/R, so this owning-surface debug evidence is reusable under canonical exact-source/configuration rules. The F consumer still requires its own changed-path tests below.

2. F: typed carrier custody and actual canonical failure/rollback orchestration, plus restart:

```text
cargo nextest run -p openwepp-hillslope-orchestrator -p openwepp-persisted-restart-v1 --profile full --success-output final -E '(package(openwepp-hillslope-orchestrator) & (test(component_carrier_rejects_stale_inner_seal_and_fresh_boundary_substitution) | test(rainy_and_reappearance_reuse_gate_rejects_every_authority_substitution_and_terminal_mode) | test(terminal_reuse_allows_only_sealed_discovery_trial_identity_rebinding) | test(receipt_chain_rejects_identity_support_owner_phase_event_and_order_poisons) | test(canonical_covered_failure_matrix_never_completes_or_publishes_a_failed_envelope) | test(canonical_covered_physical_prefix_poisons_reject_with_exact_rollback) | test(wb14_failure_preserves_every_resource_candidate_byte))) | (package(openwepp-persisted-restart-v1) & test(snow_stage3_v11::tests::))'
```

The selected custody tests cover stale sealed boundaries, foreign authority substitutions, bounded legitimate discovery rebind, receipt identity/owner/order poisons, canonical final failures, exact rollback, WB14 resource rollback, and actual terminal/reappearance restart chronology. Existing F-focused-04 supplies one-call versus both references and sole-call error precedence; adding it to this filter would repeat already satisfied evidence. If the full profile excludes an explicitly required selected case, record that fact and run that case separately using its existing explicit ignored/profile posture, without editing profile policy.

3. Only if current common observer or clock source differs from its PASS snapshot, append a targeted refresh (otherwise evidence reuse):

```text
cargo nextest run -p openwepp-hillslope-orchestrator -p openwepp-runner --profile full --success-output final -E '(package(openwepp-hillslope-orchestrator) & test(stage3_mechanism_experiment_audit::tests::)) | (package(openwepp-runner) & test(controlled_mechanism_clock_is_monotonic))'
```

4. Authority integration contracts already execute in optimized full where matched. For a changed contract/source-guard correction, rerun only the named integration binary (snow_terminal_enthalpy_event_numerics_contract and/or land_surface_energy_balance_authority_contract) on the relevant source cut. These are source/authority gates, not an additional default-debug physics requirement. Preserve prospective versus historical expected-red distinctions; do not change a historical expected-red test into an acceptance claim by omission.

## Sufficiency boundary

Canonical testing-and-gate-strategy sections on focused claims and command-plan revision, plus both independent build-variant reviews, require affected debug surfaces; they do not require unrelated full-debug repetition. Default-debug dependency tests cannot cure optimized scientific failures. This selection is the minimum bounded supplement, not a statement that remaining full-correctness, anti-evasion, lint, docs, source custody, complete scientific/output closure, or review obligations are passed. Parent reconciles terminal source changes and actual outcomes before disposition.
