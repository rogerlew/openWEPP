# PERFDEEP07 - Zero-Cost Disabled Path and Direct-Frame Hydrology Fast Path

Status: executed 2026-06-19. Disposition: HOLD.

HOLD reason: the ordered P0 default-disabled H2637 timing gate did not pass.
The retained disabled-path patch improved the PERFDEEP05 regression but the
best viable single run was still `685.85 s`, above the required `<= 676.67 s`
threshold. Direct-frame hydrology implementation was therefore not started.

Package type: performance implementation / ADR-0025 direct-frame execution
package.

## Objective

First remove or bypass the default-disabled dense-first compatibility tax, then
implement a bounded opt-in direct-frame hydrology fast path over typed
`HillslopeDayFrame`/view APIs.

The default path must be zero-cost-when-disabled for PERFDEEP02/03/05/07
plumbing. When all PERFDEEP opt-ins are disabled, the scheduler must not build
or resolve compact dense views, indexed shadow surfaces, direct-frame shadow
state, hot symbol tables, or other direct-frame compatibility structures.

After that P0 gate passes, add a bounded direct-frame hydrology daily OFE chain
that keeps all new behavior fail-closed behind an explicit opt-in. The migrated
normal success path must not use `HillslopeKernelRequest`,
`KernelWritebackPayload`, `HillslopeWritebackSurface`, `BoundarySymbol`,
`BoundaryValue`, `SymbolRegistry::id_of`, logical fallback reads, or
dense/logical refresh/flush.

## Rationale

PERFDEEP06 closed `READY-FOR-PERFDEEP07` and made the default-disabled
regression load-bearing. PERFDEEP05 final-code default-disabled H2637 measured
`701.95 s` versus the `669.97 s` reference, while PERFDEEP03 default-disabled
measured in the `697-708 s` band. That means accumulated opt-in plumbing is a
shipped regression on `main` even when the failed islands are off.

The opt-in path also remains a no-go: PERFDEEP05 removed the PERFDEEP04
full-sync hotspot but still measured `911.11 s` versus `669.97 s`. The
remaining opt-in costs are cached daily refresh, logical dense writeback apply,
`SymbolRegistry::id_of`, and dirty flush. More seam shaving is insufficient.

PERFDEEP07 therefore has two ordered obligations:

1. make the disabled path zero-cost for dense/direct-frame compatibility work;
2. add a bounded direct-frame hydrology fast path that proves the migrated
   success path can run without symbol/logical/writeback payload machinery.

## Scope

In scope:

- audit and patch default-disabled scheduler/request construction so dense-first
  and direct-frame compatibility plumbing is bypassed when opt-ins are off;
- preserve default-disabled H2637 identity for HBP, WAT, PASS, plot/loss, and
  manifest/provenance surfaces already protected by prior PERFDEEP gates;
- measure default-disabled H2637 no-UI endpoint at least three clean times,
  recording min/median/max seconds and RSS, with same-machine control where
  feasible;
- implement one bounded direct-frame hydrology daily OFE chain behind a new or
  existing explicit opt-in after the disabled-path gate passes;
- add typed frame/context/view structures or adapters needed for the bounded
  chain, using PERFDEEP06 field and publication ledgers as the source plan;
- add shadow identity tests/fixtures for migrated frame outputs and arrays;
- add static proof that migrated direct-frame success paths contain no map,
  symbol, writeback payload, dense/logical refresh, or logical fallback
  machinery;
- record layout/type-size and allocation evidence for new frame structures;
- update package artifacts, roadmap/catalog status, and review/verification
  disposition.

Out of scope:

- default activation of PERFDEEP02/03/05/07 opt-ins;
- physics or numerical formula changes;
- canonical `SC-*` contract changes unless the package is amended first;
- output schema changes;
- deleting logical/indexed surfaces globally;
- porting all 14 phases unless a bounded implementation plan and gates are
  added before code edits;
- accepting a faster opt-in while leaving the default-disabled regression on
  `main`.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/package.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/package.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-working-set-inventory.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-direct-frame-api-plan.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-layout-allocation-ledger.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-no-hot-loop-map-proof.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-follow-on-package-sequence.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05_disposition.md`
- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05-profile.md`

Required before Rust edits:

- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` if invariant authority,
  guard semantics, diagnostic attribution policy, output meaning, or process
  physics would change.
- `tests/AGENTS.md` before adding or editing root tests.

Source inventory:

- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-hillslope-output/src/**`

## Dependencies

- PERFDEEP06 is the direct planning authority for this package.
- ADR-0025 and `docs/architecture/array-native-runtime-specification.md` remain
  binding design authority.
- PERFDEEP05 profile and disposition are the immediate empirical inputs.
- Existing H2637 output identity gates are protected surfaces.

## Intended Write Set

- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md`
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs` only if phase routing
  metadata is required
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**` only for bounded
  new typed frame/view contract exposure
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` only
  for typed publication projection shadowing

Execution amendment 2026-06-19:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  for disabled-path registry/hot-table lifecycle guarding.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs`
  for fail-closed indexed scheduler resource guards.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/perfdeep02_frame_roundtrip.rs`
  for explicit diagnostic fail-closed behavior when registry authority is
  unavailable.
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_scheduler_pl_activation.rs`
  for test context construction after runner lifecycle guard changes.

Any additional production write set requires amending this package before the
edit.

## Phase Plan

1. Populate required-reading and owned-file artifacts. Confirm the package is
   executing the PERFDEEP06 handoff and not a new seam-shaving package.
2. Audit the default-disabled path. Identify every dense-first, indexed,
   direct-frame, hot-symbol, shadow, or compatibility object that is constructed
   or resolved when all PERFDEEP opt-ins are disabled.
3. Patch the default-disabled path split. Keep previous opt-ins fail-closed and
   preserve their behavior when explicitly enabled.
4. Run the P0 disabled-path gate. Required: H2637 no-UI identity, at least three
   clean default-disabled endpoint runs with all PERFDEEP opt-ins off,
   min/median/max/RSS, same-machine control where feasible, median
   `<= 676.67 s`, and static proof that dense/indexed/direct-frame compatibility
   plumbing is not constructed on the disabled path. If this gate fails, stop
   with `HOLD` or `NO-GO` before adding direct-frame hydrology.
5. Design the bounded direct-frame hydrology chain from PERFDEEP06 artifacts.
   Declare the migrated phase span, frame fields, context/forcing/view APIs,
   seed boundary, shadow oracle, and publication projection edge.
6. Implement the direct-frame opt-in chain. The migrated normal success path
   must not construct or read `HillslopeKernelRequest`,
   `KernelWritebackPayload`, `HillslopeWritebackSurface`, `BoundarySymbol`,
   `BoundaryValue`, `SymbolRegistry::id_of`, logical fallback reads, or
   dense/logical refresh/flush.
7. Add focused identity tests and H2637 validation. Compare migrated scalar
   outputs by `f64::to_bits()`, fixed arrays element-by-element, and final
   HBP/WAT/PASS outputs by existing byte/Arrow equivalence gates.
8. Record layout/type-size/allocation evidence and line-count governance.
   Because `scheduler.rs` is currently above 3000 lines, either keep edits under
   an explicit package exception with a sunset plan or split touched scheduler
   code before closure.
9. Run full closure gates: `cargo fmt --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, package-specific H2637 gates,
   docs lint, dual review, finding disposition, dual verification, roadmap and
   catalog updates.
10. Close truthfully as `READY-FOR-PERFDEEP08`, `HOLD`, or `NO-GO`.

## Acceptance Criteria

- Default-disabled H2637 identity passes for protected output surfaces.
- Default-disabled H2637 endpoint gate passes: at least three clean no-UI runs
  with all PERFDEEP opt-ins disabled, min/median/max/RSS recorded, same-machine
  control where feasible, median `<= 676.67 s`, and no hard-attributed external
  environment exception required.
- Static disabled-path proof shows no dense-first/request-view/direct-frame
  shadow work runs when all PERFDEEP opt-ins are disabled.
- Direct-frame hydrology opt-in remains default-disabled and fail-closed.
- The migrated direct-frame normal success path has zero direct use of the
  map/symbol/writeback mechanisms named in the PERFDEEP06 no-hot-loop-map
  checklist.
- Focused fixtures prove `f64::to_bits()` identity for migrated frame outputs
  and element identity for fixed arrays.
- H2637 opt-in shadow run preserves HBP/WAT byte identity and PASS Arrow
  equivalence against the default logical path.
- H2637 opt-in endpoint/RSS is measured against `669.97 s`, the repaired
  default-disabled control, and PERFDEEP05 opt-in `911.11 s`.
- Layout/type-size and allocation evidence is recorded for new frame/view
  structures and any reusable buffers.
- Full Rust closure gates pass.
- Markdown lint passes for the package and touched docs.
- Review findings are dispositioned as `accepted`, `rejected`, `deferred`, or
  `follow-up`; accepted findings are fixed and verified.
- Gate Evidence Non-Deferral is checked explicitly by both reviews and both
  verifications.

## Conservation / Output Acceptance

PERFDEEP07 may touch publication projection only to shadow the direct-frame
path. It must preserve current output schemas and use the PERFDEEP06 operand
ledger. Any future production output cutover requires HBP byte identity, WAT
byte/Arrow identity, PASS Arrow identity, metadata/provenance equivalence, and
anti-alias fixtures for every ledgered operand and metadata field.

## Contract-First Rule

No physics or canonical `SC-*` contract change is intended. If execution
discovers that frame APIs require changing invariant authority, guard semantics,
diagnostic attribution policy, output meaning, or process physics, stop and
amend the package before implementation. Any such amendment must follow the
contract-first sequence: canonical contract, contract-derived tests,
pre-implementation contract gate, then production edits.

## Security Impact Gate

No secrets, credentials, external network dependencies, or user data are in
scope. Do not weaken fail-closed behavior, typed error handling, validation
gates, output schema contracts, or serialization safeguards.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only reviewer and verifier subagents for package artifact review,
no-hot-loop-map proof review, disabled-path regression-gate review,
publication-shadow review, line-count governance review, and gate-legitimacy
verification. It also explicitly authorizes spawning/delegating to comparator
or batch-runner subagents for H2637 endpoint/identity runs if the local tooling
supports it. Expected outputs are compact findings or metrics recorded in the
package artifacts. Write access is limited to artifact files unless this package
is explicitly amended.

## Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/perfdeep07-disabled-path-audit.md`
- `artifacts/perfdeep07-disabled-path-baseline.md`
- `artifacts/perfdeep07-zero-cost-disabled-proof.md`
- `artifacts/perfdeep07-direct-frame-implementation-plan.md`
- `artifacts/perfdeep07-hydrology-frame-identity-ledger.md`
- `artifacts/perfdeep07-publication-shadow-ledger.md`
- `artifacts/perfdeep07-layout-allocation-evidence.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`

## Autonomy

Execute end-to-end when triggered. Do not proceed past the disabled-path P0 gate
if it fails. Do not ask the user for next steps unless a hard blocker prevents a
truthful `READY-FOR-PERFDEEP08`, `HOLD`, or `NO-GO` disposition. Do not activate
any runtime path by default.
