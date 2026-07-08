# Serial/Parallel Identity

Status: `passed`

Evidence mode: `Ran:`

Commands:

```sh
target/release/openwepp-cli-watershed \
  --run-dir tests/fixtures/watershed/p102-sediment-active/runs \
  --run-file case.run \
  --output-dir /tmp/wshedw7r_p102_fixture_jobs1 \
  --policy compat \
  --jobs 1 \
  --hillslope-binary target/release/openwepp-cli-hill

target/release/openwepp-cli-watershed \
  --run-dir tests/fixtures/watershed/p102-sediment-active/runs \
  --run-file case.run \
  --output-dir /tmp/wshedw7r_p102_fixture_jobs4 \
  --policy compat \
  --jobs 4 \
  --hillslope-binary target/release/openwepp-cli-hill
```

Results:

- `--jobs 1`: `exit=0`, `wall=0:00.78`
- `--jobs 4`: `exit=0`, `wall=0:00.74`

Decoded parquet comparison:

| File | Schema columns | Rows | Schema delta | Row delta |
| --- | ---: | ---: | ---: | ---: |
| `chan.out.parquet` | `11` | `1` | `0` | `0` |
| `chanwb.parquet` | `15` | `1` | `0` | `0` |
| `chnwb.parquet` | `30` | `1` | `0` | `0` |
| `ebe_pw0.parquet` | `16` | `1` | `0` | `0` |
| `loss_pw0.all_years.chn.parquet` | `13` | `1` | `0` | `0` |
| `loss_pw0.all_years.class_data.parquet` | `10` | `1` | `0` | `0` |
| `loss_pw0.all_years.hill.parquet` | `13` | `1` | `0` | `0` |
| `loss_pw0.all_years.out.parquet` | `5` | `1` | `0` | `0` |
| `loss_pw0.chn.parquet` | `13` | `1` | `0` | `0` |
| `loss_pw0.class_data.parquet` | `9` | `1` | `0` | `0` |
| `loss_pw0.hill.parquet` | `13` | `1` | `0` | `0` |
| `loss_pw0.out.parquet` | `4` | `1` | `0` | `0` |
| `soil_pw0.parquet` | `22` | `1` | `0` | `0` |
| `totalwatsed3.parquet` | `80` | `1` | `0` | `0` |

Conclusion: scheduling does not change decoded public watershed outputs.
