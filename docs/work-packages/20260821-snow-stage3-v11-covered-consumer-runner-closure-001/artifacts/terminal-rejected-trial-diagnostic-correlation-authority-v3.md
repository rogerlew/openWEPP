# Terminal rejected-pair evidence-correlation seam authority V3

Status: `IN REVIEW / EVIDENCE-ONLY / NO SOURCE IMPLEMENTATION AUTHORITY`

Base: `8b2a7fe1789fb06386110fb5d6e3bc5fd2f7d962`

V2 remains frozen at SHA-256
`f4a7ff15127fdfd5068f16126f440a57a25026b44a5c610f175dfab30417cc5c`.
This is a distinct successor that accepts and closes all V2 review findings.
Its companion `terminal-diagnostic-correlation-v3-adapter-schema-manifest.md`
is normative and must be reviewed at its frozen hash with this authority.

Objective: authorize only the smallest private, value-returning correlation
path joining upstream carrier evidence to the existing downstream rejected
pair and floor decision. No physics, acceptance, controller, temporal
operator, Batch V2, event, receiver, restart, runner, output, model state,
public API, Child 3 or cutover change is authorized.

## New finding TDCV2-NUM-006

The V2 provider record conflates four separately owned facts:
provider-owned carrier phase, evaluation-owned flux and preview,
coupling-owned convergence/selection, and terminal-solver-owned hydrology
ending. An append-time provider record cannot truthfully contain a later
selection or hydrology-complete joint. V3 separates these records and never
mutates an arena entry after append.

## Private execution architecture

The existing internal terminal core may become generic over a sealed
compile-time `TerminalEvidenceMode`. `NoEvidence` is the zero-sized production
mode used exclusively by unchanged public and `pub(crate)` wrappers.
`CaptureEvidence` is reachable only from a crate-private `#[cfg(test)]` unit
path. No generic mode parameter appears in an existing public or `pub(crate)`
signature. There is no runtime selector, feature, environment input, callback,
closure supplied by diagnostics, global/thread-local recorder or internal
`catch_unwind`.

The complete forwarding chain that a later implementation intent must name is:

1. `snow_stage3_v11_terminal_execution.rs` provider/caller;
2. `hydrology/support_helpers_mod/runoff_reconciliation.rs` private trial
   types/provider context;
3. `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs`;
4. `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs`;
5. `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs`;
6. `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`.

Capture construction uses ordinary infallible value returns. Physical
`Result` bytes and beginning-state bytes are retained before arena resolution,
canonical serialization, assertions, deliberate diagnostic failure or
artifact I/O. Diagnostic failure cannot replace, suppress or translate the
already-returned physical result.

## Identity and immutable arena

`TerminalCarrierPhaseKeyV3` is fixed-size: schema `u32(3)`, prefix digest,
support start/end `u128`, exact live-provider-role `u8`, attempt `u32`,
coupling ordinal `u32`, beginning-joint digest, carrier ending-joint digest,
provider-call ordinal `u64`, arena index `u64`, phase-record digest and key
digest. It carries no pointer, vector, receipt payload or selected bit.

The caller-local arena is append-only in actual provider-call order. Each
entry is `(index, TerminalCarrierPhaseRecordV3)`; indices are contiguous from
zero. Entries are immutable once appended. Selection and hydrology joining
create later records referencing keys; they never update an entry or its
digest. Resolution requires every key field and digest to equal the indexed
record. Cross-prefix/support/role/attempt/coupling/joint/call substitution is
invalid in post-return diagnostic validation.

## Exact record ownership

### TerminalCarrierPhaseRecordV3

Provider-owned and immutable. Domain
`openwepp-terminal-carrier-phase-record-v3`. It contains in order: schema,
prefix/parent identity, A1 request, A3 probe child/forcing/topology, exact LSE
admission and active set, A4 prescribed amounts, A5 rate/components and
carrier envelope, A6 generated amounts, A7 snow--soil and `q_ss`, A8 soil
candidate/top-boundary credit, A9 WB14 child/hydrology replay excluding any
hydrology-complete ending joint, complete carrier ending candidates/joint A2,
explicit terminal parcel/ingress absence witnesses, provider-call ordinal and
record digest. It has no selected flag, coupling selection, terminal preview,
terminal flux integral, terminal ledger or hydrology-complete ending joint.

### TerminalCouplingIterationRecordV1

Evaluation-owned. Domain
`openwepp-terminal-coupling-iteration-record-v1`. Fields: schema, prefix,
support, exact live role, attempt and coupling ordinals; carrier-phase key;
A10 Stage-3 flux integral; A10 terminal preview; incoming hint option;
outgoing hint; convergence operands in this exact order: outgoing minus
incoming ice, liquid, cold content and surface temperature, corresponding
existing tolerances, absolute comparisons and combined convergence boolean;
iteration record digest. First iteration has absent incoming hint. This record
does not claim selection.

### TerminalCouplingSelectionRecordV1

Coupling-owner record. Domain
`openwepp-terminal-coupling-selection-record-v1`. Fields: schema, prefix,
support, exact live role, attempt ordinal, ordered iteration-record keys in
actual evaluation order, exact selected iteration key, selected carrier-phase
key, selected coupling ordinal, selection proof, record digest.

The selection proof contains the selected iteration's convergence operands,
`combined_converged=true`, exact returned flux digest, exact returned preview
digest and exact returned carrier-ending-joint digest. All preceding
iterations must have `combined_converged=false`; no later iteration exists.
The selected key must be a member exactly once. Selection is emitted by
`evaluation.rs`, never inferred by last entry, ending-joint lookup or numeric
matching. No arena mutation occurs.

### SelectedTerminalTrialRecordV3

Terminal-solver-owned after its hydrology join. Domain
`openwepp-selected-terminal-trial-record-v3`. Fields: schema, prefix,
pair-position option, exact live role, attempt ordinal, exact support,
coupling-selection receipt, A10 physical beginning state, A10 physical ending
state and complete terminal ledger, carrier ending-joint digest,
hydrology-complete ending joint A2, trial digest. The hydrology-complete joint
is produced only by the existing `join_hydrology_ending` call after applying
the unchanged terminal transition.

## Pair position and live role

Pair position and provider role are distinct closed enums. `PairPosition` is
`COARSE=0`, `FINE_1=1`, `FINE_2=2`. `LiveProviderRole` exactly mirrors the
live Rust discriminants: `FULL=0`, `HALF_1=1`, `HALF_2=2`, `RETRY=3`,
`BRACKET_LOWER=4`, `BRACKET_UPPER=5`, `ROOT=6`.

The only adaptive pair mappings are:

- `COARSE + FULL`;
- `COARSE + RETRY`;
- `FINE_1 + HALF_1`;
- `FINE_2 + HALF_2`.

Bracket and root trials have no pair position and retain their exact live role.
Retry is never translated to Full. No event-root or other synthetic role is
defined.

## Evaluated pair decision

`TerminalPairDecisionRecordV3` has domain
`openwepp-terminal-pair-decision-record-v3`. It contains schema, prefix and
pair ordinal; exactly one COARSE trial, one FINE_1 trial and one FINE_2 trial;
the refined composition (FINE_2 ending state plus exact binary64 left-to-right
ledger sum FINE_1 then FINE_2); five component error records; maximum scaled
error; diagnostic winner; decision `ACCEPT=0` or `REJECT_RETRY=1`; current
duration; optional proposed next duration; pair record digest.

COARSE and FINE_1 begin from the same accepted prefix. FINE_2 begins from the
exact FINE_1 hydrology-complete ending state/joint. Full support is exactly the
concatenation of the two fine supports. A trial key occurs in exactly one
position of exactly one pair. Each pair owns exactly one decision. A rejected
pair remains `REJECT_RETRY`; it never acquires a later floor error.

For each canonical component in order `ice`, `liquid`, `cold content`,
`complete energy`, `unallocated energy`:

```text
delta       = refined - coarse
denominator = abs_tol + rel_tol * max(abs(coarse), abs(refined))
scaled      = abs(delta) / denominator
```

Every operation is binary64 in the written order. Mass components use the
existing mass absolute tolerance; energy components use the existing energy
absolute tolerance; all use the existing relative tolerance. The maximum is
the exact binary64 left fold over canonical component order, initialized by
the ice scaled value and applying the live `max` operation to each subsequent
value. The diagnostic winner is the first canonical component whose scaled
bits equal the maximum bits. Winner is computed after the physical decision
and is never an acceptance/controller input.

## Pre-provider admission decision

`TerminalTrialAdmissionRecordV1` has domain
`openwepp-terminal-trial-admission-record-v1`. Fields: schema, prefix,
admission ordinal, proposed pair support start/end, proposed duration,
required fine duration (`proposed/2` in exact existing binary64 evaluation),
minimum carrier duration `0.6`, admission decision, exact typed error option,
provider-call count before, provider-call count after and record digest.

An admitted decision uses `ADMIT`. The terminal rejected-prefix proof ends in
a separate `BELOW_CARRIER_DOMAIN` admission record created by the existing
pre-provider `dt < 2 * minimum` branch. It follows the last evaluated pair,
which remains `REJECT_RETRY` with its proposed next duration. The floor record
must bind that proposal exactly and prove equal before/after provider-call
counts: zero provider calls occurred for the failed admission. It contains no
fabricated trial, selection or carrier key.

## Rejected-prefix record and cardinality

`TerminalRejectedPrefixRecordV3` has domain
`openwepp-terminal-rejected-prefix-record-v3`. Fields: schema; parent/prefix
identity; exact beginning owner/joint/clock/provider/cursor/receipt/parcel and
all `last_*` bytes/digests; one ordered tagged sequence whose members are
admission, selected non-pair trial, or pair-decision records in actual order;
final physical result bytes; complete immutable carrier arena; complete
ordered coupling iteration/selection records; post-return unchanged-state
witness; record digest.

Every selected adaptive trial is referenced exactly once by a pair. Every
selected bracket/root trial is referenced exactly once as a non-pair member.
Every arena carrier record is referenced by exactly one coupling iteration;
each coupling selection references one nonempty ordered iteration sequence and
one selected member. No discarded iteration can substitute for the selected
one. Sequence validation independently proves every positive provider support
is at least `600000000 ns`, zero subminimum calls occurred, and the final
admission has zero provider calls.

The final admissible 1.875-second COARSE and two 0.9375-second fine records
must resolve complete receipts and independently reproduce the known
`27.2131278332233 J m^-2` discrepancy. The record also proves no terminal
parcel, terminal-liquid hydrology/WB14/surface ingress, state/owner/clock/
provider/receipt/parcel/cursor/`last_*` installation or output publication.
Final physical error bytes must equal the existing
`Stage3(TerminalNumerics(BelowCarrierDomain))` exactly.

## Canonical encoding

All V3/V1 records use the normative companion adapter manifest, the
repository-established big-endian framed primitives and SHA-256. `ModelTimeNs`
is `u128` big-endian. Byte and sequence lengths are `u64` big-endian. Floats
retain exact binary64 bits including signed zero. Enum tags, collection order,
nested receipt encoding and A1--A10 exact field order are frozen by the
manifest. Unknown variants, missing/duplicate fields, noncanonical order,
trailing bytes, digest-only receipt substitution or digest mismatch fail the
post-return diagnostic validator. No schema choice is deferred to source
implementation.

## Test and noninterference boundary

The selected boundary remains a crate-private `#[cfg(test)]` unit path, not an
external integration test and not a Cargo feature. Production/default
execution instantiates `NoEvidence`, performs no arena allocation and returns
the existing result/error exactly. Capture mode is package-owned and private.

The same fixture is executed from identical serialized beginnings with
`NoEvidence` and `CaptureEvidence`. The test compares physical result/error
bytes, all provider calls/order/support, state, owners, clock, provider cursor,
receipts, parcels, cursor, every `last_*` field and output. Serialization and
deliberate harness failure occur only after both physical calls returned and
their beginning/result bytes were retained. No callback-panic requirement is
revived.

## Review gate and stop

Only authority-stage documentation and structural/historical guards may change
before review. Run formatting, diff hygiene and the package-owned V20/V21
historical/structural contract guards. Freeze both authority and adapter
manifest hashes. Obtain two independent exact-hash reviews:

1. numerical/evidence/cardinality `GO-to-evidence` or `HOLD`;
2. Rust/custody/API/compilation-boundary `GO-to-evidence` or `HOLD`.

Either HOLD stops before source edits. Two GO reviews authorize only a later,
separate exact-file implementation intent naming the complete forwarding chain
above. Only after that recorded intent may the private
`NoEvidence`/`CaptureEvidence` correlation seam be implemented to observe the
unchanged `BelowCarrierDomain` path. Final v21 review remains prohibited until
the exact receipt capture and estimator effectivity matrix exist.
