# SNOWDENSITY-10.3.15 Default Activation Under Active Cap

Evidence mode: Static/Ran.

- Disposition: `COMPLETE-DEFAULT-ACTIVATED-UNDER-ACTIVE-CAP`
- Activation complete: `True`
- Default trace ok: `True`
- Rollback trace ok: `True`
- Paired rows: `1415`
- Snow-control failures: `498`
- Frost attribution blocker: `SNOW-CONTROL-RESIDUALS-REMAIN`
- Active density cap: `522.0 kg m^-3`

## Activated Default

- Melt model: `coe_liquid_holding_capacity_v1`
- Density model: `physics_bulk_density_compaction_v1`
- Trace melt count: `112502`
- Trace density count: `112502`

## Rollback

- Surface: `sleepers_south_field`
- Melt model: `legacy_coe`
- Density model: `legacy_wepp`
- Trace melt count: `13880`
- Trace density count: `13880`

## Surface Results

| Surface | Scope | Cover | Paired rows | Failures |
|---|---|---|---:|---:|
| `hjandrews_conifer` | observation_blocked | conifer | 0 | 0 |
| `sleepers_south_field` | paired_observation | open_field | 384 | 150 |
| `sleepers_w9_hardwood` | paired_observation | hardwood | 193 | 57 |
| `harvard_hardwood` | paired_observation | hardwood | 448 | 153 |
| `harvard_open` | paired_observation | open | 390 | 138 |
| `hubbardbrook_deciduous` | observation_blocked | deciduous | 0 | 0 |
| `hubbardbrook_mixed` | observation_blocked | mixed | 0 | 0 |

## Boundary Disposition

- Parser/runfile/user CLI selector added: `false`.
- Fixture inputs changed: `false`.
- Public output schema changed: `false`.
- Density cap changed: `false`.
- New process physics added: `false`.
- Frost attribution remains blocked while snow-control residuals remain.
