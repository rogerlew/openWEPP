# hillstab05-preimplementation-contract-gate

Status: complete  
Evidence mode: Ran

## Commands
```bash
cargo test --test infile_slope_parser_contract compatibility_mode_accepts_near_endpoint_terminal_distance
cargo test --test parser_runtime_seam_integration slope_runtime_surface_compatibility_floor_accepts_non_positive_avgslp_projection
```

## Gate Result (Pre-Code-Edit)
- Expected: fail before production code edits.
- Observed:
  - endpoint tolerance vector failed under pre-fix parser behavior
    (`0.9996` terminal distance rejected against strict `1e-6` tolerance),
  - runtime seam vector failed before code edits because the new slope-runtime
    options API was not yet available.

## Conclusion
- Contract-first pre-implementation gate produced the required failing signal;
  production closure edits then proceeded.
