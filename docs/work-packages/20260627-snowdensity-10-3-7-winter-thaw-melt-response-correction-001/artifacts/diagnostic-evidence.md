# Diagnostic Evidence

Evidence mode: Ran.

Command:

```bash
.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_correction.py
```

Result:

```json
{
  "aggregate_depth_loss_deficit_delta_m": -6.476198898410999,
  "disposition": "WINTER-THAW-MELT-RESPONSE-CANDIDATE-IMPROVES",
  "schema": "snowdensity10-3-7-winter-thaw-melt-response-correction-v1",
  "under_ablation_delta": -24
}
```

Key paired Sleepers/Harvard evidence:

| Metric | Legacy | Candidate | Delta |
|---|---:|---:|---:|
| Under-ablation windows | 132 | 108 | -24 |
| Aggregate depth-loss deficit m | 24.105 | 17.629 | -6.476 |
| Modeled depth loss m | 15.868 | 26.400 | +10.532 |
| Raw CoE melt m | 8.685 | 8.506 | -0.179 |
| Routed melt m | 5.895 | 11.235 | +5.340 |
| Snowpack SWE loss m | 4.628 | 10.615 | +5.987 |
| Rain retained m | 1.664 | 2.286 | +0.622 |
| Rain released m | 1.267 | 0.620 | -0.647 |

Conservation/routing evidence:

| Model | Max SWE balance residual m | Max routed state-loss residual m | Min storage margin m | Passed |
|---|---:|---:|---:|---|
| `legacy_coe` | 0 | 0 | 0.009214 | true |
| `coe_winter_thaw_state_loss_v1` | 0 | 0 | 0 | true |

Coupled WAT evidence:

Command:

```bash
.venv/bin/python tools/snowfreeze_observed/winter_thaw_melt_response_coupled_gate.py
```

Result:

- Disposition: `WINTER-THAW-COUPLED-WAT-IMPROVES`.
- Coupled no-worse gate: `true`.
- Paired snow-control failures: `1147 -> 978`.
- Paired surfaces improved/worse: `4/0`.
- Direct trace selected count: `112502` candidate rows.
- Remaining blocker: `SNOW-CONTROL-NOT-CLEARED`.

Artifacts:

- `artifacts/winter-thaw-melt-response-correction.json`
- `artifacts/winter-thaw-melt-response-correction.md`
- `artifacts/coupled-wat-melt-response.json`
- `artifacts/coupled-wat-melt-response.md`
- `target/snowdensity10_3_7_winter_thaw_melt_response_correction/`
- `target/snowdensity10_3_7_coupled_wat_melt_response/`

Interpretation: the opt-in candidate improves the 10.3.6 event-window defect
signature by realizing positive thaw melt as snowpack state loss. The
candidate is conservation/routing-clean on active snowpack/liquid rows and
improves coupled WAT snow-control failures, but it leaves 108 thaw-window
under-ablation intervals and 978 coupled WAT snow-control failures. It is not
default activation, frost unblock, or full snow-control closure.
