# Spring Snowmelt Infiltration Localization

Status: executed
Evidence mode: Ran

Ran:

- Pre-fix comparison run: `/tmp/hphys0292_full_release_final_20260605T031145Z`.
- Final comparison run: `/tmp/hphys0292_full_release_segment_final_20260605T032734Z`.
- Final target table: `/tmp/hphys0292_full_release_segment_final_20260605T032734Z/reports/hphys0292_target_capacity_rows.md`.

Localization result:

- The first implementation exposed a real WB14 defect: `wb14_effective_conductivity_m_s` was equivalent to 40 mm/h, but early spring rows infiltrated only about 0.65-0.77 mm/day and emitted 29-36 mm/day residual `Q`.
- The final segment-level fix closes that capacity defect. Target H1/H7/H39 spring rows now show `Q = 0`, `wb12_infiltration = snow.routed_melt_m` within tolerance, and trace residual near zero except direct post-winter rain on final snow-free transition rows.
- Remaining spring residual ownership is not WB14 infiltration capacity. It remains in snowpack melt magnitude/timing and downstream storage/lateral/percolation after same-day ingress.

Representative final rows:

| Hill | Year | Julian | Routed Melt mm | Infiltration mm | Candidate Q mm | Baseline Q mm | Candidate Snow mm | Baseline Snow mm | Candidate Total-Soil mm | Baseline Total-Soil mm |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| H1 | 2014 | 141 | 37.238161 | 37.238161 | 0.000000 | 0.000000 | 124.324719 | 149.570000 | 558.011195 | 511.980000 |
| H7 | 2016 | 110 | 29.922681 | 29.922681 | 0.000000 | 0.000000 | 5.263813 | 21.170000 | 528.449804 | 520.880000 |
| H39 | 2014 | 142 | 36.226792 | 36.226792 | 0.000000 | 0.000000 | 83.824394 | 109.710000 | 463.508845 | 498.580000 |
