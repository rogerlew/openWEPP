# Public API Parity

Status: PASS after review-driven additive-wrapper remediation.

Evidence mode: Static plus Ran on 2026-08-06.

Static diff: no `pub` production type, field, trait, or callable signature
changed. The only new extraction visibility is the private module seam
`pub(super) fn resolve_stage3_liquid_routing`; the nested evaluation functions
are likewise visible only to their parent module.

Ran:

- `cargo check -p openwepp-hillslope-orchestrator`: PASS.
- focused pre-existing Stage 3 surface-energy, liquid-routing, and decoupled
  water-temperature suites: PASS, 30 tests.

Final additions use only new exported types and callables:
`SnowStage3EvaluationOperator`, `DirectSnowStage3EvaluationDiagnostics`,
`DirectSnowStage3EvaluationHourDiagnostics`,
`DirectSnowStage3EvaluationResult`, `DirectSnowStage3EvaluationError`,
`SnowStage3TurbulentTransferError`, and the two additive evaluator entry
points. `DirectSnowSurfaceEnergyOptions`,
`DirectSnowSurfaceEnergyHourDiagnostics`, `DirectSnowStage3Diagnostics`, and
`Wb11HydrologyKernelGuardError` retain their pre-package required
fields/variants; the options record differs only by a documentation comment.

The real runner stores evaluation evidence separately from the authoritative
partition and passes it only to the opt-in internal JSONL writer. A test writes
WAT, HBP, and PASS from enabled and disabled authoritative results and proves
exact byte identity. Focused integration, all-target compilation, and static
field/variant guards prove existing consumers remain source-compatible.
