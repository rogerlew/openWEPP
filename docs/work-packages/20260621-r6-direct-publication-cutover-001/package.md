# R6 - Direct Publication Cutover

Status: executed-hold.

Disposition: `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER`.

Package type: implementation work package / array-native runtime R6.

## Objective

Cut public hillslope publication over to typed direct projection only after
promoting the PERFDEEP06 publication operand ledger into canonical authority.

R6 must make HBP, WAT, PASS, loss, and run-manifest publication read from typed
direct projection structures rather than runtime symbol/logical/writeback
surfaces. The cutover is allowed only after the operand ledger is canonical,
reviewed, fixture-backed, and independently reconstructable.

## Resumed Execution Summary

R6 resumed after R5E closed at pushed commit `d8f6bbea`. The prerequisite gate
is cleared.

R6 completed the required first step by promoting the PERFDEEP06 publication
operand ledger into canonical architecture authority in
`docs/architecture/array-native-runtime-specification.md` section
`5.2.1 R6 Canonical Publication Operand Ledger`.

R6A then lifted the original direct-publication-frame blocker by adding
`DirectRunPublicationFrame` and direct HBP/WAT/PASS/loss/manifest projection
consumers.

This resumed R6 execution added an explicit
`DirectPublicationFrameCutover` opt-in and CLI flag
`--direct-publication-frame-cutover`. In that mode, the production output
boundary routes HBP, WAT, PASS, and loss candidate writes through the direct
publication artifacts only after a fail-closed parity gate compares the direct
projection against the accepted compatibility baseline. The candidate currently
stops before writing outputs with
`R6-DIRECT-PUBLICATION-PARITY HBP byte identity failed: direct=1654 bytes
compatibility=1654 bytes`.

The hold is therefore no longer "frame absent". It is now a parity and
manifest cutover hold: the current direct frame is still skeleton/zero seeded
for real run operands, so byte/Arrow identity cannot pass, and the production
manifest writer remains compatibility-provenance based.

## Rationale

R4 closed direct hydrology projection and R5 is closing the full OFE-day direct
executor while preserving the no-publication boundary. The remaining
compatibility publication edge is both a performance boundary and a correctness
risk: it can silently reintroduce symbol aliases, stale logical surfaces, or
metadata drift after direct execution has produced typed operands.

PERFDEEP06 produced the seed publication operand ledger at
`docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`.
R0/R1 then recorded that the ledger is planning evidence, not canonical
authority, and must be promoted before cutover. R6 exists to do that promotion
first, then perform the direct publication cutover under byte/Arrow identity,
metadata parity, anti-alias fixture, and independent operand-reconstruction
gates.

## Scope

In scope:

- promote the PERFDEEP06 publication operand ledger, or an equivalent amended
  ledger, into binding architecture or contract authority before production
  output code changes;
- preserve every output operand's units, basis, row/column destination,
  producer phase, source direct-frame field, legacy alias, wrong-alias rejection
  list, metadata/provenance obligations, and reconstruction requirement;
- make HBP reads use typed direct projection only;
- make WAT parquet reads use typed direct projection only;
- make PASS parquet reads use typed direct projection only;
- make loss JSON reads use typed direct projection only;
- make run-manifest publication/provenance reads use typed direct projection
  only;
- add anti-alias fixtures that distinguish accepted direct operands from legacy
  publication aliases, diagnostic ledgers, stale logical state, compatibility
  output frames, area/volume denominator aliases, and metadata shortcuts;
- add independent operand reconstruction for conservation-sensitive outputs;
- prove byte identity for byte-stable outputs and Arrow/schema/metadata identity
  for Arrow/parquet outputs;
- prove direct publication does not enter runtime symbol/logical/writeback
  surfaces after cutover;
- preserve default-disabled compatibility behavior and protected-output
  identity during migration.

Out of scope unless this package is amended before implementation:

- changing output schemas, column names, unit metadata, or manifest schema IDs;
- changing process physics, conservation equations, or phase order;
- default activation if R5E has not already authorized it;
- deleting compatibility publication adapters needed for replay, diagnostics,
  rollback, or shadow validation;
- watershed fan-in redesign beyond the direct hillslope publication surfaces
  needed for HBP/WAT/PASS/loss/manifest parity;
- broad R7 hot compatibility runtime deletion.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/r5-burndown-execplan.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/publication-ledger-promotion-plan.md`

Prerequisite package evidence:

- R5E package `package.md`, `gate-results.md`, `endpoint-rss-evidence.md`,
  `no-compatibility-proof-checklist.md`, and `disposition.md` after R5E closes.

Required before Rust edits:

- `crates/AGENTS.md`
- `tests/AGENTS.md` before test edits.
- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` before any `SC-*`
  contract amendment.

On-demand source inventory:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-hillslope-output/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-legacy-bridge/src/hbp.rs`
- `tests/integration/**`
- `tests/python/test_open_wepp_runner_api.py`

## Dependencies

- R5E must be complete, or this package must record a reviewed waiver explaining
  why R6 can proceed without full direct endpoint readiness. Without that, R6
  is scaffolded but not execution-ready.
- `docs/architecture/array-native-runtime-specification.md` is binding
  architecture authority.
- The PERFDEEP06 publication operand ledger is the seed authority, but not
  sufficient by itself until promoted or superseded by equivalent canonical
  authority.
- Existing output contracts and schema crates remain authoritative for output
  meaning, units, metadata, and serialization.

## Intended Write Set

- `docs/work-packages/20260621-r6-direct-publication-cutover-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` if ledger
  promotion requires publication-system authority.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` if ledger
  promotion or reconstruction gates touch water-balance publication authority.
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` if HBP/PASS
  sediment operands require contract authority.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-hillslope-output/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-legacy-bridge/src/hbp.rs`
- `tests/integration/**`
- `tests/python/test_open_wepp_runner_api.py`

Files outside this set require package amendment before edits.

## Phase Plan

1. Confirm R5E status. If R5E is incomplete and no waiver exists, stop in
   `HOLD` with this package remaining scaffolded.
2. Populate required-reading, owned-file, scope-selection, pre-implementation
   contract gate, and source-inventory artifacts.
3. Promote the PERFDEEP06 publication operand ledger into canonical authority
   before production publication edits. Promotion must be either an architecture
   specification amendment or a contract-backed output/publication authority.
4. Expand the promoted ledger into a field-level authority table for HBP, WAT,
   PASS, loss, and manifest publication, including metadata/provenance parity
   and reconstruction obligations.
5. Build anti-alias fixtures before each output-family cutover. Fixtures must
   separate direct operands from plausible wrong aliases named in the promoted
   ledger.
6. Build independent operand reconstruction harnesses before accepting
   conservation-sensitive output families. Reconstruction must not call the
   production direct projection builder being tested.
7. Cut over HBP to typed direct projection only and prove byte identity.
8. Cut over WAT to typed direct projection only and prove Arrow row/schema/
   metadata identity, with byte identity where stable.
9. Cut over PASS to typed direct projection only and prove Arrow row/schema/
   metadata identity, with byte identity where stable.
10. Cut over loss JSON and run manifest to typed direct projection only and
    prove byte-normalized JSON/metadata/provenance parity.
11. Prove no public publication reader in direct mode reads
    `HillslopeWritebackSurface`, `KernelWritebackPayload`,
    `BoundarySymbol`, `BoundaryValue`, `SymbolRegistry`, hot tables, indexed
    surfaces, dense refresh, dirty flush, or stale logical state.
12. Run focused tests, protected output comparisons, H2637 default-disabled and
    direct-publication endpoint/RSS gates, full Rust closure gates, scoped docs
    lint, and `git diff --check`.
13. Complete line-count governance, dual review, finding disposition, dual
    verification, roadmap/catalog updates, disposition, and worker handoff.

## Acceptance Criteria

- R5E completion or a reviewed waiver is recorded before implementation.
- The PERFDEEP06 publication operand ledger is promoted into canonical
  architecture or contract authority before production output edits.
- The promoted ledger covers every HBP, WAT, PASS, loss, and manifest field in
  scope with units, basis, source direct-frame field, producer phase, legacy
  alias, row/column destination, wrong aliases to reject, metadata/provenance
  obligations, and independent reconstruction requirement.
- HBP publication reads typed direct projection only and passes byte identity
  against the accepted compatibility baseline.
- WAT publication reads typed direct projection only and passes Arrow row,
  schema, field metadata, producer metadata, and calendar/identity parity.
- PASS publication reads typed direct projection only and passes Arrow row,
  schema, field metadata, producer metadata, and volume/area basis parity.
- Loss JSON publication reads typed direct projection only and passes
  byte-normalized JSON identity or a reviewed key-order-only normalization.
- Run manifest publication/provenance reads typed direct projection only and
  passes schema ID, checksum, execution-provenance, direct-runtime counter, and
  metadata parity.
- Anti-alias fixtures fail if any output family reads a wrong alias from
  compatibility publication state, stale logical state, diagnostic ledgers,
  adjacent hydrology operands, area/volume denominators, or metadata shortcuts.
- Independent operand reconstruction agrees with every conservation-sensitive
  publication family under declared tolerance and does not call the production
  projection function under test.
- Direct-publication source scans and runtime counters prove no compatibility
  symbol/logical/writeback publication reads remain in direct mode.
- Default-disabled H2637 median remains `<= 676.67 s` with protected-output
  identity.
- Direct-publication H2637 endpoint/RSS evidence is recorded against the latest
  accepted R5E baseline, with any regression dispositioned before closure.
- Full Rust closure gates pass:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check`.
- Scoped markdown lint and `git diff --check` pass.
- Review and verification artifacts explicitly check Gate Evidence
  Non-Deferral, ledger-promotion authority, anti-alias adequacy, independent
  reconstruction, metadata parity, no-compatibility proof, timing evidence, and
  line-count governance.

## Conservation / Publication Acceptance

This package is conservation/publication-sensitive. It must satisfy the
`docs/work-packages/AGENTS.md` Conservation / Publication Acceptance Rule as
current-scope acceptance, not a later follow-on.

Every output-family cutover must include:

- operand lineage with units, normalization/denominator, area or volume basis,
  source authority, and authoritative/diagnostic classification;
- anti-alias fixtures with expected values that differ from rejected aliases;
- independent reconstruction from inputs/direct state/output rows, not from the
  production direct projection implementation;
- metadata/schema parity checks that match the accepted operand lineage;
- closure or magnitude audit where the output is water, sediment, mass, routed
  runoff, or closure-ledger sensitive.

## Contract-First Rule

Publication authority promotion is the first production phase. If output
meaning, unit basis, schema metadata, guard semantics, conservation authority,
or process physics must change, amend canonical authority and contract-derived
tests before production code edits.

Comparator/output identity is a regression gate and a flag, not a correctness
target. Any residual must be adjudicated under the correctness authority model
before accepting a changed publication result.

## Security Impact Gate

No secrets, credentials, external network dependencies, user data, or
production host actions are in scope. R6 must preserve typed fail-closed output
domain validation and serialization safeguards. It must not add fallback
wrappers that silently mask missing direct projection operands.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only ledger-authority, anti-alias fixture,
benchmark runner, reviewer, and verifier subagents for publication-ledger
promotion review, output-family parity review, independent reconstruction
review, no-compatibility proof review, H2637 benchmark/comparison execution,
line-count-governance review, package artifact review, and gate-legitimacy
verification. Expected outputs are compact findings, command logs, metrics,
and review/verification findings recorded in package artifacts. Write access is
limited to package artifacts unless this package is explicitly amended.

## Deliverables

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/scope-selection.md`
- `artifacts/publication-ledger-authority-promotion.md`
- `artifacts/publication-cutover-plan.md`
- `artifacts/output-family-gate-matrix.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/no-compatibility-proof-checklist.md`
- `artifacts/identity-metadata-reconstruction-gates.md`
- `artifacts/default-disabled-regression-gate.md`
- `artifacts/endpoint-rss-evidence.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`
- `prompts/active/20260621-r6-direct-publication-cutover-001_kickoff_agent_prompt.md`
- `prompts/archived/README.md`

## Autonomy

When authorized for execution and after the R5E prerequisite is satisfied, run
this package end to end. Do not stop after ledger promotion without either
executing the cutover or recording a named `HOLD` blocker. Do not proceed to R7
compatibility-runtime deletion.

## Execution Result

R6 execution was attempted on 2026-06-21 and initially stopped at Phase 1 until
R5E completed. It later resumed, promoted the canonical publication ledger, and
held at `HOLD-R6-DIRECT-PUBLICATION-FRAME-ABSENT`. R6A lifted that blocker.

Current execution continued into Rust/output code and added a guarded cutover
candidate, but the package remains `executed-hold` because current-scope R6
acceptance gates do not pass. The direct frame is constructed and consumed at
the output boundary, but it is still populated from skeleton/zero direct state
instead of parity-grade typed direct run operands. HBP identity fails before
writing public outputs, WAT/PASS/loss parity remains unaccepted, and manifest
publication is still blocked by compatibility-provenance writer wiring.
