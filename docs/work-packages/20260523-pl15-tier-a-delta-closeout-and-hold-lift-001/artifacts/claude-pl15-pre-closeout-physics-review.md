# PL15 Pre-Closeout Physics Review (Claude Code)

Status: `complete`
Evidence mode: `Static`
Disposition: `BLOCK lift — or narrow PL15 disposition scope`
Author: Claude Code (review lane per `CLAUDE.md`)
Date: 2026-05-23

Static:
- Read PL10 through PL14 work-package outputs and the current state of
  `crates/openwepp-hillslope-orchestrator/src/lib.rs`,
  `crates/openwepp-input-contract/src/parsers/`, and
  `crates/openwepp-watershed-orchestrator/src/lib.rs`.
- Cross-referenced the implemented kernel surface against
  `/workdir/wepp-forest_260430_baseline/src/*.for` (233 Fortran files; spot
  inventory below).
- Carried forward open items from
  [`claude-pl09-pre-execution-review.md`](../../20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/claude-pl09-pre-execution-review.md).

Ran:
- Nothing executable. No `cargo` gates were invoked. Grep against the live
  source tree was the only command surface used.

## Scope

This artifact reviews the **physics comprehensiveness** of the kernel surface
that PL10–PL14 landed, immediately before PL15 issues a PL08 hold-lift
verdict. The objective is to ensure the PL15 disposition language is
literally true about what the openWEPP engine can and cannot do today.

This review:
- does **not** re-disposition PL08, PL09, or any prior plant-side package;
- does **not** dispute that PL10–PL14 landed substantial typed-scaffold work;
- does ask whether a PL08 hold-lift, on the basis of a PL14 Tier-A replay
  pass alone, would be **honest about what the engine does**.

## Headline

There is **one** production `impl HillslopeKernel` in the workspace:
[`Wb11HydrologyKernel`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs).
It covers six daily-aggregate hydrology phase classes with real
mathematical structure. Every other "kernel" landed in this queue
(`PL12-decomp`, `PL13-growth`) is **typed plumbing without physics
equations**, and the single most important missing kernel —
**infiltration** — is not present at all.

A passing PL14 Tier-A `H5.wat.dat` replay does not establish hillslope
hydrology parity; it establishes that six closure formulas reproduce
legacy *when given identical daily-aggregate inputs from a fixture that
masks every gap below*. PL15 must either narrow its disposition scope
accordingly or remain on HOLD.

## Kernel Gap Register

| id | severity | gap |
|---|---|---|
| `KERNEL-GAP-001` | `critical` | No infiltration kernel; `q_runoff` is bookkeeping over an externally-supplied infiltration value |
| `KERNEL-GAP-002` | `critical` | PL13 growth kernel has typed plumbing only; no GDD/biomass/canopy/phenology/senescence equations |
| `KERNEL-GAP-003` | `critical` | PL12 decomposition kernel has typed plumbing only; no residue-decay kinetics from `decomp.for` |
| `KERNEL-GAP-004` | `critical` | No within-day hyetograph integration; hydrology kernels run once per day on daily aggregates |
| `KERNEL-GAP-005` | `high` | No snow accumulation / snowmelt kernel; `parsers/snow.rs` is an orphan |
| `KERNEL-GAP-006` | `high` | No frost / frozen-soil kernel; `parsers/frost.rs` is an orphan |
| `KERNEL-GAP-007` | `high` | No canopy interception kernel; no analogue to `covcal.for`/`swu.for` |
| `KERNEL-GAP-008` | `high` | No irrigation kernel; `parsers/irrigation_{depletion,fixeddate}.rs` are orphans |
| `KERNEL-GAP-009` | `medium` | No peak runoff calculation (`peak.for`); needed by future sediment coupling |
| `KERNEL-GAP-010` | `acknowledged` | No erosion / sediment kernels — README out-of-scope; PL15 language must reflect this |
| `KERNEL-GAP-011` | `acknowledged` | No channel routing / impoundment kernels — watershed orchestrator has only test/probe `WatershedKernel` impls |
| `KERNEL-GAP-012` | `medium` | CRF-001 still open; all kernel state landed against `BoundarySymbol(String)` — retypable but at growing cost |

## Findings

### `KERNEL-GAP-001` — No infiltration kernel (`critical`)

[`runtime_inputs.rs` / `lib.rs:2065`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L2065):

```rust
let q_runoff = rainfall_input + runon_input - infiltration - depression_storage_delta;
```

`infiltration` is read from the state surface
(`WB12_SYMBOL_INFILTRATION`), not computed. **No kernel in openWEPP
writes it.** WEPP's signature hydrology — Green-Ampt infiltration
integrated against rainfall intensity — is absent. The legacy files
`infpar.for`, `qinf.for`, `purk.for`, and the Green-Ampt machinery in
`watbal.for` are not ported.

Operational consequence: runoff reconciliation is a bookkeeping closure
check, not a hydrology kernel. If the fixture seeds
`WB12_SYMBOL_INFILTRATION` to a legacy-derived value, Tier-A parity is
a tautology (legacy infiltration laundered through openWEPP's adder
against legacy output). If it seeds to `0`, every drop of rain is
runoff.

This is the single largest substantive gap. It must be closed or
explicitly named in the PL15 scope.

Evidence:
- [`lib.rs:84`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L84) — `WB12_SYMBOL_INFILTRATION` defined
- [`lib.rs:2025`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L2025) — infiltration read site
- [`lib.rs:2065`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L2065) — runoff formula degenerates to bookkeeping
- Legacy: `/workdir/wepp-forest_260430_baseline/src/{infpar,qinf,purk,watbal}.for`

### `KERNEL-GAP-002` — PL13 growth kernel has typed plumbing only (`critical`)

[`lib.rs:2654-2760`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L2654-L2760):

- `require_growth_state_surface` **reads** `sumgdd`, `vdmt`, `cancov`,
  `lai`, `rtmass`, `rtd`, `hia` from the state surface and
  **range-validates** them.
- `reset_growth_state_surface` returns **all zeros**.
- The dispatch returns `GrowthPhaseDispatch::Skip` in the default case.

There is no GDD accumulator
(`sumgdd = sumgdd + max(0, (tmax+tmin)/2 - tbase)`), no biomass
partition, no `cancov = f(lai)` derivation, no phenology stage
transitions, no root-depth growth, no senescence kinetics, no harvest
index dynamics. The PL13 acceptance criterion *"state updates"* was
satisfied as *"the code writes to these symbols"* — but the only values
written are pass-through or zero.

**The growth kernel does not grow anything.** The architecture is
correct; the physics is not inside it.

Evidence:
- [`lib.rs:57-63`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L57-L63) — growth symbol constants
- [`lib.rs:2748-2760`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L2748-L2760) — `reset_growth_state_surface` returns zeros
- Legacy: `/workdir/wepp-forest_260430_baseline/src/{grow,gcurve,gdmax,ptgra,ptgrp,grna,growop}.for`

### `KERNEL-GAP-003` — PL12 decomposition kernel has typed plumbing only (`critical`)

PL12 implemented dispatch and typed context construction
(`HillslopeDecompositionKernelContext`,
`HillslopeDecompositionTransitionPayload`,
`HillslopeDecompositionManagementClass`), but the residue-decay kinetics
from `decomp.for` — temperature/moisture-dependent decay rates, pool
transfers between standing/flat/buried residue and root pools,
tillage-induced incorporation, the `resup.for` slot-shift logic at
senescence/harvest/kill transitions — are not implemented.

Same pattern as `KERNEL-GAP-002`: the dispatch is real, the physics
inside is not.

Evidence:
- `lib.rs` decomposition dispatch surface (search for
  `HillslopeKernelPhaseClass::DecompositionTransition` and
  `decomposition_phase_dispatch_for_state`)
- Legacy: `/workdir/wepp-forest_260430_baseline/src/{decomp,resup,rgrcur,rngint}.for`

### `KERNEL-GAP-004` — No within-day hyetograph integration (`critical`)

CLIM03 ported the daily double-exponential event shape — `adapt_no_breakpoint`
builds `timem[]` and `intsty[]` series. But every `Wb11HydrologyKernel`
phase function runs **once per day** on daily-aggregate inputs (soil
water, rainfall, runoff). WEPP's signature physics is sub-daily:
Green-Ampt integrated against the breakpoint hyetograph to produce
peak runoff, time-of-peak, and the time-distribution of detachment.

Without a sub-daily integration loop in the hydrology kernel, the
entire `ip` / `tp` / `mxint` / breakpoint apparatus — including the
`0.70` correction debate that consumed three review rounds — is
decorative. A daily kernel that doesn't consume the within-day
intensity is not WEPP hydrology.

This is structurally coupled to `KERNEL-GAP-001`: real infiltration
requires real within-day rainfall intensity.

Evidence:
- [`lib.rs:1548`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L1548) — `pub struct Wb11HydrologyKernel;`
- [`lib.rs:2217`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs#L2217) — `run_hillslope_phase` (one call per phase per day)
- Legacy: `/workdir/wepp-forest_260430_baseline/src/{disag,dblex,brkpt,watbal_hourly}.for`

### `KERNEL-GAP-005` — No snow kernel (`high`)

`crates/openwepp-input-contract/src/parsers/snow.rs` parses snow control
inputs (`SC-INFILE-SNOW-001`). There is **no** runtime adapter, no
snow phase in the 13-phase graph, no kernel.

Legacy WEPP: `snowd.for`, `melt.for`, `mltbtm.for`, `mlttp.for`,
`winit.for`, `winter.for`, `winthd.for`.

Operational consequence: any climate with measurable winter
precipitation produces wrong water balance by construction — not by
calibration error, by missing equations.

### `KERNEL-GAP-006` — No frost / frozen-soil kernel (`high`)

`parsers/frost.rs` exists; no runtime/kernel consumer. Legacy:
`frostn.for`, `frsoil.for`, `frwatc.for`, `frzng.for`, `frznw.for`,
`getfreezecond.for` — a substantial subsystem.

Frozen soil drastically alters infiltration (often → 0). Without it,
the spring snowmelt + frozen soil → high-runoff event behavior is
absent.

### `KERNEL-GAP-007` — No canopy interception kernel (`high`)

No openWEPP function performs canopy/stem interception of rainfall.
Legacy: `covcal.for`, `swu.for`, and interception in `watbal.for`. The
PL13 plumbing carries `lai` and `cancov` as state symbols but no
kernel consumes them to deduct intercepted rainfall before it reaches
the soil surface.

### `KERNEL-GAP-008` — No irrigation kernel (`high`)

`parsers/irrigation_depletion.rs` and `parsers/irrigation_fixeddate.rs`
are present; no kernel consumes them. Legacy:
`irflow.for`, `irrig.for`, `irinpt.for`, `irprnt.for`, `irs.for`,
`depirr.for`, plus six `fur*.for` furrow-irrigation files (out-of-scope
for the initial port but baseline behavior includes them).

Any irrigated cropland fixture is misrepresented.

### `KERNEL-GAP-009` — No peak runoff calculation (`medium`)

Legacy `peak.for` produces the storm peak runoff rate. Required input
for any future sediment kernelization (detachment is driven by peak,
not mean, hydraulic shear). Not present; will become a blocker for
the wepp-palimpsest sediment program.

### `KERNEL-GAP-010` — No erosion / sediment kernels (`acknowledged`)

`detach.for`, `erod.for`, `depos.for`, `enrcmp.for`, `enrich.for`,
`enrprt.for`, `falvel.for`, `seddia.for`, `sedist.for`, `sedmax.for`,
`sedout.for`, `sedseg.for`, `sedsta.for`, `sloss.for`, `sndrft.for`,
`tfail.for`, `trcoef.for`, `trncap.for`, `trnlos.for`, `xcrit.for`,
`yalin.for` (~20 files) — none ported.

The [README](../../../../README.md) flags sediment routing as
out-of-scope (deferred to the wepp-palimpsest sediment kernelization
program), so this gap is **acknowledged-by-design**, not missed. But
PL15's disposition language must say this explicitly. *"Water Erosion
Prediction Project"* without erosion kernels is a tool of a different
kind, and a hold-lift should not let that read implicit.

### `KERNEL-GAP-011` — Watershed orchestrator has no production kernels (`acknowledged`)

[`crates/openwepp-watershed-orchestrator/src/lib.rs`](../../../../crates/openwepp-watershed-orchestrator/src/lib.rs):
four `impl WatershedKernel` instances — `NominalKernel`,
`PointerProbeKernel`, `RejectKernel`, `PhaseMismatchKernel` — all in
test contexts. Legacy channel/impoundment routing
(`chnero.for`, `chnpar.for`, `chnrt.for`, `chnvar.for`, `hydchn.for`,
13× `imp*.for`, `wshchr.for`, `wshrun.for`, `wshpas.for`, …) is not
present.

Hold-lift scope must be explicit that this is the hillslope CLI
boundary only.

### `KERNEL-GAP-012` — CRF-001 carry-forward (`medium`)

Confirmed: every kernel that landed (Wb11 + PL12/PL13 typed contexts)
is built on `BTreeMap<BoundarySymbol, BoundaryValue>` where
`BoundarySymbol(String)`. ARCH14 ratified moving to typed state
surfaces; no package did so. The lift-queue's own ordering rationale
(#8) records that "PL10/WB10/PL10b execution must preserve
ARCH15/ARCH21 CRF-001/CRF-002 typed-seam closure posture" — but the
posture being preserved is *that the surface is still stringly-typed*.
Each kernel landed grows the eventual retrofit cost.

Note also the existence of a new
`crates/openwepp-climate-runtime-adapter/` crate that I have not
inspected in depth — worth confirming whether it unified the
hillslope+watershed climate duplication I flagged in CLIM03 or adds a
third copy.

## What PL15 has Authority to Declare

If PL15 lifts the PL08 hold, the disposition asserts that openWEPP can
produce a Tier-A `H5.wat.dat`-equivalent at parity with legacy. Based
on the code state above, the **literal** scope of that assertion is:

> *"The daily evapotranspiration, percolation/deep-seepage, lateral
> transfer, drainage, runoff-reconciliation, and storage-reconciliation
> closure kernels in `Wb11HydrologyKernel` produce daily-aggregate
> output that matches the pinned legacy `H5.wat.dat` for the Tier-A
> fixture(s), with growth/decomp state held at fixture-seeded values,
> infiltration supplied externally rather than computed by openWEPP, no
> within-day hyetograph integration, and snow/frost/canopy
> interception/irrigation/erosion/channel-routing physics absent."*

That is a real and useful claim. It is **not** "openWEPP is at hillslope
hydrology parity with legacy WEPP."

PL15 has three honest options:

1. **Narrow the disposition language** to the literal scope above and
   lift the hold with explicit risk-acceptance against
   `KERNEL-GAP-001` through `KERNEL-GAP-008`.
2. **Remain on HOLD** pending closure of `KERNEL-GAP-001` (infiltration)
   and `KERNEL-GAP-004` (within-day integration) at minimum, since
   those two are what make WB11 a real hydrology kernel rather than a
   closure check.
3. **Restructure** the hold-lift verdict as a per-phase-class
   acceptance: ET-OK, perc-OK, lateral-OK, drainage-OK, runoff/storage
   = closure-only (not hydrology), growth/decomp = plumbing-only,
   snow/frost/canopy/irrigation = absent. This is the most informative
   disposition for downstream consumers.

The current PL15 acceptance criterion language ("blocker set empty or
formally risk-accepted under policy") permits any of (1)–(3); what it
does not permit is asserting parity that the kernel set cannot produce.

## Pre-Closeout Preconditions

Before the PL15 disposition is signed:

1. **Diagnose what value `WB12_SYMBOL_INFILTRATION` actually holds**
   during the PL14 Tier-A replay. If it is fixture-seeded from
   legacy, document that as the parity mechanism (and recognize
   parity is then tautological for this surface). If it is `0`,
   `WB11_SYMBOL_DRAINAGE_QDD`-derived, or otherwise non-trivial,
   document the actual source so the comparator result is
   interpretable.
2. **Disclose in PL15's disposition** which kernel gaps from the
   register above are open. Use the `KERNEL-GAP-XXX` ids so the
   register is referenceable from subsequent packages.
3. **Pick disposition path (1), (2), or (3) above explicitly** and
   record the choice.

## What This Review Does Not Address

- The numerical correctness of the WB11 ET / percolation / lateral /
  drainage formulas themselves (their algebraic structure looks
  real; whether the equations match the legacy formulation needs a
  separate per-phase comparison).
- The CLIM03 `0.70` `ip` correction debate (settled in prior review
  rounds; faithful port of legacy `stmget.for:182`).
- The `H5.wat.dat` `structure_diff` from PL09 `GAP-002` — was that
  resolved by PL14 or worked around? Worth a one-line PL15 note.
- Watershed orchestrator scope: out of PL08 hold-lift scope, but
  `KERNEL-GAP-011` records it so downstream watershed work cannot
  start from a false premise about the current state.

## Evidence Links

- [`/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`](../../../../crates/openwepp-hillslope-orchestrator/src/lib.rs)
- [`/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`](../../../../crates/openwepp-watershed-orchestrator/src/lib.rs)
- [`/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`](../../../../crates/openwepp-kernel-contract/src/lib.rs)
- [`/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/snow.rs`](../../../../crates/openwepp-input-contract/src/parsers/snow.rs)
- [`/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/frost.rs`](../../../../crates/openwepp-input-contract/src/parsers/frost.rs)
- [`/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs`](../../../../crates/openwepp-input-contract/src/parsers/irrigation_depletion.rs)
- [`/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs`](../../../../crates/openwepp-input-contract/src/parsers/irrigation_fixeddate.rs)
- [`/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/claude-pl09-pre-execution-review.md`](../../20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/claude-pl09-pre-execution-review.md)
- [`/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`](../../../specifications/science-contracts/contracts/SC-WATBAL-001.md)
- Legacy WEPP physics anchors: `/workdir/wepp-forest_260430_baseline/src/{watbal,infpar,qinf,purk,decomp,grow,resup,ptgra,ptgrp,covcal,swu,snowd,melt,winter,frostn,frsoil,irrig,disag,dblex,brkpt,peak}.for`
