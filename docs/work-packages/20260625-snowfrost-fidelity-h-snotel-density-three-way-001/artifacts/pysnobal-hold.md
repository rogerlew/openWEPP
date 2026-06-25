# PySnobal Hold

Evidence mode: Ran + Static.

Route: `HOLD-PYSNOBAL-CSS-WY2017-SNOBAL-CORE-FAILURE`.

The H rerun uses SNOTEL STO-derived ground forcing where available and water-year
segmentation to avoid one synthetic snowpack state spanning decades. Four of the
five SNOTEL sites produced passing PySnobal summaries. CSS Lab failed inside the
PySnobal C core for water year 2017:

```text
[pysnobal/c_snobal/libsnobal/sati.c:17] ERROR: Input temperature (tk): -1811.981217 is less than zero
```

Generated summary:

- `target/snowfrost_fidelity_h/pysnobal_snotel_summary.md`
- `target/snowfrost_fidelity_h/pysnobal_snotel_summary.json`

Static checks against local PySnobal source:

- `percent_snow` is documented as a fraction from `0` to `1.0`.
- `T_a`, `T_g`, and `T_pp` are converted from degC to Kelvin by
  `/home/workdir/pysnobal/pysnobal/pysnobal.py` before entering the C core.

CSS WY2017 exported forcing ranges were finite and physically bounded:

- `temp_air_degC`: `-14.97` to `29.97`
- `temp_ground_degC`: `0.0` to `20.56`
- `precip_mass_mm`: `0.0` to `18.23`
- `snow_precip_fraction`: `0.0` to `1.0`
- `downwelling_thermal_Wm-2`: `181.39` to `464.93`

Disposition: H does not claim full PySnobal three-way closure. PySnobal remains
diagnostic flag evidence only; the CSS WY2017 failure needs a focused PySnobal
minimal reproducer or a documented site/year-exclusion policy before PySnobal
can be treated as a complete profile overlay for this corpus.
