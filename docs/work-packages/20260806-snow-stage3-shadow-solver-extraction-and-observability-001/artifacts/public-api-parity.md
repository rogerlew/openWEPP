# Public API Parity

Status: PASS for the extraction increment.

Evidence mode: Static plus Ran on 2026-08-06.

Static diff: no `pub` production type, field, trait, or callable signature
changed. The only new extraction visibility is the private module seam
`pub(super) fn resolve_stage3_liquid_routing`; the nested evaluation functions
are likewise visible only to their parent module.

Ran:

- `cargo check -p openwepp-hillslope-orchestrator`: PASS.
- focused pre-existing Stage 3 surface-energy, liquid-routing, and decoupled
  water-temperature suites: PASS, 30 tests.

Final typed additions are limited to
`SnowStage3EvaluationOperator`, `DirectSnowStage3EvaluationDiagnostics`,
`SnowStage3TurbulentTransferError`, the optional evaluation selector on the
existing options record, and optional evaluation evidence on Stage 3
diagnostics. No production state, ledger, routing, or publication callable
changed. Focused integration and all-target clippy compilation prove existing
consumers remain source-compatible after the prospectively declared diagnostic
API delta.
