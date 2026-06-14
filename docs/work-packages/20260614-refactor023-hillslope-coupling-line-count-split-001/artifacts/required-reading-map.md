# Required Reading Map

Status: complete

## Reading Budget

- local_required_bytes_total: 106472
- threshold_outcome: OK
- measurement_method: `wc -c AGENTS.md docs/codex_exec_plans.md docs/work-packages/AGENTS.md docs/work-packages/README.md docs/work-packages/20260614-refactor023-hillslope-coupling-line-count-split-001/package.md docs/standards/mechanical-refactor-authoring-guide.md crates/AGENTS.md docs/specifications/science-contracts/AGENTS.md docs/standards/kernel-work-package-preparation.md`
- measured_at_utc: 2026-06-14T22:56:26Z

Canonical thresholds from `docs/standards/kernel-work-package-preparation.md`:
OK <= 400000 bytes; WARN > 400000 bytes; REQUIRES-JUSTIFICATION > 800000
bytes.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| `AGENTS.md` | Core | Root governance for package and Rust kernel work | Always | Pre-edit | Codex | 2026-06-14T22:56:26Z | Provided in prompt; repo path exists. |
| `docs/codex_exec_plans.md` | Core | Work-package autonomy, review, and line-count addendum | Always | Pre-edit | Codex | 2026-06-14T22:56:26Z | Read package addendum and relevant ExecPlan requirements. |
| `docs/work-packages/AGENTS.md` | Core | Package execution, gate, evidence, and review rules | Always | Pre-edit | Codex | 2026-06-14T22:56:26Z | Read. |
| `docs/work-packages/README.md` | Core | Package catalog and process conventions | Always | Pre-edit | Codex | 2026-06-14T22:56:26Z | Read relevant sections. |
| `docs/work-packages/20260614-refactor023-hillslope-coupling-line-count-split-001/package.md` | Core | Package-local authority, write set, and gates | Always | Pre-edit | Codex | 2026-06-14T22:56:26Z | Authored from current instructions. |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | Mechanical refactor seam and closure loop requirements | Mechanical refactor | Pre-edit | Codex | 2026-06-14T22:56:26Z | Read. |
| `crates/AGENTS.md` | Core | Rust crate rules, typed guards, and line-count governance | Rust source edits | Pre-edit | Codex | 2026-06-14T22:56:26Z | Read. |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Kernel authority guardrails for no-behavior-change refactor | Kernel-adjacent source path | Pre-edit | Codex | 2026-06-14T22:56:26Z | Read. No `SC-*` amendment expected. |
| `docs/standards/kernel-work-package-preparation.md` | On-demand | Scaffold and reading-budget details | Package authoring | Pre-edit scaffold | Codex | 2026-06-14T22:56:26Z | Read threshold and scaffold sections. |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-06-14T22:56:26Z | Codex | Initialized required-reading map for REFACTOR023. |
