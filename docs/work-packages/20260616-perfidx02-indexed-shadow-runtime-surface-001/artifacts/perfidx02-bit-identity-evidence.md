# PERFIDX02 Bit Identity And Determinism Evidence

Status: PASS 2026-06-16
Evidence mode: **Ran**

Runtime code was touched, so PERFIDX02 required output identity against the
PERFOPT01 anchor and a same-config determinism check.

## Anchor

Baseline anchor:

```text
/tmp/perfopt01/after
```

Current outputs:

```text
/tmp/perfho01/outputs/ofe1
/tmp/perfho01/outputs/ofe2
/tmp/perfho01/outputs/ofe3
/tmp/perfho01/outputs/ofe4
/tmp/perfho01/outputs/ofe5
/tmp/perfho01/outputs/h2637
/tmp/perfopt01/outputs/h2637_with_ui
```

Comparison method:

- byte identity for `.hbp`, `.loss.json`, and `.plot.parquet`;
- `pyarrow` table equality for `.pass.parquet` and `.wat.parquet`;
- manifests excluded because command paths and timestamps are run-specific.

Result:

```text
CASE ofe1
  byte H15.hbp: PASS
  byte H15.loss.json: PASS
  byte H15.plot.parquet: PASS
  table H15.pass.parquet: PASS rows=2192 cols=17
  table H15.wat.parquet: PASS rows=2192 cols=34
CASE ofe2
  byte H11.hbp: PASS
  byte H11.loss.json: PASS
  byte H11.plot.parquet: PASS
  table H11.pass.parquet: PASS rows=2192 cols=17
  table H11.wat.parquet: PASS rows=4384 cols=34
CASE ofe3
  byte H12.hbp: PASS
  byte H12.loss.json: PASS
  byte H12.plot.parquet: PASS
  table H12.pass.parquet: PASS rows=2192 cols=17
  table H12.wat.parquet: PASS rows=6576 cols=34
CASE ofe4
  byte H25.hbp: PASS
  byte H25.loss.json: PASS
  byte H25.plot.parquet: PASS
  table H25.pass.parquet: PASS rows=2192 cols=17
  table H25.wat.parquet: PASS rows=8768 cols=34
CASE ofe5
  byte H1.hbp: PASS
  byte H1.loss.json: PASS
  byte H1.plot.parquet: PASS
  table H1.pass.parquet: PASS rows=2192 cols=17
  table H1.wat.parquet: PASS rows=10960 cols=34
CASE h2637
  byte H2637.hbp: PASS
  byte H2637.loss.json: PASS
  byte H2637.plot.parquet: PASS
  table H2637.pass.parquet: PASS rows=12419 cols=17
  table H2637.wat.parquet: PASS rows=235961 cols=34
CASE h2637_with_ui
  byte H2637.hbp: PASS
  byte H2637.loss.json: PASS
  byte H2637.plot.parquet: PASS
  table H2637.pass.parquet: PASS rows=12419 cols=17
  table H2637.wat.parquet: PASS rows=235961 cols=34
ANCHOR_MISMATCHES=0
```

After the final H2637 UI shadow run rewrote UI outputs, the UI case was compared
again:

```text
byte H2637.hbp: PASS
byte H2637.loss.json: PASS
byte H2637.plot.parquet: PASS
table H2637.pass.parquet: PASS rows=12419 cols=17
table H2637.wat.parquet: PASS rows=235961 cols=34
POST_SHADOW_UI_MISMATCHES=0
```

## Determinism

OFE5 was rerun twice without audit or shadow hooks, using temporary runfiles
under `/tmp/perfidx02/runfiles` and outputs under `/tmp/perfidx02/determinism`.

Timing:

```text
PERFIDX02_DETERMINISM case=ofe5 run=1 elapsed_s=27.54 user_s=27.50 sys_s=0.03 maxrss_kb=27940
PERFIDX02_DETERMINISM case=ofe5 run=2 elapsed_s=27.59 user_s=27.54 sys_s=0.04 maxrss_kb=27812
```

Comparison:

```text
byte H1.hbp: PASS
byte H1.loss.json: PASS
byte H1.plot.parquet: PASS
table H1.pass.parquet: PASS rows=2192 cols=17
table H1.wat.parquet: PASS rows=10960 cols=34
DETERMINISM_MISMATCHES=0
```
