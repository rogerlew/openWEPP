# Worker Handoff

Status: EXECUTED. Evidence mode: Static.

First actionable follow-on:

- Scaffold and execute the D16 active plain-vs-hybrid suite package at current
  mesh.

Inputs now available:

- WEPPpy opt-in native producer flag:
  `disturbed.openwepp_native_managements_enabled`.
- Disturbed route coefficient matrix:
  `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py`.
- Generated native smoke artifact:
  `artifacts/generated-native-smoke/p1.man`.
- openWEPP fixture:
  `tests/fixtures/disturbed_native_route_coefficients/p1.man`.

Follow-on gates:

- Generate selected D16 cohort native `ow-lanuse-1` managements from actual
  Disturbed roots.
- Run active plain and explicit hybrid preflights.
- Run plain-vs-hybrid tolerance and timing adjudication.
- Keep D16 promotion held until active cohort evidence closes.
