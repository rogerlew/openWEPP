# openWEPP Engine Roadmap

Status: living — **canonical**, **forward-only planning queue**
Last updated: 2026-06-07
Audience: all contributors
Owner: maintainers (Claude Code maintains this document)

This is the single authoritative roadmap for the openWEPP simulation engine — a
**forward-looking planning queue only.** Completed work is **not** recorded here; it
lives in the [work-packages execution log](work-packages/README.md). When a rung
closes, it is **removed** from this queue and its detail moves to that log. Other
locations (ADRs, backlog, agent memory) point here; when they disagree, this document
wins. See [§ Keeping this current](#keeping-this-current).

---

## How to read this queue

openWEPP is built **architecture-first** with **top-down science contracts** as the
correctness authority
([ADR-0011](decisions/0011-architecture-first-top-down-science-contracts.md)). Legacy
WEPP is a **flag, not a target**
([ADR-0017](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)).
Defects are closed with **Defect-Closure ExecPlans**
([ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md),
[defect_closure_execplans.md](defect_closure_execplans.md)).

**The ordering principle — closure, not magnitude, not comparator-match.** Each item's
acceptance target is **closure** (does it *conserve* — water/mass balance, bounds),
**not** whether forcing magnitude is physically perfect and **not** whether it matches
a legacy binary. Conservation is independent of magnitude, so the structure is closed
first; **magnitude/physics fidelity is judged last**, against an already-closed and
routed system, so magnitude error is never aliased with structural error. Every item
adds **one mechanism** on an already-closed foundation. Boundaries are **closure gates,
not calendar phases**.

**Current position:** single-OFE water-balance closure and the frost
activation/conservation gate are closed; the next active mechanism is **MOFE routing**.
(Completed-rung detail and commits: [work-packages execution log](work-packages/README.md).)

---

## Queue

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| 1 | **MOFE inter-OFE routing** | Run-on/run-off routing across OFEs on a vertically-closed, frost-gated per-element balance | **Routing closure** (conservation across elements) on the 17-OFE `pw0` surface + watershed outputs | ⏭️ **Next** |
| 2 | **Stage-2 physics-magnitude** | Fidelity of deferred magnitudes vs external authority | Magnitude correctness, judged against the closed + routed balance with comparator as flag | ⏸️ **Deferred** |

---

### 1. MOFE inter-OFE routing ⏭️ (next)

Layer run-on/run-off routing onto a per-element balance that is already vertically
closed and frost-gated. MOFE = single-OFE + routing — frost (a per-column vertical
mechanism) was settled first so its error is not aliased into routing error. Target is
**routing closure** (conservation across elements), not magnitude: snow and frost-depth
magnitude remain deferred (item 2) and are judged after this closes. The active
work-package handoff must name MOFE as its next item for this to bind.

Surface: `docs/work-packages/20260502_mofe_flagged_hillslope_triage`.

### 2. Stage-2 physics-magnitude ⏸️ (deferred — judged last)

Fidelity questions, deferred by the closure-not-magnitude principle until the structure
is closed and routed so the comparator can attribute error cleanly. They do **not**
block item 1. Promote a defect-shaped backlog entry per item when deferring, so each is
tracked and ready.

| Item | What | Provenance | Backlog |
|---|---|---|---|
| Snow magnitude | `snowd.for` melt/settling/density/partition equation fidelity (CRM Ch. 3.7) | legacy physics adjudication | [backlog/20260605-snow-code-deferred-science-review.md](backlog/20260605-snow-code-deferred-science-review.md) (Stage 2) |
| Frost depth model | openWEPP uses a freeze-index proxy (`frdp = 0.20·clamp(−mean_temp/6)`, capped 0.20 m); legacy `frostn.for` uses a layered energy-balance **heat-flow** model (Dun-2008 fine sublayers, `frdp ≤ 1.0 m`) per `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-006`/`-012`, `GAP-SNOWFREEZE-002` | **openWEPP-introduced** simplification; activation + conservation are correct, depth *fidelity* is openWEPP's own | [backlog/20260607-frost-depth-model-heat-flow-parity.md](backlog/20260607-frost-depth-model-heat-flow-parity.md) |

Frost depth note: the **kfactor conductivity magnitude is legacy-faithful** (openWEPP
uses the documented WEPP defaults; annual crops get the near-impermeable "concrete
frost" coefficient). The deferred gap is the **depth model** only — when frost forms,
how deep, the 0.20 m cap — which governs frost timing/extent. The **sizing gate is
complete** ([FDMC01 characterization](work-packages/20260608-fdmc01-frost-depth-comparator-characterization-001/)):
verdict **materially off** — openWEPP capped at 200 mm vs legacy 240–503 mm (depth-series
correlation 0.13) and frozen-duration +258 days (the proxy ratchets and over-persists) →
target = heat-flow parity (not contract-sanction the proxy). **Execution still deferred
to Stage-2 (post-MOFE)** per the staging principle; MOFE runs `ksflag` off so the proxy
does not touch rung-3.

---

## Keeping this current

This is a forward-only queue; it only works if it stays true.

1. **This file is canonical, and forward-only.** It contains what is *next* and
   *deferred* — never completed work. The [work-packages execution log](work-packages/README.md),
   ADRs, backlog, and agent memory reference it; they do not redefine the queue.
2. **When an item closes, remove it from the queue.** Move its detail/commits to the
   execution log and update the "Current position" line and `Last updated`. This is
   part of the closing package's documentation, not a separate chore. Do not let
   completed items accumulate here.
3. **A strategic decision is not bound until it lands here and in the active handoff.**
   A roadmap living only in an ADR or memory has no hook into the execution chain — to
   redirect work you must change both this file *and* what the active work-package
   handoff names as its next item. (Lesson from the HPHYS0314–0320 arc, where the
   snow-comparator relay continued for seven packages after the strategy had already
   changed elsewhere.)
4. **Deferred ≠ forgotten.** Stage-2 items carry a backlog pointer; promote a
   defect-shaped backlog entry when an item is deferred.

## Authority pointers

- [ADR-0011](decisions/0011-architecture-first-top-down-science-contracts.md) — architecture-first, top-down contracts, comparator-tier policy
- [ADR-0017](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) — comparator is a flag, not a target
- [ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md) — Defect-Closure ExecPlan conversion rule
- [defect_closure_execplans.md](defect_closure_execplans.md) — DC-ExecPlan authoring
- [specifications/science-contracts/README.md](specifications/science-contracts/README.md) — `SC-*` contract authority
- [work-packages/README.md](work-packages/README.md) — completed-work execution log
