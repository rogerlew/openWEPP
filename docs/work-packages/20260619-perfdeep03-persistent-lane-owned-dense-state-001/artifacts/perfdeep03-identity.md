# PERFDEEP03 Identity

Evidence class: Ran.

## Verdict

`PASS` for the package identity requirements:

- HBP byte identity passed.
- WAT parquet byte identity passed.
- PASS parquet Arrow row equivalence passed.
- PERFDEEP02 roundtrip diagnostic stayed zero-mismatch.

Loss JSON and plot parquet checksums differ only by run-name metadata/string
content because the PERFDEEP03 runfile uses `run_name = "perfdeep03-h2637"`.

## HBP and WAT Byte Identity

Ran:

```text
sha256sum \
  /tmp/perfdeep01/current/h2637_same/H2637.hbp \
  /tmp/perfdeep03/current/h2637_same/H2637.hbp \
  /tmp/perfdeep01/current/h2637_same/H2637.wat.parquet \
  /tmp/perfdeep03/current/h2637_same/H2637.wat.parquet
```

Result:

```text
44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8  /tmp/perfdeep01/current/h2637_same/H2637.hbp
44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8  /tmp/perfdeep03/current/h2637_same/H2637.hbp
c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474  /tmp/perfdeep01/current/h2637_same/H2637.wat.parquet
c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474  /tmp/perfdeep03/current/h2637_same/H2637.wat.parquet
```

## PASS Arrow Equivalence

Ran DuckDB row-difference checks over the baseline and PERFDEEP03 PASS parquet
files.

Result:

```text
left_minus_right 0
right_minus_left 0
left_rows 12419
right_rows 12419
```

The PASS parquet byte checksum differed, but row/schema equivalence passed.

## Loss and Plot Metadata Difference

Loss JSON diff:

```diff
-  "run_name": "perfmig01-final-h2637",
+  "run_name": "perfdeep03-h2637",
```

Plot string metadata:

```text
openwepp_optional_output_v1
run_name=perfmig01-final-h2637
file=H2637.plot.parquet
first_year=1987
first_day=1
last_year=2020
last_day=366
climate_day_count=12419
executed_day_count=12419
precipitation_mm=45200.000
openwepp_optional_output_v1
run_name=perfdeep03-h2637
file=H2637.plot.parquet
first_year=1987
first_day=1
last_year=2020
last_day=366
climate_day_count=12419
executed_day_count=12419
precipitation_mm=45200.000
```

## Roundtrip Diagnostic

Ran:

```text
wc -l /tmp/perfdeep03/frame_roundtrip/h2637_final.jsonl
jq -r '.stage' /tmp/perfdeep03/frame_roundtrip/h2637_final.jsonl | sort | uniq -c
rg -n '"state_mismatch_count":[1-9]|"flux_mismatch_count":[1-9]' /tmp/perfdeep03/frame_roundtrip/h2637_final.jsonl
```

Result:

```text
235961 /tmp/perfdeep03/frame_roundtrip/h2637_final.jsonl
235961 mofe_pre_scheduler
```

The mismatch search returned no matches.

## Default-Disabled Identity

Default-disabled HBP and WAT hashes matched the PERFDEEP01 reference. PASS
parquet Arrow row equivalence also passed:

```text
default_left_minus_right 0
default_right_minus_left 0
```
