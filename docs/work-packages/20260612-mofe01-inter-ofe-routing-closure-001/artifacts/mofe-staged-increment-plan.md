# MOFE Staged Increment Plan — Dispatch Artifact

Status: active (governs all MOFE01 implementation dispatches)
Author: Claude Code, 2026-06-12
Template: FDHP01 `d3-staged-increment-plan.md` (proven over 13 increments;
agent memory `staged-increment-port-template`). Companion scope authority:
`mofe-routing-port-scope.md` (produced by increment M-A).

## Universal rules (every increment)

- **Conservation hard stops** (all three, once defined by the contract in
  M-B): per-element identity, inter-OFE transfer identity, hillslope-total
  identity — at the FDHP01-era noise floor. Any regression stops the
  increment; fix or back out within it.
- **Single-OFE anchor**: the 7 single-OFE arboreal-dendrite hillslopes (and,
  at major boundaries, the 43-prefix algebraic-radium cohort) must stay
  bit-identical/at-noise vs the pre-increment boundary in any increment
  whose scope does not touch single-OFE paths — the deterministic
  bit-identical gate is the cheapest strongest check (FDHP01 increment-A
  lesson).
- **Comparator posture**: legacy is a weak, per-OFE-count-calibrated flag
  (see package.md). Divergence from legacy is adjudicated ONLY through the
  conservation identities. No tuning toward legacy numbers at any count.
- Contract-first within each increment; red tests before production edits;
  commit each completed increment under `executed-hold` until the
  acceptance increment; truthful evidence labels.
- Subagent requirement per package.md §4a block (comparator_suite_runner
  REQUIRED for heavy runs; record evidence if unavailable).
- Diagnostics are env-gated and removed before commit (Da/Df pattern).

## Increment M-A — characterization + routing scope (no production edits)

**Objective:** ground everything in measured reality before any code.

1. **openWEPP current behavior**: run the as-is binary on the full
   arboreal-dendrite cohort. Record per OFE count: does multi-OFE execute;
   what do `UpStrmQ`/`SubRIn`/`QOFE`/per-OFE WAT rows carry; does anything
   already route (the `runon_input` carryover seam in
   `hydrology_phase_runoff_reconciliation.rs:268-326` — what is it now?);
   where does per-element closure break today.
2. **Legacy per-OFE-count closure measurement** (the comparator-trust
   calibration): from the on-disk `H*.wat.dat` outputs, compute legacy's
   own per-element and hillslope-total closure per OFE count (1→5). This
   turns the operator's "legacy degrades with OFE count, typically above
   ~10 OFEs" into a measured curve over the reachable range. Expected
   shape (verify, don't assume): clean at 1-5, which would make legacy a
   usable flag for this package; cross-reference the wepppy MOFE triage
   defect families. **`pw0.slp` (15 OFEs) is the watershed-representative
   profile, not a hillslope run (operator, 2026-06-12) — it is not part of
   the cohort.** The >10-OFE defect demonstration is a named follow-on on
   a high-OFE substrate or the watershed step.
3. **Routing scope artifact** (`mofe-routing-port-scope.md`, the FDHP01
   scope-artifact shape): legacy state-machine map with lifecycle columns —
   the per-plane loop, runoff → run-on hand-off symbols (`runoff`,
   `roffon`, and whatever the source actually uses — **read the lines, do
   not infer from symbol tables**; recorded Dh lesson), run-on infiltration
   coupling on the receiving OFE, lateral/subsurface inter-element terms,
   sediment hand-off (mapped, flagged in/out of scope); openWEPP seam
   mapping (element orchestration, kernel phase order, where run-on enters
   the receiving element's day); state-shape proposal with legacy alias
   table; red-test definitions (per identity, per arm); sizing + increment
   refinement for M-C/M-D.
- Gates: no production edits; evidence artifacts + scope artifact complete
  with file:line citations against the pinned baseline; the legacy
  closure-degradation curve recorded.

## Increment restructure after M-A (2026-06-12)

M-A refuted the shadow-first premise for multi-OFE: current openWEPP
**fail-closes on all 29 multi-OFE surfaces** (28× `HKERNEL-WB14-RUNOFF-E-003`
day 2; H34 `E-001` day 1), so there is no multi-OFE behavior to preserve and
a bit-identical shadow gate has nothing to bite on. The single-OFE anchor
(7/7 passing) carries the preservation gate instead. Increments now follow
the scope artifact's port-scope sections (`mofe-routing-port-scope.md`,
authority for seams/symbols/red tests):

## Increment M-B — hydrology route closure (scope §"M-B hydrology route closure")

- Contract-first within the increment: pin the three conservation
  identities (per-element with run-on/run-off, transfer, hillslope-total)
  and the upstream-carry semantics in the routing contract(s) BEFORE
  production edits; then the scope's red tests (2-OFE surface carry, 2-OFE
  lateral handoff, H11/H6/H9/H1 cohort smoke past day 2); then populate
  `runon_input` from real upstream carry and feed lateral carry to the
  downstream `SubRIn` path per the scope's seam map.
- Gates: red tests green; 36/36 execute (fail-closed boundary retired on
  valid input); the three identities at noise on every executing surface;
  single-OFE anchor bit-identical; full Rust closure loop.

## Increment M-C — per-OFE WAT publication closure (scope §"M-C WAT publication closure")

- Per-OFE publication semantics pinned in contract (no `UpStrmQ = 0` for
  downstream OFEs; no `QOFE = Q` aliasing; one row per OFE per day or an
  explicitly contracted equivalent); handoff-to-printed-precision checks
  against the M-A calibration (legacy-clean at 1–5, so usable here).
- Gates: scope red tests; identities still at noise; single-OFE anchor;
  full loop.

## Increment M-D — erosion `qin`/sediment coupling decision (scope §"M-D erosion qin and sediment coupling")

- Per the scope: implement only if the water seam owns it inseparably;
  otherwise contract-pin the boundary and emit the follow-on. Operator
  visibility on whichever way the evidence lands.

## Increment M-E — ladder acceptance + closure

- Full-cohort acceptance per OFE count (36-run 1–5 ladder); totalwatsed3
  end-to-end audit on routed output (the WBVAL06/6a deferral resolved or
  explicitly re-stated); package closure obligations (ROADMAP item 1,
  README narrative, handoff naming the next mechanism + the named >10-OFE
  far-point follow-on and any erosion-coupling follow-on).

## Dispatch instructions

Each Codex dispatch: *"Execute increment <M-A|M-B|M-C|M-D|M-E> of
`docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001/artifacts/mofe-staged-increment-plan.md`
end-to-end."* Required reading order: this plan; `package.md`;
`mofe-routing-port-scope.md` (once it exists); the FDHP01 staged plan
(failure modes the rules encode). An increment that cannot meet its gates
backs out with evidence recorded — localized to its seam.
