# A01 Findings

Six surfaces swept (S1..S6; notes in `sweep-notes.md`). Every finding
verified with file:line evidence; evidence class per row. No production
source was modified.

| # | Finding | Class | Verified | Disposition |
|---|---|---|---|---|
| F-A1 | `INV-RUNOFFPART-029` case-classifier outcome is **seed-static**: computed once from seed `qout=0` with synthetic companion scalars, never recomputed per day; runtime validation is seeded-vs-seeded | spec-vs-code divergence | **confirmed** (Ran: producer, non-recompute, validator-only consumption, manifest label all read) | **routed to the `INV-RUNOFFPART-030` hold-closure package** — consumption is validator-only, the water path never reads it, the hold covers sediment coupling, and `erod14_qin_source_policy` labels the seeding. The real per-day classifier belongs to MOFE sediment acceptance; Lane D supplies its hydraulic operands |
| F-A2 | **Runon excluded from infiltration supply**: WB14 infiltrates the hyetograph only; runon enters post-infiltration and cannot re-infiltrate downslope, opposite to legacy hourly source intent at both granularities (pinned baseline `wepp-forest_260430_baseline/src/watbal_hourly.for:361-363` daily `fin += net upstream runon`; `:471-473` hourly `xfin += (ui_LfUrf + ui_SUrunf) × area-ratio` — surface and lateral both; unchanged in current source `:411-413`) | source-intent divergence (fidelity; conservation unaffected — INV-028 closure holds either way) | **confirmed** (Ran: both sources read) | **contract decision → Lane D D1** must specify runon re-infiltration semantics (Papanicolaou assumption 2 — per-OFE Green–Ampt of routed excess — aligns with legacy intent); magnitude-impact interpretation flagged to Lane C3; operator may alternatively order a pre-D Defect-Closure ExecPlan anchoring legacy `fin` semantics under ADR-0024. **Comparator hygiene:** expect openWEPP-high surface runoff / low infiltration vs legacy on runon-bearing MOFE days |
| F-A3 | Dormant `QcapSoftLimit` clamp-status taxonomy value — defined and taxonomy-tested, never emitted by any producer | dead surface | confirmed (Ran: grep) | hardening/cleanup candidate (mechanical lane) |
| F-A4 | No external tool can reconstruct per-OFE conservation from published WAT/PASS (the wepp-forest audit's independence property); `INV-WATBAL-096` itself warns row aliases are structural-only | capability gap | confirmed (Ran: tools sweep) | follow-up: external per-OFE closure audit tool, designed under the B11 `latqcc`-day constraint; natural Lane C2 sibling |
| F-A5 | R4B `closure_residual_m` is tautological (assignment RHS re-evaluated minus itself); substantive guards are nonnegativity + projection ledger-vs-state pair | misleading naming | confirmed (Ran: verified read) | hardening: rename/document so contract citations point at the real guards |
| F-A6 | FARPOINT01 `watbtm→Dp` double-count class structurally absent; `watbtm`/`watpdg` have no arithmetic consumer beyond the frost residual. FDHP01's documented `Dp += watbtm` identity is unwired | positive verification + watch-item | confirmed (Ran: consumer audit) | record; **re-audit this seam first** if the Dp coupling is ever activated. Outlet-lane-only HBP frost provenance noted as convention |

Clean surfaces: S1 transfer lineage (mechanics, ordering, area scaling),
S2 hourly carry arrays (same-day, zeroed, gated), S6 single-OFE
specialization (natural zero-feed; manifest-guarded).

## Summary

One fidelity-significant confirmed divergence (F-A2, runon
re-infiltration — the sweep's headline, feeding Lane D's contract stage),
one spec-vs-code divergence safely inside a declared hold (F-A1), two
hardening items, one capability gap, one positive verification with a
watch-item. **No conservation defect found** — consistent with the
closure-gate record.
