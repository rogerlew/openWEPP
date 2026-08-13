# C3 Woody V4 Diagnostic Fixture Manifest

These two compact JSON files are local, synthetic `ASSUMED_FOR_EXECUTION`
fixtures for strict V4 configuration/state parsing, digest binding, migration,
and default-off implementation-contract tests:

- `c3_woody_v4_diagnostic_configuration.json`
- `c3_woody_v4_diagnostic_state.json`

They are derived from the earlier one-stratum/one-tile diagnostic fixture by
applying the exact `SC-VEGETATION-001` v8 / `OPENWEPP_C3_WOODY_V4` shared-state
schema: V4 model identity is rebound, the two unconsumed V3 offset fields are
absent, displayed leaf C owns the exact area caches, and configuration/state
digests are recomputed by the production canonical encoders. No observed data,
calibration result, parameter recommendation, empirical bound, or independent
constitutive oracle is represented.

Units and area/time bases are the canonical V4 schema units embedded in the
field names and definition. The fixture is consumed by vegetation crate tests
and `c3_vegetation_implementation_contract`; it is not runtime-selectable.

Verify exact installed bytes with:

```sh
sha256sum -c tests/fixtures/c3_woody_v4_diagnostic.sha256
```

