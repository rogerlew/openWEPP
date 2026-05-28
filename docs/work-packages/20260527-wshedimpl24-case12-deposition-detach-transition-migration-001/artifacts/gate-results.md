# WSHEDIMPL24 Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Ran
1. `cargo fmt --check`  
   Result: pass.
2. `cargo clippy --workspace --all-targets -- -D warnings`  
   Result: pass.
3. `cargo test --workspace`  
   Result: pass (workspace-wide integration/unit/doc tests).
4. `cargo deny check`  
   Result: pass (warnings-only duplicate crates and unmatched license
   allowances in `deny.toml`; no deny failures).
5. `cargo test --test ws11_channel_routing_physics_equivalence_contract`  
   Result: pass (`24 passed; 0 failed`).
