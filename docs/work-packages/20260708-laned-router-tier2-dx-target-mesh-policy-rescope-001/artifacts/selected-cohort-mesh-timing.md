# Selected-Cohort Mesh Timing

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY
Evidence mode: Ran.

Primary evidence:
- `artifacts/mesh-ladder-summary.md`
- `artifacts/mesh-ladder-summary.json`
- Package-local run trees under `artifacts/mesh-ladder-runs/`

Command:

```text
.venv/bin/python docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001/artifacts/run_mesh_ladder.py
```

Release binary:
`target/release/openwepp-cli-hill`
SHA256 `9a4f9c2755723c2e312dea460ed714bb183e283968fef2f003cf7690a71d48b8`.

## Real Cohort

| Member | Rung | Status | Wall | Solver steps | Trace rows | Outlet m3 |
|--------|------|--------|------|-------------:|-----------:|----------:|
| `mn_corn_h4` | fixed10 | PASS | `0:00.59` | 43819 | 2557 | 4473.730010803795 |
| `mn_corn_h4` | dx20 | PASS | `0:00.59` | 43819 | 2557 | 4473.730010803795 |
| `mn_corn_h4` | dx10 | PASS | `0:00.58` | 43819 | 2557 | 4473.730010803795 |
| `mn_corn_h4` | dx5 | PASS | `0:00.72` | 51324 | 2557 | 4473.610743669093 |
| `mn_corn_h4` | dx2p5 | PASS | `0:01.35` | 71475 | 2557 | 4473.470955961828 |
| `mn_corn_h4` | dx1p25 | PASS | `0:04.90` | 149657 | 2557 | 4473.22036520137 |
| `n_idaho_forest_h1` | fixed10 | PASS | `0:01.01` | 76900 | 1461 | 99723.5338460626 |
| `n_idaho_forest_h1` | dx20 | PASS | `0:01.49` | 95691 | 1461 | 99725.39021033363 |
| `n_idaho_forest_h1` | dx10 | PASS | `0:04.34` | 162332 | 1461 | 99725.84766563277 |
| `n_idaho_forest_h1` | dx5 | PASS | `0:20.65` | 403843 | 1461 | 99720.11900774238 |
| `n_idaho_forest_h1` | dx2p5 | PASS | `1:41.17` | 984337 | 1461 | 99718.51017221969 |
| `n_idaho_forest_h1` | dx1p25 | PASS | `8:03.47` | 2332425 | 1461 | 99717.42307921124 |
| `wa_cascades_forest_h1` | fixed10 | PASS | `0:15.73` | 3285282 | 10960 | 1007798.7596702089 |
| `wa_cascades_forest_h1` | dx20 | PASS | `0:15.72` | 3285282 | 10960 | 1007798.7596702089 |
| `wa_cascades_forest_h1` | dx10 | PASS | `0:18.61` | 3407233 | 10960 | 455174146.416385 |
| `wa_cascades_forest_h1` | dx5 | PASS | `1:01.90` | 4884752 | 10960 | 27678112025.381126 |
| `wa_cascades_forest_h1` | dx2p5 | FAIL | `2:18.96` | n/a | n/a | n/a |
| `wa_cascades_forest_h1` | dx1p25 | FAIL | `10:13.44` | n/a | n/a | n/a |

## Synthetic Stress

H2637 is recorded only as stress evidence. It shows the expected short-OFE
cost sensitivity: fixed/floor rungs around `40 s`, `dx2p5` at `47.48 s`, and
`dx1p25` at `2:46.00`.

H2637 `dx2p5` versus `dx1p25` failed adequacy on shape and annual sediment
surfaces, so it cannot support production promotion either.
