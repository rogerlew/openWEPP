# Authority Review B — Custody, Restart, Wire

Status: `HOLD`

Evidence mode: `Static + Ran`

Reviewed exact commit:
`5f4d3902065c316309785cc47ac63f766884bbd1`.

Reviewer scope: transaction/ownership, staged water/N/material custody, closed
wire identity, additive restart, serialization, one atomic parent finalization,
and required poison/reference coverage. No production Rust was reviewed.

## Ran evidence

- strict Binding Exposure Index check => PASS, 15 rows consolidated;
- science-contract unit-compliance check => PASS, no findings;
- `nix develop --command cargo test --test
  c3_woody_v11_authority_contract` => PASS, 3/3.

These gates establish structural presence and execution of the current 22-case
calculator population. They do not close the semantic findings below.

## Findings

### `V11-AUTH-B-001` — BLOCKER — additive restart omits required durable state

The amendment requires restart to retain publication/reduction state and the
next parent sequence, but `v11-restart-schema.json` has no fields for
diagnostic/reduction operands, parent publication buffer/outbox state, next
parent sequence, or explicit accepted event receipts. Its generic arrays cover
owner/resource/material/scheduled blobs only. A schema-valid restart can lose a
pre-restart peak, republish output, or resume with an ambiguous successor.

Required disposition: extend the closed restart schema and authority with all
continuation state, including accepted event chronology, accepted-only
reduction operands, pending parent publication/outbox state, and checked next
parent sequence. State whether each field is held by embedded coupled-time V2
or V11 and require equality/join validation where duplicated. Add before/after-
event, reduction, publication, abort, and successor-sequence restore poisons.

### `V11-AUTH-B-002` — BLOCKER — opaque blobs are not a closed authenticated wire

Configuration, state, and restart schemas admit base64 plus claimed SHA-256,
but define no mandatory semantic validator that decodes bytes, verifies
canonical base64, reconstructs identities/digests, validates the imported V10
wire version, enforces owner/receipt ordering and uniqueness, or reserializes
canonically. The base64 regex admits invalid-length/noncanonical encodings.
Generic restart blobs do not identify owner class, lineage, schema version, or
cardinality. Forged, duplicate, reordered, or wrong-owner custody can therefore
appear schema-valid.

Required disposition: define/version a mandatory V11 semantic validator,
canonical framing, typed owner/receipt envelopes, ordering/uniqueness/
cardinality, byte-to-digest reconstruction, and cross-object joins. Add padding,
digest, embedded-schema, duplicate, reorder, unknown-owner, and canonical-
reserialization poisons.

### `V11-AUTH-B-003` — BLOCKER — resource arithmetic is not reproducibly specified

The contract requires cumulative water and NH4/NO3 reconstruction but does not
define amount wire representation, finite/nonnegative admission, accumulation
algorithm/order, exact-vs-tolerance closure, or comparison of receipt sums with
parent-beginning minus final candidate. The calculator silently chooses Python
`math.fsum` over anonymous `f64` debits, an uncontracted policy without
owner/source/species/basis identities or staged ending inventory.

Required disposition: bind canonical amount encoding and deterministic ordered
reconstruction for water, NH4, and NO3 separately; define domain guards and
closure behavior; add cancellation/order aliases plus authorization/final-use
and beginning/final-owner reconstruction.

### `V11-AUTH-B-004` — BLOCKER — vectors do not prove release obligations

The segmented vectors model only supports, scalar debits, an increment count,
a duplicate string receipt, and a replay flag. They do not model staged owners,
water authorization, separate NH4/NO3, materials, events, restart restore,
atomic commit, or publication. The Rust test checks a short case-name list and
result count. Missing executable positives include parent-start/end events,
zero-remainder skip, mid-parent restore, and consecutive parents. Missing
poisons include wrong slab/participant, local duration conversion, segment 2
from parent beginning, per-segment commit, water/N swaps, material reorder,
event rate integration, rejected leakage, and publication rollback.

Required disposition: replace the scalar toy with typed owner/receipt/restart
cases executing every package-required positive and poison obligation. Tests
must assert semantics, not only name presence and total count.

### `V11-AUTH-B-005` — MAJOR — parent finalization identity is underspecified

Prose requires one atomic complete-owner install and consuming finalization but
does not define the complete candidate/receipt wire, owner membership and
cardinality, live clock/beginning joins, stable material proposal framing, or a
nonforgeable one-shot capability. This permits divergent owner ordering or a
vegetation-only finalize path while claiming conformance.

Required disposition: freeze typed candidate, parent receipt, proposal framing,
complete-owner manifest/order, precondition joins, and consuming API
obligations. Exercise duplicate finalize, partial/reordered owner set, stale
clock, material reorder, and rollback after late owner failure.

## Positive authority assessment

The amendment correctly protects immutable V10, sequences ending-to-beginning
state, retains NH4/NO3 identity in prose, chooses ordered segment material
accumulation, forbids segment commit, and requires one parent increment/atomic
commit. The HOLD is confined to making those decisions closed, reproducible,
and executable before implementation.

## Verdict

`HOLD / authority release blocked by V11-AUTH-B-001 through -005`.

No production Rust may begin until blockers are corrected, invalidated gates
are rerun, and independent verification passes the corrected checkpoint.

---

## Re-review — corrected candidate `675296fdb262efd052be40d32d6730b3d895220a`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran`

Ran on the exact corrected candidate:

- strict Binding Exposure Index checks => PASS for both contracts;
- science-contract unit-compliance checks => PASS for both contracts;
- `nix develop --command cargo test --test
  c3_woody_v11_authority_contract` => PASS, 4/4;
- independent calculator => 46/46 expected records;
- `git diff --check` => PASS before this review append.

### Finding closure audit

#### `V11-AUTH-B-001` — PARTIAL, remains BLOCKER

The restart schema now explicitly carries cursor/ordinal/participants, slab and
event receipts, reductions, pending publication/outbox, and current/next parent
sequence. That closes the original missing-field list in form. It does not yet
close equivalent continuation: all receipt kinds share one generic shape that
requires a nonempty `segment_id` and `accepted_slab_id`, including parent-start
events and scheduled/publication receipts that need not belong to a slab;
`CommittedParent` still requires an active segment; and duplicated coupled-time
and V11 fields have no executed cross-wire join. Restart positives merely
compare caller-provided strings (`restored_digest == uninterrupted_digest`) and
never parse, validate, serialize, restore, or continue the restart schema.

Required closure: define phase-appropriate receipt unions/cardinality and
optional/required fields, phase-specific restart constraints, and executable
fresh-object roundtrip/continuation that reconstructs all duplicated joins,
reductions, publication, sequence, and event replay state.

#### `V11-AUTH-B-002` — OPEN, remains BLOCKER

`OPENWEPP_C3_WOODY_V11_SEMANTIC_VALIDATOR_V1` exists only as a descriptive
string in `model-definition.json`; it is neither normative contract text with
a complete algorithm/error map nor an independent executable validator. The
configuration/state schemas replaced base64 with unconstrained JSON objects:
`v10_configuration_canonical_json` and `v10_physical_state_canonical_json`
accept arbitrary properties and unknown structures. Restart receipt
`payload_canonical_json` is equally open. The new test proves only that the
literal word `base64` disappeared, not closed V10/V11 schema identity, digest
reconstruction, ordering/cardinality, or parse-reserialize equality. No
malformed canonicalization/digest/duplicate/reorder/unknown-owner cases execute.

Required closure: import closed versioned V10 schemas or enumerate the embedded
types, make semantic validation canonical authority with an executable
independent population, and prove every claimed digest, order, cardinality,
schema identity, and canonical reserialization poison.

#### `V11-AUTH-B-003` — PARTIAL, remains BLOCKER

The contract now correctly selects ordinary IEEE-754 left folds from `+0.0`,
finite intermediates, distinct water/NH4/NO3 keys, and bit-exact parent closure.
But vectors encode decimal strings, not canonical `f64` bit strings, and the
calculator converts them with Python `float`; it checks only `total > inventory`.
It does not advance a staged owner after each receipt, enforce
`0 <= finalized <= authorization <= demand`, compare bit-exact cumulative debit
with parent-beginning-minus-final ending, retain layer/owner/basis identities,
or execute non-associative/order and identity-swap aliases. Thus the selected
policy is frozen in prose but not independently proven.

Required closure: model canonical amount bits and typed receipt keys, segment-
by-segment authorization/final-use/ending inventory, independent parent
reconstruction, exact subtraction bits, and non-associative/reordered/
water-NH4-NO3-layer-owner-basis poisons.

#### `V11-AUTH-B-004` — OPEN, remains BLOCKER

The case count increased from 22 to 46 and names the requested surfaces, but
most new cases are Boolean/string switches rather than the admitted model:
events are counted without beginning/ending owners or custody ledgers;
participant sets are checked only for array length; restart equivalence is a
string equality; consecutive parents are not chained; `atomic_commits` is a
constant; publication is a hash of an input list; no material receipt is
represented; and no typed owner/receipt identity is constructed. The test still
asserts selected names and result count, not schema/wire/owner semantics. This
is expanded self-consistency, not independent transaction reconstruction.

Required closure: make the oracle construct/authenticate typed slab, event,
resource, material, restart, parent-candidate, commit, and publication objects;
derive ending owners and receipts; chain restart/consecutive parents; and
mutate exact operands for every poison.

#### `V11-AUTH-B-005` — PARTIAL, remains BLOCKER

The model definition and transaction prose now list parent-candidate fields and
state one-shot atomic semantics. There is still no closed candidate/receipt
schema, typed complete-owner manifest/cardinality, field types, framing/domain
tags, canonical proposal-ID algorithm, or executable live-clock/beginning
authentication. The oracle returns constant `increments=1` and
`atomic_commits=1`; it never constructs or consumes a candidate, so duplicate,
partial/reordered/stale/late-owner paths are not tested.

Required closure: freeze the full typed wire and identity framing, complete
owner manifest, proposal IDs, live preconditions, consuming transition, and
independent construction/poison population.

### Overlapping Review A findings

The event capability and phenology classification prose materially improves
`A-002/A-004`, and migration boundary vectors materially improve `A-006`.
`A-001/A-003/A-005` remain open for the same non-executable chronology,
restart, and resource reasons above. Compatibility classification improvements
must be judged separately under `A-007`; they do not cure custody/wire gaps.

### Re-review verdict

`HOLD / corrections are directionally substantial but V11-AUTH-B-001..005 are
not terminally closed at 675296fdb`.

Do not promote or begin production Rust from this candidate.

---

## Final re-review — candidate `c7ec8e73096f9816ffbe812ac15deeba1d2b8574`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial probe`

Ran on the exact candidate:

- strict BEI checks => PASS for both contracts;
- science-contract unit-compliance => PASS for both contracts;
- independent semantic validator => PASS valid construction, before/after
  checkpoint equivalence, atomic commit, and 21/21 poisons;
- V11 authority contract test => PASS, 5/5;
- `git diff --check` => PASS before this append.

The executable authenticated receipt model, staged resource folds, event
custody, material ordering, publication buffering, one-shot store, and poison
suite are substantive corrections. They close the former purely flag/count
posture. Three exact blockers remain.

### `V11-AUTH-B-001` — OPEN — the released restart wire is still not exercised

`semantic_schema_validator.py` does not construct, validate, serialize, or
restore `OPENWEPP_C3_WOODY_V11_RESTART_V1` from `v11-restart-schema.json`.
Instead it invents the narrower
`OPENWEPP_C3_WOODY_V11_CHECKPOINT_TEST_V2`, containing only parent/receipt
prefixes and staged resource bits. It omits the released restart's complete
owner candidates, active participants/segment, coupled-time V2 bytes,
reductions, pending publication, outbox, scheduled/material receipts, and
current/next sequence. `restore_and_continue` then validates the already-complete
parent candidate and commits it; it does not resume physical construction from
the checkpoint. Thus fresh-object/event-boundary equivalent continuation of the
actual additive wire remains unproven.

Required closure: construct the exact V11 restart schema before and after the
event, execute the mandatory semantic validator over every field/cross-wire
join, deserialize into a fresh staged coordinator, continue only the remaining
chronology, and compare terminal owners/receipts/reductions/publication bytes
with uninterrupted execution. Poison each omitted/reordered/duplicated field.

### `V11-AUTH-B-002` — PARTIAL — validator scope does not cover all admitted wires

The semantic validator is now normative and executable for the parent
candidate and its receipt payloads, closing the prior unauthenticated-receipt
defect. It still never admits V11 configuration, V11 state, or V11 restart,
although the contract says JSON shape alone never admits any of them. Their
`v10_*_canonical_json` payloads remain schema-open objects and no executed case
checks embedded V10 schema/version, configuration/state digest reconstruction,
unknown fields, or canonical parse-reserialize identity. The test assertion
that base64 field names are absent is not semantic admission.

Required closure: extend the independent validator and poisons to the exact
configuration/state/restart wires, including closed embedded V10 schema
identity, digest joins, unknown-field rejection, canonical JSON bytes, and
roundtrip equality.

### `V11-AUTH-B-005` — OPEN — live beginning-owner authentication is missing

The candidate schema and validator now authenticate receipt payloads and a
fixed seven-class order. However `AtomicStore.commit` checks only live clock
identity; it never requires the store's current owner digests to equal
`candidate.beginning_owner_sha256`. An adversarial read-only probe changed the
candidate's vegetation beginning digest to all zeros, recomputed the parent
receipt, and observed both `validate(c) => PASS` and `commit(c) => consumed`.
This violates the required same-beginning complete-owner join and permits a
stale/foreign candidate to overwrite live owners.

The candidate also represents one digest per owner class, while the normative
manifest requires owner IDs, expected per-class counts, schema/model/config IDs,
and multiple within-class owners ordered by canonical ID. Those fields do not
exist in the schema or executable manifest.

Required closure: authenticate exact live owner set against candidate
beginnings before any install; encode/validate the full typed owner manifest and
cardinality; add stale-beginning, multiple-owner, wrong owner-schema/config,
missing/extra ID, and late-failure rollback poisons.

### Prior findings otherwise

- `V11-AUTH-B-003`: materially closed for the reference transaction. It now
  uses binary64 bits, sequential request/authorization/final-use/ending folds,
  distinct water/NH4/NO3, and independent receipt authentication. Production
  implementation must retain full OFE/tile/layer/basis identity.
- `V11-AUTH-B-004`: materially closed for candidate/event/resource/material/
  publication/atomicity poisons by the independent semantic model, subject to
  the actual-restart and live-owner gaps above.

### Final verdict

`HOLD / B001, B002, and B005 remain release-blocking at c7ec8e730`.

Do not promote the authority or begin production Rust.

---

## Terminal review — candidate `205e0ad4e628044093e42eb99388fbbac6942d2c`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial restart probes`

Ran at the exact checkpoint:

- strict BEI checks => PASS for both contracts;
- science-contract unit-compliance => PASS for both contracts;
- semantic validator => PASS, 26/26 declared poisons and reported restart
  equivalence;
- V11 authority contract test => PASS, 5/5;
- `git diff --check` => PASS before this append.

The candidate closes the prior live-owner forgery, restores
`INV-VEGTRANSACTION-010`, constructs the actual V11 restart tag, closes receipt
body keys, reconstructs ending owner digests, and introduces typed owner
descriptors. Those are material corrections. Two release blockers remain.

### `V11-AUTH-B-001` — OPEN — restart admits unauthenticated continuation facts

`checkpoint` now emits `OPENWEPP_C3_WOODY_V11_RESTART_V1` and
`restore_and_continue` executes the remaining event/slab/resource suffix. But
restore validates only selected receipt prefixes, staged owners/resources,
coupled blob digest/canonical JSON, and the inner physical-state digest. It does
not authenticate or reconstruct several required fields.

Four independent read-only probes mutated one field at a time and were all
accepted through terminal commit:

- `reduction_state.peak_bits = bits(999.0)`;
- `staged_v11_state.state_sha256 = "00" * 32`;
- `active_participant_ids = ["wrong"]`;
- `next_parent_transaction_sequence = "999"`.

The same code does not validate checkpoint phase/cursor consistency,
parent/current sequence, authority identity, scheduled/material receipt
prefixes, pending publication records, or outbox joins. The 26-poison set has
only rejected-resource leakage and event replay restart mutations, so reported
equivalence does not cover these fields.

Required closure: reconstruct and compare every required restart field and
cross-wire join, derive active participants/ordinals/sequences from accepted
chronology, authenticate outer V11 state identity, rebuild reductions and
publication/outbox, validate scheduled/material prefixes, and add one-field
poisons for every retained continuation fact.

### `V11-AUTH-B-002` — OPEN — configuration/state admission is a reduced surrogate

The new imported schemas are closed syntactically, but they define a five-field
V10 configuration and five-field V10 state fixture rather than the complete
immutable V10 configuration/state named by the contract. The real V10 surface
includes topology/strata and the full C/N, canopy, hydraulic warm-start, T10,
phenology, NSC/XS/retranslocation, occupancy, and pending-transfer state. A
reduced fixture cannot establish bit-identical migration or reject unknown/
omitted real V10 fields.

Moreover the executable validator never constructs/admit-checks a complete V11
configuration object, and restart checks `physical_state_sha256` but not the
outer `state_sha256` framing, as the accepted forgery probe demonstrates.

Required closure: bind/import the complete released V10 canonical schemas or a
lossless generated exhaustive projection, execute V11 configuration/state
digest reconstruction and migration, and poison every omitted/unknown/changed
physical field plus outer identity joins.

### Prior findings otherwise

- `V11-AUTH-B-003`: closed for staged water/NH4/NO3 fold authority in this
  reference transaction.
- `V11-AUTH-B-004`: closed for the declared parent/event/material/publication/
  atomicity population, but cannot close restart while B001 remains.
- `V11-AUTH-B-005`: live beginning owners, clock, ending-owner reconstruction,
  consuming commit, and seven typed descriptors now execute. Multiple owners
  within a class remain unrepresented despite the contract's within-class
  ordering language; reconcile that wording/schema before promotion if the
  complete parent owner set can contain more than one owner per class.

### Terminal verdict

`HOLD / actual restart authentication and complete V10 config/state admission
remain release-blocking at 205e0ad4e`.

No production Rust may begin from this checkpoint.

---

## Re-review — candidate `ab07b1cf62b3da4299baf4ce045ebecccd85911e`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial restart probes`

Ran at the exact checkpoint:

- strict BEI checks => PASS for both contracts;
- science-contract unit-compliance => PASS for both contracts;
- semantic validator => PASS with 34/34 declared poisons and suffix-equivalent
  restore;
- V11 authority contract test => PASS, 5/5;
- `git diff --check` => PASS before this append.

The prior forged reduction, staged-state digest, participants, successor
sequence, cursor, scheduled replay, and outbox probes are now rejected. Actual
suffix event/resource/state advancement executes. Two exact blockers remain.

### `V11-AUTH-B-001` — OPEN — restart does not authenticate all retained owner/receipt bytes

Two new independent one-field probes were accepted through restore and terminal
commit:

- replacing `parent_beginning_owner_sha256[0]` with all zeros;
- replacing the accepted material receipt's `payload_sha256` with all zeros
  while retaining its claimed receipt identity.

`restore_and_continue` reconstructs staged owners from the complete candidate,
not from the restart's retained parent-beginning array, so that array is never
joined. For accepted material receipts it compares identity IDs only; it never
calls `validate_receipt`, so payload bytes/digest/body can be forged. The same
ID-only prefix pattern must be audited for every accepted receipt category;
every retained receipt requires full identity and payload authentication, not
projection equality alone.

Required closure: require restart parent beginnings to equal candidate/live
parent beginnings, decode and authenticate every accepted slab/event/resource/
material/scheduled/publication receipt from restart bytes, reconstruct their
IDs and ordered predecessor chain, and add payload/digest/identity poisons per
category.

### `V11-AUTH-B-002` — OPEN — closed config/state artifacts remain incomplete V10 projections

The embedded fixtures are syntactically closed and canonical, but the V10
configuration projection has five fields and the V10 state projection has five
fields. The production `VegetationConfiguration` alone contains the full
stratum parameter stack, topology, roots, optics, hydraulics, carbon/nitrogen,
phenology, and many other fields; V10 state likewise contains complete shared/
occupancy and persistent physical state. The reduced fixtures cannot prove the
contract's bit-identical migration of the complete V10 physical surface.

The validator also checks fixture key sets directly rather than constructing a
complete `OPENWEPP_C3_WOODY_V11_CONFIGURATION_V1` and reconstructing its outer
configuration/initial-state digests. This remains a surrogate admission surface.

Required closure: freeze a lossless exhaustive canonical projection of every
V10 configuration/state field (prefer generated from released types), execute
outer V11 configuration/state identity reconstruction, and add omission,
unknown-field, changed-bit, and cross-digest poisons across the complete tree.

### Other finding status

- `V11-AUTH-B-003`: closed for staged water/NH4/NO3 authority.
- `V11-AUTH-B-004`: closed for the declared non-restart semantic population;
  restart remains blocked by B001.
- `V11-AUTH-B-005`: independent live owner/clock, ending owner reconstruction,
  and consuming atomic store remain closed for the seven aggregate owners.
  Multiple within-class owners remain a prose/schema mismatch unless the
  contract explicitly fixes one aggregate owner per class.

### Verdict

`HOLD / restart parent-beginning and material-receipt authentication plus
complete V10 config/state admission remain open at ab07b1cf6`.

No production Rust may begin.
