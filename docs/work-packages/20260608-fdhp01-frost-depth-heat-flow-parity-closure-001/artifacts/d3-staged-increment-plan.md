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
- Gates: those tests green; 43/43 clean; years 2–6 closure at noise;
  capacity invariant never violated on the cohort (zero guard trips on
  valid input); freeze-arm directional metrics (de-pinning, decorrelation)
  not regressed vs the B boundary. Bit-identical outputs are NOT expected
  (redistribution is new physics on the freeze path) — declare deltas.

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

## Dispatch instructions

Each Codex dispatch is: *"Execute increment <A|B|C1a|C1b|C2> of
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
