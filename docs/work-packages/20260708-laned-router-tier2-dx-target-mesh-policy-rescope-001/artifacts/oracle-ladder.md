# Oracle Ladder

Status: EXECUTED-HOLD-DX-REFERENCE-ADEQUACY
Evidence mode: Ran.

## Case-4 Dimensionless Oracle

Ran:

```text
cargo nextest run -p openwepp-hillslope-orchestrator --lib case4_manning_solver_converges_to_iwagaki_oracle case4_manning_tvd_dissipation_is_mass_neutral_and_tv_transient_bounded case4_solver_and_oracle_source_histories_agree_exactly
```

Result: 3/3 passed. Case-4 is retained as dimensionless machinery evidence
only; no absolute candidate `dx` conclusion is drawn from the 24 m flume.

## Selected-Cohort Fine Reference

Reference rule: `dx2p5` is adequate only when `dx1p25` moves every judged
surface by no more than one third of the declared tolerance.

| Member | `dx2p5` vs `dx1p25` | Verdict |
|--------|---------------------|---------|
| `mn_corn_h4` | Outlet L1 rel `5.85386e-05`, max shape L1 `0.0201805`, annual sediment max rel `0` | Adequate |
| `n_idaho_forest_h1` | Outlet L1 rel `2.86582e-05`, max shape L1 `0.0166187`, annual sediment max rel `0.00252249` | Adequate |
| `wa_cascades_forest_h1` | `dx2p5` and `dx1p25` both failed active closure at day 1122 | Not adequate |
| `h2637` | Outlet L1 rel `0.0091233`, max shape L1 `0.0730917`, annual sediment max rel `0.0763134` | Synthetic stress; not adequate |

WA closure failures:
- `dx2p5`: day 1122 cascade residual `-0.0001100301742553711 m3`,
  relative `2.2504181899264942e-8`, above the `1e-9` active closure guard.
- `dx1p25`: day 1122 cascade residual `0.000011086463928222656 m3`,
  relative `2.2674852834578698e-9`, above the `1e-9` active closure guard.

Conclusion: real-cohort self-convergence is not closed for the wet-forest
runtime-stress member, so target-`dx` cannot be promoted.
