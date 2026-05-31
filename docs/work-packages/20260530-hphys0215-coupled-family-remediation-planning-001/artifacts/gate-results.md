# HPHYS0215 Gate Results

Status: completed
Evidence mode: Ran

## Required gates
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`

## Results
Run root: `/tmp/hphys0215_20260531T041655Z/`

1. `cargo fmt --check`: pass
   - stdout: `/tmp/hphys0215_20260531T041655Z/gates/cargo_fmt_check.out`
   - stderr: `/tmp/hphys0215_20260531T041655Z/gates/cargo_fmt_check.err`
2. `cargo clippy --workspace --all-targets -- -D warnings`: pass
   - stdout:
     `/tmp/hphys0215_20260531T041655Z/gates/cargo_clippy_workspace.out`
   - stderr:
     `/tmp/hphys0215_20260531T041655Z/gates/cargo_clippy_workspace.err`
3. `cargo test --workspace`: pass
   - stdout: `/tmp/hphys0215_20260531T041655Z/gates/cargo_test_workspace.out`
   - stderr: `/tmp/hphys0215_20260531T041655Z/gates/cargo_test_workspace.err`
4. `cargo deny check`: pass (warnings only)
   - stdout: `/tmp/hphys0215_20260531T041655Z/gates/cargo_deny_check.out`
   - stderr: `/tmp/hphys0215_20260531T041655Z/gates/cargo_deny_check.err`
   - warnings observed:
     - `duplicate` (`twox-hash` versions in lockfile dependency graph)
     - `license-not-encountered` entries for `ISC` and `Unicode-DFS-2016`
