# C3 Woody V5 Diagnostic Fixture Manifest

These two compact JSON files are local, synthetic `ASSUMED_FOR_EXECUTION`
fixtures for strict V5 configuration/state parsing, digest binding, migration,
and default-off implementation-contract tests:

- `c3_woody_v5_diagnostic_configuration.json`
- `c3_woody_v5_diagnostic_state.json`

They are derived from the earlier one-stratum/one-tile diagnostic fixture by
applying the identity-only `SC-VEGETATION-001` v9 /
`OPENWEPP_C3_WOODY_V5` transition to the exact V4 shared-state schema: the V5
model identity is rebound, all schema payload fields are unchanged, and distinct
configuration/state digests are recomputed by the production canonical encoders.
No observed data,
calibration result, parameter recommendation, empirical bound, or independent
constitutive oracle is represented.

Units and area/time bases are the imported canonical V4 schema units embedded in the
field names and definition. The fixture is consumed by vegetation crate tests
and `c3_vegetation_implementation_contract`; it is not runtime-selectable.

Verify exact installed bytes with:

```sh
sha256sum -c tests/fixtures/c3_woody_v5_diagnostic.sha256
```
