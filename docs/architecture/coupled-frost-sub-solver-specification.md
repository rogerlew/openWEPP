# Coupled Frost Sub-Solver Architecture Specification

Status: **Draft / Proposed** - design-input authority for the direct-frame
frost/winter port. NOT yet ratified. Adoption requires an ADR or an amendment to
[array-native-runtime-specification.md](array-native-runtime-specification.md)
§12 (Open Design Decisions).
Audience: contributors working on the direct-frame runtime, hydrology kernels,
snow/freeze coupling, and the R7 burndown.
Owner: architecture authority; implementation by Codex work packages.
Subordinate to: [array-native-runtime-specification.md](array-native-runtime-specification.md)
controls runtime architecture; [SC-SNOWFREEZE-001](../specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md)
controls frost/snow science behavior. This document proposes a runtime *shape*;
it changes no physics.
Evidence mode: static - source reads of the pinned legacy baseline
(`/workdir/wepp-forest_260430_baseline`, commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`), the openWEPP direct runtime, and
`SC-SNOWFREEZE-001`. No simulations were run for this document.
Last updated: 2026-06-23.

---

## 0. Summary

Frost is not a feed-forward water-balance phase. It is a **coupled, stateful,
hourly sub-solver** with long-memory seasonal state, an internal moving
freeze/thaw front, and a hard dependency on the snowpack that insulates it. The
current runtime - in both compatibility and direct modes - shapes frost as a
*coupling re-entered from inside two feed-forward phases*, re-resolving its full
state from the symbol surface at each entry. The direct-frame port has built the
two easy thirds (typed persistent state, typed publication projection) and
stalled on the hard third (the solver itself), so direct mode carries and
publishes frost state but never advances it.

This document records the motivation, the legacy deficiencies that make frost
reconstruction intrinsically hard, and the suitability findings from the current
implementation shape. It proposes a **winter-column sub-solver boundary**: one
coupled snow+frost solver that owns persistent lane state, runs once per OFE-day
(advancing its 24 hourly steps internally), and exposes typed outputs that the
feed-forward phases consume as ordinary inputs.

The recommendation is to stop porting frost as a feed-forward phase fragment and
give it a sub-solver boundary that matches its physics, doing snow first because
frost depends on it.

---

## 1. Motivation

### 1.1 Frost is structurally unlike the other phases

The array-native runtime's phase model (array-native spec §4.7) assumes phases
are narrow, roughly pure functions over the day frame: immutable inputs in,
owned outputs out, producer/consumer edges expressible as borrows. That model
fits the water-balance phases (Normalization, StorageBounds, Drainage,
Evapotranspiration, LateralTransfer, etc.), which are predominantly feed-forward
within a day.

Frost violates every assumption of that model. Per `SC-SNOWFREEZE-001` and the
WEPP technical documentation chapter 3 (`REF-SNOWFREEZE-CH3-FROST`, §3.8,
Eq. [3.8.1]-[3.8.4]), the frost subsystem is:

| Property | Consequence for the phase model |
|---|---|
| **1-D vertical energy balance with a moving freeze/thaw front (Stefan-type)** | The day output is the result of an internal time integration, not a single arithmetic pass. |
| **Hourly internal iteration (24 sub-steps)** | The "phase" hides a loop; the feed-forward executor cannot see or own it. |
| **Long-memory seasonal state** (deep "stable" temperature from a fitted annual harmonic; multi-day frost depth, thaw flags, freeze-thaw count, frozen-water ledger) | State persists across the whole winter, not day-to-day. A per-OFE-day frame optimized for daily-forcing -> daily-flux is the wrong owner. |
| **Snow coupling** (snow depth *and* density govern insulation; not SWE) | Frost cannot be solved without authoritative snow state; the two are one coupled column. |
| **Feedback loops** (ice content -> conductivity -> temperature -> ice content; snow insulation -> frost depth -> infiltration -> soil water -> freezing point) | Not a DAG of producer/consumer edges; an iterated coupled system. |
| **Two-point hydrology coupling** (frozen-soil infiltration reduction *and* freeze/thaw liquid partition into runoff/storage) | Frost effects are consumed at more than one place in the day's process order. |

### 1.2 The physics openWEPP frost reconstructs

For completeness, the target model (authority: `SC-SNOWFREEZE-001`, WEPP
chapter 3, August 1995 release lineage) is an hourly energy balance over a
layered **snow -> residue -> soil** column:

- **Top boundary**: adjusted surface temperature from a surface energy balance
  (air temp, radiation, cloud, wind, albedo, canopy roughness, snow/residue/frost
  conductance), capped at `0 degC` when snow-covered (`REF-SNOWFREEZE-LEGACY-TMPADJ`).
- **Bottom boundary**: heat flow `Qdry` from a deep "stable" temperature derived
  from a fitted annual air-temperature harmonic damped with depth
  (`REF-SNOWFREEZE-LEGACY-FROSTN-QDRY`).
- **Front energy balance**: net energy converted to ice mass through the latent
  heat of fusion, advancing the freeze/thaw fronts on a fine-layer discretized
  profile (10 sublayers per soil layer). Frozen conductivities are fixed
  constants (`kftill = 1.75`, `kfutil = 2.1 W m^-1 degC^-1`,
  `REF-SNOWFREEZE-LEGACY-FROZEN-PATH-KF`).
- **Migration latent heat** `Qwet` (Eq. [3.8.4]): documented but dead in the
  baseline; see §2.1.
- **Hydrology coupling** (chapter 4-5): frozen-soil infiltration reduction,
  snowmelt treated as rainfall, snow-water storage in the daily balance, excess
  liquid routed to surface ponding (`watpdg`) or below profile (`watbtm`).

This is a coherent coupled-column model. Its shape is a sub-solver, not a
feed-forward phase.

---

## 2. Legacy Codebase Deficiencies

Static: read from `frzng.for` and `frostn.for` in the pinned baseline. These are
the reasons frost reconstruction is intrinsically hard and the reason a boundary
that *localizes the contract decisions* matters.

### 2.1 The dead `Qwet` migration term

`frzng.for:381-394` contains a physically-motivated derivation in comments,
followed by a hardcoded override that disables it:

```fortran
cd    Based on the discussion with Kunio Watanabe, Associate Professor, Mie University, Japan
c      in January 2008 when he was visiting WSU.
c      ... pressure potential at frozen front should be in the range of -20 m to - 160 m.
c      Here, we select to use -100 m.
cd      frzftp = -50          ! commented out
      frzftp = 0.0            ! what actually runs
```

Three values appear: the documented intent (`-100 m`), a commented-out debug
leftover (`-50`), and the live value (`0.0`). With `frzftp = 0.0`, the migration
gate at `frzng.for:410` (`if ((frzftp .lt. wtpm) ...)`, comparing `0.0` against a
negative soil-water potential) is effectively never true, so the entire
Clausius-Clapeyron migration-heat computation (`frzng.for:381-437`, the documented
Eq. [3.8.4] physics) is unreachable. The documented physics is dead code wearing
a citation.

### 2.2 Deficiency patterns

That one window is a representative catalog:

| Pattern | Evidence | Why it blocks reconstruction |
|---|---|---|
| **Debug override shipped as production** | `frzng.for:393-394` (`cd frzftp = -50` over `frzftp = 0.0`) | A provisional value became 30-year behavior. |
| **Comments contradict code** | `frzng.for:385-394` (documented `-100 m` vs live `0.0`) | The only question that matters - *what is this supposed to do?* - is unanswerable from source. |
| **Magic fudge factors with verbal justifications** | `frzng.for:405-413` (`/2.0`, "assuming migration rate slows down due to water depletion") | A fitted convenience presented as physics. |
| **Hardcoded geometric assumptions** | `frostn.for:11-18` (1 cm tilled / 2 cm untilled fine layers assuming 10 cm/20 cm soil layers), `:62` (`frdp <= 1.0 m` cap), `:74` (`tilld = 0.2 m`), `:79-82` (stable temp 1 m below isotherm) | Calibrated to a specific cropland geometry; silently wrong elsewhere. |
| **Sedimentary authorship** | `frostn.for:22-36` (Witte 1993, Savabi 1994, Meyer 1996 "Eliminated calls to AVOID2, CAQWET, CAQDRY", Flanagan 1997, Dun 2008) | No single coherent design; five strata over fifteen years. |
| **Silent bail-outs** | `frzng.for:462-465` (`wepp_observe(...)` then `return` on zero fine-layer / zero thickness) | Edge cases handled by abandoning the calculation. |

### 2.3 The load-bearing consequence

Legacy frost encodes **no clean contract**. The contract-vs-bug question - is a
given baseline behavior an intended interface to preserve for parity, or a
defect to correct? - is unresolvable from the source for the central terms
(`Qwet` says `0.0` in code, `-100 m` in comment, `-50` in the debug line). A
faithful transliteration would carry this ambiguity forward forever. Contract-
first reconstruction (`SC-SNOWFREEZE-001`, currently `in_review`/`draft`) forces
the decision to be made and written down - which is the slow, judgment-heavy work
that no representation change can shortcut. A boundary that localizes those
decisions to one place is therefore an asset, not just an optimization.

---

## 3. Current Implementation - Suitability Findings

Static: read from the openWEPP hydrology kernels and direct runtime.

### 3.1 Frost is a coupling re-entered from multiple feed-forward phase points

`resolve_active_frost_coupling` is invoked at three sites across two phases:

- `hydrology/kernel_phases_mod/hydrology_phase_infiltration_evap.rs:334`
  (Evapotranspiration / WB14 infiltration capacity), followed by
  `compute_active_frost_coupling(...)`.
- `hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs:44`.
- `hydrology/support_helpers_mod/runoff_reconciliation.rs:180`.

Frost's hourly solver is reached as a mid-phase coupling, in two unrelated
feed-forward phases, each re-resolving frost state from the symbol surface. This
is the feed-forward mold imposed on a sub-solver, present already in the
compatibility code and inherited by the direct port.

### 3.2 The compatibility solver is already a coherent sub-solver - bound to the symbol surface

`compute_active_frost_coupling` (`coupling/frost_entry.rs:1892`) is a clean,
encapsulated Stefan-type solve: require controls -> layer water state ->
fine-layer shadow -> surface inputs -> prior context -> thermal context ->
`compute_active_frost_hourly_state` (the 24-hour freeze/thaw-front stepping) ->
`FrostCouplingOutcome`. The physics is already a unit. The deficiency is the
**boundary**: every input is a `require_*_for_symbol` read off
`HillslopeKernelRequest`, re-resolved on each call rather than passed as typed
arguments and mutated in place.

### 3.3 The direct port built state and publication, not the solver

| Piece | Status | Evidence |
|---|---|---|
| Typed persistent cross-day state | **Exists** | `DirectFrostRuntimeCarry` (`direct_runtime/00_core_frames.rs:170-201`): `frdp/tfrdp/thdp/tthawd`, `fgthwd`, `nft`, `ws_frz`, `infcap_frz`, 8-field `frwatc` ledger, conductivities, `watpdg/watbtm`, full fine-layer shadow (`layer_shadows`, `fine_layers`). |
| Typed publication projection | **Exists** | `DirectFrostLayerCarryProjection`, `DirectFrostRunoffSurface`, `DirectFrostLiquidPartition` (`direct_runtime/01_publication.rs:50-53, 85+`). |
| Solver invocation | **Absent** | `frost_runtime_carry: None` default (`00_core_frames.rs:250`); `frost_liquid_delta_m: 0.0` hardcoded (`normalization.rs:426`). Direct runtime files contain no call to `compute_active_frost_coupling` or any thermal solver. |

Direct mode therefore *carries and publishes* frost state but never *advances*
it. Active-frost / active-snow days are exactly where this surfaces.

### 3.4 Snow is the upstream instance of the same problem

The current R7G blocker, `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`,
is the snow partition helper (`runoff_reconciliation.rs`) still reading
`snow.runtime_*` symbols off `HillslopeWritebackSurface` because no typed direct
snow partition exists. Frost's thermal forcing depends on snow depth and density,
so snow must be typed-direct first. Same shape, one layer upstream.

### 3.5 Suitability verdict

The direct port replicated the compatibility shape - "frost = coupling woven into
ET and runoff, re-resolved from symbols." To finish it *in that mold*, two
different direct phases would each need typed access to the entire coupled winter
state (snow + frost + fine-layer ice + seasonal curve + priors), re-resolved at
two points, while R7F has deliberately cut the hot loop off from the symbol
surface. The mold fights the physics. The required change is a boundary change,
not more input plumbing.

---

## 4. Suitability Assessment - Why a Sub-Solver Boundary Fits

| Frost requirement (from §1) | Feed-forward phase fit | Coupled sub-solver fit |
|---|---|---|
| Internal hourly time integration | Poor - the loop is hidden inside a "pure" phase | Good - the loop is the solver's body, invisible to the executor |
| Long-memory seasonal state | Poor - day frame is not the owner | Good - persistent lane state, mutated in place |
| Snow+frost coupling | Poor - two subsystems re-resolved separately | Good - one coupled column owns both |
| Feedback loops | Poor - not expressible as borrow edges | Good - internal to the solver |
| Two-point hydrology coupling | Poor - drives the double re-entry | Good - one solve, typed outputs read by both consumers |
| Contract-decision localization (§2.3) | Poor - decisions smeared across phase fragments | Good - one typed boundary to bind `SC-SNOWFREEZE-001` |
| Independent validation | Poor - state re-resolved from symbols inside phases | Good - narrow typed in/out is directly shadowable |

The sub-solver shape matches every property where the feed-forward phase model
is a poor fit. This is the architecture-shaped decision underneath the frost
grind.

---

## 5. Proposed Boundary (Sketch)

This is a design sketch, not a ratified design. It changes runtime shape only;
physics remains governed by `SC-SNOWFREEZE-001`.

### 5.1 Reframe

Move frost (and snow with it) from "a coupling call inside ET and runoff" to
"one coupled **winter column** sub-solver that owns its state, runs once per
OFE-day, advances its 24 hourly steps internally, and emits typed outputs the
feed-forward phases consume as ordinary inputs."

### 5.2 State ownership - promote the carry to authority

Introduce a lane-owned, persistent `WinterColumnState` that fuses snow and frost
(they are one coupled column). This is the existing `DirectFrostRuntimeCarry`
plus a snow equivalent, changed from *shadow that is `None`/zero* to
*authoritative mutable lane state advanced each day*. The type substantially
exists; it must be used as state, not plumbing.

### 5.3 One sub-solver call with a narrow typed boundary

```rust
fn advance_winter_column(
    state: &mut WinterColumnState,     // persistent lane state, mutated in place
    forcing: &DayWinterForcing,        // hourly temp/rad/precip/wind/cloud (typed)
    soil: &SoilColumnView,             // layer thickness, theta, conductivity inputs
    residue_depth_m: f64,              // from residue/management state
) -> WinterDayOutcome;                 // typed: infcap_frz, routed_melt, post_winter_rain,
                                       // frost_liquid_delta, watpdg/watbtm, swe/depth + pub operands
```

This is `FrostCouplingOutcome` plus the snow partition, with **typed inputs
instead of symbol reads** and **state mutated in place instead of re-resolved**.
The 24-hour loop and freeze/thaw-front advancement stay inside the function; the
executor sees one typed in, one typed out.

### 5.4 Placement - run once, early, as a forcing preprocessor

Logically the winter column transforms (daily climate + prior winter state) ->
(today's infiltration capacity, liquid forcing, frost liquid deltas). Run it at
the top of the OFE-day, before infiltration/ET and runoff-reconciliation, which
then read `WinterDayOutcome` fields as ordinary typed inputs. The three coupling
re-entries collapse to one solve and several typed consumers - no mid-phase
re-invocation, no symbol surface in the hot loop. (See §6 for the one ordering
question this placement depends on.)

### 5.5 Snow first

Promote `DirectSnowPartition*` to typed lane state plus a typed snow step feeding
`WinterColumnState`; frost then reads snow from shared typed state, not a symbol.
This is the literal R7G unblock and a precondition for frost.

### 5.6 Validation seam

Because the sub-solver has a narrow typed in/out, it is directly shadowable: feed
identical typed forcing and initial state to the compatibility
`compute_active_frost_coupling` (via an adapter) and the direct
`advance_winter_column`, then diff `WinterDayOutcome` and end-of-day state with
`f64::to_bits()`. This is materially easier to validate than re-resolving state
from symbols inside two phases - which is itself an argument for the boundary.

---

## 6. Open Questions and What to Verify

1. **Ordering dependency (verify before implementation).** Frost is currently
   reached from both the ET/infiltration phase and runoff-reconciliation. Before
   committing to "solve once at the top," confirm against legacy ordering whether
   the runoff-reconciliation frost effects depend on intra-day state the
   ET/infiltration phase produces. If independent, a single early solve is
   correct. If dependent, the solver must sit after that dependency, or split
   into a "pre" step (infiltration capacity) and a "post" step (liquid partition)
   sharing the same `&mut WinterColumnState`. **This has not been traced and must
   be resolved first.**

2. **Pure-phase-model exception.** The winter column is a stateful sub-solver
   that owns persistent mutable lane state and runs an internal time loop - a
   deliberate exception to the array-native spec §4.7 pure-phase model. Adoption
   should add this as a named entry in array-native spec §12 (Open Design
   Decisions) - e.g. "stateful sub-solver phases" - rather than pretending frost
   fits the uniform mold.

3. **Contract localization.** The `Qwet`/`frzftp` adjudication (§2.1), the fixed
   `kftill`/`kfutil` constants, and the fine-layer geometry assumptions should
   bind to `SC-SNOWFREEZE-001` at the single sub-solver boundary, not be spread
   across phase fragments. The boundary makes the dead-code decision a one-place
   contract decision.

4. **Snow/frost contract coupling.** `SC-SNOWFREEZE-001` already fuses snow and
   freeze into one contract; the `WinterColumnState` fusion mirrors that. Confirm
   the snow partition invariants (melt bounds, density gates) and frost invariants
   are jointly satisfiable from one coupled state without re-introducing a symbol
   surface.

---

## 7. Relationship to Existing Authority

- **array-native-runtime-specification.md** controls runtime architecture. This
  document is subordinate and proposes a §12 open-design-decision amendment; it
  does not relax any R7 gate, no-compatibility-surface proof, or identity
  requirement. The winter-column sub-solver, if adopted, must still satisfy the
  direct-mode no-compatibility-surface obligations in the hot loop.
- **SC-SNOWFREEZE-001** controls frost/snow science. This document changes no
  physics, guards, units, or conservation obligations. Any `Qwet`/`frzftp`
  decision is a contract amendment under that authority, not a runtime decision.
- **R7 burndown.** This addresses the architecture under
  `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`: snow partition
  first, then frost, both as typed sub-solver state.
- **ADR requirement.** Promotion from Draft/Proposed to ratified authority
  requires either an ADR or the array-native spec §12 amendment in (2) above.

---

## Appendix A - Reference Map

| Concern | Location |
|---|---|
| Dead `Qwet` / `frzftp` override | `wepp-forest_260430_baseline/src/frzng.for:381-394, 410-437` |
| Frost driver, fine-layer/geometry assumptions, authorship strata | `wepp-forest_260430_baseline/src/frostn.for:11-36, 62, 74, 79-82` |
| Compatibility frost gate / solver | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs:372 (resolve), 1892 (compute)` |
| Frost coupling call sites | `hydrology_phase_infiltration_evap.rs:334`; `hydrology_phase_runoff_reconciliation.rs:44`; `support_helpers_mod/runoff_reconciliation.rs:180` |
| Direct typed frost state (carry) | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs:170-201` |
| Direct frost publication projections | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs:50-53, 85+` |
| Direct frost unported markers | `direct_runtime/00_core_frames.rs:250` (`None`); `direct_runtime/normalization.rs:426` (`0.0`) |
| Frost/snow science authority | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` |
| Runtime architecture authority | `docs/architecture/array-native-runtime-specification.md` |
