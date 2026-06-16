# openWEPP Engine Roadmap

Status: living — **canonical**, **forward-only planning queue**
Last updated: 2026-06-16
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

**Current position:** single-OFE water-balance closure, frost
activation/conservation, single-OFE frost-depth heat-flow parity,
MOFE hillslope inter-OFE water-routing closure, the **openWEPP-native
totalwatsed3 CLI + closure** (WSHED01, the WBVAL06/6a deferral, closed
2026-06-14), and the **MOFE >10-OFE far-point demonstration** (FARPOINT01,
closed 2026-06-16 — openWEPP's three identities close at 19 OFEs past the legacy
ceiling; the frost `watbtm` double-count it surfaced was closed contract-first)
are closed. The next active mechanism is **per-OFE runoff magnitude
adjudication**; PERFHO01 (complete 2026-06-16) attributed the ~80–110× high-OFE
wall-clock gap to per-OFE-day runtime-surface map churn (CPU-bound, modestly
superlinear) and recommends the `PERFOPT01` optimization follow-on.
(Completed-rung detail and commits: [work-packages execution log](work-packages/README.md).)

---

## Queue

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| 1 | **Per-OFE runoff magnitude adjudication** | Decide if per-OFE runoff vs legacy (FARPOINT01: openWEPP 71% vs legacy 55.5% of precip on H2637) is expected Stage-2 divergence or a defect | A per-term verdict (expected vs defect-shaped follow-on) | ⏭️ **Next** (`MOFE-MAGPARITY01`) |
| 2 | **Runtime-surface map-churn optimization** | Cut per-OFE-day symbol-keyed `BTreeMap` clone/insert/remove churn + success-path writeback-validation detail (PERFHO01 GDB-sampled these as ~73 % of cost) | Bit-identical outputs + measured speedup; the gap's first necessary optimization (~1.5–2.5× expected, 3.75× Amdahl cap) | ▶️ follow-on (`PERFOPT01`, from PERFHO01) |
| 3 | **MOFE line-count split** | Behavior-preserving split of the 3 files that crossed 2000 lines | Each under 2000 WARN; bit-identical outputs | ▶️ follow-on (`REFACTOR022`) |
| 4 | **Stage-2 physics-magnitude** | Fidelity of deferred magnitudes vs external authority | Magnitude correctness, judged against the closed + routed balance with comparator as flag | ⏸️ **Deferred** |

(MOFE01 + FARPOINT01 closed hillslope water-routing closure through 19 OFEs; the
remaining items are separate follow-on mechanisms.)

---

### 1. Per-OFE runoff magnitude adjudication ⏭️ (next)

FARPOINT01 closed the >10-OFE routing **conservation** but surfaced a
**magnitude** divergence: on H2637 openWEPP routes 71 % of precip to the outlet
vs legacy's 55.5 % (both bounded; legacy with_ui is the q-cap-broken 127.7 %).
Adjudicate whether the per-OFE runoff magnitude is expected Stage-2 divergence or
a defect-shaped follow-on, judged against the already-closed routed balance
(comparator a flag, ADR-0017). Package: `MOFE-MAGPARITY01`.

### 2. Runtime-surface map-churn optimization ▶️ (follow-on, from PERFHO01)

PERFHO01 *(complete 2026-06-16)* characterized the ~80–110× H2637 wall-clock gap:
CPU-bound (`977.99/978.55` user s), **not** I/O or parquet, scaling
roughly linear-to-modestly-superlinear in OFE count (`b≈1.12`) — a large
**constant per-OFE-day cost**, not an explosive exponent. GDB-sampled dominant
cost: per-OFE-day symbol-keyed `BTreeMap<BoundarySymbol, BoundaryValue>`
runtime-surface clone/insert/remove/lookup + success-path writeback validation
(`11/15` samples); the WB13-string lead was tested and found **not** dominant.
Verdict: **not acceptable as-is**. Follow-on `PERFOPT01` cuts the runtime-surface
churn + makes writeback detail lazy (bit-identical, determinism-preserving per
`docs/numerics/`); expected ~1.5–2.5× (3.75× Amdahl cap on the named component) —
the first necessary optimization, not full gap closure. Characterization:
`work-packages/20260616-perf-high-ofe-hillslope-characterization-001/`.

### 4. Stage-2 Physics-Magnitude ⏸️ (deferred, judged last)

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
