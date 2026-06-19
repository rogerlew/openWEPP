# PERFDEEP01 Roundtrip Identity

Evidence: Ran.

## Executed Validation

Command:

```bash
cargo test -p openwepp-hillslope-orchestrator perfdeep01_frame_ -- --nocapture
```

Observed result:

- 3 tests passed, 0 failed.
- Test names:
  - `perfdeep01_frame_seed_flush_roundtrip_is_bit_identical`
  - `perfdeep01_frame_captures_io_edge_scalars_and_mofe_arrays`
  - `perfdeep01_frame_borrows_climate_forcing_series_without_copy`

## Identity Assertion Semantics

Primary identity gate is `assert_shadow_roundtrip_bits` over `HillslopeWritebackSurface`:

- Seeds frame from source surface.
- Flushes frame back to logical maps.
- Compares state and flux symbol sets and each scalar bit pattern.
- Fails on first mismatch with symbol-scoped diagnostic including expected/observed bits.

This is strict bit identity for represented symbols; no tolerance-based compare is used.

## Stage-0 Coverage Notes

- Roundtrip harness is currently exercised with a H2637-like warm-rain fixture in orchestrator tests.
- Runtime authority path is unchanged; scaffold is shadow/test-harness only for Stage-0.
