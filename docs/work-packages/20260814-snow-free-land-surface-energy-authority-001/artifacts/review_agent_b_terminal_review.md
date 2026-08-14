# Terminal Hydrology And Ownership Review

Evidence class: `Static + targeted Ran`, fresh independent exact-worktree
review.

Verdict: **PASS / GO**. No material hydrology, custody, owner-envelope,
area-basis, transaction-lineage, or rollback finding remains on the reviewed
bytes. This review does not claim Child-2 real-production-hydrology evidence;
the Python arbiter remains Child-1 authority evidence only.

## Exact Bytes Reviewed

The repository remained on local `main` at base commit
`0db1960129ad4f8fc4e292b20574dfe7229d5fe1` with the uncommitted Child-1
worktree. Exact reviewed hashes were:

- `SC-LANDSURFACEENERGY-001.md`:
  `67b51fde024e85668d1bb605bbb54fd58ea6b7a0e798b68db1293ebbb93a0a62`;
- `SC-VEGETATION-001.md`:
  `1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a`;
- `SC-VEGETATIONTRANSACTION-001.md`:
  `44b77e5d9854ce9c4b64214f2a669e1b09e2fdc5bdb6d1391ef25a909f186e73`;
- `SC-WATBAL-001.md`:
  `c30b7c243a36f7fc2aec316c3ba590c8f7629759d36bf1f91b60c0cf0c419188`;
- LSE definition:
  `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`;
- top-level calculator:
  `1156fa88a6d7e4dd98f6dd70fe5b891f69e0b6825694179ac4d687a38907c859`;
- frozen vectors:
  `7b6a303ae434ca6ad59c7082ebf486300214427d6abe20c36bfaa9b8cbdab91c`;
- coupled-transaction schema:
  `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f`;
- water-protocol schema:
  `2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07`;
- focused Rust authority test:
  `c4e50345f696321cad79d490b4cf97645735a4f98e064a563a98bfb6e774a041`.

Per the assignment, I did not regenerate the oracle. I imported the
checksum-bound calculator without invoking generation and exercised its
independent validators against the frozen fixture.

## Targeted Execution

Ran:

- all six strict instances through the calculator's registered Draft 2020-12
  schemas: **PASS**;
- the physical post-ingress five-owner candidate through
  `validate_owner_candidates` and independently reproduced owner receipts:
  **PASS**;
- post-ingress, complete-water, shared-competition, and positive-condensation
  D/A/F validation: **PASS**;
- positive-condensation candidate reconstruction and receipt validation:
  **PASS**;
- routed multi-OFE mass, enthalpy, state/transaction lineage, area conversion,
  and forcing-schema validation: **PASS**;
- all eleven failure records for null candidate, one attempted transaction,
  exact five-owner-plus-envelope rollback, and diagnostics-schema status:
  **PASS**;
- `cargo nextest run --test land_surface_energy_balance_authority_contract
  --profile quick`: **7 passed, 0 skipped**.

## Ownership And Transaction Assessment

### Five physical owner bodies and independent receipts

The strict coupled transaction contains the same five nonempty physical
candidate bodies as `post_ingress_owner_candidates`: vegetation, hydrology,
land-surface energy, soil thermal, and biogeochemistry. The strict receipts are
the independently issued receipts for those same bodies. Each receipt hashes
the candidate body embedded in the transaction and the complete corresponding
beginning owner record. Candidate validation reconstructs endings from
primitive vegetation components, water records, advection, ground-heat, soil-
thermal, and material operands; copied ending-state/hash joins are explicitly
prohibited.

The material path is nonempty. One vegetation proposal is independently
converted to a BGC receipt with exact transaction, proposal, receiver, carbon,
nitrogen, and dry-material identity before the BGC ending body is accepted.
Thus `OWN5-CRITICAL-001` is corrected without relying on two equal empty hashes
or a producer-supplied validation boolean.

### Water D/A/F and condensation

The physical post-ingress protocol retains 9/9/9 request, authorization, and
finalized-use identities. The decisive shared-source vector retains 19/19/19
identities, one immutable-snapshot arbitration, `F <= A <= D` for every key,
and six independently reconstructed source-store ledgers. Finalized use alone
is debited; unused authorization remains.

The positive condensation transaction passes the normative water schema with
one typed positive credit. Its hydrology ending store and LSE signed enthalpy
credit are reconstructed from the same mass, source, OFE, tile, surface,
transaction, and specific-enthalpy operands. Authorization is not substituted
for final use and condensation is not represented as a negative withdrawal.

### Routed runon custody and area conversion

The accepted upstream runoff record constructs the downstream runon parcel
with exact accepted source transaction/state lineage. The nondegenerate route
converts `0.6 kg m^-2` over `120 m^2` to `0.36 kg m^-2` over `200 m^2`; both
reconstruct `72 kg`. Source and destination reconstruct the same
`5952940.008379017 J` extensive energy and retain the same route, interval,
specific enthalpy, and accepted source state. The downstream forcing instance
passes its strict schema.

### `A6/OWN6` soil-thermal receipt consumption

The soil-thermal ending node now consumes both accepted energy crossings. From
the immutable beginning node-1 temperature, the validator independently uses

```text
T_1,end = T_1,begin
        + (Q_ground_heat,stand + Q_infiltration,stand)
          / (f_tile * C_1,tile)
```

exactly once. The reviewed operands reconstruct
`292.28354996106884 K`, byte-for-byte equal to the physical soil-thermal
candidate. The typed infiltration receipt remains present separately with the
same node and stand-ground basis; receipt presence is no longer mistaken for
owner-state consumption.

All four required poisons reject with null candidates:

- omitted infiltration enthalpy;
- duplicated infiltration enthalpy;
- wrong receiving soil node;
- wrong area basis.

The positive exact temperature, independent candidate comparison, generic
soil-candidate mutation poison, and four specific receipt poisons together
close `A6-CRITICAL-001` and `OWN6-CRITICAL-001`.

### Rollback and production boundary

All eleven natural/domain failures use attempted transaction `20260814001`
for vegetation, hydrology, LSE, soil thermal, BGC, and envelope, expose no
candidate, and retain identical before/after hashes for all six entries.

This evidence admits the owner protocol and authority fixtures only. The
fixture's `hydrology-real-owner` identity does not make the Python arbiter the
actual production water owner. Child 2 still must extract and prove the real
production hydrology state/candidate path and production byte invariance.

## Conclusion

The final soil-node correction consumes accepted infiltration enthalpy in the
mutable receiving state while preserving exact source, receiver, transaction,
area, and equal/opposite ground-heat identities. The strict envelope contains
the five actual candidate bodies and independently reconstructed receipts;
water, condensation, routed advection, material custody, and rollback remain
closed on the exact reviewed bytes.

**Result: PASS / GO for Child-1 hydrology and ownership authority release. No
material finding.**
