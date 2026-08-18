# Quota-Interruption Intake

Evidence class: `Static + Ran`

Date: `2026-08-17`

Status: `executing / quota-interrupted remediation checkpoint / exact-head compile and public-path integration incomplete`

## Git Identity And Custody

- Exact working HEAD: `099b15d2b13f52899e65b8d266a3c067cb9773c1`.
- Required Child-3 source checkpoint: `70d855ff6ccc5f4387547f05969079c3db6b353f` (`v8 work in progress`).
- Child-3 checkpoint parent and independent-review target: `dfc7cf971284d772246f147382f4bb8a2292ee4c`.
- `099b15d2b` is the already-pushed merge of the Nix/devbox branch with `70d855ff6`; its source delta from `70d855ff6` is empty. Its only tree delta is the separately authorized Nix/devbox package and tooling. The active branch was clean at intake.
- The historical surface-liquid custody HOLD, the independent reviews against `dfc7cf971`, all failed/rejected attempts, prior gate logs, the active kickoff prompt, and the `70d855ff6` WIP checkpoint remain preserved. No prior finding is marked corrected by this intake.

## Complete Changed-File Inventory

Child-3 checkpoint inventory, `dfc7cf971..70d855ff6`:

- added: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_derived_ingress.rs`
- modified: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_forest.rs`
- modified: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs`
- added: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/multi_tile_runtime.rs`
- added: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_input_projection.rs`
- added: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/v8_rollback.rs`
- added: `crates/openwepp-land-surface-energy/src/covered_oracle_conformance_tests.rs`
- modified: `crates/openwepp-land-surface-energy/src/covered_output.rs`
- modified: `crates/openwepp-land-surface-energy/src/error.rs`
- modified: `crates/openwepp-land-surface-energy/src/lib.rs`
- added: `crates/openwepp-land-surface-energy/src/numerics.rs`
- modified: `crates/openwepp-land-surface-energy/src/solver.rs`
- modified: `crates/openwepp-land-surface-energy/src/transaction.rs`
- modified: `crates/openwepp-vegetation/src/persistent_phase.rs`
- modified: `crates/openwepp-vegetation/src/v8_candidate.rs`
- modified: this package's `checkpoint-diff-reconciliation.md`, `final-disposition.md`, `owned-file-manifest.md`, and `package.md`
- added: this package's `review-finding-disposition.md`, `review_agent_a.md`, and `review_agent_b.md`
- modified: `tests/integration/land_surface_energy_real_hydrology_shadow_contract/covered_forest_tests.rs`

The exact working HEAD additionally contains only the 15-file Nix/devbox branch delta recorded in `docs/work-packages/20260814-nix-agent-devbox-feasibility-001/`; it does not alter Child-3 Rust or test bytes.

## Exact-Head CI Evidence At Intake

No exact-head CI, focused remediation, benchmark, heavy-gate, fresh-review, or terminal-verification evidence existed for `70d855ff6` or `099b15d2b` at intake. Historical evidence remains commit-scoped and is not promoted.

The first direct shell attempts for all four required checks exited `127` with `cargo: command not found`. The host migration contract requires the pinned Nix development shell, so the commands were repeated as `OPENWEPP_TASK_ID=child3-resume nix develop -c cargo check -p <crate>` without changing source:

- `openwepp-land-surface-energy`: PASS.
- `openwepp-vegetation`: PASS.
- `openwepp-biogeochemistry`: PASS.
- `openwepp-hillslope-orchestrator`: FAIL with `E0599` at `v8_input_projection.rs:282`: no method named `solver_ready` for `V8ProjectedTileRuntimeInput`. Five unused-import groups corroborate the interrupted conversion.

## Source-Level Intake Findings

- The strict projection calls a wholly absent solver-ready conversion.
- Projected/validated physics fields remain public and caller-mutable, and a public method exposes an unexported type.
- the current radiation projection independently consumes forcing-owned ground VIS/NIR albedo instead of the digest-bound LSE tile owner.
- vertical-rank/component/occupancy validation is duplicated and incomplete before physics; soil-thermal layer ordering is not sealed in projection.
- legacy raw covered/open/V8 wrappers remain public bypasses and the positive endpoint fixture still supplies raw physics, trials, bindings, ingress, and companion placeholders.
- the internal multi-tile runtime has no public consumer, accepts raw mutable inputs, uses bit-exact fraction summation rather than canonical topology closure, stops before V8/BGC composition, and reduces covered OFE energy to ground-only operands.
- `V8RollbackSnapshot` is implemented but disconnected from execution; no phase failure-injection matrix proves actual-byte rollback.
- `solver.rs` is exactly 3,025 lines and therefore remains above the mandatory 3,000-line closure hard stop.

## Dependency-Ordered Remediation Plan

1. Complete sealed solver-ready projection from validated canonical owners, including LSE-owned albedo, exact soil ordering, topology-derived rank bindings, and projection poison tests.
2. Repair the orchestrator compile before broader tests.
3. Connect projection to a strict heterogeneous multi-tile runtime with one combined authorization, final-from-beginning rebuilds, owner-derived ingress, complete local/OFE energy, V8 receipts/persistent phase, and BGC candidate.
4. Make the strict canonical-owner operation the sole closure-eligible public endpoint and retire raw public bypasses.
5. Connect actual-byte rollback capture and phase failure injection to that endpoint.
6. Add focused public execution, energy, vegetation/BGC, rollback, and API-exclusion acceptance tests.
7. Split `solver.rs` below 3,000 lines without changing production behavior, then reconcile exact file counts and package evidence.
8. Run focused gates, obtain fresh exact-byte Rust and science reviews, remediate every accepted finding, and only then dispatch comparator benchmarks/heavy gates and two independent terminal verifiers.
