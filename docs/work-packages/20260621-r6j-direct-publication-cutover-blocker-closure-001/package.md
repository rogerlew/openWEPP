# R6J - Direct Publication Cutover Blocker Closure

Status: complete with final disposition
`COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`.

Package type: Defect-Closure ExecPlan / iterative R6 direct-publication cutover
closure.

Defect ID: `R6J-DIRECT-PUBLICATION-CUTOVER-BLOCKER`.

This ExecPlan is a living document. Maintain `Progress`, `Surprises &
Discoveries`, `Decision Log`, and `Outcomes & Retrospective` as execution
proceeds. This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, and `docs/work-packages/AGENTS.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for R6J
correctness review, no-compatibility proof review, gate-evidence audit, and
line-count governance review. Expected outputs are compact Markdown findings
summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files.

## Purpose / Big Picture

R6 is not complete until `DirectPublicationFrameCutover` writes production
HBP, WAT, PASS, loss, and run manifest outputs from typed direct projection
only. R6I closed the current-fixture HBP/WAT/PMET blocker and moved the
cutover candidate to the next fail-closed boundary:

`manifest direct projection is not wired to the production manifest writer`

R6J exists to close that blocker and continue through every remaining
in-envelope R6 direct-publication blocker until real direct publication
cutover is achieved. The worker must not stop after fixing manifest wiring if
PASS, loss, manifest parity, output writes, no-compatibility proof,
default-disabled isolation, endpoint/RSS, or anti-alias/reconstruction gates
remain incomplete.

## Non-Negotiable Terminal-State Rule

This package has exactly two honest terminal states:

1. `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`: production direct publication
   cutover succeeds. HBP, WAT, PASS, loss, and manifest are written from typed
   direct projection only; required byte/Arrow/schema/metadata/checksum parity
   and independent reconstruction gates pass; no forbidden compatibility
   authority remains in the cutover path; public output writes occur only after
   all gates pass.
2. `HOLD-R6J-<SPECIFIC-BOUNDARY>`: a blocker is reduced to a concrete
   mechanism and proven outside this package's declared authority envelope by
   direct evidence; dual review accepts the boundary; `artifacts/worker-handoff.md`
   names the next defect to close rather than a next inspection step.

The following are invalid terminal reasons by themselves:

- "manifest direct projection is not wired";
- "manifest parity failed";
- "PASS or loss is still compatibility-backed";
- "the next fail-closed marker changed";
- "HBP and WAT are green";
- "public output writes are still disabled";
- "a fixture, helper, or reconstruction check is missing";
- "another blocker remains";
- "the implementation is complex";
- "the package made progress";
- "another package should handle the next in-envelope blocker."

Each of those is an iteration target. If the mechanism is inside the authority
envelope, the worker must implement the correction, validate it, record it in
the blocker ledger, and re-run cutover until complete or a legitimate boundary
is proven.

## Current Starting State

R6I completed at
`COMPLETE-R6I-DIRECT-PMET-LAYER-ULP-PARITY`.

Current evidence:

- current-fixture HBP byte identity is green;
- current-fixture WAT identity is green;
- R6G and R6H WAT hold markers are absent;
- `DirectPublicationFrameCutover` still fails closed before public output
  writes;
- the first known blocker is
  `manifest direct projection is not wired to the production manifest writer`.

The first implementation action is to reproduce this exact state from the
current repository, record it in `artifacts/r6j-blocker-ledger.md`, and then
close the manifest writer cutover blocker without wrapping compatibility
manifest state as direct authority.

## Correction Authority Envelope

Observed violation:

- `R6J-DIRECT-PUBLICATION-CUTOVER-BLOCKER`: valid opt-in
  `DirectPublicationFrameCutover` input cannot yet complete R6 public output
  writes because remaining publication consumers and gates have not all been
  cut over to typed direct projection.

Current first blocker:

- `manifest direct projection is not wired to the production manifest writer`.

In-scope defect mechanisms:

- manifest writer wiring, provenance, metadata, checksum, and publication
  parity blockers;
- HBP byte identity regressions, including nonzero peak-runoff/event-duration
  fixture gaps;
- WAT Arrow row/schema/metadata/value parity blockers, including canonical
  multi-OFE `wepp_id` and row identity authority gaps;
- PASS Arrow row/schema/metadata/value parity blockers and PASS fixture gaps;
- loss JSON identity, metadata, and provenance blockers;
- any direct publication consumer still reading compatibility WB13 rows,
  compatibility runtime surfaces, writeback payloads, stale logical state,
  skeleton publication capture, or wrappers around those structures;
- missing typed direct publication producers for hydrology, storage,
  subsurface, evaporation, transfer, profile, interception, snow/frost,
  PASS-volume, loss, manifest, and erosion/sediment families;
- missing or insufficient anti-alias fixtures and independent operand
  reconstruction for accepted output families;
- default-disabled isolation regressions introduced by cutover plumbing;
- endpoint/runtime/RSS failures required by the R6 acceptance row;
- line-count governance issues in touched Rust files.

In-scope write set:

- `docs/work-packages/20260621-r6j-direct-publication-cutover-blocker-closure-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only when R6
  authority or acceptance wording must be clarified before implementation
- R6F/R6G/R6H/R6I handoff artifacts only when updating superseded pointers
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
  cutover, PASS/loss/manifest fixtures, anti-alias, reconstruction, and parity
  gates
- `tools/owcmp/**` or `tools/release/**` only for existing comparison,
  release-sidecar, or anti-evasion harness reuse; do not change
  authority-suite posture without running required anti-evasion guards.

Allowed production edit classes:

- wire typed direct manifest projection into the production manifest writer;
- add or correct typed direct publication operands from direct runtime state,
  direct phase projections, parsed inputs, and canonical runner execution
  state;
- make HBP/WAT/PASS/loss/manifest consumers read typed direct projection only
  on the cutover path;
- add fail-closed guards for missing or non-authoritative direct operands;
- add anti-alias fixtures and independent reconstruction tests for each
  accepted output family;
- add fixture coverage that exposes nonzero HBP runoff/event-duration, PASS,
  loss, manifest, and multi-OFE WAT aliases;
- split or move direct-publication helper code when required by line-count
  governance;
- amend canonical architecture or `SC-*` authority before process-family,
  unit, or schema-meaning corrections.

Protected boundaries:

- no default activation of direct publication;
- no compatibility WB13/runtime/writeback/stale logical source may be used as
  direct authority;
- no skeleton/direct self-consistency evidence may close cutover gates;
- no process-physics formula changes without contract-first authority;
- no output schema/unit/metadata meaning changes without parity evidence and
  canonical authority;
- no silent fallback wrappers for missing direct producers;
- no broad refactors unrelated to cutover closure or line-count governance;
- no public direct outputs on fail-closed paths.

Authority:

- `docs/architecture/array-native-runtime-specification.md`, especially the R6
  publication operand ledger and R6 acceptance criteria.
- R6 through R6I package artifacts, especially
  `docs/work-packages/20260621-r6i-direct-pmet-layer-ulp-parity-001/artifacts/worker-handoff.md`.
- Canonical `SC-*` contracts for process-family publication operands.
- Pinned baseline provenance at `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` when contract confirmation or
  amendment needs source provenance.

## Conversion Rule

If R6J establishes a reproducible root cause inside this envelope, and the
expected behavior is supported by canonical architecture, canonical `SC-*`
contracts, pinned-baseline provenance, or a contract-authorized
physical/publication invariant, R6J must proceed through authority
confirmation or amendment, contract-derived tests, pre-implementation
evidence, production correction, validation, review, and disposition in this
package. It may not close as `HOLD` because a later blocker remains, because
the implementation is large, because another work package could perform the
same correction, or because focused gates passed before full cutover passed.

## Required Iterative Loop

Repeat this loop until `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER` or a
legitimate `HOLD-R6J-<SPECIFIC-BOUNDARY>`:

1. Reproduce the current cutover state and record command, marker, stderr,
   output-file state, direct counters, and candidate artifact state.
2. Reduce the next blocker to output family, file, row, field, metadata key,
   byte span, operand, producer, consumer, and authority.
3. Record the blocker in `artifacts/r6j-blocker-ledger.md` with status,
   evidence, owner, correction plan, tests, and acceptance gate.
4. Determine whether the blocker is inside this envelope. If yes, implement
   the correction in this package.
5. Add or update contract-derived tests, anti-alias fixtures, independent
   reconstruction, and no-compatibility scans before accepting an output-family
   correction.
6. Run focused validation for the corrected family plus any directly adjacent
   publication family.
7. Re-run `DirectPublicationFrameCutover`.
8. If a new blocker appears, return to step 2. Do not stop because the marker
   changed.
9. Do not write public direct outputs until every required parity and manifest
   gate passes.
10. After all output families pass focused cutover, run the full root closure
    gates and record final evidence.

## Premature-Stop Prevention Rules

The worker must complete `artifacts/no-premature-stop-audit.md` before any
final disposition. The audit must answer every item below with evidence.

1. A fail-closed marker is a symptom, not a boundary.
2. A parity mismatch is a defect symptom, not a boundary.
3. A field, metadata, checksum, or byte mismatch is not a boundary until the
   direct producer, consumer, and authority are traced.
4. A missing manifest writer path is not a boundary when manifest writer wiring
   is in the write set.
5. A missing PASS/loss/WAT/HBP fixture is not a boundary when fixture creation
   is in the write set.
6. A missing comparison helper or reconstruction helper is not a boundary when
   adding the helper is in the write set.
7. A missing direct producer is not a boundary when the producer belongs to an
   in-scope direct runtime process family and has architecture or contract
   authority.
8. A new blocker exposed after fixing one blocker starts the next iteration in
   this package.
9. Full R6 closure cannot be claimed while any public output family still uses
   compatibility authority, has unproven parity, lacks anti-alias evidence, or
   lacks independent reconstruction.
10. "Made progress", "focused tests pass", "the next blocker is known", and
    "the remaining work is follow-up" are not closure evidence.

If any audit item fails, continue the R6J loop or amend the package before
implementation with an explicit reviewed scope change. Do not close complete.

## HOLD Legitimacy Checklist

A `HOLD` is allowed only when every item below is true and recorded in
`artifacts/no-premature-stop-audit.md`:

- the blocker has a stable `HOLD-R6J-...` marker;
- the blocker is reduced to concrete output family, field/row/metadata item,
  direct operand, producer, and consumer;
- the exact reason it is outside the package envelope is cited;
- all plausible in-envelope corrections have been attempted or ruled out with
  evidence;
- missing authority is documented as a specific canonical architecture or
  `SC-*` gap, not general uncertainty;
- dual reviews accept the `HOLD` legitimacy;
- the worker handoff first actionable item is `close defect <id>` with an exact
  authority/write-set boundary, not "inspect", "investigate", or "trace";
- if the blocker is out of scope, a follow-on package is scaffolded or the
  current package is amended before stopping.

If any item is false, continue the loop.

## Scope

In scope:

- reproduce and document the current R6I manifest writer fail-closed state;
- implement manifest direct projection writer cutover without compatibility
  manifest wrapping;
- continue through every subsequent in-envelope HBP/WAT/PASS/loss/manifest
  parity and consumer-path blocker;
- prove successful public output writes only from typed direct projection;
- prove byte/Arrow/schema/metadata/checksum parity for required output
  families;
- add anti-alias fixtures and independent operand reconstruction;
- prove no compatibility authority in direct publication cutover;
- preserve compatibility-mode output identity and default-disabled isolation;
- satisfy endpoint/runtime/RSS gates required by the R6 architecture row;
- complete dual review, verification, line-count governance, gate evidence,
  no-premature-stop audit, worker handoff, and final disposition.

Out of scope:

- default activation of direct publication;
- unrelated process-physics magnitude corrections;
- unrelated performance tuning not required to preserve R6 default-disabled or
  endpoint/RSS gates;
- broad style refactors not required by cutover closure or line-count
  governance;
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
- `docs/work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/package.md`
- `docs/work-packages/20260621-r6f-direct-publication-cutover-blocker-closure-001/package.md`
- `docs/work-packages/20260621-r6g-direct-wat-producer-authority-001/package.md`
- `docs/work-packages/20260621-r6h-direct-pmet-day-state-carry-builder-001/package.md`
- `docs/work-packages/20260621-r6i-direct-pmet-layer-ulp-parity-001/package.md`
- `docs/work-packages/20260621-r6i-direct-pmet-layer-ulp-parity-001/artifacts/execution-evidence.md`
- `docs/work-packages/20260621-r6i-direct-pmet-layer-ulp-parity-001/artifacts/worker-handoff.md`

Before Rust edits:

- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- canonical `SC-*` contracts for any process-family operand whose authority is
  ambiguous;
- pinned baseline source under `/workdir/wepp-forest_260430_baseline` when
  provenance is needed for a contract amendment;
- `tools/release/check_authority_suite_antievasion.sh` and
  `cargo test --test auth11_required_suite_obligation_guards_contract` before
  any change touching external-authority suite posture, cohort fixtures, or
  required-case bindings.

## Deliverables

- Updated production code that completes direct publication cutover or a
  legitimate boundary hold with evidence.
- `artifacts/r6j-blocker-ledger.md` maintained after every loop iteration.
- `artifacts/no-compatibility-proof.md` proving forbidden sources are absent
  from the cutover path.
- `artifacts/output-parity-and-reconstruction.md` proving HBP, WAT, PASS,
  loss, and manifest parity plus independent reconstruction.
- `artifacts/no-premature-stop-audit.md` proving no in-envelope blocker was
  deferred.
- `artifacts/review-disposition.md` with dual review and explicit finding
  disposition.
- `artifacts/verification.md` with dual verification and gate table.
- `artifacts/line-count-governance.md`.
- `artifacts/worker-handoff.md`.
- Updated `docs/work-packages/README.md` and `docs/ROADMAP.md`.

## Validation Gates

Focused gates:

- Reproduce the inherited R6I manifest writer fail-closed marker.
- Manifest direct projection writer cutover focused test.
- HBP byte identity test, including a nonzero runoff/event-duration fixture if
  current coverage does not exercise the producer.
- WAT Arrow row/schema/metadata/value parity tests, including multi-OFE
  identity where applicable.
- PASS Arrow row/schema/metadata/value parity tests.
- Loss JSON parity tests.
- Manifest provenance/checksum/metadata parity tests.
- CLI cutover test proving public direct output writes only after all gates
  pass.
- Static and test-backed no-compatibility proof for the cutover path.
- Default-disabled isolation gate for any new always-on plumbing.
- Endpoint/runtime/RSS gate required by the R6 architecture row.

Root closure gates:

- `cargo fmt --check`
- `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`

Review and artifact gates:

- Dual independent reviews with finding disposition.
- Dual verification with explicit gate status.
- Gate Evidence Non-Deferral Rule audit.
- Consumer-Path Closure Rule audit.
- Conservation / Publication Acceptance Rule audit.
- No-premature-stop audit.
- Line-count governance: every touched `.rs` file at 2000+ lines is `WARN`
  with rationale; every touched 3000+ non-exempt `.rs` file is split before
  closure or backed by explicit owner-approved exception and sunset plan.

## Progress

- [x] (2026-06-21) Scaffolded from R6I manifest writer handoff.
- [x] Reproduce current R6I cutover state and record first ledger entry.
- [x] Close manifest direct projection writer cutover without compatibility
  wrapping.
- [x] Iterate through subsequent in-envelope R6 blockers until public direct
  output writes succeed.
- [x] Prove HBP/WAT/PASS/loss/manifest parity and independent reconstruction.
- [x] Prove no-compatibility authority and default-disabled isolation.
- [x] Run endpoint/RSS gates and close H2637 direct-cutover public-output
  parity.
- [x] Complete dual review, dual verification, no-premature-stop audit,
  line-count governance, worker handoff, and final disposition.

## Surprises & Discoveries

- Enabling PASS parquet after manifest wiring exposed a second in-envelope
  blocker: direct PASS projected calendar year instead of simulation year and
  used an HBP/runoff peak fallback for `peakro`.
- After public writes succeeded, the production cutover gate still used
  compatibility HBP/loss/WAT/PASS artifacts as an in-run parity oracle. R6J
  moved those checks to test evidence so production cutover validates direct
  artifacts only.
- The manifest schema did not yet expose direct runtime counters. R6J added an
  optional `direct_runtime_counters` object for direct cutover manifests while
  leaving compatibility manifests unchanged.
- H2637 opt-in direct cutover initially exposed scale-only public-output
  blockers after current-fixture parity passed. The reductions were concrete:
  PASS needed outlet-only public projection, HBP needed HBP-specific erosion
  operands from producer-authoritative runtime scalars, and PASS Parquet needed
  the same stable Arrow schema metadata path already used by WAT. Fresh
  same-binary H2637 default and direct runs then closed HBP/WAT/PASS/loss/plot
  byte identity.

## Decision Log

- Decision: Scope R6J to the full remaining R6 direct-publication cutover
  envelope rather than a manifest-only package.
  Rationale: R6I left manifest writer wiring as the first blocker, but R6
  closure requires parity and direct publication cutover across all public
  output families. A manifest-only package would recreate the premature
  diagnostic handoff failure mode.
  Date/Author: 2026-06-21 / Codex.

## Outcomes & Retrospective

Final disposition:
`COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`.

R6J closes the inherited R6I manifest writer blocker and several subsequent
in-envelope R6 blockers exposed during cutover execution. Current-fixture
opt-in `DirectPublicationFrameCutover` now writes production HBP, loss, PASS
parquet, WAT parquet, plot, and run manifest outputs from direct publication
artifacts generated by the cutover adapter. The production cutover writer no
longer builds compatibility HBP/loss/WAT/PASS artifacts as an authority gate;
those parity checks are retained as focused test evidence.

Manifest provenance now reports `direct-publication-frame`, empty replay
candidate surfaces, direct row counts/keys, unique-OFE area, run-local direct
runtime counters, and output checksums for every public output. The CLI
contract proves current-fixture public writes and manifest direct-counter
evidence through `--direct-publication-frame-cutover`. Focused parity evidence
covers HBP bytes, WAT rows, PASS rows, loss JSON, disk checksum/readback, and a
two-OFE direct publication row/provenance shape.

The H2637 endpoint/RSS gate is now closed on fresh same-binary release output.
Default H2637 ran in `640.41 s` with `227396 KiB` RSS. Opt-in
`DirectPublicationFrameCutover` ran in `637.53 s` with `349400 KiB` RSS and
`compatibility_edge_invocations = 0`. HBP, WAT parquet, PASS parquet, loss
JSON, and plot parquet are byte-identical between default and direct cutover.
DuckDB row-set checks report zero bidirectional differences for WAT
(`235961` rows) and PASS (`12419` rows). Direct manifest provenance and output
checksums are direct-publication-frame sourced.

Evidence:

- `artifacts/r6j-blocker-ledger.md`
- `artifacts/no-compatibility-proof.md`
- `artifacts/output-parity-and-reconstruction.md`
- `artifacts/no-premature-stop-audit.md`
- `artifacts/review-disposition.md`
- `artifacts/verification.md`
- `artifacts/line-count-governance.md`
- `artifacts/worker-handoff.md`
