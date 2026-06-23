# Coupled Frost Sub-Solver Architecture Specification

Status: **Ratified** - accepted design authority for the direct-frame
frost/winter port under
[ADR-0026](../decisions/0026-stateful-winter-column-sub-solver.md).
Audience: contributors working on the direct-frame runtime, hydrology kernels,
snow/freeze coupling, and the R7 burndown.
Owner: architecture authority; implementation by Codex work packages.
Subordinate to: [array-native-runtime-specification.md](array-native-runtime-specification.md)
controls runtime architecture; [SC-SNOWFREEZE-001](../specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md)
controls frost/snow science behavior. This document ratifies a runtime *shape*;
it changes no physics.
Evidence mode: static plus R7G package evidence - source reads of the pinned
legacy baseline (`/workdir/wepp-forest_260430_baseline`, commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`), the openWEPP direct runtime, and
`SC-SNOWFREEZE-001`; R7G endpoint/parity evidence from
`docs/work-packages/20260623-r7g-iterative-completion-001/`. No new
simulations were run for this document.
Last updated: 2026-06-23.

---

## 0. Summary

Frost is not a feed-forward water-balance phase. It is a **coupled, stateful,
hourly sub-solver** with long-memory seasonal state, an internal moving
freeze/thaw front, and a hard dependency on the snowpack that insulates it. The
current runtime still shapes frost as a coupling reentered from inside
feed-forward phases and reconstructed from request/symbol surfaces. R7G proved
that this mold is not a viable direct-runtime closure path: direct frost can
execute with zero compatibility-edge counters, but the one-day
`DirectFrostRunoffSurface` path cannot simultaneously preserve fine/shadow
state, avoid corrupt coarse projection, meet protected output parity, and keep
H2637 within the `<=10x` performance gate.

This document records the motivation, the legacy deficiencies that make frost
reconstruction intrinsically hard, and the suitability findings from the current
implementation shape. It ratifies a **winter-column sub-solver boundary**: a
runtime component that orchestrates distinct typed snow and frost sub-states,
owns their shared persistent lane state, advances the 24 hourly frost steps
internally, and exposes typed outputs that the feed-forward phases consume as
ordinary inputs.

The recommendation is to stop porting frost as feed-forward phase fragments.
Promote snow and frost to a stateful winter-column boundary, with explicit
legacy ordering: frost thermal forcing sees the prior snowpack; same-day snow
partition then mutates snow state for downstream liquid forcing and publication.
Snow remains a distinct typed sub-state with independent `SC-SNOWFREEZE-001`
producer obligations and parity gates; frost does not own snow physics.

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

Static: read from the openWEPP hydrology kernels and direct runtime. R7G
evidence: `docs/work-packages/20260623-r7g-iterative-completion-001/`.

### 3.1 Frost is a coupling reentered from multiple feed-forward phase points

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

### 3.3 R7G proved the boundary problem, not just a missing function

| Piece | Status | Evidence |
|---|---|---|
| Typed persistent cross-day state | **Partial** | `DirectFrostRuntimeCarry` can carry front scalars, `frwatc` ledger fields, conductivities, `watpdg/watbtm`, layer shadows, and fine layers. R7G showed that treating this as optional carry rather than canonical lane state drops zero-material fine/shadow state. |
| Typed snow direct path | **Advanced** | R7G added typed snow controls, hourly forcing, snow partition, carry mutation, and publication projection. The remaining blocker is not sidecar-only active snow authority, but snow publication/parity remains unresolved inside the broader R7G protected-output failure set (`Snow-Water`, `RM`, and related operands). |
| Active frost execution | **Partial / unsuitable** | R7G can drive active frost through `DirectFrostRunoffSurface` and `Wb11HydrologyKernel::compute_direct_frost_liquid_partition` with `compatibility_edge_invocations=0`, but that path remains request/symbol-surface backed and not a typed stateful solver. |
| Typed publication projection | **Partial** | Direct publication can emit frost/snow-sensitive operands, but R7G showed that persistent fine/shadow carry must not imply coarse layer projection, and no-material partitions must not strip residual water from WAT `Total-Soil`. |
| Parity/performance closure | **Failed** | R7G held at `HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`: HBP/WAT/PASS parity remained red, and fine-layer carry preservation pushed latest measured H2637 direct endpoints to `188.57-195.27 s` before the final no-material consumer safeguard was measured. |

The old diagnosis "direct carries and publishes frost state but never advances
it" is now too weak. The current direct path can advance frost, but only by
reentering a symbol-surface solver and then projecting its one-day outcome back
into direct state. That architecture repeatedly trades one defect for another:
material-state gates drop fine liquid state, no-freeze echoes can become unsafe
coarse projections, and preserving the state needed for parity regresses the hot
loop.

### 3.4 Snow is the upstream instance of the same problem

Snow was the upstream instance of the same problem. The inherited R7G blocker
was `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`: the snow
partition helper read `snow.runtime_*` symbols off `HillslopeWritebackSurface`
because no typed direct snow partition existed. R7G closed that snow-authority
gap far enough for active snow endpoint execution, but it also clarified the
ordering constraint that any winter-column design must preserve: frost forcing
uses **prior** snow depth and density; same-day snow partition mutates the snow
state afterward.

### 3.5 Suitability verdict

The direct port replicated the compatibility shape - "frost = coupling woven into
ET and runoff, re-resolved from symbols." R7G demonstrated the cost of finishing
it in that mold. Two direct consumers need the same winter state (prior snow,
frost fronts, fine-layer liquid/ice, layer shadows, seasonal thermal context,
and liquid exchange ledger), but the state cannot be safely reconstructed from
coarse layer fields or inferred from current-day frozen material. R7F also
deliberately cut the hot loop off from compatibility symbol surfaces. The mold
fights both the physics and the direct-runtime architecture. The required change
is a boundary change, not more input plumbing.

---

## 4. Suitability Assessment - Why a Sub-Solver Boundary Fits

| Frost requirement (from §1) | Feed-forward phase fit | Coupled sub-solver fit |
|---|---|---|
| Internal hourly time integration | Poor - the loop is hidden inside a "pure" phase | Good - the loop is the solver's body, invisible to the executor |
| Long-memory seasonal state | Poor - day frame is not the owner | Good - persistent lane state, mutated in place |
| Snow+frost coupling | Poor - two subsystems re-resolved separately | Good - one winter column orchestrates both typed sub-states |
| Feedback loops | Poor - not expressible as borrow edges | Good - internal to the solver |
| Two-point hydrology coupling | Poor - drives the double reentry | Good - one solve, typed outputs read by both consumers |
| Contract-decision localization (§2.3) | Poor - decisions smeared across phase fragments | Good - one typed boundary to bind `SC-SNOWFREEZE-001` |
| Independent validation | Poor - state re-resolved from symbols inside phases | Good - narrow typed in/out is directly shadowable |

The sub-solver shape matches every property where the feed-forward phase model
is a poor fit. This is the architecture-shaped decision underneath the frost
grind.

---

## 5. Ratified Boundary

This is a ratified runtime boundary. It changes runtime shape only; physics
remains governed by `SC-SNOWFREEZE-001`.

### 5.1 Reframe

Move frost from "coupling calls inside ET and runoff" to a coupled
**winter-column** sub-solver. The sub-solver orchestrates persistent snow and
frost lane state, advances the internal frost hourly loop, preserves snow/frost
ordering, and emits typed outputs that the feed-forward phases consume as
ordinary inputs. It is allowed to be internally iterative and stateful; the outer
direct executor still sees a typed day-level producer.

### 5.2 State ownership - promote carry to lane authority

Introduce a lane-owned `DirectWinterColumnState` with snow state plus a rich
`DirectFrostLaneState`. `DirectFrostRuntimeCarry` is evidence for the fields
that matter, not the final ownership model. The final state must be canonical
even when current-day frozen material is zero. `DirectWinterColumnState` is the
sole authoritative persistent lane state after a winter-column call; returned
outcomes can expose diagnostics, projections, deltas, and immutable snapshots,
but not a second persistence authority.

Minimum `DirectSnowLaneState` responsibilities:

- prior snowpack snapshots: SWE, depth, density, age/settle-day count, and any
  direct-runtime albedo/coverage controls needed by `SC-SNOWFREEZE-001`;
- same-day partition ledger: retained rain, released rain, `wmelt`, `S`, `RM`,
  routed melt, post-winter rain, and closure residuals;
- publication operands: `Snow-Water`, SWE/depth/density snapshots, and manifest
  metadata needed for HBP/WAT/PASS/loss/plot reconstruction;
- explicit prior-vs-post views so frost thermal forcing reads prior snow while
  downstream liquid and publication consumers read post-partition snow.

Minimum `DirectFrostLaneState` responsibilities:

- front and thaw scalars: `dfrost`, `dthaw`, `frdp`, `thdp`, `tfrdp`, `tthawd`,
  `fgthwd`, `nft`, `ws_frz`, `infcap_frz`;
- fine-layer state: `fgfrst`, `slfsd`, `slsic`, `slsw`, `sltime`, including
  zero-material heterogeneous liquid state;
- layer shadow/exchange state: `st`, `yst`, `nwfrzz`, `soilf`, `frzw`,
  `frozen_depth`, and total liquid/frozen accounting;
- liquid exchange ledger: `frwatc` before/after, freeze debit, thaw credit,
  net liquid delta, `watpdg`, `watbtm`, and closure residual;
- thermal context inputs or cached controls needed to advance the next day
  without reading a symbol surface.

Hard invariant: persistent fine/shadow carry is not coarse layer projection.
Coarse layer mutation is emitted only when the sub-solver explicitly closes a
liquid/frozen storage exchange for the day.

### 5.3 One sub-solver call with a narrow typed boundary

```rust
fn advance_winter_column_day(
    state: &mut DirectWinterColumnState,
    forcing: DirectWinterDayForcing,
    soil: DirectFrostSoilColumnInputs,
    residue: DirectResidueThermalInputs,
) -> Result<DirectWinterDayOutcome, DirectRuntimeError>;
```

`DirectWinterColumnState` is the only persistent authority. The function mutates
it in place. `DirectWinterDayOutcome` must not contain an alternate end-of-day
state channel; when validation needs state evidence, expose immutable snapshots
or digest/projection fields that are explicitly non-authoritative.

`DirectWinterDayOutcome` must separate concerns that the current retrofit
confuses:

- non-authoritative state snapshots or diagnostics for validation;
- frost pre-runoff operands: frozen infiltration capacity, frost depth,
  frozen-water total, and front diagnostics;
- snow/liquid operands: post-winter rain, routed melt, SWE/depth/density after
  snow partition, and liquid hyetograph scaling inputs;
- storage operands: `frwatc_net_liquid_delta_m`, optional coarse layer
  mutations, and independent closure fields;
- downstream operands for R4A/R4G/R4PQZ and publication operands for
  HBP/WAT/PASS/loss/manifest reconstruction.

The 24-hour loop and freeze/thaw-front advancement stay inside the function.
The executor sees typed inputs, typed state mutation, and typed outputs; it does
not see `BTreeMap<BoundarySymbol, BoundaryValue>` or
`HillslopeKernelRequest` in the production hot loop.

### 5.4 Placement - one winter-column producer with staged outputs

Run the winter-column producer once per OFE-day, before feed-forward hydrology
consumers. The producer may internally stage its work:

1. Build typed forcing and soil-column inputs from the direct day/lane frame.
2. Advance frost thermal state using the **prior** snow depth and density.
3. Compute snow partition and same-day liquid forcing.
4. Emit the frost and snow operands consumed by infiltration/ET,
   runoff-reconciliation, storage reconciliation, downstream transfer, and
   publication.

The follow-up implementation package must complete and record this source trace
before locking the final API shape. If runoff-reconciliation needs a post-ET
frost liquid partition, split the API into
`prepare_winter_column_pre_hydrology` and
`finish_winter_column_post_hydrology`, both sharing the same
`&mut DirectWinterColumnState`. Do not reintroduce symbol-surface reentry.

### 5.5 Snow authority before frost closure

Snow state must be typed before frost can close because frost thermal forcing
depends on snow depth and density. That does not mean same-day snow mutation
runs before the frost thermal solve. The legacy ordering captured by R7G is:
frost sees prior snowpack, then same-day snow partition mutates snow state for
liquid forcing and publication.

### 5.6 Validation seam

Because the sub-solver has a narrow typed in/out, it is directly shadowable: feed
identical typed forcing and initial state to the compatibility
`compute_active_frost_coupling` (via an adapter) and the direct
`advance_winter_column_day`, then compare `DirectWinterDayOutcome` and
end-of-day state snapshots under `SC-SNOWFREEZE-001` semantic parity and named
tolerances unless a field is explicitly declared bit-exact. Publication gates
remain stricter: byte/Arrow identity and metadata parity for HBP/WAT/PASS/loss,
plot, and manifest outputs. The adapter is a test/comparator boundary only.
Production direct code must not invoke map-backed request surfaces in the hot
loop.

---

## 6. Design Decisions and Verification Gates

1. **Ordering dependency.** R7G established one ordering invariant: frost forcing
   sees prior snow depth and density. The follow-up implementation must still
   trace whether freeze/thaw liquid partition depends on state mutated by
   ET/infiltration before runoff-reconciliation. If it does, split the
   winter-column API into pre/post steps over one `&mut DirectWinterColumnState`.
   If it does not, one early day-level producer is sufficient. This trace is an
   implementation entry gate, not a clean-up item: record the decision before
   building beyond skeleton types and adapters.

2. **Pure-phase-model exception.** The winter column is a stateful sub-solver
   that owns persistent mutable lane state and runs an internal time loop - a
   ratified exception to the array-native spec §4.7 pure-phase model under
   `ADR-0026`. Array-native direct execution still sees a typed day-level
   producer; the exception is internal state ownership and hourly iteration, not
   permission to reintroduce compatibility surfaces.

3. **Contract localization.** The `Qwet`/`frzftp` adjudication (§2.1), fixed
   `kftill`/`kfutil` constants, fine-layer geometry assumptions, bounded
   canonicalizations, and invalid-state guards must bind to
   `SC-SNOWFREEZE-001` at the sub-solver boundary. Do not spread those decisions
   across R4A/R4G/R4PQZ fragments.

4. **No-material carry invariant.** Zero current-day frozen material is not
   zero frost state. Heterogeneous fine-layer liquid and layer-shadow state must
   persist across no-freeze/no-material days and must be independently
   reconstructable.

5. **Projection invariant.** Fine/shadow carry is not coarse layer mutation.
   Coarse layer mutation requires an explicit liquid/frozen storage exchange and
   closure ledger. Publication may read fine/shadow state, but it must not
   back-fill active-water-only `theta` into total soil water.

6. **No-compatibility hot-loop gate.** Production direct execution must not call
   `DirectFrostRunoffSurface`, `HillslopeKernelRequest`,
   `HillslopeWritebackSurface`, compatibility WB13 rows, or map-backed symbol
   helpers for frost. The measurable source gate is: no production references to
   `DirectFrostRunoffSurface`, `BoundarySymbol`, `BoundaryValue`,
   `HillslopeWritebackSurface`, or `HillslopeKernelRequest` in the winter-column
   hot path, except named test/comparator adapters.

7. **Closure gates.** The follow-up package must rerun the R7G gates from the
   new architecture: H2637 direct default `<=10x` legacy, HBP/WAT/PASS/loss/plot
   parity, manifest metadata parity, `compatibility_edge_invocations=0`, source
   scans for no symbol-surface authority using the identifiers in gate (6),
   anti-alias fixtures, and independent operand reconstruction.

---

## 7. Relationship to Existing Authority

- **array-native-runtime-specification.md** controls runtime architecture. This
  document is subordinate and ratifies the ADR-0026 winter-column exception; it
  does not relax any R7 gate, no-compatibility-surface proof, or identity
  requirement. The winter-column sub-solver must still satisfy the direct-mode
  no-compatibility-surface obligations in the hot loop.
- **SC-SNOWFREEZE-001** controls frost/snow science. This document changes no
  physics, guards, units, or conservation obligations. Any `Qwet`/`frzftp`
  decision is a contract amendment under that authority, not a runtime decision.
- **R7 burndown.** This addresses the architecture under
  `HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`: active snow authority advanced,
  active frost can execute, but frost requires a stateful typed sub-solver before
  R7G parity/performance closure is honest.
- **ADR-0026.** Ratifies this specification as the accepted runtime shape for
  R7G snow/frost hold-lift work and the deletion path for the current direct
  snow/frost retrofit.

---

## Appendix A - Reference Map

| Concern | Location |
|---|---|
| Dead `Qwet` / `frzftp` override | `wepp-forest_260430_baseline/src/frzng.for` (`frzftp`, migration gate, Eq. [3.8.4] block) |
| Frost driver, fine-layer/geometry assumptions, authorship strata | `wepp-forest_260430_baseline/src/frostn.for` |
| Compatibility frost gate / solver | `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs` (`resolve_active_frost_coupling`, `compute_active_frost_coupling`) |
| Frost coupling call sites | `hydrology_phase_infiltration_evap.rs`; `hydrology_phase_runoff_reconciliation.rs`; `support_helpers_mod/runoff_reconciliation.rs` |
| Current direct frost state/carry | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs` (`DirectFrostRuntimeCarry`) |
| Current direct frost publication/projection surfaces | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`; `direct_runtime/runoff.rs`; `direct_publication/day_input_and_helpers.rs` |
| R7G HOLD evidence and handoff | `docs/work-packages/20260623-r7g-iterative-completion-001/` |
| Frost/snow science authority | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` |
| Runtime architecture authority | `docs/architecture/array-native-runtime-specification.md` |
