# Independent Terminal Verification B — Occupancy-Identity Remediation

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
  `422f0a6fb778de73568259b0d1bad19f63e5b6fcac5fd608accace45b316bcd2`.

No unresolved material finding remains. The V8 amendment, V4 definition,
independent generator and fixture, and production canonical encoder agree on a
structural occupancy array. Each element contains the exact separate
`identity` and `state` objects. Canonical ordering is lexicographic over the
independent `(stratum_id UTF-8 bytes,tile_id UTF-8 bytes)` pair. The typed-line
encoder length-frames each string independently; no delimiter-composed key or
delimiter reconstruction remains in the whole-state digest route.

Independent inspection and reconstruction confirmed:

- the rejected flattened forms for `("a@b","c")` and `("a","b@c")` are
  both `a@b@c`, while their structural preimages and SHA-256 values differ;
- the separate Greek/CJK vector preserves `@`, tab, and NUL as framed string
  content rather than treating them as syntax;
- reversing the occupancy input array normalizes to the same canonical digest;
- a repeated typed pair returns `VEG-E-087 duplicate_occupancy` before a V4
  preimage or digest can be emitted;
- the committed canonical preimage is exactly 25,001 bytes and hashes to
  `4b4cd487c67abd26439014e0bb75dd4edaa79c06ceb23759aeae1e37a7e12de4`;
- all 26 fixture checks are true; and
- all 155 whole-state scalar mutations differ from the base digest and from
  one another, including the stratum and tile identity fields of both lanes.

The production encoder constructs the same
`{identity:{stratum_id,tile_id},state}` nodes from a typed
`BTreeMap<OccupancyId, OccupancyState>`. Its strict persisted-state parser
rejects duplicate occupancy entries before state validation. Verifier-run
focused tests passed for the complete released whole-state preimage and all 155
mutations, structural arbitrary-ID collision safety, and duplicate occupancy
rejection. A fresh reference-calculator run preserved all three frozen hashes,
and post-run diff hygiene passed.

The definition is the exact identity recorded by `SC-VEGETATION-001@8`, the
production model registry, and the committed V4 diagnostic identity. The
protected predecessor definitions remain byte-identical:

- V1 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`;
- V2 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`;
- V3 `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`.

Both post-remediation science rereviews are GO with no unresolved material
finding. Reviewer A's initial duplicate-policy HOLD is preserved, accepted,
corrected, and rerun to GO. The finding disposition defers or rejects no part
of the occupancy-identity defect.

The authoritative heavy campaign is
`artifacts/run-20260813T030632Z-ZzxORt/`. Its fingerprint binds the exact
commit and three frozen hashes above. The six return-code files are all zero,
and their log hashes reproduce the fingerprint. The logs record:

- workspace warnings-denied Clippy: PASS;
- full-workspace nextest: PASS, 2,582 passed and 33 skipped;
- workspace doctests: PASS;
- dependency audit: PASS with only the recorded non-fatal unmatched-license
  allowance warning;
- formatting: PASS; and
- diff hygiene: PASS.

The prior HOLDs, failed/interrupted/capacity-limited gate attempts, science
reviews, COMPLETE checkpoint, and capacity-correct historical heavy campaign
remain present as historical evidence. The archived kickoff prompt remains at
SHA-256
`7f31e3a82634aaab31aa9de2d4bf5ac9bfd34c11241671fb3a80685b6839df25`;
it is not relabeled as the remediation directive.

This verification is limited to corrected V4 shared-state implementation
authority. It claims no new constitutive science, Rust ownership, completed
vegetation implementation, runtime activation, selector change, production
consumer cutover, calibration, empirical validation, or transferability.
`calibration_evidence_status=NOT_CALIBRATION_READY`,
`identifiability_status=NOT_ASSESSED`, and `GAP-VEGETATION-027` remains
fail-closed.

Terminal verifier B returns `PASS` for the post-release occupancy-identity
remediation candidate.
