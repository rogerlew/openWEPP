# PERFARRAY01 Bit-Identity Evidence

Evidence class: Ran + Static.

## Stage A

Ran:

```text
cargo test -p openwepp-kernel-contract
```

The focused Stage A tests prove exact logical-map identity for the contract
shell:

- `array_hot_state_round_trips_logical_surfaces`;
- `array_writeback_accept_matches_logical_writeback`;
- `array_writeback_reject_matches_logical_message_class_and_subject`;
- `array_writeback_apply_exports_same_maps_as_logical_apply`.

The final test applies the same logical payload through the current
`evaluate_kernel_writeback` + `apply_kernel_writeback` path and through the new
id-backed `evaluate_array_writeback` + `apply_array_writeback` path. Exported
state and flux maps are required to be equal.

## Default Production Path

Static: no scheduler, runner, CLI, kernel, HBP/parquet writer, or publication
path calls the new array shell. The default production path is unchanged by
write-set.

## Stage B

Not run. No H2637, OFE ladder, HBP/loss/wat/plot, or pass-parquet identity
evidence was produced because the existing production seam cannot host a valid
array-authoritative pilot without per-day logical export or logical + array
dual-write.

This package therefore does not satisfy the Stage B bit-identity acceptance
gate.
