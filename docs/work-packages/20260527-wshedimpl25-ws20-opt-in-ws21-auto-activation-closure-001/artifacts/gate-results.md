# WSHEDIMPL25 Gate Results

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
   - Result: pass.
2. `cargo clippy --workspace --all-targets -- -D warnings`  
   - Result: pass.
3. `cargo test --workspace -q`  
   - Result: pass (no failing tests in workspace execution).
4. `cargo deny check`  
   - Result: pass.
   - Notes: reported non-failing warnings for duplicate crate versions
     (`getrandom`, `hashbrown`, `twox-hash`) and unmatched license allowances
     (`ISC`, `Unicode-DFS-2016`).
