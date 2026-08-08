# Snow Stage 3 Persistent Accumulation Shadow

Status: `queued / authorized for end-to-end execution`

Date: `2026-08-07`

Package ID: `20260807-snow-stage3-persistent-accumulation-shadow-001`

Plan class: `Critical kernel evaluation implementation`

## Purpose

Implement a typed, default-off, evaluation-only Stage 3 operator whose private
snow state persists across simulation days. Prove snowfall, liquid, thin-pack
disappearance/reappearance, restart equivalence, and linked mass/energy custody
without mutating CoE production state or making physical-validation, promotion,
publication, or cutover claims.

## Implementation Intent

Intent is `science implementation for an internal evaluation operator`. Wind
exposure, canopy applicability, stability geometry, and physical-magnitude
uncertainties constrain interpretation; they do not block this isolated state-
continuity experiment. Measured SWE remains `DIAGNOSTIC_ONLY`. This package is
not empirical calibration or independent physical validation.

## Included Scope

- Amend canonical snow contracts before code to authorize one named persistent
  evaluation operator and its state/custody rules.
- Add contract-derived tests and record the pre-implementation contract gate.
- Add a typed `persistent_accumulation_shadow_v1` request, absent by default.
- Keep one private per-lane shadow state across ordered days and water years.
- Ingest hourly snowfall and liquid into the shadow with explicit ledgers.
- Preserve/recreate state across thin-pack disappearance and later snowfall.
- Provide deterministic snapshot/restore and prove uninterrupted-versus-restart
  equivalence without treating process restart as production ownership.
- Emit internal trace fields sufficient for a real independent consumer to
  reconstruct mass, energy, state continuity, support, and censoring.
- Prove disabled-path production state and WAT/HBP/PASS outputs unchanged.

## Excluded Scope And Claim Limits

- No change to CoE production melt, snow mass, defaults, public schema, output,
  calibration, promotion, assurance approval, or cutover.
- No wind, canopy, geometry, turbulent, receiving-surface, soil-heat, terminal
  land-surface, or other process-physics correction.
- No claim that current turbulent magnitudes or persistent results are
  physically valid. Unresolved exposure/geometry/magnitude authority is carried
  as an explicit interpretation limit.
- Energy remaining after complete snow disappearance and liquid lacking a
  resolved recipient are recorded and censored, never silently routed.

## Dependencies And Authority

- `SC-SNOWFREEZE-001` and `SC-SNOWENERGY-001`.
- Stage 3 evaluation-shadow authority and observability packages.
- Evolving-state plausibility and Paradise WY2015 support-resolution packages.
- ADR-0042 science/data/calibration separation.
- Existing Stage 3 carrier equations only; no new surrogate physics.

## Intended Write Set

- This package tree, work-package catalog, and snow campaign roadmaps.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`.
- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` only if
  a linked energy-custody clarification is required.
- `docs/specifications/science-contracts/index.md` and focused contract tests.
- Stage 3 typed state/evaluator modules in `openwepp-hillslope-orchestrator`.
- Direct runner builder, snow authority adapter, internal JSONL trace consumer,
  and focused tests in `openwepp-runner`.
- Ignored immutable execution evidence under
  `target/snow_stage3_persistent_accumulation_shadow/`.

Any expansion requires prospective amendment and review.

## Contract-First Phase Plan

1. Scaffold, freeze intent/write set, required reading, operand lineage, and
   validation selection.
2. Amend contracts, implement contract-derived tests, and record a passing
   pre-implementation contract gate.
3. Implement typed private persistent state, snowfall/liquid custody,
   disappearance/reappearance, and snapshot/restore.
4. Connect the direct runner and internal trace real consumer while preserving
   exact disabled production behavior.
5. Run focused, quick, frost, and critical full-workspace correctness gates;
   produce deterministic synthetic and retained-fixture evidence.
6. Reconcile the exact diff, line counts, assurance impact, dual review,
   finding disposition, dual verification, roadmap/catalog, and disposition.

## Conservation / Output Acceptance

Before production edits, record operand lineage with units, normalization,
state basis, source authority, and diagnostic status. Tests must separate and
reject authoritative production state, raw vapor opportunity, bounded vapor
transfer, snowfall, rain/liquid, melt, terminal unallocated energy, and carried
shadow state aliases. Acceptance requires independent reconstruction from the
emitted internal trace and exact mass/energy closure; self-consistency alone is
insufficient.

## Validation And Exit Criteria

- Contract schema/profile and focused authority tests pass before code edits.
- Disabled/absent request allocates no persistent state, emits no new payload,
  and preserves production state/arithmetic/outputs/defaults exactly.
- Enabled state is lane-isolated, ordered, non-aliasing, and inaccessible to
  production consumers.
- Synthetic multi-day tests prove snowfall, liquid accounting, sublimation,
  melt, thin-pack disappearance, snow-free dormancy, reappearance, and exact
  snapshot/restore equivalence.
- Independent trace reconstruction proves start + inputs - outputs = end for
  mass and the contract-defined energy ledger within named tolerances.
- Unresolved terminal and receiving-surface quantities are explicit and
  excluded from physical efficacy claims.
- Applicable Clippy, focused, quick, frost, full workspace, Markdown, link,
  diff-hygiene, assurance, and line-count requirements pass.
- Two independent reviews, finding disposition, and two independent terminal
  verifications leave no current-scope requirement or finding unresolved.

## Protected Invariants

- CoE remains the sole production snow-mass/melt authority.
- No persistent state reaches production state, routing, WAT/HBP/PASS, public
  outputs, fixtures, observations, defaults, or calibration.
- Evaluation failures fail the evaluation request without partially updating
  its committed state and without changing authoritative execution.
- No surrogate, provisional, proxy, or heuristic physics is introduced.

## Security And Data Impact

Local source and retained fixtures only. No secrets, external messages,
deployment, observation edits, or public release. Internal trace paths remain
explicit opt-in. Security/data disposition must be recorded before closure.

## Line-Count Governance

Record touched Rust line counts before and after. Files at 2000+ lines are
`WARN` with decomposition rationale; nonexempt 3000+ files block closure.

## Review And Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only science/Rust reviewers, one independent read-only
consumer/reconstruction reviewer, the `comparator_suite_runner` for heavy
frost/full-workspace and retained-fixture commands, and two independent
read-only terminal verifiers. Expected outputs are compact findings, reproduced
metrics, exact commands/counts, and artifact/log paths. Reviewers/verifiers are
read-only; the comparator may write only ignored package target outputs.

Reviews and verifications must check contract authority, conservation anti-
tautology, production isolation, claim limits, gate legitimacy, and line-count
governance. Every finding is dispositioned before closure.

## Progress

- [x] (2026-08-07) User authorized scaffold and end-to-end execution.
- [ ] Freeze scaffold and pre-implementation evidence.
- [ ] Complete contract-first authority and tests.
- [ ] Implement and validate persistent evaluation state.
- [ ] Complete reviews, verification, and disposition.

