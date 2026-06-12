# D3 Staged Increment Plan — Dispatch Artifact

Status: active (governs all further D3 implementation dispatches)
Author: Claude Code, 2026-06-11
Companion to: `d3-fine-sublayer-port-scope.md` (content authority — state map,
seam mapping, test definitions). This artifact governs **execution shape**:
how the port lands, in what order, behind which gates.

## Why staged increments

Three D3 attempts have failed the same way (scalar: addendum 2e; coarse
front: `efd2152b`; fine-sublayer fused pass: `910ad7f2` +
`d3-fine-sublayer-implementation-attempt-20260611.md`): state model and
physics changed in one motion, conservation validated only at the end. The
fused attempt's own evidence shows the signature — `frozwt/frdp` correlation
back at `0.9987` (mass re-slaved to depth inside the new state), depth still
pinned, D2 regressed `70.27 mm`. The port therefore lands as **three
separately dispatched, separately committed, gated increments**. One
increment per Codex pass. No pass may start increment N+1 before increment N
is committed with its gates green.

## Universal rules (every increment)

- **D2 hard stop:** from Da forward, the accepted gate is the independent
  WAT flux ledger for years 2–6:
  `RM + Irr - Interception - Q - Ep - Es - Er - Dp - latqcc - Tile -
  delta(Total-Soil + frozwt)`. It must remain at the repaired WAT-publication
  numerical texture established by Da/Db (Da max abs
  `1.3813070645629644e-07 mm`; Db max abs
  `1.9976620946327017e-07 mm`; Db p43 year 2
  `-5.3290705182007514e-14 mm`), with any material regression stopping the
  increment. If an implementation restores the former
  `~3e-11 mm` floor, record that improvement; otherwise do not carry a broken
  identity into the next increment.
- **Contract-first within each increment:** amend `SC-SNOWFREEZE-001` for
  the increment's symbols/semantics and author that increment's red tests
  (from scope §5) before production edits.
- Commit each completed increment with package status `executed-hold`
  (D3 open until increment C's acceptance gate). Truthful evidence labels.
- Protected boundaries per `package.md` (no comparator tuning, no kfactor
  change, no activation regression, single-OFE only).
- Subagent requirement (REQUIRED, not optional): each implementation
  dispatch explicitly authorizes subagent spawning/delegation to
  `comparator_suite_runner` for all heavy batch/closure/comparator runs
  (`cargo test --workspace`, clippy/deny loops, 43-prefix cohort runs).
  Do NOT run heavy batch work on the parent model unless the subagent is
  unavailable; record command-level evidence if so
  (`docs/standards/prompt-wording-guidance.md` §4a).

## Increment A — shadow fine-state + `frwatc` handoffs (conservation proof)

**Objective:** land the fine-sublayer state and both handoff directions with
the state **driving nothing**. Existing depth/freeze/thaw physics untouched.

- In scope: `FrostFineLayerState`/`FrostLayerExchangeState` per scope §3;
  `frwatc(1)` ingress at hour-1 per `frostn.for:335-337` (amend the
  `INV-SNOWFREEZE-012` "hourly entry" wording to daily hour-1 ingress with
  this provenance); `frwatc(0)` aggregation at day-end/thaw-complete;
  `yst`/`nwfrzz` persistent surfaces; shadow-state diagnostics.
- Explicitly NOT in scope: any change to how `frdp` advances, how
  conductivity is gated, how `frozwt` is computed for publication, or any
  freeze/thaw arm. Published outputs must be **unchanged**.
- Red tests first: scope test 1 (`..._frwatc_round_trip_conserves_mass`)
  plus a shadow-consistency test asserting the internal-seam identity from
  the fused-attempt disposition:
  `Δ(fine liquid + nwfrzz + slsic) == WB storage delta − external fluxes`.
- Gates: round-trip ≤ 1e-12 m; internal-seam identity holds on a multi-day
  freeze/thaw fixture; full `clim06` suite; full Rust closure loop;
  **cohort outputs bit-identical to the pre-increment baseline** (the
  engine is deterministic and the state drives nothing — any output drift
  is a wiring leak), which subsumes the years 2–6 noise gate.

Execution clarification (2026-06-11): the WAT parquet physical-byte subgate was
not executable against the clean `20a1e91f` pre baseline because that baseline
emitted nondeterministic `ARROW:schema` footers (`43` unique footer hashes for
`43` WAT files). Increment A therefore also fixes WAT footer determinism. The
pre/post behavior-preservation gate is `H.hbp` and `H.loss.json` physical
byte equality plus decoded WAT row/column equality; WAT physical-byte equality
is enforced current-vs-current after the deterministic footer precondition.

## Increment B — depth derived from fine state + freeze arms

**Objective:** invert the authority — depth becomes a *derived* scan of
`fgfrst`/`slfsd` (the `watdst.for:300-511` recomputation), and the freeze
arms (`frzng` + `frznw`) advance the front by consuming energy against
fine-layer water (`lhfh2o · slsw · dz`) through `Σ(dz/k)` resistance,
accumulating `slsic`.

- In scope: `watdst` mode semantics (`fgfzft` 0/1/2, `sltime`); arm
  selection `frzflg` 0/1/2 freeze paths per scope §2; retirement of the
  target-depth projection (`apply_layered_frost_target` lineage) as
  authority; `frost.hourly.frzflg_####` diagnostic.
- Explicitly NOT in scope: top/bottom thaw arms (`mlttp`/`mltbtm`) beyond
  whatever minimal thaw the existing (surviving) day-scale path provides —
  declare in the increment evidence exactly which thaw behavior is
  carry-over vs ported.
- Red tests first: scope tests 2 (dispatch arms — freeze paths), 3
  (front steps by energy and resistance), 4 (`frznw` refreezes `nwfrzz`
  once), 7 (`watdst` mode flags).
- Gates: those tests green; years 2–6 cohort closure at noise; 43/43 clean;
  **directional movement checks** — max depth no longer pinned at the
  profile bound on the majority of prefixes and `frozwt/frdp` correlation
  materially below the 0.9987 scalar signature. Depth/duration *acceptance*
  is NOT this increment's gate; conservation and de-pinning are.

## Increment C — SPLIT (2026-06-11, after the backed-out thaw attempt)

The first increment C pass failed the D2 hard stop catastrophically
(years 2–6 max residual `2.33e15 mm`; `Total-Soil` reaching `1.6e35 mm` by
p1 year 4 — geometric amplification per freeze/thaw cycle) and was backed
out (`d3-increment-c-thaw-arms-20260611.md`). Root cause per the attempt
evidence: melting `slsic` into fine-layer liquid without capacity-aware
redistribution or overflow routes recycles water into a positive feedback.
The attempt's temporary pore-cap was the silent-clamp anti-pattern standing
in for legacy's real constraints. Increment C therefore splits:

## Increment C1a — seam accounting specification (diagnostic; added 2026-06-11)

The first C1 pass also failed the D2 hard stop (years 2–6 max `16628 mm`;
storage exceeding `ProfilePorosityCap` by metres; the follow-up aggregate cap
collapsed overfill but closure still missed by up to `200 mm` —
`d3-increment-c1-capacity-redistribution-20260611.md`). Two consecutive
conservation failures in the same subsystem mean the next dispatch is
**diagnosis, not a third implementation attempt** (per the C1 disposition's
own "map the exact accounting target first").

**Objective:** produce the seam accounting specification that any
redistribution/overflow implementation must satisfy. No production physics
edits; bounded instrumentation only.

- Core question to answer: **liquid-water ownership over the day.** Legacy
  protocol (scope §2/§4): between `frwatc(1)` and `frwatc(0)` the fine state
  owns liquid; `frwatc(0)` *recomputes* coarse `st`/`soilw` wholesale from
  fine state; WB-side mutations enter only as the `st − yst` delta at the
  next ingress. Map openWEPP's actual daily ordering (which kernel phases
  mutate coarse pools before/after the frost hourly loop and its writeback
  in `hydrology_phase_runoff_reconciliation`) against that protocol, and
  identify where the C/C1 attempts mutated both representations in the same
  day.
- Method: re-apply the backed-out C1 redistribution as a **diagnostic-only
  branch/patch** (not landed), instrument a per-day double-entry ledger on
  one failing prefix (p43): WB external fluxes, ingress delta, freeze
  exchange, each redistribution motion, overflow routing, writeback values,
  published totals. Locate the **first non-conserving day** and attribute
  the leak to a specific motion (per the CLAUDE.md primary debugging lane —
  single run, in-process state, no comparator needed).
- Candidate failure classes to discriminate (from the C/C1 evidence):
  (a) dual-ownership desync — fine and coarse liquid both mutated mid-day,
  writeback overwrites or double-counts; (b) seed/writeback asymmetry —
  fine state seeded from coarse pools at a different phase point than the
  writeback targets; (c) overflow routed outside the closure identity
  (`watpdg`/`watbtm` water leaving the books).
- Deliverable: an accounting-specification section appended to the scope
  artifact — the ownership timeline, the exact writeback semantics
  (recompute vs delta), where `watdst` motions must be mirrored, and where
  overflow enters the WAT identity — plus the failing-day ledger as
  evidence. C1b implements **to that spec**.
- Gates: first non-conserving day located and attributed; spec written with
  code-point citations; no production edits landed; tree returns to the B
  boundary.

**Execution result (2026-06-11):** executed-hold diagnostic complete. See
`d3-increment-c1a-seam-accounting-20260611.md`,
`fdhp01_increment_c1a_seam_accounting_summary_20260611.json`, and the C1a
addendum appended to `d3-fine-sublayer-port-scope.md`. The first hard boundary
is day 93/94 on both p43 and p1: the freeze path writes aggregate `frzw` about
`50 m` above `ul`, after shadow `frwatc(1)` residuals reach about `33 m`.
C1b must implement the C1a single-owner, capacity-bound handoff specification
before any cohort acceptance run.

## Increment C1b — capacity constraints + `watdst` redistribution + overflow surfaces

**Objective:** land the water-side infrastructure thaw needs, on the
freeze-only B boundary, before any thaw arm exists.

- Pre-condition: increment C1a committed; this increment implements **to the
  C1a accounting specification** (ownership timeline, writeback semantics,
  overflow identity routing). An implementation choice that contradicts the
  spec requires amending the spec first, with evidence.
- In scope:
  - Legacy capacity bound on ice formation: `frznw.for` limits new ice by
    `ul/dg·slfsd − slsic`; the freeze arms must respect pore capacity as
    **contract-authorized legacy behavior with provenance** — replacing the
    backed-out attempt's ad-hoc cap. A typed fail-closed guard (not a
    silent clamp) for `frzw > ul`-class violations.
  - Saxton–Rawls `watdst` liquid redistribution (modes 0/1/2, `sltime`
    semantics) for unfrozen fine layers under a front.
  - `watpdg`/`watbtm` overflow surfaces (state + publication plumbing),
    even though only freeze-side flows can reach them pre-thaw.
- Red tests first: scope test 7 (`watdst` mode flags); a capacity-invariant
  test (no fine layer's `slsic` exceeds pore capacity under sustained
  freezing; guard trips fail-closed rather than clamping); an overflow
  conservation test (water routed to `watpdg`/`watbtm` stays in the
  identity).
- Starter gate (from the C1a ledger, before the cohort): p43 and p1 pass
  the day-94 capacity boundary with **zero** aggregate `frzw > ul` rows and
  shadow `frwatc` residuals at numerical noise (the C1a baseline is a 33 m
  accumulated ingress residual and 50 m ice overfill by day 93).
- Gates: those tests green; 43/43 clean; years 2–6 closure at noise;
  capacity invariant never violated on the cohort (zero guard trips on
  valid input); freeze-arm directional metrics (de-pinning, decorrelation)
  not regressed vs the B boundary. Bit-identical outputs are NOT expected
  (redistribution is new physics on the freeze path) — declare deltas.
- Watch expectation (not a gate): per the C1a unification — the runaway
  fine ice is what drives the front to the profile bound — bounded ice
  should pull **depth** materially below ~1782 mm already in this
  increment, before any thaw arm. If depth does not move, the energy/
  resistance side of the freeze arms has an independent defect; record it
  rather than tuning.

**C1b outcome (2026-06-11, `1ee1e171`): landed; watch expectation FIRED.**
Conservation is the cleanest of the arc — years 2–6 at `1.53e-12 mm`, and
the long-standing year-7 boundary item is **resolved** (`6.96e-13`, was
`1.27e-7`). Zero capacity-guard trips. But depth did not move: mean max
`1791.97 mm` (marginally deeper than B's `1782.27` — mechanistically
consistent: capacity-bounded layers hold less water, so each mm of advance
costs less latent heat). The C1a unification claim ("runaway ice drives
depth") is therefore **refuted as stated**. Two readings remain:
(1) the freeze-arm energetics carry an independent defect (resistance not
growing with the frozen path, or `qdry` opposing heat too weak); or
(2) depth-near-bound is *expected* without bottom thaw — in legacy the
240–503 mm equilibrium is the freeze/thaw **balance**, with `mltbtm`
(driven by `qdry > 0`, including in winter arm 2) arresting and retreating
the front; absent any thaw arm, a correct freeze arm still deepens
monotonically all winter. **C2 is the discriminating experiment:** if the
thaw arms bring depth into the envelope, reading 2 holds and the port
closes; if depth still rides the bound after `mltbtm`/`mlttp`, reading 1
is established and the freeze-arm energy/resistance becomes a scoped
defect with the C2 evidence as its localization.

**Execution result (2026-06-12):** C1b landed and passed its water-side gates.
See `d3-increment-c1b-capacity-overflow-20260612.md` and
`fdhp01_increment_c1b_execution_summary_20260612.json`. The parent model ran
the comparator work locally without the comparator subagent per user quota
direction. The `43/43` cohort exits clean, years 2-6 additive closure remains
at noise (max abs `1.5347723092418164e-12 mm`), p1/p43 starter traces show
zero `frzw > ul` rows, and `frozwt/frdp` max correlation remains below
Increment B. The depth watch did not improve: mean maximum depth is
`1791.9747961835646 mm`, so C2 inherits both thaw-arm completion and the
freeze-side energy/resistance depth-magnitude investigation.

## Increment C2 — thaw arms + sandwich geometry + D3 acceptance

**Objective:** complete the state machine — `mlttp`/`mltbtm`, sandwich
frost (`fgfrst=2/3`, `tfrdp`/`tthawd`), `fgthwd` thaw-through and early
`frwatc(0)`, `watpdg`/`watbtm` overflow paths — and take the package's D3
acceptance gate.

- Pre-condition: increment C1b committed with gates green. Thaw melt must
  route through the C1b redistribution/overflow paths — never directly
  accumulate into `slsw` beyond capacity (the backed-out attempt's failure
  mode).
- Red tests first: scope tests 5 (bottom thaw), 6 (top thaw + `fgthwd`),
  8 (multi-day additive closure), 9 (cohort gate); plus a multi-cycle
  amplification test (repeated freeze/thaw cycles on one profile leave
  total water non-increasing absent external input — the `1.6e35`
  signature's regression test).
- Gates (the package acceptance, per scope §6 and addendum 3): 43/43
  clean; depth cap-free and inside the heat-flow envelope (FDMC01/legacy
  240–503 mm range as flag, not target); depth correlation rises materially
  from the 0.13 baseline; frozen-duration delta collapses from −518/−428
  toward zero; years 2–6 closure at noise; the year-7 boundary residual
  explained or eliminated; FQ-4 activation non-regressed.
- On pass: FDHP01 disposition to complete, `GAP-SNOWFREEZE-002` closed,
  ROADMAP item 1 removed, README 7f updated, handoff names MOFE.

**Execution result (2026-06-12):** C2 landed as `executed-hold`, not package
closure. The parent model ran local comparisons without the comparator subagent
per user quota direction. The authoritative hourly cohort at
`/tmp/fdhp01_increment_c2_cohort_hourly_fix_20260612T035740Z` ran `43/43`
clean, preserved years 2-6 and year-7 additive closure at the package C1b
ledger noise floor, kept profile-bound pinning removed (`0/43` pinned), and
kept the old scalar `frozwt/frdp` signature rejected (max correlation
`0.9441102161636825`). The D3 acceptance gate still failed: mean maximum depth
`1793.52198510966 mm`, median depth correlation
`-0.16722397856345997`, median frozen-duration residual `111` days, and median
days above `200 mm` `815`. This selects reading 1 above: freeze-side
energy/resistance/front-advance behavior is the remaining scoped defect.

## C2 outcome (2026-06-12, `7c2e6d64`) — discriminator resolved: reading 1

C2 landed the full thaw side (top/bottom arms, sandwich geometry,
thaw-through, overflow routing; `SC-SNOWFREEZE-001` v62) at
`executed-hold`. The fine-sublayer state machine is now **structurally
complete** — A, B, C1a, C1b, C2 all landed — and duration moved decisively
(median frozen-duration residual `+111` days, from −518/−428/+382;
`frozwt/frdp` max correlation down to `0.944`).

Depth did not move (mean max `1793.52 mm`). Per the recorded C1b
discriminator, **reading 1 is established**: the freeze-arm energetics
carry an independent defect. The remaining D3 work is energy/resistance/
front-advance behavior, not thaw or storage plumbing.

Claude independent-audit notes (Ran, duckdb on the C2 cohort):

- Conservation verified independently of the package ledger: p1/p20 worst
  annual additive residual `~2e-13 mm`. **p43 is `1.9e-8 mm`** — above the
  `3e-11` gate; small but real; watch item, plausibly overflow-path
  rounding on the prefix that previously hit the capacity smoke.
- **The C2 closure ledger is tautological and must be repaired before it
  gates anything again:** `flux_balance_mm` equals `storage_delta_mm`
  bit-for-bit on every row, so `residual_mm ≡ 0.0` by construction. The
  ledger must recompute flux and storage from independent WAT columns (the
  C1b-era ledger's `1e-12`-texture values were genuine).

## Increment D — freeze-arm energetics closure (the scoped D3 remainder)

**Da — hourly energy characterization (diagnostic first, per the C1a
pattern):** instrument one freezing season on p1 (env-gated, not landed):
hourly `qhtout`/`qdry`/resistance-sum/latent-consumption series against the
CRM Ch. 3.8 / `frostn.for` expectations. Discriminate the candidate causes
of ~4.3× depth overshoot and `815` vs `185` days-above-200mm:
(a) resistance `Σ(dz/k)` (incl. snow/residue `dmfrsn`) not growing with the
frozen path → `qhtout` stays large at depth; (b) `qdry` opposing heat too
weak → `mltbtm` never arrests the front in winter; (c) latent term too
cheap (dry deep sublayers freeze at near-zero energy cost — check legacy's
handling of low-`slsw` layers); (d) flux→energy unit error (W m⁻² × 3600 s
per hour). Da also repairs the tautological closure ledger and explains or
clears the p43 `1.9e-8` watch item.

**Da execution result (2026-06-12):** Da landed as diagnostic evidence only,
with no production physics changes. The temporary p1 trace was removed before
the production release rebuild and cohort run. Static legacy inspection
anchors the expected feedback to `frzng.for:235-240` and `:287-335`: within
the 3600-second freeze loop, every front advance grows `qoutdm` by the newly
frozen tilled/untilled path and recomputes `qhtout`. The p1 trace isolates the
openWEPP defect to that missing in-hour resistance growth. On year 1 day 1 hour
2, depth advances `0.000397484 -> 1.162927773 m` while resistance remains
`0.000227134 m2 C/W`; projecting the same hour-end frozen path would raise
resistance to `0.572822749 m2 C/W` and reduce `|qhtout|` from `35602.871` to
`14.117 W/m2`. The independent WAT closure ledger is repaired: years 2-6 max
abs residual is `1.3813070645629644e-07 mm`, p43 year 2 is
`-1.912025027195341e-08 mm`, and both are recorded as WAT-publication numerical
texture rather than a storage leak. The fresh Da cohort at
`/tmp/fdhp01_increment_da_cohort_20260612T044217Z` ran `43/43` clean and is
row-identical to C2, so the D3 acceptance failures carry forward unchanged.

**Db — fix to contract:** correct the established term(s) under
`INV-SNOWFREEZE-006` authority; red tests from the CRM equations
(resistance growth with depth; bounded advance on a known profile). Db's
first implementation target is now the legacy `frzng` in-hour
front-advance/resistance loop, not thaw, storage, capacity, publication, or
unit conversion. Then run the full package D3 acceptance gate (depth in the
240-503 mm envelope as flag, correlation up from 0.13, duration residual
collapsing, conservation at noise on an independent ledger) and, on pass, the
closing obligations (ROADMAP item 1, README 7f, handoff naming MOFE).

Two additional Db obligations (Claude review, 2026-06-12):

- **Strengthen scope test 3 to within-hour.** Its existing across-hour form
  was green on C2 while the front advanced metres inside a single hour —
  the red test must assert that one hour of sustained cooling on a
  thin-front profile advances a bounded, resistance-limited number of
  sublayers, with `|qhtout|` decaying as the frozen path grows (the
  y1/d1/h2 `35,603 W/m²` and y5/d83/h2 `1.0e6 W/m²` traces are the
  regression fixtures).
- **Re-pin the closure gate explicitly.** The universal rule still says
  `≤ ~3e-11 mm`, while the repaired independent ledger reads years 2–6 max
  `1.38e-7 mm`, dispositioned as WAT-publication numerical texture. Db must
  either restore the `3e-11`-grade floor or amend the universal-rule gate
  value with that attribution — the rule text and the accepted evidence
  must not silently disagree.

**Db execution result (2026-06-12):** Db landed at `executed-hold`. The
within-hour red test failed on the pre-fix runtime path with a one-hour
thin-front advance of `0.1996 m`, then passed after the freeze loop recomputed
surface resistance and `Qsrf` after each fine-layer front advance. The local
43-prefix cohort at `/tmp/fdhp01_increment_db_cohort_20260612T051524Z` ran
`43/43` clean without the comparator subagent. The independent years 2-6 WAT
ledger remains at WAT-publication numerical texture with max abs residual
`1.9976620946327017e-07 mm`; p1/p20 spot checks remain `~1e-13 mm` and p43
year 2 is `-5.3290705182007514e-14 mm`. Db fixes the stale-resistance depth
runaway: `0/43` prefixes pin at profile depth, all 43 maximum depths fall
inside the legacy `240..503.2 mm` envelope, and mean/median max depths are
`409.16220799389805/407.3294069097544 mm`. D3 acceptance still fails because
depth correlation remains weak (median `-0.05296014769462692`) and frozen
duration under-persists (median open-minus-legacy `-452` days). The next D3
increment must target seasonal freeze/thaw persistence and timing under the
fine-layer state, while preserving the Db/C1b/C2 conservation and capacity
guards.

## Increment Dc — seasonal lower-front heat + thaw-arm dynamics (added 2026-06-12)

Source: Claude code review of the post-Db tree (Static, code- and
legacy-cited; supersedes the diagnostics-only thaw hypothesis). The
remaining D3 failures (median depth correlation `-0.053`, median frozen
duration `-452` days) are attributed to two production defects plus one
acceptance hazard:

**F1 (dominant) — `qdry` is synthetic, not the legacy soil-heat reservoir.**
`coupling.rs:1988-1994`: `lower_front_temp_c = max(7.0 °C,
midpoint(tmax, tmin))` (an air-temperature midpoint with a 7 °C floor —
`FROST_RUNTIME_STABLE_SOIL_TEMP_C`), times fixed
`FROST_RUNTIME_KFUTIL_W_M_K = 2.1` over fixed
`FROST_RUNTIME_UNFROZEN_HEAT_PATH_M = 1.0` → **≥ 14.7 W/m² of bottom melt
every hour of the year** (the constant `qdry = 14.7` in every Da trace
row; ≈ 25 mm/day at `θ_ice ≈ 0.15` — a 400 mm pack gone in ~16 days).
Legacy authority: `tmpbl = YavgT + YampT·exp(−tmpdp/2)·
sin(2π/365·(sdate − YpshfT) − tmpdp/2)` evaluated at `frdp + 1.0 m`
(`frostn.for:384-386`; same form `mltbtm.for:283-284`), **zero-gated**
(`tmpbl ≤ 0 → qdry = 0`, `frostn.for:396-397`), conductivity from the
content-dependent harmonic mean over fine layers (Saxton-style polynomial,
fallback `0.2`, `frostn.for:430-458`). Wave parameters are fitted from the
climate file's 12 monthly mean temperatures in `tmpcft.for:69-100`
(`YavgT` = mean of monthly midpoints, `YampT` = half the monthly range,
`YpshfT` Newton-fitted) — inputs openWEPP already parses.

**F2 — thaw arms carry the Db-class defect, mirrored, plus a wrong
resistance path.** `thaw_fine_bottom` (`coupling.rs:1055`) and
`thaw_fine_top` (`:1120`) spend `flux × 3600` across all sublayers in one
pass — no in-loop resistance growth, no time debit (contrast the landed
`freeze_fine_front_with_resistance_feedback`, `:948`, which steps one
sublayer per iteration). Additionally the top-thaw energy (`:2044-2045`)
uses `frost_surface_heat_path(depth_before.frdp, …)` — the full
frozen-depth path — whereas legacy `mlttp.for:187-247` computes the
surface-to-**thaw-front** resistance through residue and the already-
thawed fine layers, re-estimated as the thaw front advances.

**F3 — acceptance hazard: Db's depth result is partly error cancellation.**
The 14.7 W/m² floor also enters `signed_net_flux` and the arm-1/4 freeze
budget (`:976`, `:995`), damping freezing year-round. Removing it (F1)
will increase net freeze energy, so **depth will likely overshoot the
240–503 mm envelope until F2's faster-correct thaw dynamics land.** F1 and
F2 therefore land in ONE increment with depth/duration/correlation
re-evaluated jointly. Do not protect the current `409 mm` mean by
rejecting or tuning down the `qdry` fix — that would be comparator-match
tuning against a known-wrong term.

In scope:
- Implement the `tmpcft` monthly-mean wave fit (`YavgT`/`YampT`/`YpshfT`)
  from the parsed climate record; replace the `lower_front_temp_c`
  floor/air-midpoint with the damped-wave `tmpbl` at `frdp + 1.0 m`,
  zero-gated; replace the fixed `2.1` with the legacy harmonic-mean
  content-dependent conductivity (fallback `0.2`).
- Rebuild `thaw_fine_top`/`thaw_fine_bottom` on the
  `..._with_resistance_feedback` pattern: one sublayer per iteration,
  re-derive the path resistance (top thaw: surface→thaw-front through
  thawed layers per `mlttp.for:187-247`), debit `remaining_seconds`,
  loop until the hour or the energy is exhausted.
- Retire `FROST_RUNTIME_STABLE_SOIL_TEMP_C` and the air-midpoint coupling
  as production authority (with provenance note).
- Contract: amend `SC-SNOWFREEZE-001` for the `tmpbl` wave (cite
  `tmpcft.for`/`frostn.for`/`mltbtm.for`) and the thaw-path semantics.

Red tests first:
- Winter zero-gating: a mid-winter day (wave phase ≤ 0 at depth) produces
  `qdry = 0` — no bottom melt; the current code's `14.7 W/m²` is the red
  fixture.
- Seasonal wave values: `tmpbl` at known `(sdate, depth)` matches the
  legacy expression for fitted `YavgT`/`YampT`/`YpshfT` on the
  algebraic-radium climate.
- Within-hour thaw (mirror of the Db freeze test): one hour of warm
  forcing on a deep-frost profile thaws a bounded, resistance-limited
  number of sublayers; `|flux|` decays as the thawed path grows.
- Top-thaw path: thaw flux computed through thawed-layer + residue path,
  not the frozen-depth path; sandwich geometry preserved.
- Multi-cycle amplification and C1b capacity/conservation tests stay
  green (no regression of the landed guards).

Gates: red tests green; full Rust closure loop; 43/43 clean; independent
years 2–6 ledger at the accepted WAT-publication texture (`~2e-7` grade or
better — record any improvement); then the **full package D3 acceptance
gate evaluated jointly** (depth envelope as flag, correlation rising
materially from 0.13, duration residual collapsing from −452 toward zero,
FQ-4 activation non-regressed). On pass: FDHP01 disposition to complete,
`GAP-SNOWFREEZE-002` closed, ROADMAP item 1 removed, README 7f updated,
handoff names MOFE. On fail: back out per the increment rules, record
which of F1/F2 survived, and the residual becomes the next scoped
increment — not a tuning pass.

**Dc execution result (2026-06-12):** failed and backed out. The local parent
run honored the no-subagent quota direction and produced a clean `43/43` cohort
at `/tmp/fdhp01_increment_dc_cohort_20260612T062840Z`, but the gates failed
materially: years 2-6 independent additive-storage residual regressed to
`0.2706094484356498 mm` (`p34`, year 2), p43 year 2 regressed to
`-0.24479853886504088 mm`, mean maximum depth jumped to
`1062.5086535449198 mm`, one prefix pinned at the profile bound, `0/43`
prefixes remained inside the `240..503.2 mm` maximum-depth envelope, and frozen
duration over-persisted by median `+751` days. Depth correlation improved to
median `0.6595441080376979`, but that improvement is not acceptable while D2
closure and the depth envelope fail. See
`d3-increment-dc-seasonal-thaw-20260612.md` and the
`fdhp01_increment_dc_*_20260612` reports. At the post-Dc backout boundary,
production returned to Db/`SC-SNOWFREEZE-001` v63 until the split Dc1 pass.

## Dispatch instructions

Each Codex dispatch is: *"Execute increment <A|B|C1a|C1b|C2|Da|Db|Dc1|Dd|De|Df> of
`docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/artifacts/d3-staged-increment-plan.md`
end-to-end."* Required reading order for every increment pass:

1. This artifact (execution rules + the increment's scope and gates).
2. `d3-fine-sublayer-port-scope.md` (state map, seam mapping, tests).
3. `d3-fine-sublayer-implementation-attempt-20260611.md` (the failure
   modes the gates exist to catch).
4. `package.md` (envelope, protected boundaries).

An increment that cannot meet its gates ends exactly like `910ad7f2`:
backed out, evidence recorded, hold preserved — but the failure is then
localized to one increment's seam instead of the whole port.

## Dc outcome (2026-06-12, `c979b990`) + F4 — the snow-insulation attribution

Dc failed its gates and was backed out (D2 regressed to `0.27 mm`; depth
overshot to mean `1063 mm`; duration flipped to `+751` days) — but
correlation jumping to `0.66` confirmed the seasonal-wave mechanism, and the
overshoot prompted the decisive analysis.

**F4 (Claude, Ran — legacy `H1.winter.dat` parse vs openWEPP cohort/trace):**
midwinter p1/H1 comparison: SWE **agrees** (legacy median `64.1 mm` vs
openWEPP `59.4`), but legacy snow depth is `258 mm` at density `250 kg/m³`
while openWEPP carries `156 mm` at implied `~381 kg/m³`. Density feeds the
Sturm conductivity (legacy `~0.088` vs openWEPP `~0.222 W/m/K`), so snow
insulation is `~2.9` vs `~0.7 m²C/W` — **~4× less in openWEPP**. At −10 °C
surface that is `qhtout ≈ 2.6` vs `≈ 9 W/m²`: a ~3.5× freeze overdrive that
integrates to ~1000 mm/winter — quantitatively reproducing the Dc depth
result. The energy-budget mystery is closed: legacy's frost is arrested by
snow insulation; openWEPP's snow **density/settling evolution** is too
dense (Stage-2 lineage, `snowd.for`), even though snow **mass** (SWE)
conserves correctly. Db's depth agreement was the `14.7 W/m²` floor
compensating for the missing insulation.

Consequences for sequencing:

1. **The D3 depth/duration acceptance is coupled to snow density/depth
   fidelity** — a dependency outside this package's envelope (snow
   magnitude is Stage-2-deferred, protected boundary). The frost state
   machine itself (A…C2, Db) is structurally sound and conserving.
2. The coupling must be **proven, then escalated** — not absorbed by
   tuning frost terms to compensate (that would be the same class of error
   as the `14.7` floor).

## Increment Dc1 — thaw/seasonal accounting repair (bounded)

Fix the `~0.27 mm` years 2–6 additive-storage leak the Dc thaw/seasonal
coupling introduced (per the Dc disposition: do not proceed to any timing
work while that residual exists). Land the seasonal `tmpbl` wave and the
within-hour thaw feedback **behind the conservation gate**: red tests for
the leak, D2 ledger at the Db floor (`~2e-7` texture), the p35 fine-theta
boundary canonicalization handled as a typed, contract-noted bound (not a
silent clamp). Depth/duration metrics are recorded but NOT gated in Dc1
(they are known-coupled to F4).

**Dc1 execution result (2026-06-12):** Dc1 landed at `executed-hold`. The
local parent-run `43/43` cohort at
`/tmp/fdhp01_increment_dc1_cohort_20260612T101238Z` restores years 2-6
independent `Total-Soil + frozwt` closure to WAT-publication texture
(`6.471338602487275e-07 mm` max abs; p43 year 2
`-1.1013412404281553e-13 mm`). Depth/duration remains red and recorded only:
mean max depth `1146.5109665924424 mm`, `1/43` profile pins, median
correlation `0.6415921721982907`, and median frozen-duration residual
`+567` days. Proceed to Increment Dd/F4; do not claim D3 closure from Dc1.

## Increment Dd — legacy-snow-forced frost certification (diagnostic)

The clean discriminator for F4, and the certification gate for this
package's frost physics independent of snow fidelity:

- Method: diagnostic-only harness (env-gated, not landed) that **forces
  openWEPP's frost inputs with legacy's snow depth/density series** (parsed
  from `H*.winter.dat`, on-disk for all 43 prefixes) while leaving all
  openWEPP frost physics (Db + Dc1) live.
- Expected outcome if F4 is the whole story: depth falls into the
  240–503 mm envelope, duration residual collapses, correlation holds at
  the Dc-demonstrated `~0.66+` level.
- On that outcome: **the D3 frost port is certified** — FDHP01 can close
  with an explicit declared boundary: frost physics complete and correct;
  residual depth/duration divergence attributed with evidence to the snow
  density/settling model; a defect-shaped handoff names the snow
  density/depth-split item (NOT melt/partition magnitude — SWE conserves)
  for promotion from the Stage-2 backlog, mirroring how FDMC01 promoted
  frost depth when it became load-bearing.
- If the forced run does NOT close the gap: the residual is frost-side
  after all; it becomes the next scoped increment with the forced-run
  evidence as localization.
- Promotion of the snow density/settling slice is an operator/roadmap
  decision (science-steered), not an autonomous rescope of the Stage-2
  protected boundary.

**Dd execution result (2026-06-12): executed-hold diagnostic complete; F4 is
not the whole remaining D3 story.** Codex generated legacy `H*.winter.dat` for
all 43 algebraic-radium prefixes, parsed hour-24 `snodpt`/`densgt` into
`/tmp/fdhp01_increment_dd_legacy_winter_20260612Tdd/legacy_snow_forcing_daily_hour24.csv`,
ran an env-gated forced-snow openWEPP cohort at
`/tmp/fdhp01_increment_dd_forced_snow_cohort_20260612T121500Z`, then removed
the hook and rebuilt production. Execution and D2 conservation stayed clean:
`43/43` exits, `43/43` WAT outputs, years 2-6 independent
`Total-Soil + frozwt` max abs residual
`6.726058817130287e-07 mm`, p43 year 2
`-1.2079226507921703e-13 mm`. Forced snow removed profile pinning and improved
timing (`0/43` pinned; median correlation `0.7118806632341061`), but it did
not close D3: mean max depth `856.817674502367 mm`, range
`654.0796339074789..1427.3939006063285 mm`, `0/43` prefixes inside the
`240..503.2 mm` legacy envelope, and median frozen-duration residual `+502`
days. The next scoped increment is therefore frost-side under the Dd
controlled-snow setup: localize the remaining hourly flux/front divergence
with in-process evidence, not snow-density tuning.

## Dd outcome (2026-06-12, `bc47e426`) + F5 — the residual is `qdry` conductivity

Dd did NOT certify F4 as the whole story: under forced legacy snow, depth
improved `1146 → 857 mm` mean max but stayed outside the envelope (0/43),
duration `+567 → +502`. F4 (snow insulation) is real but partial (~25% of
the gap). Claude trajectory + code analysis of the residual:

- **Trajectory discriminator (Ran):** the divergence is at **onset** —
  openWEPP carries `199 mm` of frost by mid-December of winter 1 while
  legacy `H1.winter.dat` shows `0`; legacy peaks that winter at `50 mm`
  vs openWEPP-forced `621 mm`. Early-season divergence happens *before*
  deep snow exists, so insulation cannot explain it.
- **Dead-end disposed (Static):** legacy `qwet` (migration-water heat,
  eqn 3.8.4 middle term, `frzng.for:381-437`) is **dead code in the pinned
  baseline** — `saxfun` returns negative potentials (error fallback
  `−150 m`) and the active `frzftp = 0.0` (the `−100 m` value is commented
  out) makes the activation condition always false. Do not port it as a
  live term; record it as a legacy-disabled mechanism (consistent with the
  known "routines disabled to work around bugs" pattern).
- **F5 (the established residual, Static):** openWEPP's
  `lower_front_heat_w_m2` (`coupling.rs:1070-1082`) uses
  `FROST_RUNTIME_UNFROZEN_CONDUCTIVITY_FALLBACK_W_M_K = 0.2`
  unconditionally. Legacy computes the **content-dependent harmonic-mean
  conductivity** over the metre below the front (`frostn.for:430-458`):
  `k(θ,ρ) = (0.5096 + 7.4493·θ − 8.7484·θ²)·(0.0014139·ρ_bulk − 1.0588)·ksoilf`,
  giving ≈ `1.0–1.5 W/m/K` for moist soil — `0.2` is only the dry
  fallback. openWEPP's `qdry` is therefore **5–7× under-powered**, which
  explains all three residuals simultaneously: autumn `tmpbl ≈ 10 °C`
  should yield `qdry ≈ 12 W/m²` (suppressing frost onset until January, as
  legacy shows) vs openWEPP's ~2 (frost onset in November); weak midwinter
  opposition → ~2× depth; weak spring bottom-melt energy (`mltbtm`
  consumes `qdry`) → `+502` days over-persistence. One term, three
  symptoms.

## Increment De — content-dependent `qdry` conductivity

**Objective:** implement legacy's harmonic-mean unfrozen conductivity for
the lower-front heat path (`frostn.for:430-458` lineage): per-fine-layer
`k(θ,ρ)` polynomial over the metre below the front, harmonic-mean
aggregation, `0.2` retained only as the genuine dry fallback; the same
conductivity feeds the `mltbtm` bottom-thaw energy.

- Red tests: (1) conductivity polynomial fixtures (moist soil ≈ 1.0–1.5,
  dry → 0.2 fallback); (2) **autumn onset suppression** — with the fitted
  wave and moist profile, November freeze days produce near-zero net front
  advance (the H1 trajectory `0 mm` at j350 is the flag fixture);
  (3) spring bottom-melt strengthens (duration decreases on a fixture
  winter). Conservation and capacity guards stay green.
- Gates: red tests green; full Rust closure loop; 43/43 clean; D2 ledger
  at the accepted texture **with the creep watch** (Db `2.0e-7` →
  Dc1 `6.5e-7` → Dd `6.7e-7`; another material growth triggers
  investigation, not acceptance); then **two cohort evaluations**:
  - **forced legacy snow** (Dd harness): this is the frost-physics
    certification — depth into the 240–503 mm envelope, duration residual
    collapsing, correlation holding ≥ the Dc `0.66` level;
  - **native snow**: recorded for the F4 disposition — the gap between
    native and forced runs is the measured cost of the snow
    density/settling defect, which prices the Stage-2 promotion decision
    (operator call).
- If the forced-snow run certifies: FDHP01 closes at the declared boundary
  (frost physics complete; native-snow residual attributed to F4 with
  paired evidence) with the defect-shaped handoff naming the snow
  density/depth-split item. If it does not: the next term comes from the
  same paired-trajectory method that found F5.

**De execution result (2026-06-12):** landed at `executed-hold`. Codex ran the
cohorts locally without the comparator subagent per user quota direction.
`SC-SNOWFREEZE-001` v65 and the De tests land the legacy content-dependent
`Qdry` conductivity and preserve D2 at the accepted WAT-publication texture.
The final native production cohort at
`/tmp/fdhp01_increment_de_native_cohort_final_20260612T171358Z` ran `43/43`
clean, with years 2-6 additive closure max abs
`5.474257917248426e-07 mm`, mean max depth `705.505148615878 mm`, and median
duration residual `+288` days. The corrected forced legacy-snow cohort at
`/tmp/fdhp01_increment_de_forced_snow_cohort_20260612T171017Z_proper` ran
`43/43` clean, with years 2-6 closure max abs
`4.355148297552347e-07 mm`, mean max depth `655.9890274782282 mm`, median
depth correlation `0.770042438411068`, and median duration residual `+186`
days. De improves the controlled residual but does not certify D3: `0/43`
prefixes enter the legacy `240..503.2 mm` maximum-depth envelope. The next
increment remains frost-side under the De forced-snow setup and must locate the
first remaining hourly front/flux divergence after content-dependent `Qdry`.

## De outcome (2026-06-12, `4d4b56cf`) — landed; residual ~1.6× on depth

De landed (not backed out): SC-SNOWFREEZE-001 v65, content-dependent
harmonic Qdry conductivity (frostn.for:430-458 authority), bulk-density
runtime surface, bottom-thaw reuse. The staged loop is now monotonically
converging on the forced-snow cohort: mean max depth 857 → 656 mm,
median correlation 0.66 → 0.770, duration residual +502 → +186 days,
closure texture improved (6.5e-7 → 4.4e-7). Native cohort: 705.5 mm /
+288 days. Still 0/43 inside the 240–503.2 mm envelope: a ~1.6× depth
residual remains after F1/F2/F4/F5.

## Increment Df — paired hourly front/flux localization (diagnostic)

The Da single-sided trace found F5's predecessor; Df pairs it against
legacy directly. Legacy `H*.winter.dat` is **hourly** (snowfall, rain,
ground, falling, melt, snow depth, snow density, frost depth, thaw depth,
frost, residue — on disk for all 43 prefixes), so the paired series costs
nothing on the legacy side.

- Method: env-gated openWEPP hourly trace (Da pattern, not landed) on
  2–3 prefixes under the **De + forced-legacy-snow** setup; align hour-by-
  hour against `H*.winter.dat`; locate the divergent hours and attribute
  each to a term. Candidate discriminations, in likelihood order:
  (a) **hourly surface-temperature synthesis** — openWEPP's tmax/tmin →
  hourly series vs legacy `winthd` (freezing-degree-hours delta integrates
  directly into depth; the winter.dat `ground` column may expose legacy's
  effective surface/interface temperature — verify its meaning against
  `winthd.for`/`frostn.for` first);
  (b) positive-surface-temperature capping under snow (legacy caps
  `surtmp` at 0 under snow on the freeze path — affects both freeze
  energy and top-thaw availability);
  (c) `kftill` tilled-path conductivity (openWEPP constant vs legacy
  computed) and residue path;
  (d) freeze-arm net-flux composition (arm ≠ 2 net of qdry) and arm
  selection frequency vs legacy `frzflg` occupancy.
- Deliverable: per-term attribution with the measured share of the
  remaining ~1.6× depth gap; the dominant term becomes increment Dg's
  bounded fix (same shape as F5 → De).
- Gates: no production edits; trace removed before commit; evidence
  artifacts + plan update.

**Df execution result (2026-06-12):** executed at `executed-hold`; no
production edits remain. Codex ran the paired p1/p2 hourly localization locally
without the comparator subagent per user quota direction. The temporary trace
root was `/tmp/fdhp01_increment_df_trace2_20260612T175406Z`; compact evidence
is in `d3-increment-df-paired-hourly-localization-20260612.md`,
`fdhp01_increment_df_localization_summary_20260612.json`,
`fdhp01_increment_df_term_attribution_20260612.csv`, and
`fdhp01_increment_df_paired_hourly_excerpt_20260612.csv`.

The first material divergence occurs on both prefixes at year 1 day 1 hour 2,
with no snow on either side: legacy frost depth is `5.0 mm`, while openWEPP is
`42.057866709 mm` on p1 and `41.417581693 mm` on p2. The winter `ground`
column is ground-drift snow, not temperature. The localized seam is the
surface resistance path: openWEPP feeds `residue_depth_m = 0.0` into frost
while legacy uses `23.0 mm` residue depth, and openWEPP also omits the legacy
`dpfsfl` shallow-front minimum top-frozen conduction distance. The conservative
legacy resistance estimate at the first divergence is roughly `899x` (p1) and
`951x` (p2) the openWEPP resistance. Freeze-arm energy is surface-flux
dominated (`|surface flux| / (|surface| + |lower front|) > 0.99995`), so
`Qdry`, snow forcing, positive-temperature snow capping, and publication are
not the first-order residual.

## Increment Dg — residue path + shallow-front minimum resistance

**Objective:** port the legacy frost surface-resistance terms identified by
Df without reopening D2 storage: (1) publish/propagate the residue depth
consumed by winter/frost (`resdep` lineage) into
`frost.runtime_residue_depth_m`; (2) apply legacy `frostn.for` shallow-front
minimum top-frozen conduction distance (`dpfsfl`, midpoint of the first fine
layer) whenever the active frost surface heat path is thinner than that
minimum.

- Authority: legacy `winter.for` writes `resdep(iplane)` into `H*.winter.dat`;
  legacy `frostn.for` includes residue resistance before frozen-path
  resistance and floors the frozen conduction distance to `dpfsfl` for
  below-freezing surface conditions.
- Red tests: p1/p2 Df fixtures should fail before Dg by showing zero open
  residue depth and shallow-front resistance below the legacy floor; pass after
  Dg with positive residue resistance and bounded first-day advance. Include a
  unit test for the `dpfsfl` minimum independent of residue.
- Gates: full Rust closure loop if production code changes; De forced-snow
  cohort must stay `43/43`, years 2-6 additive closure at WAT-publication
  texture, and the forced-snow D3 maximum-depth envelope must move materially
  toward `240..503.2 mm` without duration regression. Native cohort remains
  recorded for the snow handoff cost.
