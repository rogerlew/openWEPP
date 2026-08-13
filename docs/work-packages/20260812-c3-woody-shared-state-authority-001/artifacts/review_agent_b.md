# Independent Science Review B

Evidence: `Static + Ran`

Disposition: `GO`

No unresolved material finding. The reviewer verified current generator,
fixture, and definition binding; byte-identical regeneration; displayed leaf
C/N ownership; fail-closed checks; two-stratum/two-occupancy migration;
distinct V4 configuration rebinding before state digest; exact owner/transaction
lineage; predecessor preservation; and one-LF definition serialization.

The initial HOLD, later typed pending-transfer HIGH, exact-width MEDIUM, and all
accepted corrections remain preserved in `review-finding-disposition.md` and
`gate-results.md`. The final exact-byte rereview returned GO with no unresolved
material finding.

## Post-release Occupancy-Identity Remediation Rereview

Evidence: `Static + Ran`

Disposition: `GO`

Frozen identities reviewed:

- definition
  `8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`;
- fixture
  `3072226f1d80359c548d87c1fa222be0c20b01627d9117e39163c39d9eb8824d`;
- generator
  `422f0a6fb778de73568259b0d1bad19f63e5b6fcac5fd608accace45b316bcd2`.

No unresolved material finding. The V8 amendment and V4 definition now bind
the same structural representation used by the production canonical encoder:
an occupancy array whose elements contain separate `identity` and `state`
objects, ordered lexicographically by the independent stratum-ID and tile-ID
UTF-8 byte strings. Each identity component is length framed by the typed-line
encoder. No delimiter-rendered occupancy key or delimiter reconstruction
remains in the V4 digest route.

Independent reconstruction reproduced the committed 25,001-byte whole-state
preimage and its SHA-256
`4b4cd487c67abd26439014e0bb75dd4edaa79c06ceb23759aeae1e37a7e12de4`.
It also independently reproduced all 155 committed scalar-mutation digests,
including both identity components of both occupancy lanes, and confirmed that
every mutation differs from the base digest. Reversing input lane order
normalizes to the same digest, while a duplicate structural pair rejects before
V4 digest calculation.

The executed delimiter poison uses distinct pairs `("a@b","c")` and
`("a","b@c")`. Both have the rejected flattened rendering `a@b@c`, but their
typed preimages and SHA-256 values are distinct. The separate Greek/CJK vector
contains `@`, tab, and NUL bytes and confirms those bytes remain length-framed
string data rather than syntax. Focused production tests passed for the exact
committed preimage/digest, structural collision behavior, and duplicate-pair
rejection (`3 passed; 151 skipped`).

The definition-to-fixture, definition-to-generator, definition-to-V8-section,
and contract-to-definition bindings reproduce exactly. V1, V2, and V3
definitions retain their protected SHA-256 identities. The prior COMPLETE
checkpoint, failed attempts, reviews, heavy-gate evidence, and terminal
verification remain present as explicitly historical evidence rather than
being rewritten as evidence for this remediation.
