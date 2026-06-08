# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-06-08

## Ran
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test -p openwepp-runner --tests`
4. `cargo test --workspace` (captured to log and validated by fail-marker scan)
5. `cargo deny check`

## Result
- `cargo fmt --check`: pass (`exit_code=0`)
- `cargo clippy --workspace --all-targets -- -D warnings`: pass (`exit_code=0`)
- `cargo test -p openwepp-runner --tests`: pass (`exit_code=0`)
- `cargo test --workspace`: pass (`WORKSPACE_STATUS:PASS` from `/tmp/refactor006_workspace_test.log` fail-marker scan)
- `cargo deny check`: pass (`advisories ok, bans ok, licenses ok, sources ok`; warnings only)

Warning capture (`cargo deny check`):
- duplicate lock entries:
	- `getrandom`
	- `hashbrown`
	- `twox-hash`
- unmatched allowlist entries:
	- `ISC`
	- `Unicode-DFS-2016`
