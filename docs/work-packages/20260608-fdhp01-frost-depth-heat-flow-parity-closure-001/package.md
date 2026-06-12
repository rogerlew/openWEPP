# FDHP01 — Frost Depth Heat-Flow Parity (single-OFE)

Status: executed-hold

Closure status update, 2026-06-11: FDHP01 remains `executed-hold`, but the
post-review D2 blocker is closed. The continuation implemented a per-layer
frozen-depth/frozen-water state behind the `frwatc` diagnostics instead of the
former scalar `frdp * (theta_fc - theta_dr)` store. A fresh release
`openwepp-cli-hill` run of the `algebraic-radium` `p1..p43` frost-on
population into `/tmp/fdhp01_layered_store_20260611T080722Z` produced `43/43`
clean exits, including the previously failing `p2`; the v152
`Total-Soil + frozwt` annual identity closed to numerical noise
(`1.2683574368566042e-07 mm` max abs residual), and `frozwt` no longer has an
exact scalar ratio to `frdp`.

Remaining hold, 2026-06-11: the depth/duration physics gate still fails. The
layered store stops the D2 publication fiction, but it is not the full
Dun-2008/legacy layered heat-flow port. Cohort maximum frost depth remains near
the profile bound (`1782.0379909380451 mm` mean openWEPP max versus
`414.22093023255815 mm` matched legacy mean; median depth correlation
`-0.27756218032931956`; open frozen-day count is `518.5348837209302` days
lower than legacy on average). The next executable work is therefore D3:
complete the layered thermal-resistance/depth-progression port so depth
advance is bounded by the layered frost state, then rerun the same additive
identity and depth/duration cohort gates before MOFE.

D3 execution attempt, 2026-06-11: Codex tested a coarse continuous per-layer
energy-front implementation against the same 43-prefix population. The attempt
ran `43/43` clean and reduced median maximum frost depth to `490.774886655666 mm`,
but failed the package's phase boundary: mean max depth remained high
(`643.2973898432339 mm` vs legacy `414.22093023255815 mm`), severe outliers
still reached `1789.9130899451595 mm`, median depth correlation stayed negative
(`-0.1876255663636445`), and frozen duration remained under-persistent by a
median `-428` days. Static legacy inspection showed the missing structure is
the full fine-sublayer `frostn`/`frzng`/`mltbtm`/`frwatc` state machine
(`fgfrst`, `slfsd`, `slsic`, `slsw`, `nwfrzz`) rather than a coarse scalar or
coarse per-layer front. The production/test experiment was backed out; only
`SC-SNOWFREEZE-001` v57 and package evidence remained at that phase boundary.

D3 Increment A, 2026-06-11: Codex landed the staged shadow-state increment from
`artifacts/d3-staged-increment-plan.md`. `SC-SNOWFREEZE-001` v58 corrects the
`INV-SNOWFREEZE-012` `frwatc(1)` wording to hour-1 daily ingress
(`frostn.for:335-337`) and adds fine-sublayer shadow aliases plus a required
internal handoff residual. The landed state drives no depth, conductivity, WAT
publication, or freeze/thaw physics. This is a behavior-preserving seam proof
for increments B/C, not D3 acceptance. The pre/post WAT physical-byte subgate
was clarified after execution because the clean pre baseline contains
nondeterministic parquet `ARROW:schema` footers; decoded WAT is unchanged, and
latest-source current/current WAT physical parity is now `43/43`.

D3 Increment B, 2026-06-11: Codex landed the staged freeze-arm increment.
`SC-SNOWFREEZE-001` v59 promotes the fine-layer frost state as active
freeze-depth authority, adds `frost.hourly.frzflg_####`, derives
`frdp`/`thdp`/sandwich depths from `fgfrst`/`slfsd`, retires scalar target-depth
projection as production authority, and ratifies threshold-bounded
exchange-debit limiting at the available-liquid handoff boundary. The required
Rust gates and authority anti-evasion guards pass. A clean `43/43`
`algebraic-radium` cohort keeps years 2-6 `Total-Soil + frozwt` closure at
noise (max abs `3.0880187296133954e-11 mm`), preserves the tiny year-7 boundary
watch item (`1.2683569483584733e-07 mm`), removes exact profile-bound pinning
(`0/43` pinned; minimum margin `16.63152804088827 mm`), and keeps
`frozwt/frdp` materially below the rejected scalar signature (max per-prefix
correlation `0.9861968090242198` vs `0.9987`).

D3 Increment C attempt, 2026-06-11: Codex implemented top/bottom thaw arms and
ran the required local/heavy gates in the attempted tree. The first cohort run
failed on p1 because `wb18_perc_frzw_0001` exceeded pore capacity; after adding
a source cap the `43/43` cohort ran clean, but the attempt hit the D2 hard
stop: years 2-6 `Total-Soil + frozwt` residuals blew up to max abs
`2325832826960980.0 mm`, year 7 blew up to
`2203549546983243.5 mm`, and depth/duration parity still failed. The production
and contract/test edits were backed out, leaving FDHP01 at the Increment B
boundary. The next D3 pass must port capacity-aware `watdst` redistribution and
`watpdg`/`watbtm` overflow handling before `mlttp`/`mltbtm` thaw arms can be
retained.

D3 Increment C1/C1a/C1b, 2026-06-12: the first C1 capacity/redistribution
attempt and the C1a seam-accounting diagnostic localized the water-side failure
to fine-state ownership, pore capacity, and missing overflow surfaces. C1b then
landed the capacity/overflow infrastructure without using the comparator
subagent per user quota direction. `SC-SNOWFREEZE-001` was v61 at that
boundary. The final
local `43/43` `algebraic-radium` cohort at
`/tmp/fdhp01_increment_c1b_cohort_final14_20260612T035618Z` keeps years 2-6
`Total-Soil + frozwt` closure at numerical noise (max abs
`1.5347723092418164e-12 mm`), has zero valid-input capacity guard trips, keeps
p1/p43 starter traces at zero `frzw > ul` rows, preserves de-pinning
(`0/43` profile-bound pinned), and keeps `frozwt/frdp` correlation below
Increment B (max `0.9860178382757524`). The package remains executed-hold
because the depth watch worsened instead of improving: mean maximum depth is
`1791.9747961835646 mm`. At that boundary, C2 was required to port thaw arms,
sandwich/thaw-through state, and discriminate whether the remaining depth
magnitude was thaw-storage plumbing or freeze-side energy/resistance.

D3 Increment C2, 2026-06-12: Codex landed the thaw-arm state-machine increment
without using the comparator subagent per user quota direction.
`SC-SNOWFREEZE-001` is now v62. The authoritative local hourly
`algebraic-radium` cohort at
`/tmp/fdhp01_increment_c2_cohort_hourly_fix_20260612T035740Z` ran `43/43`
clean with `43/43` WAT outputs. Years 2-6 and year-7 `Total-Soil + frozwt`
closure remain at the package C1b ledger noise floor (`0.0 mm` in the retained
annual table), profile-bound pinning remains removed (`0/43` pinned; minimum
margin `5.54557792097421 mm`), and the old scalar `frozwt/frdp` signature stays
rejected (max correlation `0.9441102161636825`). The D3 acceptance gate still
fails: mean maximum depth is `1793.52198510966 mm`, median depth correlation is
`-0.16722397856345997`, median open-minus-legacy frozen-duration residual is
`111` days, and median days above `200 mm` is `815`. C2 therefore closes the
thaw/storage ambiguity but leaves FDHP01 in executed-hold; the next scope is a
freeze-arm heat-flow resistance/latent-heat audit and repair.

D3 Increment Da, 2026-06-12: Codex executed the freeze-arm energetics
diagnostic without using the comparator subagent per user quota direction. A
temporary p1 hourly trace was added, run, and removed before rebuilding the
production release CLI. Static legacy inspection shows `frzng.for:235-240` and
`:287-335` update `qoutdm` and recompute `qhtout` inside the 3600-second
front-advance loop whenever the frozen path grows. The p1 trace proves openWEPP
is missing that in-hour feedback: year 1 day 1 hour 2 advances from
`0.000397484 m` to `1.162927773 m` while keeping the pre-advance resistance
`0.000227134 m2 C/W`; applying the hour-end frozen path raises resistance to
`0.572822749 m2 C/W` and drops `|qhtout|` from `35602.871` to `14.117 W/m2`.
Da also replaced the C2 tautological closure ledger with an independent WAT
flux ledger. The local `43/43` cohort at
`/tmp/fdhp01_increment_da_cohort_20260612T044217Z` is row-identical to C2,
years 2-6 independent `Total-Soil + frozwt` closure is within
`1.3813070645629644e-07 mm`, and the p43 year-2 watch is
`-1.912025027195341e-08 mm`, cleared as WAT-surface numerical texture. FDHP01
remains executed-hold; Db must port the legacy in-hour resistance/front-advance
loop before any D3 acceptance claim.

D3 Increment Db, 2026-06-12: Codex landed the legacy `frzng` in-hour
resistance feedback. The local `43/43` cohort at
`/tmp/fdhp01_increment_db_cohort_20260612T051524Z` kept years 2-6 independent
`Total-Soil + frozwt` closure within `1.9976620946327017e-07 mm`, removed
profile-bound pinning, and put all 43 maximum depths inside the legacy
`240..503.2 mm` envelope with mean max `409.16220799389805 mm`. D3 still held
because depth correlation stayed weak and frozen duration under-persisted by a
median `-452` days.

D3 Increment Dc/Dc1, 2026-06-12: the first combined seasonal lower-front heat
and thaw-feedback pass improved timing correlation but reopened D2 accounting
and depth/duration, so it was backed out. Dc1 then split the work and landed
only the accounting-preserving boundary: seasonal `tmpbl`/`Qdry`, in-hour thaw
resistance feedback, and bounded fine-theta lower-bound canonicalization.
`SC-SNOWFREEZE-001` is now v64. The local `43/43` cohort at
`/tmp/fdhp01_increment_dc1_cohort_20260612T101238Z` restores years 2-6
independent `Total-Soil + frozwt` closure to `6.471338602487275e-07 mm` max
abs, with p43 year 2 at `-1.1013412404281553e-13 mm`. FDHP01 remains
executed-hold because the depth/duration evidence is still red: mean max depth
`1146.5109665924424 mm`, one profile pin, `0/43` prefixes inside the legacy
envelope, median depth correlation `0.6415921721982907`, and median frozen
duration residual `+567` days. The next scope is Dd/F4 legacy-snow-forced
frost certification, not MOFE closure.

D3 Increment Dd, 2026-06-12: Codex executed the legacy-snow-forced diagnostic
without the comparator subagent per user quota direction. A temporary
env-gated hook forced only the snow depth/density consumed by frost from
legacy `H*.winter.dat` hour-24 rows, then was removed before the production
rebuild. The forced cohort at
`/tmp/fdhp01_increment_dd_forced_snow_cohort_20260612T121500Z` ran `43/43`
clean and preserved years 2-6 independent `Total-Soil + frozwt` closure at
WAT-publication texture (`6.726058817130287e-07 mm` max abs; p43 year 2
`-1.2079226507921703e-13 mm`). Dd does not certify F4 as the whole remaining
D3 story: forced legacy snow removes profile pinning and improves median
correlation to `0.7118806632341061`, but mean max depth remains
`856.817674502367 mm`, `0/43` prefixes enter the legacy `240..503.2 mm`
envelope, and frozen duration still over-persists by median `+502` days. The
next increment remains frost-side, using the Dd forced-snow setup to localize
the remaining hourly front/flux divergence.

D3 Increment De, 2026-06-12: Codex landed the content-dependent `Qdry`
conductivity correction without the comparator subagent per user quota
direction. `SC-SNOWFREEZE-001` is now v65, the lower-front heat path uses the
legacy `frostn.for:430-458` fine-layer polynomial/harmonic conductivity with
`0.2 W/m/K` only as the dry fallback, and bottom thaw consumes the same
conductivity. The final native production cohort at
`/tmp/fdhp01_increment_de_native_cohort_final_20260612T171358Z` ran `43/43`
clean with years 2-6 independent `Total-Soil + frozwt` closure at
`5.474257917248426e-07 mm`, but native D3 remains red: mean max depth
`705.505148615878 mm`, `0/43` prefixes inside the `240..503.2 mm` envelope,
and median duration residual `+288` days. The corrected forced legacy-snow
diagnostic at
`/tmp/fdhp01_increment_de_forced_snow_cohort_20260612T171017Z_proper` ran
`43/43` clean with years 2-6 closure at `4.355148297552347e-07 mm` and
improved the controlled residual (mean max `655.9890274782282 mm`, median
correlation `0.770042438411068`, median duration residual `+186` days), but it
still failed certification because `0/43` prefixes entered the legacy maximum-
depth envelope. FDHP01 remains `executed-hold`; the next scope is paired hourly
front/flux localization under De forced-snow forcing, not MOFE closure.

D3 Increment Df, 2026-06-12: Codex executed the paired hourly front/flux
localization without the comparator subagent per user quota direction. A
temporary env-gated forced-snow + hourly trace hook was added, run for p1/p2,
and removed before the final clean-source production rebuild (release binary
SHA `981da203d9ced9b1d73f049fa3a4b227710862a3dbecaad9d4619f03ae7dd2d5`).
Static source inspection corrected one candidate: the legacy winter `ground`
column is ground-drift snow, not temperature. The paired traces localize the
first material residual to the frost surface-resistance path. On both p1 and
p2, year 1 day 1 hour 2 is snow-free in both models; legacy frost depth is
`5.0 mm`, while openWEPP reaches `42.057866709 mm` (p1) and
`41.417581693 mm` (p2). openWEPP feeds `residue_depth_m = 0.0` into frost on
those rows while legacy uses `23.0 mm` residue depth, and openWEPP omits the
legacy shallow-front `dpfsfl` minimum conduction distance. Df is diagnostic
only and leaves FDHP01 in executed-hold; the next scope is Dg, a bounded
residue-path plus shallow-front-minimum resistance implementation, followed by
the De forced-snow certification and native cohort.

D3 Increment Dg, 2026-06-12: Codex landed the legacy residue-depth frost
surface resistance and below-freezing shallow-front `dpfsfl` conduction floor
without the comparator subagent per user quota direction. `SC-SNOWFREEZE-001`
is now v66. The native production cohort at
`/tmp/fdhp01_increment_dg_native_cohort_20260612T184601Z` ran `43/43` clean
with years 2-6 independent `Total-Soil + frozwt` closure at
`6.261351281899863e-07 mm`; mean maximum depth improved to
`498.08123930883653 mm`, `30/43` prefixes entered the legacy
`240..503.2 mm` depth envelope, and median duration residual improved to
`+84` days. The forced legacy-snow diagnostic cohort at
`/tmp/fdhp01_increment_dg_forced_snow_cohort_20260612T185203Z` also ran
`43/43` clean with years 2-6 closure at `5.835723933533821e-07 mm`; relative
to De forced-snow, mean max depth dropped `655.9890274782282 ->`
`490.0923199552928 mm`, envelope membership improved `0/43 -> 30/43`, and
median duration residual improved `+186 -> +73` days. Dg passes its
directional gate but FDHP01 remains executed-hold because `13/43`
forced-snow prefixes still exceed the `503.2 mm` upper envelope bound.

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `FDHP01-FROST-DEPTH-HEATFLOW-001`: openWEPP computes frost **depth** with a
freeze-index proxy (`frdp = 0.20·clamp(−mean_temp/6)`, hard-capped 0.20 m) instead of the
energy-balance **heat-flow** model the contract already mandates
(`SC-SNOWFREEZE-001#INV-SNOWFREEZE-006`/`-012`; legacy `frostn` lineage; CRM Ch. 3.8
Eq. [3.8.1]–[3.8.4]; Dun et al. 2010 ASABE 53(5):1399–1411). Replace the proxy with the
heat-flow depth model on the frost-active single-OFE substrate so frost depth and
frozen-soil duration track the heat-flow authority, and **close `GAP-SNOWFREEZE-002`** —
with the rung-1 + frost conservation closure still holding.

This package owns correction inside the frost depth-model envelope. The contract authority
(`INV-SNOWFREEZE-006`/`-012`, hard-fail) **already exists**; this is "make the
implementation match the contract," so if the root cause is in-envelope it must land.

## Why now (the re-sequence) and why single-OFE

Re-sequenced ahead of MOFE (2026-06-07, Roger): frost is a per-column **vertical**
mechanism, and the ladder settles vertical mechanisms on single-OFE **before** routing so
their error is not aliased into routing error. FQ-4 settled frost *activation* that way;
this settles the frost *depth model* the same way, before MOFE (rung-3). Building MOFE on
the proxy and fixing depth later would force a re-validation of MOFE under frost; doing it
now means MOFE is built once on a faithful frost foundation, and the heat-flow physics is
debugged in isolation — one column, no routing to confound it.

The gap is sized, not assumed:
[FDMC01](../20260608-fdmc01-frost-depth-comparator-characterization-001/) verdict
**materially off** — openWEPP depth capped 200 mm vs legacy 240–503 mm (43/43 exceed the
cap), depth-series median correlation **0.13**, and frozen-water duration **+258 days**
(the proxy ratchets via `frdp = max(prior, …)`, thawing only when `tmin>0`, so it
over-persists). The duration error is the more consequential one: the conductivity bite is
near-total whenever frost exists, so the proxy holds the frozen runoff-generation window
open ~34% longer than legacy.

## Rationale / authority

- `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-006` (frost heat-flow `Qsrf`/`Quf` + harmonic-mean
  layered conductivity, Eq. [3.8.1]–[3.8.4]) and `INV-SNOWFREEZE-012` (`winter → frostN →
  frzng → frznw` dispatch chain) are **hard-fail and already in the contract**;
  `GAP-SNOWFREEZE-002` records the implementation/parity as open. This DC implements to
  the existing authority and closes that gap.
- External authority: CRM Ch. 3.8; Dun et al. 2010 (improved WEPP frost subroutines,
  fine-layer discretization).
- Legacy `frostn.for`/`frzng.for`/`frznw.for`/`frsoil.for` are the **corroborating
  reference implementation** of that authority (ADR-0017: flag/reference, not acceptance
  target). The comparator (`wepp_260606_hill`) is a flag that depth/duration should match
  the heat-flow envelope, not a number to tune to.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `FDHP01-FROST-DEPTH-HEATFLOW-001`
  - Observable: `frdp_m` capped 0.20 m (proxy) vs legacy heat-flow 0.24–0.50 m; depth
    series correlation 0.13; frozen-duration +258 days. Evidence:
    `../20260608-fdmc01-frost-depth-comparator-characterization-001/artifacts/`
    (metrics CSV, summary JSON, ledger, verdict).

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
    (**primary** — implement to `INV-SNOWFREEZE-006`/`-012`, tighten with the
    implementation spec as needed, **close `GAP-SNOWFREEZE-002`**).
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (downstream:
    closure must still hold with the new `frozwt` magnitude/timing).
- Production/test files (localization re-verified 2026-06-10 after REFACTOR015/019–021
  split the hydrology modules; cite symbols, not pre-refactor line numbers):
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
    — **the proxy-replacement seam.** Holds the freeze-index proxy
    (`freeze_index`/`thaw_index` over `FROST_RUNTIME_FREEZE_INDEX_SCALE_C`, the
    `frdp_m.max(WB14_FROST_MAX_DEPTH_M * freeze_index)` ratchet), the hourly
    `qsrf_w_m2`/`quf_w_m2` heat-flux block, `kfactor1..3` selection
    (`resolve_frozen_soil_kfactor`), and `resolve_active_frost_coupling` /
    `compute_active_frost_coupling`. The heat-flow model replaces the proxy here.
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
    — frost runtime state and symbols (`frdp_m`/`tfrdp_m` fields,
    `FROST_RUNTIME_FRDP_M_SYMBOL`, `FROST_RUNTIME_FREEZE_INDEX_SCALE_C` — this
    constant lives here post-refactor, not in `constants.rs`).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/`
    — frost outcome → soil conductivity consumption
    (`hydrology_phase_infiltration_evap.rs`, `hydrology_phase_runoff_reconciliation.rs`)
    and harmonic-mean layered conductivity (`hydrology_phase_plant_percolation.rs`).
  - `crates/openwepp-hillslope-orchestrator/src/constants.rs` (`WB14_FROST_MAX_DEPTH_M` —
    retire the proxy cap, add heat-flow constants with provenance).
  - `crates/openwepp-runner/src/hillslope/` publication helpers
    (`02_output_and_climate_helpers.rs`, `00_runner_intake_and_lane_setup.rs` — **publish
    `frdp`** to the WAT/output surface per the FDMC01 caveat so future comparisons read
    actual runtime state, not a reconstruction).
  - `tests/integration/**frost**.rs`, `**snowfreeze**.rs`, `**watbal**.rs`,
    `clim06_frost_frozen_soil_kernel_contract.rs`.
  - `docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/**`,
    `docs/ROADMAP.md`, `docs/work-packages/README.md`,
    `docs/backlog/20260607-frost-depth-model-heat-flow-parity.md`.

### Allowed Edit Classes

- Amend `SC-SNOWFREEZE-001` to specify the heat-flow depth implementation and close
  `GAP-SNOWFREEZE-002` **before** production code.
- Implement the energy-balance layered heat-flow frost-depth model (fine-layer
  discretization per Dun 2008; depth permitted to the physical range, retiring the 0.20 m
  cap) in place of the freeze-index proxy.
- Publish `frdp` to the WAT/output surface.
- Add contract-derived tests (heat-flow depth on a known-cold profile; frozen-duration
  without the ratchet over-persistence; warm-day/non-freezing non-regression; activation
  non-regression).
- Add bounded diagnostics for the depth/energy-balance state.

### Protected Boundaries (do not cross)

- **No comparator-match tuning.** `wepp_260606_hill` is a flag that depth/duration should
  fall in the heat-flow envelope (ADR-0017); acceptance is contract-correct heat-flow
  behavior, not matching legacy `frdp` to the millimetre.
- **Do not regress frost activation** (FQ-4) — the `ksflag`/`wintRed` activation gate
  stays; this changes only how deep/how long, not whether frost engages.
- **kfactor conductivity magnitude is legacy-faithful — do not change it.** This DC fixes
  the depth/duration that *drives* the bite, not the bite coefficient.
- **Forest `ksatadj`** is a separate path — out of scope.
- **Conservation must close** — the new `frozwt` magnitude/timing must keep the rung-1 +
  frost closure identity (with `frozwt` in storage) closed; no balance break.
- Snow magnitude (Stage-2), ET/runoff (closed), `p11` percolation (FQ1-P11) — do not
  touch.
- **Single-OFE only.** MOFE / the 17-OFE hillslope is rung-3 (item 2) — out of scope.

### Acceptance Criteria

- Frost depth and frozen-soil duration on the `algebraic-radium` single-OFE cohort track
  the `INV-SNOWFREEZE-006`/`-012` heat-flow authority (comparator as flag): the FDMC01
  gap materially closes — depth reaches the physical range (cap retired), depth-series
  correlation rises substantially, and the frozen-duration over-persistence (+258 days /
  ratchet) is eliminated.
- `GAP-SNOWFREEZE-002` is closed (or its residual scope explicitly re-stated if a phase
  boundary is declared — see Branch-out).
- Water-balance closure (rung-1 identity incl. `frozwt`, + totalwatsed3 audit) still
  closes after the depth model changes.
- Frost activation (FQ-4) is non-regressed; `frdp` is published to the output surface.
- Contract-derived red/green tests; pre-implementation failing evidence; post-fix
  validation over the cohort + non-regression on warm/non-freezing days.
- No conservation break, comparator-target tuning, kfactor change, activation regression,
  or downstream compensation.

### Branch-out Boundaries

- The full legacy heat-flow chain (layered energy balance + hourly `Qsrf`/`Quf` + fine
  sublayers) is substantial. If Milestone 1 shows the complete port exceeds one package,
  a **phased boundary** is legitimate **only if** the landed phase still closes the FDMC01
  depth+duration gap (a faithful energy-balance depth model retiring the cap and the
  ratchet) — deferring, e.g., hourly `Qsrf`/`Quf` sub-family *publication* detail with the
  residual re-stated in `GAP-SNOWFREEZE-002`. A cosmetic tweak to the proxy is **not** a
  legitimate phase.

## Conversion Rule

Reproducible in-envelope root cause (the proxy depth model) + canonical authority
(`INV-SNOWFREEZE-006`/`-012` + CRM Ch. 3.8 / Dun 2010) ⇒ proceed through contract
amendment → tests → pre-implementation gate → production correction → validation →
disposition. May not close `HOLD` because the heat-flow port is large — scope it (phased
boundary) and land the gap-closing phase.

## Symptom-Existence + Ownership Gate (Milestone 1, first)

1. Reproduce the FDMC01 baseline on single-OFE (proxy depth capped 200 mm, duration
   over-persisting) vs legacy heat-flow.
2. **Scope the implementation**: confirm the heat-flow depth model extent to port (full
   `frostn`/`frzng`/`frznw` layered energy balance with Dun-2008 fine sublayers vs a
   faithful energy-balance subset that closes the gap), and localize the proxy-replacement
   seam in `support_helpers_mod/coupling.rs`. Declare a phased boundary here if warranted
   (per Branch-out).
3. Ownership: contract `INV-SNOWFREEZE-006`/`-012` already mandate heat-flow; the proxy is
   the openWEPP divergence (`GAP-SNOWFREEZE-002`). openWEPP defect, in-envelope.

## Legitimate HOLD Conditions

- Heat-flow authority genuinely under-specified in the contract for an implementation
  decision (amend the contract, do not stall).
- Required heat-flow inputs (e.g. an energy-balance forcing) unavailable on the substrate
  (document with evidence).
- A declared phased boundary (per Branch-out) — lands the gap-closing phase, re-states the
  residual; this is a scoped close, not a grind-HOLD.

Grind-HOLD (forbidden): "tune the freeze-index scale," "raise the 0.20 m cap and stop,"
"port one more subroutine then defer."

## Milestones

1. Symptom-existence + ownership + implementation-scope gate (above).
2. Contract: amend `SC-SNOWFREEZE-001` (heat-flow depth implementation spec; close/restate
   `GAP-SNOWFREEZE-002`).
3. Contract-derived red tests (heat-flow depth/duration; ratchet eliminated; activation +
   warm-day non-regression).
4. Pre-implementation gate evidence.
5. Production correction: heat-flow depth model replaces the proxy; publish `frdp`.
6. Validation: FDMC01 gap closed on the cohort; conservation still closes; activation
   non-regressed.
7. Dual review, finding disposition, dual verification, defect-shaped handoff (naming MOFE
   as the next ROADMAP item).

## Deliverables

- `artifacts/frost-depth-heatflow-localization.md` (M1 scope + seam + ownership).
- `artifacts/fdhp01-frost-depth-validation-ledger.md` (post-fix depth/duration vs legacy +
  FDMC01-metric improvement + conservation preservation + activation non-regression).
- `artifacts/required-reading-map.md` (tiered authority map, living artifact).
- Standard DC artifact set: contract implementation evidence, contract-test
  implementation evidence, pre-implementation contract gate, implementation/test
  evidence, kernel-profile compliance checklist, owned-file manifest, gate results,
  line-count governance, dual review with finding disposition, dual verification,
  disposition, worker handoff.

## Dependencies

- `docs/ROADMAP.md` (queue item 1), `docs/backlog/20260607-frost-depth-model-heat-flow-parity.md`
- FDMC01 package + artifacts (the sized gap + metrics this must close)
- FQ-4 package (activation — must stay non-regressed). Substrate re-validated
  2026-06-11 by the FROSTVAL01 rerun (43/43 frost-active, closure-under-frost at
  `3.2e-11 mm`; frost-off paired runs also 43/43 clean — see
  `../20260608-frostval01-ksflag-frost-single-ofe-closure-validation-001/artifacts/rerun-20260611-frostval01.md`).
  That rerun is the activation/closure non-regression baseline for this package.
- `AGENTS.md`, `docs/defect_closure_execplans.md`, ADR-0011/0017/0018
- `docs/standards/kernel-work-package-preparation.md`,
  `docs/standards/prompt-wording-guidance.md` (§4a subagent requirement),
  `docs/prompt_templates/owcmp-comparator-runner-guidance.md` + `tools/owcmp/`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`, `SC-WATBAL-001.md`
- Legacy reference: `/workdir/wepp-forest_260430_baseline/src/frostn.for`, `frzng.for`,
  `frznw.for`, `frsoil.for`; CRM Ch. 3.8; Dun et al. 2010
- Comparator `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`; substrate
  `/wc1/runs/al/algebraic-radium/wepp/runs/` (single-OFE, `ksflag=1`)

## Subagent Requirement

Subagent requirement: REQUIRED, not optional. This package explicitly authorizes
subagent spawning/delegation to:

- `comparator_suite_runner` (gpt-5.3-codex-spark) for **all heavy batch/closure/
  comparator runs** — `cargo test --workspace`, clippy/deny closure loops, `owcmp`
  comparator suites, and the 43-prefix `algebraic-radium` population validation runs.
  Outputs: compact metrics + log/artifact paths only (no source/contract edits);
  write access: read-only plus package `artifacts/` logs. **Do NOT run heavy
  batch/closure work on the parent model** unless the subagent is unavailable, in
  which case record command-level evidence as justification
  (`docs/standards/prompt-wording-guidance.md` §4a).
- review/verification subagents for the dual review (`review_agent_a/b`) and dual
  verification (`verification_agent_a/b`) artifacts. Outputs: the named artifacts
  with finding dispositions; write access: bounded to package `artifacts/`.

## Comparator Execution

Comparator/cohort work goes through the `owcmp` CLI per
`docs/prompt_templates/owcmp-comparator-runner-guidance.md`: discover suites with
`tools/owcmp/owcmp manifest list`, prefer a manifest under `tools/owcmp/suites/` +
`tools/owcmp/owcmp env --manifest <path>`, and return only compact metrics and
artifact paths (`summary.json`, `summary.md`, `command-log.json`). Do not paste raw
per-hillslope reports into chat. The raw `wepp_260606_hill` binary remains the
underlying comparator (a flag, per ADR-0017), invoked via the suite tooling.

## Security-Impact Gate

No new input-parsing surfaces, no subprocess/argument-construction changes, no
network egress, no `unsafe`. The package touches kernel frost physics, constants,
and output publication only. Publishing `frdp` adds one column to the existing
parquet/WAT output writer. If execution discovers it must touch a parser or
subprocess boundary, stop and record the scope change before proceeding.

## Autonomy

Execute end-to-end for the declared single-OFE scope — M1 scope/ownership, contract
amendment, red tests, pre-impl gate, heat-flow implementation, validation, dual
review/verification, disposition, handoff — without asking for direction on intermediate
steps. A phased boundary (per Branch-out) is a permitted scoped close; a proxy tweak is
not. Ask only if the heat-flow authority is under-specified for a needed implementation
decision (then amend the contract).
