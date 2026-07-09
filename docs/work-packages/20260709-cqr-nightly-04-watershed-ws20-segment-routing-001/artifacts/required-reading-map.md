# Required Reading Map

Evidence label: Static.

Status: `SCAFFOLDED`

Required-reading budget:

- Byte total: `306448`.
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
| `docs/specifications/science-contracts/AGENTS.md` | Core | `5599` | Contract/kernel process and fail-closed rules. | Kernel routing target | Read |
| `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | Core | `80433` | Canonical WS10/WS20 routing authority and route gap closure. | Kernel routing target | Targeted sections read; full file required before implementation if authority-adjacent edits occur |
| `docs/specifications/science-contracts/contracts/SC-SED-001.md` | Core | `92568` | Canonical sediment/channel process authority and WS20/WS21 gap closure. | Channel sediment routing target | Targeted sections read; full file required before implementation if authority-adjacent edits occur |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` | On-demand | `38722` | Target module. | Before characterization/refactor | Inspected |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | On-demand | not counted | Existing direct-physics marker test naming WS20 helpers. | If integration marker changes are needed | Relevant section read |

Implementation note: this package is behavior-preserving CQR. Contract files are
required authority context, not an amendment surface. Any need to change
contract authority is a hold boundary.
