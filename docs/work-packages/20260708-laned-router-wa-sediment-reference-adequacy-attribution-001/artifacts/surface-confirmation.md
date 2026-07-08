# Surface Confirmation

Evidence mode: Ran.

## Confirmed Trigger

The prior coupled space-time package reported one WA fine-reference adequacy
blocker:

| Field | Value |
|---|---|
| Member | `wa_cascades_forest_h1` |
| Role | `fine_reference_adequacy_dt75` |
| Candidate rung | `dx2p5_dt75` |
| Reference rung | `dx1p25_dt75` |
| Surface | `tdep:4` |
| Prior relative delta | `0.022131683796129127` |
| One-third adequacy threshold | `0.006666666666666667` |
| Prior verdict | `FAIL` |

Replay command:

```bash
.venv/bin/python docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/analyze_wa_sediment_reference.py
```

The replay recomputed the same year-4 annual `tdep` delta from the pass
parquets:

| Surface | Candidate | Reference | Delta |
|---|---:|---:|---:|
| Year-4 `tdep` kg | `0.6107069659777166` | `0.5974836468326581` | `0.013223319145058476` |
| Relative delta | | | `0.022131683796129127` |
| Absolute one-third threshold kg | | | `0.003983224312217721` |
| Excess over absolute threshold kg | | | `0.009240094832840755` |

## Prior Routed-Water Surfaces

The prior mesh-policy comparison for the same rung pair passed the routed-water
surfaces:

| Surface | Value | Verdict |
|---|---:|---|
| Terminal outlet relative L1 | `0.000012154323824704575` | PASS |
| Routed hourly shape max L1 | `0.00723148885806725` | PASS |
| End-window storage delta/source | `0.000005573386703295994` | PASS |
| Tail-fold delta/source | `0.000013395141479284063` | PASS |
| Uniform-shape row increase | `0` | PASS |
| Source-shape-degenerate row increase | `0` | PASS |

## Artifact Hashes

The analyzer records SHA-256 hashes for both pass parquets, both active trace
files, both manifests, and both prior summary JSON files in
`wa-sediment-attribution.json`.
