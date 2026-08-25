# Terminal rejected-pair evidence-correlation seam authority V2

Status: `IN REVIEW / EVIDENCE-ONLY / NO SOURCE IMPLEMENTATION AUTHORITY`

Base: `21c4a1983d667dadac9c07ff3f4340255487256e`

Objective: authorize only the minimum internal data path needed to correlate
already-computed upstream carrier receipts with the downstream terminal
full/half pair decision. This authority permits observation of unchanged
execution. It does not authorize a temporal operator, Batch V2 wiring, event
acceptance, terminal-liquid consumption, model-state change, output change,
restart work, or any SnowEnergy v21 production implementation.

## Exact source boundary

The present provider closure in
`snow_stage3_v11_terminal_execution.rs` owns each complete
`CoveredCarrierPhaseResultV1` and indexes it by ending-joint digest. The
`solve_terminal_enthalpy_event` layer in `terminal_event.rs` later owns the
selected full, half-1 and half-2 aggregate trials, their ending joints, the
refined composition, scaled error and decision. Neither layer alone owns the
complete rejected-pair proof.

This V2 authority permits those two internal functions and only the private
types needed between them to become generic over a sealed compile-time
`TerminalEvidenceMode`. The internal core returns two values: the unchanged
physical result and mode-owned diagnostic evidence. The caller-local provider
closure retains a local evidence arena and resolves returned fixed-size keys
only after the physical core has returned.

Conceptually, the permitted shapes are:

```text
TerminalTrialOutcome<J, E> { physical: TerminalTrial, ending_joint: J, evidence: E }
TerminalPairOutcome<J, E> {
    physical_result: Result<(DirectSnowTerminalEventResult, J), TerminalError>,
    diagnostic: E,
}
```

The names may follow existing private module conventions, but their custody
and cardinality may not change. The existing production/default wrapper
instantiates sealed `NoEvidence`; the package-owned diagnostic unit path
instantiates sealed `CaptureEvidence`. `NoEvidence` must be zero-sized and
must not allocate or retain an arena. Neither mode is public or configurable
at runtime.

The sealed mode has associated fixed-size carrier token and pair-trace types.
Its construction methods are total ordinary value functions: they do not
return `Result`, invoke a closure or trait object, call user code, publish
output, mutate global or thread-local storage, translate an error, inspect a
future decision, or alter acceptance/controller/root selection. There is no
callback, observer, hook, thread-local/global recorder, or `catch_unwind`
inside the physical transaction.

## Canonical wire foundation

Every V2 record begins with its exact ASCII schema tag framed as `u32` byte
length plus bytes, followed by `schema_version = u32(2)`, then the listed
fields in listed order. Unsigned integers are fixed-width little-endian.
Booleans are one byte `0` or `1`. Floating values are their raw IEEE-754
little-endian bits and must already have passed the existing finite/domain
guards. A digest is exactly 32 bytes. A time support is signed start nanoseconds
then signed end nanoseconds. An option is a one-byte presence tag followed by
its value when present. A sequence is `u64` length followed by ordered items.
A map is encoded as that sequence in the canonical key order stated by its
own released schema. Closed enums use the explicit tags in this document.

Existing typed receipts are embedded as `(schema tag, schema version,
canonical digest, canonical byte length, canonical bytes)`. A receipt without
a released canonical byte encoding is represented by a closed V2 adapter
record containing every typed field in declaration order; the adapter schema
and field list must be frozen in the implementation-intent diff before source
editing. Digest-only substitution for a required complete receipt is
forbidden. Every record digest is SHA-256 of all preceding fields, and the
stored digest is appended last. Independent reconstruction must reject unknown
tags/versions, noncanonical order, duplicate keys, missing items, trailing
bytes, or a digest mismatch.

Closed role tags are `full=1`, `half_1=2`, `half_2=3`; root/event-localization
calls, if retained for the rejected-prefix proof, use `root=4`, `retry=5`, and
`event_root=6`. Closed decision tags are `accept=1`, `reject_retry=2`,
`below_carrier_domain=3`, `step_underflow=4`, `rejection_limit=5`, and
`other_typed_terminal_error=6`. No string-derived enum tag is allowed.

## Carrier evaluation record

`TerminalCarrierEvaluationRecordV2` has schema tag
`openwepp.terminal-carrier-evaluation-v2` and contains, in order:

1. parent transaction digest, prefix identity digest, exact support, role,
   attempt ordinal, coupling-iteration ordinal and provider-call ordinal;
2. lane/batch identity, beginning owner-set digest, complete beginning-joint
   digest and complete canonical beginning-joint bytes;
3. complete forcing digest and receipt, topology digest and receipt, and LSE
   admission result/active-set tag;
4. ordered prescribed-amount receipt set, ordered endpoint/collocation
   rate/component receipt set, ordered arm-generated-amount receipt set,
   ordered snow--soil receipt set, `q_ss` receipt, hydrology receipt and WB14
   child receipt set;
5. complete physical beginning and ending snow state, component and complete
   physical ledgers, beginning and ending seven-owner-set identities, complete
   ending-joint digest and canonical bytes; and
6. selected-for-trial boolean, record digest.

There is one record for every actual provider/coupling evaluation, including
discarded nonlinear iterations. Prescribed and generated amounts occupy
different typed sets. Terminal liquid is not a generated carrier amount: the
record contains an explicit `terminal_parcel_absent=true`, zero terminal-liquid
hydrology ingress, zero terminal-liquid WB14 credit and zero surface-liquid
terminal ingress. These are witnessed fields, not inferred from a missing map.

## Evidence key and arena

`CarrierTrialEvidenceKeyV2` is fixed-size and contains schema version, prefix
digest, exact support start/end, role, attempt ordinal, selected coupling
ordinal, beginning-joint digest, ending-joint digest, carrier-phase receipt
digest, arena index `u64`, arena-record digest and key digest. It contains no
reference, pointer, owner bytes, vector or callback.

`TerminalEvidenceArenaV2` is caller-owned local memory. Its canonical index is
an append-only sequence of `(arena index, evaluation-record digest)` in actual
provider-call order followed by the complete ordered evaluation records and
arena digest. Indices are contiguous from zero; record digests are unique per
provider call; duplicate keys or records are invalid. A key resolves only when
all fixed fields equal the indexed record and the key digest reconstructs.
Cross-prefix, cross-support, cross-role, cross-attempt, cross-coupling,
cross-beginning, cross-ending, or cross-phase substitution fails closed in the
post-return diagnostic resolver.

The selected coupling iteration is the one whose complete ending joint was
returned to the terminal solver. Exactly one evaluation record for a trial is
marked selected and matches its key. All other coupling records remain ordered
and marked unselected. Selection is proved by returned value identity, never
by taking the last arena entry. Therefore a discarded iteration cannot resolve
as the selected trial.

## Selected trial record

`SelectedTerminalTrialRecordV2` has schema tag
`openwepp.selected-terminal-trial-v2` and contains prefix identity, role,
exact support, attempt ordinal, ordered provider-call record digests for that
trial, exactly one selected `CarrierTrialEvidenceKeyV2`, complete beginning
physical state/ledger, complete terminal physical state/ledger, ending-joint
digest and canonical bytes, and trial digest.

For each pair there is exactly one full record over `[t,t+h)`, one half-1
record over `[t,t+h/2)`, and one half-2 record over `[t+h/2,t+h)`. Half-2's
beginning joint and physical state must equal half-1's selected ending. Full
and half-1 begin from the same accepted prefix. Role, support and attempt
ordinal are included in both the trial and selected carrier record; mismatch
is invalid.

## Pair-decision record

`RejectedTerminalPairDecisionV2` has schema tag
`openwepp.terminal-pair-decision-v2` and contains, in order:

1. prefix identity and pair ordinal;
2. exactly one full trial digest, one half-1 trial digest and one half-2 trial
   digest, with the three complete selected trial records;
3. refined composition containing half-2 ending physical state and the exact
   componentwise sum of half-1 plus half-2 ledgers;
4. ordered scaled-error vector for ice, retained liquid, cold content,
   complete energy and unallocated energy, including each raw difference,
   absolute tolerance, relative tolerance, denominator and scaled value;
5. maximum scaled error, winning component tag, exact decision tag, current
   trial duration, and optional proposed next duration; and
6. typed terminal result/error tag and payload, pair digest.

Exactly one triple authorizes one pair decision. No trial digest may appear in
two roles or two pair ordinals. The pair support identities must form the exact
full/half partition above. The error vector and maximum are independently
reconstructed from the recorded physical values; a supplied aggregate alone
is insufficient. Accepted pairs may appear in the rejected-prefix chronology,
but the final record ending in `BelowCarrierDomain` must bind the last
admissible full/half triple that caused the controller to propose an
unsupported next pair. A pre-provider floor check has no fabricated carrier
key.

## Ordered rejected-prefix record

`TerminalRejectedPrefixRecordV2` has schema tag
`openwepp.terminal-rejected-prefix-v2` and contains parent/prefix identity,
initial complete owner/joint/clock/provider/cursor/receipt/parcel/`last_*`
digests, ordered pair-decision records, ordered non-pair root/event trial
records when present, final typed physical result bytes, final diagnostic arena,
post-return unchanged-state witness, and record digest.

The sequence order is actual call/decision order. It ends exactly in
`Stage3(TerminalNumerics(BelowCarrierDomain))`. Its validation independently
proves:

- every positive carrier/provider call support is at least `600000000 ns` and
  there are zero subminimum or zero-duration carrier calls;
- the final admissible `1.875 s` full and `0.9375 s` half-1/half-2 selected
  receipts reconstruct the recorded energy operands and the known
  `27.2131278332233 J m^-2` difference;
- no accepted terminal parcel was produced and terminal liquid entered no
  pre-event hydrology, WB14 or surface-liquid transaction;
- the typed physical error is byte-identical to the existing
  `Stage3(TerminalNumerics(BelowCarrierDomain))`; and
- beginning and post-return state, owner, clock, provider cursor/call sequence,
  receipt, parcel, cursor and every `last_*` byte are identical and no output
  was published.

The post-return unchanged-state witness stores before/after canonical digests
and byte lengths for every named surface; equality is checked against retained
bytes outside the physical call, not asserted from shared identity.

## Noninterference and failure boundary

The package-owned test runs the identical physical fixture once with
`NoEvidence` and once with `CaptureEvidence` from identical serialized
beginnings. It compares physical result/error bytes, provider call order and
support, all caller-visible owners/state/output and all custody surfaces above.
Capture mode may construct and move values only; it cannot fail or affect a
branch used by the physical result. Arena resolution, independent digest
reconstruction, diagnostic assertions and serialization begin only after the
physical call has returned and the unchanged physical error and beginning
bytes have been retained.

Serialization or test-harness failure is deliberately injected only after
that return. Such failure may fail the diagnostic test/artifact write but
cannot be translated into, replace or suppress the already-retained physical
result. The former observer-panic requirement is retired. No panic-capable
callback exists inside the transaction, and no new unwind boundary is
authorized.

## Compilation boundary and API custody

The chosen boundary is option 1: a crate-private `#[cfg(test)]` unit-test path
compiled with `openwepp-hillslope-orchestrator` itself. `CaptureEvidence`, the
complete record payloads, arena resolver and artifact serializer are visible
only to that crate-private test module. The generic sealed mode and zero-sized
`NoEvidence` core may exist in normal compilation only as the minimum private
source correlation seam; production/default wrappers instantiate
`NoEvidence` exclusively and preserve their existing signatures.

No external integration test is claimed to see library `cfg(test)`. A separate
integration source guard may inspect source text and non-test symbols but may
not invoke capture. No Cargo feature, public/re-exported type, runtime flag,
environment selector or dependency is authorized.

## Prospective write set and mandatory stop

After two independent reviews on the exact hash return `GO-to-evidence`, the
bounded implementation may touch only the existing private terminal solver,
its existing covered provider/caller module, one crate-private diagnostic test
module, structural guards, and this package's evidence artifacts. Before edits,
the exact files and implementation intent must be recorded and applicable
instruction files discovered. Production signatures, public API, physics,
acceptance/controller/root/event logic, floor, Batch V2, receiver, restart,
runner, Child 3 and cutover remain excluded.

Required pre-implementation reviews are:

1. numerical/evidence reachability and exact pair-correlation review;
2. Rust/custody, compilation-boundary, noninterference and API review.

Either `HOLD` stops before source implementation. Two `GO-to-evidence`
recommendations authorize only this correlation seam, exact
`BelowCarrierDomain` reproduction, receipt capture, analytical reconstruction,
effectivity/conservation/floor matrix and frozen evidence. Final SnowEnergy
v21 numerical and science/ownership reviews remain separate subsequent gates.
No temporal-operator or Batch V2 production implementation intent may be
recorded unless both final v21 reviews return `GO`.
