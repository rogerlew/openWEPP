# CQR06 Gate Results

Evidence class: Ran

Focused characterization:

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo test --test wb19_lateral_drainage_physics_kernel_contract` before edits | `0` | `15 passed; 0 failed` |
| `cargo test --test wb19_lateral_drainage_physics_kernel_contract` after edits | `0` | `15 passed; 0 failed` |

Coverage and CRAP:

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo llvm-cov clean --workspace` before LCOV | `0` | cleaned coverage artifacts |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path .../lcov_before.info` | `0` | report saved |
| `cargo crap --workspace --lcov .../lcov_before.info --min 0 --format json --output .../crap_before.json` | `0` | max target CRAP `300.2455501433063` |
| `cargo llvm-cov clean --workspace` after refactor | `0` | cleaned coverage artifacts |
| `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path .../lcov_after.info` | `0` | report saved |
| `cargo crap --workspace --lcov .../lcov_after.info --min 0 --format json --output .../crap_after.json` | `0` | max target CRAP `26.541362973760947` |

Final closure gates:

| Command | Exit | Result |
| --- | ---: | --- |
| `cargo fmt --check` | `0` | passed |
| `cargo clippy --workspace --all-targets -- -D warnings` | `0` | passed |
| `cargo test --workspace` | `0` | passed |
| `cargo deny check` | `0` | `advisories ok, bans ok, licenses ok, sources ok` |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260615-cqr06-lateral-drainage-complexity-001 --format json` | `0` | `files_scanned: 26, errors: 0, warnings: 0` |

Notes:

- The first before JSON coverage attempt failed to generate a usable report
  after coverage-mode runner/object-path issues. The LCOV before and after
  reports are the authoritative coverage inputs for this package.
- `cargo crap` emitted the existing workspace warning that `124` source files had
  no matching LCOV entries for both before and after. The target file had
  matching LCOV entries and target rows were emitted.
