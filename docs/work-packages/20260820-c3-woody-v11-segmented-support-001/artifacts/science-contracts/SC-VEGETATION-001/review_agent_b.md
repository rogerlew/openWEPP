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
