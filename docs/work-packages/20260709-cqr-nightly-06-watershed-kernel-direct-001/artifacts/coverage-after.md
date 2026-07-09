# Coverage After

Evidence label: Static/Ran.

Status: `TARGETED-PASS`

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`

Targeted coverage command sequence:

- `cargo llvm-cov clean --workspace` - exit `0`.
- `cargo llvm-cov -p openwepp-watershed-orchestrator --lib --no-report` -
  exit `0`.
- `cargo llvm-cov --workspace --test wshedw5_typed_watershed_runtime_contract --lcov --output-path /tmp/openwepp-cqr-nightly-06-direct-targeted-final7.lcov --no-clean` -
  exit `0`.
- `cargo llvm-cov --json --output-path /tmp/openwepp-cqr-nightly-06-direct-targeted-final7.json --no-run` -
  exit `0`.

LCOV line coverage after:

- `LF:1888`
- `LH:1782`
- Line coverage: `94.385593220339%`

Baseline comparison:

- Baseline `LF:1506`, `LH:1042`, line coverage `69.18990703851262%`.
- Targeted after coverage improves the target-module line rate by
  `25.195686181826` percentage points.

Region coverage:

- Deduplicated source-span regions from the llvm-cov JSON export:
  `2123 / 2274`.
- Region coverage: `93.35971855760774%`.

Function coverage evidence:

- `cargo crap` after report has no target rows above CRAP `30`.
- Max target CRAP after is `23.069544598035826`, with the top row carrying
  `94.91525423728814%` cargo-crap function coverage and cyclomatic complexity
  `23`.
