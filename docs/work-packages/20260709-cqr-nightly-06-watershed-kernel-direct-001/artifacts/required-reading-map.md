# Required Reading Map

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Required-reading budget:

- Local byte total: `375407`.
- Threshold: `OK` (`<=400000` bytes).
- Source: `wc -c` over kickoff required-reading paths.

Instruction discovery:

- `tools/agents/find-agents --for crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs docs/work-packages/20260709-cqr-nightly-06-watershed-kernel-direct-001/package.md`
  reported root `AGENTS.md`, `crates/AGENTS.md`, and
  `docs/work-packages/AGENTS.md`.

| Path | Tier | Budget counted | Rationale | Applicability trigger | Read status |
|---|---|---:|---|---|---|
| `AGENTS.md` | Core | yes | Root governance, CQR commit boundaries, kernel/science posture. | Always | Read |
| `docs/work-packages/AGENTS.md` | Core | yes | Work-package execution, gate non-deferral, review/verification, CQR nightly process. | Always | Read |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | yes | Operator shorthand, target selection, scaffold/commit protocol. | Always | Read |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | yes | Behavior-preserving refactor procedure and closure ladder. | Always | Read |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | yes | CRAP-specific cover-then-decompose procedure and numeric identity guard. | Always | Read |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | yes | Science-tier coverage and CRAP thresholds. | Always | Read |
| `docs/standards/prompt-wording-guidance.md` | Core | yes | Required prompt shape and subagent wording. | Always | Read |
| `crates/AGENTS.md` | Core | yes | Rust crate, kernel behavior, and validation rules. | Editing crate code | Read |
| `tests/AGENTS.md` | Core | yes | Test conventions and full gate expectations. | Adding/materially changing tests | Read |
| `docs/specifications/science-contracts/AGENTS.md` | Core | yes | Science-contract authority and kernel-affecting sequencing. | Target is science-tier direct runtime code | Read |
| `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | Core | yes | Channel routing invariants and payload obligations touched by direct runtime. | Before production edits | Pending before implementation |
| `docs/specifications/science-contracts/contracts/SC-SED-001.md` | Core | yes | Sediment load/capacity invariants touched by target CRAP rows. | Before production edits | Pending before implementation |
| `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | Core | yes | Direct impoundment execution obligations. | Before production edits | Read for prior package; re-check before implementation as needed |
| `docs/work-packages/20260709-cqr-nightly-06-watershed-kernel-direct-001/package.md` | Core | yes | Package-local scope, phases, and exit criteria. | Always | Authored |
| `docs/work-packages/20260709-cqr-nightly-06-watershed-kernel-direct-001/artifacts/required-reading-map.md` | Core | yes | Local instruction map and reading state. | Always | Authored |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | On-demand | yes | Target module and source surface. | Before implementation | Inspected for scaffold sizing |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | On-demand | not counted | Helper APIs and prior package behavior touched by direct impoundment flow. | If call-site or identity proof needs it | Available |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs` | On-demand | not counted | Sediment detachment helper context for direct sediment capacity assembly. | If sediment branch characterization needs it | Available |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` | On-demand | not counted | WS20 segment routing helper context for channel profile rows. | If WS20 characterization needs it | Available |
