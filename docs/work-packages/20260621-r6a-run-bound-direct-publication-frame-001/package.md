# R6A - Run-Bound Direct Publication Frame

Status: executed.

Package type: implementation work package / R6 hold-lift.

## Objective

Close `HOLD-R6-DIRECT-PUBLICATION-FRAME-ABSENT` by building a run-bound direct
publication frame populated from typed direct run/lane/day state, then proving
real downstream projection consumers can read that frame without compatibility
WB13 rows, runtime symbols, writeback payloads, stale logical state, or wrappers
around those structures.

R6A does not claim full public output cutover by itself. It creates the missing
publication frame and explicit direct projection consumers needed before R6 can
cut HBP/WAT/PASS/loss/manifest production writers over.

## Failure Being Corrected

R5E proved direct-runtime execution/counters and shadow projection but did not
prove that runner-owned publication outputs consume typed direct state. Resumed
R6 promoted the publication operand ledger, then found public outputs still
build from compatibility WB13 rows/runtime surfaces.

The old closure was too weak. This package is governed by the
`Consumer-Path Closure Rule` in `docs/work-packages/AGENTS.md`: producer-only,
skeleton-only, counter-only, or shadow-only evidence cannot close a direct,
endpoint, publication, or cutover claim.

## Scope

In scope:

- expand or replace the narrow direct-runtime `DirectPublicationFrame` with a
  run-bound frame that covers the promoted R6 publication ledger;
- construct that frame from typed direct run/lane/day state, not compatibility
  rows or runtime surfaces;
- expose explicit direct projection consumers for HBP, WAT, PASS, loss JSON,
  and run-manifest candidate payloads;
- add consumer-path proof artifacts before Rust edits and update them after
  implementation;
- add anti-alias fixtures that fail if the frame is populated from plausible
  compatibility aliases or adjacent direct diagnostics;
- add independent reconstruction fixtures that do not call the production
  direct projection builder under test;
- prove direct projection source code and runtime counters avoid compatibility
  publication reads;
- preserve default-disabled compatibility behavior and current public output
  identity.

Out of scope unless this package is amended before implementation:

- making direct publication the default;
- deleting compatibility publication adapters;
- changing output schemas, column names, units, metadata keys, manifest schema,
  or HBP binary format;
- changing process physics, conservation equations, or phase order;
- claiming R6 public cutover complete.

## Authority

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/publication-ledger-authority-promotion.md`
- `docs/work-packages/20260621-r5e-full-ofe-day-endpoint-readiness-001/artifacts/disposition.md`

## Intended Write Set

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` if frame schema
  authority needs amendment;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/**`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/tests03/**`
- `crates/openwepp-hillslope-output/src/**` only for direct projection helper
  tests that preserve existing schemas;
- `tests/integration/**` for package/consumer-path guards.

Files outside this set require package amendment before edits.

## Phase Plan

1. Record pre-implementation consumer-path evidence: current producers, current
   frame, runner handoff, output consumers, and old compatibility reads.
2. Define the run-bound direct publication frame schema against the promoted R6
   ledger. Preserve explicit `Option`/not-authoritative fields where a producer
   is not yet authoritative.
3. Add a typed constructor that takes direct run/lane/day state and rejects
   missing or invalid required operands with typed errors.
4. Add direct projection consumers for HBP, WAT, PASS, loss JSON, and manifest
   candidate payloads. These consumers must accept the direct publication frame,
   not compatibility WB13 rows or runtime surfaces.
5. Add anti-alias and independent reconstruction fixtures before accepting each
   projection consumer.
6. Wire an explicit runner shadow/opt-in path that constructs the run-bound
   frame for real parsed run dimensions. `DirectSkeletonNoop` and
   `DirectSkeletonShadowOnly` are not valid R6A acceptance evidence.
7. Run focused tests and source scans until the consumer-path proof is clean.
8. Run package-required Rust gates, docs lint, whitespace checks, reviews,
   verification, line-count governance, disposition, and worker handoff.

## Acceptance Criteria

- Root and work-package agent guidance include the Consumer-Path Closure Rule.
- Package artifacts include a current producer/frame/runner/consumer proof.
- `DirectSkeletonNoop`, `DirectSkeletonShadowOnly`, and counter-only evidence
  are labeled scaffolding only and are not used as acceptance evidence.
- A run-bound direct publication frame exists and is populated from typed direct
  state for real run/lane/day dimensions.
- The frame covers every promoted R6 ledger family: HBP, WAT, PASS, loss JSON,
  manifest, row identity, calendar identity, and metadata/provenance operands.
- Missing direct operands fail closed; they are not silently defaulted except
  where the promoted ledger explicitly marks an output field as currently absent.
- Direct projection consumers for all five output families read the frame.
- Negative source scans prove those direct consumers do not read
  `SimulationOwnedWb13Row`, `HillslopeWritebackSurface`, `BoundarySymbol`,
  `BoundaryValue`, `KernelWritebackPayload`, `SymbolRegistry`, hot tables,
  stale logical state, or diagnostic compatibility ledgers.
- Runtime counters or focused tests prove default-disabled compatibility builds
  no direct publication frame.
- Anti-alias fixtures distinguish accepted direct operands from compatibility
  WB13 aliases, adjacent direct diagnostics, stale logical state, area/volume
  denominator aliases, and metadata shortcuts.
- Independent reconstruction agrees with direct projection outputs without
  calling the production projection builder under test.
- Current public compatibility outputs remain protected-output identical until
  the later R6 output cutover package explicitly changes production writers.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass before completion.
- Scoped markdown lint and `git diff --check` pass.
- Dual review and dual verification explicitly check Consumer-Path Closure Rule,
  Gate Evidence Non-Deferral, anti-alias adequacy, independent reconstruction,
  no-compatibility proof, and line-count governance.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only consumer-path reviewer, ledger-coverage
reviewer, no-compatibility source-scan reviewer, benchmark runner, and verifier
subagents for the scopes declared above. Expected outputs are compact findings,
command logs, and metrics recorded in package artifacts. Write access is limited
to package artifacts unless this package is explicitly amended.

## Deliverables

- `artifacts/README.md`
- `artifacts/scope-selection.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/internal-scaffolding-evidence.md`
- `artifacts/data-path-proof.md`
- `artifacts/publication-frame-schema.md`
- `artifacts/operand-lineage.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/anti-alias-fixture-plan.md`
- `artifacts/independent-reconstruction-plan.md`
- `artifacts/no-compatibility-proof-checklist.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`
- `prompts/active/20260621-r6a-run-bound-direct-publication-frame-001_kickoff_agent_prompt.md`
- `prompts/archived/README.md`
