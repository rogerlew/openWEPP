# Required reading map

Status: `COMPLETE`

Evidence mode: `Static`

Initial instruction discovery used:

`tools/agents/find-agents --for docs/work-packages/20260830-workspace-gate-hold-lift-001/package.md crates/openwepp-coupled-time/src/event.rs crates/openwepp-biogeochemistry/src/lib.rs tests/integration`

Applicable chains are root plus `docs/work-packages/AGENTS.md` for package
artifacts, root plus `crates/AGENTS.md` for the two initial Rust sources, and
root plus `tests/AGENTS.md` for integration tests. Nearest instructions must be
rediscovered and recorded before any newly classified path is edited.

The initial governance reading totals 137,872 bytes and is
`REQUIRES-JUSTIFICATION`: this package owns a critical cross-workspace
correctness hold-lift, so the full work-package, defect-closure, validation,
prompt, crate, and test rules cannot be moved to on-demand reading.
