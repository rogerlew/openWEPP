# Required Reading Map

Evidence label: Static.

Status: `SCAFFOLDED`

Required-reading budget:

- Byte total: `144275`.
- Threshold disposition: `OK` (`<=400000` bytes).

| Path | Tier | Bytes | Rationale | Trigger | Read status |
|---|---:|---:|---|---|---|
| `AGENTS.md` | Core | `10624` | Root repository governance and CQR command meaning. | Always | Read |
| `docs/work-packages/AGENTS.md` | Core | `18585` | Work-package process, CQR nightly rules, subagent authorization, commit boundaries. | Always | Read |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | `11066` | Nightly CQR selection, scaffold, execution, gate, and commit protocol. | Always | Read |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | `10569` | Behavior-preserving refactor mechanics and closure loop. | Always | Read |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | `10087` | CRAP-specific cover-then-decompose and numeric-equivalence requirements. | Always | Read |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | `8710` | Binding science-tier coverage and CRAP closure policy. | Always | Read |
| `docs/standards/prompt-wording-guidance.md` | Core | `9780` | Required prompt wording, reading budget, and subagent wording. | Always | Read |
| `crates/AGENTS.md` | Core | `5171` | Rust crate/kernel authoring rules. | Always | Read |
| `tests/AGENTS.md` | Core | `4534` | Test authoring and focused gate conventions. | Always | Read |
| `docs/specifications/science-contracts/AGENTS.md` | Core | `5599` | Contract/kernel process and fail-closed rules. | Kernel impoundment target | Read |
| `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | Core | `36598` | Canonical impoundment stage-discharge/adaptive routing authority. | WS12 impoundment target | Targeted sections read; full file required before implementation if authority-adjacent edits occur |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | On-demand | `18189` | Target module. | Before characterization/refactor | Inspected |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs` | On-demand | not counted | Include ordering and test-layout context. | Before adding module-local tests | Inspected |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | On-demand | not counted | Runtime call-site and consumer context for impoundment helper outputs. | If behavior identity needs call-site proof | Relevant section inspected as needed |

Implementation note: this package is behavior-preserving CQR. Contract files are
required authority context, not an amendment surface. Any need to change
contract authority is a hold boundary.
