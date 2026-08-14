# Oracle Fixture Manifest

The authority oracle is independent Python and never calls Rust. The top-level
`reference_calculator.py` composes the digest-verified inherited V3 canopy
oracle with the exact V8 canopy-ground joint core, source-resolved
arbitrary-rank longwave, immutable-snapshot source-keyed water arbitration,
final fixed-cap rebuild, hydrology ingress/partition, liquid-advection
receipts, ground-heat receipts, and local/OFE closure reconstruction. The
mandatory matrix does not call the rejected three-unknown surrogate.

The committed output is
`openwepp_snow_free_lse_v1_vectors.json`. Normal Rust authority tests consume
the committed JSON and do not require Python.

Strict machine schemas are frozen separately so persistent LSE state cannot
silently absorb adjacent-owner mass or temperature:

- `lse_v1_configuration_schema.json`;
- `lse_v1_coupled_transaction_schema.json`;
- `lse_v1_state_schema.json`;
- `lse_v1_forcing_schema.json`;
- `lse_v1_water_protocol_schema.json`;
- `lse_v1_diagnostics_schema.json`.

The persistent state schema contains one physical thermal state,
`surface_enthalpy_j_m2_tile`, and one explicitly numerical temperature warm
start. Hydrology water masses and soil-thermal temperatures are immutable
forcing snapshots with owner-state digests.

Digest binding is deliberately acyclic. The immutable LSE definition binds the
six schema hashes. Each schema structurally validates its SHA-256 identity
field, but does not const-bind the LSE definition digest back into itself. The
independent contract/fixture validator requires the exact LSE definition digest
and exact cross-owner configuration/state equality. This preserves strict
identity without a definition-to-schema-to-definition hash cycle.

Terminal frozen and independently confirmed digests:

- LSE canonical section:
  `9302913ba951d3a8a4caa934c5f981cc61b5bc184bdf9284bdcf377ffd667fe8`;
- LSE model definition:
  `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`;
- V8 model definition:
  `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`;
- top-level generator:
  `9278be79b1a74d4d609ab5857d00071b1e5717e036cc7323cbfcbf970795666c`;
- joint canopy-ground core:
  `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5`;
- committed vectors:
  `9f171b0fd0e9a9a2e40d6ea8773d120b961c343e2aad6ad951ae705c8d683f3b`;
- configuration schema:
  `6499b98cc1e25f1379bc0ad6052a7536e20c4bfbb9335f9ba5c8de191ae2f009`;
- coupled-transaction schema:
  `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f`;
- diagnostics schema:
  `41fb7909d073b4fdf4e59c9fa7da26b9a965ad916688b7867a56525d1bf1460c`;
- forcing schema:
  `f1fb785e9e582ae9e20eac4b5f44fa2b5f0651f8535d0972520dbfff3d926b55`;
- state schema:
  `91243e4087fa2c4775cb3629fe14c64379def4977d3c54a72348ac56d5fa4ee8`;
- water-protocol schema:
  `2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07`.

Historical: two controlled post-third-review regenerations produced
byte-identical fixture digest `68ebdb09...`; the subsequent release reviews
failed and invalidated that candidate. The historical release regeneration was
byte-identical at `7b6a303a...`, and both terminal reviews returned `PASS / GO`.
The later canonical-JSON evidence correction binds the strict positive state
after its final forest-tile projection; corrected vectors are `9f171b0f...`.
The exact core
executes 22 mandatory families. Its
natural singular-pivot, iteration-limit, and backtracking-limit branches return
no candidate and preserve the beginning-state rollback hash. The top-level
transaction instantiates all six strict schemas, constructs five owner
candidates, executes one shared-layer root/ground arbitration and one routed
multi-OFE case with distinct 120 and 200 square-metre OFE areas, and
reconstructs rather than asserts the single-authorization and
rebuild-from-beginning invariants.
