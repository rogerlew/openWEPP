# Required-Reading Map

| Source | Why it is required | Read result |
|---|---|---|
| `AGENTS.md` | Repository invariants, CQR protocol, typed errors, and final gates. | Read before scaffold. |
| `crates/AGENTS.md` | Rust CLI, subprocess, line-governance, and validation rules. | Read before scaffold. |
| `docs/work-packages/AGENTS.md` | Package shape, CQR/nightly, commit, review, and verification requirements. | Read before scaffold. |
| Science-contract guidance | Contract routing for runtime watershed integration surfaces. | Read before scaffold. |
| `SC-SYSTEM-001` | Assembly handoff, publication, and hard-fail boundary invariants. | Read relevant integration/guard sections before scaffold. |
| `SC-ROUTE-001` | Watershed routing applicability and consumer guard boundaries. | Read relevant guard/obligation sections before scaffold. |
| `SC-GWBASEFLOW-001` | Groundwater-sidecar authority and no-default/no-double-count boundary. | Read relevant branch/guard sections before scaffold. |
| CQR ExecPlan and authoring guides | Behavior-preserving scope, metrics, scaffold/completion commits. | Read before scaffold. |
| ADR-0021 | Test/coverage closure threshold and real-consumer requirements. | Read before scaffold. |
| Target module and CLI contract tests | Existing behavior, parse/manifest seams, and output consumers. | Read before scaffold. |

No extraction may alter CLI option semantics, error strings, runfile/manifest
grammar, typed parse/validation order, source-relative paths, units, topology,
publication, or fail-closed contract behavior.
