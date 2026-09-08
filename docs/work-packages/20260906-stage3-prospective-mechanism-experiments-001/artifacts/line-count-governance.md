Ran: read-only Git changed-path inventory, newline counts and SHA-256 inspection.
Static: decomposition disposition. Provisional F/R source inspection; R-PC1
remains mutable. Refresh against terminal source manifests before closure.

Authority: docs/work-packages/AGENTS.md and crates/AGENTS.md: >=2000 lines WARN;
>=3000 nonexempt lines require refactor before closure. No generated/fixture
exemption is claimed. No inspected touched Rust file reaches 3000 lines.

A baseline is 2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3; A changes compare to
e89befa4678eadec039b3e7f7fe0a176af8e9dc5. F/R compare their current detached
working sources to A. Counts include untracked new Rust files; unrelated
pre-existing tmp/ is excluded. The complete touched inventory is in
owned-file-manifest.md. The table lists every WARN file at the inspection cut.

| Arm | Path (relative to crates/) | Before | Observed | Disposition |
|---|---|---:|---:|---|
| A | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs | 2424 | 2447 | WARN: compact observation hooks only; extract terminal evaluation helpers in the authorized architecture successor. |
| A | openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs | 2885 | 2910 | WARN: retain source-real error order; future split by provider projection/retention/finalization boundaries. |
| A | openwepp-land-surface-energy/src/solver_covered_evaluation.rs | 2639 | 2651 | WARN: observation hooks; future canonical node decomposition must preserve evaluation order. |
| A | openwepp-land-surface-energy/src/transaction.rs | 2944 | 2952 | WARN: near threshold; future transaction audit extraction without changing validation order. |
| A | openwepp-runner/src/hillslope/03_tests.rs | 2932 | 2933 | WARN: module registration only; continue extracting test bodies into tests03 modules. |
| A | openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs | 2921 | 2654 | WARN: experimental harness extracted; future split fixture authoring from qualification assertions. |
| A | ../tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs | 2394 | 2436 | WARN: authority assertions; future split historical and prospective contract checks without changing required-case obligations. |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs | 2689 | 2699 | WARN: typed-provider module wiring; future evidence/type module extraction. |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs | 2955 | 2954 | WARN: near threshold, no growth; future persistent evaluator/API split. |
| F | openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs | 2447 | 2298 | WARN: treatment helper extracted; retain canonical shared arithmetic. |
| F | openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs | 2465 | 2533 | WARN: authentic forced-reference tests; future separate replay/ownership test module. |
| F | openwepp-hillslope-orchestrator/src/snow_stage3_v11_terminal_execution.rs | 2910 | 2942 | WARN: near threshold; focused typed wiring, future provider/retention extraction. |
| F | openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs | 2874 | 2944 | WARN: near threshold; future carrier request and test separation. |
| R | openwepp-land-surface-energy/src/solver_covered_evaluation.rs | 2651 | 2700 | WARN: shared evaluator hooks; dependency graph/replay already extracted into separate files. |

No present mandatory >=3000 refactor blocker was observed. The 2952/2954/2942/
2944 files have little headroom; later edits require a fresh count. Split intent
is future architecture maintenance, not authorization to alter this package's
physics or silently expand its treatment. Optional CRAP/coverage is not used
as a substitute for this line-count rule.

Terminal reconciliation 2026-09-08 PDT: the only resumed Rust delta is the
common test-harness OFE support-count assertion captured in A-source-05/F-source-06;
it adds no lines and creates no new threshold. No experimental Rust remains
applied in the primary checkout.
