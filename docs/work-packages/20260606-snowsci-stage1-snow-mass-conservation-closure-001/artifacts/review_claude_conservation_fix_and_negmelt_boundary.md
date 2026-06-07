# Claude Code Review — SNOWSCI Stage 1 Conservation Fix and the Negmelt Boundary

Reviewer: Claude Code
Date (UTC): 2026-06-06
Evidence mode: **Static** — read the package, the single-source design, the
validation ledger, the disposition, the kernel diff
(`03_kernel_support_00_support_helpers.rs`), both contract diffs
(`SC-SNOWFREEZE-001` v52, `SC-WATBAL-001` v145), the hourly snow loop (~4000–4060),
and the legacy `snowd.for` melt branch. I did **not** run any command; the CLI
reruns, the p7 trace, and the 22-runfile sweep are Codex's `Ran` evidence,
attributed. The changes are **uncommitted** (status `executed-local-review`), so
this is the pre-commit gate.

Verdict: **Qualified — the conservation fix is real and good, but it must not be
committed until three things are done** (F3 full-blast-radius validation, F4 the
WBVAL06 common-cause measurement, F2 honest accounting of the negmelt-boundary
supersession). The kernel change itself is sound; the gaps are about blast radius
and a quietly-resolved Stage-2 question.

---

## F1 — A real non-conservation is fixed, by construction (positive)

I verified the bug and the fix against the code, not just the design. Old
`redistribute_daily_signed_snowmelt` returned `routed_melt_total_m = net =
positive + negative` (= `positive − |neg|`) but `snowpack_state_loss_m =
positive − negative` (= `positive + |neg|`). Those differ by `2·|neg|`: the pack
was debited `positive + |neg|` while only `positive − |neg|` was routed — so
`2·|neg|` of water was removed from the snowpack and **routed nowhere**. On the J-95
example (`positive=0.007376`, `neg=−0.006171`) that over-debit drove SWE to
`−0.006171`. The fix sets `routed = state_loss = positive_melt_total`.

This is conserving **by construction**, not a clamp, and I confirmed it is
consistent with the legacy-faithful depth store: the hourly loop reduces depth only
by positive melt (`smelt` is computed only when `wmelt > 0`, `snodep = snodpt −
smelt`, lines ~4006/4022), exactly as legacy `snowd.for` (`snodep = snodpt −
smelt`). So `positive_melt_total` *is* the authoritative pack loss, and making SWE
and routed melt equal to it makes SWE track the depth store. The p7 trace showing
closure error `0.0` on the repaired day corroborates. Importantly, **the
`2·|neg|`-vanishing mechanism is a textbook `R>0` water-vanishing source — it is a
strong candidate for the WBVAL06 leak itself** (see F4).

## F2 — The fix quietly resolved a Stage-2 negmelt question (the boundary finding)

The kernel diff alone reads as pure accounting, but the contract diff shows more:
this **supersedes `INV-SNOWFREEZE-019`**, whose prior text was the
HPHYS0284/0285/0303 *corrected negative-melt carry-state* interpretation citing
`REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX`. The prior interpretation gave
negative melt a **pack-state role** (routed = net, plus a companion
`snodpt = snodpt + ngtvML*1000/densgt` depth adjustment). The new rule makes
negative raw melt **diagnostic-only** — no SWE debit, no routing, no pack effect —
and the amendment says so explicitly: "supersedes the separate openWEPP SWE-debit
interpretation."

That is a substantive position on negmelt *semantics*, which is the heart of the
Stage-2 physics review and the ADR-0016/0017 negative-melt lineage. Two readings:

- **In-envelope (defensible):** the old code was simply non-conserving and
  inconsistent with the legacy depth store, so the only conserving, depth-faithful
  answer is `state_loss = routed = positive`, and negative melt's "no pack effect"
  *follows from* the legacy positive-only depth update rather than being a new
  physics choice.
- **Boundary-crossing (the risk):** it superseded the contract that ported the
  wepp-forest negmelt fix, i.e., it provisionally decided that negative melt has no
  physical pack/routing role — a decision the Stage-1 protected boundary says to
  *escalate to Stage 2*, not settle in-package.

My assessment: the conservation fix is correct, and the depth-store-consistency
argument makes it the right *Stage-1* move — but the package's "no
physics-magnitude change" claim is too clean. Routed melt magnitude **did** change
(`net → positive`) on every mixed-signed-melt day, and the **negmelt semantics**
changed by superseding the negmelt-fix invariant. Required: state plainly that
SNOWSCI-S1 supersedes the wepp-forest-negmelt-fix interpretation of `INV-019` with
a conservation-first treatment, and **route to Stage 2 the ratification of whether
negative melt truly has no physical pack/routing role** (vs the negmelt-fix
intent). This keeps the protected boundary honest rather than silently moved.

## F3 — Validation blast radius is far too narrow (must fix before commit)

`redistribute_daily_signed_snowmelt` runs on **every day with mixed positive/
negative hourly melt, on every hillslope** — and the change alters routed melt
(`net → positive`) on all of them. Validation was: 4 J-95 hillslopes publish, one
p7 trace, and "22 runfiles produce WAT." The disposition discloses that
`cargo test --workspace` and `cargo deny check` were **not** run, and there is no
H1..H39 semantic rerun. For a kernel change with population-wide reach — and
especially a negmelt-semantics change (F2) — that is insufficient. Before commit:
run `cargo test --workspace`, `cargo deny check`, and the H1..H39 semantic suite,
and confirm no regression outside the four target hillslopes. (Note: `cargo test
--workspace` was already red earlier on an ADR0017 decisions-README assertion from
the doc work — that needs resolving too so this gate is meaningful.)

## F4 — The common-cause measurement was skipped, though the data was in hand

The whole reason WBVAL05's negative-SWE follow-on and WBVAL06 were grouped
(ADR-0018 §4, Milestone 1) was to confirm common cause. F1 shows the fixed bug is a
prime `R>0` leak candidate — so the fix likely collapsed a large part of the
WBVAL06 residual. The package had the 22 post-fix WAT outputs and **did not
recompute the WBVAL06 complete-identity residual** to measure the before/after.
That single number tells us whether rung-1 WB closure is now nearly done or barely
moved, and whether WBVAL06 is even still a separate defect. Instead it deferred,
noting "the WAT residual formula needs a term/unit audit" — which is itself a tell
that part of the residual may be a ledger/measurement issue (my WBVAL01 B1), making
the measurement *more* worth doing, not less. Before declaring WBVAL06 a separate
open package: recompute the residual on pre- and post-fix outputs and report the
collapse.

## F5 — Independent review/verification not performed (disclosed)

The disposition states truly independent dual review/verification needs user
authorization to spawn sub-agents, so the `review_agent_*`/`verification_agent_*`
artifacts are not independent. This review partially fills the gap on the science
and the diff, but the convention's independent gate is unmet; the cargo suites
(F3) are the minimum objective backstop.

---

## Recommendation (pre-commit)

The kernel fix is sound and should land — but not as-is. Before commit:

1. **F3:** run `cargo test --workspace` (resolve the pre-existing red README
   assertion so the gate is real), `cargo deny check`, and the H1..H39 semantic
   suite; confirm no out-of-target regression from the routed-melt change.
2. **F4:** recompute the WBVAL06 complete-identity residual before/after and report
   how much the conservation fix closed — this is the common-cause confirmation the
   grouping was for.
3. **F2:** state in the package that `INV-SNOWFREEZE-019`'s wepp-forest-negmelt-fix
   interpretation was superseded by a conservation-first treatment, and route the
   physical ratification of "negative melt has no pack/routing role" to Stage 2.

F1 is a genuine win; the conservation defect — and very likely the WBVAL06 leak —
is real and fixed. These actions are about proving the blast radius is safe and
keeping the Stage-1/Stage-2 boundary honest, not about reworking the fix.
