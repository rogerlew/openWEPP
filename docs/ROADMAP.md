# openWEPP Engine Roadmap

Status: living — **canonical**, **forward-only planning queue**
Last updated: 2026-06-12
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
activation/conservation, and single-OFE frost-depth heat-flow parity are closed.
The next active mechanism is **MOFE routing**, followed by Stage-2
physics-magnitude review.
(Completed-rung detail and commits: [work-packages execution log](work-packages/README.md).)

---

## Queue

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| 1 | **MOFE inter-OFE routing** | Run-on/run-off routing across OFEs on a vertically-closed, frost-settled per-element balance | **Routing closure** (conservation across elements) on the `arboreal-dendrite` graded 1–5-OFE ladder; watershed outputs follow | ⏭️ **Next** |
| 2 | **Stage-2 physics-magnitude** | Fidelity of deferred magnitudes vs external authority | Magnitude correctness, judged against the closed + routed balance with comparator as flag | ⏸️ **Deferred** |

---

### 1. MOFE inter-OFE routing ⏭️ (next)

Layer run-on/run-off routing onto a per-element balance that is already vertically
closed and frost-settled (depth model at parity, not just the gate). Target is
**routing closure** (conservation across elements), not magnitude: snow magnitude
remains deferred (item 2) and is judged after this closes. The active work-package
handoff must name MOFE as its next item for this to bind.

Development substrate (operator decision 2026-06-12):
`/wc1/runs/ar/arboreal-dendrite/wepp` — a graded OFE ladder (7×1, 5×2, 5×3,
3×4, 16×5 OFE hillslopes; one 15-OFE hillslope excluded/observe-only), with
36 legacy outputs on disk. The ladder allows routing closure to be validated
per OFE count.

**Comparator posture for this rung (stronger than the default ADR-0017
flag):** legacy WEPP has *known water-balance defects that grow with OFE
count* (operator knowledge; corroborated by the legacy-replay MOFE
closure-audit triage at wepppy
`docs/work-packages/20260502_mofe_flagged_hillslope_triage` — a defect-family
taxonomy of legacy's own flagged hillslopes). Legacy is therefore a weak flag
at low OFE counts and progressively untrustworthy as OFE count rises — in
exactly the dimension this rung builds. Acceptance authority is openWEPP's
own inter-OFE conservation closure; part of the rung's characterization is
*measuring* legacy's per-OFE-count closure defect from the on-disk outputs so
comparator trust is calibrated with evidence rather than assumed.

Work package: `docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001/`.

### 2. Stage-2 Physics-Magnitude ⏸️ (deferred, judged last)

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
