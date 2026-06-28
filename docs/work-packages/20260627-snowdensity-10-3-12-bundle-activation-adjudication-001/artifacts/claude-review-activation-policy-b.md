# Claude Code Review — 10.3.12 Bundle Adjudication, Activation Policy B, and Next Diagnostic

- Author: Claude Code (review + operator-decision record)
- Date: 2026-06-27
- Evidence class: **Static** (read `closeout.md`, `bundle-activation-adjudication.json`,
  `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-069`/`003`, `_calc_layers.c`/`_h2o_compact.c`)
  over **Codex's Ran** coupled direct-production WAT adjudication.
- Status of artifact: findings + ratified operator decisions for Codex to architect
  against. Contract-amendment form, attribution method, package structure, and
  dispositions are **Codex's to design** — this artifact surfaces *what* and *why*,
  not *how*.

## 1. Operator decisions recorded (authority)

These were decided by the operator on 2026-06-27 and supersede the implicit policy
currently baked into `INV-SNOWFREEZE-069`.

1. **Reject zero paired snow-depth failures as an activation criterion.** It is an
   overfitting target: it cannot be reached without fitting to the validation
   fixtures, and part of the residual is structurally irreducible (patchy-meltout is
   a point-vs-areal limit; see §3). Requiring it keeps a demonstrably-worse default
   shipping indefinitely. This is consistent with — not a departure from — the arc's
   standing no-overfit / contract-first discipline (ADR-0017): it operationalizes
   "do not fit to legacy or to fixtures."

2. **Adopt Activation Policy B.** Default-activation of an opt-in bundle is gated on
   **(a)** strictly better than the current default on the gate-eligible snow
   surfaces (already established: `1147 -> 498`), **and** **(b)** proven no-worse
   across the **full model surface** (regression/identity suite, non-snow climates,
   erosion / water-balance, watershed routing) — **not** on perfect snow fidelity.
   **Frost attribution remains a separate bar**, blocked until the snow-control gate
   is good enough to isolate frost, and decoupled from the activation decision.

3. **Direction of travel is libsnobal-ward**, and the density cap should likely
   re-anchor from WEPP Ch.3 §3.7 (`522 kg m^-3`) to the SNOBAL operational
   liquid-compaction maximum (`550 kg m^-3`, `_h2o_compact` `MAX_DENSITY`),
   **sooner rather than later** so subsequent residual attribution runs against the
   final cap. See §4.3 for the gating this still requires.

## 2. Review of 10.3.12 (what stands, what to correct)

**Stands.** The bundle result is solid and well-evidenced: `498` failures
(Δ649 vs default, Δ263 vs holding-only), 0 paired surfaces worse than holding-only,
transitively no-worse than default. The `HOLD-OPT-IN-BUNDLE` disposition was
**contract-faithful** under the *old* criterion (069 mandates HOLD while any paired
failure remains). The v97 amendment is clean: it composes already-ratified
boundaries (067 holding-capacity + 060/062 density) with no new physics/schema/
default, trace proof, comparator ladder, and residual classification.

**Correct.** The closeout attributes non-activation **solely** to the unmet snow
gate (`498/1415`). That is incomplete and should not be left on the record as the
only barrier: activation changes the **global default melt/density physics for every
run**, and the no-worse evidence here is confined to **4 paired snow sites**. Under
Policy B, the binding activation gate is now **(2b) full-surface no-regression**, a
separate body of evidence this package neither produced nor claims. Even with the
snow gate cleared, activation could not proceed without it.

## 3. The residual now — bias removed, two-sided scatter remains

The most important forward finding is slightly buried in the closeout: the residual
has **rebalanced to near-symmetric scatter**:

- `MODELED_OVER_OBSERVED`: **264** | `MODELED_UNDER_OBSERVED`: **234**
- March/April remaining `197`; under-persistence `128`; cap-limited/patchy `49`;
  compaction-only headroom `20`.

The arc has removed the systematic maritime **over-accumulation bias** (`1147 -> 498`);
what remains is close to two-sided scatter — **two opposite-sign defects now
coexist.** Lead hypothesis for the under-persistence tail (`234` / `128` March-April):
it is **bulk-compaction-arm over-densification** — the same mechanism that sank
10.3.11, milder. Rationale: holding-capacity is a melt/liquid lever and cannot
*create* too-shallow packs, so the too-shallow tail most plausibly traces to the
density arm. If so, the bundle's "net no-worse" **masks a mechanism cost** (trading
over-persistence failures for under-persistence ones) — material to whether the
bundle is *safe*, not just net-better. This needs attribution, not assertion.

## 4. Next package: diagnostic (not another physics-lever chase)

Purpose (the *what*; Codex owns the *how*):

### 4.1 Attribute the residual tails
Split the `234` under-persistence rows by cause: **compaction-arm-induced**
over-densification vs **independent** over-ablation / under-accumulation. Make the
compaction-arm hypothesis the lead and try to falsify it (e.g. compare the
under-persistence population under holding-only vs the bundle). Separately confirm
whether the `264` over-persistence tail is the cap-limited mass excess (→ open-surface
ablation, the next *physics* lever) vs remaining compaction headroom.

### 4.2 Produce the Policy-B activation evidence basis
Policy B makes **full-surface no-regression** the activation gate. The diagnostic
package should define and begin to run that comparison (the scope sufficient to
authorize a global default change), so the activation decision has an evidence basis
rather than waiting on a snow gate that will never reach zero.

### 4.3 Re-baseline at the `550 kg m^-3` cap (gated, first)
Per decision (3), fold the cap re-anchor in as a **gated first step** so attribution
in 4.1 runs against the final cap. This is **not** an unguarded edit:

- The cap is **global** (`INV-SNOWFREEZE-003` / `REF-SNOWFREEZE-CH3-n`), so raising it
  changes default/legacy behavior wherever a pack would otherwise clamp at `522` —
  a real default-physics change, **squarely inside the Policy-B full-surface gate**
  (likely small blast radius — only dense ripe packs reach the cap — but verify, do
  not assume).
- It re-anchors the cap **authority** from WEPP Ch.3 §3.7 to SNOBAL operational
  (`_h2o_compact` `MAX_DENSITY = 550`); ADR-0017 permits preferring a better-justified
  authority over legacy WEPP, but the amendment to `INV-SNOWFREEZE-003` /
  `REF-SNOWFREEZE-CH3-n` must cite it.
- A raised **ceiling** only bites packs pinned at the cap — the dense, over-deep ones
  (the cap-limited over-persistence tail), **not** the shallow under-persistence tail
  — so it is inherently more targeted and lower-risk than the 10.3.11 *rate* increase.
  Expect a *partial* reduction of the over-persistence tail (≈5% density headroom),
  not a silver bullet; true mass-excess rows still need ablation.
- Re-verify the **density→holding-capacity coupling** at the new ceiling (denser cap →
  less pore space → slightly less liquid retained) and re-run the coupled WAT gate
  (over-persistence should improve without worsening under-persistence).

### 4.4 Explicitly NOT
Another compaction-*rate* variant (exhausted, 10.3.11); any zero-failure chase; any
fitting to observed depth/density, fixture identity, or residual class.

## 5. Governance actions needed (Codex/operator owns form)

- **Supersede the `INV-SNOWFREEZE-069` activation criterion** with Policy B: decouple
  default-activation from zero paired failures; gate it on strictly-better +
  full-surface no-regression; keep frost attribution separately blocked. Decide
  whether this is a contract amendment or warrants an ADR (it is cross-cutting
  activation governance) — flag, do not pre-decide.
- **Amend `INV-SNOWFREEZE-003` / `REF-SNOWFREEZE-CH3-n`** if the `550` cap is pursued,
  re-anchoring the cap authority to SNOBAL operational with the gating in §4.3.

## 6. Composite conservation (confirm, do not inherit)

The bundle composes two levers that interact **through density** (the compaction arm
changes density; holding capacity = `0.01 × pore space`, which depends on density).
The package leans on each component's individually-proven conservation. Confirm the
**composite** is conservation-clean — the coupling means the interaction should be
re-verified, especially after a cap change.

## 7. Open questions left to Codex

- What full-surface regression scope is *sufficient* to authorize a global default
  change under Policy B (which suites/fixtures/outputs; bit-identity where unchanged
  + named tolerance where changed)?
- Is the residual now at the no-overfitting floor, or is open-surface ablation a
  defensible non-overfitting lever? (Distinguish structural/irreducible residual from
  real remaining defect.)
- Should the under-persistence tail change how the bundle is described (net-better
  with a mechanism cost) even while it stays opt-in?
- ADR vs contract-amendment for Policy B.
