# Baseline Provenance Map

Status: complete

Evidence mode: Static

Normative baseline: commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, inspected with
`git show <sha>:<path>`. The checkout at
`/workdir/wepp-forest_260430_baseline` was at `2f65506d...`; its working HEAD
was not treated as normative.

| Family | Pinned source | Authority admitted here | Boundary/gap |
|---|---|---|---|
| Daily winter/hydrology custody | `src/contin.for:839-882,907-922` | winter produces `wmelt`; ordinary hydrology receives rain + melt | no unified surface-energy recipient |
| Water ordering | `src/watbal.for:331-344,431-498` | liquid ingress, percolation, then ET mutation | water partition remains with hydrology |
| Daily ET demand | `src/evap.for:171-360,428-680`; `src/evappm.for:178-430` | radiation/ET demand and water withdrawal lineage | not an hourly prognostic surface-energy solver; soil heat is neglected/approximated |
| Frost conduction/phase | `src/frostn.for:383-686`; `src/frzng.for:331-381`; `src/frznw.for:62-113` | resistance, conductive boundary, and phase mechanics | `surtmp` is supplied, not jointly solved |
| Event infiltration/runoff | `references/50201000/chap4.pdf` §4.2 and legacy `irs` family | Green-Ampt/rainfall-excess owner | LSE cannot repartition water |
| Daily water/ET reference | `references/50201000/chap5.pdf` §§5.1-5.3 | mass balance and modified-Ritchie context | no complete coupled LSE algorithm |

Conclusion: no pinned routine closes net shortwave, net longwave, sensible,
latent, precipitation heat, ground heat, and storage while solving a snow-free
surface temperature. Constitutive gaps remain `AUTHORITY_MISSING` and
`NON_PROMOTABLE`; no legacy clamp or fallback was generalized.
