# Disposition

Status: executed-hold

Evidence mode: Static + Ran.

Final disposition: `HOLD-PYSNOBAL-SANITY-FAILURE`.

Static: the Rust exporter and Python harness are diagnostic-only and do not
change production snow, frost, hydrology, erosion, runtime activation, or
observation tolerances.

Ran: PySnobal is available through `/tmp/pysnobal-g0-venv/bin/python`, and the
Rust exporter emits required PySnobal forcing/config/lineage/audit artifacts
for all five pilot sites.

Ran: all exporter, schema, anti-alias, format, clippy, workspace test, deny,
whitespace, and source-scan gates are green.

Ran: the current all-site PySnobal sanity gate does not pass. The wrapper
printed `PYSNOBAL_HARNESS_EXIT=1`. Site 4 GGD498 Morris, lane
`tg_neg0p5c_zg0p10m`, fails in PySnobal C code with:

```text
[pysnobal/c_snobal/libsnobal/sati.c:17] ERROR: Input temperature (tk): -153.450833 is less than zero
```

Ran: the failing lane's exported forcing is finite at the file boundary, and
the adjacent Site 4 `Tg=0.0 degC` and `Tg=-2.5 degC` lanes pass. G0 therefore
does not classify a PySnobal snow-depth comparator as usable across the full
pilot set. The successor must isolate or dispose the Site 4 `Tg=-0.5 degC`
lane before using PySnobal as SNOWFROST-FIDELITY-G comparator evidence.
