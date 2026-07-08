# Line-Count Governance

Evidence mode: Static.

Relevant documentation line counts after execution:

| File | Lines | Disposition |
|---|---:|---|
| `AGENTS.md` | 132 | Within root size policy. |
| `docs/work-packages/AGENTS.md` | 219 | Existing package governance file, not modified. |
| `docs/specifications/science-contracts/AGENTS.md` | 72 | Existing science-contract governance file, not modified. |
| `package.md` | 214 | Package execution spec; acceptable for this evidence-heavy package. |
| `artifacts/implementation.md` | 66 | Focused implementation summary. |
| `artifacts/hold-legitimacy-audit.md` | 100 | Focused hold audit. |
| `artifacts/mechanism-attribution.md` | 94 | Generated evidence summary. |
| `artifacts/raw-hydrograph-numerics-summary.md` | 55 | Generated run summary. |

The committed JSON artifacts are compact summaries:

- `raw-hydrograph-numerics-summary.json`: 28 KiB.
- `mechanism-attribution.json`: 24 KiB.

Bulk run trees and raw trace JSONL files are ignored under
`artifacts/raw-hydrograph-numerics-runs/`.

Relevant Rust line counts after execution:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | 1985 | Below the 2000-line WARN threshold after trace additions. |
| `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` | 627 | Below WARN. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs` | 1472 | Below WARN. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs` | 1264 | Below WARN. |
| `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` | 1506 | Below WARN. |
| `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | 2753 | Existing WARN-size builder helper; this package added only the active step-trace config plumbing. No extraction was attempted because the package scope was diagnostic evidence, not builder CQR. |
| `crates/openwepp-runner/src/hillslope/laned_active.rs` | 225 | Below WARN. |
