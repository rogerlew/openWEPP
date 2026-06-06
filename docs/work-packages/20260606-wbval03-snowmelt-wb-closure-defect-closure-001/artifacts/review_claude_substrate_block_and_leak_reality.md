# Claude Code Review — WBVAL03 Substrate Block and Leak-Reality Confirmation

Reviewer: Claude Code
Date (UTC): 2026-06-06
Evidence mode: **Static** — read the package, the complete-balance-identity audit,
the J-95 attribution ledger, the validation ledger, the disposition, and the
handoff; cross-read WBVAL01/WBVAL02 evidence. I did **not** re-run the binary; the
reruns and the parquet audit are Codex's, attributed.

Verdict: **APPROVE the legitimate HOLD.** WBVAL03 ran its symptom-existence gate
first and produced real progress (the leak is confirmed real), then closed at a
genuine substrate boundary with a defect-shaped handoff. It is a legitimate HOLD,
not a grind-HOLD. One campaign-level finding and one resume-path sharpening
follow.

---

## F1 — The symptom-existence gate worked; the leak is real (positive)

WBVAL03 did exactly what ADR-0018 §8 requires of a diagnostic-first defect: it
ran the complete-identity audit *before* attributing. Using
`R = (P+Irr+UpStrmQ+SubRIn) − (Q+Ep+Es+Er+Dp+latqcc+Tile) − Δ(SoilWaterTotal +
Snow-Water + InterceptionStorage)` over the 12 saved WBVAL01 emitters, it found
`UpStrmQ=SubRIn=Tile=0`, `InterceptionStorage` null, and `SoilWaterTotal` already
equal to `Total-Soil + frozwt`. The residual reproduces exactly.

**This resolves my WBVAL01 review's open question (B1):** the +24–79 mm/yr leak is
**not** a ledger-completeness artifact — the omitted terms do not explain it. The
residual is a real internal non-closure. The symptom-existence gate did its job:
it confirmed there is a defect to attribute rather than a phantom.

## F2 — Campaign-level finding: the rung-1 substrate is now invalidated

This is the most important consequence of the WBVAL02/03 pair. WBVAL02's correct
source-`radly` guard **escalated the failure from 6/22 to 22/22** on this run:
the invalid `radly=486` row is `1990-02-18` (DOY 49), which precedes both J-95
(DOY 95) and any year-2+ WAT publication. So every hillslope on the
`indispensable-presenter` climate file now fails closed at DOY 49, and neither
the J-95 percolation surface nor a regenerated WAT ledger is reachable.

In WBVAL01 the 12 emitters ran only because the old guard caught a
*geometry-dependent hourly* bound; the new *source-daily* guard (more correct)
catches the invalid input for all geometries. The net: **the chosen rung-1
validation substrate has a CLIGEN data-quality defect that makes it unusable past
DOY 49 of year 1.** WBVAL04 is therefore not a side cleanup — it is on the
critical path for *all* of rung-1 on this run (the leak attribution, J-95, and
any later frost validation).

## F3 — Resume-path gap: quarantine alone does not unblock WBVAL03

The WBVAL04 handoff acceptance reads "WBVAL03 target runs can reach the J-95/WAT
surfaces, **or** the upstream climate source is conclusively reclassified and
quarantined." Those two are not equivalent for unblocking WBVAL03. A
quarantined/rejected invalid day still **fails the run closed before DOY 95** —
the substrate stays unrunnable. Per the no-clip rule, WBVAL04 cannot make DOY 49
"pass." So only **regenerating a valid climate** (or selecting a different valid
fixture) restores a runnable substrate for WBVAL03's resume.

Recommend sharpening WBVAL04's acceptance: *to unblock WBVAL03, WBVAL04 must
produce a runnable valid-climate substrate, not merely quarantine the invalid
source.* Otherwise WBVAL04 could close legitimately while WBVAL03 remains
permanently blocked on this run.

## F4 — HOLD legitimacy (the key conformance check): legitimate, not grind

This is the discipline's first hard test, and it passes. The HOLD names a
**boundary** (validation substrate unavailable / blocked by upstream defect), not
a next inspection step; it names the **next defect** (WBVAL04), then the resume of
two *named* defects (`WBVAL03-HKERNEL-WB11-PERC-E-003-J95`,
`WBVAL03-WAT-LEDGER-CONSERVATION-RESIDUAL`); and it explicitly asserts "no
in-envelope, authority-backed, testable, measurable fix was identified and
deferred." The old grind would have spawned "WBVAL03b: inspect the next
percolation variable." It did not. The handoff also correctly instructs the
resume not to re-treat the already-audited identity terms as unexplored
breadcrumbs.

## F5 — The hold could be marginally more productive (mild, optional)

The leak *attribution* — decomposing the residual by day/term/season on the
**saved** WBVAL01 daily WAT parquets — does not strictly require a runnable
substrate, only trace granularity. Doing that pre-work would determine whether the
resumed leak-work even belongs in this envelope or routes to the **snow protected
boundary**: the residual's sign (`R>0`, water vanishing) points at a possible
snow-pack mass-loss, which §8's protected boundary sends to the backlog snow
review rather than an in-package fix. Establishing that now would sharpen WBVAL04
priority and the resume scope. This is optional and is *not* a grind-HOLD
objection — the fix and its validation genuinely require the substrate.

Note: the snow protected boundary was never reached (attribution did not run), so
it was neither tested nor violated.

---

## Disposition boundary

Findings + evidence per the review model. F1 and F4 are confirmations. F2 is a
campaign-level escalation that should reach the roadmap decision (rung-1 is
blocked on a valid substrate). F3 is a concrete sharpening of WBVAL04's
acceptance. F5 is optional pre-work. No change is requested of WBVAL03's closed
state; the HOLD is correct.
