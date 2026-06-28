# Line-Count Governance

Evidence class: Static plus command-backed validation recorded in
`gate-results.md`.

No Rust source file is edited by this package. The `.rs` line-count refactor
threshold is therefore not triggered.

The package intentionally reads current Rust implementation shape for design
grounding only:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`

No 2000-line warning or 3000-line required refactor applies to the package
write set.

