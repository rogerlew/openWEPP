# Temporary Allowlist And Next Work

Evidence class: Static routing decision.

## Temporary Allowlist

These survivor classes may remain until their replacement slices exist:

| Allowlist class | Allowed temporarily because | Exit condition |
| --- | --- | --- |
| `IO` runtime-input projection adapters | Some parser/intake paths still encode or validate symbol-keyed surfaces. | Typed input projections feed typed consumers without symbol emission. |
| `KB` hydrology request/writeback boundary | Hydrology phase code still accepts `HillslopeKernelRequest` and returns `KernelWritebackPayload`. | Typed phase context and typed result/mutation APIs cover each phase family. |
| `TRACE` diagnostics | Frost, HPHYS, frame roundtrip, and shadow diagnostics still read request/surface state. | Typed event payloads replace request-carried symbol access. |
| `PUB` WB13/publication/audit support | Some audit and scheduler-publication helpers still format compatibility rows. | Typed publication/audit streams are the only publication authority. |
| `TEST` scheduler/symbol tests | They preserve coverage until equivalent typed tests exist. | Typed tests replace semantic coverage; scheduler-only tests delete. |

No allowlist class may be used to reintroduce a public compatibility selector or
production fallback. The selector absence scan is already clean.

## Deletion Targets

Delete after replacement, not before:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/perfdeep02_frame_roundtrip.rs`
- scheduler-only tests in `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/`
  and `crates/openwepp-runner/src/hillslope/tests03/publication/`

## Recommended Follow-On Package Sequence

1. **Typed diagnostic/event payloads.**
   Move HPHYS, frost trace, shadow/audit, and reconciliation diagnostics from
   `HillslopeKernelRequest` or `HillslopeWritebackSurface` reads into typed event
   structs emitted by direct runtime code. This is the lowest-risk first slice
   because it does not change physics.
2. **Typed phase context and typed result shell.**
   Add a typed kernel-boundary crate/module with phase context and phase result
   shapes. Initially support one hydrology family and prove output identity.
3. **Hydrology phase-family migrations.**
   Move phase families from symbol request/writeback to typed inputs/results in
   coherent groups:
   - runoff/infiltration/reconciliation;
   - frost/coupling;
   - lateral drainage/subsurface;
   - storage/erosion and EROD13/14/19;
   - plant/decomposition transitions.
4. **Typed publication/audit stream.**
   Replace WB13 scheduler rows and scheduler publication support with typed
   publication/audit events from direct state.
5. **Test migration.**
   Replace scheduler-era tests with typed-boundary tests, then delete
   scheduler-only fixtures.
6. **Boundary deletion.**
   Delete `scheduler.rs`, `day_frame.rs`, carrier exports, and non-allowlisted
   symbol helpers. Rescan `BoundarySymbol`/`BoundaryValue` and document any true
   I/O adapter survivors.

## First Actionable Slice

Start with typed diagnostics/trace support:

- it is behavior-preserving and not physics-changing;
- it shrinks the reason `HillslopeKernelRequest` must be threaded through helper
  code;
- it unblocks later deletion of `day_frame.rs` shadow/roundtrip support;
- it gives a narrow output-identity gate before touching phase math.

Acceptance for that slice should include source-scan count reduction, direct
H2637/multi-OFE/Wave-2 identity, and focused trace fixture equivalence.
