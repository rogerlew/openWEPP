# Acquisition Log

Evidence mode: Ran.

Commands:

```bash
.venv/bin/python tools/snowfreeze_observed/snotel_density_three_way.py \
  --cache target/snowfrost_fidelity_h/snotel_awdb_cache \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowfrost_fidelity_h fetch

.venv/bin/python tools/snowfreeze_observed/snotel_density_three_way.py \
  --cache target/snowfrost_fidelity_h/snotel_awdb_cache \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowfrost_fidelity_h normalize
```

One initial fetch invocation failed because global options were placed after the
subcommand; it was rerun with the corrected argparse ordering shown above.

Normalized corpus:

| Site | Station | Rows | Paired SWE/depth density rows | Observed SSD kg/m3 | STO |
| --- | --- | ---: | ---: | ---: | --- |
| `snotel_mica_creek_st_joe_id` | `623:ID:SNTL` | 12448 | 2540 | 370 | yes |
| `snotel_paradise_wa` | `679:WA:SNTL` | 15204 | 3170 | 495 | yes |
| `snotel_css_lab_ca` | `428:CA:SNTL` | 15379 | 1744 | 380 | yes |
| `snotel_snowbird_ut` | `766:UT:SNTL` | 11946 | 2754 | 445 | yes |
| `snotel_niwot_co` | `663:CO:SNTL` | 16022 | 3382 | 340 | no |

Checked-in normalized outputs:

- `tests/fixtures/snotel_observed/observations/manifest.json`
- `tests/fixtures/snotel_observed/observations/sites/*.csv`
- `tests/fixtures/snotel_observed/observations/provenance/*.json`
- `tests/fixtures/snotel_observed/observations/ssd_characterization.{json,md}`

Raw AWDB responses are local cache artifacts under
`target/snowfrost_fidelity_h/snotel_awdb_cache/` and are not committed.
