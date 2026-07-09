# Required Reading Map

Status: `COMPLETE`

Required-reading budget: `248247` bytes, `REQUIRES-JUSTIFICATION`.

Justification: this package targets a science-tier WS12 impoundment runtime
projection module. The large target module, two live integration-test surfaces,
test guidance, and `SC-IMPOUND-001` are required before edits because the work
must preserve active-structure projection coefficients, floating-point grouping,
and typed fail-closed behavior.

| Path | Tier | Bytes | Rationale | Applicability trigger | Read status |
|---|---|---:|---|---|---|
| `AGENTS.md` | Core | 10269 | Repository governance and CQR shorthand. | Always. | Read |
| `docs/work-packages/AGENTS.md` | Core | 18383 | Work-package process, CQR nightly, reviews, gates. | Always. | Read |
| `docs/work-packages/20260709-cqr-nightly-02-watershed-chaninp-001/package.md` | Core | 9678 | Package objective, scope, write set, phases, and exit criteria. | Always. | Read |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | 11066 | Nightly target selection and per-module execution protocol. | Always. | Read |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | 10569 | Behavior-preserving refactor rules and gates. | Always. | Read |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | 10087 | CRAP-specific cover-then-decompose procedure. | Always. | Read |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | 8710 | Binding coverage and CRAP thresholds. | Always. | Read |
| `docs/standards/prompt-wording-guidance.md` | Core | 9780 | Prompt budget and subagent wording. | Prompt scaffolding. | Read |
| `crates/AGENTS.md` | Core | 5171 | Rust crate rules and final gates. | Target under `crates/`. | Read |
| `tests/AGENTS.md` | Core | 4534 | Test conventions and gates. | Focused tests in scope. | Read |
| `docs/specifications/science-contracts/AGENTS.md` | Core | 5599 | Contract-first and science-authority rules. | Science-tier target. | Read |
| `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md` | Core | 36598 | WS12 impoundment active-projection and typed guard authority. | Target projects impoundment coefficient families. | Read |
| `docs/work-packages/20260709-cqr-nightly-02-watershed-chaninp-001/artifacts/required-reading-map.md` | Core | 2909 | Kickoff required-reading budget/map and read-status tracking. | Always. | Read |
| `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs` | On-demand | 56746 | Target source module. | Before implementation edits. | Read in targeted chunks |
| `tests/integration/infile_watershed_impoundment_parser_contract.rs` | On-demand | 19714 | Parser-side impoundment coefficient behavior. | Characterization/focused tests. | Read in targeted chunks |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | On-demand | 41021 | Runtime WS12 impoundment projection behavior. | Characterization/focused tests. | Read in targeted chunks |

Conditional/on-demand if needed:

- `docs/standards/local-ci-gate-selection.md` if focused iteration gates need
  narrowing before final closure.
- `docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md`
  if parser-file field interpretation is needed for new characterization.
