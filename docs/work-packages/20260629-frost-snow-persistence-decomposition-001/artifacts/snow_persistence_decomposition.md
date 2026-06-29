# FROST Snow-Persistence Decomposition

Evidence mode: Ran.

Diagnostic-only: no melt-model, snow-model, frost-model, contract, default, fixture, or schema change.

## Summary

- Scoped cells: `9` (`7` snow-buried + `2` mixed buried portions).
- Route counts: `{'INCONCLUSIVE-SPARSE-OBS': 8, 'OVER-ACCUMULATION-FORCING-LIMITED': 1}`.
- Determination: `NOT-ESTABLISHED-SPARSE-OBS`.
- Next route: do not promote a melt-rate fix from these cells alone; carry sparse snow-persistence uncertainty and keep Qwet limited to snow-free subset.

The rate gate reuses the existing snow-program winter-thaw discriminator: paired intervals no longer than `45` days, observed depth loss at least `0.05 m`, and modeled loss deficit greater than `max(0.05 m, 0.3 * observed_loss)`.

## Per-Cell Routes

| Cell | Prior snow route | Route | Confidence | Paired rows | Obs ablation intervals | Under-ablation intervals | Peak obs/model/resid m | Loss obs/model/ratio | Reason |
| --- | --- | --- | --- | ---: | ---: | ---: | --- | --- | --- |
| `site1_sleepers_south_field_vt:1987:thaw` | `MIXED-SNOW-CONTROL` | `INCONCLUSIVE-SPARSE-OBS` | `low` | `2` | `1` | `0` | `0.55075` / `0.535217` / `-0.0155331` | `0.19955` / `0.218793` / `1.09643` | paired ablation rate is comparable and modeled peak does not exceed observed tolerance; snow-persistence mechanism is not resolved |
| `site2_sleepers_w9_hardwood_vt:1994:thaw` | `SNOW-BURIED-ACCUMULATION` | `INCONCLUSIVE-SPARSE-OBS` | `low` | `3` | `1` | `0` | `0.868333` / `0.799094` / `-0.0692395` | `0.256` / `0.308121` / `1.2036` | paired ablation rate is comparable and modeled peak does not exceed observed tolerance; snow-persistence mechanism is not resolved |
| `site2_sleepers_w9_hardwood_vt:1995:thaw` | `MIXED-SNOW-CONTROL` | `INCONCLUSIVE-SPARSE-OBS` | `none` | `0` | `0` | `0` | `` / `` / `` | `` / `` / `` | only 0 paired snow rows in scoped window; cannot estimate observed-vs-modeled ablation rate |
| `site2_sleepers_w9_hardwood_vt:1996:thaw` | `SNOW-BURIED-UNDER-MELT` | `INCONCLUSIVE-SPARSE-OBS` | `low` | `6` | `2` | `1` | `0.513` / `0.663257` / `0.150257` | `0.241334` / `0.439634` / `1.82168` | 1/2 intervals under-ablated, but aggregate paired modeled loss is not below observed loss; do not call spring under-melt |
| `site2_sleepers_w9_hardwood_vt:1997:thaw` | `SNOW-BURIED-UNDER-MELT` | `OVER-ACCUMULATION-FORCING-LIMITED` | `moderate` | `8` | `3` | `0` | `0.865` / `1.12961` / `0.264606` | `0.543333` / `1.20064` / `2.20978` | modeled peak depth exceeds observed peak by more than TOL-SNOWFREEZE-011 while paired observed ablation intervals do not show a modeled loss-rate deficit |
| `site2_sleepers_w9_hardwood_vt:2004:thaw` | `SNOW-BURIED-UNDER-MELT` | `INCONCLUSIVE-SPARSE-OBS` | `low` | `2` | `1` | `0` | `0.8228` / `0.908603` / `0.0858025` | `0.1552` / `0.274621` / `1.76947` | paired ablation rate is comparable and modeled peak does not exceed observed tolerance; snow-persistence mechanism is not resolved |
| `site2_sleepers_w9_hardwood_vt:2009:thaw` | `SNOW-BURIED-UNDER-MELT` | `INCONCLUSIVE-SPARSE-OBS` | `low` | `5` | `1` | `0` | `0.938` / `1.0298` / `0.0917957` | `0.68` / `0.986939` / `1.45138` | paired ablation rate is comparable and modeled peak does not exceed observed tolerance; snow-persistence mechanism is not resolved |
| `site2_sleepers_w9_hardwood_vt:2010:thaw` | `SNOW-BURIED-UNDER-MELT` | `INCONCLUSIVE-SPARSE-OBS` | `low` | `3` | `1` | `0` | `0.588` / `0.586105` / `-0.00189472` | `0.486` / `0.423579` / `0.871562` | paired ablation rate is comparable and modeled peak does not exceed observed tolerance; snow-persistence mechanism is not resolved |
| `site2_sleepers_w9_hardwood_vt:2011:thaw` | `SNOW-BURIED-ACCUMULATION` | `INCONCLUSIVE-SPARSE-OBS` | `low` | `2` | `0` | `0` | `0.698` / `0.875089` / `0.177089` | `` / `` / `` | paired rows do not include an observed ablation interval meeting the reused 0.05 m floor |

## Spring-Melt Lineage

- Source: `docs/work-packages/20260627-snowdensity-10-3-10-spring-pack-depletion-compaction-adjudication-001/artifacts/spring-pack-depletion-compaction-adjudication.json`.
- Sleepers March/April failures in 10.3.10: `153`.
- Sleepers 10.3.10 failure adjudication counts: `{'CAP_LIMITED_DEPLETION_REQUIRED': 15, 'COMPACTION_ONLY_FEASIBLE_WITHIN_522_CAP': 121, 'PATCHY_MELTOUT_OR_DEPLETION_REQUIRED': 10, 'UNDER_PERSISTENCE_OR_ACCUMULATION_DEFICIT': 7}`.

## GAP-SNOWFREEZE-002 Disposition

`GAP-SNOWFREEZE-002` remains open. The snow-buried thaw-late residual routes to `NOT-ESTABLISHED-SPARSE-OBS` with route counts `{'INCONCLUSIVE-SPARSE-OBS': 8, 'OVER-ACCUMULATION-FORCING-LIMITED': 1}`. Do not promote a melt-rate fix from these cells alone; carry sparse snow-persistence uncertainty and keep Qwet limited to snow-free subset. The two snow-free persistent cells remain the deferred `Qwet` subset.
