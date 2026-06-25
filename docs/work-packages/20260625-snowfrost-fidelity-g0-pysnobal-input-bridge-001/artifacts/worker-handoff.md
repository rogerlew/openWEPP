# Worker Handoff

Status: executed-hold

Evidence mode: Static + Ran.

Next authorized route: scaffold a narrow follow-up package to resolve
`HOLD-PYSNOBAL-SANITY-FAILURE` before treating PySnobal as a usable
SNOWFROST-FIDELITY-G comparator.

First actionable task: build a minimal reproducer for Site 4
`tg_neg0p5c_zg0p10m` by bisecting `target/snowfrost_fidelity_g0/site4/tg_neg0p5c_zg0p10m/forcing.csv`
into smaller time windows, preserving the exact `config.yaml`, until the
PySnobal C-layer `sati.c:17` failure is isolated to the shortest triggering
window. Record whether the failure is sensitive to PySnobal dependency
versions, constant `Tg`, or a specific forcing transition.

Required next decisions:

- Decide whether the `Tg=-0.5 degC` lane is invalid for G0, needs a PySnobal
  compatibility patch, or exposes a forcing proxy that must be replaced before
  snow-depth comparator work.
- Add a harness option for lane/window selection so PySnobal investigations do
  not require rerunning all five full sites every time.
- Decide whether G1 should populate `openwepp_snow.csv` from current WAT
  publication before PySnobal-vs-openWEPP comparisons are considered
  metric-bearing; G0 currently records `NO_ROWS` for those comparisons.
- Preserve the G0 date-continuity and precipitation-reconstruction guards when
  adding any windowed or parallel PySnobal harness mode.

Do not use the 14 passing lanes to authorize production snow-depth physics
changes. They are useful diagnostic evidence only after the held lane is
dispositioned.
