# Gate Results

| Gate | Result | Evidence |
|---|---|---|
| Pre-decomposition public characterization | PASS | Detached scaffold: 21 passed; `characterization.md`. |
| Focused parser contract test | PASS | `cargo nextest run --test infile_irrigation_depletion_parser_contract --profile quick`; 21/21. |
| Focused clippy/fmt/diff | PASS | Current target checks exit 0. |
| Target workspace-instrumented coverage / CRAP | PASS | 92.329% lines, 91.633% regions, zero rows >30. |
| Package/catalog docs lint | PASS | `markdown-doc lint --path docs/work-packages/20260709-cqr-nightly-b02-06-irrigation-depletion-001 --path docs/work-packages/README.md --format plain`; 23 files, 0 errors/warnings. |
| `cargo fmt --check` | PASS | Delegated final closure, exit 0. |
| Workspace clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings`, exit 0. |
| Workspace full nextest | PASS | 1633 passed, 3 skipped, 4 slow, 604.242s. |
| `cargo deny check` | PASS | Delegated final closure, exit 0. |

The root-only coverage command emitted empty dependency LCOV; package closure
uses the valid same-test workspace-instrumented fallback, recorded in
`coverage-after.md`.
