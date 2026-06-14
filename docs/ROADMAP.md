# openWEPP Engine Roadmap

Status: living — **canonical**, **forward-only planning queue**
Last updated: 2026-06-14
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
activation/conservation, single-OFE frost-depth heat-flow parity, and
MOFE hillslope inter-OFE water-routing closure are closed. The next active
mechanism is **watershed routed outputs / totalwatsed3 audit**, followed by
Stage-2 physics-magnitude review.
(Completed-rung detail and commits: [work-packages execution log](work-packages/README.md).)

---

## Queue

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| 1 | **Watershed routed outputs / totalwatsed3 audit** | Consume the closed MOFE hillslope pass outputs through the watershed output stack | End-to-end `totalwatsed3` water-balance audit on routed openWEPP output; handle the arboreal-dendrite no-impoundment `pw0.imp` state explicitly | ⏭️ **Next** |
| 2 | **MOFE >10-OFE far-point demonstration** | Run MOFE routing on a >10-OFE substrate where legacy's WB defect appears | openWEPP three-identity closure holds at >10 OFEs (exceed the legacy ceiling) | ▶️ follow-on (`MOFE-FARPOINT01`) |
| 3 | **Per-OFE runoff magnitude adjudication** | Decide if the ±10–25% per-OFE runoff vs legacy is expected Stage-2 divergence or a defect | A per-term verdict (expected vs defect-shaped follow-on) | ▶️ follow-on (`MOFE-MAGPARITY01`) |
| 4 | **MOFE line-count split** | Behavior-preserving split of the 3 files that crossed 2000 lines | Each under 2000 WARN; bit-identical outputs | ▶️ follow-on (`REFACTOR022`) |
| 5 | **Stage-2 physics-magnitude** | Fidelity of deferred magnitudes vs external authority | Magnitude correctness, judged against the closed + routed balance with comparator as flag | ⏸️ **Deferred** |

(MOFE01 is done-done for hillslope water-routing closure after M-I; the
remaining MOFE-adjacent items below are separate follow-on mechanisms.)

---

### 1. Watershed Routed Outputs / totalwatsed3 Audit ⏭️ (next)

MOFE01 closed hillslope-internal inter-OFE water routing on the
`arboreal-dendrite` 36-run 1-5-OFE ladder. M-I added the independent in-runner
hillslope-total identity, closing at `3.306423012547295e-13 mm` against the
`1e-9 mm` tolerance, with every multi-OFE case nonzero-at-noise. The next
mechanism is the watershed output seam that consumes those closed hillslope
pass outputs and produces the end-to-end `totalwatsed3` audit surface deferred
since WBVAL06/6a.

The immediate blocker is not the MOFE conservation identity. M-H attempted
`openwepp-cli-watershed` with the fresh H1-H36 pass files and failed closed
before output writing on the substrate's no-impoundment `pw0.imp` state:
`CLIWAT-E-010` / `IMP-E-004`, `jpond=0`. This queue item owns modeling or
accepting that no-impoundment state explicitly, then producing
`totalwatsed3.parquet` and running the water-balance audit on routed openWEPP
output.

Work package: `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/` (active 2026-06-13; W-A executed, W-B next). W-A confirmed `jpond=0` is a valid no-impoundment state and the current `IMP-E-004`/`CLIWAT-E-010` rejection is a parser defect that blocks `chan.inp`, HBP parsing, dispatch, and output writing.

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
