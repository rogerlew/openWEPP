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

---

## Release re-review — candidate `c53adab0a91c0ecbe853c884bfe05591826441c5`

Date: 2026-08-20

Status: `PASS`

Evidence mode: `Static + Ran + adversarial restart probes`

### Gate evidence

- strict BEI checks => PASS for both contracts;
- science-contract unit-compliance => PASS for both contracts;
- semantic validator => PASS, 36/36 declared poisons and restored suffix
  equivalence;
- V11 authority contract test => PASS, 5/5;
- `git diff --check` => PASS before this append.

### Prior blocker closure

- `V11-AUTH-B-001`: CLOSED. The actual V11 restart is constructed, its suffix
  executes, and retained owner beginnings plus every accepted slab/event/
  resource/material envelope are authenticated. Rerun adversarial probes for
  forged parent beginning, material payload digest, reduction value, outer
  state digest, participants, and successor sequence all reject typed.
- `V11-AUTH-B-002`: CLOSED for preimplementation authority. The migration API
  now normatively accepts only the complete released Rust
  `VegetationConfiguration` and `VegetationStateV10`; source files, released
  model definition, and recursive compatibility ledger are hash-bound. The
  reduced canonical fixture is explicitly a framing KAT and explicitly
  non-admitted. V11 configuration/state identity inputs and reconstruction are
  named contractually; exhaustive values remain implementation-gate evidence,
  not implementation-selectable authority.
- `V11-AUTH-B-003`: CLOSED. Water, NH4, and NO3 use authenticated typed receipt
  bits, sequential staged folds, and exact ending-owner reconstruction.
- `V11-AUTH-B-004`: CLOSED. The independent semantic model constructs and
  authenticates slab/event/resource/material/publication/parent/restart
  chronology and exercises 36 adversarial mutations, including rollback and
  replay.
- `V11-AUTH-B-005`: CLOSED. Live owner and clock beginnings, reconstructed
  endings, consuming commit, publication-after-commit, and rollback execute.
  The contract now explicitly fixes Version 1 to exactly seven aggregate owner
  envelopes; per-OFE/tile/occupancy/layer identities remain inside each typed
  envelope, removing the prior manifest ambiguity.

`INV-VEGTRANSACTION-010` is restored and all Review B findings have exact
authority/evidence closure. No Review B waiver is required.

### Verdict

`PASS / authority may proceed to independent verification at c53adab0a`.

This verdict authorizes the contract verification checkpoint only. Production
Rust remains prohibited until the package obtains both required independent
authority verifications and records the exact promoted authority checkpoint.

---

## Restart V2 amendment review — candidate `ac8cb0eda4110d5b5fe8811d82da314b6d8bf25e`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + adversarial surface audit`

### Gate evidence

- independent `restart_v2_reference.py` => PASS for its valid fixture and
  15/15 declared poisons;
- V11 authority contract test => PASS, 6/6 (with unrelated concurrent Rust
  warnings outside this amendment);
- strict BEI checks => PASS for both contracts;
- science-contract unit-compliance => PASS for both contracts;
- restart V2 schema and poison JSON parse => PASS.

These gates prove the checked fixture, not the complete typed admission
claimed by the amendment.

### `V11-RESTART-V2-B-001` — BLOCKER — the oracle admits a reduced surrogate checkpoint

The amendment normatively requires the complete deny-unknown-fields
`V11ParentTransactionCheckpoint`. The reference instead embeds a package-local
five-field object containing only schema, parent ID, cursor, slab count, owner
arrays, and `finalized`. It omits the complete beginning/staged V11 states,
accepted segment projections, cumulative resource debits, staged owner map,
receipt support/duration/predecessor facts, typed resource/material payloads,
and reconstructed final staged state. `decode` merely parses generic JSON and
canonicalizes it; it never admits the production checkpoint type. A reduced
checkpoint can therefore pass every current poison while being impossible to
continue equivalently.

Required closure: construct the complete runtime checkpoint projection in the
independent oracle, enumerate its closed field set, authenticate its complete
nested states/segments/debits/materials/owners, and execute a remaining suffix
from that restored checkpoint. Unknown/omitted/changed nested-field poisons are
required.

### `V11-RESTART-V2-B-002` — BLOCKER — retained outer chronology is not completely joined

The reference joins only parent ID, cursor, slab/event ordinals, participants,
and successor arithmetic. `active_segment_id`, `active_regime_id`,
`controller_policy_sha256`, configuration identity, scheduled execution keys,
accepted event predecessor/support chains, and current parent sequence are not
cross-checked against either embedded authority. The embedded coupled-time
fixture itself is not admitted as the released typed V2 wire. Independent
mutations of these retained fields therefore have no declared poison and no
reconstruction path.

Required closure: typed-admit exact coupled-time V2 bytes with expected model,
authority, and policy; cross-check every duplicated cursor/regime/participant/
sequence identity; reconstruct ordered event and scheduled-once chronology;
and add one-field poisons for every retained fact.

### `V11-RESTART-V2-B-003` — BLOCKER — owner, reduction, and publication authentication is partial

Owner envelopes verify base64 and digest but do not parse-reserialize canonical
owner state or reconstruct `OwnerState::new`; the schema's seven repeated
`owner` references do not itself bind class order. Reduction admission checks
only that an operand does not end in the future: it does not authenticate
source receipts, ordering/uniqueness, finite value bits, or rebuild the fold.
Publication payload digests are never checked. Event receipt IDs/predecessors
are not authenticated or ordered. Outbox state/delivery-count consistency and
record uniqueness are absent.

Required closure: execute the contract-declared typed owner constructor and
digest comparison; rebuild event, reduction, publication, and outbox identity
from operands; and add forged payload/digest/source/order/duplicate/state-count
poisons for each surface.

### Verdict

`HOLD / Restart V2 is directionally the correct additive boundary, but its
reference and poison authority do not yet prove complete typed restoration at
ac8cb0eda`.

Do not promote this amendment or resume restart production until all three
blockers are corrected and independently re-reviewed.

---

## Restart V2 amendment re-review — candidate `a38e2cfa12705a6692ced186b5dc4e51d97ab3f3`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent mutation probes`

### Gate evidence

- corrected restart V2 reference => PASS one-slab suffix equality and 24/24
  declared poisons;
- strict BEI and science-contract unit-compliance => PASS for both contracts;
- schema/poison JSON parse => PASS;
- Rust authority test could not run because concurrent uncommitted
  orchestrator implementation does not compile; the failures are outside this
  amendment and are not used as the semantic verdict.

The correction materially improves typed owner-body parsing, coupled
authority/configuration/controller/cursor joins, complete checkpoint-shaped
fixture structure, event/scheduled checks, reduction/publication checks, and a
one-slab suffix comparison. Two blockers remain under direct adversarial
probe.

### `V11-RESTART-V2-B-001` — OPEN — checkpoint resource/material custody is not reconstructed

The checkpoint now contains beginning/staged states, a segment, typed receipts,
cumulative debit fields, and owner envelopes. Admission authenticates receipt
framing and cardinality, but it does not reconstruct resource folds or material
effects. Each of these independently reframed mutations was accepted through
`restore_suffix`:

- change the water receipt's final use from 1.0 to 2.5;
- change checkpoint cumulative water debit from 1.0 to 99.0;
- change the accepted material amount from 0.01 to 99.0.

The accepted suffix uses only scalar `staged_state.value`; it does not derive
that state or staged owners from authenticated accepted receipts. Thus a
complete-looking checkpoint can retain contradictory debit/material custody
and still produce the hard-coded equality `7 == 7`.

Required closure: reconstruct cumulative water/NH4/NO3 bits and staged ending
owners from the accepted receipt sequence, validate material effects and
staged state predecessors, then start suffix execution from those reconstructed
objects. Add changed-use, changed-ending, changed-cumulative, material amount/
source/receiver, and staged-owner mismatch poisons.

### `V11-RESTART-V2-B-002` — OPEN — retained identities and reduction/publication IDs remain forgeable

Direct mutations of `active_regime_id`, reduction `operand_id`, and publication
`record_id` were accepted. Active regime is not joined to an embedded regime/
event transition. Reduction validates source/support/value but never
reconstructs operand identity or uniqueness/order. Publication authenticates
payload and source reduction but never reconstructs record identity; outbox IDs
and uniqueness are likewise not authenticated.

Required closure: derive active regime from the accepted event/participant
transition; reconstruct reduction operand/reduction/result identities in order;
reconstruct publication and outbox IDs and enforce uniqueness/state transition
rules. Add one-field identity, duplicate, reorder, regime, and outbox-ID
poisons.

### Verdict

`HOLD / the corrected V2 model closes much of the former surface, but still
admits contradictory checkpoint custody and forged retained identities at
a38e2cfa1`.

This verdict supersedes the Review B amendment verdict at `ac8cb0eda`; another
correction and independent re-review are required before promotion.

---

## Restart V2 final amendment re-review — candidate `887d92ec557f22682cc5e4df048a20aa249d2cbf`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent mutation probes`

### Gate and closure evidence

- reference one-slab suffix equality => PASS;
- declared restart V2 poisons => PASS, 34/34;
- the prior six reframed probes (active regime, resource final use, cumulative
  debit, material amount, reduction operand ID, publication record ID) now all
  reject with the intended typed category;
- strict BEI, science-contract unit-compliance, and poison JSON parsing => PASS.

The correction materially closes the two findings from `a38e2cfa1`: resource
and material receipts now reconstruct cumulative/staged custody for the
fixture, active regime is checked, and reduction/publication identities are
derived. A final closed-body/outbox audit found two remaining blockers.

### `V11-RESTART-V2-B-004` — BLOCKER — current sequence and nested event/state bodies remain open

The outer current parent sequence is checked only against its claimed
successor. Changing `parent_transaction_sequence` from 24 to 30 and
`next_parent_transaction_sequence` from 25 to 31 is accepted while the
checkpoint beginning/staged states remain at last sequence 23. Current
chronology therefore is not joined to the typed checkpoint.

The top-level, coupled, checkpoint, segment, slab, resource, material,
reduction, and publication shapes now receive exact-key checks, but V11 state
bodies and event bodies do not. Adding an unknown field to checkpoint
`beginning_state` is accepted. Adding an unknown field to a digest-valid event
body is accepted. Changing the event's `source_owner` from snow to BGC is also
accepted while the staged snow-to-surface custody changes remain unchanged.

Required closure: exact-key/type validation for beginning/staged V11 state and
event bodies; join checkpoint last-parent sequence plus one to outer current
sequence; derive event source/receiver/transfer custody from beginning and
staged owner envelopes. Add current-sequence-pair, nested-state unknown,
event-unknown, source/receiver, transfer, and owner-ending poisons.

### `V11-RESTART-V2-B-005` — BLOCKER — outbox identity, uniqueness, and binding are not authenticated

The reference checks record membership and delivery-count polarity only. A
`CommittedUndelivered` row with an all-zero `outbox_id` is accepted, and two
identical outbox rows are accepted. No outbox ID derivation, exact body set,
uniqueness, or one-record/one-outbox cardinality is enforced.

Required closure: define and reconstruct outbox ID from record and parent
identity, exact-check the outbox body, enforce unique IDs/record bindings and
valid state/count transitions, and add forged-ID, duplicate-ID, duplicate-
record, unknown-field, and impossible-transition poisons.

### Verdict

`HOLD / the prior six findings are closed, but current chronology, nested event
custody, and durable outbox identity remain forgeable at 887d92ec5`.

This verdict supersedes the amendment re-review at `a38e2cfa1`. Promotion still
requires one bounded correction and independent confirmation.

---

## Restart V2 independent final Review B — candidate `937aadb329ced16f050c676f89769fec2d8f5efe`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent reframed probes`

### Gate evidence

- restart V2 independent reference => PASS complete-continuation digest and
  52/52 declared poisons;
- all twelve probes retained from the two preceding amendment reviews now
  reject;
- restart schema JSON, strict BEI, and science-contract unit-compliance =>
  PASS;
- V11 authority contract test => PASS, 6/6.

The correction closes current/next sequence, nested state/event body shape,
event-owner custody, typed LSE/thermal state shape, ordered unique reductions/
publication, reconstructed outbox identity, and full ending-owner continuation
for the reference fixture. One exact checkpoint-authentication blocker remains.

### `V11-RESTART-V2-B-006` — BLOCKER — accepted-segment predecessor and ending-owner fields are ignored

Two independent mutations were reframed with a valid checkpoint digest and
accepted through complete continuation:

- replace the accepted segment's `beginning_state_sha256` with 64 zeroes;
- replace `accepted_segments[0].ending_complete_owners[0]` with the beginning
  vegetation owner envelope while leaving the checkpoint/outer staged owner
  set unchanged.

The validator exact-checks the accepted-segment field names but never
reconstructs `beginning_state_sha256` from the checkpoint beginning/staged
predecessor and never joins the segment's ending complete owners to its ending
state, the checkpoint staged set, or the outer staged set. `restore_suffix`
uses the outer staged owners, so contradictory retained segment custody is
silently bypassed. The fixture's claimed beginning digest is itself not derived
from the embedded beginning state.

Required closure: define and reconstruct the canonical beginning-state digest;
require each segment's beginning digest to equal the previous accepted ending
state; typed-admit `ending_complete_owners`; require the terminal segment
ending set to equal checkpoint and outer staged owners byte-for-byte; and add
forged predecessor, reordered/missing/changed segment ending owner, and stale
terminal-staged-set poisons.

### Verdict

`HOLD / all prior Review B probes are closed, but accepted-segment predecessor
and ending-owner custody remain independently forgeable at 937aadb32`.

The amendment is close, but PASS is prohibited until every retained checkpoint
field participates in authenticated continuation.

---

## Restart V2 final regression Review B — candidate `5918d4dbdfd0a7641d16b1f5f2040289c9893788`

Date: 2026-08-20

Status: `PASS`

Evidence mode: `Static + Ran + independent reframed probes`

### Gate evidence

- restart V2 reference => PASS complete-continuation digest and 54/54 declared
  poisons;
- every direct probe retained from all prior amendment reviews rejects,
  including the exact zero predecessor digest and terminal vegetation-owner
  substitution forms;
- restart V2 schema and poison JSON parse => PASS;
- strict BEI and science-contract unit-compliance => PASS for both contracts;
- V11 authority contract test => PASS, 6/6.

### Final finding closure

- `V11-RESTART-V2-B-001..003`: CLOSED by complete checkpoint-shaped admission,
  typed owner parsing, exact coupled/configuration/controller/cursor joins,
  receipt/event/scheduled reconstruction, resource/material staged custody,
  ordered reduction/publication identity, and suffix-equivalent continuation.
- `V11-RESTART-V2-B-004`: CLOSED. Current/next/predecessor sequences join;
  nested state/event bodies are closed; event source/receiver/participants and
  transfer amount reconstruct the admitted snow-to-surface owner transition.
- `V11-RESTART-V2-B-005`: CLOSED. Outbox row shape, identity derivation,
  ordering/uniqueness, record binding, and state/count rules execute and reject
  the prior forged/duplicate forms.
- `V11-RESTART-V2-B-006`: CLOSED. Every segment beginning digest is derived
  from the preceding accepted state, ending state digest is reconstructed,
  support predecessors chain, and the terminal segment ending-owner set must
  equal both checkpoint and outer staged owner sets byte-for-byte.

The complete continuation comparison now includes all seven ending owners,
accepted event/scheduled chronology, reduction operands, publication/outbox,
resource/material prefix receipts, and successor sequence. No Review B waiver
is required.

### Verdict

`PASS / all Restart V2 Review B findings are closed at 5918d4dbd; the amendment
may proceed to independent verification`.

This supersedes every earlier Restart V2 Review B HOLD. It does not itself
authorize production implementation before the package records the required
verification and promoted authority checkpoint.

---

## Sequential-debit amendment Review B — candidate `3065c209c7d5d203a2a06fca793dc8cbc340e26e`

Date: 2026-08-20

Status: `PASS`

Evidence mode: `Static + Ran + independent binary64 reframing`

### Transaction, restart, and serialization audit

- Version 19 makes the accepted-order staged subtraction the sole
  authoritative resource-owner evolution. Each segment's declared ending is
  reconstructed from its current staged beginning and becomes the next
  segment's bit-identical beginning.
- The `+0.0`-seeded accepted-order cumulative debit remains a separate receipt
  diagnostic. Parent closure must reconstruct both folds and expressly cannot
  gate or replace the sequential owner ending with
  `parent_beginning - cumulative_debit`.
- The frozen three-segment case separates these identities for water, NH4, and
  NO3. Independent bit probes confirmed that every sequential terminal differs
  from its regrouped alias, while every cumulative total matches its declared
  bits. Both wrong-regrouped fixtures reject with `VEG-E-124`.
- Restart V2's closed wire is unchanged. Its retained ordered segment resource
  bodies, staged owner envelopes, and cumulative debit rows contain the inputs
  needed to reconstruct the two folds independently on admission; Version 19
  changes their semantic interpretation without changing their canonical
  representation. The released schema, poison population, and reference
  calculator hashes remain respectively `af9314c3...2441`,
  `fa5ae93f...ad34`, and `13f3d009...f7c`.

### Gate evidence

- segmented-support reference population => PASS, 49/49;
- restart V2 reference => PASS, 54/54, complete-continuation digest
  `512c259b...f0e`;
- V11 authority contract => PASS, 7/7;
- strict binding-exposure and science-contract unit-compliance => PASS for
  both amended contracts;
- amendment `git diff --check` => PASS;
- independent sequential/cumulative/regrouped binary64 probe => PASS for all
  three resource classes.

### Verdict

`PASS / SC-VEGETATION-001 Version 19 unambiguously separates authoritative
sequential custody from cumulative receipt identity; no Review B finding or
waiver remains at 3065c209c`.

Production and restart admission must implement both reconstructions; this
authority review does not admit a regrouped shortcut or authorize release
before the amendment's independent verification and promotion checkpoint.

---

## Resource-custody amendment Review B — candidate `1302b60b9c4d07f28e58c92a30dce6f39cd70c8e`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + independent wire/restart reframing`

The amendment correctly recognizes that occupancy vegetation-use receipts are
not shared hydrology/BGC inventory predecessors. Its proposed split into typed
debits and shared-owner transitions is directionally required, and the nominal
54/54 reference, unchanged Restart V2 54/54 reference, authority 7/7, schema
meta-validation, strict binding exposure, unit compliance, and diff hygiene all
pass. Four authority closures remain before release.

### Findings

- `V11-RESOURCE-CUSTODY-B-001` — **Restart V2 omits the new transition
  authority.** The closed `OPENWEPP_C3_WOODY_V11_RESTART_V2` schema and its
  checkpoint segment retain old resource receipts but contain neither shared
  transition bodies nor transition IDs. Its unchanged reference likewise
  reconstructs no transition. A mid-parent restore therefore cannot prove the
  transition ending-to-next-beginning chain, exact debit-link coverage, or
  candidate digest already accepted before the checkpoint. Add the closed
  typed transition collection and its joins to the additive restart wire, or
  introduce an explicitly reviewed successor wire; add fresh suffix and
  missing/duplicate/reordered/forged transition poisons.
- `V11-RESOURCE-CUSTODY-B-002` — **Debit identity is not closed to the shared
  owner key.** `INV-VEGTRANSACTION-009` requires inherited
  parent/owner/OFE/tile/occupancy/layer/species/basis identity, but the new debit
  schema lacks owner, OFE, tile, and basis. The reference joins a debit to a
  transition using only slab/resource/layer/source, so a receipt can be linked
  across OFE/owner/basis while remaining schema-valid. Freeze the exact water
  and NH4/NO3 identity projections and require equality with the transition and
  current staged owner.
- `V11-RESOURCE-CUSTODY-B-003` — **Candidate, ordering, and cardinality
  authentication is not executable.** The oracle checks only that
  `owner_candidate_sha256` is 64 lowercase hex characters; changing it to an
  arbitrary valid digest is accepted. Reversing `ordered_debit_receipt_ids` is
  also accepted, and an extra same-key transition with a duplicate transition
  ID, an out-of-parent slab, no links, and a new ending is accepted. The schema
  has no global transition identity/ordering/cardinality constraints and no
  admitted candidate body or lineage from which its digest is reconstructed.
  Bind transition ID derivation, canonical transition and link order, uniqueness
  and one-transition-per admitted key/slab rules, support membership, exact
  other-flux alternative semantics, and owner-candidate bytes/digest linkage;
  add digest-valid and structurally valid reframed poisons.
- `V11-RESOURCE-CUSTODY-B-004` — **The package authority artifact remains
  contradictory.** `resource-staging-and-arbitration.md` still states that
  final use alone forms the next staged owner and that authoritative custody is
  `current_staged_beginning - admitted_amount`, immediately before stating that
  occupancy debit rows are not shared-owner inventory and transitions own the
  predecessor. Remove the superseded rule and state the debit diagnostic and
  shared-owner transition chronologies once, consistently.

The current wrong-transition poison is only a malformed short digest, so it
does not address `B-003`. The new population exercises water only; equivalent
typed BGC NH4 and NO3 cases remain necessary to prove that species identities
cannot collapse or alias the shared BGC owner.

### Verdict

`HOLD / SC-VEGETATION-001 Version 20 cannot release at 1302b60b9 until the
restart wire persists transition custody, debit identity closes to the shared
owner, candidate/order/cardinality rules are executable, and the contradictory
staging artifact is reconciled`.

No production implementation is authorized by this review.

---

## Resource-custody final re-review B — candidate `9020f3dcb4cabfde3517f3ee5e23142c8517ce50`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + coordinated identity/continuation reframing`

The correction closes substantial portions of the prior review. Debit bodies
now carry owner/OFE/tile/occupancy/layer/source/basis identity; transition
ordering, support membership, link coverage, cross-owner/OFE/tile/basis
poisons, candidate digest syntax, terminal joins, and the contradictory staging
prose are improved. Nominal gates pass: segmented 54/54, immutable V2 54/54,
V3 12/12, authority 8/8, all three schema meta-checks, strict BEI, unit
compliance, and diff hygiene. V2 schema/reference hashes remain
`af9314c3...2441` and `13f3d009...f7c`.

Two Review B authority findings remain.

### Residual findings

- `V11-RESOURCE-CUSTODY-B-001` — **not closed: V3 is not a complete restart
  successor.** `OPENWEPP_C3_WOODY_V11_RESTART_V3` contains only parent/support,
  other-flux IDs, debit/transition arrays, and terminal owners. It neither
  embeds nor authenticates the complete V2 checkpoint: coupled-time V2,
  vegetation parent checkpoint and staged state, beginning owners,
  current/next sequence, cursor/regime/participants, accepted event/scheduled/
  material lineage, reductions, publication, and outbox are absent. The V3
  reference validates a static custody envelope and executes no restored suffix.
  Consequently the contract's claim that restore resumes persisted staged state
  and equals uninterrupted continuation is not executable. Make V3 a closed
  additive superset/composition of byte-identical V2 plus typed custody, join
  duplicated facts, and compare a real fresh-object suffix over complete state.
- `V11-RESOURCE-CUSTODY-B-003` — **partially closed: IDs and complete owner
  candidates remain caller-reframable.** The reference does not derive debit
  receipt IDs or transition IDs from their bodies. Coordinated replacement of a
  debit ID and all links is accepted, as is arbitrary replacement of a
  transition ID. Coordinated replacement of an admitted other-flux ID and all
  transition links is also accepted because V3 retains no authenticated flux
  bodies or V2 receipt join. Further, `owner_candidate_bytes_base64` decodes
  only `{owner, ending_bits}`. For BGC, separate NH4 and NO3 scalar candidates
  exist but only the last transition for owner `bgc` is joined to the terminal
  owner; this is not a canonical complete BGC owner candidate and leaves the
  other species transition outside terminal custody. Derive all IDs with frozen
  domains/field order, authenticate other-flux receipts through the complete
  checkpoint, and bind transitions to one reconstructable complete owner
  candidate per owner/slab (or an exact admitted component-to-complete-owner
  aggregation with all components joined).

Prior `V11-RESOURCE-CUSTODY-B-002` and `B-004` are closed by the expanded
identity bodies/alias poisons and reconciled staging artifact. The previous
candidate-digest, reversed/missing/duplicate link, extra/out-of-support
transition, and cross-owner/OFE/tile/basis probes now reject, but that does not
close the coordinated-ID and complete-BGC-candidate forms above.

### Verdict

`HOLD / SC-VEGETATION-001 Version 21 remains non-releasable at 9020f3dcb;
Restart V3 must carry complete V2 continuation and reconstruct receipt,
transition, flux, and complete-owner candidate identities before Review B can
PASS`.

No waiver or production authorization is recommended.

---

## Resource-custody final bounded Review B — candidate `bf2c288c4e1010c47042078c362925db747d46b1`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + exact support/continuation reframing`

The correction closes the prior coordinated identity and component-candidate
findings. Debit, transition, and typed other-flux IDs are now body-derived;
linked components bind one candidate per owner/slab; NH4 and NO3 share the same
BGC candidate; declared candidate/link/order/cardinality/terminal poisons
reject. All requested nominal gates pass: segmented 54/54, immutable V2 54/54,
V3 14/14, authority 8/8, schema meta-validation, strict BEI, unit compliance,
and diff hygiene. Direct reruns of cross-owner/OFE/tile/basis, extra transition,
duplicate link, and missing link probes reject.

One release-blocking continuation contradiction remains under
`V11-RESOURCE-CUSTODY-B-001`.

### Residual finding

The admitted V3 control is not joined to the embedded V2 clock support. V3
declares parent support `[0,1800)` and transition slabs `[0,900)` and
`[900,1800)`, while the byte-identical embedded V2 coupled-time checkpoint owns
`[0,1800000000000)` and is already accepted through `600000000000`. The current
validator admits this exact mismatch. It checks only parent transaction ID
across the composition; parent support, accepted cursor, slab/segment identity,
and accepted-prefix chronology are never joined.

Consequently `restore_suffix()` merely reruns the unchanged V2 suffix and
returns its preexisting digest. It does not consume or compare V3 debit,
transition, candidate, or terminal-owner state in the restored continuation.
V3 also fixes exactly two slab ordinals (`0..1`) and exactly fourteen owner
candidates, which cannot represent the authority's arbitrary accepted-slab
cursor or a checkpoint after a different number of accepted slabs.

Required correction: use the exact V2 integer support/cursor and admitted slab
receipts in every V3 debit/flux/transition/candidate join; derive candidate
cardinality from persisted accepted chronology rather than a two-slab fixture;
embed V3 custody into the staged checkpoint restored by the suffix; and compare
complete post-suffix V3 owner/custody/receipt bytes with an independently run
uninterrupted V3 parent. Add support-unit, cursor/prefix, variable-slab-count,
and custody-affects-suffix poisons.

### Verdict

`HOLD / SC-VEGETATION-001 Version 22 cannot release at bf2c288c4 because the
accepted Restart V3 fixture has contradictory V2/V3 time support and the
claimed actual suffix does not continue V3 custody state`.

Prior coordinated-ID and complete-candidate findings are closed. No waiver or
production authorization is recommended.

---

## Resource-custody cross-wire Review B — candidate `e97f1683b5de8615e5c45b62aae2e346d3ca8d1c`

Date: 2026-08-20

Status: `HOLD`

Evidence mode: `Static + Ran + coordinated cross-wire identity probes`

The correction closes the prior time-scale, cursor projection, prefix-only
collection, dynamic `7 x accepted_slabs` cardinality, terminal V11 state, and
terminal staged-owner joins for the frozen control. V2 54/54, V3 10/10,
authority 8/8, schema meta-validation, strict BEI, unit compliance, and diff
hygiene pass. Prior support-scale, cursor, prefix substitution,
missing/extra-candidate, terminal-owner, forged-candidate, missing-link, and
suffix-not-consumed poisons reject.

One exact-identity/independence portion of `V11-RESOURCE-CUSTODY-B-001` remains
open.

### Residual finding

Debit and transition bodies are proven internally hash-consistent but their
`parent_transaction_id` and `segment_id` are not required to equal the embedded
V2 parent and admitted segment. A direct coordinated probe changed both fields
in one debit and its transition, rederived the debit/transition IDs and
component link, and was accepted. Its complete suffix digest changed, proving
the forged prefix entered continuation without failing the cross-wire join.

Typed other-flux receipts have the same gap. They are checked for derived ID
and admitted slab ID, but their parent, segment, support, owner, OFE, layer,
source, and basis are not required to equal the linked transition. A
schema-valid `snow` flux was linked into a hydrology transition and accepted.

Finally, the claimed uninterrupted comparison is not independent:
`restored = execute_suffix(c, consume)` and
`full = execute_suffix(c, True)` both start from the same caller-supplied V3
checkpoint. Coordinated prefix reframing therefore changes both sides. This
proves suffix consumption, but not equality to an independently constructed
uninterrupted parent chronology.

Required correction: join every debit, transition, and flux parent/segment to
the exact V2 parent and admitted slab/segment projection; join every linked
flux's complete owner/OFE/layer/source/basis/support identity to its transition
under an explicitly admitted flux-class direction rule; and construct the
uninterrupted comparison independently from frozen parent beginning inputs and
forcing rather than the restored checkpoint. Add coordinated parent/segment,
cross-owner flux, cross-basis flux, and shared-origin self-consistency poisons.

### Verdict

`HOLD / SC-VEGETATION-001 Version 23 cannot release at e97f1683b because
cross-wire parent/segment and typed-flux identity joins remain incomplete and
suffix equality is checkpoint self-consistency rather than an independent
uninterrupted reconstruction`.

No waiver or production authorization is recommended.
