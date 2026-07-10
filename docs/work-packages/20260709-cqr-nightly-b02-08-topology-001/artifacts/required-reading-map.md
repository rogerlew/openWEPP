# Required-Reading Map

| Source | Tier | Why | Result |
|---|---|---|---|
| `AGENTS.md` | Core | Root governance, package scope, CQR fresh-batch intent, validation gates. | Read before scaffold and re-read on resumed execution. |
| `crates/AGENTS.md` | Core | Rust crate typed-error, no production unwrap/expect, and closure-loop rules. | Read before implementation. |
| `tests/AGENTS.md` | Core | Integration-test conventions and focused/full gate selection. | Read before test edits. |
| `docs/work-packages/AGENTS.md` | Core | CQR package phases, subagent authorization wording, review/verification/disposition rules. | Read before scaffold and re-read on resumed execution. |
| `docs/work-packages/20260709-cqr-nightly-b02-08-topology-001/package.md` | Core | Local objective, scope, write set, gates, and subagent requirements. | Read before implementation. |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | Batch selection/execution protocol. | Read before scaffold; no target-specific reread needed after package scaffold. |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | Behavior-preserving extraction and artifact closure rules. | Read before implementation. |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | CRAP-driven cover-then-decompose policy and numeric-equivalence guard. | Read before implementation. |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | Binding CRAP <= 30 and coverage/obligation closure thresholds. | Read before implementation. |
| `docs/standards/prompt-wording-guidance.md` | Core | Required kickoff prompt structure and subagent authorization language. | Read before prompt repair. |
| `crates/openwepp-topology/src/lib.rs` | On-demand | Target parser, display, validation, graph/reference behavior. | Read before test/source edits. |
| `tests/integration/topology_graph_validation_gate.rs` | On-demand | Focused existing public behavior oracle and characterization target. | Read before test edits. |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | On-demand | Downstream typed watershed consumer references for topology graph/validation. | Searched for call sites; read details only if consumer behavior changes. |
| `tests/fixtures/topology/*` | On-demand | Fixture grammar examples and validation cases. | Read before fixture-dependent changes if needed. |
| `docs/specifications/science-contracts/AGENTS.md` plus nearest `SC-*` | Conditional | Required only for contract authority, conservation output, or contract-derived test changes. | Not triggered; package is behavior-preserving topology glue CQR. |
| `docs/standards/local-ci-gate-selection.md` | Conditional | Required only if focused iteration gates need narrowing before final closure. | Not triggered yet; default focused test and full closure loop remain applicable. |
