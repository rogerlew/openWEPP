# R6E - Direct Publication Cutover Iterative Defect Closure

Status: executed-held at
`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

Package type: Defect-Closure ExecPlan / iterative R6 publication cutover
blocker closure.

Defect ID: `R6E-DIRECT-PUBLICATION-CUTOVER-BLOCKER`.

## Objective

Close the R6 direct-publication blocker left by R6D. The package must
iteratively identify, document, remediate, and validate every in-envelope
blocker that prevents `DirectPublicationFrameCutover` from becoming the
production direct-publication path.

Closure requires real direct publication cutover: HBP, WAT, PASS, loss, and
run manifest publication must read typed direct projection only, must pass
byte/Arrow/schema/metadata parity, must have anti-alias fixtures and
independent operand reconstruction, and must not source authority from
compatibility WB13 rows, compatibility runtime publication surfaces, writeback
payloads, stale logical state, skeleton frames, or wrappers around those
structures.

This package must not close as complete after one more diagnostic step. If an
in-envelope root cause is found and correction authority exists, the package
must implement the correction, validate it, and continue the loop until direct
publication cutover closes or a legitimate DC `HOLD` boundary is proven.

## Current Failure

R6D lifted `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT` by retaining a
cutover-only `DirectRunPublicationFrame` in the production climate-day loop.
R6D then held at
`HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT` because the retained frame
contains parsed climate/calendar/geometry rows but lacks parity-grade direct
hydrology, storage, subsurface, evaporation, PASS, loss, manifest, and erosion
publication producers.

R6E reproduced that fail-closed state, split the direct-publication helper block
out of the over-3000-line runner file, and first reduced the held marker to
`HOLD-R6E-PRODUCTION-DIRECT-RUNTIME-INPUT-BINDING-ABSENT`. R6E then resolved
that blocker by adding typed direct publication day inputs, binding parsed
climate precipitation and effective temperature into direct day frames, and
constructing the retained cutover execution through
`DirectFrameExecutor::run_publication_capture_with_day_inputs` instead of
hand-authored rows.

The cutover candidate now runs direct phase spans and reaches HBP byte
comparison without using compatibility WB13/runtime/writeback authority as
direct publication input. It still fails closed before public output writes at
`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`: the direct process operands
produced by the current R4/R5 direct spans do not yet byte-match the
compatibility HBP publication. The package therefore preserves fail-closed
no-output behavior and does not claim R6 direct publication cutover.

## Correction Authority Envelope

Observed violation:

- `R6E-DIRECT-PUBLICATION-CUTOVER-BLOCKER`: on the opt-in
  `DirectPublicationFrameCutover` path, public outputs are still fail-closed
  because the direct publication frame now reaches HBP comparison but its
  process operands do not pass byte identity. Current blocker marker:
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

In-scope defect mechanisms:

- missing retained direct publication producers for hydrology, storage,
  subsurface, evaporation, transfer, profile, interception, snow/frost,
  PASS-volume, loss, manifest, and erosion/sediment families;
- direct projection consumers still depending on compatibility WB13 rows,
  runtime surfaces, writeback payloads, stale logical state, skeleton
  publication capture, or wrappers around those surfaces;
- missing output-family anti-alias fixtures;
- missing independent operand reconstruction for accepted direct publication
  families;
- direct manifest provenance/checksum parity blockers;
- HBP byte identity, WAT/PASS Arrow row/schema/metadata parity, loss JSON
  identity, and run-manifest parity blockers;
- monolithic runner direct-publication helper ownership that blocks closure
  under line-count governance.

In-scope write set:

- `docs/work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only if R6 cutover
  authority or acceptance wording must be clarified before implementation
- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/artifacts/worker-handoff.md`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` only for cutover flag,
  manifest, or validation plumbing
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/storage/**`
- `crates/openwepp-hillslope-orchestrator/src/subsurface/**`
- `crates/openwepp-hillslope-orchestrator/src/growth/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-hillslope-output/**`
- `crates/openwepp-summary-accumulator/**` only if PASS/WAT/loss comparison
  helpers need direct reconstruction support
- `tests/integration/**` for direct-publication cutover, anti-alias,
  reconstruction, and parity gates
- `tools/owcmp/**` or `tools/release/**` only for existing comparison or
  release-sidecar harness reuse; do not change authority-suite posture without
  anti-evasion guards.

Allowed production edit classes:

- add or promote retained typed direct publication operands from existing
  direct runtime state, direct phase projections, parsed inputs, and canonical
  runner execution state;
- make direct HBP/WAT/PASS/loss/manifest consumers read only typed direct
  projection surfaces on the cutover path;
- add fail-closed guards for missing or non-authoritative direct publication
  operands;
- add anti-alias fixtures and independent reconstruction tests for each
  accepted output family;
- split `00_runner_intake_and_lane_setup.rs` direct-publication helpers into
  narrower modules when required by line-count governance;
- amend canonical architecture/spec text only when it clarifies R6 cutover
  authority already implied by the promoted publication operand ledger.

Protected boundaries:

- no default activation of direct publication unless a separate explicit
  activation package authorizes it;
- no process-physics formula changes unless the package is amended
  contract-first and the correction is required by canonical `SC-*` authority;
- no output schema/unit/metadata meaning changes except where parity evidence
  and canonical authority explicitly require them;
- no compatibility WB13/runtime/writeback/stale logical source used as direct
  authority;
- no skeleton/direct self-consistency evidence accepted as cutover closure;
- no silent fallback wrappers for missing direct producers.

Authority:

- `docs/architecture/array-native-runtime-specification.md`, especially the R6
  publication operand ledger and R6 acceptance row.
- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/`
  for the current retained producer state and hold marker.
- R6/R6A/R6B/R6C package artifacts for previous blockers and rejected shortcuts.
- Canonical `SC-*` contracts for any process-family publication operand whose
  direct producer authority is ambiguous.

Conversion rule:

If this package establishes a reproducible root cause inside the declared
envelope, and the expected behavior is supported by canonical architecture,
canonical `SC-*` contracts, pinned-baseline provenance, or a
contract-authorized physical/publication invariant, it must proceed through
contract confirmation or amendment, contract-derived tests, pre-implementation
evidence, production correction, validation, review, and disposition in this
package. It may not close as `HOLD` because another blocker might remain, more
inspection is possible, or the implementation is complex.

Seven-gate bar:

1. Reproduction: reproduce the current R6D cutover failure and record command,
   marker, no-output result, direct runtime counters, and retained-frame state.
2. Mechanism: reduce each blocker to a named output family, direct producer, or
   consumer-path mechanism.
3. Ownership: prove the mechanism is in this envelope or name the exact
   legitimate boundary.
4. Authority: confirm the expected direct operand source from the architecture
   ledger, canonical `SC-*`, pinned baseline provenance, or an explicit
   physical/publication invariant.
5. Safety: do not loosen typed guards, invent physics, silently default missing
   state, or change output meaning without authority.
6. Testability: add or reuse a fixture that fails before the correction and
   distinguishes plausible wrong aliases.
7. Validation: prove direct cutover with parity, independent reconstruction,
   no-compatibility scans, output writes, manifest checksums, and full closure
   gates.

## Iterative Defect-Closure Loop

This package is intentionally iterative. After reproducing the current R6D
failure, repeat the following loop until direct publication cutover closes or a
legitimate DC boundary is proven:

1. Identify the next blocking output family or consumer-path read.
2. Record the blocker in `artifacts/r6e-blocker-ledger.md` with marker,
   evidence, owner, authority, candidate correction, and acceptance gate.
3. If the blocker is in-envelope and authority-supported, implement the
   correction in this package.
4. Add or update anti-alias and independent reconstruction evidence before
   accepting the output-family correction.
5. Run focused validation for the corrected family plus no-compatibility scans.
6. Re-run `DirectPublicationFrameCutover` and record the next blocker or the
   full cutover result.
7. Retain corrections that move the package toward direct cutover; revert or
   document rejected candidates that introduce compatibility authority, parity
   regressions, or output-write hazards.

The loop may not close with "next package should inspect X" or "next package
should implement the correction" for any in-envelope, authority-supported
blocker.

## Scope

In scope:

- reproduce and document the current R6D cutover failure;
- build the blocker ledger from actual cutover execution, source scans, and
  parity/reconstruction evidence;
- implement retained direct producers and consumer-path cutover for every
  required output family inside the envelope;
- add anti-alias fixtures and independent reconstruction checks for accepted
  direct publication operands;
- prove HBP byte identity, WAT/PASS Arrow row/schema/metadata parity, loss JSON
  identity, manifest provenance/checksum parity, no forbidden compatibility
  authority, and output-write success for cutover;
- preserve compatibility-mode output identity and default-disabled isolation;
- address line-count governance for touched 3000+ `.rs` files before complete
  disposition;
- update package artifacts, roadmap/catalog state, reviews, verifications,
  disposition, and handoff.

Out of scope:

- default activation of direct publication;
- unrelated performance work;
- unrelated process-physics magnitude corrections;
- broad style refactors not required by R6 cutover or line-count governance;
- closing by diagnostic handoff.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/defect_closure_execplans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/package.md`
- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/package.md`
- `docs/work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/package.md`
- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/package.md`
- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/artifacts/gate-results.md`
- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/artifacts/no-compatibility-proof-checklist.md`
- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/artifacts/operand-lineage.md`

Required before Rust edits:

- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `tests/AGENTS.md` before editing root integration tests
- `tools/owcmp/AGENTS.md` before editing or relying on owcmp tooling

On-demand source inventory:

- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/storage/**`
- `crates/openwepp-hillslope-orchestrator/src/subsurface/**`
- `crates/openwepp-hillslope-output/**`

## Dependencies

- R6D is executed-held at
  `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`.
- The R6 publication operand ledger is canonical architecture authority.
- Compatibility mode and default-disabled direct runtime isolation must remain
  intact.

## Phase Plan

1. Populate required-reading, owned-file, correction-envelope, and baseline
   artifacts.
2. Reproduce the current cutover failure and record the marker, no-output
   result, retained-frame state, direct runtime counters, and output-write
   state.
3. Build the initial blocker ledger from R6D evidence, source scans, and direct
   cutover execution.
4. Enter the remediation loop. For each blocker, record mechanism, owner,
   authority, anti-alias/reconstruction requirement, patch, focused tests,
   parity result, retain/revert decision, and next blocker.
5. Continue until HBP/WAT/PASS/loss/manifest all read typed direct projection
   only on `DirectPublicationFrameCutover`.
6. Run full parity and output acceptance gates:
   - HBP byte identity;
   - WAT Arrow row/schema/metadata parity;
   - PASS Arrow row/schema/metadata parity;
   - loss JSON identity;
   - manifest provenance/checksum parity;
   - anti-alias fixtures;
   - independent operand reconstruction;
   - no forbidden compatibility-source scans;
   - cutover output-write success.
7. Verify default-disabled/compatibility isolation and protected compatibility
   output identity.
8. Run final closure gates:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, scoped docs lint, and
   `git diff --check`.
9. Complete line-count governance, dual review, finding disposition, dual
   verification, roadmap/catalog updates, and worker handoff.
10. Close only as `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`, `NO-GO`, or a
    legitimate DC `HOLD` at a declared boundary.

## Legitimate HOLD Boundaries

This package may close `HOLD` only when one of these boundaries is proven:

- a required output-family mechanism lies outside the declared envelope and
  cannot be safely amended without crossing process-family authority;
- the governing `SC-*` or architecture authority is missing or contradictory;
- required parity or reconstruction evidence cannot be generated in the
  available environment;
- line-count governance requires a prerequisite split too large to combine
  with the R6 cutover correction without losing reviewability;
- an upstream input is invalid and the correct behavior is the current typed
  fail-closed error.

Forbidden `HOLD` states:

- "inspect the next output family";
- "implement the known in-envelope producer later";
- "parity is close enough except one family";
- "consumer still reads compatibility but cutover is otherwise complete";
- "anti-alias/reconstruction evidence is deferred";
- "line-count is deferred while claiming complete cutover".

## Acceptance Criteria

- Current R6D failure is reproduced and recorded.
- Blocker ledger names every discovered R6 cutover blocker and its disposition.
- `DirectPublicationFrameCutover` succeeds and writes HBP, WAT, PASS, loss, and
  run manifest outputs from typed direct projection only.
- HBP byte identity passes against compatibility output.
- WAT and PASS Arrow row/schema/metadata parity pass.
- loss JSON identity passes.
- run manifest provenance/checksum parity passes, including direct-mode
  runtime-selection/counter/output-policy fields.
- Anti-alias fixtures distinguish direct operands from plausible wrong aliases
  for every accepted output family.
- Independent reconstruction passes for direct runoff, PASS volumes, lateral
  volumes, storage, snow/frost, loss, and manifest checksum/provenance
  families.
- No compatibility WB13 rows, runtime publication surfaces, writeback payloads,
  stale logical state, skeleton frame capture, or wrappers around those sources
  are used as direct publication authority.
- Compatibility/default-disabled mode constructs no retained direct publication
  surface and preserves protected output identity.
- Touched 3000+ `.rs` files are split or have an approved closure disposition;
  complete cutover cannot be claimed with an undispositioned 3000+ file.
- Full Rust closure gates pass.
- Markdown lint and `git diff --check` pass.
- Dual reviews and dual verifications explicitly check Gate Evidence
  Non-Deferral, Consumer-Path Closure, Conservation/Publication Acceptance,
  DC `HOLD` legitimacy, envelope adequacy, and protected-boundary integrity.

## Conservation / Publication Acceptance

R6E is conservation/output-publication work. Before accepting any output-family
correction, record:

- operand name, units, normalization/denominator, area or volume basis, source
  authority, and whether it is authoritative or diagnostic;
- rejected aliases/formulas that the fixture distinguishes;
- independent reconstruction method and tolerance;
- metadata/schema alignment evidence;
- protected output parity evidence.

Self-consistency and one-sided bounds are not enough to close any output-family
gate.

## Contract-First Rule

No `SC-*` change is intended at scaffold time. If a direct producer requires
new or clarified process/publication authority, amend or confirm canonical
authority before production edits and add contract-derived tests before
claiming the correction.

## Security Impact Gate

No secrets, credentials, production host actions, external network dependencies,
or user data are in scope. Preserve typed fail-closed validation, serialization
safeguards, output checksum integrity, and manifest schema contracts.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only blocker-inventory, parity runner, anti-alias/reconstruction
reviewer, no-compatibility source-scan, line-count-governance reviewer, and
verification subagents for R6 cutover execution, output parity, independent
reconstruction, package artifact review, line-count governance, and
gate-legitimacy verification. Expected outputs are compact findings, command
logs, metrics, and artifact updates. Write access is limited to package
artifacts unless this package is explicitly amended.

## Deliverables

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/r6e-correction-authority-envelope.md`
- `artifacts/r6e-current-failure-reproduction.md`
- `artifacts/r6e-blocker-ledger.md`
- `artifacts/r6e-remediation-iteration-log.md`
- `artifacts/r6e-candidate-ledger.md`
- `artifacts/r6e-operand-lineage.md`
- `artifacts/r6e-anti-alias-fixtures.md`
- `artifacts/r6e-independent-reconstruction.md`
- `artifacts/r6e-consumer-path-proof.md`
- `artifacts/r6e-no-compatibility-proof.md`
- `artifacts/r6e-parity-evidence.md`
- `artifacts/r6e-manifest-cutover-evidence.md`
- `artifacts/r6e-default-disabled-isolation.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`
- `prompts/active/r6e_kickoff_agent_prompt.md`
- `prompts/archived/README.md`

## Autonomy

Execute end-to-end when triggered. Do not stop after a single diagnostic run,
single output-family correction, or single new hold marker. Continue the
blocker/remediation loop until direct publication cutover closes or a
legitimate DC boundary is proven. Do not ask the user for next steps unless a
hard blocker prevents truthful disposition.
