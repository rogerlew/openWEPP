# Required Reading Map

Evidence label: Static/Ran.

Status: `SCAFFOLDED`

Required-reading budget:

- Local byte total: `354434`.
- Threshold: `OK` (`<=400000` bytes).
- Source: `wc -c` over kickoff core, target, focused tests, and
  `SC-OFEROUTE-001`.

Instruction discovery:

- `tools/agents/find-agents --for crates/openwepp-runner/src/hillslope/laned_shadow.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs tests/integration/laned_shadow_h2637.rs docs/work-packages/README.md`
  reported root `AGENTS.md`, `crates/AGENTS.md`,
  `docs/work-packages/AGENTS.md`, and `tests/AGENTS.md` as applicable.

| Path | Tier | Budget counted | Rationale | Applicability trigger | Read status |
|---|---|---:|---|---|---|
| `AGENTS.md` | Core | yes | Root governance, CQR commit boundaries, kernel/science posture. | Always | Read |
| `docs/work-packages/AGENTS.md` | Core | yes | Work-package execution, gate non-deferral, review/verification, CQR nightly process. | Always | Read |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | yes | Operator shorthand, target selection, scaffold/commit protocol. | Always | Read |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | yes | Behavior-preserving refactor procedure and closure ladder. | Always | Read |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | yes | CRAP-specific cover-then-decompose procedure and behavior identity guard. | Always | Read |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | yes | Coverage and CRAP thresholds. | Always | Read |
| `docs/standards/prompt-wording-guidance.md` | Core | yes | Required prompt shape and subagent wording. | Always | Read |
| `crates/AGENTS.md` | Core | yes | Rust crate, typed guard, API, and validation rules. | Editing crate code | Read |
| `tests/AGENTS.md` | Core | yes | Integration-test conventions and full gate expectations. | Adding/materially changing tests | Read |
| `docs/specifications/science-contracts/AGENTS.md` | Core | yes | Kernel/science contract governance before edits. | Science-sensitive runtime target | Read |
| `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | Core | yes | Lane D shadow, active routing, coefficient authority, selector, and `INV-OFEROUTE-012` obligations. | Science-sensitive runtime target | Relevant rows inspected for scaffold; read before implementation |
| `docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/package.md` | Core | yes | Package-local scope, phases, and exit criteria. | Always | Authored |
| `docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001/artifacts/required-reading-map.md` | Core | yes | Local instruction map and reading state. | Always | Authored |
| `crates/openwepp-runner/src/hillslope/laned_shadow.rs` | On-demand | yes | Target module and source surface. | Before implementation | Inspected for scaffold sizing/metrics |
| `crates/openwepp-runner/src/hillslope/03_tests.rs` | On-demand | yes | Runner crate unit-test surface if needed. | If characterization requires crate-local tests | Search-inspected |
| `crates/openwepp-runner/src/hillslope/tests03/direct_publication_source_guards.rs` | On-demand | yes | Existing source guard for live dynamic Lane D operands. | If source-guard binding is touched | Search-inspected |
| `tests/integration/laned_shadow_h2637.rs` | On-demand | yes | H2637 fail-closed, output-identity, and selector integration surface. | If integration characterization is touched | Search-inspected |
| `docs/standards/local-ci-gate-selection.md` | Conditional | yes | Focused gate selection and timing diagnostics. | If focused iteration gates need narrowing | Read |
| adjacent direct-publication modules | Conditional | no | Producer/consumer path source if unit/source-guard tests must be widened. | If target changes require consumer-path proof beyond existing guards | Pending unless triggered |
