# Required Reading Map

Evidence label: Static/Ran.

Status: `EXECUTED`

Required-reading budget:

- Local byte total: `364227`.
- Threshold: `OK` (`<=400000` bytes).
- Source: `wc -c` over kickoff core required-reading paths, excluding
  conditional/on-demand large contracts until triggered.

Instruction discovery:

- `tools/agents/find-agents --for crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs crates/openwepp-watershed-orchestrator/src/lib.rs docs/work-packages/README.md tests/integration`
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
| `docs/specifications/science-contracts/AGENTS.md` | Core | yes | Kernel/science contract governance before edits. | Kernel/science target | Read |
| `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | Core | yes | Watershed channel routing and detachment/deposition authority. | Kernel/science target | Inspected for scaffold |
| `docs/specifications/science-contracts/contracts/SC-SED-001.md` | Core | yes | Sediment continuity and WSHEDIMPL detachment history/provenance. | Kernel/science target | Inspected for scaffold |
| `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` | Core | yes | Shear and hydraulics coupling authority. | Kernel/science target | Inspected for scaffold |
| `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/package.md` | Core | yes | Package-local scope, phases, and exit criteria. | Always | Authored |
| `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/required-reading-map.md` | Core | yes | Local instruction map and reading state. | Always | Authored |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs` | On-demand | yes | Target module and source surface. | Before implementation | Inspected for scaffold sizing/metrics |
| `crates/openwepp-watershed-orchestrator/src/lib.rs` | On-demand | yes | Existing test module with target helper tests. | Before characterization | Inspected |
| `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` | Conditional | no | Publication, system balance, and watershed assembly authority. | If publication/system semantics are touched | Pending unless triggered |
| `docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md` | Conditional | no | Channel input binding authority. | If characterization needs channel-input binding | Pending unless triggered |
| `docs/standards/local-ci-gate-selection.md` | Conditional | no | Focused gate selection and timing diagnostics. | If focused iteration gates need narrowing | Pending unless triggered |
