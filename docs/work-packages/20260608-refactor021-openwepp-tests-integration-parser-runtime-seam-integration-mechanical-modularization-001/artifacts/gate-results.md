# Gate Results

Status: complete
Evidence mode: Static/Ran

Static:
- Package scope and validation contract captured in `package.md`.
- Required gates and pass criteria are recorded below.

Ran:
- 2026-06-08T23:39:12Z: `cargo fmt --check` (exit 0)
- 2026-06-08T23:39:12Z: `cargo clippy --workspace --all-targets -- -D warnings` (pass)
- 2026-06-08T23:39:12Z: `cargo test -p openwepp --test parser_runtime_seam_integration` (`49` passed, `0` failed)
- 2026-06-08T23:39:12Z: `cargo test --workspace` (exit 0, pass; trailing fail-closed source-line checks remain scoped to source-line provenance families)
- 2026-06-08T23:39:12Z: `cargo deny check` (`advisories ok, bans ok, licenses ok, sources ok`)

Required Gates:
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p openwepp --test parser_runtime_seam_integration`
- `cargo test --workspace`
- `cargo deny check`
