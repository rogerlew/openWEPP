# R7H Iterative Completion

Status: closed (operator-directed strategic re-sequence, 2026-06-24).

Current disposition: `CLOSED-OPT-IN` by operator decision on 2026-06-24.
Performance and zero-compatibility runtime evidence are accepted for opt-in
retention; direct default activation is explicitly deferred.

Package type: closed Defect-Closure ExecPlan / iterative R7H closure
continuation.

Closed defect: `R7H-003-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`,
reclassified as opt-in close plus reopened frost-depth fidelity gap.

This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, and
`docs/architecture/array-native-runtime-specification.md`.

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only reviewer and verifier subagents for performance evidence,
protected-output parity, no-compatibility proof, conservation/output
anti-tautology, line-count governance, and HOLD-legitimacy review. Expected
outputs are compact findings summarized into `artifacts/review-disposition.md`
and `artifacts/verification.md`; subagents may not edit files.

## Purpose

Historical execution purpose:

R7H was blocked after
`20260624-r7h-closure-activation-gates-001` because current H2637 direct
default-candidate reaches endpoint with zero compatibility counters but misses
the `<=91.2 s` performance gate and does not have current-code protected-output
parity evidence. This package owns that defect end-to-end.

Operator closure override (2026-06-24): R7H is closed `OPT-IN`, not finished.
The remaining typed-frost vs compatibility freeze divergence is not an R7H
parity defect to chase. Compatibility frost is conservation-closed but not
validated to frost-depth magnitude, so the public-output divergence is now a
contract-tracked delta under reopened `GAP-SNOWFREEZE-002`. Direct remains
opt-in; compatibility, rollback, and shadow paths remain intact.

## Non-Negotiable Terminal-State Rule

Historical rule superseded by the operator closure override above. The evidence
below remains useful execution history, but it is not an active instruction to
continue frost-vs-compatibility bit-parity.

This package has exactly two honest terminal states:

1. `COMPLETE-R7H-DIRECT-DEFAULT-READINESS`: H2637 default compatibility,
   explicit rollback, direct default-candidate, and explicit direct production
   pass on the same release binary; direct default and explicit direct complete
   within `<=91.2 s`; direct manifests report
   `compatibility_edge_invocations=0`; HBP/WAT/PASS/loss/plot/manifest
   protected parity is green; snow/frost anti-alias and independent operand
   reconstruction evidence is current; no forbidden compatibility authority
   remains in the winter hot path; rollback remains explicit; and R7H
   release-readiness/default-candidate activation state is updated.
2. `HOLD-R7H-<SPECIFIC-BOUNDARY>`: a blocker is reduced to a concrete mechanism
   and proven outside this package's declared authority envelope by direct
   evidence; dual review accepts the boundary; `artifacts/worker-handoff.md`
   names the next defect to close rather than a next inspection step.

Invalid terminal reasons by themselves:

- direct reaches endpoint but remains slower than `91.2 s`;
- performance is green but protected parity is red or unrun;
- direct and compatibility outputs differ without row/field/operand reduction;
- current compatibility was not rerun;
- profile evidence names an in-envelope hot-loop cost that was not corrected;
- a new fail-closed marker appears;
- a focused test passes before the H2637 matrix passes;
- another blocker remains inside this envelope;
- the implementation is complex;
- progress was made.

Each of those is an iteration target. If the mechanism is inside the authority
envelope, continue in this package.

## Correction Authority Envelope

Observed violation:

- `R7H-003-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`: current H2637 direct
  default-candidate endpoint evidence is `113.53 s / 1083636 KiB`, above the
  `91.2 s` `<=10x` budget, and protected parity is red/not current-matrix green.

In-scope defect mechanisms:

- winter/frost hot-loop overhead introduced by no-material frost safeguards,
  stale coarse frozen-layer clearing, fine/shadow carry preservation, direct
  winter-column state projection, direct publication projection, allocation,
  cloning, map/string lookups, or repeated per-layer scans;
- direct-vs-compatibility protected output residuals for HBP, WAT, PASS, loss,
  plot, and manifest where the producing process family is already in R7G/R7H
  direct scope;
- missing or stale direct publication operands for snow/frost-sensitive fields;
- no-compatibility counter/source-scan regressions in direct winter hot paths;
- default-candidate/explicit-direct mode-selection or manifest-provenance
  blockers;
- missing snow/frost anti-alias or independent operand reconstruction evidence
  required to accept parity;
- line-count governance issues in touched Rust files.

In-scope write set:

- `docs/work-packages/20260624-r7h-iterative-completion-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only for R7H state
  or acceptance wording updates required by execution evidence
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/winter/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**` only to expose
  typed, contract-authoritative helpers without changing process math
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- existing comparison, release, and evidence tooling used for H2637 parity,
  profiling, source scans, and reconstruction.

Allowed production edit classes:

- remove measured direct-mode overhead from allocations, clones, map/string
  work, redundant scans, or repeated projection recomputation;
- preserve snow/frost typed state while avoiding no-op layer rewrites and
  no-material storage mutations;
- correct direct publication operands from typed direct state and direct
  projections;
- add fail-closed guards for missing/non-authoritative direct operands;
- add tests/source scans/fixtures/comparison helpers that expose aliases;
- split direct runtime code when line-count governance requires it;
- amend canonical architecture or `SC-*` authority before any process-family,
  unit, or schema-meaning correction.

Protected boundaries:

- no process-physics formula, unit, or schema-meaning change without canonical
  `SC-*` or architecture authority;
- no comparator-match edits without independent authority;
- no compatibility WB13 rows, writeback surfaces, runtime symbol maps,
  scheduler requests, or stale logical state as production direct authority;
- no silent fallback wrappers for missing direct operands;
- no default activation while performance, parity, reconstruction,
  no-compatibility, or rollback gates are red or unrun.

## Conversion Rule

If this package establishes a reproducible root cause inside the envelope, and
the expected behavior is supported by canonical architecture, canonical `SC-*`
contracts, pinned-baseline provenance, or a contract-authorized
physical/publication invariant, this package must proceed through authority
confirmation or amendment, contract-derived tests, pre-implementation evidence,
production correction, validation, review, and disposition. It may not close as
`HOLD` because another in-envelope blocker remains or because the next action is
diagnostic.

## Required Iterative Loop

Repeat until terminal:

1. Reproduce the current H2637 matrix state or statically tie it to prior
   current-code evidence.
2. Record command, binary hash, fixture, mode, seconds, RSS, manifest counters,
   output paths, and current marker in `artifacts/performance.md` and
   `artifacts/blocker-ledger.md`.
3. If timing is red, profile or instrument the direct path enough to name file,
   function, branch, allocation, scan, or operand family; record the finding in
   `artifacts/profile-and-blockers.md`.
4. If parity is red, reduce it to output family, row/key, field, magnitude,
   producer, consumer, authority, and rejected aliases; record it in
   `artifacts/output-parity.md` and
   `artifacts/operand-reconstruction.md`.
5. Classify the next blocker as in-envelope or a legitimate boundary. If it is
   in-envelope, implement the correction, add or update focused regression,
   anti-alias, reconstruction, or source-scan coverage, and run focused gates.
6. Rerun the smallest relevant H2637 mode or parity check.
7. When direct endpoints are available, rerun the same-binary H2637 matrix:
   default compatibility, explicit rollback, direct default-candidate, and
   explicit direct production.
8. If a new blocker appears, return to step 3 or 4. Do not stop because the
   marker changed.
9. Before any terminal state, complete
   `artifacts/no-premature-stop-audit.md`, dual review disposition,
   verification, line-count governance, and final roadmap/catalog updates.

## Acceptance Gates

- H2637 same-binary matrix records default compatibility, explicit rollback,
  direct default-candidate, and explicit direct production.
- Direct default-candidate and explicit direct complete in `<=91.2 s`.
- Direct manifests report `compatibility_edge_invocations=0` and direct
  publication provenance.
- Protected outputs HBP/WAT/PASS/loss/plot and manifest parity are green for
  current-code direct versus compatibility/rollback evidence.
- Snow/frost-sensitive operand reconstruction and anti-alias evidence are
  current and do not self-restatingly reuse producer formulas.
- Source scans prove the winter direct hot path does not use forbidden
  compatibility request/symbol/writeback authority.
- Default activation/release-readiness is updated only after all gates pass;
  otherwise default activation remains disabled.
- Final root gates after production Rust changes:
  `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`; `git diff --check`; targeted
  Markdown lint.

## Deliverables

- `artifacts/required-reading.md`
- `artifacts/blocker-ledger.md`
- `artifacts/performance.md`
- `artifacts/profile-and-blockers.md`
- `artifacts/output-parity.md`
- `artifacts/operand-reconstruction.md`
- `artifacts/no-compatibility-proof.md`
- `artifacts/no-premature-stop-audit.md`
- `artifacts/line-count.md`
- `artifacts/verification.md`
- `artifacts/review-disposition.md`
- `artifacts/worker-handoff.md`
- active kickoff prompt under `prompts/active/`

## Progress

- [x] Scaffold continuation package and artifacts.
- [x] Reproduce current H2637 R7H blocker on current code.
- [x] Attribute performance blocker to named in-envelope mechanism or
      legitimate boundary.
- [x] Attribute protected parity blocker to named in-envelope mechanism or
      legitimate boundary.
- [ ] Implement all in-envelope corrections exposed by the loop.
- [ ] Rerun same-binary H2637 matrix.
- [ ] Complete no-compatibility, reconstruction, review, verification,
      line-count, and no-premature-stop evidence.
- [ ] Update roadmap/catalog and close complete or held.

## Surprises & Discoveries

- The direct endpoint performance blocker was not physics; it was valid-path
  frost fine-layer guard symbol formatting/allocation. Removing that hot-path
  formatting moved direct default-candidate from `112.99 s` to `61.40 s`.
- Protected parity remains red after the performance fix. Default compatibility
  and explicit rollback are byte/row identical, while direct default and
  explicit direct are byte-identical to each other. The direct-vs-compatibility
  residual starts on Julian day 6 as typed direct under-freezing relative to
  compatibility: `frozwt`, `frdp`, `Total-Soil`, and `SoilWaterTotal` diverge
  first, followed by downstream runoff fields.

## Decision Log

- The package owns performance and protected parity together because R7H cannot
  activate if either is red, and fixing one may expose or mask the other.

## Outcomes & Retrospective

Terminal disposition (2026-06-24): `CLOSED-OPT-IN — operator-directed strategic
re-sequence`.

Codex reached its usage limit mid-package; the R7H bit-parity work is
interrupted, not completed. On operator direction the package is closed without
finishing the frost bit-parity grind, for a deliberate reason: the remaining
blocker (`R7H-TYPED-FROST-FREEZE-PARITY`) is bit-parity to a compatibility frost
solver that is itself conservation-closed but never validated to legacy/physical
frost-depth magnitude (FDHP01 closed `GAP-SNOWFREEZE-002` at the ADR-0017
conservation/activation boundary, not a depth-magnitude target). Grinding direct
output to match it perfects a port of a frost model slated for replacement.

Decision:

- Performance objective MET and retained: the direct default-candidate endpoint
  reached `61.40 s` (~6.7x legacy, within the `<=10x` gate) after the hot-path
  frost-guard symbol-formatting removal. This fix is keep-worthy and must survive
  the working-tree cleanup.
- Protected-parity objective RE-CLASSIFIED, not abandoned: the frost-influenced
  public-output divergence (HBP/WAT/PASS frost + hydrology cascade) is
  reclassified from a blocking regression to a characterized, contract-tracked
  delta under reopened `GAP-SNOWFREEZE-002`. All non-frost surfaces remain
  bit-clean and stay gated.
- Direct mode remains OPT-IN (not default). Default activation and compatibility
  deletion are deferred until frost is contract-correct in the shipped runtime.

Working-tree cleanup disposition: completed in the R7H closeout pass. The
keep-worthy perf fix and reusable `r7g_frost_trace_*` instrumentation are
retained. The frost-storage changes in `runoff/storage/subsurface` are retained
only as stable aggregate/local-liquid plumbing with explicit opt-in authority;
they are not a compatibility frost bit-parity continuation.

Successor: a frost-depth heat-flow fidelity Defect-Closure ExecPlan validated
against historic frost-depth observations via site hillslope models
(`GAP-SNOWFREEZE-002` reopened; ADR-0017 external-authority discipline).
