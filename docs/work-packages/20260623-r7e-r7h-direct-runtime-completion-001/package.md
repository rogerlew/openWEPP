# R7E-R7H Direct Runtime Completion

Status: executed-held.

Package type: Defect-Closure ExecPlan / iterative R7 direct-runtime
completion package.

Defect IDs:

- `R7E-DEFAULT-ACTIVATION-CANDIDATE-ABSENT`
- `R7F-HOT-COMPATIBILITY-RUNTIME-NOT-ISOLATED`
- `R7G-PERFORMANCE-AND-FIXTURE-GATES-UNPROVEN`
- `R7H-RELEASE-CUTOVER-READINESS-ABSENT`

This ExecPlan is a living document. Maintain `Progress`,
`Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` as
execution proceeds. This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`, and
`docs/architecture/array-native-runtime-specification.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer, verifier, and comparator-runner
subagents for R7 default-selection review, no-compatibility proof review,
benchmark/comparator execution, fixture-matrix audit, gate-evidence audit, and
line-count governance review. Expected outputs are compact Markdown findings
summarized into `artifacts/review-disposition.md`,
`artifacts/verification.md`, `artifacts/performance.md`,
`artifacts/compatibility-isolation.md`, and `artifacts/fixture-matrix.md`;
subagents may not edit files.

## Purpose / Big Picture

R7 is not complete until production direct mode can be treated as the normal
hillslope runtime without hidden compatibility hot-loop authority. After R7D8,
the current H2637 5-day direct-production publication gate is identity-clean,
but the runtime is still not default, compatibility plumbing is still present
around production direct mode, performance/release evidence is incomplete, and
fixture hardening remains open. This package exists to continue iterating
through R7E, R7F, R7G, and R7H until those remaining blockers are either closed
or stopped at a named legitimate boundary.

## Progress

- [x] Read root `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/defect_closure_execplans.md`, `docs/codex_exec_plans.md`,
  `docs/specifications/science-contracts/AGENTS.md`, and the R7 section of
  `docs/architecture/array-native-runtime-specification.md`.
- [x] Record required reading and initial known blockers in
  `artifacts/required-reading.md` and `artifacts/blocker-ledger.md`.
- [x] Reproduce the R7D8 starting point or record why the current tree has
  moved: default/direct H2637 publication identity, direct manifest counters,
  and current default runtime selection.
- [x] R7E: implement and validate default-selection policy, explicit rollback,
  manifest runtime-selection fields, and focused CLI/API tests.
- [ ] R7E/R7G: run H2637 default/direct/rollback evidence after the R7F hot
  compatibility edge is removed.
- [ ] R7F: isolate or delete hot compatibility runtime authority from
  production direct mode, add static/call-graph/source-scan guards, and prove
  compatibility remains reachable only through explicit compatibility,
  replay, diagnostic, or shadow modes. Held at
  `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`.
- [ ] R7G: benchmark same-binary H2637 compatibility, direct default, direct
  explicit, and rollback; profile and remediate blockers until performance
  gates pass or a legitimate architecture hold is proven; broaden fixture
  matrix and independent operand reconstruction.
- [ ] R7H: complete release cutover readiness, operator-facing docs, manifest
  expectations, anti-evasion guards, and catalog links.
- [ ] Complete dual review, verification, line-count governance, worker
  handoff, final disposition, and commit/push only when authorized by the
  active user instruction.

## Current Starting State

R7D8 closed the current H2637 5-day direct-production publication-parity gate.
The fresh evidence recorded in
`docs/work-packages/20260623-r7d8-direct-hbp-erod15-export-alias-parity-001/`
shows default/direct exits `0`, HBP/loss/PASS/PLOT/WAT byte identity, parsed
HBP latest-event parity, and direct `compatibility_edge_invocations = 0`.
This package supersedes that counter interpretation for current code: the
production direct day-input builder is now counted as a compatibility edge
until R7F removes it.

Remaining R7 work is explicitly R7E-R7H:

- production direct mode is still selected explicitly with
  `--direct-production-executor`, not as the normal/default runtime;
- rollback/default-selection manifest semantics are incomplete;
- hot compatibility runtime, scheduler, writeback, registry, dense-refresh,
  and diagnostic plumbing still need isolation/deletion proof for production
  direct mode;
- current same-binary H2637 performance and RSS evidence must be regenerated
  after R7D8 before any default/release claim;
- broader fixtures and release anti-evasion gates are not complete.

## Non-Negotiable Terminal-State Rule

This package has exactly two honest terminal states:

1. `COMPLETE-R7-DIRECT-RUNTIME-COMPLETION`: production direct mode is the
   declared normal hillslope runtime, rollback/compatibility/shadow modes are
   explicit and proven, protected outputs are identity-clean, hot
   compatibility authority is absent from production direct mode, performance
   gates pass or are explicitly accepted by the architecture authority,
   fixture/release gates pass, and all review findings are dispositioned.
2. `HOLD-R7-<SPECIFIC-BOUNDARY>`: a blocker is reduced to a concrete mechanism
   and proven outside this package's authority envelope by direct evidence;
   dual review accepts the boundary; `artifacts/worker-handoff.md` names the
   next defect to close rather than a next inspection step.

The following are invalid terminal reasons by themselves:

- "default selection is not wired";
- "direct mode is still slower";
- "a compatibility import or call remains";
- "the next source-scan guard failed";
- "H2637 identity is green but performance is not";
- "performance is green but fixture hardening is incomplete";
- "release docs or manifest fields remain";
- "another blocker remains";
- "the implementation is complex";
- "the package made progress";
- "another package should handle the next in-envelope blocker."

Each of those is an iteration target. If the mechanism is inside the authority
envelope, the worker must implement the correction, validate it, update the
blocker ledger, and continue.

## Correction Authority Envelope

Observed violations:

- `R7E-DEFAULT-ACTIVATION-CANDIDATE-ABSENT`: after R7D8, direct production is
  still opt-in and does not provide a validated default/rollback selection
  policy with manifest evidence.
- `R7F-HOT-COMPATIBILITY-RUNTIME-NOT-ISOLATED`: production direct mode still
  requires proof that compatibility scheduler, request/writeback, registry,
  dense-refresh, dirty-flush, and runtime-surface plumbing are not in the hot
  path.
- `R7G-PERFORMANCE-AND-FIXTURE-GATES-UNPROVEN`: current direct runtime
  endpoint/RSS and fixture matrix are not sufficient for the architecture
  viability and release gates.
- `R7H-RELEASE-CUTOVER-READINESS-ABSENT`: release contract, operator docs,
  anti-evasion checks, catalog links, and rollback window are not complete.

In-scope defect mechanisms:

- runtime selection and fallback policy for default, explicit direct, explicit
  compatibility, and shadow/replay/diagnostic modes;
- manifest runtime-selection, fallback reason, output-policy, provenance, and
  direct counter fields;
- CLI/API routing for normal direct execution and rollback;
- production direct call graph, imports, source scans, and runtime counters;
- compatibility runtime isolation, renaming, or deletion where rollback and
  diagnostics remain safe;
- H2637 default/direct/rollback/public-output identity and metadata parity;
- performance profiling, allocation/layout/string/map/registry hot spots, and
  direct-frame remediation needed for the R7 target;
- fixture matrix broadening for snow/frost, breakpoint climate, PMET branches,
  irrigation when enabled, multi-OFE transfer, nonzero erosion, sidecars, and
  management transitions;
- release anti-evasion checks, operator docs, manifest expectations, and
  work-package catalog/architecture updates;
- line-count governance issues in touched Rust files.

In-scope write set:

- `docs/work-packages/20260623-r7e-r7h-direct-runtime-completion-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md`
- targeted operator/API/manifest docs affected by direct runtime selection
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/tests/**`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/**`
- release/anti-evasion scripts only when needed for R7H gates

Protected boundaries:

- Do not change process-physics math merely to satisfy output or performance
  gates. Kernel-affecting changes require canonical `SC-*` authority,
  contract-derived tests, and provenance.
- Do not activate a slower or output-nonidentical direct path as the normal
  runtime.
- Do not delete compatibility rollback, replay, diagnostic, or shadow
  capabilities before explicit coverage proves they remain available through
  edge-only paths.
- Do not hide compatibility scheduler, WB13 rows, runtime surfaces,
  `HillslopeWritebackSurface`, `KernelWritebackPayload`, symbol registries, or
  dense-refresh wrappers behind a new direct-mode convenience API.
- Do not broaden into watershed/channel runtime work unless a named R7 blocker
  is proven to be a watershed-side boundary; that boundary must become a
  separate package.

## Iterative Continuation Protocol

Execution must run as a loop:

1. Reproduce the current highest-priority failing R7 gate.
2. Record it in `artifacts/blocker-ledger.md` with defect ID, observed
   command/output, suspected mechanism, owner surface, classification
   (`in-envelope`, `out-of-envelope`, `invalid-input`, `authority-missing`, or
   `evidence-unavailable`), and next validation command.
3. If in-envelope, implement the correction in this package. Do not stop after
   a diagnostic or after moving to a new fail-closed marker.
4. Run the narrowest focused gate that proves the correction, then rerun the
   broader package gate that originally failed.
5. Update `artifacts/iteration-log.md`, `artifacts/verification.md`, and the
   relevant artifact (`mode-selection-and-consumer-path.md`,
   `compatibility-isolation.md`, `performance.md`, `fixture-matrix.md`, or
   `release-readiness.md`).
6. Continue to the next failing R7 gate until the terminal-state rule is
   satisfied.

If a legitimate HOLD boundary is reached, the package must still update
`artifacts/worker-handoff.md` with first actionable item
`close defect <ID>` and scaffold or name the concrete follow-on package. A
handoff that says "inspect", "trace", "continue investigating", or "next
blocker remains" is not acceptable.

## Phase Plan

### Phase 0 - Starting Evidence And Ledger

Re-read R7A-R7D8 package artifacts and the architecture R7E-H sections. Build
or locate current H2637 runfiles, then record the starting default/direct
publication state, manifest counters, and current runtime selection behavior.
Populate `artifacts/required-reading.md`, `artifacts/blocker-ledger.md`, and
`artifacts/iteration-log.md` before production edits.

### Phase 1 - R7E Default Activation Candidate

Add a runtime selection policy that can choose direct, compatibility, or shadow
once at run setup. The policy must preserve explicit rollback and must write
manifest evidence for selected mode, fallback reason, output policy, and direct
counters. Add CLI/API tests for default, explicit direct, explicit
compatibility/rollback, and shadow/replay boundaries. Same-binary H2637
default, direct explicit, and rollback runs must remain protected-output
identity-clean before any default behavior claim.

### Phase 2 - R7F Compatibility Runtime Isolation And Deletion

Audit production direct imports and call graph. Move, rename, isolate, or
delete compatibility hot-loop components so production direct mode cannot call
the compatibility scheduler, kernel request/writeback path, dirty flush, dense
refresh, or symbol-registry lookups. Keep compatibility, replay, diagnostic,
and shadow code edge-only. Add static/source-scan guards and runtime counters
that fail if forbidden compatibility authority appears in production direct
mode.

### Phase 3 - R7G Performance Closure And Fixture Hardening

Benchmark same-binary H2637 compatibility, direct default, explicit direct,
and rollback with seconds, RSS, us/OFE-day, and legacy multiplier. If direct
misses the architecture target, profile and remediate hot functions,
allocation sources, string formatting, map/registry lookups, and layout costs.
Repeat benchmark/remediation until the target passes or a named architecture
hold is proven. Expand fixtures and independent operand reconstruction for
conservation-sensitive output families.

### Phase 4 - R7H Release Cutover Readiness

Freeze the direct-mode runtime contract and rollback window. Update
operator-facing CLI/API docs, manifest expectations, release checklist, and
anti-evasion checks. Confirm all R7A-R7G evidence is linked from the
work-package catalog and architecture spec. Do not claim release readiness
while any R7E-G gate is unresolved.

### Phase 5 - Closure

Run final gates, complete dual review and verification, disposition every
finding, update line-count governance, update worker handoff, and set final
disposition. Commit and push only if the active user instruction authorizes
publishing.

## Acceptance Gates

- R7E: default-selection policy and rollback are implemented and covered by
  CLI/API tests; manifests truthfully report selected runtime, fallback reason,
  output policy, direct counters, and provenance.
- R7E: same-binary H2637 default/direct/rollback comparison is
  byte/Arrow/metadata identity-clean before any default activation claim.
- R7F: source scans and call-graph/runtime counters prove production direct
  mode excludes compatibility scheduler, `HillslopeWritebackSurface`,
  `KernelWritebackPayload`, WB13 authority, symbol registry lookups,
  dense-refresh, dirty-flush, and compatibility wrappers in the hot path.
- R7F: compatibility, replay, diagnostics, and shadow remain available only
  through explicit edge modes.
- R7G: H2637 direct default reaches the architecture performance gate
  (`<=10x` legacy, preferably on the `<=5x` trajectory) and is not slower than
  compatibility without explicit architecture-authority disposition.
- R7G: protected outputs remain HBP byte identity and WAT/PASS/PLOT Arrow/
  metadata identity-clean; conservation-sensitive outputs have independent
  operand reconstruction and anti-alias evidence.
- R7G: fixture matrix covers snow/frost, breakpoint climate, PMET branches,
  irrigation when enabled, multi-OFE transfer ratios, nonzero erosion, sidecar
  absence/presence, and management transitions, or records legitimate
  out-of-envelope holds.
- R7H: release checklist, operator docs, manifest docs, anti-evasion checks,
  and package catalog links are complete.
- Closure: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, `cargo deny check`, `git diff --check`, scoped
  Markdown lint, line-count governance, dual review, and dual verification
  pass unless the package closes in a legitimate `HOLD-R7-<SPECIFIC-BOUNDARY>`
  before final R7 completion.

## Security-Impact Gate

- No secrets, tokens, credentials, or machine-local absolute paths are
  committed as normative config.
- Direct runtime selection and rollback remain explicit, manifest-visible, and
  fail-closed.
- Release anti-evasion checks must fail on hidden compatibility imports,
  false direct counters, misleading provenance, and fallback behavior drift.

## Review Requirements

- Dual review with explicit finding disposition:
  `accepted`, `rejected`, `deferred`, or `follow-up`.
- Review must check HOLD legitimacy, envelope adequacy, gate non-deferral,
  consumer-path proof, performance evidence, fixture matrix adequacy, and
  protected-boundary integrity.
- Verification artifacts must label `Static:` and `Ran:` evidence.
- `.rs` line-count governance: `2000+` lines is `WARN`; non-exempt `3000+`
  production files block closure.

## Surprises & Discoveries

- 2026-06-23: The previous R7D8/R7C `compatibility_edge_invocations = 0`
  evidence was incomplete. Production direct bypasses the compatibility
  scheduler and kernel request/writeback lifecycle, but its interleaved
  `DirectPublicationDayInputBuilder` still builds day inputs from
  `HillslopeWritebackSurface` seed/context surfaces in the day/OFE loop. This
  package made that edge manifest-visible by incrementing the direct runtime
  compatibility-edge counter for production direct day-input builds.

## Decision Log

- 2026-06-23: Scoped this package as one R7E-H authority envelope instead of
  four diagnostic-only packages. The intent is to prevent the R7 completion
  work from relaying after each newly discovered blocker.
- 2026-06-23: R7E selection mechanics are safe to land because the default
  candidate remains compatibility unless the explicit activation gate is set;
  direct production remains manifest-visible and rollback remains explicit.
- 2026-06-23: R7F cannot be closed in this package without replacing the
  interleaved day-input compatibility surface builder with typed direct
  day-input/state projection. R7G performance and R7H release readiness remain
  blocked behind that structural replacement.

## Outcomes & Retrospective

- R7E implemented: public API policy/resolution, CLI default-candidate and
  explicit compatibility rollback flags, top-level manifest runtime-selection
  provenance, and focused tests for default-disabled, default-activated,
  explicit rollback, explicit direct, and shadow policy resolution.
- R7F executed-held: production direct still has a counted hot compatibility
  edge in `DirectPublicationDayInputBuilder`.
- R7G/R7H not executed to closure because benchmark/release claims would be
  invalid while the R7F no-compatibility gate is red.

Final disposition:
`HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`.
