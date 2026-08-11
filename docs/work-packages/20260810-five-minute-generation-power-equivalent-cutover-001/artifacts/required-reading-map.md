# Required Reading Map

Status: `executed — intake scope`

Evidence mode: `Static + Ran`

| Tier | Paths | Trigger / rationale |
|---|---|---|
| Core | `AGENTS.md`; `docs/codex_exec_plans.md`; `docs/work-packages/AGENTS.md`; `docs/work-packages/README.md`; package-local `package.md` | Required before edits. |
| Conditional | `docs/specifications/science-contracts/AGENTS.md`; science-contract authoring procedure, profile, and index; `docs/standards/testing-and-gate-strategy.md`; `docs/standards/kernel-work-package-preparation.md`; `docs/standards/prompt-wording-guidance.md`; `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` | Required before the applicable contract, kernel, validation, prompt, or baseline-provenance work. |
| On-demand | Named `SC-*` contracts, ADR-0036, direct-runtime sources, output schema, Topanga plan/harness, and pinned legacy sources | Load only for the touched mechanism or evidence surface. |

Record actual instruction-chain output, byte totals, and required-reading budget disposition before production edits.

## Instruction chains

Ran:

    tools/agents/find-agents --for <declared package, contract, Rust, output, and integration-test paths>

The applicable chains were:

- package and catalog: root `AGENTS.md` -> `docs/work-packages/AGENTS.md`;
- canonical contracts and index: root `AGENTS.md` ->
  `docs/specifications/science-contracts/AGENTS.md`;
- Rust crates: root `AGENTS.md` -> `crates/AGENTS.md`;
- integration tests: root `AGENTS.md` -> `tests/AGENTS.md`.

Validation and prompt work additionally loaded `docs/standards/AGENTS.md` and
`docs/standards/prompt-wording-guidance.md` as directed by package governance.

## Byte budget

Static: core required reading totals `512290` local bytes. Conditional
pre-edit governance and instruction reading totals `130811` bytes. The
combined `643101` bytes is `WARN` under the canonical `>400000` and
`<=800000` thresholds. The dominant core item is the work-package catalog;
it remains core because the kickoff prompt names it explicitly. On-demand
contract/source files were inspected only for the prerequisite and consumer
surfaces reached before the hold. Production edits did not begin.
