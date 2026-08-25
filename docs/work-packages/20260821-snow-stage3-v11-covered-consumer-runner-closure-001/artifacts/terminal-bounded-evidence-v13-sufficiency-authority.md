# Terminal bounded evidence V13 sufficiency and noninterference qualification

Status: `REVIEW CANDIDATE / SOURCE EDITS FORBIDDEN`.

Base and origin: `ce58080c06f604ec1c5579db83517c8495c8514c`.
Last qualified physical implementation: `43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.

## Exact-head baseline

- rustfmt and diff hygiene: PASS;
- affected crate: HOLD, nextest
  `875ee602-1e45-4a2a-8010-0387f238f6cc`, 844 passed, 11 failed, 1 skipped;
- V20/V21 structural guards: PASS 5/5, nextest
  `291ab6ff-3bcc-46b8-b386-978861fc109d`.

The affected-suite failures are retained baseline facts, not V13 regressions or
passing claims. `BelowCarrierDomain` remains authoritative.

## Findings

- `CHILD1-TERM-EVIDENCE-011`: V12 called its implementation an eleven-file
  source set, but commit `ce58080c` changes exactly ten Rust source files.
- `CHILD1-TERM-EVIDENCE-012`: the omitted eleventh owner is
  `src/v11_covered/carrier_phase.rs`; without an owner-local projection V12
  cannot retain complete carrier/component receipt identities without either
  private access leakage or whole-result capture.
- `CHILD1-TERM-EVIDENCE-013`: V12 mutates the last floor-admission tuple after
  return when provider observations are merged. The provider epoch and floor
  decision must instead form one immutable post-return record.
- `CHILD1-TERM-EVIDENCE-014`: V12 provider observations omit coupling iteration,
  beginning/ending joint identities, selected iteration, complete selected
  trials, full ledgers, five component errors and lineage-bound ingress
  witnesses; it is insufficient for candidate-v21 effectivity/conservation.
- `CHILD1-TERM-EVIDENCE-015`: V12 performs receipt scans in the generic carrier
  closure even for `NoEvidence`. Monomorphization is not a custody boundary;
  capture-only scans must be unreachable through the `NoEvidence` trait path.

## Frozen current implementation

The exact binary patch of commit `ce58080c` has SHA-256
`006d9885ca6ff6c6020ad9373d1e78207e0a8dc7c4bd2082e5552341d4a60456`.
Its Rust source manifest is the ten paths below:

1. `src/hydrology/03_kernel_support_00_support_helpers.rs`
2. `src/hydrology/support_helpers_mod/mod.rs`
3. `src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
4. `src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`
5. `src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`
6. `src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs`
7. `src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`
8. `src/snow_stage3_v11_attachment.rs`
9. `src/snow_stage3_v11_terminal_execution.rs`
10. `src/v9_real_consumer_shadow_wb14_tests.rs`

All paths are relative to `crates/openwepp-hillslope-orchestrator`.

## Proposed exact source write set

V13 uses the ten paths above plus exactly:

11. `src/v11_covered/carrier_phase.rs`

No other crate source, workspace Cargo file, production signature or frozen
V3--V12 artifact may change.

## Exact expansion

The test-only DTO graph uses named structs and fixed arrays, not positional
tuples. `ProviderObservationV13` contains support, role, attempt, coupling
iteration, beginning/ending joint digests, success, owner-local projected
carrier/component receipt identities and three separately typed ingress
witnesses. The owner-local projection is constructed in `carrier_phase.rs`
under `cfg(test)` from the live result; it never serializes the whole carrier.

`SelectedTrialV13` contains role/position, attempt, selected coupling iteration,
beginning/ending state, beginning/ending joint digest, complete ledger and the
selected carrier projection. `RejectedPairV13` contains exactly FULL-or-RETRY,
HALF_1 and HALF_2 trials plus `[PairComponentErrorV13; 5]`. Component order is
complete energy, external liquid, melt, refrozen and cold-energy change. Each
record retains coarse/refined bits, refined-minus-coarse bits, scale bits and
normalized-error bits. Selection is first-bitwise-equal among the exact
component fold.

The provider buffer remains separately owned during physical evaluation. After
the physical return, `CaptureEvidence` consumes it once to construct an
immutable `ProviderEpochFloorV13` containing call count before/after, the
distinct 0.9375/0.46875/0.6 admission and its outcome. No earlier evidence
record is mutated. Capture-only carrier projection and receipt scanning are
trait operations whose `NoEvidence` implementation receives no projected value
and performs no scan; the production call path cannot name the test-only DTO.

`NoninterferenceSnapshotV13` explicitly retains before/after parent, consumer,
clock and Stage-3 state identities. Validators require exactly one relevant
1.875-second rejected pair; exact role/position mapping and half state/joint
joins; exact five-component binary64 arithmetic; observed complete-energy
bits; distinct zero-call floor chronology; all successful provider calls of at
least 600 ms; three lineage-bound zero-ingress witnesses; and bitwise physical
result plus caller-state equality between independent NoEvidence and Capture
runs.

Focused negative tests independently reject omission, duplicate, substitution,
reorder and wrong-joint evidence. The existing success-named terminal test is
not relabelled. V13 authorizes only this evidence expansion after two fresh GO
reviews; it does not authorize final reviews or temporal/event implementation.
