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

- **D2 hard stop:** the years 2–6 `Total-Soil + frozwt` cohort closure at
  ≤ ~3e-11 mm is a per-increment gate. Any regression stops the increment;
  fix or back out *within the increment* — never carry a broken identity
  into the next.
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

## Dispatch instructions

Each Codex dispatch is: *"Execute increment <A|B|C1a|C1b|C2|Da|Db> of
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
