# Sediment Fixture Inventory

Status: `executed-hold`

Evidence mode: `Ran:` local CLI probes plus `Static:` source/output
inspection.

## Result

No W7 acceptance fixture was adopted. Current openWEPP did not produce a
production-generated nonzero sediment pass/publication signal for any inspected
candidate, so W7 cannot truthfully close complete.

## Committed Fixtures

| Fixture | Full watershed | Probe | Sediment signal | Disposition |
| --- | --- | --- | --- | --- |
| `tests/fixtures/watershed/carnivorous-adobo/` | yes, `32` hillslopes | `target/release/openwepp-cli-watershed --jobs 8` to `/tmp/wshedw7_probe_carn/out` | `totalwatsed3`: `tdet=0`, `tdep=0`, `sed_del=-0.0`; all class deposition null | Rejected for W7 nonzero-sediment acceptance; still useful development fixture. |
| `tests/fixtures/watershed/onshore-xenophobia/` | yes, `1305` hillslopes | W6 evidence reviewed | W6 recorded zero detachment/deposition and sediment yield | Rejected for W7 nonzero-sediment acceptance. |
| `tests/fixtures/cli01/hillslope_run_dir/` | hillslope only | `openwepp-cli-hill` probe; generated output removed from fixture tree | loss manifest shows `ofe_count=1`; no sediment fields | Rejected; not a watershed fixture and one-OFE erosion producer disabled. |

## Local Source Substrates

`/wc1/runs/in/insensible-aliquot/wepp` has `477` hillslope runfiles and
`370` multi-OFE hillslope slopes. It does not appear to contain public watershed
`pw0.str` / `pw0.chn` inputs, so it is not directly adoptable as a full
watershed fixture. It is still useful evidence because its legacy output is
sediment-active while current openWEPP production HBP emission remains zero.

Legacy source evidence:

- `/wc1/runs/in/insensible-aliquot/wepp/output/H1.loss.dat` reports
  `Soil Loss (Avg. of Net Detachment Areas) = 0.239 kg/m2`,
  maximum soil loss `0.449 kg/m2`, and deposition summaries.

Current openWEPP probes, all with `erod14_wave2_enabled = true`, produced zero
pass sediment:

| Hillslope | OFEs | Rows | `max_tdet` | `sum_tdet` | `max_tdep` | `sum_tdep` | `max_sedcon_sum` | Notes |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| `1` | `5` | `18262` | `0.0` | `0.0` | `0.0` | `0.0` | `0.0` | Warning: `MOFE01-MG-W-001`; EROD14 qin is water-transfer-only. |
| `21` | `2` | `18262` | `0.0` | `0.0` | `0.0` | `0.0` | `0.0` | Same warning; `erod14_qin_clamped_events=22`. |
| `172` | `2` | `18262` | `0.0` | `0.0` | `0.0` | `0.0` | `0.0` | Same warning; `erod14_qin_clamped_events=43`. |
| `297` | `10` | `18262` | `0.0` | `0.0` | `0.0` | `0.0` | `0.0` | Same warning; `erod14_qin_clamped_events=279`. |
| `333` | `8` | `18262` | `0.0` | `0.0` | `0.0` | `0.0` | `0.0` | Same warning; `erod14_qin_clamped_events=182`. |
| `390` | `7` | `18262` | `0.0` | `0.0` | `0.0` | `0.0` | `0.0` | Same warning; `erod14_qin_clamped_events=58`. |
| `437` | `5` | `18262` | `0.0` | `0.0` | `0.0` | `0.0` | `0.0` | Same warning; `erod14_qin_clamped_events=24`. |

## Hold Boundary

W7 excludes hillslope erosion/sediment physics changes for convenience. The
observed blocker is outside the W7 publication/fixture envelope:
current production hillslope HBP generation does not emit nonzero sediment for
real sediment-active source substrates, even when multi-OFE EROD14 is enabled.
