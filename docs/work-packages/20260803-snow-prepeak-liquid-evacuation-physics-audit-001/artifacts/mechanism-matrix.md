# Mechanism Matrix

Status: `executed / reviewed / verified / HOLD-EVIDENCE`

Evidence mode: `Static + Ran`

`SUPPORTED` describes a demonstrated implementation interaction or causal
bound. It does not declare the governing physics correct. Primary statistics
exclude right-censored WY2025.

| Rank | Mechanism | Signature and magnitude | Falsifier | Verdict |
|---:|---|---|---|---|
| 1 | Gross-positive CoE generation plus positive-parts export | Snowbird median pack loss is `0.5296 m` versus `0.5379 m` gross-positive hourly applied melt; negative-hour magnitude is `0.1243 m`. Across the cohort, 6,716 days contain mixed signed hours and 5,658 negative-daily-net days still lose `5.539 m`. | Authoritative stateful thermal storage that receives negative terms and offsets later melt, demonstrated on the same events. | `SUPPORTED`, high implementation/magnitude confidence; physical correction `UNRESOLVED`. |
| 2 | Modern capacity/export trajectory versus legacy density-gated routing | Same-binary legacy routing reduces median loss by `0.198/0.180/0.610/0.403 m` at Mica/Niwot/Paradise/Snowbird and increases peak SWE by `0.279/0.143/0.751/0.279 m`. | A controlled boundary-state experiment showing negligible export divergence. | Order-one causal rollback bound `SUPPORTED`; legacy correctness and isolated capacity ownership `UNRESOLVED`. |
| 3 | Warm/aerodynamic CoE `B/C` scale | Snowbird signed medians are `B=0.1473 m`, `C=0.1990 m`; together they are the largest signed raw-term scale. Niwot is heterogeneous. Signed sums are not causal shares of gross-positive melt. | Event-level authoritative temperature/dewpoint/wind agreement plus exact term reconstruction. | Magnitude `SUPPORTED`; forcing versus empirical formula `UNRESOLVED`. |
| 4 | Missing authoritative pre-export enthalpy/cold-content coupling | Stage 3 runs downstream and is snow-neutral. Stage3-off and longwave-on both have zero authoritative mass response, while longwave increases Snowbird refreeze `0.01287 m`. | A complete authoritative thermal solve that acts before export and remains immaterial. | Architectural fact `SUPPORTED`; true SWE effect `UNRESOLVED`. |
| 5 | Mixed exported melt plus refreeze reachability | The accepted trace has 1,031 such days across the cohort and 298 at Snowbird, contrary to INV-SNOWFREEZE-015's empirical premise. | A corrected predicate or authority showing these trace operands do not represent the governed branch. | Reachability `SUPPORTED`, high; correct signed-hour treatment `UNRESOLVED`. |
| 6 | Wet-compaction duplicate data-flow alias | `liquid_for_compaction/1000 = 2*pack_loss + rain_released` closes within `2.78e-17 m`; total duplicated state-loss component is `73.123 m` over primary windows. Density remains outside the direct CoE/SWE boundary. | Active multilayer authority proving the two inputs are disjoint water or prescribing a different complete driver. | Duplicate alias `SUPPORTED`; physical-defect verdict `UNRESOLVED`; direct SWE cause `EXCLUDED`. |
| 7 | Stage-3 refreeze/shortwave as current direct mass cause | Stage3-off has zero median mass/peak response at all sites; maximum loss delta is `3.33e-16 m`. | Material same-binary mass response to Stage3-off. | Current direct cause `EXCLUDED`, high. |
| 8 | Explicit longwave omission as current direct mass cause | Longwave-on has exact zero mass/peak response at every site but changes diagnostic refreeze. | Any authoritative mass delta in a same-binary longwave-only replay. | Current direct cause `EXCLUDED`, high; stateful thermal implication `UNRESOLVED`. |
| 9 | Shortwave/albedo state feedback | `A=0.0883 m` at Snowbird is secondary on the frozen trajectory; prior ERA horizontal-daily shortwave is higher than retained forcing. Neither is a stateful slope/hour ablation. | Site-authoritative albedo plus slope-aware same-binary replay. | Dominance on current trajectory `EXCLUDED`; state-feedback contribution `UNRESOLVED`. |
| 10 | SIMIMPL cloud proxy | Prior checksum-bound diagnostic reports Snowbird proxy/ERA cloud `0.913/0.769`, correlation `0.230`; no retained stateful cloud-only replay exists. | Local cloud authority or shortwave-fixed stateful replay. | Proxy mismatch `SUPPORTED`; SWE magnitude `UNRESOLVED`. |
| 11 | Rain heat/rain-on-snow as systemic driver | Snowbird `D=0.00443 m`; 79.7% of aggregate primary-window loss is on dry days. Cross-site dry fractions range 42.9–81.7%. | Events where rain heat/release supplies an order-one share. | Snowbird/systemic dominance `EXCLUDED`, high; site-specific role retained. |
| 12 | Capacity as seasonal storage or pack exhaustion as dominant loss | Snowbird median maximum capacity/retained liquid is `0.01232/0.01196 m`, far below seasonal loss. Hourly exhaustion pre-state is missing. | Typed exhaustion test with carried liquid and exact pre/post operands. | Seasonal-storage dominance `EXCLUDED`; exhaustion alias `UNRESOLVED`. |
| 13 | CoE Rust transcription defect | Static term/unit mapping matches pinned `dac3c950`; independent hourly identities close within `2.02e-17 m`. Handbook and post-2007 implementation still differ. | Same-input mismatch to the pinned blob or superseding authority. | Rust mistranscription `EXCLUDED`, high; authority choice `UNRESOLVED`. |
| 14 | Downstream accounting/publication as loss owner | Primitive mass closes within `1.00e-12 m`; routed alias closes within `1.56e-17 m`. WAT values were not parsed, so the dynamic claim is limited to trace storage/routing plus static publication lineage. | Independent real-consumer value reconstruction outside tolerance. | Trace storage/routing owner `EXCLUDED`; full WAT value closure `NOT CLAIMED`. |
| 15 | Tested phase-threshold family as sufficient recovery | Predecessor Harder-Pomeroy is invariant to `rst`; extreme legacy thresholds were insufficient. This does not test all phase or precipitation-forcing error. | An authoritative phase/forcing reconstruction that recovers the deficit. | Tested `rst` recovery `EXCLUDED`; systemic phase/precipitation error `UNRESOLVED`. |

## Evidence Hold

Stage-3 mass and energy are reconstructable, but independent liquid closure is
not: four produced operands are absent from the real JSONL consumer. This is a
current package exit criterion, so the terminal disposition remains
`HOLD-EVIDENCE`.
