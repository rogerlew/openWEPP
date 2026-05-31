# AUTH09 Gate Results

Status: completed  
Evidence mode: Ran

Ran:
1. `cargo fmt --check`
   - result: pass (exit 0)
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass (exit 0)
3. `cargo test --workspace`
   - result: pass (exit 0)
4. `cargo deny check`
   - result: pass (exit 0)
   - notes: duplicate crate and unmatched-license-allowance warnings reported;
     no deny gate failures (`advisories ok, bans ok, licenses ok, sources ok`).
