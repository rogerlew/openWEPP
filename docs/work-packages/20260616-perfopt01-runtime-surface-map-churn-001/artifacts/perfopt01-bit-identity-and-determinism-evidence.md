# PERFOPT01 Bit Identity And Determinism Evidence

Status: PASS 2026-06-16
Evidence mode: **Ran** (fixture reruns and structured comparisons) + **Static** (comparison policy)

## Artifact Roots

- Baseline outputs: `/tmp/perfopt01/baseline/{ofe1,ofe2,ofe3,ofe4,ofe5,h2637,h2637_with_ui}`
- Optimized outputs: `/tmp/perfopt01/after/{ofe1,ofe2,ofe3,ofe4,ofe5,h2637,h2637_with_ui}`
- Determinism outputs: `/tmp/perfopt01/determinism/ofe5_run{1,2}`

## Baseline Versus Optimized Identity

Ran: Python comparison using byte identity for HBP, JSON, and ASCII optional-output marker files, plus `pyarrow` table equality for readable Parquet files.

Result:

```text
CASE ofe1
  BYTE H15.hbp: True
  BYTE H15.loss.json: True
  PARQUET H15.pass.parquet: True rows=2192 cols=17
  BYTE H15.plot.parquet: True
  PARQUET H15.wat.parquet: True rows=2192 cols=34
CASE ofe2
  BYTE H11.hbp: True
  BYTE H11.loss.json: True
  PARQUET H11.pass.parquet: True rows=2192 cols=17
  BYTE H11.plot.parquet: True
  PARQUET H11.wat.parquet: True rows=4384 cols=34
CASE ofe3
  BYTE H12.hbp: True
  BYTE H12.loss.json: True
  PARQUET H12.pass.parquet: True rows=2192 cols=17
  BYTE H12.plot.parquet: True
  PARQUET H12.wat.parquet: True rows=6576 cols=34
CASE ofe4
  BYTE H25.hbp: True
  BYTE H25.loss.json: True
  PARQUET H25.pass.parquet: True rows=2192 cols=17
  BYTE H25.plot.parquet: True
  PARQUET H25.wat.parquet: True rows=8768 cols=34
CASE ofe5
  BYTE H1.hbp: True
  BYTE H1.loss.json: True
  PARQUET H1.pass.parquet: True rows=2192 cols=17
  BYTE H1.plot.parquet: True
  PARQUET H1.wat.parquet: True rows=10960 cols=34
CASE h2637
  BYTE H2637.hbp: True
  BYTE H2637.loss.json: True
  PARQUET H2637.pass.parquet: True rows=12419 cols=17
  BYTE H2637.plot.parquet: True
  PARQUET H2637.wat.parquet: True rows=235961 cols=34
CASE h2637_with_ui
  BYTE H2637.hbp: True
  BYTE H2637.loss.json: True
  PARQUET H2637.pass.parquet: True rows=12419 cols=17
  BYTE H2637.plot.parquet: True
  PARQUET H2637.wat.parquet: True rows=235961 cols=34
PERFOPT01_IDENTITY_OK
```

Disposition: `anchor_mismatches = 0`.

## Determinism

Ran: OFE5 optimized fixture twice with the same inputs and runfile.

```text
PERFOPT01_DETERMINISM case=ofe5 run=1 elapsed_s=27.79 user_s=27.68 sys_s=0.09 maxrss_kb=24960
PERFOPT01_DETERMINISM case=ofe5 run=2 elapsed_s=27.38 user_s=27.35 sys_s=0.03 maxrss_kb=25728
BYTE H1.hbp: True
BYTE H1.loss.json: True
PARQUET H1.pass.parquet: True
BYTE H1.plot.parquet: True
PARQUET H1.wat.parquet: True
PERFOPT01_DETERMINISM_OK
```

Static: PERFOPT01 did not reorder floating-point reductions or per-OFE transfer sequencing. Output identity across baseline/after and repeated optimized runs supports within-config reproducibility.

## Conservation And Fail-Closed Behavior

Ran:

- H2637 without UI exited 0 after optimization.
- H2637 with UI exited 0 after optimization.
- `cargo test -p openwepp-kernel-contract` passed, including `rejects_non_finite_payload_with_typed_status`.
- `cargo test --workspace` passed, including `kernel_writeback_contract` and scheduler/writeback integration tests.

Static: Lazy writeback validation only bypasses diagnostic string construction for fields that already satisfy the same finite/domain predicates. On potential failure, the code still constructs the same subject string and calls the same closure check helpers with the same invariant/message IDs.

