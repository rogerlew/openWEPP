# V3 Test-Vector Ledger

Status: `independent oracle and vector evidence complete`

Evidence mode: `Static`

| Family | Positive/reduction vectors | Required poisons/failures |
| --- | --- | --- |
| Radiation | two ranks, mixed leaf/stem, unequal optics, non-unit clumping, direct/diffuse VIS/NIR, ground/upward reflection, leaf-only, stem-only, identical optics, zero leaf/stem/direct/plant | leaf/stem-only optics, arithmetic mean, area-only partition, clumping twice/omitted, sunlit plant as leaf, stem PAR, band/component swap, zero lower boundary, direct-summed reflection |
| Wind | reference wind, exact `u_star`, three semantic winds, three dimensions/conductances | direct `u_ref`, hidden floor, heat/vapor roughness, invalid geometry/nonpositive speed |
| Hydraulics/potential | Emax, accepted beta, four nodes, every q1/q2/q3, class/total continuity, two layers, dry/frozen, alternate starts | beta-one publication, sequential/post-hoc stress, aggregate-only equality, authorization in potential, redistribution, singular Jacobian, iteration limit |
| Migration | nonempty identical V2 vector | empty/unequal vector, average, first, root-weighted, broadcast |
| Respiration | Rd25, sun/shade response, An, exact class-area carbon debit | nonpositive Atkin/clamp, old `rd_leaf_n_rate`, wrong response, class swap, double debit |
| Diagnostics | one payload per solve/pass failure | opaque error, missing identity/residual evidence, nonfinite payload, usable last iterate |

Expected values are produced only by the package-local stdlib Python oracle and
are consumed independently by Rust authority tests.
