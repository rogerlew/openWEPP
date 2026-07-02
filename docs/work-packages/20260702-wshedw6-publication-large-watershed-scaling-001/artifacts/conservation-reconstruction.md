# Conservation Reconstruction

Status: `passed`

Evidence mode: `Ran:` independent parquet reconstruction plus focused writer
tests.

## Produced-Output Reconstruction

Command:

```sh
.venv/bin/python - <<'PY'
from pathlib import Path
import pyarrow.parquet as pq
...
PY
```

Fixtures:

- `/tmp/wshedw6_onshore_scaling_rerun/jobs1-full/out/interchange`
- `/tmp/wshedw6_carnivorous_scaling_rerun/jobs1-full/out/interchange`

Checks reconstructed independently from produced parquet outputs:

| Fixture | Check | Reconstructed | Output | Result |
| --- | --- | ---: | ---: | --- |
| `onshore-xenophobia` | `Q = Runoff(mm) * Area(m2) / 1000` | `0.9026067600000023` | `0.9026067600000023` | PASS |
| `onshore-xenophobia` | `runvol == chanwb Inflow` | `0.9026067600000022` | `0.9026067600000022` | PASS |
| `onshore-xenophobia` | `Balance = Inflow - Outflow - Loss` | `0.9026067600000022` | `0.9026067600000022` | PASS |
| `carnivorous-adobo` | `Q = Runoff(mm) * Area(m2) / 1000` | `0.003179880000000001` | `0.003179880000000001` | PASS |
| `carnivorous-adobo` | `runvol == chanwb Inflow` | `0.0031798800000000004` | `0.0031798800000000004` | PASS |
| `carnivorous-adobo` | `Balance = Inflow - Outflow - Loss` | `0.0031798800000000004` | `0.0031798800000000004` | PASS |

Actual area operands are source-slope geometry sums from committed hillslope
runfiles:

- `onshore-xenophobia`: `Area = 118391327.80000003 m^2`,
  `Runoff = 7.623926319373555e-06 mm`, `runvol = 0.9026067600000022 m^3`.
- `carnivorous-adobo`: `Area = 3333484.6800000006 m^2`,
  `Runoff = 9.539206881850737e-07 mm`, `runvol = 0.0031798800000000004 m^3`.

Unavailable typed operands remain null in produced parquet outputs:
`sbrunv`, `seddep_1`, and `sed_vol_conc` are null for both fixture rows. The
fixture event rows have zero detachment/deposition and sediment yield in the
touched publication row; those zero-valued fixture terms are pass-backed values,
not surrogate process fills, and are not used alone to close alias separation.

## Alias Separation

Focused writer tests:

```sh
cargo test -p openwepp-watershed-output typed_publication_writer
```

Result: `PASS` in focused iteration.

The test constructs a `WatershedPublicationFrame` with nonzero, non-aliased
values:

- `area_m2 = 5000`
- `precipitation_mm = 10`, expected `P = 50`
- `runoff_mm = 5`, expected `Q = 25`
- `deep_percolation_mm = 2`, expected `Dp = 10`
- `lateral_flow_mm = 1`, expected `latqcc = 5`
- `runoff_volume_m3 = 25`, expected `runvol = 25`
- `sediment_yield_kg = 6`, expected `sed_del = 6`

This rejects the plausible wrong aliases of depth-as-volume, runoff volume as
all depth-derived volume fields, and sediment/volume column swaps.

`typed_publication_writer_keeps_unavailable_operands_null` additionally proves
that unavailable typed publication operands such as `Area`, `Q`, `sbrunv`,
`seddep_1`, and `sed_vol_conc` emit nulls rather than silent zero defaults.
