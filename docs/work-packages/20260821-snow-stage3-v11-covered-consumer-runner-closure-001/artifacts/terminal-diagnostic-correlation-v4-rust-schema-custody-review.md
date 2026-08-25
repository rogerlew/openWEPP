# Terminal diagnostic correlation V4 Rust/schema/custody/privacy review

Evidence class: `Static` and `Ran`.

Recommendation: **HOLD**.

I independently reviewed the frozen V4 set, the V3 Rust review and
disposition, the live framing helper, the six forwarding files, the named
owner modules, and the external LSE owner files. I did not read or communicate
with the numerical reviewer. I changed no source, package, authority, adapter,
census, generator, guard, or candidate-manifest file.

## Frozen-input verification

Ran `sha256sum` over the requested files. All hashes matched:

- authority: `1b0b85661fba67e9161fca2bda2e2e5cc8ca7d507f9eeceed6c93af7289aeda1`;
- adapter: `0ea6d5449b401860a9d10336b980adfab90102087488f67ba54ec032b61f1edc`;
- census: `e1ade576923c0267eb139e8b5e5fcd4930218737c1a98ca032f09e42ecd9b255`;
- tool `Cargo.toml`: `373dd0ee0aa2ea6b7d7474afb8a4ff47c03a540da55f1cdaa1e8a7f9cf8ea113`;
- tool `Cargo.lock`: `6891f3a17daf0aaf37f47ad50dec7bc4b693c15b2a7201f8463fbf83ba316b0f`;
- generator: `07175588c66b4a27ecf7dcb1449050e544fcb577351f95e42d653bc638fa9e58`;
- guard: `a8408a5a96c2eee65c5f6983152d591cbbbf5a4f45b5c70ba5392c27d1755357`.

The V4 framing statement matches live `framed_sha256`: `OPENWEPP\0`, version
`u16(1)`, `u16` domain/tag lengths, `u32` value lengths, and big-endian
integers. The crate-private unit-test boundary, sealed private evidence mode,
zero-sized `NoEvidence`, post-return serialization boundary, and explicit
external LSE owner files remain feasible in principle.

## Findings

### Critical — TDCV4-RUST-001: the replay classifier is source-file textual, not type-owned

`replay_class` ignores the declaration name and classifies a type as native
replay whenever the entire containing source file contains `replay_bytes`.
Consequently `CoveredCarrierPhaseResultV1` is classified
`1-native-replay-bytes` solely because two of its fields are WB14 replay byte
vectors. The type has no native whole-record replay wire, domain, digest, or
validator. The adapter then requires class-1 embedding of an exact type tag,
native domain, native digest, length, and bytes that do not exist.

The same file-wide heuristic can classify unrelated declarations as class 2
when any function in their file mentions `receipt_sha256`, `digest(`, or
`framed_sha256`. `native_functions` is likewise line-text filtering rather
than AST association with the reviewed type. The census therefore does not
establish the required per-type native-wire/preimage classification, and its
byte-comparison guard only preserves the misclassification.

### Critical — TDCV4-RUST-002: the census is not recursively complete

The target list freezes only 38 hand-selected top-level declarations. It
reports nested type spellings but does not resolve and census their
declarations. Examples include `OfeId`, `TileId`, `SoilLayerId`,
`ResourceOwnerId`, `Sha256Digest`, `BandDirectionalFluxes`, and the two boxed
payloads of `DirectSnowStage3EvaluationError`.

This directly breaks `typed_error_v4`: `Kernel` requires a recursive adapter
for `Wb11HydrologyKernelGuardError`, and `TurbulentTransfer` requires one for
`SnowStage3TurbulentTransferError`, but neither type is in the census. The
schema's instruction to recurse through “their census adapter” therefore has
no defined target. It also leaves identifier representation and multiple
nested LSE payloads outside the supposedly complete field/variant freeze.
There is no closed, unique byte stream for every typed error or class-3
record.

### Critical — TDCV4-RUST-003: `carrier_phase_v4` retains an undefined semantic projection

The schema names a “provider-owned projection of
`CoveredCarrierPhaseResultV1`” and says that projection contains carrier
evidence while excluding terminal-owned values. It does not enumerate the
projection's exact fields, tags, order, or transformation rules. The live
type contains `transition`, ending candidates, precipitation maps, an owner
envelope, lower boundaries, source receipts, LSE states, soil candidate,
top-boundary credit, WB14 digests, and child/parent replay bytes. Deciding
which portions survive the projection is precisely the semantic adapter
judgment V4 forbids deferring.

It is also internally unclear how “parent WB14 replay is absent” is obtained
from the classified native whole-type bytes, which necessarily include the
live optional parent fields if such whole-type replay existed. The corrected
ownership prose is sound, but the frozen wire does not implement it uniquely.

### Major — TDCV4-RUST-004: the AST tool does not prove the advertised ownership and native API facts

`owner_stage` is a name-substring heuristic, not an ownership analysis.
`module_name` derives a path from the source filename and does not resolve
inline module nesting or reexports. Validator/digest discovery scans lines
and uses a lossy lowercase stem heuristic. Visibility is captured only for
the declaration, not for each field or variant payload. “Availability of
replay bytes” is inferred from arbitrary source text rather than from an AST
field/method and its visibility/return type.

Thus the generated rows are useful declaration snapshots, but they do not
support the authority's stronger claims of fully qualified ownership, native
validator/digest/domain/preimage, replay availability, or exact required
owner-module access. Normalized declaration hashes cannot guard facts the
generator never models.

### Major — TDCV4-RUST-005: several new-record encodings are still not mechanically unique

The schema gives ordered top-level tags, but several payloads remain semantic
placeholders rather than exact wires: `key`, `prefix`, `support`, `parent`,
beginning owner/joint/provider/receipt/parcel/cursor snapshots, every
lexically discovered `last_*`, and the tagged heterogeneous rejected-prefix
sequence. It does not bind each of these to a named census type or a complete
field-level adapter. “Ascending `(qualified_name,bytes)`” cannot determine
which state locations are in the set or how their bytes are produced.

The universal primitive rule also says every `f64` is followed by a finiteness
flag, while individual records use shorthand such as census adapters and
`Option<f64+flag>` without defining whether flags are distinct framed fields,
nested values, or part of one field's value. Implementations can produce
different framed preimages while satisfying the prose.

### Major — TDCV4-RUST-006: prospective owner-file authority is incomplete for the frozen recursive claim

The authority now names important private owner modules, including external
LSE, which improves V3. But a genuinely recursive error adapter also needs
the hydrology guard-error owner file and the owners of its nested payloads.
Recursive class-3 encoding of unresolved ID, digest, flux, and error types may
require further owner modules not named in the exact prospective boundary.
Because the census did not compute the transitive closure or field-level
privacy, the asserted complete private-access boundary is not established.

## Custody, compilation, and noninterference assessment

The intended architecture remains Rust-feasible: private generic cores can be
monomorphized with a sealed zero-sized production mode, while a crate unit
test can reach `cfg(test)` capture APIs that an external integration crate
cannot. Fixed-size keys can traverse the six-file chain without exposing a
public mode or callback. Owner-module helpers can preserve privacy if their
exact functions and return wires are first authorized.

That feasibility is not implementation authority. Capture-time evidence
construction must be infallible and cannot invoke native validators or
`framed_sha256`, both of which return `Result`; those operations must remain
post-return. V4 states that boundary, but its false replay classes and open
schemas prevent determining which raw values must be moved or cloned before
return. A later implementer would have to choose representations, projection
fields, transitive adapters, and owner access beyond the frozen authority.
Those choices could change allocation, cloning, failure, or call-order
behavior and cannot be accepted under a noninterference claim without another
reviewed authority.

## Recommendation

**HOLD**

The high-level ownership and private-mode architecture is feasible, and the
live framing primitive is now stated correctly. The frozen V4 census and
adapter are nevertheless not implementable as a unique custody wire. The
type-unowned replay classifier, missing transitive declarations, absent typed
error payload schemas, undefined carrier projection, incomplete snapshot
wires, and unproven private owner-file closure are authority defects. This
HOLD authorizes no exact-file implementation intent, source edit, diagnostic
seam, receipt capture, estimator matrix, temporal operator, Batch V2, event,
receiver, restart, runner, Child 3, or cutover work.
