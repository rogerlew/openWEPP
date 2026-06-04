# Verification Agent A

Status: complete
Evidence mode: Static + Ran

## Verification

Static:
- Verified `Cargo.toml` registers `hphys0286_layer_retention_wb18_wb17_contract`.
- Verified the focused test asserts both aggregate conservation and layer cap redistribution.

Ran:
- `cargo test --test hphys0286_layer_retention_wb18_wb17_contract -- --nocapture`: passed.
- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`: passed.
- `cargo test --workspace`: passed.
