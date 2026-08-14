# Terminal Verification B: Final Ownership And Release Verification

Evidence class: `Static + targeted Ran` against the corrected exact current
worktree.

Verdict: **PASS / GO for Child-1 implementation-authority release**.

No material hydrology-custody, source-identity, D/A/F, advected-energy,
routing, soil-thermal, owner-envelope, rollback, reference-rights,
default-off, production-exclusion, or evidence-lifecycle finding remains.

## Exact authority bytes

The evidence-only terminal corrections did not change any canonical,
constitutive, schema, oracle, fixture, or test byte:

| Surface | SHA-256 |
| --- | --- |
| `SC-LANDSURFACEENERGY-001.md` | `67b51fde024e85668d1bb605bbb54fd58ea6b7a0e798b68db1293ebbb93a0a62` |
| `SC-VEGETATION-001.md` | `1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a` |
| `SC-VEGETATIONTRANSACTION-001.md` | `44b77e5d9854ce9c4b64214f2a669e1b09e2fdc5bdb6d1391ef25a909f186e73` |
| `SC-WATBAL-001.md` | `c30b7c243a36f7fc2aec316c3ba590c8f7629759d36bf1f91b60c0cf0c419188` |
| LSE V1 definition | `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f` |
| C3 woody V8 definition | `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b` |
| independent calculator | `1156fa88a6d7e4dd98f6dd70fe5b891f69e0b6825694179ac4d687a38907c859` |
| joint canopy-ground core | `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5` |
| frozen vectors | `7b6a303ae434ca6ad59c7082ebf486300214427d6abe20c36bfaa9b8cbdab91c` |
| configuration schema | `6499b98cc1e25f1379bc0ad6052a7536e20c4bfbb9335f9ba5c8de191ae2f009` |
| coupled transaction schema | `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f` |
| diagnostics schema | `41fb7909d073b4fdf4e59c9fa7da26b9a965ad916688b7867a56525d1bf1460c` |
| forcing schema | `f1fb785e9e582ae9e20eac4b5f44fa2b5f0651f8535d0972520dbfff3d926b55` |
| state schema | `91243e4087fa2c4775cb3629fe14c64379def4977d3c54a72348ac56d5fa4ee8` |
| water protocol schema | `2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07` |

The hashes match `terminal-diff-reconciliation.md`, both terminal reviews,
and both initial verifier reports. V1--V7 remain immutable; V8 is a separate
successor identity.

## Verification-finding closure

The initial verifier reports remain immutable FAIL evidence. Their accepted
`TVA-HIGH-001` and `TVB-HIGH-001` findings are explicitly dispositioned and
corrected across the amendment, contract-test, fixture, owner, custody,
advection, equation, schema, final-disposition, and handoff surfaces.

The first remediation reports also remain immutable FAIL evidence. Their
`TVA2-HIGH-001` and `TVB2-HIGH-001` findings independently identified the same
remaining current schema-index sentence. Both findings are now dispositioned;
that sentence states that schema digests are frozen, model-definition-bound,
fixture-bound, and confirmed. No normative JSON schema or authority byte
changed.

Historical failed science/ownership reviews and checkpoint-specific table
cells remain unmodified as historical evidence. The terminal closure matrix
still supersedes those checkpoint cells for the final hashes. Current
nonhistorical lifecycle prose now consistently records:

- dual terminal-review `PASS / GO`;
- heavy-gate PASS;
- terminal hashes frozen and confirmed;
- terminal verification/final disposition still pending only where required
  before the package is formally closed.

No accepted material finding is rejected, deferred, left to follow-up, or
undispositioned.

## Ownership, custody, and transaction result

Hydrology remains the only mutable water-mass owner. LSE owns one surface
enthalpy state and immutable keyed mass views; soil thermal owns its layer
temperatures/enthalpy. No ponded, litter, layer, routed, or condensed water is
duplicated in LSE state.

The water protocol preserves the full source key through one beginning-store
arbitration and fixed-cap rebuild:

```text
0 <= finalized use F <= maximum authorization A <= request D
```

Finalized use alone debits storage. Condensation is a distinct typed positive
credit. Precipitation, runon, throughfall, both drainage releases, and
stemflow enter only after final ET and cannot inflate the immutable
same-interval authorization inventory. There is no second authorization,
request donation, or legacy complementary PMET partition.

Advected-energy records pair precipitation, canopy release, routed runon,
infiltration, runoff, and terminal export with exact source temperature,
transaction, route, interval, OFE/tile, and mass basis. The nondegenerate
120-to-200 square-metre route preserves 72 kilograms and identical extensive
enthalpy. OFE-local tile fractions and upstream/downstream OFE areas are each
applied once.

The soil-thermal ending candidate consumes the exact opposite ground-heat
receipt plus accepted infiltration enthalpy. Its independently reconstructed
first-node temperature is `292.28354996106884 K`; omission, duplication,
wrong-node, and wrong-basis alternatives reject.

The strict envelope contains five nonempty physical candidate bodies for
vegetation, hydrology, LSE, soil thermal, and BGC. Each receipt is independently
reconstructed from primitive beginning/candidate operands; the material path
is nonempty. All eleven typed failure records expose no candidate and retain
identical before/after hashes for the five owners plus transaction envelope
under one attempted transaction identity.

This remains Child-1 authority evidence. It does not claim that its independent
Python arbiter is the actual production hydrology owner; that endpoint remains
Child 2's implementation obligation.

## Rights, gates, and protected production boundary

The four committed Copernicus references match their acquisition hashes and
recorded CC-BY-3.0/4.0 rights. Restricted CLM5 bytes remain gitignored under
`references/copyrighted/`. Citations, DOI/version, dates, checksums, rights,
equation locators, and selected/rejected process roles are present.

The heavy history remains truthful: two delegated comparator capacity failures
are preserved as infrastructure failures, followed by parent completion of
only the unfinished commands. Strict workspace Clippy, 2,674/2,674 full
workspace tests, doctest invocation, dependency policy, baseline-relative
documentation links, formatting, and diff hygiene passed.

Ran on the final verifier-B input:

```text
cargo nextest run --test land_surface_energy_balance_authority_contract --profile quick
7 passed / 0 skipped

markdown-doc lint --path \
  docs/work-packages/20260814-snow-free-land-surface-energy-authority-001 \
  --format plain
50 files validated / 0 errors / 0 warnings

cargo fmt --all -- --check
PASS

git diff --check
PASS
```

Static diff inspection finds no file below `crates/`, Cargo manifest or lock,
runner selector, production dispatch, default, state mutation, or production
output-publication change. The kickoff prompt remains correctly active and
unarchived during verification. This child authorizes implementation work
only; it does not implement or activate the runtime, connect production
hydrology, authorize cutover, or claim calibration or empirical validation.

## Conclusion

The exact final input preserves immutable authority and historical evidence,
fully corrects and dispositions every verifier-B finding, and retains complete
hydrology/thermal/material ownership and rollback evidence without widening
the authority claim.

**Result: PASS / GO for `COMPLETE / snow-free land-surface-energy
implementation authority released`. No material finding remains.**
