# V43 projected-base custody implementation and validation

Status: `IMPLEMENTED_PENDING_CANONICAL`

Evidence mode: `Static + Ran`

## Correction

Static: r113 proved the first charged `1860..1920 s` map failed before
physics. `validate_unpublished_fixed_point_v2` flattened every generic V2
candidate into the ordinary base-physical path. Exact identity/support passed,
then ordinary reconstruction produced `AcceptedReceiptChain` custody while the
authentic V38 sibling retained `NumericalCoordinateProjection` custody. This
was the first unequal `SoilThermalTrialStateV2` field.

Version 43 introduces
`DirectSoilThermalUnpublishedFixedPointPostureV2`. Ordinary base physics still
requires exact authenticated-operand reconstruction. A sealed numerical
coordinate trial instead enters a dedicated same-support private validator
that requires exact resident/prepared beginning, transaction, predecessor,
support, beginning-state and accepted-receipt custody, both numerical
authority/set digests, ordered OFE/layer topology, finite top high and
temperature, canonical zero exact carry, unchanged lower layers/lineage, and
empty physical credits. It creates no continuation result or publication
authority. Base authentication, sequential beginning, accepted composition,
installation, and publication continue to reject the projection.

## Evidence

- Ran: contract-first expected red Nextest run
  `b5f36647-e2bb-48c1-9002-2fafc9233fd9`: contract binding passed; source and
  five required behaviors were absent as expected.
- Ran: V43 contract/source gate Nextest run
  `a6077f5a-4827-4fc5-ac7a-9f47e19988f5`: 2/2 passed.
- Ran: V43 positive/base-byte-lock/poison/no-publication behaviors Nextest runs
  `fb6eae96-1a0e-495b-be7a-a0638a0ad868` and
  `c78fdd97-4095-4365-87f0-a18d21a97a00`: 5/5 passed.
- Ran: retained V38--V43, V39 transaction-custody, numerical-projection, and
  same-support regression run `a2cf7a57-24e2-460e-9e72-082686201c9c`: 38/38
  passed.
- Ran: retained V38--V43 contract/source binding run
  `7c8bcda9-063d-4487-b03b-ff7fc805cd59`: 12/12 passed.
- Ran: `cargo check -p openwepp-hillslope-orchestrator --all-targets` passed.
- Ran: `cargo fmt --all -- --check` and scoped `git diff --check` passed.
- Static: `DFF_R112`, `DFF_R113`, their helper names, `eprintln!`, and `dbg!`
  are absent from the touched production paths.

The unfiltered retained source-contract binary still contains four known
pre-existing stale V32/V33/index assertions; the V43-filtered canonical/source
binding is green and no stale assertion was weakened or edited for V43.
Canonical r114 remains owned by `/root`.
