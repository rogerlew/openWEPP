# Annual Sediment Metric Replay

Evidence mode: Ran.

## Metric

- Material-year threshold: `0.05` of a column's total absolute reference annual sum.
- Material member-years keep the existing relative tolerance.
- All member-years are also bounded by per-column annual-vector L1 relative to the reference annual vector.
- Low-contribution member-year strict-relative excursions remain reported but are not standalone blockers.

## Summary

- Comparisons replayed: `21`
- Strict-relative annual sediment blockers: `1`
- Rev-44 annual sediment blockers: `0`

### Strict-Relative Blockers

- `wa_cascades_forest_h1` `fine_reference_adequacy_dt75` `dx2p5_dt75` vs `dx1p25_dt75`: `tdep:4` `0.0221316838` > `0.00666666667`

### Rev-44 Replay Table

| Role | Member | Candidate | Reference | Strict max rel | Strict surface | Vector max rel | Vector column | Material max rel | Low max rel | Rev-44 verdict |
|---|---|---|---|---:|---|---:|---|---:|---:|---|
| `fine_reference_adequacy_dt300` | `mn_corn_h4` | `dx2p5_dt300` | `dx1p25_dt300` | `0` | `None` | `0` | `None` | `0` | `0` | PASS |
| `fine_reference_adequacy_dt75` | `mn_corn_h4` | `dx2p5_dt75` | `dx1p25_dt75` | `0` | `None` | `0` | `None` | `0` | `0` | PASS |
| `candidate_vs_reference_dt300` | `mn_corn_h4` | `dx5_dt300` | `dx2p5_dt300` | `0` | `None` | `0` | `None` | `0` | `0` | PASS |
| `candidate_vs_reference_dt75` | `mn_corn_h4` | `dx5_dt75` | `dx2p5_dt75` | `0` | `None` | `0` | `None` | `0` | `0` | PASS |
| `timestep_control_dx5` | `mn_corn_h4` | `dx5_dt300` | `dx5_dt75` | `0` | `None` | `0` | `None` | `0` | `0` | PASS |
| `timestep_control_dx2p5` | `mn_corn_h4` | `dx2p5_dt300` | `dx2p5_dt75` | `0` | `None` | `0` | `None` | `0` | `0` | PASS |
| `timestep_control_dx1p25` | `mn_corn_h4` | `dx1p25_dt300` | `dx1p25_dt75` | `0` | `None` | `0` | `None` | `0` | `0` | PASS |
| `fine_reference_adequacy_dt300` | `n_idaho_forest_h1` | `dx2p5_dt300` | `dx1p25_dt300` | `0.00252079938` | `sedcon_5:3` | `0.00102778696` | `sedcon_1` | `0.00252079938` | `0` | PASS |
| `fine_reference_adequacy_dt75` | `n_idaho_forest_h1` | `dx2p5_dt75` | `dx1p25_dt75` | `0.00151622664` | `sedcon_5:4` | `0.000776019838` | `sedcon_1` | `0.00151622664` | `0` | PASS |
| `candidate_vs_reference_dt300` | `n_idaho_forest_h1` | `dx5_dt300` | `dx2p5_dt300` | `0.00266009935` | `sedcon_5:3` | `0.000484530771` | `sedcon_5` | `0.00266009935` | `0` | PASS |
| `candidate_vs_reference_dt75` | `n_idaho_forest_h1` | `dx5_dt75` | `dx2p5_dt75` | `0.0024683212` | `sedcon_2:3` | `0.000500223314` | `sedcon_4` | `0.0024683212` | `0` | PASS |
| `timestep_control_dx5` | `n_idaho_forest_h1` | `dx5_dt300` | `dx5_dt75` | `2.06450199e-05` | `sedcon_1:1` | `1.85982105e-05` | `tdet` | `2.06450199e-05` | `0` | PASS |
| `timestep_control_dx2p5` | `n_idaho_forest_h1` | `dx2p5_dt300` | `dx2p5_dt75` | `0.000180105966` | `sedcon_5:3` | `8.17169239e-05` | `sedcon_4` | `0.000180105966` | `0` | PASS |
| `timestep_control_dx1p25` | `n_idaho_forest_h1` | `dx1p25_dt300` | `dx1p25_dt75` | `0.00270897811` | `sedcon_1:3` | `0.000578915291` | `sedcon_1` | `0.00270897811` | `0` | PASS |
| `fine_reference_adequacy_dt300` | `wa_cascades_forest_h1` | `dx2p5_dt300` | `dx1p25_dt300` | `0.00636869125` | `tdep:3` | `0.000886567213` | `tdep` | `0.00636869125` | `0.00443032026` | PASS |
| `fine_reference_adequacy_dt75` | `wa_cascades_forest_h1` | `dx2p5_dt75` | `dx1p25_dt75` | `0.0221316838` | `tdep:4` | `0.000612007475` | `tdep` | `0.00173788779` | `0.0221316838` | PASS |
| `candidate_vs_reference_dt300` | `wa_cascades_forest_h1` | `dx5_dt300` | `dx2p5_dt300` | `0.0184359971` | `tdep:3` | `0.00152796606` | `tdep` | `0.0184359971` | `0.00165760779` | PASS |
| `candidate_vs_reference_dt75` | `wa_cascades_forest_h1` | `dx5_dt75` | `dx2p5_dt75` | `0.0181162061` | `tdep:4` | `0.00174682289` | `tdep` | `0.0111236625` | `0.0181162061` | PASS |
| `timestep_control_dx5` | `wa_cascades_forest_h1` | `dx5_dt300` | `dx5_dt75` | `0.00872331271` | `tdep:3` | `0.000632668145` | `tdep` | `0.00872331271` | `0.00213221926` | PASS |
| `timestep_control_dx2p5` | `wa_cascades_forest_h1` | `dx2p5_dt300` | `dx2p5_dt75` | `0.0185829932` | `tdep:4` | `0.00066649349` | `tdep` | `0.00148071486` | `0.0185829932` | PASS |
| `timestep_control_dx1p25` | `wa_cascades_forest_h1` | `dx1p25_dt300` | `dx1p25_dt75` | `0.00760141462` | `tdep:4` | `0.000729752645` | `tdep` | `0.00658649017` | `0.00760141462` | PASS |

## Decision Impact

The rev-44 annual sediment metric closes the WA `tdep:4` strict-relative
low-denominator blocker without changing routed-water, shape, storage,
tail-fold, closure, active selector, or production default behavior.

No `dx5` production flip is made by this package.
