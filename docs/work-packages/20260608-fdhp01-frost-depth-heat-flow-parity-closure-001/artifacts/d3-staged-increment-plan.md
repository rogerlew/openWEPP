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

## Increment C — thaw arms + sandwich geometry + D3 acceptance

**Objective:** complete the state machine — `mlttp`/`mltbtm`, sandwich
frost (`fgfrst=2/3`, `tfrdp`/`tthawd`), `fgthwd` thaw-through and early
`frwatc(0)`, `watpdg`/`watbtm` overflow paths — and take the package's D3
acceptance gate.

- Red tests first: scope tests 5 (bottom thaw), 6 (top thaw + `fgthwd`),
  8 (multi-day additive closure), 9 (cohort gate).
- Gates (the package acceptance, per scope §6 and addendum 3): 43/43
  clean; depth cap-free and inside the heat-flow envelope (FDMC01/legacy
  240–503 mm range as flag, not target); depth correlation rises materially
  from the 0.13 baseline; frozen-duration delta collapses from −518/−428
  toward zero; years 2–6 closure at noise; the year-7 boundary residual
  explained or eliminated; FQ-4 activation non-regressed.
- On pass: FDHP01 disposition to complete, `GAP-SNOWFREEZE-002` closed,
  ROADMAP item 1 removed, README 7f updated, handoff names MOFE.

## Dispatch instructions

Each Codex dispatch is: *"Execute increment <A|B|C> of
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
