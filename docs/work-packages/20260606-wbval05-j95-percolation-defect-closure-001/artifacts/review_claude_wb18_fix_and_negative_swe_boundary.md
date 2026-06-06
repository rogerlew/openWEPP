# Claude Code Review — WBVAL05 WB18 Percolation Fix and the Negative-SWE Boundary

Reviewer: Claude Code
Date (UTC): 2026-06-06
Evidence mode: **Static** — read the package, the J-95 attribution ledger, the
disposition/validation/handoff artifacts, and verified the working-tree diffs
(`SC-PERC-001.md` v29, `hydrology/03_kernel_support_01_kernel_phases.rs`,
`runner/hillslope/mod.rs`). I did **not** re-run the suite or the four CLI
targets; the 102-test pass and the p7/p11/p18/p20 reruns are Codex's `Ran`
evidence, attributed.

Verdict: **APPROVE.** This is a strong DC-ExecPlan execution and the **first real
contract-first kernel correction under ADR-0018** — diagnosis converted to a
landed, validated fix, with a legitimate boundary HOLD on the part it does not
own. Two strategic flags (F3, F4) matter more than the fix itself, because the
root cause lands in suspended snow territory and likely shares a cause with
WBVAL06.

---

## F1 — The WB18 fix is real, sound, and contract-first (positive)

The J-95 failure decomposed into two mechanisms, and the seven-gate bar was
applied to each — exactly the envelope discipline working:

- **WB18 misattribution (owned, fixed):** pre-fix WB18 recomputed the WB14/WB12
  liquid partition whenever `tillay2_m` existed, and that recomputation
  re-validated a stale `snow.runtime_swe = -0.006171` and surfaced it as
  `HKERNEL-WB11-PERC-E-003`. The fix (`resolve_wb18_same_pass_infiltration_lineage`)
  consumes an already-published finite non-negative `wb12_infiltration` as the
  authoritative ingress and only recomputes when it is absent. I confirmed the
  diff matches `SC-PERC-001` v29, which now makes the published infiltration
  authoritative and explicitly leaves snow-state fail-closed guards to
  `SC-SNOWFREEZE-001`/`SC-RUNOFFPART-001`. This is an architectural correctness
  fix — WB18 stops validating snow state that is not its concern — **not** a
  guard-loosening: the helper adds a non-negative range check, and the invalid
  snow state still fails closed (now at WB14, its correct owner).

Contract-first sequencing is intact (contract amendment + contract-derived test
`wbval05_wb18_percolation_consumes_published_zero_infiltration_without_snow_recompute`
+ kernel edit). The `runner/mod.rs` change is permanent enrichment of the
percolation guard's diagnostic terms — in-envelope ("improve typed fail-closed
evidence") and useful.

## F2 — Honest caveat: the four targets still do not produce WAT

The acceptance criterion "reach WAT publication" is **not** met. p7/p11/p18/p20
still fail closed at J-95 — now with `HKERNEL-WB14-RUNOFF-E-003` instead of the
percolation code. WBVAL05 correctly invokes the fallback clause (the remaining
failure is *correct* typed fail-closed behavior on genuinely invalid state), so
the closure is legitimate. But it should be read plainly: the value delivered is
a real WB18 correctness fix plus **precise localization of the true root cause**,
not closure of the J-95 symptom. The hillslopes are no closer to emitting WAT;
the fail-closed simply moved to the architecturally-correct guard.

## F3 — Strategic flag: the root cause is negative SWE — suspended snow territory, and a bounds-vs-science judgment

The true root cause is `snow.runtime_swe = -0.006171` — a **negative snow-water
equivalent**, which is the negative-melt / snow-mass-balance family that the
entire ADR-0016/0017 arc orbited and that the roadmap + backlog science review
deliberately **suspended**. So the WBVAL05 follow-on lands in the protected snow
domain, and it requires a careful classification the handoff does not yet make:

- Negative SWE is a **hard bounds/conservation violation** (`SWE ≥ 0`), which
  *blocks* water-balance closure and therefore cannot simply be deferred the way
  snow *magnitude* can.
- But the mechanism that drives SWE negative lives in the **snow producer**,
  which is behind the protected boundary.

The follow-on DC-ExecPlan must therefore decide explicitly whether fixing
negative SWE is (a) a bounds/mass-conservation enforcement — a hard-gate fix that
is in-scope — or (b) a snow-physics/magnitude question that routes to the backlog
review. This is exactly the snow-protected-boundary call (ADR-0018 §8), and it is
subtle precisely because negative SWE sits on the seam between a conservation gate
and the suspended science. The follow-on must be authored as a DC-ExecPlan with
the snow protected boundary declared, not opened as an open-ended "trace the snow
producer."

## F4 — Cross-package: WBVAL05's negative SWE and WBVAL06's leak are plausibly one defect

This is the cross-WP observation worth acting on before either follow-on is
authored. WBVAL05's root cause is **mass destroyed from the snowpack** (SWE driven
below zero). WBVAL06's conservation residual is **water vanishing** (`R > 0`,
inputs exceed outputs + ΔstorageΔ), which I and WBVAL03 already localized as a real
internal non-closure whose sign points at snow mass-loss. A snowpack that
overdraws below zero would manifest *both* symptoms. So `WBVAL05`'s negative-SWE
defect and `WBVAL06`'s leak are plausibly the **same snow mass-balance defect**
seen through two guards.

Recommendation: before authoring WBVAL05-follow-on and WBVAL06 as two separate
packages, run a common-cause check (do the J-95 negative-SWE hillslopes and the
high-residual emitters share the snow mass-balance lineage?). If they do, the
grouping rule (ADR-0018 §4 / one authority envelope) argues for **one snow
mass-conservation DC-ExecPlan**, not two — and both hit the same
bounds-vs-suspended-science boundary from F3 together. Splitting a single
mechanism across two packages would reintroduce relay risk.

## F5 — Handoff wording (minor)

The handoff's first item ("open a snow/runoff boundary closure for
`HKERNEL-WB14-RUNOFF-E-003` … trace the snow producer that emitted negative
runtime SWE") is anchored to a named defect (negative SWE at WB14), so it
satisfies the forbidden-relay rule. The "trace the snow producer" clause edges
toward a diagnostic step; keep the follow-on framed as "close the negative-SWE
bounds defect," with tracing as an internal milestone, per F3.

---

## Disposition boundary

Findings + evidence per the review model. F1 confirms a sound, landed fix — no
change requested. F2 is a framing clarification. **F3 and F4 are the load-bearing
items**: the follow-on(s) land in suspended snow territory, must declare the snow
protected boundary, must classify negative-SWE as a bounds gate vs suspended
science, and should be checked for common cause with WBVAL06 before being authored
as separate packages. Those are decisions for the follow-on authoring, not changes
to WBVAL05's closed state.
