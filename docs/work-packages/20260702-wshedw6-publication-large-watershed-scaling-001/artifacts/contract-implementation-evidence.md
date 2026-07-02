# Contract Implementation Evidence

Status: `not-applicable`

Evidence mode: `Static:`

W6 is schema-preserving and physics-preserving.

No canonical `SC-*` amendments were required because:

- watershed routing, impoundment, sediment, erosion, runoff partition, and
  water-balance physics were not changed;
- output schemas were not changed;
- unit labels and Arrow/Parquet schema constructors were not changed;
- public watershed output publication now consumes
  `WatershedPublicationFrame` directly;
- unavailable typed publication operands are emitted as null rather than filled
  with compatibility-stage zeroes;
- `chanwb` channel-balance volume fields are not backfilled from impoundment
  outflow, routed runoff aliases, or the `cbase` routing global;
- W6 area normalization uses committed source hillslope slope geometry when
  available instead of a fake unit area;
- conservation-sensitive touched fields have operand lineage in
  `artifacts/publication-operand-lineage.md` and reconstruction evidence in
  `artifacts/conservation-reconstruction.md`.
