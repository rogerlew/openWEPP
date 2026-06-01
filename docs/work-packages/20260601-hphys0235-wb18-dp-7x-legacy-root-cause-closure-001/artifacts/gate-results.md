# HPHYS0235 Gate Results

Status: completed  
Evidence mode: mixed (`Ran` + `Static`)

## Executed Gates

1. Hourly lane probe run (`openwepp-cli-hill`) — pass
2. Daily lane probe run (`openwepp-cli-hill`) — pass
3. Manifest lane provenance checks (`jq`) — pass
4. Numeric comparison checks (`duckdb`) — pass

## Not Executed in This Package

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

Rationale: this package is a docs/diagnostic closure slice with no production
code edits. Full workspace gates are required in the follow-on implementation
package that changes kernel behavior.
