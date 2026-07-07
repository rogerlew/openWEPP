# Required Reading Map

## Authority and Maintenance Responsibility

Agents executing or authoring this package have explicit authority and
responsibility to maintain this file as a living control artifact. A stale or
incomplete required-reading map is a governance defect and keeps package
disposition in HOLD until corrected.

## Reading Budget

- local_required_bytes_total: 208462
- threshold_outcome: OK
- measurement_method: `wc -c AGENTS.md docs/work-packages/AGENTS.md docs/specifications/science-contracts/AGENTS.md crates/AGENTS.md tests/AGENTS.md docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/package.md docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/worker-handoff.md docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/gate-results.md docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i1-implicit-stepper-evidence.md docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i2-hybrid-evidence.md docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/package.md docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/fix-evidence.md docs/standards/prompt-wording-guidance.md`
- measured_at_utc: 2026-07-07T04:43:54Z

Thresholds from `docs/standards/kernel-work-package-preparation.md`:
- `OK`: `<=400000` bytes.
- `WARN`: `>400000` bytes.
- `REQUIRES-JUSTIFICATION`: `>800000` bytes.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| `AGENTS.md` | Core | Root governance for package and kernel work | Always | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/work-packages/AGENTS.md` | Core | Work-package execution, gate, review, and subagent rules | Always | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/specifications/science-contracts/AGENTS.md` | Core | Contract-first sequencing and science-contract governance | Kernel/contract edits | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `crates/AGENTS.md` | Core | Rust crate-local implementation/test rules | Rust edits | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `tests/AGENTS.md` | Core | Test governance | Test edits/gates | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Core | Canonical `ofe_routing` authority and BEI | Contract, solver, or ratification edits | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/package.md` | Core | Parent package scope and open closure gates | Always | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/worker-handoff.md` | Core | Parent handoff naming solve-cost and ratification work | Always | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/gate-results.md` | Core | Parent open gate state | Always | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i1-implicit-stepper-evidence.md` | Core | I1 implicit stepper ladder and fidelity evidence | Ratification | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/i2-hybrid-evidence.md` | Core | I2 hybrid timing/closure evidence | Ratification | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/package.md` | Core | Rev-30 aggressive-rule scope and closure posture | Solve-cost and ratification | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/work-packages/20260706-laned-router-t3-aggressive-deficit-carry-001/artifacts/fix-evidence.md` | Core | Current timing/counter evidence proving solve-cost bottleneck | Solve-cost | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/standards/prompt-wording-guidance.md` | Core | Execution prompt and subagent wording standard | Prompt/package authoring | Pre-edit | Codex | 2026-07-07T04:43:54Z | Read |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional | Detailed contract amendment procedure | If local amendment pattern is insufficient | Before contract schema/profile repair | Agent | 2026-07-07T04:43:54Z | Not triggered yet |
| `docs/standards/local-ci-gate-selection.md` | Conditional | Narrowed iteration gate selection | If using reduced pre-closure gates | Before narrowed gate selection | Agent | 2026-07-07T04:43:54Z | Not triggered yet |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/*.rs` | On-demand | Touched routing mechanisms | Per file touched | Before edit | Agent | 2026-07-07T04:43:54Z | Partial reads in progress |
| D10B/D15A package artifacts | On-demand | Active-lane and Case-4 lineage if ratification needs historical proof | Comparator/consumer lineage | Phase-local | Agent | 2026-07-07T04:43:54Z | Use only when needed |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| 2026-07-07T04:43:54Z | Codex | Initialized required-reading map from canonical template. |

