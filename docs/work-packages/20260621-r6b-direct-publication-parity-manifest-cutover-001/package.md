# R6B - Direct Publication Parity and Manifest Cutover

Status: executed-hold.
Final disposition: `HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`.

Package type: implementation work package / R6 hold-lift.

## Objective

Close `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER` by completing
the five first-actionable items from the R6 worker handoff:

1. Populate `DirectRunPublicationFrame` from parity-grade typed direct run
   operands instead of skeleton/zero direct state.
2. Add anti-alias fixtures that distinguish HBP `peakro`, `watdur`, sediment,
   WAT water-balance fields, PASS volumes, loss static fields, and manifest
   provenance from compatibility aliases.
3. Add independent reconstruction for accepted HBP/WAT/PASS/loss operands.
4. Replace the manifest production provenance/checksum path with typed direct
   publication projection in cutover mode.
5. Re-run the cutover candidate until HBP, WAT, PASS, loss, and manifest gates
   pass, then run default-disabled and endpoint/RSS benchmarks.

R6B is not a characterization-only package. It must either close the R6 direct
publication cutover gates or stop in a named `HOLD` with a concrete blocker and
defect-shaped follow-on package.

## Execution Result

R6B stopped at
`HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`.

The current production cutover candidate does not have a parity-grade typed
direct operand bridge. `build_direct_publication_artifacts` still constructs a
fresh `DirectRunFrame::skeleton`, seeds only lane geometry/calendar metadata,
and captures publication rows from zero/default direct day state. The direct
projection helpers exist, but the runner does not populate the direct frame
from production parsed inputs or from the compatibility execution's validated
typed operands.

R6B retained a fail-closed diagnostic improvement: when the cutover gate fails
while every publication operand is zero or absent, the error now includes
`R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT`. That makes the current blocker
machine-visible without accepting compatibility wrappers or silent defaults.

## Failure Being Corrected

R6 promoted the publication operand ledger into canonical architecture
authority and added `DirectPublicationFrameCutover`, but the candidate fails
closed before public writes:

```text
R6-DIRECT-PUBLICATION-PARITY HBP byte identity failed:
direct=1654 bytes compatibility=1654 bytes
```

The current direct publication frame is still seeded from skeleton/zero direct
state for parity-critical operands. The production manifest writer also still
uses compatibility provenance/checksum surfaces.

## Scope

In scope:

- populate the run-bound direct publication frame from typed direct run/lane/day
  operands that are authoritative for publication;
- expand direct publication state or capture surfaces as needed to cover the
  canonical R6 ledger for HBP, WAT, PASS, loss, and manifest;
- add anti-alias fixtures before accepting each publication family;
- add independent reconstruction for accepted HBP/WAT/PASS/loss operands;
- cut manifest production provenance/checksum reads over to typed direct
  publication projection in `DirectPublicationFrameCutover` mode;
- run the real cutover candidate until HBP byte identity, WAT Arrow/metadata
  parity, PASS Arrow/metadata parity, loss JSON parity, and manifest parity all
  pass;
- prove no direct public output reader uses compatibility WB13 rows, runtime
  symbols, writeback payloads, stale logical state, or wrappers around those
  structures after cutover acceptance;
- run default-disabled H2637 timing/output identity and direct-publication
  endpoint/RSS evidence after the output gates pass;
- preserve default compatibility behavior and protected-output identity while
  the opt-in cutover is still gated.

Out of scope unless this package is amended before implementation:

- default activation of direct publication;
- R7 hot compatibility runtime deletion;
- changing output schemas, column names, unit metadata, manifest schema IDs, or
  HBP binary format;
- changing process physics, conservation equations, phase order, or scheduler
  activation semantics;
- replacing missing typed direct operands with compatibility row wrappers;
- accepting one-sided bounds or self-consistency checks as output-family
  closure evidence.

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
- `crates/AGENTS.md`
- `tests/AGENTS.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` before any `SC-*`
  contract amendment.
- Relevant `SC-*` contracts before changing output meaning, units, aliases,
  physics lineage, guard semantics, or conservation authority.

## Dependencies

- R5E complete at pushed commit `d8f6bbea`.
- R6A complete with `DirectRunPublicationFrame` and direct projection
  consumers.
- R6 executed-hold committed as `cb10be17`, with final disposition
  `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER`.
- Canonical R6 publication operand ledger exists in
  `docs/architecture/array-native-runtime-specification.md` section
  `5.2.1 R6 Canonical Publication Operand Ledger`.

## Intended Write Set

- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/**`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/worker-handoff.md`
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
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/**`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-output/src/**` only for preserved-schema direct
  writer/projection tests.
- `tests/integration/**` for package guards, output parity, and
  no-compatibility proof tests.
- `tools/**` only for benchmark/comparison wrappers needed to record
  default-disabled or endpoint/RSS evidence.

Files outside this set require package amendment before edits.

## Phase Plan

1. Record required reading, owned-file manifest, scope selection, and current
   R6/R6A data-path evidence before Rust edits.
2. Build a field-level operand lineage table for HBP, WAT, PASS, loss, and
   manifest from the canonical ledger. Include units, basis, producer phase,
   direct-frame field, legacy alias, wrong aliases to reject, metadata
   obligation, and reconstruction obligation.
3. Contract-first gate: amend architecture or `SC-*` authority before changing
   output meaning, units, metadata, guard semantics, provenance, or process
   physics. If no authority amendment is needed, record why.
4. Populate `DirectRunPublicationFrame` from parity-grade typed direct run
   operands. Missing or invalid required operands must fail closed with typed
   errors; no silent defaults or compatibility wrappers are allowed.
5. Add anti-alias fixtures for HBP `peakro`, `watdur`, sediment fields, WAT
   water-balance fields, PASS volumes, loss static fields, and manifest
   provenance. Fixture expected values must differ from every rejected alias.
6. Add independent reconstruction for accepted HBP/WAT/PASS/loss operands.
   Reconstruction must not call the production direct projection builder under
   test.
7. Replace the cutover-mode manifest production provenance/checksum path with
   typed direct publication projection.
8. Run the `DirectPublicationFrameCutover` candidate and iterate until HBP,
   WAT, PASS, loss, and manifest gates pass. Do not write public direct outputs
   before current gates pass.
9. Prove no direct-publication consumer reads compatibility WB13 rows, runtime
   symbols, writeback payloads, hot tables, stale logical state, or diagnostic
   compatibility ledgers after acceptance.
10. Run default-disabled H2637 timing/protected-output identity and
    direct-publication endpoint/RSS benchmarks.
11. Run focused tests, package source scans, full Rust closure gates, scoped
    docs lint, and `git diff --check`.
12. Complete line-count governance, dual review, finding disposition, dual
    verification, final gate table, disposition, and worker handoff.

## Acceptance Criteria

- All five R6 worker-handoff items are complete or the package stops in a named
  hold with a concrete blocker.
- `DirectRunPublicationFrame` is populated from parity-grade typed direct
  run/lane/day operands, not skeleton/zero state or compatibility wrappers.
- HBP direct publication passes byte identity against the accepted compatibility
  baseline.
- WAT direct publication passes Arrow row/schema/field metadata/dataset
  metadata/producer metadata/calendar parity.
- PASS direct publication passes Arrow row/schema/field metadata/dataset
  metadata/producer metadata/volume basis parity, with a fixture that exercises
  PASS parquet output under the cutover candidate.
- Loss JSON direct publication passes byte-normalized JSON identity or a
  reviewed key-order-only normalization.
- Manifest publication/provenance/checksums read typed direct publication
  projection in cutover mode and pass schema ID, checksum, provenance, direct
  runtime counter, warning metadata, and output-policy parity.
- Anti-alias fixtures fail if any accepted output field reads a wrong alias
  from compatibility WB13 rows, runtime surfaces, stale logical state,
  diagnostic ledgers, adjacent direct diagnostics, area/volume denominators, or
  metadata shortcuts.
- Independent reconstruction agrees with accepted HBP/WAT/PASS/loss operands
  under declared tolerances and does not call the production projection builder
  under test.
- Direct-publication no-compatibility source scans and runtime counters prove
  accepted direct consumers do not read forbidden compatibility state.
- Default-disabled H2637 median remains `<= 676.67 s` with protected-output
  identity/equivalence.
- Direct-publication endpoint/RSS evidence is recorded and any regression is
  dispositioned before closure.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass before completion.
- Scoped markdown lint and `git diff --check` pass.
- Review and verification explicitly check Gate Evidence Non-Deferral,
  Consumer-Path Closure, Conservation / Publication Acceptance, anti-alias
  adequacy, independent reconstruction, no-compatibility proof, benchmark
  evidence, and line-count governance.

## Security Impact Gate

No secrets, credentials, production host actions, or external network
dependencies are in scope. R6B must preserve typed fail-closed validation,
serialization safeguards, and manifest checksum integrity. It must not add
fallback wrappers that silently mask missing direct publication operands.

## Hold-Lift Boundary

The first actionable follow-on is to implement the missing production bridge
from parsed inputs and accepted direct run/lane/day operands into
`DirectRunFrame` before `run_publication_capture` executes. That bridge must
fail closed for missing required publication operands, must not wrap
compatibility WB13 rows or runtime surfaces as direct authority, and must include
anti-alias and independent-reconstruction evidence before any public-output
family is accepted.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only operand-lineage, anti-alias, independent
reconstruction, no-compatibility source-scan, benchmark runner, reviewer, and
verifier subagents for the scopes declared above. Expected outputs are compact
findings, command logs, metrics, and review/verification findings recorded in
package artifacts. Write access is limited to package artifacts unless this
package is explicitly amended.

Subagent requirement: heavy closure gates, comparator/protected-output
comparisons, default-disabled H2637 timing, endpoint/RSS benchmarks, and full
workspace closure runs should be delegated to an available benchmark/closure
runner subagent. If no suitable subagent is available, record the failed spawn
or unavailable-tool evidence before running the command locally.

## Deliverables

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/scope-selection.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/data-path-proof.md`
- `artifacts/operand-lineage.md`
- `artifacts/publication-frame-population-plan.md`
- `artifacts/anti-alias-fixture-plan.md`
- `artifacts/independent-reconstruction-plan.md`
- `artifacts/manifest-cutover-plan.md`
- `artifacts/cutover-rerun-benchmark-plan.md`
- `artifacts/pre-implementation-contract-gate.md`
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
- `prompts/active/20260621-r6b-direct-publication-parity-manifest-cutover-001_kickoff_agent_prompt.md`
- `prompts/archived/README.md`

## Autonomy

When authorized for execution, run this package end to end. Do not stop after
diagnosis, producer-only wiring, shadow-only proof, or the first output family.
If a required current-scope gate cannot pass, record a named `HOLD` with exact
blocker evidence and a concrete first actionable follow-on.
