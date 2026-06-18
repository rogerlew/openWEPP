# PERFARRAY02 Structural Proofs

Evidence: Static + Ran.

## Proof 1: No Full `BTreeMap` Export At Kernel Request Seam

Result: PASS for the scoped WB11 runoff pilot.

Static: the pilot seeds `ArrayHotState` before constructing the kernel request, then
constructs empty logical maps and passes `Some(&hot_state)`:

- `scheduler.rs:1644` seeds the array from the current boundary surface;
- `scheduler.rs:1665` creates `empty_state_surface`;
- `scheduler.rs:1666` creates `empty_flux_surface`;
- `scheduler.rs:1667` calls `with_transition_context_and_indexed_array`;
- `scheduler.rs:1673` and `scheduler.rs:1674` pass the empty maps;
- `scheduler.rs:1676` passes `Some(&hot_state)`.

The array export is after `apply_array_writeback`, for downstream non-pilot phases and
publication:

- `scheduler.rs:1856` applies array writeback;
- `scheduler.rs:1878` exports post-apply array state.

Ran: `perf record` on OFE5 pilot captured `3477` samples, `0` lost samples, event
count `113536551953`. Kernel symbols were restricted, but user-space symbols resolved.

Relevant `perf report --stdio` samples:

```text
8.02%  ArrayHotState::from_btreemap_surfaces
1.17%  ArrayHotState::export_btreemap_surfaces
1.93%  ArrayWritebackPayload::from_logical_payload (via resolve_logical_fields)
0.15%  Wb11HydrologyKernel::run_hillslope_phase
```

Interpretation: perf confirms the boundary seed/export functions are present and costly,
and static branch structure proves they are outside the kernel request/run segment.

## Proof 2: No Normal-Path Logical + Array Dual-Write For The Piloted Phase

Result: PASS for the scoped WB11 runoff pilot.

Static: when `array_payload` is `Some`, the scheduler uses array evaluation/apply. The
logical apply path is the `else` branch:

- `scheduler.rs:1724` creates `Some(array_payload)` only for `run_array_pilot`;
- `scheduler.rs:1783` calls `evaluate_array_writeback`;
- `scheduler.rs:1856` calls `apply_array_writeback`;
- `scheduler.rs:1923` is the `else` branch that calls `apply_kernel_writeback`;
- `scheduler.rs:1946` suppresses indexed mirror sync when `array_payload.is_some()`.

Ran: the same OFE5 perf report shows both writeback functions globally because non-runoff
phases still use the normal path:

```text
0.06%  apply_kernel_writeback
0.03%  apply_array_writeback
0.03%  evaluate_array_writeback
```

Interpretation: global `apply_kernel_writeback` samples are expected for other phases.
The piloted runoff phase does not dual-write because the static branch makes logical
apply and indexed mirror sync mutually exclusive with `array_payload.is_some()`.
