# HPHYS0244 Data Availability Audit

Static: repository/output schema inspection.
Ran: targeted file inventory, parquet schema audit, HBP string audit.

## Evidence Roots
- Candidate HPHYS0243 root:
  `/tmp/hphys0243_20260602T042747Z/parity`
- HPHYS0244 diagnostic root:
  `/tmp/hphys0244_20260602T045926Z`
- Baseline partitions:
  `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions`
- Baseline full interchange:
  `/wc1/runs/un/unpalatable-rind/wepp/output/interchange`

## Targeted Candidate Files
For each of `H1`, `H7`, and `H39`, HPHYS0243 emitted only:
- `H*.wat.parquet`
- `H*.hbp`
- `H*.loss.json`

No per-hillslope candidate `soil`, `theta`, `layer`, `state`, `perc`, or `Pe`
parquet/debug artifact was emitted under `hillslope_output`.

## Available Surface Matrix
| Surface | Candidate | Baseline | Diagnostic consequence |
| --- | --- | --- | --- |
| `Dp` | WAT column available | WAT column available | Direct numeric comparison possible. |
| `Total-Soil` | WAT column available | `Total-Soil Water` WAT column available | Direct numeric comparison possible after column aliasing. |
| `SoilWaterTotal` | WAT column available | WAT column available | Direct numeric comparison possible. |
| WB18 `Pe` | Not emitted in WAT/loss/HBP as named telemetry | Not emitted in WAT partition | Must be inspected through internal flux lineage or new diagnostics. |
| layer `st`/`theta` | Not emitted as daily named telemetry | Not emitted in WAT partition or `H.soil.parquet` | Current artifacts show symptoms, not direct layer-state trajectories. |

## Baseline Soil Surface
The baseline full `H.soil.parquet` surface exists and has `56979` rows, but its
columns are OFE/day aggregate soil descriptors:
`Poros`, `Keff`, `Suct`, `FC`, `WP`, `Rough`, `Ki`, `Kr`, `Tauc`,
`Saturation`, `TSW`, and `TSMF`.

It does not expose per-layer legacy `st(i)` or a daily layer `theta` vector.
Representative baseline soil rows were written to:
`/tmp/hphys0244_20260602T045926Z/baseline_soil_first30.tsv`.

## HBP Surface
Static source inspection shows current hillslope HBP emission is not useful for
daily layer-state diagnostics:
- `crates/openwepp-runner/src/hillslope/mod.rs:3732` fixes `max_layers = 1`
  for the emitted fixture payload.
- `crates/openwepp-runner/src/hillslope/mod.rs:3914` through
  `crates/openwepp-runner/src/hillslope/mod.rs:3922` writes zero-valued generic
  state entries.
- HBP string audit for `H1`, `H7`, and `H39` found only generic `state_N`
  labels, not `wb18_perc_theta`, `st`, `Pe`, or `pei`.

Audit files:
- `/tmp/hphys0244_20260602T045926Z/surface_file_inventory.tsv`
- `/tmp/hphys0244_20260602T045926Z/schema_audit.tsv`
- `/tmp/hphys0244_20260602T045926Z/availability.json`
- `/tmp/hphys0244_20260602T045926Z/hbp_string_audit.txt`

## Finding
HPHYS0244 can directly assess `Dp`, `Total-Soil`, and `SoilWaterTotal` from
existing WAT outputs. It cannot directly assess layer `st`/`theta` or WB18 `Pe`
from emitted artifacts. Those must be traced by a diagnostics-only internal
runtime surface or by an explicit comparator/instrumentation package before
claiming layer-level root-cause closure.
