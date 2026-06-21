# R6C - Direct Publication Typed Operand Bridge

Status: executed-hold.
Final disposition: `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`.

Package type: implementation work package / R6 hold-lift.

## Objective

Resolve `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER` by lifting
the R6B blocker
`HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`.

R6C must install the production typed operand bridge that makes
`DirectRunPublicationFrame` consume accepted direct run/lane/day operands before
publication projection, then close the HBP, WAT, PASS, loss, and manifest
cutover gates. If production direct producers are still absent for a required
publication family, R6C must stop in a named `HOLD` with concrete source
evidence and a follow-on write set; it must not wrap compatibility WB13 rows,
runtime surfaces, writeback payloads, or stale logical state in direct-named
structures.

## Execution Result

R6C did not close `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER`.
It stopped at
`HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`.

The package found that the production climate lifecycle returns compatibility
publication products (`WB13`, PASS rows, runtime surface, provenance), but does
not retain a direct day/publication frame or accepted direct publication
operands from production execution. The R6B candidate then reran a fresh direct
skeleton to synthesize publication rows. That cannot be accepted under the R6
ledger because it is neither production direct authority nor parity-grade input
state.

R6C corrected the cutover behavior by making
`DirectPublicationFrameCutover` fail before any skeleton direct frame,
executor, or publication capture is constructed. The error now carries the
specific hold marker
`HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT` and preserves fail-closed
public writes. The next package must wire retained production direct
run/lane/day publication producers into the climate lifecycle; wrapping WB13
rows, runtime surfaces, writeback payloads, or stale logical state remains
forbidden.

## Failure Being Corrected

R6B proved the current `DirectPublicationFrameCutover` candidate still creates
a skeleton direct frame, seeds lane geometry and calendar metadata, and captures
zero/default direct rows. The fail-closed diagnostic marker is
`R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT`.

The remaining R6 hold is not a manifest-only problem. HBP/WAT/PASS/loss and the
manifest must all read from accepted direct publication operands, with parity,
anti-alias, independent reconstruction, and no-compatibility evidence before
public writes are enabled.

## Scope

In scope:

- build the production bridge from parsed inputs and accepted direct
  run/lane/day operands into the direct publication frame;
- add or expose required direct publication producers without changing
  physics, schemas, units, or scheduler activation semantics;
- fail closed when a required direct publication operand is missing or invalid;
- populate HBP, WAT, PASS, loss, and manifest projection operands from direct
  authority;
- cut the opt-in production writer path over only after all output-family gates
  pass;
- add anti-alias fixtures for HBP `peakro`, `watdur`, sediment fields, WAT
  water-balance fields, PASS volumes, loss static fields, and manifest
  provenance/checksums;
- add independent reconstruction for accepted HBP/WAT/PASS/loss operands;
- prove accepted direct publication consumers do not read compatibility WB13
  rows, runtime symbols, writeback payloads, stale logical state, or wrappers
  around those structures;
- preserve default-disabled compatibility behavior and protected-output
  identity;
- run default-disabled H2637 timing and opt-in endpoint/RSS evidence after
  parity gates pass.

Out of scope unless this package is amended before implementation:

- default activation of direct publication;
- deleting compatibility writers or hot compatibility runtime;
- schema, column, unit, HBP binary format, or manifest schema changes;
- process-physics changes or heuristic/proxy math;
- accepting skeleton/zero direct rows, compatibility wrappers, or
  self-consistency-only checks as cutover evidence.

## Authority

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/package.md`
- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/package.md`
- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/artifacts/worker-handoff.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

Conditional:

- Relevant `SC-*` contracts before changing output meaning, units, aliases,
  physics lineage, guard semantics, checksum authority, or conservation
  authority.

## Dependencies

- R5E complete at pushed commit `d8f6bbea`.
- R6A complete at pushed commit `9ce6af17`.
- R6 executed-held at pushed commit `cb10be17`.
- R6B executed-held at pushed commit `c1b77cfe`.
- Canonical R6 ledger exists in
  `docs/architecture/array-native-runtime-specification.md` section
  `5.2.1 R6 Canonical Publication Operand Ledger`.

## Intended Write Set

- `docs/work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/**`
- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/artifacts/worker-handoff.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` if ledger/frame
  authority needs amendment before implementation.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` if
  manifest/provenance authority needs contract amendment.
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` if
  water-balance publication/reconstruction authority needs amendment.
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` if
  HBP/PASS sediment publication authority needs amendment.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/**`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-output/src/**` only for preserved-schema direct
  writer/projection tests.
- `tests/integration/**` for package guards, parity, no-compatibility, and
  benchmark evidence.
- `tools/**` only for comparison/benchmark wrappers needed to record evidence.

Files outside this set require package amendment before edits.

## Phase Plan

1. Record required reading, scope selection, owned files, and pre-change
   data-path proof.
2. Rebuild the R6 output-family operand lineage from the canonical ledger and
   identify the exact producer for each accepted direct operand.
3. Run the contract gate. Amend architecture or `SC-*` authority before any
   meaning/unit/provenance/checksum change; otherwise record no amendment.
4. Implement the typed bridge so publication capture starts from accepted
   direct run/lane/day operands, not skeleton defaults.
5. Add fail-closed missing-operand diagnostics for each required publication
   family.
6. Wire cutover-mode HBP, WAT, PASS, loss, and manifest projection through the
   direct publication frame only.
7. Add anti-alias fixtures before accepting each family.
8. Add independent reconstruction for accepted HBP/WAT/PASS/loss operands.
9. Add no-compatibility source scans and runtime/focused tests proving accepted
   direct consumers avoid forbidden compatibility surfaces.
10. Iterate the `DirectPublicationFrameCutover` candidate until HBP byte
    identity, WAT parity, PASS parity, loss parity, and manifest parity pass.
11. Run default-disabled timing/protected-output identity and direct endpoint
    RSS evidence.
12. Run focused tests, package guards, full Rust gates, scoped docs lint, and
    whitespace checks.
13. Complete review, verification, line-count governance, disposition, and
    worker handoff.

## Acceptance Criteria

- `HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT` is lifted or R6C
  stops in a more precise named hold with concrete source evidence.
- `DirectRunPublicationFrame` is populated from accepted direct operands, not
  skeleton state, compatibility WB13 rows, compatibility runtime surfaces,
  writeback payloads, stale logical state, or wrappers around those structures.
- HBP direct publication passes byte identity.
- WAT direct publication passes row/schema/metadata parity.
- PASS direct publication passes row/schema/metadata/volume-basis parity.
- Loss JSON direct publication passes normalized JSON identity with documented
  ordering policy.
- Manifest production writes in cutover mode read typed direct publication
  projection, including provenance, checksums, direct runtime counters, warning
  metadata, and output policy.
- Anti-alias fixtures fail on every rejected alias class listed in scope.
- Independent reconstruction agrees with accepted HBP/WAT/PASS/loss operands
  and does not call the production projection builder under test.
- No-compatibility source scans and focused tests pass.
- Default-disabled H2637 median remains `<= 676.67 s` with protected-output
  identity/equivalence.
- Direct-publication endpoint/RSS evidence is recorded and dispositioned.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass before completion.
- Scoped markdown lint and `git diff --check` pass.
- Review and verification explicitly check Gate Evidence Non-Deferral,
  Consumer-Path Closure, anti-alias adequacy, independent reconstruction,
  no-compatibility proof, benchmark evidence, and line-count governance.

## Security Impact Gate

No secrets, credentials, production host actions, or external network
dependencies are in scope. R6C must preserve typed fail-closed validation,
serialization safeguards, and manifest checksum integrity.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only operand-lineage, anti-alias, independent
reconstruction, no-compatibility source-scan, benchmark runner, reviewer, and
verifier subagents for the scopes declared above. Expected outputs are compact
findings, command logs, metrics, and review/verification findings recorded in
package artifacts. Write access is limited to package artifacts unless this
package is explicitly amended.

## Deliverables

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/scope-selection.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/data-path-proof.md`
- `artifacts/operand-lineage.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/typed-bridge-design.md`
- `artifacts/anti-alias-fixture-plan.md`
- `artifacts/independent-reconstruction-plan.md`
- `artifacts/manifest-cutover-proof.md`
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
- `prompts/active/20260621-r6c-direct-publication-typed-operand-bridge-001_kickoff_agent_prompt.md`
- `prompts/archived/README.md`
