# Conservation Reconstruction

Status: `passed`

Evidence mode: `Ran:` independent fixture/source reconstruction, produced
parquet reconstruction, and focused writer tests.

## Independent Fixture Reconstruction

Fixtures:

- `/tmp/wshedw6_onshore_scaling_final/jobs1-full/out/interchange`
- `/tmp/wshedw6_carnivorous_scaling_final/jobs1-full/out/interchange`

Command class:

```sh
.venv/bin/python - <<'PY'
# Reads every committed pN.source.run and pN.slp, independently parses
# datver=2023.3 fwidth/slplen geometry, sums area, then compares the
# reconstructed area against produced parquet outputs.
PY
```

The audit does not call the Rust watershed publication area helper. It parses
the committed TOML source runfiles with Python `tomllib`, tokenizes the
referenced slope files, and independently sums `fwidth * slplen` for each OFE.

| Fixture | Source runfiles | Reconstructed area m2 | Published area m2 | Area delta m2 | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| `onshore-xenophobia` | `1305` | `118391327.8` | `118391327.80000003` | `2.9802322387695312e-08` | PASS |
| `carnivorous-adobo` | `32` | `3333484.6799999997` | `3333484.6800000006` | `9.313225746154785e-10` | PASS |

The same independent area operands were then used to reconstruct the normalized
runoff and published `Q`/`runvol` relationship:

| Fixture | Check | Reconstructed | Output | Result |
| --- | --- | ---: | ---: | --- |
| `onshore-xenophobia` | `Runoff = runvol / reconstructed_area * 1000` | `7.623926319373556e-06` | `7.623926319373555e-06` | PASS |
| `onshore-xenophobia` | `Q = Runoff(mm) * reconstructed_area / 1000` | `0.9026067600000021` | `0.9026067600000023` | PASS |
| `onshore-xenophobia` | `runvol == chanwb Inflow` | `0.9026067600000022` | `0.9026067600000022` | PASS |
| `carnivorous-adobo` | `Runoff = runvol / reconstructed_area * 1000` | `9.53920688185074e-07` | `9.539206881850737e-07` | PASS |
| `carnivorous-adobo` | `Q = Runoff(mm) * reconstructed_area / 1000` | `0.00317988` | `0.003179880000000001` | PASS |
| `carnivorous-adobo` | `runvol == chanwb Inflow` | `0.0031798800000000004` | `0.0031798800000000004` | PASS |

`runvol == chanwb Inflow` is a publication projection check, not a channel
water-balance closure claim. W6 does not publish channel `Outflow`, `Storage`,
`Baseflow`, `Loss`, or `Balance` values without authoritative volume operands.
Those `chanwb` fields are null in both committed fixture rows.

Produced row values:

- `onshore-xenophobia`: `Area = 118391327.80000003 m^2`,
  `Runoff = 7.623926319373555e-06 mm`,
  `runvol = 0.9026067600000022 m^3`.
- `carnivorous-adobo`: `Area = 3333484.6800000006 m^2`,
  `Runoff = 9.539206881850737e-07 mm`,
  `runvol = 0.0031798800000000004 m^3`.

Unavailable typed operands remain null in produced parquet outputs:

- `totalwatsed3`: `sbrunv`, `seddep_1`, and `sed_vol_conc`
- `chanwb`: `Outflow (m^3)`, `Storage (m^3)`, `Baseflow (m^3)`,
  `Loss (m^3)`, and `Balance (m^3)`
- `ebe_pw0`: `precip`

The current fixture event rows have zero detachment/deposition and sediment
yield in the touched publication row. Those zero-valued terms are actual
pass-backed routed-frame values and are not used to fill unavailable process
fields.

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
that unavailable typed publication operands such as `sbrunv`, `seddep_1`,
`sed_vol_conc`, and channel-balance detail fields emit nulls rather than silent
zero defaults.
