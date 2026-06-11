# openWEPP Engine Roadmap

Status: living — **canonical**, **forward-only planning queue**
Last updated: 2026-06-11
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
activation/conservation gate are closed; the next active mechanism is **frost-depth
heat-flow parity on single-OFE** (completing the vertical frost mechanism before
routing), then **MOFE routing**.
(Completed-rung detail and commits: [work-packages execution log](work-packages/README.md).)

---

## Queue

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| 1 | **Frost-depth heat-flow parity (single-OFE)** | Replace the freeze-index depth proxy with the legacy-lineage energy-balance heat-flow model, on single-OFE | Frost depth/duration matches `INV-SNOWFREEZE-006`/`-012` heat-flow (comparator as flag); conservation still closes | ⏭️ **Next** |
| 2 | **MOFE inter-OFE routing** | Run-on/run-off routing across OFEs on a vertically-closed, frost-settled per-element balance | **Routing closure** (conservation across elements) on the 17-OFE `pw0` surface + watershed outputs | ▶️ After item 1 |
| 3 | **Stage-2 physics-magnitude** | Fidelity of deferred magnitudes vs external authority | Magnitude correctness, judged against the closed + routed balance with comparator as flag | ⏸️ **Deferred** |

---

### 1. Frost-depth heat-flow parity (single-OFE) ⏭️ (next)

Frost is a per-column **vertical** mechanism, and the ladder settles vertical mechanisms
on single-OFE **before** routing so their error is not aliased into routing error. FQ-4
settled frost *activation* that way; this item finishes the job by settling the frost
*depth model* on single-OFE before MOFE. Replace the freeze-index proxy
(`frdp = 0.20·clamp(−mean_temp/6)`, capped 0.20 m) with the energy-balance heat-flow
model the contract already mandates (`INV-SNOWFREEZE-006`/`-012`, legacy `frostn`
lineage, CRM Ch. 3.8 / Dun et al. 2010), closing `GAP-SNOWFREEZE-002`.

Why before MOFE (the re-sequence, 2026-06-07): FDMC01 sized the proxy as **materially
off** (depth capped 200 mm vs legacy 240–503; depth-series correlation 0.13; frozen
duration +258 days from the ratchet). Building MOFE on the proxy and fixing depth later
means re-validating MOFE under frost; doing it now means MOFE is built once on a faithful
frost foundation, and the heat-flow physics is debugged in isolation (one column, no
routing). This completes the vertical frost mechanism, not a magnitude footnote.

In scope: standard `ksflag` frost depth model on the frost-active single-OFE substrate
`/wc1/runs/al/algebraic-radium` (`ksflag=1`). Out of scope: kfactor conductivity
magnitude (legacy-faithful), forest `ksatadj`, frost activation (closed), MOFE/17-OFE
(item 2), snow magnitude (item 3). Conservation must still close (`frozwt` in storage).
For frost-active WAT audits, `SC-WATBAL-001` v151 defines that storage term as
`Total-Soil + frozwt`, with `SoilWaterTotal = Total-Soil` as the unfrozen
`watcon` alias, and binds WAT `frozwt` to
`frost.runtime_frwatc_frozen_water_after_m`. FDHP01 Addendum 2d showed this
source binding is still behaviorally neutral because that diagnostic aliases
the depth-derived store; the next pass must implement the true exchanged frozen
store behind the diagnostic before D3 depth evidence is trusted.

DC-ExecPlan: `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/`.
Sized by [FDMC01](work-packages/20260608-fdmc01-frost-depth-comparator-characterization-001/);
authority [backlog/20260607-frost-depth-model-heat-flow-parity.md](backlog/20260607-frost-depth-model-heat-flow-parity.md).

### 2. MOFE inter-OFE routing ▶️ (after item 1)

Layer run-on/run-off routing onto a per-element balance that is already vertically
closed and frost-settled (depth model at parity, not just the gate). Target is
**routing closure** (conservation across elements), not magnitude: snow magnitude
remains deferred (item 3) and is judged after this closes. The active work-package
handoff must name MOFE as its next item for this to bind.

Surface: `docs/work-packages/20260502_mofe_flagged_hillslope_triage`.

### 3. Stage-2 physics-magnitude ⏸️ (deferred — judged last)

Fidelity questions deferred by the closure-not-magnitude principle until the structure is
closed and routed so the comparator can attribute error cleanly. They do **not** block
items 1–2.

| Item | What | Provenance | Backlog |
|---|---|---|---|
| Snow magnitude | `snowd.for` melt/settling/density/partition equation fidelity (CRM Ch. 3.7) | legacy physics adjudication | [backlog/20260605-snow-code-deferred-science-review.md](backlog/20260605-snow-code-deferred-science-review.md) (Stage 2) |

(Frost depth was a Stage-2 candidate; the FDMC01 verdict + the settle-vertical-before-routing
principle promoted it to active queue item 1 on 2026-06-07.)

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
