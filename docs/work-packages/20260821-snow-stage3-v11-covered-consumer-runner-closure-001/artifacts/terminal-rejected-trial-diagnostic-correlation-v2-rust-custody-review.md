# Terminal rejected-trial diagnostic correlation V2 Rust/custody review

Evidence class: `Static`

Reviewed authority SHA-256:
`f4a7ff15127fdfd5068f16126f440a57a25026b44a5c610f175dfab30417cc5c`.
The hash matched the requested frozen authority. `HEAD` and `origin/main` both
resolved to `21c4a1983d667dadac9c07ff3f4340255487256e`. The authority and package
update were pre-existing uncommitted work; this review changed neither source
nor authority and did not read or communicate with the other reviewer.

## Static evidence

- `snow_stage3_v11_terminal_execution.rs` owns the caller-local provider
  closure. Each invocation receives role, attempt and coupling ordinal and
  owns the complete `CoveredCarrierPhaseResultV1` before returning only its
  `CoveredTerminalTrialTransitionV1`. This is a valid location for a local,
  append-only capture arena; no global, thread-local or callback is needed.
- `stage3_solver/evaluation.rs` performs the bounded coupling loop, retains the
  selected receipt/flux, and is the first layer that can identify which
  provider call supplied the converged trial. It then forwards only flux and
  ending joint to `terminal_event.rs`.
- `stage3_solver/terminal_event.rs` owns the exact full/half ordering, refined
  ledger composition, scaled error, retry decision and eventual
  `BelowCarrierDomain` floor return. A fixed-size selected key returned beside
  each trial is therefore sufficient to join the pair decision to the
  caller-owned arena without moving complete receipts through the solver.
- The existing covered entry point and provider alias are `pub(crate)`, while
  the adaptive solver is `pub(super)`. The crate already has ordinary
  `#[cfg(test)]` unit modules. A crate-private unit diagnostic can see
  crate-private items when explicitly exposed within the crate; no public
  re-export, feature, environment selector or runtime flag is required.
- The existing path is value-returning and already propagates physical errors
  with typed `Result`. The proposed evidence constructors can remain
  infallible values, while resolution, canonical encoding, digest checking,
  assertions and deliberate artifact failure occur after the physical return.

## Findings

### Major

1. **The implementation intent must name the full private forwarding chain.**
   The correlation token cannot travel directly from
   `snow_stage3_v11_terminal_execution.rs` to `terminal_event.rs`. The actual
   path crosses the private provider type/context in
   `runoff_reconciliation.rs`, `stage3_solver.rs`,
   `stage3_solver/support.rs`, and `stage3_solver/evaluation.rs`. The
   authority's prospective boundary is sufficient because it permits the
   existing private terminal solver, existing covered provider/caller module,
   and private types needed between them, and separately requires exact files
   and implementation intent before edits. Treating the boundary as only the
   two endpoint files would be nonconforming.

2. **Selection must be emitted by the coupling owner, not inferred in the
   arena.** `evaluation.rs`, not the outer provider closure, decides which
   coupling receipt is selected. The implementation must carry that selected
   coupling ordinal/key out with the converged flux/joint and post-return
   resolution must mark exactly that arena record selected. Ending-joint
   lookup, last-entry selection, or approximate value matching would violate
   the authority and permit discarded-iteration substitution.

### Minor

1. **Sealing must prevent downstream/custom implementations.** The mode trait
   and both implementations should remain in a private module (or use a
   private sealing supertrait); no generic mode parameter may appear in a
   `pub` or `pub(crate)` production entry-point signature. Existing wrappers
   should keep their signatures and monomorphize `NoEvidence` internally.

2. **Capture-only payload work must remain under the unit-test boundary.** The
   normal build may contain only the private generic core, sealed mode contract,
   zero-sized `NoEvidence`, and fixed-size plumbing required by that
   monomorphization. Complete records, arena resolution, serialization and
   artifact-writing code must be `#[cfg(test)]` and crate-private. An external
   integration test may source-inspect this boundary but cannot execute it.

3. **Physical-return retention must precede every fallible diagnostic act.**
   Canonical receipt adapters whose existing serializers return `Result`,
   allocation-heavy reconstruction, digest resolution, injected failures and
   artifact I/O belong after the unchanged physical result/error and beginning
   witness bytes have been retained. Inside the transaction, capture may only
   move/clone already-produced values, append locally and construct fixed-size
   identities; it must not map any diagnostic condition into the physical
   `Result` or introduce `catch_unwind`.

## API and noninterference disposition

The architecture can preserve the existing public and production-default API.
`NoEvidence` can be zero-sized and allocation-free, with no arena or runtime
branch. `CaptureEvidence` is reachable through a crate unit test compiled with
the library's own `cfg(test)` and need not be visible to integration crates.
The local arena and fixed-size key have single-call custody and cannot publish
or mutate model state. Pair traces can be constructed from the already chosen
physical branch without participating in acceptance, controller, root or
floor decisions. Retiring observer panic injection and placing all deliberate
failure after return removes the prior unwind/control-flow defect.

The allowed write boundary is sufficient only with Major finding 1 enforced
in the prospective exact-file intent. No public/runtime exposure or new Cargo
feature is needed.

## Recommendation

**GO-to-evidence**

This recommendation authorizes only the bounded correlation seam and evidence
run described by the frozen V2 authority. It does not authorize SnowEnergy
v21, temporal-operator, Batch V2, event acceptance, receiver, restart, runner,
Child-3 or cutover implementation.
