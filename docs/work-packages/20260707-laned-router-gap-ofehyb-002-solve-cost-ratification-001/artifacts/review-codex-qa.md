# Codex QA Review: GAP-OFEHYB-002 Bare-Skin Direct Equilibrium

Evidence class: Static + Ran.

QA disposition: NO-GO for closure.

## Findings

### High - H2637 output deltas are real but undispositioned for a cost-only/no-publication-surface change

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:189` authorizes the direct evaluator only as a cost-only exact evaluator; `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:200` says it may not alter publication surfaces.
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:483` repeats that rev 35 makes no publication-surface change.
- Raw after-effective evidence contradicts any implicit byte/no-delta assumption: baseline hashes in `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/baseline-profile.md:61` show `H2637.hbp` `939e...` and `H2637.pass.parquet` `a26d...`, while after-effective manifest output hashes in `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch-after-effective/output/openwepp_hillslope_run_manifest.json:38` show `H2637.hbp` `bfb2...` and `H2637.pass.parquet` `44e3...`.
- The manifest summary also moved: baseline `total_routed_outlet_m3` / `total_clamp_m3` are at `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch/output/openwepp_hillslope_run_manifest.json:103`, while after-effective values differ at `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch-after-effective/output/openwepp_hillslope_run_manifest.json:103`.
- No artifact accepts, bounds, or explains those deltas: `artifacts/timing-and-fidelity.md:3` and `artifacts/ratification-audit.md:3` are still QUEUED.

Impact: the optimization may still be scientifically acceptable, but closure cannot proceed until the package records an output-delta audit with magnitude, tolerance authority, and whether the selector remains unpromoted.

### High - Required package gates and disposition evidence remain queued/NOT RUN after implementation

- The package requires review, verification, gate evidence, disposition, and handoff at `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/package.md:176`, requires artifacts at `package.md:199`, and requires gates at `package.md:215`.
- Exit criteria require before/after H2637 counters, ratification disposition, and every gate PASS or explicitly non-applicable with evidence at `package.md:244`.
- Current artifacts do not satisfy that: `artifacts/gate-results.md:3` is Static/QUEUED and `artifacts/gate-results.md:7` through `artifacts/gate-results.md:24` still mark every listed gate NOT RUN, including doc lint, contract checks, Case-4 ladder, H2637 timing/profile, clippy, full nextest, deny, and line-count governance.
- `artifacts/implementation.md:3`, `artifacts/optimization-plan.md:3`, `artifacts/contract-amendment.md:3`, `artifacts/disposition.md:3`, `artifacts/final-disposition.md:3`, and `artifacts/worker-handoff.md:3` are also QUEUED.
- The required-reading map still says Pending for core reads at `artifacts/required-reading-map.md:3` and `artifacts/required-reading-map.md:7`.

Impact: even with passing focused tests, the package cannot truthfully close or claim GAP-OFEHYB-002 disposition.

### Medium - Edge-case tests are useful but too narrow to close the exact-equivalence/output-risk surface

- Nonzero rain term is covered only in the private branch-value equivalence table at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1588`.
- `ko=0` is covered at `kinematic_wave.rs:1599`, but the assertion compares direct output to the existing iterated implementation at `kinematic_wave.rs:1614`; it does not independently assert the intended zero-resistance branch behavior or full `solve_cell` LOW/HIGH chain behavior.
- Near-`Qc` coverage exists just below/above the laminar edge at `kinematic_wave.rs:1576`, but again only at branch-value level.
- The only full cell-solve counter test for the new direct path uses `ko=500`, `skin_rain_term=0`, and one ordinary state at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:561`.

Impact: these tests are good smoke coverage, and they passed locally, but they are not enough to explain the observed H2637 output deltas or prove the composed residual path across `ko=0`, rain-term, and near-crossover cases.

### Medium - "Effective-zero" addend coverage does not match the wording's risk

- The gate predicate is exact zero-based at `kinematic_wave.rs:250`: `element_tip_height_m == 0.0`, `roughness_concentration == 0.0`, `leaf_area_index == 0.0`, `canopy_height_m == 0.0`, `vegetation_drag_coefficient == 0.0`, and `manning_n == 0.0`.
- The test at `kinematic_wave.rs:1628` covers disabled addends only by exact-zero combinations and one active non-bare case; it does not include near-zero positive operands or prove why exact-zero-only is the intended interpretation of "effective-zero."
- `SC-OFEROUTE-002.md:194` says "No active addend" follows the friction guards, while the revision history at `SC-OFEROUTE-002.md:439` uses "zero/effective-zero guards." That wording should either be narrowed to exact-zero component-absence or backed by explicit positive-near-zero tests/threshold authority.

Impact: not an immediate correctness failure if exact-zero is the intended contract, but the current wording/test pair leaves avoidable ambiguity for future maintainers.

### Low - Maintainability governance is missing for the now-2000+ line routing module

- `kinematic_wave.rs` is 2109 lines after this change. The crate guidance treats 2000+ line Rust files as WARN and requires decomposition rationale plus follow-on split intent.
- The package explicitly requires `.rs` line-count governance at `package.md:235`, but `artifacts/gate-results.md:24` still says NOT RUN.

Impact: not a closure blocker by itself under the 3000-line hard threshold, but it must be recorded before package closure.

## Non-Blocking Debt / Follow-Ups

- Move the bare-skin direct-equilibrium tests toward a table helper that names branch, `ko`, rain term, crossover relation, and expected behavior. The current tuple table is compact but hard to audit.
- Add a composed `solve_cell` or `implicit_step` test for nonzero rain term, `ko=0`, and near-crossover depths so future changes do not rely only on private fixed-point equivalence.
- Record whether H2637 HBP/pass deltas are byte-level metadata/order effects or numeric publication changes. If numeric, quantify maxima and identify the accepted tolerance authority.

## Coverage Notes

- Ran: `git diff --check` passed.
- Ran: `cargo fmt --check` passed.
- Ran: `cargo test -p openwepp-hillslope-orchestrator bare_skin_direct_equilibrium -- --nocapture` passed: 3 tests.
- Ran: `cargo test -p openwepp-hillslope-orchestrator branch_warm_seed_preserves_solution_and_reduces_or_matches_map_work -- --nocapture` passed: 1 test.
- Ran: `cargo test -p openwepp-hillslope-orchestrator hybrid_source_memory_allows_implicit_after_cooldown -- --nocapture` passed: 1 test.
- Not run by this QA pass: doc lint, contract/profile/BEI checks, SC unit compliance, retained Case-4 full-hybrid ladder, full H2637 rerun, clippy, full nextest, cargo deny, authority anti-evasion guards.

## Final QA Statement

The code-level unit smoke checks for the new direct evaluator pass, and the raw after-effective profile shows the intended counter movement (`implicit_equilibrium_map_evaluations = 0`). The package is still NO-GO because required closure artifacts are stale and the observed H2637 output deltas have not been audited or dispositioned.
