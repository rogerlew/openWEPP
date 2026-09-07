# Stage 3 authentic coverage and memory attribution

This is a living ExecPlan under `docs/codex_exec_plans.md`. It answers why the
predecessor's revision-31 release audit never observed its required completed
component-temperature sweep, which real runner workloads exercise that
opportunity, and what the historical `rss_kib` samples actually measure. It
also decides whether either rejected mechanism merits a separately authorized
successor. This package is diagnostic characterization and measurement-harness
correction only; it does not restore either rejected optimization or qualify
throughput.

Status: `EXECUTED — TERMINAL HOLD / V31 DISCRIMINATOR CLOSED; V61 HISTORICAL EVIDENCE LIMITATION`

Execution mode: `package-end-to-end`

Implementation intent: `diagnostic characterization + bounded coverage/memory measurement-harness correction`

Review checkpoint: predecessor commit `0d56001edbbe55b119a5248249fcedefe2a3bbb0`.
The checkout is not reset to that commit; the actual HEAD and dirty state are
recorded in `artifacts/baseline-and-recovery-manifest.md`.

## Purpose and decision boundary

The predecessor is terminal HOLD because its one-OFE release workload failed
the performance/RSS qualification and revision 31 failed before JSON emission.
This package must replace speculation with authenticated observation. A valid
result may reject both salvage paths. No result may weaken the old
`58/14/16/28` assertion, the 64 MiB ceiling, or the predecessor's production
correctness rules.

## Questions and acceptance

The package must answer, with separate historical, current-baseline, and
diagnostic-reconstruction labels:

1. The earliest real predicate that prevented an authentic revision-31
   `N=2,S=6` completed `58/14/16/28` sweep, with source/runtime cause.
2. The authentic runner workloads, topology, stencil, lifecycle, and frequency
   that exercise or fail to exercise component replay.
3. The exact meaning and sampling lifetime of `rss_kib`, plus supported and
   unresolved causes of revision-61 versus baseline memory differences.
4. Whether either mechanism is worth a new authorized increment, with the
   evidence and gate that increment would require.

Acceptance requires direct executed evidence, independent stream/result
reconstruction, explicit uncertainty, closed in-scope harness corrections,
two independent reviews, two independent terminal verifications, and a
truthful disposition. A HOLD is valid only for evidence that cannot be produced
inside this package after the declared alternatives are exhausted.

## Correction authority envelope

Allowed edits are observation-only instrumentation on actual paths, new
diagnostic entry points, authenticated fixture construction using existing
constructors/validators, bounded parser/lifecycle/aggregation/measurement
harness corrections, offline result reconstruction, and isolated diagnostic
recovery of the already-reviewed candidate mechanism.

Forbidden edits include equations, physical tolerances, stopping criteria,
Jacobian arithmetic, solver selection, cadence, event handling, physics,
receipt/restart/transfer/publication semantics, fallback solvers, approximation,
cross-evaluation caching, production replay activation, and changes to the
predecessor's acceptance rules. Any source patch must be test-only or
diagnostic-only and remain unreachable from retained production builds.

## Exact initial write set

Phase-0 edits are limited to this package's `package.md`, active/archived
prompts, package catalog/roadmap entry, and the evidence artifacts listed
below. No Rust source path is authorized until Phase 0 resolves its exact
caller/consumer location, records a content-addressed pre-edit snapshot, runs
`tools/agents/find-agents --for` on the path, and amends this write set before
the first source edit. Later source paths must be listed explicitly, never by
directory wildcard.

Required package artifacts:

- `artifacts/required-reading-map.md`
- `artifacts/baseline-and-recovery-manifest.md`
- `artifacts/experiment-protocol.md`
- `artifacts/source-and-consumer-map.md`
- `artifacts/coverage-findings.md`
- `artifacts/coverage-matrix.json`
- `artifacts/memory-attribution.md`
- `artifacts/memory-results.jsonl`
- `artifacts/science-and-output-parity.md`
- `artifacts/gate-results.md`
- `artifacts/finding-disposition.md`
- `artifacts/review-a.md`, `review-b.md`, `verification-a.md`,
  `verification-b.md`, and `final-disposition.md`
- `artifacts/raw/` logs, schemas, scripts, hashes, and clearly labeled
  nonproduction candidate patches as required by executed evidence.

The unrelated predecessor `tmp/pdfs/r156-review/` files remain out of scope.

## Subagent authorization and ownership

This package explicitly authorizes spawning/delegating to two independent
reviewer agents, two independent verifier agents, and the
`comparator_suite_runner` role for the scope below. Reviewers and verifiers are
read-only except for their assigned package artifact; the comparator may run
the frozen commands and write only package logs/results. The parent must not
run heavy release, parity, population, or full-workspace gates when the
comparator role is available. Record any spawn/tool failure before using an
authorized local fallback.

## Phase plan

### Phase 0 — scaffold, preserve, and freeze

Create the package and evidence skeleton, record HEAD/branch/dirty paths,
toolchain/host/CPU/affinity/build/runtime settings, required-reading bytes,
predecessor identities, raw logs, and recoverability. Resolve exact source
anchors and prospective write paths before any Rust edit.

### Phase 1 — protocol and controls

Freeze the original workload, mechanism-coverage, and memory-attribution
surfaces. Declare warmups, balanced paired runs, timeouts, invalid-run rules,
trace limits, output hashes, and analysis rules. Build outside measurement
windows and prove executable/runtime provenance.

### Phase 2 — authentic revision-31 coverage

Trace actual runner-to-solver lifecycle positions, occupancy/soil topology,
signed stencils, eligibility, completion/error outcomes, and diagnostic joins.
Persist bounded summaries before assertions and independently reconstruct the
stream. If the original workload lacks the opportunity, retain that negative
result and run a separately named lawful fixture through the real runner.

### Phase 3 — revision-61 memory attribution

Identify the exact PID and `/proc` source for `rss_kib`; preserve the old
`VmRSS` endpoint and add named pre/post/audit-drop/cleanup observations. Use
matched baseline/candidate controls, balanced fresh-process repetitions, and
bounded allocation/mapping evidence. Distinguish current RSS, high-water mark,
heap bytes, allocator retention, mappings, stacks, and unexplained residuals.

### Phase 4 — correction and validation

Run malformed-field, wrong-PID, truncation, contamination, incomplete-sweep,
boundary-stencil, duplicate-ID, output-identity, and cleanup-order tests. Run
affected contract/anti-evasion, focused LSE, orchestrator, runner,
observation-on/off parity, output/closure, formatting, scoped Clippy, and
line-count gates selected from the exact final diff. Preserve predecessor
expected-red seams and remove all rejected candidate activation.

### Phase 5 — review, verification, and disposition

Obtain two independent reviews and two independent terminal verifications on
immutable evidence cuts. Disposition every finding and choose for each
mechanism: worth a successor, authentic but immaterial, absent from original
but exercised by lawful fixture, harness defect, or insufficient evidence with
the precise missing discriminator. Update catalog/roadmap without changing the
predecessor HOLD.

## Required heavy validation

When selected by impact and exact diff, the comparator role must execute the
frozen release/parity/batch commands. The parent may run only bounded quick
checks unless the role is unavailable and the package records that failure.
Any critical source, test/fixture, or runner change escalates to the applicable
full correctness profile; no planner/TESTGATE is used.

## Progress

- [x] 2026-09-04: user-authorized package request received; predecessor remains
  terminal HOLD and current checkout is preserved.
- [x] Scaffold package/catalog/roadmap and freeze intake identities.
- [x] Freeze protocol and resolve exact diagnostic source paths.
- [x] Execute authentic revision-31 coverage tracing and causal predicate audit.
- [x] Execute revision-61 memory attribution controls and analysis.
- [x] Apply bounded harness corrections, validate, review, and verify; the
  historical v31 debugger route is now directly captured, while v61 remains
  non-equivalent.
- [x] Complete salvage disposition and record terminal HOLD: the v31 runtime
  discriminator is closed, the v61 historical-custody boundary is precise
  after automatic/manual/reverse recovery routes, and no successor is
  authorized pending any newly supplied historical source/binary custody.

## Surprises and discoveries

Populate this section with short evidence-backed observations as phases run.

- The authentic persisted seed is N=2/S=6 with exact beta=1.0 and 273.15 K
  boundary values; the earlier S=5 interpretation was disproved by live-frame
  rebinding and raw seed custody.
- The current retained runner has no component-replay implementation or
  published replay counter. Carrier finalization timing fields are unrelated
  telemetry and were removed from the replay inference.
- The cached historical v31 binary's focused real-runner replay/parity test
  passes, while the same binary's authentic one-OFE release profile reproduces
  the `58/14/16/28` assertion panic. A debugger-only audit-return capture then
  decoded the authentic release rows before aggregation, closing the runtime
  discriminator without bypassing the assertion or mutating the binary.
- The R1 observer control is internally complete, but observation adds a
  median 577,643.5 us of paired wall time; it is not a candidate treatment.
- Ordered v61 session recovery was attempted in isolation (22/38 source
  patches applied, 16 failed; scoped cargo check exit 101) without producing
  an equivalent executable.
- Exact historical v31 GDB capture now observes all 2,000 authentic audit
  sweeps before aggregation: 200 rows (iteration 0/1 for maps 0..99) are
  `48/14/10/24`, while the remaining 1,800 are `54/14/16/24` (600 rows from
  iterations 0/1 for the other 300 maps and 1,200 rows from iterations 2..4).
  All are completed and not failed. The missing target is therefore a
  boundary-aware signed-stencil accounting shortfall, not collector or
  lifecycle loss; the two displayed vectors are representative captures, not
  the sole basis for the aggregate cross-tab.
- Manual reconciliation of the v61 failed contexts produced a compiling,
  real-consumer detached run with 200 calls and exact closure. Its documented
  changed-Rust manifest is `19a07121…23421` (an earlier undocumented
  computation recorded `2fc5e8ca…e6eb7f`), while the historical `650f6713…57d41`
  digest has no per-file manifest, Git-index bytes, or candidate binary for
  comparison; it is feasibility evidence only, not causal treatment evidence.
- Reverse application of the originally successful cleanup payloads was also
  attempted from a clean exact-HEAD clone. Twelve inversions applied, one
  formatting-context inversion failed, and the resulting tree failed scoped
  Cargo checks with ten concrete errors because the payloads omit placement
  context and some candidate-side request edits. This closes the remaining
  reversible-source route without treating a malformed tree as historical.

## Decision log

Populate decisions with date, author, rationale, and affected evidence identity.

- 2026-09-04, parent: admit only the R1 memory-control set after readiness was
  persisted and field gaps failed closed; preserve earlier partial attempts.
- 2026-09-04, parent: classify replay as static-not-applicable in the retained
  implementation; do not infer it from carrier finalization timing.
- 2026-09-04, parent: run ordered v61 session-patch recovery in a detached
  worktree after reviewer challenge; no candidate bytes entered retained HEAD.
- 2026-09-04, parent: debugger-only capture at the exact historical v31 audit
  return closed the release runtime discriminator without bypassing the
  assertion or mutating the binary; preserve the distinction between v31
  coverage cause and v61 memory attribution.
- 2026-09-05, parent: retain terminal HOLD because v61 has no authenticated
  equivalent candidate/control treatment after automatic, manual, and reverse-
  cleanup recovery routes; fresh dual reviews/verifications approve the
  evidence boundary, and no successor is authorized.

## Outcomes and retrospective

Complete only at terminal disposition; compare each question with its direct
evidence, unresolved discriminator, and next-step recommendation.

Current outcomes are recorded in `artifacts/final-disposition.md`. The package
answers the v31 coverage question with direct debugger evidence, answers the
retained-workload and `rss_kib` questions with authenticated controls, and
preserves the predecessor terminal HOLD. Revision-61 causal attribution
remains open because no exact candidate source/binary cut or matched treatment
was recovered. No production activation or science-contract change remains in
the retained checkout; no successor is authorized.
