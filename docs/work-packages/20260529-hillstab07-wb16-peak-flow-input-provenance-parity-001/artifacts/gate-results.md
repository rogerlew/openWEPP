# HILLSTAB07 Gate Results

Status: complete  
Evidence mode: Ran

## Command Results

1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test --workspace`
   - result: pass
4. `cargo deny check`
   - result: pass (warnings only: duplicate crate entries and unmatched
     license allowlist entries in `deny.toml`; no deny failure)

## Notes

- An intermediate clippy failure (`struct_excessive_bools`) was resolved by
  adding a local allow on manifest-only provenance struct
  `HillslopeExecutionProvenance`.
- An intermediate targeted-test compile failure (unresolved `serde_json`) was
  resolved by switching to manifest-string assertions.
