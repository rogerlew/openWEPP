# PERFDEEP03 — Independent Review (Claude Code)

Verdict: **NO-GO confirmed — §7 falsification of the *bridge-the-plumbing* approach, NOT of the array-native
thesis.** The lane-owned persistent frame was built correctly (1147.96 s, a big improvement over PERFDEEP02's
2417 s — the ownership fix genuinely worked), but it **still loses to the 669.97 s baseline (1.71×)**. The
root cause is now verified and it is the same across all four production migrations: **none of them rewrote
the kernel physics; they only moved state plumbing.**

Evidence mode: **Static** (verified the code scope + kernel bodies) + **Ran** (none; relied on Codex's runs).

## The verified root cause — bridge, not rewrite

PERFARCH03's 0.96 µs (146×) came from rewriting one runoff branch's **physics** to compute over dense typed
state — *no per-symbol resolution, no `WritebackField` construction*. Every production migration has instead
**bridged the existing symbol-keyed kernels** to a new state representation:

- PERFDEEP03's code scope is **plumbing only** (day_frame, state_access, scheduler, kernel-contract,
  runner). **No `kernel_phases_mod/` body was modified.**
- `hydrology_phase_runoff_reconciliation.rs` still contains **148** per-symbol `require_state_scalar` /
  `WritebackField` / `state_updates.push` calls, and was **last modified by PERFMIG01** (`6b54db2d`) —
  untouched by PERFDEEP02/03.

So the kernels still do all their per-symbol work (resolve symbol → read → compute → build `WritebackField`);
the frame migration changed the read *source* and writeback *destination* and added dense-slot indirection
**on top of** that unchanged machinery. The ~140 µs/branch cost — which *is* the per-symbol machinery — was
never touched. Plumbing added, cost not removed → net negative, every time.

## The progression makes the pattern unmistakable

| Attempt | What it changed | H2637 |
|---|---|---:|
| PERFMIG01 | writeback → dense payload (plumbing) | 669.97 s (+0.5%) |
| PERFMIG02 | reads → dense-first + retire 6 materializations (plumbing) | 672–675 s (flat/neg) |
| PERFDEEP02 | temporary full-registry frame mirror (plumbing) | 2417 s (3.6×) |
| PERFDEEP03 | lane-owned persistent compact frame (plumbing) | 1147.96 s (1.71×) |

Four attempts, one invariant: **the kernel bodies were never rewritten.** Ownership/representation fixes
moved the number (2417 → 1148) but cannot cross zero, because the cost lives *inside* the kernels.

## The lever no one has pulled (spec §4.2)

Spec §4.2 already says it: *"kernels become pure functions over the frame — no `WritebackField`
construction, no symbol resolution."* That is the PERFARCH03 method, and **it is the part every
implementation short-cut.** The win requires replacing the 148 per-symbol calls in each kernel with direct
dense typed computation (`frame.field = physics(frame.other_fields)`). The frame is necessary
infrastructure; the **kernel-physics rewrite is the actual lever** — and it is the ~10.8k-line lift the spec
scoped but no rung attempted.

## Two concerns to surface

1. **Default-path regression (unproven flat, looks real).** Default-disabled measured **697–707 s vs
   669.97 s (+4–6%)**, two runs both elevated (beyond the ~±3 s variance). The always-on dense-first read
   path (symbol → indexed-symbol resolution + slot check, run even when the island is off) likely taxes the
   default path. If kept on `main`, this must be confirmed and the always-on overhead made truly zero-cost
   when disabled — otherwise it is a shipped regression (the PERFMIG02 lesson).
2. **The meta-signal.** This is the second NO-GO and the fourth migration to confirm the same root flaw. The
   remaining lever (kernel-physics rewrite) is real and PERFARCH03-proven, but it is a large, irreversible
   commitment. That decision is the operator's, not something to scaffold past.

## Disposition

NO-GO / §7 falsification is correct — of the bridge approach, not the floor (PERFARCH03 stands). The path is
a **kernel-physics rewrite**: reproduce PERFARCH03 *in production* for the runoff branch first (replace its
148 per-symbol calls with dense compute over the lane-owned frame), measure the real H2637 endpoint, and
only then judge whether the per-phase rewrite converts and is worth scaling to all ~10.8k lines. The
PERFDEEP02/03 frame infrastructure is the substrate that rewrite computes over. **Decision for the operator:
commit to the kernel-physics rewrite (the only proven lever), and what to do with the current gated code
given its apparent default-path tax.**
