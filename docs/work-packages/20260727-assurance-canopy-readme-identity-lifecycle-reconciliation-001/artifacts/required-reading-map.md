# Required Reading Map

Status: `COMPLETE`

Evidence class: `Static`

Core reading, completed before edits:

| Path | Bytes | Rationale |
| --- | ---: | --- |
| `AGENTS.md` | 12,488 | repository invariants |
| `docs/work-packages/AGENTS.md` | 27,243 | package governance |
| `crates/AGENTS.md` | 5,165 | Rust authoring |
| `tests/AGENTS.md` | 4,684 | integration-test rules |
| `tests/fixtures/AGENTS.md` | 9,631 | fixture authority |
| `assurance/v2/README.md` | 12,591 | lifecycle/hash authority |
| `docs/defect_closure_execplans.md` | 24,803 | defect closure |

Core total: 96,605 bytes, `OK` under the 400,000-byte threshold.

Conditional reading:

- `docs/standards/testing-and-gate-strategy.md`, assurance-impact and terminal
  gate sections, triggered by lifecycle implementation and full-gate repair;
- `docs/standards/prompt-wording-guidance.md`, triggered by package prompt
  authoring;
- `docs/codex_exec_plans.md`, triggered by ExecPlan authoring.

On-demand implementation reading:

- `crates/openwepp-assurance/src/v2/amendment.rs`
- `crates/openwepp-assurance/src/v2/transaction.rs`
- `crates/openwepp-assurance/src/v2/identity.rs`
- `crates/openwepp-assurance/src/cli.rs`
- `tests/integration/assurance_v2_amendment_contract.rs`

Instruction discovery ran with `tools/agents/find-agents` for the declared
assurance, fixture, test, and work-package paths.
