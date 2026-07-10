# Required Reading Map

| Path | Tier | Why | Status |
|---|---|---|---|
| `AGENTS.md` | Core | Root governance, CQR fresh-batch intent, validation gates. | Read before scaffold. |
| `crates/AGENTS.md` | Core | Rust crate rules, typed errors, no production unwrap/expect. | Read before scaffold. |
| `tests/AGENTS.md` | Core | Integration-test conventions and focused/full gate selection. | Read before scaffold. |
| `docs/work-packages/AGENTS.md` | Core | CQR package phases, subagent authorization, review/verification/disposition rules. | Read before scaffold. |
| `docs/standards/prompt-wording-guidance.md` | Core | Prompt wording, subagent requirement wording, autonomous scope. | Read before scaffold. |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | Fresh nightly batch and package sequencing. | Read for batch setup. |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | Behavior-preserving refactor closure loop. | Read before production edits. |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | Metric-driven CQR artifact requirements. | Read before production edits. |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | ADR-0021 coverage closure thresholds. | Read before coverage disposition. |
| `crates/openwepp-input-contract/src/parsers/slope.rs` | On-demand | Target source. | Read during scaffold. |
| `tests/integration/infile_slope_parser_contract.rs` | On-demand | Existing public parser behavior oracle. | Read before test edits. |

Instruction discovery:

`tools/agents/find-agents --for crates/openwepp-input-contract/src/parsers/slope.rs docs/work-packages/20260709-cqr-nightly-b02-09-slope-parser-001 tests/integration`
