# Review Disposition

Status: **COMPLETE**.

| Finding | Disposition | Verification |
|---|---|---|
| Producer allocation authority unclear | accepted | Helper renamed/commented with `SC-RUNOFFPART-001#INV-RUNOFFPART-022`; producer allocation test passes. |
| Dry-day source-shape returned raw depths | accepted | `dc01_surface_shape_returns_zero_weights_without_runoff` passes. |
| H2637 residual class undispositioned | accepted | Ignored H2637 evidence passes with `with_routed_melt=0`, `without_routed_melt=6`. |
| Pending artifacts | accepted | Artifact set completed. |
| H2637 nextest ignored evidence harness issue | accepted/follow-up | D12 evidence uses passing `cargo test --ignored`; full nextest delegated. |
| Non-finite malformed-limb coverage gap | accepted | `dc01_surface_shape_rejects_nonfinite_inputs` passes. |
| Package write-set/boundary mismatch | accepted | Package write set amended for the producer, executor, erosion helper, and runner surfaces required by the shared D12 source-shape path; D13 erosion promotion remains explicitly excluded. |
| 3000-line touched test module | accepted | DC01 tests moved to `direct_runtime_dc01.rs`; `direct_runtime.rs` is now `2988` lines. |
| Full nextest size-layout failure | accepted | Duplicate hourly routed-melt storage removed from snow-coupling state/projection and downstream operand vector boxed; size guard passes with `DirectDayFrame=15328`. |
