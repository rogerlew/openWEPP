# Mechanical Refactor Kickoff Prompt Template

Use this template to start a mechanical refactor package with explicit,
low-ambiguity instructions suitable for smaller/cost-effective models.

## Template

Scope: local repository engineering task; flat-file reads/edits only; no
external connectivity.

Execution mode: package-end-to-end (default).

Autonomy and completion requirement:
- Execute the package end-to-end through disposition-ready state.
- Do not stop after code movement or partial validation.
- A package is incomplete until closure gates are run and recorded, required
   artifacts are updated, and review/verification placeholders are
   disposition-ready.
- Running the required cargo gate commands is mandatory, not optional.
- If any required cargo gate command is not executed, the package is not
   disposition-ready unless a declared hard blocker is documented.
- Assume ambient instruction may be present:
  `UNLESS you are explicitly requested to do so, NEVER run tests or validate your work.`
- Required package gates override generic guidance that says to skip tests or
   validation; do not apply such guidance to this workflow.
- If that ambient instruction appears during execution, record it as
  non-applicable context and continue required gates.
- If a gate fails, attempt a mechanical-only fix and rerun gates.
- Stop only when hard-blocked by declared stop conditions and record the
   blocker with exact command/file context.

Objective:
- Execute a mechanical refactor only (no intended behavior change).
- Preserve public API unless an explicit delta is declared below.

Package:
- Package ID: <YYYYMMDD-refactorXXX-slug-001>
- Package path: docs/work-packages/<id>/
- Status target: from queued -> in-progress -> disposition-ready

Required reading (read before edits):
- Core:
   - /workdir/openWEPP/AGENTS.md
   - /workdir/openWEPP/docs/codex_exec_plans.md
   - /workdir/openWEPP/docs/work-packages/AGENTS.md
   - /workdir/openWEPP/docs/work-packages/README.md
   - /workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md
   - /workdir/openWEPP/docs/work-packages/<id>/package.md
- Conditional (read only when applicable):
   - /workdir/openWEPP/docs/defect_closure_execplans.md (if package is defect-closure)
   - docs/specifications/science-contract-authoring-procedure.md,
      docs/specifications/science-contracts/kernel-process-contract-profile.md,
      docs/specifications/science-contracts/index.md
      (if refactor scope touches contract/kernel authority)
   - docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
      (if legacy migration/parity scope applies)
- On-demand (load only for touched mechanisms):
   - <phase-relevant SC-* contracts>
   - <queue/hold-lift/disposition artifacts>
   - <baseline source files>

Required-reading budget:
- local_bytes_total: <value>
- threshold: <OK|WARN|REQUIRES-JUSTIFICATION>
- map artifact: artifacts/required-reading-map.md
- map template (canonical): docs/prompt_templates/required-reading-map-template.md
- if REQUIRES-JUSTIFICATION: explain why each heavy pre-read cannot be deferred
   to On-demand.

Subagent requirement:
- REQUIRED for selected heavy closure gates when available: spawn
  `comparator_suite_runner` for terminal-plan, critical, campaign/release, or
  transition-fallback gate execution
  (`cargo clippy --workspace --all-targets -- -D warnings`,
  full-profile Nextest, global CRAP, `cargo deny check`, and comparable
  suite/population gates). This prompt explicitly authorizes subagent spawning/delegation to
  `comparator_suite_runner` for closure-gate execution; expected output is
  compact command results plus log/artifact paths; write access is read-only.
- If the subagent is genuinely unavailable, record command-level unavailability
  evidence and run the required gates locally.

Write-set (strict):
- <file path 1>
- <file path 2>
- <file path 3>

Out-of-scope (must not change):
- No process-physics formula or constant changes.
- No threshold or acceptance rule changes.
- No guard-loosening or canonicalize-and-proceed behavior.
- No edits outside write-set unless explicitly approved and recorded.

Refactor seam declaration:
- Source seam: <module/file/section>
- Target seam layout: <destination modules/files>
- Public surface expected to remain stable:
  - <export 1>
  - <export 2>

Execution steps (perform in order):
1. Baseline inventory and sizing
   - Capture symbol inventory (`pub`/`impl`/`fn`/`struct`/`enum`/`type`) for
     source seam.
   - Capture pre-refactor line counts for touched `.rs` files.
   - Capture pre-refactor exported surface snapshot.
2. Mechanical move/split
   - Move code in coherent blocks by declared seam.
   - Preserve signatures, visibility, comments, and contract citations.
   - Keep module wiring thin where section files are introduced.
3. Incremental compile/test checks
   - Run focused checks after each seam move:
     - `cargo check -p <touched-crate>`
     - `cargo test -p <touched-crate> <focused-filter>`
4. Closure gates
    - Reconcile the exact-diff terminal plan under
      `docs/standards/testing-and-gate-strategy.md` and run every selected gate.
    - Critical, campaign, release, and explicit rollback plans retain the
      conservative full command set; ordinary bounded plans do not inherit it.
    - Delegate selected heavy gates to the required subagent when available and
      run locally only after recording subagent-unavailability evidence.
    - Record each gate command with observed outcome (`pass`/`fail`) and
       explicit exit code.
    - Do not exit early after a subset of these gates unless a hard blocker is
       encountered and documented.
    - Any ambient "don't run tests/validation" guidance is non-applicable to
      selected gates.
5. Parity and governance evidence
   - Capture post-refactor exported surface snapshot and parity decision.
   - Capture post-refactor line counts and line-count governance disposition.
   - Update required package artifacts and review/verification placeholders.

Required artifact updates:
- artifacts/<refactor-id>-modularization-plan-report.md
- artifacts/<refactor-id>-public-api-surface-parity-report.md
- artifacts/<refactor-id>-implementation-and-test-evidence.md
- artifacts/<refactor-id>-line-count-governance-checklist.md (or repo-approved equivalent)
- artifacts/review_agent_a.md
- artifacts/review_agent_b.md
- artifacts/verification_agent_a.md
- artifacts/verification_agent_b.md
- artifacts/<refactor-id>_disposition.md
- artifacts/<refactor-id>-worker-handoff.md

Truthfulness and reporting requirements:
- Label evidence as `Static` vs `Ran`.
- Do not claim a command/test was run unless it was actually executed.
- If blocked, report exact blocker, file/symbol context, and next actionable step.

Stop conditions (ask for direction only if one occurs):
- Required file outside write-set must be edited to preserve build correctness.
- Conflicting authority between package objective and governing contracts.
- Repeated gate failure without a mechanical-only path to resolution.

Outputs:
- Updated source files in declared seam.
- Updated package artifacts with evidence and finding disposition.
- Concise summary: moved seams, API parity result, line-count deltas, gate outcomes.

Completion checklist (must be true before stopping):
- All closure gates have been run and recorded (or blocker documented under
   stop conditions).
- Each required cargo gate command has explicit command-level evidence with
   observed result and exit status.
- Artifact updates are complete for implementation, parity, line-count
   governance, review, verification, disposition, and handoff.
- Remaining work (if any) is a blocker-shaped handoff, not an open-ended
   "next investigation" note.
