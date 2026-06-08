# Gate Results

Status: complete
Evidence mode: Ran

| Gate | Result | Exit code | Artifact |
|---|---|---:|---|
| `cargo fmt --check` | PASS | 0 | `gate-cargo-fmt-check.md` |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | 0 | `gate-cargo-clippy.md` |
| `cargo test --workspace` | PASS | 0 | `gate-cargo-test.md` |
| `cargo deny check` | PASS | 0 | `gate-cargo-deny-check.md` |
| doc-path integrity | PASS | 0 | `doc-path-integrity.md` |

Notes:
- `cargo deny check` emitted duplicate-crate and unmatched-license-allowance warnings, then reported `advisories ok, bans ok, licenses ok, sources ok` and exited 0.
