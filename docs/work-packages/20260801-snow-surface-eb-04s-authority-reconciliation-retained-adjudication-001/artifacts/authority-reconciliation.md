# Frozen Authority Reconciliation

Evidence mode: `Static`.

Status: `FROZEN_AUTHORITY_ONLY`.

The authority phase read exactly the four whitelisted pre-result inputs. It did
not read EB-04R outputs, residuals, scores, observations, attempt records, or
terminal audit evidence.

## Dimensional Decision

```text
area_mass_tolerance_kg_m2 = water_equivalent_tolerance_m * liquid_water_density_kg_m3
1e-9 m * 1000 kg m^-3 = 1e-06 kg m^-2
```

Therefore the canonical `1e-9 m` snow-mass closure tolerance is
`1e-6 kg m^-2` when the same residual is expressed as area mass. The
`1e-9 kg m^-2` represented-layer lifecycle boundary is a different predicate
and must not be substituted for an aggregate or transfer-identity residual.

Decision: `CROSS_UNIT_PROTOCOL_TRANSCRIPTION_ERROR`.

The frozen machine-readable receipt is `authority-freeze.json`. Its SHA-256 is
`20c227029ccc876209cd81cdc830c9c68811307ee055d300836a769aa388798f`.
