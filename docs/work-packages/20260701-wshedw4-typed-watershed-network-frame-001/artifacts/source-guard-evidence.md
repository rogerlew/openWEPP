# Source Guard Evidence

Status: `QUEUED`

W4 execution must record source-level guards proving production routing loops no
longer use symbol-map surfaces.

Minimum guard targets:

- `openwepp-cli-watershed` production routed-stage handoff.
- watershed orchestrator production routing modules.
- publication seed/output helpers for protected watershed output files.

The guard may allow explicitly named compatibility/replay/diagnostic adapters,
but those allowances must be path-scoped and justified.
