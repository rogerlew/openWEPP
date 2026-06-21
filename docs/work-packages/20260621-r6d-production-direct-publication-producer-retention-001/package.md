# R6D - Production Direct Publication Producer Retention

Status: executed-held.

Package type: implementation work package / R6 hold-lift.

## Objective

Lift `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT` by adding a retained
production direct-publication producer surface to the climate lifecycle. The
runner must be able to carry direct run/lane/day publication rows out of
climate execution without building them from compatibility WB13 rows, runtime
surfaces, writeback payloads, or stale logical state.

R6D is not allowed to claim full R6 public-output cutover unless HBP, WAT,
PASS, loss, and manifest parity plus anti-alias, independent reconstruction,
no-compatibility, default-disabled, and endpoint/RSS gates pass in this package.
If retained producers exist but are not parity-grade for a required output
family, R6D must stop in a named `HOLD` with the missing producer family named.

## Execution Result

R6D lifted `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT` for the
`DirectPublicationFrameCutover` path by adding an opt-in retained
`DirectRunPublicationFrame` producer surface inside the production climate-day
loop. The retained rows are sourced from parsed climate/calendar and slope
geometry and are carried out of climate execution without constructing a
skeleton direct frame.

R6D remains executed-held at
`HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`. The retained frame now
proves run/lane/day row retention, but the production direct publication path
still lacks parity-grade hydrology, storage, subsurface, evaporation, PASS,
loss, manifest, and erosion producers. Public direct-output writes remain
fail-closed.

## Failure Being Corrected

R6C proved the R6B/R6C boundary was wrong: the output artifact builder could not
manufacture direct publication authority after compatibility climate execution.
The production climate lifecycle returned only compatibility publication
products and did not retain direct publication producers.

## Scope

In scope:

- add an opt-in retained direct-publication producer accumulator to
  `execute_hillslope_climate_days`;
- source accepted retained rows from parsed climate/calendar/geometry and
  direct-mode state only;
- keep compatibility WB13 rows, runtime publication symbols, writeback payloads,
  and stale logical state out of the retained direct source;
- make `DirectPublicationFrameCutover` consume the retained producer surface
  instead of running a post-hoc skeleton capture;
- fail closed with a precise hold if retained rows are incomplete, all-zero for
  required families, or lack parity-grade hydrology/erosion/manifest producers;
- preserve default-disabled compatibility behavior and protected output
  identity;
- update tests, no-compatibility proof, reviews, verification, and gate
  artifacts.

Out of scope unless amended before implementation:

- default direct-publication activation;
- deleting compatibility writers;
- changing public output schemas, HBP binary format, units, or process physics;
- using WB13/runtime/writeback compatibility products as direct authority;
- accepting skeleton-only, zero-only, or self-consistency-only output evidence
  as cutover completion.

## Authority

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/package.md`
- `docs/work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/artifacts/worker-handoff.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

## Dependencies

- R6C executed-held at pushed commit `7054a839` with
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`.
- R6 canonical publication operand ledger in
  `docs/architecture/array-native-runtime-specification.md` section
  `5.2.1`.

## Intended Write Set

- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/**`
- `docs/work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/artifacts/worker-handoff.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` only if a
  typed constructor/validator is needed.

Files outside this set require package amendment before edits.

## Phase Plan

1. Record required reading, scope, owned files, and pre-change data-path proof.
2. Add an opt-in retained direct-publication accumulator to climate execution,
   gated by runtime selection so default compatibility remains zero-cost.
3. Populate retained rows from parsed climate/calendar/geometry and direct
   publication state that exists during the climate lifecycle.
4. Build direct publication artifacts from the retained frame for shadow/cutover
   paths; do not run post-hoc skeleton publication capture for cutover.
5. Add fail-closed completeness and missing-producer guards for required
   output-family operands.
6. Update focused runner and CLI tests to prove retained producer behavior,
   fail-closed cutover, no public writes, and no skeleton capture.
7. Run no-compatibility scans, line-count governance, reviews, verification,
   package gates, full Rust gates, docs lint, and whitespace checks.
8. Close complete only if all R6 gates pass; otherwise close executed-held with
   the next precise missing producer family.

## Acceptance Criteria

- Production climate execution can retain a `DirectRunPublicationFrame` for
  opt-in direct publication modes without constructing it from WB13/runtime
  compatibility state.
- `DirectPublicationFrameCutover` consumes the retained frame and does not call
  post-hoc skeleton publication capture.
- Default compatibility mode constructs no retained direct publication surface.
- Missing required parity-grade output-family producers fail closed with a
  named hold and no public direct-output writes.
- Source scans and focused tests prove no accepted direct-publication consumer
  reads forbidden compatibility surfaces.
- R6 full cutover gates pass or the package stops in a named hold.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass before closure.
- `wctl doc-lint --path docs/work-packages` and `git diff --check` pass.
- Review and verification artifacts disposition findings and check
  Gate Evidence Non-Deferral, Consumer-Path Closure, publication acceptance,
  no-compatibility proof, benchmarks, and line-count governance.

## Security Impact Gate

No secrets, credentials, production host actions, or external network
dependencies are in scope. R6D must preserve typed fail-closed validation,
serialization safeguards, and manifest checksum integrity.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only producer-lineage, no-compatibility source-scan,
reviewer, verifier, and benchmark subagents for the scopes declared above.
Expected outputs are compact findings, command logs, and review/verification
evidence recorded in package artifacts. Write access is limited to package
artifacts unless this package is amended.

## Deliverables

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/scope-selection.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/data-path-proof.md`
- `artifacts/producer-retention-design.md`
- `artifacts/operand-lineage.md`
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
- `prompts/active/20260621-r6d-production-direct-publication-producer-retention-001_kickoff_agent_prompt.md`
- `prompts/archived/README.md`
