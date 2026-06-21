# R6H - Direct PMET Day-State Carry Builder

Status: scaffolded.

Package type: Defect-Closure ExecPlan / R6 direct WAT publication cutover.

Defect ID: `R6H-DIRECT-PMET-DAY-STATE-CARRY-BUILDER`.

## Defect-Closure Rule

If this package identifies a reproducible WAT publication root cause inside the
declared write set and the expected behavior is supported by canonical `SC-*`
authority, pinned-baseline provenance, or contract-authorized physical
invariants, it must land the contract-first correction in this package. It may
not stop at another diagnostic handoff, reduced mismatch list, or missing
producer label while the correction remains in-envelope.

## Purpose

Close the R6G hold marker:

`HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`

R6G reduced WAT mismatch to day-2 `Es`, `Total-Soil`, and
`SoilWaterTotal`. The mechanism is known: PMET component inputs are built from
a precomputed multi-day `DirectPublicationDayInput` vector before the prior
direct day commits carried layer state. R6H must replace that static vector
with an interleaved day-input builder that executes day `n`, commits direct
lane/day state, and constructs day `n+1` PMET operands from direct-carried
state only.

R6G review also left three WAT-publication authority blockers that must be
handled inside this package before it can truthfully claim WAT cutover:

- lane-dimensional direct day inputs for non-trivial OFE/lane cases;
- canonical WAT id semantics beyond the inherited single-WAT fixture;
- allowlisted direct symbol lineage for private seed-surface and dynamic PMET
  operands.

## Required Outcome

Terminal states:

- `COMPLETE-R6H-DIRECT-PMET-DAY-STATE-CARRY-BUILDER`: WAT row/schema/metadata
  parity passes from typed direct projection only; current-fixture HBP identity
  remains green; the R6G hold marker no longer fires; canonical WAT id
  semantics, lane-dimensional direct day inputs, anti-alias fixtures, and
  allowlisted direct symbol lineage are documented and gated; no forbidden
  compatibility authority is used.
- `HOLD-R6H-<SPECIFIC-BOUNDARY>`: only allowed after field-level reduction,
  attempted or ruled-out in-scope corrections, dual review, verification, and
  a new exact follow-on package. A same-field diagnostic relay is not an
  acceptable hold.

## In Scope

- Replace precomputed `DirectPublicationDayInput` construction with an
  interleaved direct day-input builder for `DirectPublicationFrameCutover`.
- Preserve day-0 static/climate seed behavior from R6G, then build each later
  day's PMET operands after the previous direct day commits direct-carried
  layer/state.
- Add or refactor direct runtime/runner APIs needed to execute, commit, and
  construct publication inputs one day at a time without compatibility aliases.
- Make direct publication day inputs lane-dimensional where the source state or
  process branch can vary by lane/OFE.
- Prove or correct WAT `wepp_id` semantics for direct publication, including
  non-trivial OFE/lane evidence.
- Build an allowlisted direct symbol lineage ledger for every private
  seed-surface input and dynamic PMET operand.
- Keep current-fixture HBP identity green while closing WAT.
- Add independent WAT operand reconstruction and anti-alias tests proving WAT
  does not read WB13 rows, compatibility runtime surfaces, writeback payloads,
  or writer rows as direct authority.

## Intended Write Set

- `crates/openwepp-runner/src/hillslope/*`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/*`
- Focused runner and orchestrator tests under the touched crates.
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`,
  `SC-SYSTEM-001.md`, or other directly governing `SC-*` contracts only when
  required to clarify existing WAT operand authority.
- Package-local artifacts under
  `docs/work-packages/20260621-r6h-direct-pmet-day-state-carry-builder-001/`.

## Out of Scope

- Default activation of direct publication.
- Treating compatibility WB13 rows, compatibility runtime surfaces, writeback
  payloads, writer rows, or compatibility output rows as direct authority.
- PASS/loss/manifest public-write cutover except as the next blocker after WAT
  parity is honestly reached.
- Provisional PMET/storage formulas without contract-first authority.
- Broad cleanup unrelated to the R6H WAT cutover path, except line-count
  extraction required by package governance.

## Security Impact Gate

This package must preserve fail-closed publication cutover behavior. It must not
add silent fallback wrappers around missing typed inputs, compatibility WB13
rows, runtime surfaces, or writer rows. Missing direct authority must surface as
a typed error or stable hold marker. No partial direct public outputs may be
written when any R6H WAT authority gate fails.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/architecture/array-native-runtime-specification.md` section 5.2.1
- R6G package and artifacts, especially `disposition.md`,
  `worker-handoff.md`, `review-disposition.md`, `r6g-operand-lineage.md`, and
  `r6g-no-compatibility-proof.md`.
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- Any `SC-*` contract governing changed process inputs, units, WAT id
  semantics, storage, PMET, or publication metadata.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/r6h-blocker-ledger.md`
- `artifacts/r6h-day-input-architecture.md`
- `artifacts/r6h-operand-lineage.md`
- `artifacts/r6h-wat-id-authority.md`
- `artifacts/r6h-independent-reconstruction.md`
- `artifacts/r6h-anti-alias-fixtures.md`
- `artifacts/r6h-no-compatibility-proof.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

## Validation Gates

- `cargo fmt --check`
- `cargo check -p openwepp-hillslope-orchestrator -p openwepp-runner`
- `cargo clippy --workspace --all-targets -- -D warnings`
- Focused direct runtime interleaved day-input/carry tests.
- Focused runner R6H tests proving WAT parity or a new exact hold marker after
  the R6G marker is gone.
- CLI `DirectPublicationFrameCutover` test proving fail-closed behavior if any
  later R6 gate remains, or public-write success only if all R6 gates pass.
- WAT row/schema/metadata parity from typed direct projection only.
- Multi-OFE/lane anti-alias fixture for lane-dimensional day inputs and WAT id
  semantics.
- Static no-compatibility scan for direct WAT producer and consumer paths.
- Allowlisted direct symbol lineage audit for private seed-surface and dynamic
  PMET operands.
- Independent WAT operand reconstruction.
- `cargo test --workspace`
- `cargo deny check`
- Dual review, finding disposition, dual verification, and line-count
  governance.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to Rust code-review and verification subagents for R6H
direct PMET day-state carry builder review and gate verification; expected
outputs are `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`; write access is read-only for review and
verification agents.

## Exit Criteria

- `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` is eliminated by a
  direct interleaved day-input builder, not by compatibility aliases.
- Current-fixture HBP identity remains green, with fixture limits stated.
- WAT row/schema/metadata parity passes from typed direct projection only, or a
  stable `HOLD-R6H-*` marker identifies a new boundary outside the write set
  after in-envelope corrections are attempted or ruled out.
- Canonical WAT id semantics and lane-dimensional publication inputs are
  proven or corrected for the WAT scope.
- No compatibility WB13 row, compatibility runtime surface, writeback payload,
  or writer row is used as direct WAT authority.
- Review findings are dispositioned as accepted, rejected, deferred, or
  follow-up, and verification artifacts independently check gate legitimacy.

## Phase Plan

1. Record required reading, source authority, and the exact R6G residual.
2. Design the interleaved direct day-input builder and lane-dimensional input
   shape before production edits.
3. Implement the smallest direct runtime/runner API needed to build day inputs
   after direct day commit.
4. Wire PMET `Es` and storage publication to direct-carried layer state.
5. Resolve WAT id semantics and add multi-OFE/lane anti-alias evidence.
6. Run focused gates, iterate until WAT parity passes or a new exact hold is
   proven, then run full closure gates.
7. Complete dual review, disposition, dual verification, line-count governance,
   and final package disposition.

## Progress

- [x] Scaffolded from R6G hold evidence.
- [ ] Record required reading and initial blocker ledger.
- [ ] Design interleaved day-input builder and lane-dimensional inputs.
- [ ] Implement PMET day-state carry builder.
- [ ] Prove WAT parity or establish a new exact hold.
- [ ] Complete reviews, verification, and final gate evidence.
