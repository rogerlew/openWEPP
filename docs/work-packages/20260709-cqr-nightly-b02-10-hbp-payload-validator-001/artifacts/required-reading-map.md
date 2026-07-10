# Required Reading Map

Ran:
`tools/agents/find-agents --for crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs tests/integration/infile_hbp_parser_contract.rs docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/package.md docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/prompts/active/20260710-codex-cqr-nightly-b02-hbp-payload-validator_prompt.md`

| Path | Tier | Rationale | Trigger | Status |
|---|---|---|---|---|
| `AGENTS.md` | Core | Root repository governance. | All edits. | Read |
| `crates/AGENTS.md` | Core | Rust crate implementation guidance. | Target parser source. | Read |
| `tests/AGENTS.md` | Core | Integration-test guidance. | Focused test write set. | Read |
| `docs/work-packages/AGENTS.md` | Core | Package governance and CQR rules. | Work-package artifacts. | Read |
| `docs/work-packages/20260709-cqr-nightly-b02-10-hbp-payload-validator-001/package.md` | Core | Package-local authority. | Current package. | Read after scaffold |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | Batch execution protocol. | User requested CQR nightly batch. | Read |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | Behavior-preserving refactor gates. | CQR decomposition. | Read |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | CRAP-specific workflow. | Quality dimension is CRAP. | Read |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | CRAP and coverage closure thresholds. | ADR-0021 CQR binding. | Read |
| `docs/standards/prompt-wording-guidance.md` | Core | Subagent and package prompt wording. | Prompt/package scaffolding. | Read |
| `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs` | On-demand | Target module. | Implementation phase. | Pending scaffold |
| `tests/integration/infile_hbp_parser_contract.rs` | On-demand | Focused behavior oracle. | Characterization phase. | Pending scaffold |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Contract governance. | Only if contract authority is touched. | Not triggered |

Required-reading budget: 167,561 bytes across core, standards, target, and
focused test paths.

