# CQR Pre-Integration High A, HA-01 — Cascade Interpolation

Package: `20260711-cqr-preint-ha-01-cascade-001`
Status: `QUEUED`
ExecPlan: `docs/work-packages/cqr-high-risk-a-execplan.md`
Campaign: `CQR-PREINT-20260711`
Target: `HA-01`
Target module:
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce the eligible `interpolate_unit_discharge` row from CRAP `56` to at most
`30`, close science-tier module coverage and the ADR-0021 function floor, and
preserve exact cascade numerical, conservation, handoff, guard, and API
behavior. Characterization lands before any production decomposition. Coverage
may close this CC-7 row without production edits.

## Required Reading

Core reading is recorded in `artifacts/required-reading-map.md`: root, crate,
work-package, science-contract governance; the active ExecPlan and binding
contract; mechanical/CQR/test-enhancement standards; ADR-0021; prompt wording;
`SC-OFEROUTE-001`; the target and its inline tests.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to implementation, review, verification, and
comparator/closure-runner subagents for HA-01 characterization, focused/full
gate execution, metric checks, and independent review/verification. Expected
outputs are package-local review/verification artifacts, compact metrics,
commands, timings, hashes, and log paths. Write access is read-only except for
an explicitly assigned bounded edit to the target or package artifacts.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for every
full-workspace coverage/CRAP, Clippy, full-nextest, deny, comparator, release,
or cohort run. The parent does not substitute while that runner is available.

## Scope And Write Set

In scope: inline characterization, exact impossible-arm annotations only when
construction-proven and dual-reviewed, package artifacts, and behavior-
preserving helper extraction only if metrics still require it.

Out of scope: formulas, tolerances, float grouping, accumulation/order,
contract authority, public API/schema, error priority, handoff selection,
semantic dead-code removal, and unrelated cleanup.

Intended write set:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `docs/work-packages/20260711-cqr-preint-ha-01-cascade-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/cqr-high-risk-a-execplan.md`

Applicable instruction chain was obtained with `tools/agents/find-agents` and
is recorded in the reading map.

## Scaffold Commit Gate

Commit this scaffold before Rust/test edits. The scaffold includes the active
prompt, source-bound eligibility, baseline metric provenance, reading map, and
all gate/review/verification placeholders. Placeholder audit must be empty.

## Phase Plan

### Phase A — Baseline

Bind HA start metrics, source SHA, 627-line count, raw/actionable row, tier,
consumer limitation, coverage gaps, A–H/named obligations, and exact floor
proof.

### Phase B — Characterization

Add inline tests for empty/singleton/endpoints, both binary-search branches,
interior interpolation, negative-output clamp, finite/domain width failures,
and exact typed errors. Prove impossible local arms at exact granularity or
cover them; do not use broad exclusions.

### Phase C — Refactor

Remeasure first. If coverage closes CRAP at or below 30, make no production
edit. Otherwise extract one cohesive whole branch while preserving every float
expression and evaluation order.

### Phase D — Metrics And Gates

Run focused tests, same-source coverage/CRAP, consumer/conservation evidence,
line counts, `git diff --check`, exact Markdown lint, `cargo fmt --check`,
workspace Clippy, full nextest, and deny. Heavy gates are delegated.

### Phase E — Review, Verification, Disposition

Complete two independent reviews, disposition every finding, complete two
independent verifications, audit non-deferral and line-count governance, update
the catalog/ExecPlan, and commit `TERMINAL-PASS` or a legitimate terminal hold.

## Exit Criteria

- Scaffold commit predates Rust/test edits.
- Every raw row has exact eligibility and source identity.
- Science tier reaches at least 90% line and region; every eligible function is
  at least 75% region or has an exact dual-reviewed closed-list disposition.
- Applicable A–H and named SC-OFEROUTE obligations are fully bound.
- Every eligible target function has CRAP at most 30 with no non-target
  regression.
- Numeric/API behavior, conservative integral consumer, and current point-
  fallback limitation are recorded truthfully.
- Every required gate is `PASS`; `FAIL`, `BLOCKED`, or unjustified `NOT RUN`
  forces terminal hold.
- Dual review, finding disposition, dual verification, line count, handoff, and
  terminal commit are complete.

Final status is `TERMINAL-PASS` or `TERMINAL-HOLD` for High-A campaign scope.
