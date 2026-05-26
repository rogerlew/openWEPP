# MOFE09 Contract-Test Implementation Evidence

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Added strict `7778` fixture with measured `fc/wp` and no Rosetta theta fields:
  - `tests/fixtures/infile/soil/valid_7778.sol`
- Added contract-derived unit test:
  - `runtime_inputs::tests::soil_runtime_surface_uses_measured_theta_fallback_for_7778`
- Added contract-derived integration test:
  - `parser_to_hillslope_runtime_surface_7778_measured_theta_fallback_closure`

Ran:
- Tests were authored before runtime seam implementation and used in pre-implementation gate to capture expected failure posture.
