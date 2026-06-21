# R6F - Direct Publication Cutover Blocker Closure

Status: executed-held.

Package type: Defect-Closure ExecPlan / iterative R6 direct-publication cutover
closure.

Defect ID: `R6F-DIRECT-PUBLICATION-CUTOVER-BLOCKER`.

This ExecPlan is a living document. Maintain `Progress`, `Surprises &
Discoveries`, `Decision Log`, and `Outcomes & Retrospective` as execution
proceeds. This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, and `docs/work-packages/AGENTS.md`.

## Purpose / Big Picture

R6 direct publication is not complete until `DirectPublicationFrameCutover`
writes production HBP, WAT, PASS, loss, and run manifest outputs from typed
direct projection only. R6E resolved the prior production direct-runtime input
binding absence and moved the first blocker to HBP direct-process parity:
`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

R6F exists to close that blocker and keep iterating through every subsequent R6
publication blocker until direct publication cutover is real. A worker executing
this package must not stop after naming a mismatch. It must reduce the mismatch
to exact output fields, direct operands, producers, authority, and corrections,
then implement and validate in-envelope fixes. Closure requires successful
direct cutover parity and public output writes, not another diagnostic handoff.

## Non-Negotiable Terminal-State Rule

This package has two honest terminal states:

1. `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`: HBP, WAT, PASS, loss, and manifest
   are written from typed direct projection only, pass required parity and
   reconstruction gates, and no compatibility authority is used.
2. `HOLD-R6F-<SPECIFIC-BOUNDARY>`: a blocker is proven outside this package's
   declared authority envelope by concrete evidence, dual review accepts that
   boundary, and the handoff names the next defect to close rather than the next
   thing to inspect.

The following are not valid terminal reasons by themselves:

- "HBP byte identity failed";
- "direct process parity mismatch";
- "manifest is not wired";
- "PASS fixture is missing";
- "more investigation is possible";
- "the fix is complex";
- "another blocker might remain";
- "this should be a follow-up package";
- "we made progress and preserved fail-closed behavior."

Each of those is an iteration target. The worker must continue reducing and
correcting it unless the `HOLD Legitimacy Checklist` below is fully satisfied.

## Current Failure

R6E left the repository at:

- direct cutover reaches HBP comparison;
- CLI exits `1`;
- stderr includes
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH HBP byte identity failed:
  direct=1654 bytes compatibility=1654 bytes`;
- no public outputs are written;
- direct frame, executor, publication capture, direct compute, state mutation,
  downstream operand, and shadow projection counters execute;
- skeleton-run and compatibility-edge counters remain zero.

R6F must start by reproducing this exact state, then decode and reduce the HBP
byte mismatch. It may not repeat the R6E mistake of treating the named mismatch
as sufficient reason to stop.

## Correction Authority Envelope

Observed violation:

- `R6F-DIRECT-PUBLICATION-CUTOVER-BLOCKER`: valid opt-in
  `DirectPublicationFrameCutover` input still cannot write public direct
  outputs because direct projection fails required parity and publication
  cutover gates.

Current first blocker:

- `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

In-scope defect mechanisms:

- HBP byte mismatch, including exact field-level mismatch reduction, HBP decode
  tooling, writer parity, direct operand value parity, row ordering, units,
  area/volume basis, and metadata/provenance mismatch;
- missing or wrong direct publication producers for hydrology, storage,
  subsurface, evaporation, transfer, profile, interception, snow/frost,
  PASS-volume, loss, manifest, and erosion/sediment families;
- direct consumers still reading compatibility WB13 rows, compatibility runtime
  surfaces, writeback payloads, stale logical state, skeleton publication
  capture, or wrappers around those structures;
- WAT/PASS Arrow row, schema, metadata, and value parity blockers;
- loss JSON identity blockers;
- manifest provenance/checksum parity blockers;
- PASS fixture absence or insufficient fixture coverage;
- anti-alias fixture gaps and independent reconstruction gaps;
- line-count governance issues in touched Rust files.

In-scope write set:

- `docs/work-packages/20260621-r6f-direct-publication-cutover-blocker-closure-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only when R6
  authority or acceptance wording must be clarified before code changes
- R6D/R6E handoff artifacts only when updating superseded pointers
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- `crates/openwepp-hillslope-output/**`
- `crates/openwepp-summary-accumulator/**`
- `tests/fixtures/**` and `tests/integration/**` for direct-publication
  cutover, PASS fixture coverage, anti-alias, reconstruction, and parity gates
- `tools/owcmp/**` or `tools/release/**` only for existing comparison or
  release-sidecar harness reuse; do not change authority-suite posture without
  anti-evasion guards.

Allowed production edit classes:

- decode and compare HBP direct and compatibility candidates to identify exact
  differing fields and operands;
- add or correct typed direct publication operands from direct runtime state,
  direct phase projections, parsed inputs, and canonical runner execution state;
- amend or confirm canonical `SC-*` authority before process-physics or unit
  corrections;
- make HBP/WAT/PASS/loss/manifest consumers read typed direct projection only
  on the cutover path;
- add fail-closed guards for missing or non-authoritative direct operands;
- add anti-alias fixtures and independent reconstruction tests;
- add fixture coverage that includes PASS Parquet;
- split touched Rust files when line-count governance requires it.

Protected boundaries:

- no default activation of direct publication;
- no compatibility WB13/runtime/writeback/stale logical source used as direct
  authority;
- no skeleton/direct self-consistency evidence accepted as cutover closure;
- no process-physics formula changes without contract-first authority;
- no output schema/unit/metadata meaning changes without parity evidence and
  canonical authority;
- no silent fallback wrappers for missing direct producers;
- no broad refactors unrelated to cutover closure or line-count governance.

Authority:

- `docs/architecture/array-native-runtime-specification.md`, especially the R6
  publication operand ledger and R6 acceptance criteria.
- R6 through R6E package artifacts, especially R6E disposition and blocker
  ledger.
- Canonical `SC-*` contracts for process-family publication operands.
- Pinned baseline provenance at `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` when contract confirmation or
  amendment needs source provenance.

## Premature-Stop Prevention Rules

The worker must apply these rules before any `HOLD` disposition:

1. A parity mismatch is a defect symptom, not a boundary. Reduce it to fields,
   row keys, operands, units, producer, and authority.
2. A field mismatch is not a boundary until the direct producer and expected
   authority have been traced.
3. A missing fixture is not a boundary when fixture creation is in the write set.
4. A missing comparison helper is not a boundary when adding the helper is in
   the write set.
5. A missing direct producer is not a boundary when the producer belongs to an
   already in-scope direct runtime process family and has contract authority.
6. A manifest provenance mismatch is not a boundary when manifest writer wiring
   is in the write set.
7. A new blocker found after fixing one blocker starts the next iteration in
   this package. It does not justify stopping.
8. "Current package made progress" is never closure evidence.

The disposition artifact must include a `Premature-Stop Audit` section proving
that each rule was satisfied. Reviews must reject the package if it stops at a
named mismatch without reducing and correcting every in-envelope mechanism.

## HOLD Legitimacy Checklist

A `HOLD` is allowed only when all checks are true and recorded in
`artifacts/no-premature-stop-audit.md`:

- the blocker has a stable marker `HOLD-R6F-...`;
- the blocker is reduced to a concrete output family, field/row/metadata item,
  direct operand, and producer or consumer;
- the exact reason it is outside the package envelope is cited;
- all plausible in-envelope corrections have been attempted or ruled out with
  evidence;
- missing authority is documented as a specific canonical `SC-*` gap or
  contradictory authority, not as general uncertainty;
- dual reviews accept the `HOLD` legitimacy;
- the worker handoff first actionable item is `close defect <id>` with an
  exact authority/write-set boundary, not "inspect" or "investigate";
- if the blocker is out of scope, a follow-on package is scaffolded or the
  current package is amended before stopping.

If any item is false, continue the R6F loop.

## Required Iterative Loop

Repeat this loop until complete cutover or a legitimate `HOLD` boundary:

1. Reproduce the current cutover failure and record command, marker, stderr,
   output file state, direct counters, and candidate artifact state.
2. Decode or otherwise reduce the next output mismatch to the smallest
   observable unit: file, row, field, metadata key, byte span, operand, producer,
   and authority.
3. Record the blocker in `artifacts/r6f-blocker-ledger.md`.
4. Determine whether the blocker is in-envelope. If yes, implement the
   correction in this package.
5. Add or update contract-derived tests, anti-alias fixtures, and independent
   reconstruction before accepting an output-family correction.
6. Run focused validation and no-compatibility scans.
7. Re-run `DirectPublicationFrameCutover`.
8. If the first blocker is fixed and a new blocker appears, return to step 2.
9. Do not write public direct outputs until every required parity and manifest
   gate passes.

## Required HBP First Pass

R6F must begin with HBP because R6E reaches HBP byte comparison first.

Minimum HBP work:

- produce direct and compatibility HBP candidate bytes without writing public
  outputs;
- parse or decode enough of both byte streams to identify the exact mismatching
  fields, row positions, and byte spans;
- map mismatching fields to `DirectPublicationDayRow` operands and writer
  inputs;
- map each operand to direct phase state/projection and canonical authority;
- reject plausible aliases such as compatibility WB13 rows, zero/default
  operands, stale runtime state, wrong area basis, and wrong unit conversions;
- implement in-envelope corrections;
- prove HBP byte identity before moving to WAT/PASS/loss/manifest.

If HBP decode support is insufficient, adding focused HBP comparison support is
in scope. Lack of decoder convenience is not a `HOLD` reason.

## Scope

In scope:

- close HBP byte identity;
- close WAT Arrow row/schema/metadata parity;
- close PASS Arrow row/schema/metadata parity, including adding/selecting a
  fixture with PASS Parquet output;
- close loss JSON identity;
- close direct manifest provenance/checksum parity;
- prove successful public output writes under `DirectPublicationFrameCutover`;
- prove no forbidden compatibility authority source is used;
- add anti-alias fixtures and independent reconstruction for accepted output
  families;
- preserve default-disabled compatibility behavior;
- update package artifacts, reviews, verifications, roadmap/catalog, and
  worker handoff.

Out of scope:

- default activation of direct publication;
- unrelated performance work;
- unrelated science/magnitude corrections outside R6 publication parity;
- broad style refactors not required by cutover or line-count governance;
- stopping after a diagnostic-only step.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/defect_closure_execplans.md`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md`
- `docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/package.md`
- `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/package.md`
- `docs/work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/package.md`
- `docs/work-packages/20260621-r6d-production-direct-publication-producer-retention-001/package.md`
- `docs/work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/package.md`
- `docs/work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/artifacts/r6e-blocker-ledger.md`
- `docs/work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/artifacts/gate-results.md`

Required before Rust or contract edits:

- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- target `SC-*` contracts for each process-family operand being corrected.

Conditional:

- `tests/AGENTS.md` before integration fixture/test edits.
- `docs/standards/AGENTS.md` before prompt wording or reusable standards edits.

## Progress

- [x] 2026-06-21 scaffolded R6F package with explicit anti-premature-stop
  language.
- [x] Execution: read required context and reproduce R6E HBP parity blocker.
- [x] Execution: reduce HBP byte mismatch to exact fields, operands, producers,
  and authority.
- [x] Execution: implement all in-envelope corrections for the inherited
  near-zero HBP fixture and prove current-fixture HBP byte identity.
- [x] Execution: iterate to WAT and reduce the next blocker to exact fields,
  operands, producers, and authority.
- [x] Execution: add direct-runtime typed input/carry coverage and HBP/WAT
  blocker-reduction tests.
- [ ] Execution: add anti-alias fixtures and independent reconstruction for
  full WAT parity.
- [ ] Execution: prove public direct output writes and no-compatibility
  authority.
- [x] Execution: run focused gates, dual reviews, dual verification, disposition,
  and handoff.

## Surprises & Discoveries

- Scaffold note: R6E stopped prematurely at HBP direct process parity. R6F is
  intentionally worded to make that failure mode invalid unless a full `HOLD`
  legitimacy audit proves an out-of-envelope boundary.
- 2026-06-21: The inherited HBP byte mismatch reduced to near-zero runoff peak
  operands. Direct publication now emits the compatibility near-zero
  `peakro`/`watdur` pair when direct runoff is below the WB16 near-zero
  threshold, and HBP byte identity is green on the current CLI fixture.
  Nonzero peak-runoff and distinct event-duration fixture coverage remains an
  R6 continuation gate.
- 2026-06-21: The next blocker is WAT, not HBP. Reduced fields are `wepp_id`,
  output simulation `year`, `Es`, `Total-Soil`, `SoilWaterTotal`,
  `ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, and `ProfileWPStore`.
- 2026-06-21: Direct runtime now has the required structural slots for typed
  process inputs and lane-carried layer state, but production runner binding
  still only supplies climate/calendar. Filling the remaining WAT fields from
  compatibility WB13 rows or runtime surfaces would violate section 5.2.1 of
  the array-native runtime specification.

## Decision Log

- 2026-06-21: Chose R6F as a DC-ExecPlan rather than another narrow HBP-only
  diagnostic package because the objective is full R6 cutover and the known
  failure mode is premature handoff after one blocker.
- 2026-06-21: Accepted `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`
  after current-fixture HBP identity was proven, WAT was reduced to concrete
  fields/operands, the direct runtime input/carry structure was implemented and
  tested, and the remaining production producer requires
  contract-authoritative parsed-input binding work scaffolded as R6G.
- 2026-06-21: Review B found required clippy failure and an incomplete R6G
  scaffold. R6F fixed the clippy issues, moved WAT reducer helpers out of the
  intake file to keep it below the 3000-line hard threshold, completed the R6G
  scaffold shape, and reran final gates.

## Outcomes & Retrospective

Final verdict:
`HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`.

R6F closed the inherited near-zero HBP blocker for the current fixture and
advanced cutover to the next concrete publication family. It did not complete
R6. The honest immediate boundary is not another generic parity mismatch; it is
the missing production typed producer that must feed ET/storage/profile WAT
operands from parsed inputs and direct runtime state without compatibility
WB13/runtime-surface authority. Full R6 cutover still also needs the
architecture-required nonzero HBP fixture, PASS/loss/manifest parity, public
writes, and final full gates.

## Delivered In R6F

- Production code and tests for the in-envelope near-zero HBP publication
  operand fix.
- Direct climate precipitation unit correction.
- Direct runtime typed process input slots, lane-carried layer state, and
  profile projection fields needed by the next WAT producer binding.
- WAT blocker reduction to exact mismatch fields and a guarded stable hold
  marker.
- No-compatibility proof for the rejected WB13/runtime-surface shortcut.
- R6G scaffold for the production parsed-input producer authority gap.
- Updated roadmap/catalog and R6F handoff artifacts.
- Complete review, verification, gate, line-count, and disposition artifacts.

## Blocked Continuation Deliverables

- Full WAT row/schema/metadata parity.
- Nonzero HBP peak-runoff/event-duration fixture coverage.
- PASS/loss/manifest parity evidence.
- Anti-alias fixtures and independent reconstruction evidence for accepted
  WAT/PASS/loss/manifest operands.
- Successful `DirectPublicationFrameCutover` public output writes.

## Validation Gates

Focused iteration gates:

- direct CLI cutover reproduction;
- HBP decode/diff evidence;
- focused tests for each corrected output family;
- no-compatibility scans for corrected producers and consumers;
- anti-alias and independent reconstruction checks for accepted operands.

Final gates:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `wctl doc-lint --path docs/work-packages`
- `git diff --check`
- successful `DirectPublicationFrameCutover` output write on the target fixture;
- HBP byte identity;
- WAT/PASS Arrow row/schema/metadata parity;
- loss JSON identity;
- manifest provenance/checksum parity;
- explicit default-disabled compatibility isolation evidence;
- dual review and dual verification with finding disposition;
- line-count governance for every touched `.rs` file.

## Review Requirements

Run two independent reviews before final disposition. Each review must check:

- DC envelope adequacy;
- `HOLD` legitimacy if any hold remains;
- premature-stop audit;
- gate evidence non-deferral;
- consumer-path closure;
- conservation/publication acceptance;
- no-compatibility proof;
- anti-alias and independent reconstruction evidence;
- line-count governance.

Every finding must be dispositioned as `accepted`, `rejected`, `deferred`, or
`follow-up` with rationale. Accepted findings must be fixed and verified before
closure.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only blocker-reduction, HBP parity-diff, WAT/PASS parity, manifest
parity, no-compatibility, review, and verification subagents for R6 cutover
execution. Expected outputs are compact findings, field/operand maps, command
logs, metrics, and package artifact updates. Write access is bounded to this
package's declared write set; review and verification agents should default to
read-only findings unless specifically asked to patch artifacts.

## Worker Handoff Rule

The final handoff must start with `close defect R6F-DIRECT-PUBLICATION-CUTOVER-
BLOCKER` if any blocker remains. It must not start with "inspect", "trace",
"investigate", or "determine whether". If a legitimate `HOLD` exists, the
handoff must name the exact out-of-envelope boundary and point to the evidence
that proves it.
