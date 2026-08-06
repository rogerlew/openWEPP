# Prospective Prediction Disposition

Evidence class: Ran + Static. Dispositioned 2026-08-06 without changing the
frozen windows, operators, geometry, or thresholds.

| Frozen prediction | Disposition | Evidence boundary |
| --- | --- | --- |
| Mid-winter melt decreases | `NOT EVALUABLE` | Shadow state reinitializes daily from the post-CoE pack and does not carry coherent snowfall or liquid state. |
| Peak SWE increases | `NOT EVALUABLE` | No persistent cross-day Stage 3 SWE trajectory exists. |
| Peak date moves later | `NOT EVALUABLE` | No persistent cross-day Stage 3 chronology exists. |
| Spring melt rate increases | `NOT EVALUABLE` | Same-substep liquid disposition and persistent state are absent. |
| Seasonal complete-carrier net energy approaches zero | `FAIL` for the frozen prescribed-state, resolved-domain screen | Corrected Snowbird median is `+170.2536 MJ m^-2`, not near zero. A coherent post-cutover seasonal balance remains not evaluable. |

The corrected shadow melts `0.5889 m` SWE versus authoritative CoE raw melt of
`0.4101 m` over the independently reconstructed windows. This is an adverse
melt-generativity signal. It is not a finding that mid-winter melt increased
in a coherent Stage 3 simulation.

The legacy `unused_positive_energy` and corrected
`Q_unallocated_after_exhaustion` are not comparable improvement metrics. The
latter is numerically zero only over evaluated resolved substeps and before
the omitted shadow cold-content-export ledger boundary.
