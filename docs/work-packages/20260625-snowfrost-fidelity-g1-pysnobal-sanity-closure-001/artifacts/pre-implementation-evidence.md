# Pre-Implementation Evidence

Evidence mode: Static + Ran.

Static: G0 held at `HOLD-PYSNOBAL-SANITY-FAILURE` because Site 4
`site4_ggd498_morris_mn`, lane `tg_neg0p5c_zg0p10m`, aborted inside PySnobal C
code:

```text
[pysnobal/c_snobal/libsnobal/sati.c:17] ERROR: Input temperature (tk): -153.450833 is less than zero
```

Static: the adjacent Site 4 lanes `tg_0p0c_zg0p10m` and
`tg_neg2p5c_zg0p10m` passed under the same exported meteorological forcing.
This makes the G0 blocker lane-specific rather than a blanket Rust forcing
schema failure.

Static: G0 wrote `openwepp_snow.csv` as a header-only placeholder:

```text
date,Snow-Water_mm,Snow-Depth_mm,source
```

Static: current openWEPP WAT publication contains `Snow-Water` as SWE in
millimeters and nullable `Snow-Depth` as the runtime snow-depth diagnostic in
millimeters. These are the current openWEPP surfaces required for
PySnobal-vs-openWEPP snow-depth comparison.

Ran: local PySnobal source is present at `/workdir/pysnobal`, and the G0 Python
environment is present at `/tmp/pysnobal-g0-venv/bin/python`.

Ran: Site 4 held lane forcing has 394,488 hourly rows from
`1980-01-01 00:00:00` through `2024-12-31 23:00:00`; 2,909 rows contain
positive snow precipitation. Full-lane iteration therefore needs site/lane and
window controls.
