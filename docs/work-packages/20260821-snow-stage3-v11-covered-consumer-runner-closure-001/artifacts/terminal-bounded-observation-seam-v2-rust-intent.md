# Terminal bounded observation seam V2 exact Rust intent

Status: `CANDIDATE / NO SOURCE AUTHORITY`

## Sealed architecture

`TerminalEvidenceMode<J>` is crate-private and sealed. It has `type State` and
`new_state`, plus hook methods taking only borrowed hook structs. `NoEvidence`
is an uninhabited zero-sized type, uses `State = ()`, and every method is an
empty `#[inline(always)]` function. Hook construction performs only references,
integer/f64 copies and enum copies: no `Clone`, allocation, hash, formatting or
conversion occurs before dispatch. Existing wrappers instantiate `NoEvidence`
and keep their exact signatures. A `cfg(test)` entry instantiates
`CaptureEvidence` and returns `(physical_result, evidence)`; all validation,
hashing and serialization occur after that return. There is no callback,
runtime flag, feature, environment selector, global, thread-local, channel,
mutex, panic interception or `catch_unwind`.

The trait is parameterized by the solver joint type so generic solver tests
remain valid. `CaptureEvidence` is implemented only for the concrete
`CoveredTerminalJointTrialStateV1`. Borrowed hooks are fully enumerated:

- `CarrierHook<'a>`: request, child identity, carrier result, call ordinal,
  arena ordinal and typed ingress projection references.
- `IterationHook<'a, J>`: key scalars, flux, preview, incoming/outgoing hint,
  four comparisons, convergence and `&J` beginning/ending joints.
- `SelectionHook<'a>`: prefix/role/attempt/support, the inclusive arena range,
  selected iteration ordinal and selected carrier ordinal.
- `SelectedTrialHook<'a, J>`: position/role, state and ledger references,
  selection ordinal, `&J` beginning and hydrology-complete ending joints.
- `PairHook<'a, J>`: the three selected-trial ordinals, five exact error
  operands/results, maximum, first-equal winner, decision and duration.
- `AdmissionHook`: proposed interval, duration, required half duration,
  600-ms minimum, outcome, and provider counts before/after.

Capture converts live values field-by-field into owned primitives. No DTO
contains a live crate type and no `PartialEq` requirement is added to one.

## Closed DTO graph

`DiagnosticF64 { bits: u64, finite: bool }`; `SupportEvidence { start_ns:
u128, end_ns: u128 }`; `Digest32([u8; 32])`; identity wrappers contain owned
UTF-8 bytes. `TerminalStateEvidence` has exactly ice, liquid and cold-content.
`TerminalLedgerEvidence` has exactly the fourteen live ledger fields in source
order. `JointEvidence` contains its canonical digest plus seven explicitly
named owner byte vectors: vegetation, snow, land-surface-energy, hydrology,
biogeochemistry, soil-thermal and surface-liquid.

`CarrierPhaseEvidence` contains primitive key/identity/support fields and
named projections for boundary, beginning/ending joint, optional snow-soil
receipt, ordered precipitation receipt digests, envelope transaction,
lower-boundary destinations, carrier-source destinations, covered-LSE
destinations, soil configuration identity, soil top-boundary credit scalars,
and WB14 child digest/replay bytes. It never embeds or serializes
`CoveredCarrierPhaseResultV1`.

`CouplingIterationEvidence` owns flux, preview, hints, four comparison bits and
joint evidence. `CouplingSelectionEvidence` owns an inclusive
`Range<usize>`-equivalent pair of ordinals, selected iteration/carrier ordinals
and convergence. `SelectedTerminalTrialEvidence` owns beginning and ending
state, ledger, selection, complete beginning joint and hydrology-complete
ending joint. `PairDecisionEvidence` owns exactly coarse/fine-1/fine-2 selected
trials, `[PairComponentErrorEvidence; 5]`, maximum, winner and decision.

`TrialAdmissionEvidence` is a separate later record and contains proposed
support/duration, required half duration, minimum duration, typed outcome and
provider counts. `RejectedPrefixEvidence` owns `Vec` arenas for carriers,
iterations, selections, selected trials, pairs and admissions, exactly
`[ZeroIngressEvidence; 3]`, and explicit before/after snapshots. It uses an
explicit constructor; it does not derive `Default`.

## Exact ingress witnesses

`ZeroIngressEvidence { source, inspected_count: usize, matches:
Vec<TerminalIngressMatch>, numeric_total: Option<DiagnosticF64> }` is validated
by `matches.is_empty()` and, where a numeric operand exists, positive-zero
bits. The three records are distinct:

1. hydrology terminal-liquid supply records every selected trial's
   `TerminalFluxIntegral::external_liquid_kg_m2`, its exact source expression
   `hourly.rain_m * 1000.0 * duration_seconds / support_seconds`, and an empty
   terminal-liquid source-tag match list. Rain mass is not relabeled terminal
   supply, so `numeric_total` is `None`.
2. WB14 terminal-liquid credit walks every typed `DirectTileGroundIngress`
   parcel at `DirectSurfaceLiquidIngressInput` construction and records every
   `DirectSurfaceLiquidParcelKind::TerminalReceiver` match destined for WB14.
3. surface-liquid terminal ingress uses the same exhaustive typed walk and
   records every `TerminalReceiver` match entering surface-liquid custody.

The latter two may have positive non-terminal rain/canopy/runon mass; absence
is established by the empty exact-kind match vector, not by claiming total
ingress is zero.

## Explicit noninterference snapshot

No map keyed by arbitrary names is allowed. `NoninterferenceSnapshot` contains
before/after values for exactly: seven named owner canonical byte vectors;
joint digest; coupled-clock cursor/time/ordinal fields; provider-call count;
ordered candidate joint digests; ordered carrier keys; ordered pending
terminal parcel IDs/kinds/mass bits; and ordered lane IDs with every
`DirectSnowStage3PersistentState` scalar converted field-by-field. Equality is
implemented explicitly per named location and reports the first differing
location. Capture-only evidence vectors are excluded from physical snapshots.
