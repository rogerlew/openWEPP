# WSHEDIMPL27 Gate Results

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-27

## Static
- Required gate set for kernel-affecting package:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Ran
1. `cargo fmt --check`
   - Initial result: fail (attribute wrapping diff in
     `crates/openwepp-watershed-orchestrator/src/lib.rs`).
   - Remediation: ran `cargo fmt`.
   - Final result: pass.
2. `cargo clippy --workspace --all-targets -- -D warnings`  
   - Result: pass.
3. `cargo test --workspace -q`
   - Result: pass.
4. `cargo deny check`  
   - Result: pass.
   - Notes: non-failing duplicate/license warnings persisted for
     `getrandom`, `hashbrown`, `twox-hash`, `ISC`, and `Unicode-DFS-2016`.
