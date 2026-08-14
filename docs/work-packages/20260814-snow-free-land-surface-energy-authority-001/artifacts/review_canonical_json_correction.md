# Canonical-JSON Correction Review

Evidence class: `Static + Ran`

Verdict: `PASS`

Reviewed exact worktree bytes on `2026-08-14` against the released Child-1
baseline commit `3f1cf8ee3`. This review is limited to the canonical-JSON
evidence-portability correction and does not assess the in-progress Child-3
constitutive implementation.

## Findings

### `JSON-REVIEW-HIGH-001` — final placeholder incorrectly says fixture bytes are unchanged

Disposition: `accepted / corrected before final review`

`artifacts/final-disposition.md` currently says that “fixture bytes are
unchanged.” They are intentionally changed: the vector digest moved from
`7b6a303a...` to `9f171b0f...`, and an independent structural comparison found
four changed scalar paths. The physical values and accepted scientific results
are unchanged, but the fixture bytes are not. The same placeholder continues
to list the old calculator and vector hashes without explicitly identifying
them as the historical release hashes.

Final review: corrected. The placeholder now says that physical and accepted
values are unchanged while exactly two copies of the strict-state digest and
their two dependent instance hashes changed. It labels `1156fa88...` and
`7b6a303a...` as historical hashes and binds calculator `9278be79...` and
vectors `9f171b0f...` as the correction candidate.

No implementation, physics, schema, model-identity, or numerical finding was
identified.

## Static Assessment

1. `canonical_digest()` first serializes the already typed value with
   `serde_json`, then applies `cpython_json_exponents()` only to the serialized
   digest bytes. The scanner tracks quoted and escaped-string state. Outside a
   string, it pads only a signed one-digit lowercase exponent; signed
   multi-digit exponents pass unchanged. Because its input is valid JSON
   emitted by `serde_json`, the matching token can only be a number. It does
   not mutate the typed binary64 value or return normalized JSON to a caller.
2. The poison test uses both a numeric `1e-7` and exponent-like content in
   ordinary and escaped strings. Only the numeric token becomes `1e-07`.
3. The strict configuration and strict state deserialize to typed Rust and
   validate against their embedded canonical digests. The independently
   reconstructed digests are respectively
   `45a5d1411043d06bf84ffc43ff22ef76da0cd7597dd47a2ef255ae2e3d684242`
   and
   `6ff22f0d72b6c4fdad3c0d8a0b2947571191e48213635609af8f3b951c07abf1`.
4. `reference_calculator.py` deep-copies the strict state, completes the
   configuration identity, forest-tile identity, enthalpy, and warm-start
   projection, and only then replaces `state_sha256` with the digest of those
   final bytes. The identical final state is installed in both
   `strict_schema_instances.state` and
   `coupled_transaction.beginning_lse_state`.
5. A recursive parsed-value comparison against commit `3f1cf8ee3` found
   exactly four changed paths:

   - the embedded strict-state digest in the standalone state;
   - the same digest in the coupled envelope;
   - the dependent standalone-state instance digest;
   - the dependent coupled-transaction instance digest.

   Every model, source-checksum, inherited V8, model-reduction, mandatory
   scenario, complete transaction, competition, condensation, equilibrium,
   owner-candidate, multi-OFE, failure, poison, and reconstructed-invariant
   section compared equal as parsed data.
6. LSE definition `e1736b8c...`, V8 definition `622bc900...`, all six schema
   hashes, and joint core `c9555b2d...` are unchanged. The only source change
   in the generator is the final-projection digest correction and its comment;
   its checksum is now `9278be79...`.
7. The exponent adaptation is reachable only through the configuration,
   state, and forcing identity digest functions. No call occurs in physics,
   residual, convergence, authorization, closure, or solver acceptance code.
   No comparison tolerance, solver tolerance, physics tolerance, model digest,
   schema, or hidden numerical normalization changed in this correction.

## Ran Evidence

| Command/check | Result |
|---|---|
| `.venv/bin/python .../reference_calculator.py --write /tmp/review_vectors.json` | PASS; generated SHA-256 `9f171b0f...` |
| `cmp /tmp/review_vectors.json .../openwepp_snow_free_lse_v1_vectors.json` | PASS; byte-identical independent regeneration |
| Independent Python canonical recomputation of strict configuration/state | PASS; both embedded digests exact; coupled and standalone state bytes equal |
| Recursive parsed comparison against `3f1cf8ee3` fixture | PASS; exactly the four dependent digest paths above changed |
| `cargo nextest run -p openwepp-land-surface-energy --profile quick` | PASS; 21/21 |
| `cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick` | PASS; 7/7 |
| `cargo clippy -p openwepp-land-surface-energy --all-targets -- -D warnings` | PASS |
| `markdown-doc lint` on this review and corrected final placeholder | PASS; 2/2 files, zero errors or warnings |

## Conclusion

The bounded serialization and stale-state-digest correction is technically
sound and does not alter science or solver acceptance.
`JSON-REVIEW-HIGH-001` is corrected on the reviewed exact bytes. No unresolved
finding remains in this review scope.
