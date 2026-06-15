# Required Reading Map

Static: reviewed the package-required governance, standards, and PL runtime
adapter authority before production edits.

| Source | Use in this package |
|---|---|
| `AGENTS.md` | Root work-package, truthfulness, Rust gate, and science-runtime rules. |
| `docs/work-packages/AGENTS.md` | Work-package evidence, gate non-deferral, review, verification, and line-count closure. |
| `docs/standards/AGENTS.md` | Standards routing. |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Behavior-preserving refactor discipline and final gate ladder. |
| `docs/standards/code-quality-refactor-authoring-guide.md` | CRAP-targeted cover-then-decompose procedure. |
| `docs/standards/module-test-enhancement-authoring-guide.md` | Coverage closure posture for target-module characterization. |
| `docs/specifications/science-contracts/AGENTS.md` | Runtime projection is kernel-adjacent; preserve contract authority and typed guards. |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Glue-tier coverage threshold and per-function CRAP `<= 30` target. |
| `docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-boundary-contract.md` | PL boundary families and runtime surface expectations. |
| `docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-runtime-adapter-contract.md` | `PL-MAN-SEAM-001` management adapter behavior. |
| `docs/work-packages/20260522-pl03-management-to-runtime-adapter-001/artifacts/pl03-runtime-surface-projection-map.md` | Expected schedule, growth, decomposition, alias, and guard symbol projection. |
| `crates/AGENTS.md` | Rust crate conventions and validation posture. |

Disposition: no contradictory instruction was found. The package stayed within
the declared write set except for `08_tests/common.rs`, a required import support
edit for added runtime-input characterization tests.
