# Surface-liquid complete-owner projection V3

Static: implemented the immutable
`OPENWEPP_SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V3` serialization/join
surface without changing V1/V2 owner bytes, `SurfaceLiquidOwnerRestartV2`,
runtime coordination, persistence, physics, publication, tolerances, or the
60-second floor.

## Exact owned files

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_projection.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner/v3_projection_tests.rs`
- module/export-only additions in `surface_liquid_owner.rs`, `direct_runtime.rs`, and `lib.rs`
- this evidence artifact

The projection binds schema/model/configuration/parent identities, parent and
accepted-child half-open supports, transaction/predecessor identity, beginning
and post-phase owner digests, the exact ending `SurfaceLiquidOwnerEnvelopeV2`
bytes, exact in-progress `DirectWb14ParentWorkingStateV2` bytes, interleaved
ordered litter vapor/phase receipt bytes, ordered current-ingress receipt
bytes, the exact parent-owned `SoilThermalOwnerEnvelopeV2` bytes and restart
identity, predecessor/final receipt-chain heads, and its own digest. The final
receipt-chain head is independently reconstructed with explicit length-framed
bytes. Nested parsers and validators replay every admitted frame.

Static: the WB14 join independently compares the phase-adjusted beginning and
candidate litter-ice bits and rejects any ice donation/substitution. The soil
join requires V15 tag/schema/definition, exact state and carry seal,
transaction/predecessor/support identity, receipt-chain identity, and restart
identity. Cross-version surface owners are rejected.

## Ran

Focused projection vectors:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(~v3_projection)' --no-fail-fast
```

Result: `PASS`, 4/4; Nextest run
`66441da4-aeca-4a20-8ace-48b5232b3fa9`. The vectors cover canonical
roundtrip, exact V2 byte lock, omission, reorder, replay, mixed transaction,
cross-version owner, projection digest, soil-carry substitution, and WB14 ice
donation poisons. Poison tests reseal outer identities where needed so nested
custody validation, rather than mutation alone, remains exercised.

V15 authority scan:

```text
nix develop -c cargo nextest run \
  --test surface_liquid_hydrology_custody_authority_contract \
  -E 'test(~version_15_)' --no-fail-fast
```

Result: `PASS`, 2/2; Nextest run
`c48c6f99-1cb3-46e3-a962-f82ba24474cd`.

Compile check:

```text
nix develop -c cargo check -p openwepp-hillslope-orchestrator --tests
```

Result: `PASS`, no diagnostics, 3.82 s.

Affected warnings-denied Clippy:

```text
nix develop -c cargo clippy -p openwepp-hillslope-orchestrator \
  --lib --tests -- -D warnings
```

Result: shared-crate `FAIL` on concurrent/unowned diagnostics. The complete
log is `/tmp/surface_owner_v3_projection_clippy_final.log`; an exact path scan
for `v3_projection`, `surface_liquid_owner.rs`, `direct_runtime.rs`, and
`lib.rs` returned no diagnostics. No lint suppression was introduced for the
projection.

Ran: explicit `cargo fmt` over the five owned Rust paths: `PASS`.

Ran: `git diff --check` over the five owned Rust paths: `PASS`.

Line counts: new projection 643 lines; new focused tests 499 lines; export
files 987 and 286 lines. The pre-existing `surface_liquid_owner.rs` is 2947
lines and remains an inherited `WARN`, below the 3000-line closure threshold;
the assignment added only nine module/export lines there.

Terminal source SHA-256 values:

```text
v3_projection.rs       e9ea0312d4df7d8655c4cdd04d3cec9eb2383c9585812245b1d78763062ab95a
v3_projection_tests.rs be90201243e294d75eb7b0606a20704dbcb173fa445e5b24b75e3968d224dd66
```

## Exact API handoff

Public exports:

- `SURFACE_LIQUID_COMPLETE_OWNER_PROJECTION_V3_SCHEMA`
- `SurfaceLiquidCompleteOwnerProjectionIdentityV3`
- `SurfaceLiquidCompleteOwnerProjectionV3::{new,canonical_bytes,from_canonical_bytes}`

The later runtime coordinator must pass the already-validated accepted child:
ending V2 surface envelope, open V2 WB14 parent bytes, exact accepted litter
receipts, exact current-ingress receipts, and parent-owned soil V2 envelope
plus restart identity. This slice does not publish or persist the projection.

## External gate note

Ran: a mixed V14/V15 authority run advanced through the new projection symbol
but the V14 source scan still failed on the pre-existing missing literal
`migrate_v1_to_v2`. That V2 migration-symbol defect is outside this
projection-only write set; V15 passed independently as recorded above. This
artifact makes no package-closure or real-consumer claim.
