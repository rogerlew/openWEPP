# Required Reading Map

Status: `PASS`

Static:

| Path | Applicability | Read |
| --- | --- | --- |
| `AGENTS.md` | Repository authority and gate lifecycle | yes |
| `crates/AGENTS.md` | Rust crate rules | yes |
| `docs/work-packages/AGENTS.md` | Package governance | yes |
| `docs/standards/AGENTS.md` | Standards routing | yes |
| `docs/standards/testing-and-gate-strategy.md` | Gate ordering and terminal reconciliation | yes |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Behavior-preserving mechanics | yes |
| `docs/standards/code-quality-refactor-authoring-guide.md` | CQR cover-first procedure | yes |
| `docs/standards/module-test-enhancement-authoring-guide.md` | Coverage and function-floor rules | yes |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | CRAP threshold authority | yes |
| `crates/openwepp-gate-planner/src/pre_heavy.rs` | Target implementation | yes |
| `crates/openwepp-gate-planner/src/pre_heavy_coverage_tests.rs` | Existing direct-coverage fixture | yes |

Ran:

- `tools/agents/find-agents --for` resolved `AGENTS.md` plus
  `docs/work-packages/AGENTS.md` for package files and `crates/AGENTS.md` for
  the Rust targets.
