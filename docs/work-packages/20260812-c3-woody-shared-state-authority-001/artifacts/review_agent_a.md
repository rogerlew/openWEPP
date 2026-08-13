# Independent Science Review A

Evidence: `Static + Ran`

Disposition: `GO`

No unresolved material finding. The reviewer verified exact shared schema,
displayed leaf C/N ownership, exact-bit area caches, injective typed state
encoding, fail-closed V3 migration identity/membership/lineage, independent
fixture checks and mutations, strict scalar/transfer domains, preserved
predecessor bytes, and explicit `GAP-VEGETATION-027`.

The initial review findings were accepted and corrected; see
`review-finding-disposition.md` and retained gate history.

## Post-release occupancy-identity remediation rereview A — first pass

Evidence: `Static + Ran`

Disposition: `HOLD`

Reviewed exact candidate identities:

- definition: `7db77c0db065fb4aee4ab3ae04206fc7b90a216905b873a13bfbbe52346a4321`;
- fixture: `67bf42244d55745e2705dd96d01e1a51725b2df387ffc7fb4930a2797e4d3f5c`;
- generator: `9c22d6c9e5e2bda8768221da3da15fcb7fbe326062a97b0674d8d029a176f7b2`.

The V8 amendment and definition correctly select a structural occupancy array,
typed-pair UTF-8-byte ordering, independently length-framed identity strings,
and rejection of duplicate structural pairs before digest calculation. The
fixture also demonstrates the `("a@b","c")` versus `("a","b@c")` collision,
Greek/CJK/control-character framing, canonical input-order normalization, a
production-consumable complete preimage, and identity-field mutation digests.
The definition, fixture, generator, and amended V4 section are mutually bound,
and the protected V1/V2/V3 definition artifacts are unchanged.

One material finding remains:

- `A-HIGH-OCCUPANCY-001`: the independent oracle does not implement the
  selected duplicate policy on its V4 canonical-state surface.
  `sorted_occupancies()`, `state_digest()`, and
  `whole_state_preimage_digest()` sort and hash duplicate `(stratum_id,tile_id)`
  pairs instead of rejecting before digest calculation. The only executed
  duplicate poison passes through the V3 migration validator, so it does not
  prove that `OPENWEPP_V4_STATE_CANONICAL_V1` itself rejects a duplicate V4
  structural pair. This conflicts with `INV-VEGETATION-092`, the definition's
  `duplicate_policy`, and the package artifacts that claim an executed V4
  duplicate-pair poison.

Required correction: make V4 occupancy canonicalization reject duplicate typed
pairs before emitting preimage bytes or a digest, add an explicit V4 duplicate
canonical-state poison/check, regenerate all bound identities, and repeat this
rereview against the corrected exact bytes. No Rust/runtime or constitutive
finding is asserted by this review.

## Post-release occupancy-identity remediation rereview A — final pass

Evidence: `Static + Ran`

Disposition: `GO`

Reviewed exact corrected identities:

- definition: `8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`;
- fixture: `3072226f1d80359c548d87c1fa222be0c20b01627d9117e39163c39d9eb8824d`;
- generator: `422f0a6fb778de73568259b0d1bad19f63e5b6fcac5fd608accace45b316bcd2`.

`A-HIGH-OCCUPANCY-001` is corrected. Canonical occupancy normalization now
sorts by the independent `(stratum_id UTF-8 bytes,tile_id UTF-8 bytes)` tuple,
detects repeated typed pairs, and raises `VEG-E-087 duplicate_occupancy` before
preimage or digest emission. `state_digest()` and the whole-state preimage
helper inherit that fail-closed behavior, and the fixture executes a direct V4
duplicate structural-pair poison with matching expected and actual rejection.

The corrected V8 amendment, definition, oracle, and fixture consistently bind:

- an array of exact `{identity:{stratum_id,tile_id},state}` elements;
- typed-pair UTF-8-byte ordering and order normalization;
- independent byte-length/hex framing of each identity component;
- delimiter independence for arbitrary admitted UTF-8, including `@`, tab,
  NUL, Greek, and CJK content;
- distinct preimages and SHA-256 values for `("a@b","c")` and
  `("a","b@c")`, despite their equal rejected flattened rendering;
- the complete whole-state preimage bytes and digest; and
- 155 whole-state scalar mutation digests, including both typed identity
  components of both occupancy lanes.

Ran evidence:

- isolated `/home` regeneration produced the exact three identities above and
  byte-identical definition/fixture output;
- all 26 independent fixture checks passed;
- independent decoding of the committed preimage hex reproduced its committed
  SHA-256;
- the focused production-consumption test
  `production_whole_state_encoder_matches_released_structural_preimage_and_digest`
  passed (`1 passed`, `153 skipped`).

The protected V1, V2, and V3 definition files remain byte-identical to the
pre-remediation repository bytes; V4 still imports exact V3 digest
`7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.
The previous COMPLETE disposition, reviews, heavy campaign, and terminal
verification remain labeled historical rather than being rewritten as proof
of the corrected identities. This authority rereview makes no constitutive,
runtime activation, consumer-cutover, or full implementation claim.

No unresolved material finding remains in the assigned remediation scope.
