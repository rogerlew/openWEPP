# openWEPP Engine Roadmap

Status: living — **canonical**
Last updated: 2026-06-07
Audience: all contributors
Owner: maintainers (Claude Code maintains this document)

This is the single authoritative roadmap for the openWEPP simulation engine. Other
locations (the `docs/work-packages/README.md` execution log, ADRs, backlog, agent
memory) **point here**; when they disagree, this document wins. See
[§ Keeping this current](#keeping-this-current) for the update protocol.

---

## Strategy

openWEPP is the Rust successor to legacy WEPP, built **architecture-first** with
**top-down science contracts** as the correctness authority
([ADR-0011](decisions/0011-architecture-first-top-down-science-contracts.md)).
Legacy WEPP is treated as a **flag, not a target**: a divergence from a pinned
legacy binary triggers investigation, never defines acceptance
([ADR-0017](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)).
Defects are closed with **Defect-Closure ExecPlans** — diagnose-and-correct in one
pass, or stop at a declared boundary
([ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md),
[defect_closure_execplans.md](defect_closure_execplans.md)).

### The governing acceptance principle: closure, not magnitude, not comparator-match

Each rung's acceptance target is **closure** — does the system *conserve* (water
balance, mass balance, bounds) — **not** whether the forcing magnitude is physically
perfect, and **not** whether it matches a legacy binary. This split is load-bearing:

- **Conservation** is independent of forcing magnitude, so the structure can be
  closed and verified while individual magnitudes are still being corrected.
- **Magnitude / physics fidelity** is judged **last**, against an already-closed and
  routed system, with the comparator as a sanity flag — so magnitude error is never
  aliased with structural (routing/closure) error.

Every rung therefore adds **one mechanism** on an already-closed foundation.
Boundaries are **closure gates, not calendar phases**: if work on one rung lands its
first failure inside the next mechanism, the rungs merge — that is expected, not a
detour.

---

## The rung ladder

| Rung | Mechanism | Target | Status |
|---|---|---|---|
| **1** | Single-OFE vertical water balance | Conservation closure | ✅ **Complete** |
| **2** | Frost (ksflag frozen-soil gate) | Activation + closure under frost | ✅ **Complete** (activation + conservation) |
| **3** | MOFE inter-OFE routing | Run-on/run-off routing closure | ⏭️ **Next** |
| **S2** | Physics-magnitude tier | Fidelity vs external authority | ⏸️ **Deferred** (judged last) |

**Current position (2026-06-07):** rung-2 is closed for activation and
conservation; **rung-3 (MOFE) is the next active rung.** Two physics-magnitude
items are explicitly deferred to the Stage-2 tier (below).

### Why this order

Frost is a **per-column vertical** mechanism coupled to the single-OFE
over-drainage/percolation path, so it must be settled on clean single-OFE geometry
**before** routing is layered on — doing MOFE first would alias frost-gate error into
routing error. MOFE = single-OFE + routing; you cannot debug routing closure over a
leaking per-element balance. Snow and frost *magnitude* couple into runoff that MOFE
routes, but per the closure-not-magnitude principle, MOFE is validated for **routing
closure** on still-imperfect magnitudes, and magnitude is judged last.

---

## Rung detail

### Rung 1 — single-OFE water-balance conservation ✅

Closed via two contract-first kernel corrections plus a producer-side audit surface:

- **SNOWSCI-S1** — snow mass conservation (the negative-melt double-debit that
  destroyed water); closed ~72% of the rung-1 leak.
- **WBVAL06** — interception published as a first-class WAT flux (`Interception`),
  closing the remaining ~28% (it was a publication-completeness gap, not a leak).
- **totalwatsed3 companion** (wepppy) — interception added as a first-class outflow
  so the audit closes on openWEPP output.

Result: single-OFE WAT conserves to ~1e-6–1e-11 mm/yr (years 2..6). Caveat: year-1
initial storage is an explicit exclusion; full end-to-end totalwatsed3 awaits
watershed (MOFE) outputs.

### Rung 2 — frost (ksflag frozen-soil gate) ✅ (activation + conservation)

Validated on `/wc1/runs/al/algebraic-radium` (PRESTON MN, gridmet daily, all
`lanuse=1` → `ksflag=1`). The substrate was first cleared of confounders, then the
frost gate itself was fixed:

- **FQ-1** — soil corrected-layer coverage (`HS-RUNTIME-E-062`); 6→42/43 runnable.
- **FQ3-DC-RUNOFFPART** — runoff partition (storage-limited infiltration cap); `Q`
  engages and conserves.
- **FQ3-DC-ET-CORN** — annual-crop ET/canopy engagement (PL activation sentinel +
  Julian-day scheduler); `Ep`/`Interception` engage and conserve.
- **FQ-4** — **frost activation**: a `frost_file_present` provenance flag was wrongly
  gating frost off when no `frost.txt` sidecar existed (the opposite of legacy, and
  the same defect class as snow's `INV-SNOWFREEZE-009`). Fixed contract-first;
  43/43 prefixes now activate frozen soil; closure holds under frost (~3.22e-11 mm).
  FQ-2's broken frost-closure ledger was folded in and corrected.

Execution detail and per-package status: [work-packages/README.md](work-packages/README.md).

### Rung 3 — MOFE inter-OFE routing ⏭️ (next)

Run-on/run-off routing layered on a per-element balance that is already vertically
closed and frost-gated. Target: **routing closure** (conservation across elements),
on the 17-OFE `pw0` surface and watershed outputs. Magnitude (snow, frost depth)
remains deferred and is judged after this closes. Surface:
`docs/work-packages/20260502_mofe_flagged_hillslope_triage`.

---

## Stage-2 physics-magnitude tier ⏸️ (deferred — judged last)

These are **fidelity** questions, deferred by the closure-not-magnitude principle
until the structure is closed and routed so the comparator can attribute error
cleanly. They do **not** block any earlier rung.

| Item | What | Provenance | Backlog |
|---|---|---|---|
| **Snow magnitude** | `snowd.for` melt/settling/density/partition equation fidelity (CRM Ch. 3.7) | legacy physics adjudication | [backlog/20260605-snow-code-deferred-science-review.md](backlog/20260605-snow-code-deferred-science-review.md) (Stage 2) |
| **Frost depth model** | openWEPP uses a freeze-index proxy (`frdp = 0.20·clamp(−mean_temp/6)`, capped 0.20 m); legacy `frostn.for` uses a layered energy-balance **heat-flow** model (Dun-2008 fine sublayers, `frdp ≤ 1.0 m`) per `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-006`/`-012`, `GAP-SNOWFREEZE-002` | **openWEPP-introduced** simplification; activation + conservation are correct, but depth fidelity is openWEPP's own | _pending — promote a defect-shaped backlog entry_ |

Note on the frost depth item: the **kfactor conductivity magnitude is
legacy-faithful** (openWEPP uses the documented WEPP defaults; annual crops correctly
get the near-impermeable "concrete frost" coefficient). The deferred gap is the
**depth model** only — when frost forms, how deep, and the 0.20 m cap — which governs
frost timing/extent. A comparator characterization (`wepp_260606_hill` frost
depth/duration vs the proxy) should size this gap before a parity package is scoped.

---

## Keeping this current

This document only works if it stays true. The protocol:

1. **This file is canonical.** The `work-packages/README.md` roadmap block, ADRs,
   backlog, and agent memory reference it; they do not redefine the rung ladder.
2. **Update on every rung transition.** When a rung closes or the active rung
   changes, update the ladder table, the "Current position" line, the rung detail,
   and `Last updated`. This is part of the closing package's documentation, not a
   separate chore.
3. **A strategic decision is not bound until it lands here and in the active
   handoff.** A roadmap living only in an ADR or memory has no hook into the
   execution chain — to redirect work you must change both this file *and* what the
   active work-package handoff names as its next item. (Lesson from the
   HPHYS0314–0320 arc, where the snow-comparator relay continued for seven packages
   after the strategy had already changed elsewhere.)
4. **Deferred ≠ forgotten.** Stage-2 items carry a backlog pointer; promote a
   defect-shaped backlog entry when an item is deferred, so it is tracked and ready.

## Authority pointers

- [ADR-0011](decisions/0011-architecture-first-top-down-science-contracts.md) — architecture-first, top-down contracts, comparator-tier policy
- [ADR-0017](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) — comparator is a flag, not a target
- [ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md) — Defect-Closure ExecPlan conversion rule
- [defect_closure_execplans.md](defect_closure_execplans.md) — DC-ExecPlan authoring
- [specifications/science-contracts/README.md](specifications/science-contracts/README.md) — `SC-*` contract authority
- [work-packages/README.md](work-packages/README.md) — per-package execution log
