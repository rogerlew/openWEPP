# Active Consumer Proof

Status: EXECUTED-PROJECTION / ACTIVE-COHORT-HOLD. Evidence mode: Static + Ran.

Evidence chain completed:

- Disturbed extended lookup row:
  `forest moderate sev fire`, `loam`, route coefficients
  `490.0, 0.4, 0.016, 0.05, 0.2`.
- Generated native file:
  `artifacts/generated-native-smoke/p1.man`.
- openWEPP fixture:
  `tests/fixtures/disturbed_native_route_coefficients/p1.man`.
- WEPPpy producer path:
  `wepp_prep_service.py` converts the real `pN.man` write path when
  `disturbed.openwepp_native_managements_enabled` is true.
- openWEPP parse/projection:
  `disturbed_native_route_coefficients_project_to_ofe_symbols` passes and
  verifies all five route symbols plus slot crop symbols.

Negative proof:

- The fixture values are class-specific Disturbed values, not the H2637 timing
  recipe `500.0 0.0 0.0 0.0 0.0`.
- WEPPpy validation rejects unsupported/missing route rows instead of falling
  back.
- `managements.py` rejects `routing_coefficients` markers under legacy datvers.

Not completed:

- Full active Lane-D executable cohort proof was not run. The existing active
  path consumes PL schedule symbols after runtime projection, and the
  Disturbed-generated fixture now reaches those symbols, but the D16 selected
  cohort active plain-vs-hybrid run remains a follow-on gate.
