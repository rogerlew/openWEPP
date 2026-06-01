# AUTH11 Gate Results

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
5. `bash tools/release/check_authority_suite_antievasion.sh --base-ref 0dc1788 --head-ref HEAD`
   - result: pass (exit 0)
