# Independent Terminal Verification A — Occupancy-Identity Remediation

Status: `PASS`

Evidence mode: `Static + Ran`

Exact terminal candidate verified:

- stable commit
  `916f24181e250d1cee5b17d9985bb082b7b53a3f`;
- V4 definition
  `8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`;
- independent fixture
  `3072226f1d80359c548d87c1fa222be0c20b01627d9117e39163c39d9eb8824d`;
- reference generator
  `422f0a6fb778de73568259b0d1bad19f63e5b6fcac5fd608accace45b316bcd2`;
- V8 shared-state amendment section
  `5a47df7d2b60cfb0acb24c9ea6c14a8560e9a6cc539a6f7f00bc96a3f4454fc7`.

No unresolved material finding remains. The corrected V8 contract, definition,
oracle, fixture, and production canonical encoder consistently represent each
occupancy as an independently framed structural
`(stratum_id,tile_id)` pair. Ordering uses the two UTF-8 byte strings rather
than a delimiter-rendered key. Duplicate pairs reject before preimage or digest
emission.

Independent inspection and reconstruction verified:

- the complete 25,001-byte whole-state preimage has SHA-256
  `4b4cd487c67abd26439014e0bb75dd4edaa79c06ceb23759aeae1e37a7e12de4`;
- all 26 fixture checks are true;
- all 155 whole-state scalar mutations have distinct digests, including both
  identity components of both occupancy lanes;
- reversing lane input order normalizes to the same digest;
- `("a@b","c")` and `("a","b@c")` share the rejected flattened spelling
  `a@b@c` but have distinct typed preimages and digests;
- Greek, CJK, `@`, tab, and NUL identity content remains length-framed data;
- the direct V4 duplicate structural-pair poison returns
  `VEG-E-087 duplicate_occupancy` before hashing; and
- isolated regeneration reproduced the exact definition and fixture bytes.

The focused production test
`production_whole_state_encoder_matches_released_structural_preimage_and_digest`
passed and consumed the committed preimage, digest, and all 155 mutation
vectors. Production parsing separately rejects duplicate structural occupancy
entries.

The protected predecessor definitions remain unchanged:

- V1 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`;
- V2 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`;
- V3 `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.

Both remediation science rereviews are GO and every accepted finding is
dispositioned. The heavy campaign in
`artifacts/run-20260813T030632Z-ZzxORt/` is bound to the exact commit and three
candidate hashes above. Its six commands have return code zero: workspace
Clippy, full-workspace nextest, workspace doctests, dependency audit,
formatting, and diff hygiene. The nextest log records 2,582 tests passed and 33
skipped; no test failed.

The prior COMPLETE checkpoint, archived prompt, reviews, failed attempts, and
heavy evidence remain immutable historical records. The remediation does not
claim new constitutive science, V4 Rust completion, runtime activation,
selector change, real-consumer cutover, calibration, empirical validation, or
transferability. `GAP-VEGETATION-027` remains explicitly fail-closed.

Terminal verifier A therefore returns `PASS` for the post-release
occupancy-identity remediation candidate.
