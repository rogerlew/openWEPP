# Line Count Governance Checklist

Static: touched Rust files are below the work-package warning threshold
(`2000` lines) and far below the hard refactor threshold (`3000` lines).

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs` | 1551 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/management.rs` | 1195 | PASS |
| `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/common.rs` | 246 | PASS |

Static: target production file grew from `1210` to `1551` lines because dense
projection logic was decomposed into named private helpers. The line increase is
accepted because the package quality dimension is function-level CRAP and
function length; the largest target-module function after refactor is `75`
lines and all CRAP rows are below target.
