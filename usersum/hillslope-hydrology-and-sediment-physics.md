# Hillslope Hydrology and Sediment Physics in openWEPP

*Version 0.2 — 2026-07-09*

*Audience: hydrologists and erosion modelers evaluating how openWEPP
represents hillslope runoff and sediment transport, and how that
representation relates to legacy WEPP.*

openWEPP is built on WEPP's process science. Its infiltration, water
balance, plant growth, residue, winter processes, and erosion mechanics
descend from the WEPP hillslope model documentation (Flanagan & Nearing,
1995) and from the forest-hydrology lineage that extended it (Dun et al.,
2009; Srivastava et al., 2013, 2017). It is not a port. Where the legacy
representation is a set of process equations, openWEPP implements those
equations; where the legacy representation is a numerical construction
shaped by the computing constraints of the early 1990s — most consequentially
the treatment of multi-element hillslopes — openWEPP replaces the
construction while keeping the physics.

This document describes the resulting hydrology and sediment model: the
overland-flow routing that replaced WEPP's equivalent-plane construction,
the shock-capturing solver underneath it, the hourly hydraulics that now
drive the sediment computation, the space–time discretization evidence
behind the production defaults, and the groundwater-baseflow component.
Throughout, it compares openWEPP behavior to legacy WEPP where a
like-for-like comparison exists — and says plainly where none does. That
second point frames everything, so it comes first.

---

## 1. What "based on WEPP" can and cannot mean

Legacy WEPP is a research code with more than thirty years of accumulated
modification. Reading its source — which the openWEPP project does
extensively, against a pinned snapshot — establishes what individual
routines compute in isolation. It does not establish what the model
computes in composition, because the code base carries the sediment of its
own history: routines that exist but are disabled by input-file gates that
no documentation describes; numerical guards commented with phrases like
"prevent model bomb"; compensating patches added decades after the routines
they compensate; at least one water-balance cap deployed and then
withdrawn with the note that it was "another band-aid"; and an hourly
water-balance pathway maintained for forest applications that the
originating laboratory's mainline agricultural use never exercised. The
maintainers of the forest-adapted lineage record this candidly in their own
working notes: years of production runs carry undocumented water-balance
closure debt.

None of this is a criticism of WEPP. It is the normal condition of a
long-lived scientific code, and WEPP's process science — the reason
openWEPP exists — is not diminished by it. But it has a hard epistemic
consequence: for most processes, nobody can state precisely what legacy
WEPP computes, and source-code access does not change that. A model whose
composed behavior is uncharacterized cannot serve as a correctness oracle
for a successor.

openWEPP therefore inverts the usual porting relationship. Correctness
authority lives in written specifications with named tolerances, and in
conservation: every routed water and sediment surface closes a mass ledger,
enforced as a hard failure at run time, not as a diagnostic. Agreement with
legacy WEPP is used as a flag — a divergence is a reason to investigate,
with the specification deciding which side is wrong — never as an
acceptance target. This posture was not adopted from purity. Early in the
project, divergences from legacy output were twice traced to comparison
artifacts (a depth-versus-water-equivalent unit mismatch, a raw-versus-released
forcing mismatch) rather than to model defects, and one legacy behavior
initially treated as a physics reference turned out to be a pipeline
intermediate. Matching an uncharacterized target does not make a model
right; conserving mass and satisfying a written contract can be checked.

A second ordering principle follows from the same logic: closure first,
magnitude last. Structural properties — conservation, boundedness,
timing — are validated before absolute magnitudes are judged, so that
magnitude error is never silently absorbed into structural error. Where a
magnitude has no external authority yet, this document says so.

## 2. The process basis retained from WEPP

The hillslope water balance is WEPP's: Green–Ampt Mein–Larson
infiltration, layered soil-water accounting with evapotranspiration,
percolation, and the forest adaptations of deep percolation and lateral
subsurface flow introduced in WEPP v2008.9 (Dun et al., 2009). Winter
processes (snow accumulation and melt, soil frost) are re-derived from
their published descriptions and validated against field observations;
they are documented separately in
[Snow and Frost in openWEPP](snow-frost-modeling-and-validation.md).

Two departures from legacy practice matter for reading everything below.

First, openWEPP's core water balance runs hourly, universally. Legacy WEPP
partitioned several core processes by land use as a workaround for
unresolved problems — frost disabled for non-agricultural soils, the hourly
pathway gated behind an undocumented flag — and openWEPP treats a partition
around a symptom as a defect to repair rather than a behavior to
replicate. Infiltration, water balance, frost, snow, percolation, and
erosion are universal physics in openWEPP; land use gates only genuinely
landuse-specific processes.

Second, the erosion mechanics retained from WEPP — interrill sediment
delivery, rill detachment by excess shear stress, transport capacity, and
the steady-state sediment continuity equation — are solved on a different
hydraulic substrate, described in Sections 5–7.

## 3. The multiple-OFE problem in legacy WEPP

A WEPP hillslope is a sequence of overland flow elements (OFEs), each with
its own soil, slope, and management. Legacy WEPP does not route unsteady
flow between them. Instead, when flow is continuous across a chain of
OFEs, it collapses the chain into a single *equivalent plane*: the
effective flow length accumulates the member slope lengths, infiltration
parameters become length-weighted averages over the accumulated plane, and
each OFE hands its outflow to the next as an inflow boundary value. The
discharge at any element is reconstructed as the product of a peak runoff
rate and the accumulated effective length, and results are referenced to
the terminal OFE — the outlet of the equivalent plane.

The legacy WEPP referenced here — and everywhere this document compares
against legacy behavior — is specific: openWEPP pins the forest-adapted
`wepp-forest` branch maintained by Anurag Srivastava as its legacy
reference for source reading and comparison. That branch is the working
continuation of the forest lineage (Dun et al., 2009; Srivastava et al.,
2013, 2017) and the engine WEPPcloud deploys; it shares the
equivalent-plane construction with mainline WEPP, and its working record
is the source of the multi-OFE evidence quoted below.

Given the constraints under which WEPP was built, this was a sensible
design. The model is steady-state within a storm; solving one equivalent
plane per event yielded a continuous soil-loss profile along the slope and
sediment delivery at the toe for the cost of a single profile solve, on
computers where an unsteady multi-element routing at event resolution was
not a realistic budget. For the short hillslopes and modest element counts
WEPP was designed and tested around, the construction behaves.

Its failure mode appears as element counts grow, and it is structural
rather than arithmetic. The equivalent plane has no relief valve along the
cascade: accumulated upslope runoff arrives at the terminal element
undiminished, and the reported per-element runoff depth is normalized by
the terminal element's own (small) length while carrying the whole plane's
accumulated flow. On a 19-OFE hillslope this normalization alone amplifies
the terminal element's reported runoff depth by roughly the ratio of total
slope length to terminal-element length — about a factor of 11 on the
geometry the openWEPP project examined — and on hillslopes with more than
ten OFEs, daily water-balance closure residuals exceeding 1000 mm at the
terminal OFE are observed in the `wepp-forest` working record. The sediment side is not the amplifier: the
transport-capacity coupling is sub-linear in effective length and bounded
several ways in the code. It is the victim — sediment yields computed from
a non-physical hydraulic state.

The legacy record around this behavior is instructive. It contains a
consumer-side rescaling patch added as recently as 2026, a state-side cap
deployed and withdrawn, and an internal register that classifies the
terminal-element problem as a model-class limitation whose recommended
disposition is "route those use cases to a different model." That is the
correct diagnosis: the equivalent plane cannot be patched into an unsteady
router, because the missing physics — water as a conserved state moving
between elements — is exactly what the construction was designed to avoid
computing. Hillslopes beyond about ten OFEs are outside legacy WEPP's
tested envelope, and this is why.

## 4. The Papanicolaou reformulation: routing OFE by OFE

The replacement openWEPP adopts comes from within the WEPP lineage.
Papanicolaou et al. (2018) — with WEPP's own architects among the
authors — reformulated hillslope overland flow as an unsteady, spatially
explicit routing: the one-dimensional kinematic wave equation solved on
each OFE, with the upstream OFE's outflow hydrograph entering the
downstream OFE as a boundary condition, and with flow resistance composed
per time step from the local surface state (Section 6). The framework was
demonstrated against flume and field cases but was never incorporated into
mainline WEPP. openWEPP implements it as the production overland-flow
router; the subsystem is named Lane D in run manifests and diagnostics,
and that name is used below.

The reformulation eliminates the multi-element blowup by construction
rather than by tuning. Water on the hillslope is a conserved state on a
computational mesh; what crosses each OFE boundary is an actual flux;
every element infiltrates, stores, and releases water on its own soil and
slope; and no quantity is ever re-referenced to a terminal element's
footprint. Mass conservation is therefore checkable — and in openWEPP,
enforced — cell by cell and day by day. Three independent ledgers close on
every active run: the router's internal cascade balance, a cross-check of
router-booked inflow against soil-released runoff (made exact by the
hourly forcing construction), and the assembled hillslope-day water
balance. On the 19-OFE steep-forest demonstration hillslope — chosen
deliberately to be far past the legacy envelope — a two-year simulation
routes 610 event days with all three identities closing at machine
roundoff (residuals of order 1e-13 in relative terms), where the legacy
construction on the same geometry produces the order-11× terminal
amplification described above.

Because the target hillslopes are subsurface-dominated (steep forested
soils can route the large majority of yield as lateral subsurface flow),
activation of the router is contractually coupled to the subsurface: return
flow and saturation excess enter the kinematic wave as source terms, and
subsurface export that does not exfiltrate is conserved and exported rather
than dropped. A surface-only router would miss the dominant pathway on
exactly the hillslopes openWEPP prioritizes.

## 5. The TVD–MacCormack solver, and how it differs from WEPP's hydraulics

The kinematic wave equation on each OFE is solved with the explicit
TVD-corrected MacCormack scheme of the Papanicolaou framework: a
second-order predictor–corrector with a symmetric total-variation-
diminishing dissipation term appended to the corrector, following
García-Navarro et al. (1992) and Mingham et al. (2001), with the
dissipation construction originating in Davis (1984) and the applied
kinematic-wave usage following Tseng (2010). The scheme is shock-capturing:
it resolves the steep wave fronts that form when runoff from a
high-resistance element enters a low-resistance one — precisely the
regime multi-element hillslopes create and steady-state constructions
cannot represent. Time stepping is CFL-limited (Courant number 0.9) with
sub-hourly substeps capped at 300 s in production.

Two implementation details are worth a hydrologist's attention because
they illustrate how the specification-first method works in practice.

The published limiter branch in Papanicolaou et al. (2018, their eq. 11c)
is a transcription error. Both sources that equation cites — Davis (1984,
eq. 3.20) and Mingham et al. (2001, eq. 31f) — agree the flux-limiter
branch is `phi(r) = min(2r, 1)` for `r > 0` and `0` otherwise; the printed
form swaps the conditions. openWEPP adjudicated the discrepancy against
the primary sources and binds the corrected branch. A model built by
matching a reference implementation would have inherited whichever branch
that implementation happened to contain; a model built against sources has
to notice.

Depth positivity is preserved inside the numerics rather than by clamping.
An early production candidate passed its conservation gates on most
geometries but, on one Cascades hillslope, generated material negative-depth
clamp mass — water created by the limiter to hide a solver artifact. The
fix was conservative stage-face limiting within the predictor–corrector
flux update; after it, total clamp mass on the offending hillslope fell to
the 1e-13 m³ range (roundoff) over multi-year runs, and a standing guard
fails any run whose clamp mass becomes material relative to its source
water. The episode is why openWEPP treats "the ledger closes" and "the
ledger closes without a hidden compensating term" as different gates.

The contrast with legacy WEPP is categorical rather than incremental.
Legacy WEPP's hillslope hydraulics reduce each event to two numbers: a
peak runoff rate, and an effective duration chosen so that their product
conserves the event's runoff volume. The flow field everything downstream
consumes is then the steady-state kinematic profile that peak discharge
would produce on the equivalent plane — the depth and shear distribution
of a storm that rises instantly to its peak, holds it for the effective
duration, and stops. There is no time-marching solution, no wave
propagation, no mesh, and no spatial discretization decision exposed to
the user.

Three fidelity consequences follow. The hydrograph's shape and timing are
discarded: rising and falling limbs, arrival time at the toe of the
slope, and the overlap of successive bursts within a day do not exist in
the representation, so nothing downstream can respond to them. Every
discharge-dependent quantity is evaluated at a single flow state:
hydraulic shear and transport capacity are strongly nonlinear in
discharge, and the time integral of a nonlinear function over a varying
hydrograph is not that function evaluated at the peak and held for an
equivalent duration — the approximation degrades exactly as hydrographs
become peaked and slope properties heterogeneous. And elements never
interact in time: a downstream element sees a steady inflow, never the
steepening front that forms when fast water from a smooth element enters
a rough one — the shock regime the TVD-MacCormack scheme exists to
capture. The unsteady solution computes depth and discharge at every cell
and substep, so limbs, arrival times, attenuation, and front steepening
emerge from the solution rather than being posited, and the sediment
physics of Section 7 can be evaluated hour by hour on the flow state that
actually occurred. The two models do not disagree about how to solve the
same equations — they solve different problems, of which the unsteady one
contains the steady one as a limit.

Solver validation follows the published framework's own cases plus
analytic oracles: the four Papanicolaou validation cases with their
published Nash–Sutcliffe efficiencies (bare plot 0.91, rock fragments
0.75, vegetation patchiness 0.87, and the concave-profile shock case 0.88
built on Iwagaki's (1955) characteristics solutions), a rising/falling
analytic ladder, exact booked-ledger conservation, and a total-variation
transient bound. These are regression-gated: every solver change re-runs
the ladder.

## 6. State-dependent flow resistance and the local Newton solve

In legacy WEPP, Darcy–Weisbach friction is a constant per element, so the
kinematic rating `q = alpha * h^(5/3)` has a closed-form coefficient and
the hydraulics never iterate. The Papanicolaou framework replaces the
constant with an additive friction composition evaluated every time step
from the local flow state: grain/raindrop resistance (Shen & Li below
Reynolds number 1000, Hirsch above), form resistance, Froude-gated wave
resistance, and vegetation drag, summed to an equivalent friction factor.
Resistance responding to Reynolds and Froude number is what lets a single
hillslope carry laminar sheet flow at its crest and roughness-dominated
concentrated flow at its toe within one storm — but it makes the rating
implicit: `alpha` depends on the discharge it helps determine.

openWEPP's production solver resolves the implicit rating with a bounded
Newton iteration per cell per substep, using analytically differentiated
celerity (implicit differentiation of `q = alpha(q, h) * h^1.5`) in place
of finite-difference perturbation. Convergence and derivative-validity are
guarded; a cell that cannot converge fails the run rather than silently
falling back. This replaced an earlier fixed-point implementation and cut
the routed two-year, 19-OFE demonstration run from 37.5 s to 11.9 s of CPU
time (3.15×) with the change governed by contract: no bit-identity claim,
but named fidelity deltas plus the full oracle ladder and exact
conservation as acceptance.

Against legacy WEPP, the honest comparison is not "faster" — legacy never
performs this computation at all. The relevant statements are that
resistance in openWEPP is a state variable with published applicability
regimes rather than a calibration constant, and that the numerical cost of
making it one has been engineered down to where it is affordable at fleet
scale (millions of hillslope simulations per year).

## 7. Hourly hydraulics as the sediment substrate

Legacy WEPP's erosion component solves steady-state sediment continuity —
interrill delivery plus rill detachment or deposition, capped by transport
capacity — once per event, at the peak discharge and an effective
duration, along the equivalent plane (Flanagan & Nearing, 1995). openWEPP
keeps that continuity equation and its detachment/deposition/transport
physics, and changes what drives it.

openWEPP's water balance produces hourly runoff: infiltration excess,
saturation excess, and routed snowmelt compose an hourly source series for
each OFE. The erosion solve runs per hydraulically active hour on that
series — each hour with flow gets its own continuity solution (a
Runge–Kutta detachment integration with analytic deposition handling, in
the same normalized-distance space as the WEPP formulation), with rill
geometry carried sequentially through the day. When Lane D routing owns
the surface-water path, the hourly shape that drives this solve is the
routed outlet hydrograph itself: the router's outlet series, mapped to
hourly weights by mass-conserving sums (late-draining mass past hour 24 is
folded into the final hour explicitly and surfaced per run — never
silently truncated). A missing or non-conserving routed shape is a hard
run failure, not a fallback.

On multi-element hillslopes the sediment computation chains OFE to OFE the
way the water does: each element's hourly outflow discharge, exported
sediment discharge, and particle-class fractions become the next element's
inflow, with each element's detachment and transport parameters derived
from its own soil, slope, and management. A locally dry element with
upstream inflow still solves — as deposition of the routed load, the case
the equivalent plane could not express. This capability has no legacy
counterpart to compare against: in the pinned legacy forest baseline, the
multi-OFE erosion pathway emits no sediment at all, so openWEPP's chained
multi-element sediment predictions are new model behavior, validated by
conservation (a telescoped mass balance across the chain, enforced on
every solved increment) rather than by reference matching.

Where a matched comparison with legacy *is* possible — single-OFE
hillslopes, per-event, at identical width and delivery cut-points — the
project ran it, and the result is a case study in comparator-as-flag. The
initial comparison showed openWEPP over-detaching by roughly a factor of
six. The water was ruled out first (50-year runoff volume within about 1%
of legacy on the paired fixture), structure was ruled out next (every
conservation closure held), and the defect localized to an operand pathway:
declared ground-cover fractions from the management input were not
reaching the erosion cover operands, which were instead deriving cover
from near-zero residue mass. With the cover pathway repaired, the dominant
event on the primary fixture computes 3.97 kg/m of delivery against
legacy's 4.2 kg/m (within ~6%, consistent with the remaining water-side
difference), and a two-element fixture's outlet detachment stands at
17.4 kg/m/yr against legacy's ~19.4 kg/m/yr. Residual small-event
divergences are attributed and bounded (a peak-rate operand difference on
trace events), not open.

One adjudication in this area remains genuinely open and is recorded as
such: under active routing, the routed hydrograph currently supplies the
*timing and shape* of the hourly erosion solve, while the water *magnitude*
operands remain the element's own water balance. Whether transport should
additionally consume routed water magnitude — which would couple erosion
on downstream elements to accumulated upstream flow — is a physics
decision with a written contract gate, deliberately not defaulted either
way. Users should read multi-element sediment magnitudes with that status
in mind.

## 8. The space–time problem: choosing the mesh, and what it buys

An unsteady router forces a decision legacy WEPP never had to make: the
spatial and temporal discretization. openWEPP's first instinct — a fixed
cell count per OFE — turned out to be a category error caught during
ratification: ten cells means 2.6 m resolution on one hillslope's short
elements and 30 m on another's single long element. The production policy
is therefore a target cell size: each OFE is meshed at
`ceil(length / 5.0 m)` cells, floored at 10 and capped at 4096, with the
300 s substep cap.

The 5 m figure was not asserted; it was ratified against a refinement
ladder on a cohort of real hillslopes (Pacific Northwest forest, Midwest
row-crop, northern-Idaho geometries), and the ladder's design carries the
useful lessons. Candidate meshes (5 m) were judged against finer references
(2.5 m and 1.25 m), with the reference itself required to prove adequacy —
one further halving must move every judged surface by no more than a third
of that surface's tolerance. The judged surfaces are daily routed outlet
mass (tolerance ≤ 1% relative), hourly hydrograph shape (maximum L1
distance ≤ 0.05), annual sediment sums (≤ 2%), and end-of-window storage
accounting (≤ 1%). And the evidence had to be *coupled* in space and time,
because the first pass produced a false failure: one cropland hillslope
missed the shape-adequacy bar at fine spatial resolution, and attribution
showed the miss came from the 300 s substep interacting with the fine
mesh, not from the mesh — refining time to 75 s collapsed the shape error
from 0.021 to 0.003. Spatial convergence judged at a fixed time step can
indict the wrong axis. A similar lesson came from the annual sediment
gate, where a strict relative metric was failed by a single near-zero-yield
day perturbing one year's total; the ratified metric judges material years
and the annual vector rather than letting a trace year veto.

The comparison with legacy WEPP here is asymmetric in an important way.
openWEPP's discretization error is measurable, was measured, and is bounded
by the tolerances above; a user who needs better than the production
default on a specific hillslope can run the diagnostic selectors and see
the delta. The corresponding error in legacy WEPP — the distance between
an equivalent-plane steady state and the unsteady flow field it stands in
for — is structural, unmeasurable from within the model, and grows with
element count and heterogeneity in ways Papanicolaou et al. (2018)
characterize by flow regime: pronounced where roughness varies along the
slope and stream power is high, modest on short uniform planes. On short
uniform hillslopes the two models should and do tell similar stories;
that is the regime WEPP was validated in, and openWEPP does not claim it
was wrong there.

## 9. Production defaults: when a run uses this path

As of contract revisions ratified in July 2026, Lane D routing with the
hourly sediment substrate is openWEPP's default production path for both
single-OFE and multi-OFE hillslopes — conditionally, on input authority:

- Runs whose management inputs are openWEPP-native files (YAML, declaring
  data version `ow-lanuse-1` or later) with complete embedded routing
  coefficients for every scheduled OFE route actively by default, no
  selector required.
- Runs on legacy-format inputs execute the legacy-compatible path,
  unchanged and byte-identical to prior behavior.
- Mixed authority — some elements with native coefficients, some
  without — refuses to run rather than guessing.
- An explicit disable selector forces the legacy-compatible path even on
  eligible inputs, as a rollback lever.

The routing coefficients in question are the five static resistance
operands of the friction composition — the skin-friction coefficient, the
form-drag coefficient, roughness-element height (m), roughness
concentration, and the vegetation drag coefficient — which the dynamic
terms (rainfall intensity, leaf area index, canopy height) join at run
time from the live water-balance and plant state. openWEPP deliberately does not
synthesize these from legacy cropland management fields: the legacy
friction quantities aggregate different physics, no deterministic bounded
mapping to the five operands exists, and fitting one against a single
fixture was considered and rejected. A migration tool produces native
management files instead. The practical consequence for users: the routed
path is never entered with invented resistance parameters, and the answer
to "which path did my run take" is recorded in the run manifest rather
than inferred.

### Default routing coefficients by disturbed class

For runs built through WEPPcloud, the producer of the management file is
the Disturbed parameterization, and each OFE carries a *disturbed class* —
a land-cover type crossed with a disturbance state, descending from the
USFS Disturbed WEPP interface: forest, shrub, and grass covers with
prescribed-fire and low/moderate/high burn-severity variants, skid trails,
mulch treatment, thinning, agriculture, and bare soil. When native
openWEPP output is selected, the producer writes that class's five routing
coefficients into the management file. openWEPP itself applies no
defaults: it reads only the explicit per-element values in the file, so
declared, overridable inputs are the only way onto the routed path.

The class defaults (values dimensionless except `D_r` in meters):

| Disturbed class | `k_o` | form `C_d` | `D_r` (m) | `lambda` | veg. `C_d` |
| --- | --- | --- | --- | --- | --- |
| agriculture crops | 480.0 | 0.25 | 0.010 | 0.050 | 0.12 |
| bare | 540.0 | 0.00 | 0.000 | 0.000 | 0.00 |
| deciduous forest | 420.0 | 0.90 | 0.050 | 0.180 | 0.65 |
| forest | 410.0 | 0.95 | 0.060 | 0.200 | 0.75 |
| forest high sev fire | 530.0 | 0.18 | 0.006 | 0.018 | 0.08 |
| forest low sev fire | 465.0 | 0.58 | 0.026 | 0.085 | 0.34 |
| forest moderate sev fire | 490.0 | 0.40 | 0.016 | 0.050 | 0.20 |
| forest prescribed fire | 450.0 | 0.70 | 0.035 | 0.110 | 0.45 |
| grass high sev fire | 530.0 | 0.08 | 0.003 | 0.010 | 0.04 |
| grass low sev fire | 475.0 | 0.27 | 0.010 | 0.045 | 0.15 |
| grass moderate sev fire | 500.0 | 0.18 | 0.007 | 0.026 | 0.09 |
| grass prescribed fire | 465.0 | 0.32 | 0.012 | 0.055 | 0.18 |
| high use skid | 575.0 | 0.03 | 0.000 | 0.000 | 0.00 |
| low or treated skid | 545.0 | 0.12 | 0.006 | 0.020 | 0.03 |
| mixed forest | 415.0 | 0.92 | 0.055 | 0.190 | 0.70 |
| mulch | 420.0 | 0.85 | 0.040 | 0.180 | 0.20 |
| short grass | 460.0 | 0.34 | 0.014 | 0.070 | 0.24 |
| shrub | 430.0 | 0.72 | 0.035 | 0.120 | 0.45 |
| shrub high sev fire | 525.0 | 0.14 | 0.004 | 0.014 | 0.06 |
| shrub low sev fire | 465.0 | 0.44 | 0.020 | 0.065 | 0.24 |
| shrub moderate sev fire | 490.0 | 0.30 | 0.012 | 0.038 | 0.14 |
| shrub prescribed fire | 450.0 | 0.55 | 0.026 | 0.090 | 0.32 |
| skid | 560.0 | 0.05 | 0.000 | 0.000 | 0.00 |
| tall grass | 440.0 | 0.48 | 0.020 | 0.100 | 0.35 |
| thinning | 435.0 | 0.90 | 0.045 | 0.160 | 0.50 |
| young forest | 430.0 | 0.85 | 0.045 | 0.160 | 0.60 |

The basis for these values is recorded alongside the table in the
producer's decision record, and it is stated plainly there: they are
operator-calibrated class defaults (provenance class
`operator_calibration`, confidence `bounded_class_calibration`) encoding a
directional ordering rather than a fit to observations. Undisturbed
vegetated classes carry the strongest form-drag and vegetation terms;
burned classes step down with severity and never exceed their unburned
counterparts in roughness or vegetation protection; compacted and bare
classes carry no vegetation drag; roughness height and concentration are
jointly zero or jointly positive, with concentration bounded in [0, 1].
The values are texture-invariant because no available source authorizes
texture-specific gradients for these coefficients — soil texture remains
in the lookup key for future measured or literature-backed refinement —
and the projection rejection above applies to the defaults as well: none
is derived from legacy roughness, cover, or residue fields. A uniform
placeholder for all classes was considered and rejected. No observational
validation of these values exists yet; in the vocabulary of Section 11
their absolute magnitudes are open, and users with site knowledge should
treat the table as a starting point — the coefficients are ordinary
per-element inputs in the native YAML and can be overridden.

## 10. Groundwater baseflow

WEPP's water balance ends at deep percolation: water leaving the soil
profile's bottom boundary exits the model, and legacy watershed streamflow
carries a constant per-channel baseflow surrogate supplied in the channel
input file. For the mountain watersheds openWEPP targets, that misses a
dominant streamflow component, and the WEPP-lineage remedy exists in the
literature: Srivastava et al. (2013; also Srivastava, 2013) added a linear
groundwater reservoir driven by WEPP-simulated deep percolation, and later
work extended the idea (Srivastava et al., 2017).

openWEPP implements the linear-reservoir formulation as published:

```text
Qb_i = kb * S_i          baseflow from groundwater storage
Qs_i = ks * S_i          deep seepage out of the reservoir
S_i+1 = S_i + (D_i - Qb_i - Qs_i)
```

where `D_i` is the day's deep percolation from the hillslope water
balance, and `kb`, `ks`, and the initial storage come from a per-run
coefficient file (`gwcoeff.txt`). The nonlinear extensions of the 2017
work are
deliberately out of scope of the current authority. The component is
fail-closed in the openWEPP style: absent coefficients disable the process
(deep percolation then exits the model as in legacy WEPP); present but
malformed or out-of-domain coefficients fail the run; no coefficients are
ever inferred. Generated baseflow and deep seepage are exported per
hillslope as first-class outputs, and the watershed channel model consumes
generated baseflow in place of the constant surrogate where hillslope
contributions exceed a configurable area threshold — closing the loop the
constant `cbase` value approximated.

## 11. What may be concluded, and what may not

Reading openWEPP hillslope output against legacy WEPP, the defensible
statements sort into the three classes this project uses everywhere.

Validated — checked against a written tolerance and passing: water and
sediment mass conservation on every routed surface (machine-roundoff
ledgers, hard-fail enforced); solver behavior against the published
validation cases and analytic oracles of Section 5; mesh and timestep
discretization error within the Section 8 tolerances on the ratified
cohort; single-OFE event sediment delivery against legacy at matched
cut-points after the cover-pathway repair (Section 7); byte-identical
legacy-path behavior for legacy-format inputs.

Bounded — measured, attributed, not zero: residual single-OFE sediment
differences riding the peak-rate operand difference on small events; the
routed end-of-window storage and tail-fold accounting classes, surfaced
per run; discretization deltas between the production mesh and finer
diagnostics.

Open — named, with contract gates, not defaulted: the routed-magnitude
coupling of erosion transport on multi-element hillslopes (Section 7);
absolute lateral subsurface flow magnitude on steep forest soils, which
awaits external observational authority rather than a legacy comparison
(the legacy figure derives from the same provisional conductivity model
and cannot arbitrate); the absolute magnitudes of the disturbed-class
default routing coefficients (Section 9); and the applicability limits
the friction submodels carry from their own literature.

And one statement about the comparison itself: for the majority of
composed model behaviors, "how does this compare to WEPP?" has no precise
answer, because
the legacy side of the comparison has never been characterized — not by
this project, and, on the record of the code base itself, not by anyone.
Where openWEPP diverges from legacy WEPP, the divergence is investigated
and adjudicated against sources; several such investigations found
openWEPP defective and fixed it, several found comparison artifacts, and
several found legacy behavior that no one could justify from any published
description. Users accustomed to treating WEPP output as ground truth
should recalibrate toward the quantities that can actually bear weight:
conservation, timing, structure, and the named tolerances above — in
either model.

## References

Davis, S. F. (1984). *TVD finite difference schemes and artificial
viscosity* (ICASE Report No. 84-20 / NASA CR-172373). NASA Langley
Research Center.

Dun, S., Wu, J. Q., Elliot, W. J., Robichaud, P. R., Flanagan, D. C.,
Frankenberger, J. R., Brown, R. E., & Xu, A. C. (2009). Adapting the
Water Erosion Prediction Project (WEPP) model for forest applications.
*Journal of Hydrology, 366*(1–4), 46–54.
https://doi.org/10.1016/j.jhydrol.2008.12.019

Flanagan, D. C., & Nearing, M. A. (Eds.). (1995). *USDA Water Erosion
Prediction Project: Hillslope profile and watershed model documentation*
(NSERL Report No. 10). USDA-ARS National Soil Erosion Research
Laboratory.

García-Navarro, P., Alcrudo, F., & Savirón, J. M. (1992). 1-D
open-channel flow simulation using TVD-McCormack scheme. *Journal of
Hydraulic Engineering, 118*(10), 1359–1372.
https://doi.org/10.1061/(ASCE)0733-9429(1992)118:10(1359)

Iwagaki, Y. (1955). Fundamental studies on the runoff analysis by
characteristics. *Bulletins — Disaster Prevention Research Institute,
Kyoto University, 10*, 1–25.

Mingham, C. G., Causon, D. M., & Ingram, D. M. (2001). A TVD MacCormack
scheme for transcritical flow. *Proceedings of the Institution of Civil
Engineers — Water and Maritime Engineering, 148*(3), 167–175.
https://doi.org/10.1680/wame.2001.148.3.167

Papanicolaou, A. N., Abban, B. K. B., Dermisis, D. C., Giannopoulos,
C. P., Flanagan, D. C., Frankenberger, J. R., & Wacha, K. M. (2018). Flow
resistance interactions on hillslopes with heterogeneous attributes:
Effects on runoff hydrograph characteristics. *Water Resources Research,
54*, 359–380. https://doi.org/10.1002/2017WR021109

Srivastava, A. (2013). *Modeling hydrological processes in three
mountainous watersheds in the U.S. Pacific Northwest* (Doctoral
dissertation). Washington State University, Pullman, WA.

Srivastava, A., Dobre, M., Wu, J. Q., Elliot, W. J., Bruner, E. A., Dun,
S., Brooks, E. S., & Miller, I. S. (2013). Modifying WEPP to improve
streamflow simulation in a Pacific Northwest watershed. *Transactions of
the ASABE, 56*(2), 603–611. https://doi.org/10.13031/2013.42691

Srivastava, A., Wu, J. Q., Elliot, W. J., Brooks, E. S., & Flanagan,
D. C. (2017). Modeling streamflow in a snow-dominated forest watershed
using the Water Erosion Prediction Project (WEPP) model. *Transactions of
the ASABE, 60*(4), 1171–1187. https://doi.org/10.13031/trans.12035

Tseng, M.-H. (2010). Kinematic wave computation using an efficient
implicit method. *Journal of Hydroinformatics, 12*(3), 329–338.

## Revision Log

| Version | Date | Changes |
| --- | --- | --- |
| 0.1 | 2026-07-09 | Initial draft. |
| 0.2 | 2026-07-09 | Operator review: named the `wepp-forest` reference branch (§3); qualified the >1000 mm closure residuals to >10-OFE hillslopes (§3); expanded the steady-state peak/effective-duration fidelity contrast with TVD-MacCormack (§5); removed solver micro-optimization detail (§6); added the disturbed-class default routing-coefficient table with provenance (§9); listed those defaults as open in §11. |
