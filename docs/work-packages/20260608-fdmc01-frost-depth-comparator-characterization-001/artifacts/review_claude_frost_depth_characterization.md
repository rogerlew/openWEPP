# Claude Code Review — FDMC01 Frost Depth Model Comparator Characterization

Reviewer: Claude Code
Date (UTC): 2026-06-07
Evidence mode: **Static** — read the ledger, verdict, summary JSON, and confirmed the
boundary via `git status`. The 43-prefix legacy + openWEPP runs and the metrics CSVs are
Codex's `Ran` evidence, attributed.

Verdict: **Approve.** A well-scoped, boundary-respecting characterization that sizes the
frost-depth gap robustly. The "materially off → heat-flow parity" verdict is
well-supported, the comparator-as-sizing-not-target posture is stated, and the roadmap
staging (full runoff quantification deferred post-MOFE) is respected.

---

## F1 — Boundary held; the gap is robustly sized (primary)

`git status` confirms the package touched only `package.md` (status → complete) and new
`artifacts/` — **no production, contract, or production-test change**, exactly as a
characterization package requires.

The "materially off" verdict does not rest on a single fragile metric:

- **Depth ceiling, robust:** openWEPP is hard-capped at **200 mm for all 43 prefixes**;
  legacy spans 240–503 mm (mean 414). 43/43 legacy prefixes exceed the cap; the cap
  binds on 36,421 prefix-days. This is robust independent of any reconstruction because
  `WB14_FROST_MAX_DEPTH_M = 0.20` is a hard constant — the proxy *cannot* represent the
  legacy depth range.
- **Shape, not just peak:** median correlation **0.133**, mean MAE 124 mm, RMSE 146 mm —
  the proxy does not track the legacy depth series, it isn't merely scaled down.
- **Duration, from actual output:** `frozwt>0` is read from the real openWEPP WAT
  (`/tmp/fq4_population`), not reconstructed — openWEPP 1017 vs legacy 759 days, **+258
  days mean**.

Like-for-like is clean (a recurring trap on this harness): both depths are physical
frost depth in mm (`frdp*1000`), both durations are `frozwt>0`, both onset/thaw are
first/last nonzero day. No dimensional or surface mismatch inflates the gap.

## F2 — The headline is a *dual-direction* error: shallower **and** stickier

This is the substantive finding and worth foregrounding over the single "−214 mm"
number. The proxy is wrong in two independent ways:

1. **Too shallow** — capped at 200 mm vs legacy up to 503 mm.
2. **Too persistent** — +258 frozen-days (≈ +37 days/yr), i.e. ~34% longer frozen than
   legacy.

The persistence error matters *more* than the depth error for the water balance, because
(per FQ-4) the conductivity bite is near-total whenever frost exists — so frost
*duration*, not depth, sets how long infiltration is shut off. The proxy keeps the
frozen runoff-generation window open ~34% longer than legacy. Net effect on frozen-season
runoff is therefore likely an **over-production**, not the under-production a "shallower
frost" framing alone would suggest. The verdict captures both; the ledger's materiality
note correctly bounds the full quantification to post-MOFE.

## F3 — The persistence error has a named mechanism (input for the parity DC)

Worth recording for whoever implements parity: the proxy is "sticky" by construction.
`frdp = max(prior_frdp, 0.20·freeze_index)` ratchets up and only enters the thaw branch
when `freeze_active` is false (`tmin > 0`). So on cold-but-warming days (sub-freezing
`tmin`, rising mean temp) the proxy **cannot retreat** — it holds frost that legacy's
energy balance would be melting from below/above. The summary's `onset/thaw edge
delta = 0` with `+258` in-window days is exactly this: the season's first and last frozen
days align, but the proxy stays continuously frozen in between where legacy
thaws/refreezes. A heat-flow model (`frostn`/`frzng`/`frznw`) fixes precisely this — it
is the concrete behavioral target, not just "deeper frost."

## F4 — Evidence caveat (disclosed, minor): depth is reconstructed, not read

The depth comparison uses an **offline reconstruction** of the proxy from `p*.cli`
climate (ledger step 3), because openWEPP does not publish `frost.runtime_frdp_m` to the
WAT surface — only `frozwt` is published (and duration *is* read from actual output). The
ledger discloses this honestly. The conclusion is unaffected (the cap is a hard constant;
duration uses actual output), so this is not a defect in the characterization. But it is a
gap to close in the eventual parity DC: **publish `frdp` to a WAT/output surface** so
future frost-depth comparisons read actual runtime state rather than a re-derivation.

## F5 — Sequencing: sizing gate complete; parity stays Stage-2, behind MOFE

The verdict correctly (a) selects backlog target (1) heat-flow parity over (2)
sanctioning the proxy — at correlation 0.13 with the cap binding everywhere, sanctioning
would bless a model that doesn't track legacy — and (b) defers full runoff quantification
post-MOFE. Two things follow:

- The **backlog item's sizing gate is now complete** with a clear verdict; update its
  state to record "materially off → target (1) selected, pending Stage-2 scheduling."
- This does **not** reorder the queue. MOFE (rung-3) remains next, and the MOFE substrate
  (`arboreal-dendrite`) runs `ksflag` **off**, so the proxy doesn't even touch rung-3 —
  which *reinforces* safe deferral. I'd **not** scaffold the parity DC-ExecPlan yet
  (Codex offered to): a parked execution package ahead of its rung is exactly the kind of
  premature scaffolding the forward-only roadmap discipline avoids. The backlog item +
  this verdict are the correct "captured and ready" state until Stage-2 opens.

---

## Recommendation

Approve. The sizing gate is met: the freeze-index proxy is materially off against legacy
heat-flow depth — shallower (capped, uncorrelated) and stickier (+258 frozen-days) — and
the right promotion target is a heat-flow parity DC under `SC-SNOWFREEZE-001`, not a
contract blessing of the proxy. Fold the verdict into the backlog item (sizing gate
complete), and keep frost-depth parity in Stage-2 behind MOFE. Two carry-ins for the
eventual parity DC: target the *duration/persistence* error (the ratchet) as much as the
depth cap, and publish `frdp` so future comparisons read actual output.
