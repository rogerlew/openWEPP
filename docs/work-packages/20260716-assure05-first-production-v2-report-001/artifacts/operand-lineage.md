# ASSURE-05 Operand Lineage And Rejected Aliases

Status: FROZEN BEFORE PRODUCTION EVIDENCE EXECUTION

Evidence class: Static

| Claim | Authoritative operands, units, timing, and basis | Producer and real consumer | Rejected aliases |
| --- | --- | --- | --- |
| Two-day recurrence | `S_0`, daily `D_i`, prior `Qb_(i-1)`, prior `Qs_(i-1)`, `kb`, `ks`; `m3` and `d^-1`; 1,000 m2 synthetic hillslope | Independent analysis compared with `DirectGroundwaterRunState::advance_day` | Same-day debit; depth treated as volume; rounded manuscript values used as oracle |
| H2637 pre-export terminal storage | Manifest `S_0`, `sum(D)`, `sum(Qb)`, `Qb_N`, `sum(Qs)`, `Qs_N`, `S_N`; `m3`; full 731-day hillslope run | `05_runner_execution_and_outputs.rs` manifest; independently reconstructed from produced JSON | Latest runoff-event HBP baseflow as `Qb_N`; `latqcc`; `cbase`; inferred storage |
| H2637 post-export storage | Same operands, with `S_N-Qb_N-Qs_N` compared with `S_0+sum(D)-sum(Qb)-sum(Qs)`; `m3` | Produced run manifest and independent analysis | Pre-export `S_N` compared directly with full-export ledger; zero-filled absent operands |
| HBP transfer | Daily generated `Qb_i`, `Qs_i`; `m3` per day | Direct runtime → daily publication → runner HBP serializer → strict HBP parser → watershed `HillslopeContribution` and channel branch | Surface runoff; lateral `latqcc`; `cbase`; diagnostic counters; producer-only state |
| Contributing-area threshold | Generated groundwater volume, explicit `bftharea` in `ha`, watershed contributing area in `ha` | Parsed `gwcoeff.txt` authority → watershed direct consumer | Channel unit-area coefficient; hillslope width/area aliases |
| Lane D water closure | Active-router source/outlet/storage/clamp and separately exported groundwater terms; `m3` over full run | H2637 active-owner manifest and production HBP/Parquet outputs | Groundwater returned as surface-router source; runoff-event value used as terminal storage/export |

The H2637 fixture's zero deep-seepage coefficient makes `sum(Qs)` and `Qs_N`
legitimate zeros, not evidence that missing operands may be zero-filled. The
enabled manifest must contain them explicitly; disabled branches must leave
recurrence operands absent as required by `INV-GWBASEFLOW-008`.

The independent expected value is calculated from produced operands with a
separate procedure. It must not call the producer function, parse a producer-
asserted pass/fail flag, or reuse a stored residual as its oracle.
