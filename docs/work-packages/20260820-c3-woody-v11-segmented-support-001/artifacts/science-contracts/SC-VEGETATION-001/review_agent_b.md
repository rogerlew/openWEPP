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
