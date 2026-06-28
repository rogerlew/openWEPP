# Claude Code Review — 10.3.19 / 10.3.20: ablation class exhausted, pivot to residual decomposition

- Author: Claude Code (review + forward direction)
- Date: 2026-06-28
- Evidence class: **Static** (read the 10.3.20 disposition, gate-results, result md,
  and the model-matrix JSON; confirmed the 10.3.19 baseline from the activation
  commit) over **Codex's Ran** diagnostic.
- Reviews: SNOWDENSITY-10.3.19 (Harder-Pomeroy partition default activation) and
  10.3.20 (sublimation diagnosis / partition+sublimation composition / Stage B unlock).
- Status: findings + forward direction for Codex; disposition and architecture
  remain Codex's.

## Headline

The Harder-Pomeroy partition (10.3.19) is the arc's real cross-climate win — it
lifted the no-env default to **15 robust fails / 179** on the cross-SNOTEL
forcing-robust rubric, now **above legacy (16 / 176)**, resolving the 10.3.18
"bundle below legacy" finding. 10.3.20 then closed the open-surface ablation thread
as a **triple negative**: sublimation, the partition+sublimation composition, and
the SNOBAL two-layer Stage B all fail to beat that default. The SNOBAL/CoE/Anderson
mechanism family is now substantially exhausted; the next move is a **residual
decomposition on the new default + the frost-attribution-threshold decision**, not
another mechanism in that family.

## 10.3.20 result (correct non-promotion)

| Candidate | Robust fail / score | Cells better / worse | Conservation |
|---|---|---|---|
| current default (bundle + partition) | 15 / 179 | — | — |
| partition + Stage A sublimation | 19 / 168 | 1 / 8 | closed |
| Stage B two-layer surface layer | 15 / 178 | 1 / 3 | closed |

Both candidates fail the primary gate and the bidirectional guardrail. The CC0
`libsnobal` port executed cleanly (commit `bf8b41c…`, provenance captured).
Non-promotion is correct under ADR-0028 (the rubric is the gate).

## Diagnostic answers (Handoff 2 questions)

- **Not (just) an implementation problem.** Even the faithful Stage B two-layer
  port from the reference C fails (15/178). The ablation/two-layer *class* does not
  improve this corpus's rubric — the "bad Stage A implementation" hypothesis is
  largely refuted.
- **The density complementarity is refuted at the rubric level.** The composition
  is 19/168 with **8 worse cells**; even if the `+23.6` / `−23.0` density biases
  offset, sublimation degrades 8 other signatures. Net-negative.
- **Sublimation stays physically valid but is not a lever *here*.** ADR-0028 gates
  on rubric improvement; on this corpus's forcing-robust signatures sublimation
  doesn't help — likely because the SNOTEL sites are not the extreme dry/windy
  regime where it dominates, the robust signatures are not sublimation-sensitive,
  and/or the under-persistence guardrail blocks the mass removal. This is the
  discrimination ADR-0028 is designed for, not a refutation of the science.

## Strategic conclusion: the mechanism family is exhausted

Across the arc: melt modernization (rejected), winter-thaw (superseded),
holding-capacity (adopted), bulk compaction (adopted), partition (adopted — the
win), spring densification (rejected), shallow-pack guard (neutral), sublimation
(rejected), two-layer Stage B (rejected). The default (15/179, above legacy) is a
**local optimum for the SNOBAL/CoE/Anderson family** — 10.3.20 confirms nothing
more in it beats the default. Any further lever must be a **new mechanism class**
admitted under ADR-0028, not another variant of the explored family.

## Forward direction (for Codex)

1. **Residual decomposition on the post-partition default.** The 10.3.18
   decomposition was pre-partition; re-run it on the 15/179 default — which
   signatures and which climates still fail, and what mechanism class (if any) each
   points to. The **under-persistence tail** (density-arm mechanism cost, never
   recovered — the shallow-pack guard was neutral) is the prime suspect for the
   binding constraint.
2. **Force the frost-attribution-threshold decision.** The default beats legacy and
   is near the family optimum; per ADR-0028, "good enough to isolate frost" need not
   be zero. If the residual is forcing-limited or irreducible, **proceed to frost**
   rather than grind more snow levers.
3. **New levers = new mechanism classes.** Not-yet-explored candidates: canopy snow
   interception/sublimation (backlog), sub-canopy longwave, wind redistribution.
   Admit only under ADR-0028 (defensible physics + rubric improvement + no overfit +
   conservation).

## Open questions (left to Codex)

- Are the residual 15 fails concentrated in a signature (density? depth-SWE slope?
  timing?) or a climate, or diffuse?
- Is the under-persistence tail still the binding constraint post-partition, and
  does it want a non-mechanism fix (accept it, or the `.run`-file partition disable
  for the humid regime)?
- What residual level constitutes "good enough" to unblock frost attribution —
  defined on the representative cross-SNOTEL instrument.
