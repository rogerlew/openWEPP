---
contract_id: SC-SNOWENERGY-001
title: Snow-Surface Energy and Sub-Canopy Longwave Contract
status: approved
maturity: active
owner: openWEPP maintainers + snow-process reviewer
contract_version: 57
producer_scope:
  - Hourly atmospheric longwave evaluated from hourly temperature and daily vapor/cloud state
  - Native-canopy effective cover to diffuse sky-view translation
  - Complementary sky and canopy longwave incident at the snow surface
consumer_scope:
  - Shared Stage 3 snow-surface energy carrier
  - Snow sublimation and melt components
  - Snow-energy diagnostics and assurance outputs
evidence_level: static+independent_oracle+contract_vectors
last_reviewed: 2026-08-30
supersedes: []
superseded_by: []
---

# SC-SNOWENERGY-001 Snow-Surface Energy and Sub-Canopy Longwave Contract

Status: `approved`
Maturity: `active`
Evidence mode: `static + independent oracle + contract vectors`

## Purpose

Define the canonical hourly atmospheric and sub-canopy longwave equations,
using hourly temperature with daily vapor/cloud state, plus operand meanings,
guards, and coupling obligations for openWEPP snow-surface energy
calculations. The contract derives diffuse sky view from existing effective
canopy cover and does not introduce a user-entered sky-view coefficient or a
required remote-sensing input.

Version 3 replaced the failed version-2 snowfall-event top-layer provider with
the Marks/SNOBAL active thermal control volume. The active volume spans the
upper `min(z_s, 0.25 m)` of snow except for the version-4 terminal lower-volume
collapse, crosses depositional-layer boundaries
conservatively, and exchanges conductive heat with the remaining lower pack
inside each stability substep. Marks/SNOBAL mass-dependent timestep
subdivision is required. The production default remains unchanged. EB-03A
real-consumer and terminal workspace evidence pass.

Version 4 defined the terminal resolved thermal-layer domain. The exact
libsnobal `1 kg m^-2` threshold suspends Stage 3 exchange only when total pack
ice mass is at or below the boundary. In a resolved pack, a lower thermal
volume strictly below the same threshold collapses into a one-volume thermal
solve; equality remains a two-volume solve. At that version, CoE remained the
current runtime owner for snow existence and mass, so persistent layer state
was retained rather than converted to water. Version 7 preserves that behavior
only under the implementation hold and does not carry it into target authority.

Version 5 separates represented-layer lifecycle from aggregate residual
acceptance. A density-layer fragment is represented when its converted ice
mass is greater than the existing `1e-9 kg m^-2` zero-mass boundary; with
`rho_w = 1000 kg m^-3`, this is `mass_swe_m > 1e-12 m`. The independent
`1e-9 m` SWE and physical-depth aggregate closure tolerances remain residual
guards and are not layer-deletion thresholds.

Version 7 reconciles melt ownership. The admitted future production target is
one Stage 3 energy-and-mass control volume: after cold-content satisfaction,
bounded positive energy converts available ice to liquid and that liquid enters
the same-substep refreeze, retention, and routing chronology. The current
runtime remains unchanged and therefore is on `IMPLEMENTATION_HOLD`: it still
generates melt with CoE and reports Stage 3 positive excess without converting
it. CoE is a compatibility implementation during the hold, not the target
scientific authority, and simultaneous CoE/Stage 3 melt generation is
prohibited.

Version 8 binds the CLIGEN/openWEPP hourly forcing projection to explicit
virtual transfer geometry for Stage 3 turbulent exchange. The model evaluates
temperature, humidity, and wind with `5 m` transfer heights above the modeled
instantaneous snow surface and exposed-snow aerodynamic roughness `0.005 m`.
Those values are model geometry derived from the pinned libsnobal point-input
contract, not measurements, forcing reference heights, calibration parameters,
or site observations.

Version 9 distinguishes the current evaluation carrier's raw signed vapor
exchange opportunity from canonical bounded vapor transfer. Schema-v6 raw
vapor and latent energy are diagnostic opportunities. Actual sequential snow
mass debit/credit uses bounded transfer. Capacity truncation without a matching
latent-energy bound is a plausibility finding, not production authority.

Version 10 separates forcing-source custody from model transfer geometry.
GRIDMET asset metadata describes daily `vs` as nominal `10 m` wind, but the
initial fixture manifests did not preserve enough generator metadata to
identify their exact asset version/status, pixel, sampling, transformation, or
exposure.
Their CLI `w-vl` therefore remained raw forcing with source/exposure authority
missing at that intake. Stage 3 consumes that raw value while using virtual `z_u=5 m`; PMET's
separate `10 m`-to-`2 m` adjustment is local and never becomes snow forcing.
A reference-height diagnostic cannot establish forest/sub-canopy exposure,
license attenuation, or authorize a production correction.

Version 11 incorporates provider-side recovery from the surviving WEPPpy
source runs. Byte-identical `/wc1` CLIs and run records directly establish
retained watershed centroids, GRIDMET-enabled run intent, complete daily
parquet wind, and equality to one-decimal CLI `w-vl`. The nearest pre-build
WEPPpy revision statically reconstructs a path that would request GRIDMET `vs`
at the watershed centroid and share it across hillslopes. The runs do not
retain the deployed
container/source SHA, raw request/response, asset version/status, server-side
pixel-selection rule, exact pixel, timezone/day boundary, fill policy, or
aerodynamic exposure. Modeled evergreen-forest landuse and `cancov=0.9`
establish target model intent only, not physical exposure or representativeness.

Version 17 defines the precipitation-custody input to the persistent Stage 3
lane. It admits one sealed, canonically ordered phase-parcel set per support;
binds precipitation mass and precipitation-advected heat to that same set;
and makes open raw rain mutually exclusive with covered vegetation terminal
liquid at each ground destination. It imports the interception, throughfall,
stemflow, and drainage chronology from `SC-VEGETATION-001@28` and the
destination topology from `SC-LANDSURFACEENERGY-001`; it adds no interception
equation or canopy-snow process.

Version 18 defines the persistent Stage 3 snow--soil conductive boundary. One
OFE-ground lane column couples its bottom represented snow thermal volume to
the first ordered OFE soil-thermal node. The interface uses the same
center-to-center, two-half-layer series resistance and Crank--Nicolson endpoint
evaluation already admitted by `SC-LANDSURFACEENERGY-001`; tile temperatures
are neither averaged nor assigned duplicate shares of the lane flux.

Version 31 admits one exact-floor terminal-one-volume numerical iterate for the
covered fixed point. Rejected candidate version 30 independently blended
persistent phase components and cumulative ledgers and is not authority. When
the current unpublished image and a finite authentic image share one immutable
beginning owner, exact support and complete support operands but straddle the
ice/liquid/cold-content phase kink, version 31 permits only their coordinated
midpoint in canonical total-water/enthalpy coordinates. The midpoint is
reconstructed from the immutable beginning state and a single coordinated
midpoint of the complete signed vapor, external-liquid, and component-energy
operand vector, then passed through the existing version-22 phase projection.
It is never an accepted or publishable state. Every identity, receipt,
structural field, closure rule, tolerance, iteration cap, and the exact
60-second floor remain unchanged; a fresh authentic map image is still required
for finalization, acceptance, replay, and publication.

Version 32 retains version 31 unchanged for same-disposition endpoints and
adds one narrower unpublished vapor-active-set transition when independently
closed pure endpoint images have strictly opposite signed actual vapor. On any
exact covered support at or above the unchanged 60-second floor, the transition
localizes the unique signed-vapor root, makes vapor mass, deposition,
sublimation, and their already-once latent-energy component exact positive
zero, interpolates only external liquid and ordered nonlatent energy, and then
uses the same canonical total-water/enthalpy projection as version 31. A
subsequent zero-to-one-sided numerical entry uses the existing support-scaled
Picard weight while preserving the fresh authentic endpoint's positive finite
specific latent heat. Neither numerical image is authentic or eligible for
convergence, finalization, replay, acceptance, persistence, or publication.
Every physical equation, tolerance, iteration cap, event, topology, custody,
receipt, rollback rule, public schema, and the exact 60-second floor is
unchanged.

Version 33 supersedes versions 31 and 32 as production control only after an
exact terminal-one-volume active-set transition reset. The trigger is not
bitwise equality of two raw authentic owner images. It requires identical
support/source/event/topology/custody/receipt joins and the exact sequence
v32 root/interface -> one-sided branch entry -> opposite pure-vapor raw
authentic image -> the same root/interface/reset coordinates and branch
predicates, with opposite pure vapor disposition. Raw authentic continuous
owner coordinates may change asymptotically between the two visits. Versions
31 and 32 remain historical diagnostic reconstruction and refusal evidence;
their affine or synthetic images cannot be accepted or published.

After that exact reset sequence, version 33 solves unchanged coupled physical
residuals in a private reduced coordinate space: ending snow total water and
enthalpy per affected lane, plus only the coupled soil endpoint
enthalpy/temperature coordinates required by the existing Crank--Nicolson
snow--soil block. `covered_phase_consistent_residual_evaluate_v1` returns one
`CoveredPhaseConsistentResidualEvaluationV1` with concrete `R_W`, `R_H`,
`R_E`, and `R_T` carriers plus algebraic canonical-phase, LSE, CN, receipt,
identity, and capacity side constraints; `F(x)-x` is not a physical residual.
Every residual/Jacobian/trust/rejection/fresh/final-replay physical evaluation
charges the same `CoveredPhysicalEvaluationBudgetV1`, bounded by the unchanged
96 total. A deterministic safeguarded semismooth Newton/trust-region method may
produce a private root. Only a fresh, resealed result admitted as
`CoveredConvergenceAdmissionV1::CoupledAuthentic` may bypass the ordinary
Picard iterate-equality/convergence predicate; all finalization, closure,
identity, receipt, rollback, and publication checks remain unchanged. Internal
event boundaries still force exact partition or refusal; the phase kink is an
internal complementarity boundary, not a new event. No physical equation,
tolerance, floor, cap, event, topology, custody, receipt, rollback, public
schema, diagnostic, or persistence policy changes.

Version 34 retains every version-33 equation, residual carrier, side
constraint, trust safeguard, budget charge, authentic replay, and publication
guard, and extends only the eligibility trigger for that same private reduced
solve. On an event-free terminal-one-volume covered support at or above the
exact 60-second floor, eight consecutive raw authentic Picard maps may enter
the existing `R_W/R_H/R_E/R_T` solver when their support, source, event,
topology, custody, static receipt joins, phase branch, and exact V2
soil-enthalpy-carry authority and representation are unchanged; every residual
is finite; the governed residual merit decreases strictly at every map; and no
`A/B/A` active-set transition,
active-set change, or finalization restart occurs. The eight raw maps and every
subsequent residual, Jacobian, trust, rejected, fresh, and final-replay
evaluation charge the same unchanged 96-physical-evaluation budget.

Static receipt joins mean the unchanged receipt schema, support, topology,
ordered operand lineage, and beginning-owner/authority identities; they do not
require the physical applied-energy value, candidate-ending identity, or
resealed receipt digest to remain byte-equal across authentic contraction
maps. Exact carry means unchanged signed-dyadic schema/version, normalization,
units, layer/owner/order authority, and exact reconstruction of the physical
enthalpy as `E=exact(H_hi)+R`; it does not require `H_hi` or `R` coordinate
values to remain byte-equal while the physical solution evolves. Every evolved
receipt and carry remains subject to unchanged exact custody, ordered
reconstruction, residual, closure, replay, and reseal guards.

Version 35 retains version-34 solver eligibility and adds exact authentic
receipt stabilization after a coupled physical root. Let `R_n` be the complete
immutable physical receipt input used by probe `n`. The authentic physical
evaluation reconstructs and reseals `R_(n+1)` from immutable beginning owners,
sealed sources, and its candidate output. That output receipt set becomes the
next authentic physical input without digest repair, tolerance comparison, or
in-place mutation. Every probe charges the same unchanged
`CoveredPhysicalEvaluationBudgetV1` already used by prior Picard and solver
evaluations. The first private-root-to-authentic reseal is therefore a probe;
its output must not be compared for exact acceptance against artifacts produced
under a different receipt input.

Receipt stabilization exists only when input `R_n` is exactly equal, in
canonical receipt bytes and every authenticated field, to reconstructed output
`R_(n+1)`. The implementation then performs one independent authentic replay
using that same stabilized receipt set and requires exact equality of physical
residuals, candidate artifacts, and reconstructed receipts with the stabilized
probe before `CoveredConvergenceAdmissionV1::CoupledAuthentic` or finalization.
Exact receipt oscillation, nonfinite state, any side-constraint failure, budget
exhaustion, replay disagreement, or attempted private/probe artifact retention
fails closed; all private/probe artifacts are discarded. No physical equation,
tolerance, floor, cap, event, topology, custody, receipt meaning, exact-carry,
rollback, adaptive controller, public schema, persistence, diagnostic, or
publication policy changes.

Version 36 retains version-35 receipt stabilization and authentic finalization
and makes the reduced physical solver geometry-complete for each affected
terminal-one-volume lane. It adds exactly one density coordinate `rho_1,l` per
affected lane. Ice mass `I_1,l` remains derived by the canonical `Pi(W,H)`
projection, and physical thickness is reconstructed on the existing mass-depth
basis as `z_1,l=I_1,l/rho_1,l`; thickness is not an independent coordinate.
The sealed physical evaluator returns `R_rho,l` from the unchanged Stage 3
density/settling constitutive map evaluated from immutable beginning state,
sealed forcing, and current physical operands. The generalized residual vector
is `R_W/R_H/R_rho/R_E/R_T`; generic `F(x)-x` remains forbidden.

Stable eligibility retains exact layer identity/order, settling authority, and
density-model branch while allowing the physical `rho_1,l` and reconstructed
`z_1,l` coordinates to evolve. Every constitutive/Jacobian/trust/rejected/
fresh/receipt-probe/replay evaluation charges the same unchanged shared 96
budget. Density or thickness interpolation, repair, tolerance substitution,
uncharged physics, private-state acceptance, receipt-stabilization bypass, or
authentic-finalization bypass is prohibited. No equation, tolerance, cap,
60-second floor, event, topology, custody, receipt, exact-carry, rollback,
publication, persistence, or diagnostic policy changes.

Version 37 corrects the version-36 geometry merit exposed by retained r93
finalization evidence. For every affected terminal-one-volume lane, the
physical evaluator already reconstructs `z_1,l=I_1,l/rho_1,l`; it must now
also return the derived physical thickness closure
`R_z,l=z_1,l-z_phys,l`. `R_z` is an admission side-constraint and merit
coordinate under the existing `depth_abs_m` bound. It is not a new independently solved coordinate
and not a replacement for `R_W`. The safeguarded solver
continues to solve only `R_W/R_H/R_rho/R_E/R_T`, but it may declare a root
only when both that residual vector and every derived `R_z` close. This makes
the existing water-mass equation resolve the tighter low-density depth image
without changing its equation or tolerance.

Every private, Jacobian, trust, rejected, fresh, receipt-probe, and replay
evaluation derives `R_z` from the same already-charged unchanged Stage 3 map;
no extra or uncharged constitutive call is allowed. V35 exact receipt
stabilization, independent same-input replay, `CoupledAuthentic`, and the
unchanged authentic finalization comparison remain mandatory.
Density or thickness copying, interpolation, post-hoc repair, tolerance substitution,
omission of `R_z` from root merit/admission, or treating `z` as an independent
unknown fails closed and discards all private artifacts. No equation,
tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry,
rollback, publication, persistence, or diagnostic policy changes.

Version 38 corrects the map-identity defect proven by retained r95. Version 37
closed `R_W/R_H/R_rho/R_E/R_T` and derived `R_z` exactly in both the coupled
root and stabilized same-input replay, yet the later authentic finalization
rebuilt different carrier operands and reproduced the retained thickness
change. Closure of the provisional/iteration carrier map is therefore not an
admissible proxy for closure of the authentic finalization map.

Every charged private, Jacobian, trust, rejected, receipt-stabilization probe,
and same-input replay evaluation must target the finalization-equivalent
endpoint map directly. From immutable beginnings and the canonical proposed `W/H/rho/E/T` coordinates,
it constructs the same unpublished endpoint snow
and exact-carry soil operands. Soil `E/T` coordinates apply only to the first ordered, snow-coupled node of each OFE: the projection must preserve every deeper layer bit-exact, including high term, exact carry, temperature, identity, order, and custody. Requiring a one-layer soil column, zeroing or rerounding a deeper-layer carry, or silently extending the reduced coordinate vector is forbidden. The map then uses non-provisional carrier posture, corrected LSE
boundaries, open/covered boundary merge, precipitation set, CN receipt input,
and single Stage 3 evaluation used by authentic finalization. Exactly one Stage 3 physical map
is evaluated for each shared-budget charge; a provisional
map may neither establish residual closure nor seed admission. V35 receipt
stabilization and one independent same-input replay remain exact. The later
authentic finalization remains an independent replay and must reproduce the
same state, LSE, boundary, receipt, and ledger image exactly before
publication. No post-hoc copy/repair, second uncharged map, provisional-map
fallback, tolerance/cap/floor change, event/custody/receipt change, or private
publication is authorized.

Version 44 corrects the premature aggregate-energy admission exposed by
retained r116 without weakening the aggregate ledger. A canonical coupled
coordinate is an uncommitted nonlinear trial until its reciprocal longwave,
shortwave, sensible, and vapor boundary exchange has been rebuilt. The first
charged `PrivateTrial` carrier therefore uses the existing uncommitted provisional LSE posture:
it produces the corrected exchange operands needed by
the one Stage 3 map, but its deferred weighted-OFE closure is neither root nor
publication evidence. Receipt-stabilization probes and the independent
same-input replay remain strict, non-provisional endpoint evaluations and must
pass the unchanged weighted-OFE energy closure before `CoupledAuthentic` can
be admitted. Independent finalization remains strict and exact.

The V38 top-soil `E/T` coordinate is consumed exactly once by the existing
snow--soil Crank--Nicolson receipt/credit path. The Stage3-covered LSE control
volume continues to freeze soil storage and ground heat at its admitted
beginning, so the numerical coordinate must not also replace the V8 soil
physical beginning. Omitting it from the CN path or feeding it into both CN
and LSE is a typed failure. No tolerance, ledger equation, physical component,
shared-budget charge, Stage 3 call count, event, topology, custody, receipt,
exact-carry, rollback, publication, persistence, or diagnostic policy changes.

Version 45 corrects the receipt-stabilization budget and precision gap exposed
by retained r117/r118. A private physical root that merely satisfies the
unchanged residual tolerances is not necessarily an exact fixed point of the
binary64 snow--soil Crank--Nicolson receipt map. Before constructing the first
authentic receipt input, private root polishing therefore continues the
existing `R_W/R_H/R_rho/R_E/R_T` safeguarded Newton/trust map while its exact
canonical scaled merit decreases strictly, even after tolerance closure. If
the finite side-valid root is still tolerance-closed but no further
representable strict descent is available, the carried best physical bundle
may seed V35 receipt stabilization; this posture is neither convergence nor
acceptance. Every private-solver evaluation must preserve three later charges
for at least one private polishing map, one authentic probe, and its independent
replay; every polishing evaluation must preserve the latter two charges; and
every nonstable receipt probe must preserve the final replay charge, all inside
the same unchanged shared cap of 96. Exact receipt input/output equality in
canonical bytes followed by one independent same-input exact residual/artifact/receipt
replay remains the only coupled-authentic admission.
The ordinary and polishing phases use one shared safeguarded step and carry
the exact trust radius across tolerance closure. Each charged physical map
returns one complete residual/artifact/finalization-input bundle stamped with
its shared-budget ordinal and exact canonical phase/density branch identity;
ordinal, coordinate, or branch substitution fails before a trial can become
the carried best bundle. Exact replay also compares the finalization inputs.
No mutable latest-evaluation side channel supplies root or replay artifacts.
No residual tolerance,
equation, receipt bit, digest, owner, or physical map is altered or repaired.

Version 46 corrects the incomplete-polishing-step budget waste proven by
retained r119/r120. On the canonical one-lane/one-soil-node support, the
unchanged coupled residual has five coordinates. The implementation entered a
further polishing step and charged a partial or complete finite-difference
Jacobian tail before discovering that no mandatory trust trial could be
charged while preserving receipt capacity. Every map in that final incomplete
step was therefore unable to change the carried root and the support reached
receipt stabilization with only one authentic probe plus the independent
replay slot. R120 did not record reverse-column/backtrack cadence, so the exact
number of wasted tail charges is intentionally not inferred from the aggregate
count.

Before beginning any ordinary or private polishing step, the shared solver
must preflight enough remaining capacity for every canonical generalized-
Jacobian column, at least one full-physics trust trial, and the role's unchanged
downstream reserve. The exact requirement is `d + 1 + r` unconsumed charges,
where `d` is the validated residual-coordinate cardinality and `r` is the
already governed post-root or receipt-entry reserve. Failure stops private
sub-tolerance polishing with the carried complete best bundle before charging
any column; above tolerance it remains typed `EvaluationBudget`. Existing
per-map reserve checks remain mandatory for reverse finite differences,
rejected trust trials, and all failure paths. No partial Jacobian, rejected
trial, or unused perturbation artifact may seed receipt stabilization.

The generalized Jacobian, Newton direction, trust loop, physical residuals,
merit, strict-descent rule, coordinates, tolerances, and map posture are
unchanged. The single budget owner, one charge per physical map, maximum 96,
immutable exact receipt iteration, oscillation refusal, protected independent
same-input replay, authentic finalization, rollback, and no-publication rules
remain unchanged. Version 46 deliberately adds no quasi-Newton/chord/secant
state and no CN heat, receipt, digest, or map-difference residual. Canonical
qualification must determine whether the five recovered charges suffice;
failure remains typed and is evidence for a later contract-first numerical
successor, not authority to relax equality or the cap.

Version 47 corrects the atomic complete-owner transaction join exposed by
retained r121 after version 46 cleared the preceding exact-floor receipt-
stabilization failure. A native-V2 soil accepted owner may be installed only
under one of two typed postures. The ordinary/public first-child posture requires the
mutually equal vegetation, LSE, and biogeochemistry source transaction to equal
the authenticated soil target transaction. The composed second-child posture
requires those same three source owners to be mutually equal, the accepted soil
state transaction to equal its authenticated target transaction, and that soil
owner's exact authenticated expected predecessor to equal the source
transaction. The split posture is available only to the authenticated
continuation install and must receive the explicit V39
`PhysicalSoilEnergyTransactionAuthorityV2` constructed by that consumer from
the outer source plus the continuation/prepared soil target.
Neither posture permits inferred numeric adjacency.

The target and predecessor are read only from the already validated native-V2
accepted owner; the source is read only from the complete outer owners. A
foreign, missing, swapped, stale, or disagreeing source/target/predecessor, a
soil state not sealed to its target, or any attempt to substitute the target
for all source owners fails before mutation. Installation remains one cloned
candidate followed by one atomic replacement after complete validation. Exact
accepted no-op, rollback, receipt/carry/support/owner custody, and no private
publication remain unchanged. Version 47 changes no physics, energy value,
equation, tolerance, budget, floor, event, topology, receipt meaning,
serialization, persistence, diagnostic, or adaptive-time policy.

Version 48 corrects the remaining real fixed-point finalization route exposed
by retained r122. The ordinary `1800..1980 s` coupled endpoint carries an
authenticated unpublished native-V2 soil result whose prepared target is 43
and exact predecessor is the mutually equal vegetation/LSE/BGC source 42, but
the real finalizer discarded that typed prepared-beginning posture and called
the strict generic installer without an explicit V39 transaction authority.
The generic/public installer remains unchanged and admits only exact
`source==target`.

One distinct authenticated-beginning finalization install may construct an
explicit native-V2 `PhysicalSoilEnergyTransactionAuthorityV2` only after the
authoritative resident validates the complete prepared beginning: owner,
model, run, state, receipt chain, exact predecessor, support, and target. The
candidate must still validate as the exact accepted result and seals derived
from that same prepared beginning. The complete outer source owners remain
mutually equal; the accepted soil state and every layer remain sealed to the
prepared target; and a split remains admissible only when the prepared exact
predecessor equals the source. The fixed-point finalizer must use this typed
path rather than the generic installer. Missing prepared-beginning authority,
foreign or substituted source/target/predecessor/support/receipt/state/seals,
or any inferred numerical adjacency fails before cloning or mutation.

Version 48 changes no physical result, transaction number, owner, receipt,
support, equation, ledger, tolerance, budget, floor, event, topology,
serialization, rollback, publication, persistence, diagnostic, or adaptive
policy. It propagates already-authenticated transaction authority through the
real final install instead of erasing it.

Version 49 corrects the later same-parent native-V2 soil successor exposed by
retained r123/r124. At direct support `1920..2040 s`, the complete vegetation,
LSE, and BGC source remains exactly transaction 42 for the open V11 parent,
while the authenticated accepted soil resident is transaction 43 and the exact
prepared successor is target 44 with predecessor
43. Requiring the soil predecessor to equal the outer source is therefore a
cross-domain identity substitution, not a valid successor proof. Rebasing the
outer owners or inferring numeric adjacency is forbidden.

One opaque authenticated prepared-install authority may bind the unchanged
explicit outer-source/soil-target authority together with the exact validated
resident predecessor bundle and exact prepared beginning. Construction and
installation must independently validate the authoritative resident's owner,
accepted custody, state/layer transaction seals, receipt chain, and support;
the prepared target, exact predecessor, contiguous support, state, receipt,
model, and run joins; the mutually equal outer source owners; and the accepted
ending plus orchestrator seals. Atomic install may admit a soil predecessor
distinct from the outer source only under this exact typed authority. Generic
installation, V48's two-ID predecessor-equals-source posture, and every
ordinary/public route remain unchanged and fail closed.

The authority contains no successor arithmetic and never derives an identity
from numeric adjacency. Missing, foreign, stale, swapped, state-, layer-,
receipt-, support-, seal-, source-, predecessor-, target-, or authority-
substituted inputs fail before cloning or mutation. Exact accepted no-op,
byte-exact rollback, no private publication, owner/receipt/carry custody,
physics, ledgers, tolerances, shared budget, temporal floor, event ordering,
topology, persistence, diagnostics, and adaptive policy remain unchanged.

Version 50 corrects the ending-source anchor exposed by retained r125-r129.
The authenticated constitutive beginning is lawfully heterogeneous at the
owner level: the exact captured tuple is vegetation 41, LSE 40, BGC 41, and
native-V2 soil resident 41. It is not an ending complete-owner source and may
not be passed through the mutually-equal ending-source validator. The exact
ending-source authority is the already validated
`UncommittedCoveredV8OwnerEnvelope.transaction_id()`: its envelope validator
joins vegetation, physical/hydrology, BGC, and material receipts, and the V11
finalizer independently joins and normalizes the candidate vegetation/LSE/BGC
ending owners to that transaction before soil installation.

The V49 opaque authority must therefore bind the candidate's exact mutually
equal complete-owner source to the explicit validated envelope transaction,
while retaining the heterogeneous authenticated beginning only as exact soil
resident/prepared custody. Construction and installation independently
reconstruct the same envelope-derived source/soil-target authority. A jointly
rebased candidate, a substituted envelope source, any candidate owner
disagreement, an invalid envelope vegetation/physical/BGC/material join, or
any beginning/resident/prepared/accepted/receipt/seal substitution refuses
before mutation. No source is inferred from numeric adjacency or copied from
the soil predecessor; generic/V47/V48 postures, rollback, no publication,
physics, ledgers, tolerances, shared budget, floor, events, topology,
persistence, diagnostics, and adaptive timing remain unchanged.
In particular, no source is inferred from numeric adjacency.

The stable-monotone trigger changes no tolerance and does not admit a Picard or
private trial. No private trial may enter acceptance. Before a physical root exists, any trigger or solver refusal
discards all private trials and resumes ordinary raw authentic Picard with only
the budget that remains; exhaustion remains typed nonconvergence. A root may
advance only through the unchanged fresh `CoupledAuthentic` evaluation,
replay, receipt reseal, finalization, and publication path. No physical
equation, phase projection, floor, cap, event, topology, custody, receipt,
rollback, adaptive controller, public schema, persistence, or diagnostic
policy changes.

Version 56 replaces transient binary64 heat-witness search as the preferred
specialization on a strictly frozen, noncrossing authentic Stage 3 branch.
Its private solve uses temperature-primary snow coordinates, derives snow
enthalpy exactly from water and temperature, rounds only the published high
word with round-to-nearest-even, and retains the exact remainder as additive
carry. The carry is part of an authenticated compound snow material owner and
whole-receipt chronology, including versioned committed, pending, and
in-progress restart state. This is a representational and custody correction:
the constitutive Stage 3 and Crank--Nicolson physics, exact 60-second minimum
support, shared maximum-96 physical-evaluation budget, physical ledger terms
and tolerances, event ordering, topology, rollback, and final admission remain
unchanged.

Version 57 corrects the V56 dispatch predicate and chronology exposed by the
canonical r147 run. A finite nonnegative external-liquid operand no greater
than `1.0e-12 kg m^-2 OFE-ground` is eligibility-neutral only: the exact
binary64 operand remains present and unchanged in every water and energy
ledger, receipt, replay, and finalization. This bound is one millionth of the
existing `1.0e-6 kg m^-2` covered mass-closure tolerance, one thousandth of
the `1.0e-9 kg m^-2` represented-layer lifecycle boundary, equals the existing
minimum terminal physical snow-closure scale, and is more than two orders
above the observed `5.480935263973555e-15 kg m^-2` roundoff residue.
Equality is eligible; a negative, nonfinite, or larger amount is ineligible.
V57 may make the same zero-charge eligibility transition from the first
tolerance-closed legacy root immediately before V55, retaining the already
spent shared budget and deriving temperature-primary coordinates without an
extra physical map. Once a V57 physical map charges, failure remains typed and
fail-closed. No operand is zeroed, clamped, dropped, repaired, or published,
and no physical tolerance, equation, phase/event rule, floor, cap, topology,
custody, receipt, rollback, persistence, or diagnostic policy changes.

## Scientific Scope

In scope:

- Dilley-O'Brien clear-sky longwave from air temperature and vapor pressure.
- Unsworth-Monteith cloud correction derived from daily clearness index.
- An effective-cover-to-diffuse-sky-view transformation derived from the same
  Beer-law canopy extinction basis used by FSM2.
- Complementary atmospheric and canopy longwave at the snow surface.
- Effective-unity canopy and snow emissivity convention; atmospheric
  emissivity remains the variable Dilley-Unsworth result.
- Outgoing snow longwave and the positive-toward-snow net-longwave sign.
- Typed runtime obligations and deterministic analytical test vectors.

Out of scope:

- Default activation, user-facing selectors, or public output-schema changes.
- A prognostic canopy-temperature energy balance.
- Explicit trunks, canopy gaps, terrain horizons, multiple reflections, or
  three-dimensional ray tracing.
- Site fitting, empirical calibration, or a new user radiative coefficient.
- Implementing the version-7 melt-owner target, selectors, defaults, or
  cutover. Shortwave, sensible, latent, ground/conductive, and precipitation-
  advection components are governed here only to make the future complete
  energy ledger and its implementation hold explicit.

Validity is limited to an equivalent, horizontally homogeneous one-layer
canopy with the FSM2 random-orientation angular extinction approximation and
an isotropic diffuse sky hemisphere. The native structural-cover floor is
treated as effective vertical optical depth in that equivalent medium. The
contract does not claim direct validity for directional crowns, explicit
gaps/edges/trunks, terrain-obstructed sky, or anisotropic diffuse radiation.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| `REF-SNOWENERGY-FLERCHINGER` | Flerchinger et al. (2009), *Water Resources Research* 45:W03423, doi: `10.1029/2008WR007394`, corrected Table 1, Tables 2 and 9 | Corrected Dilley-O'Brien clear-sky equation, precipitable-water proxy, Unsworth-Monteith cloud correction, daily clearness bounds, and reported uncertainty. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-ESSERY2008` | Essery et al. (2008), *Hydrological Processes* 22:2788-2800, doi: `10.1002/hyp.6930` | Hemispherical view-factor integration and two-component forest longwave exchange. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-FSM2` | Essery et al. (2025), “FSM2.1.1: an efficient model of snow processes and surface energy balance”, *Geoscientific Model Development* 18:3583-3607, doi: `10.5194/gmd-18-3583-2025`, Eq. 13-14 and §2.3 | Beer-law direct and diffuse canopy transmission, factor `1.6`, and complementary longwave exchange with unity emissivity. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-SVS2` | Leonardini et al. (2025), “SVS2-Crocus”, *Geoscientific Model Development* 18:9119-9154, doi: `10.5194/gmd-18-9119-2025`, Eq. 1 and §2.1.2 | Independent support that canopy sky-view factor is an exponential extinction function rather than direct plan-view cover. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-RUTTER2023` | Rutter et al. (2023), *Journal of Geophysical Research: Atmospheres* 128:e2022JD037980, doi: `10.1029/2022JD037980` | Stand-scale complementary sky/canopy formulation, effective-unity canopy behavior, and canopy-temperature approximation evidence and limits. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-PLANT` | `SC-PLANT-001#INV-PLANT-034` and native canopy variables | Canonical openWEPP meanings of effective canopy cover, structural cover floor, LAI, and height. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-EB01A` | `docs/work-packages/20260730-snow-surface-eb-01a-longwave-authority-research-001/` | Package evidence that reconciled atmospheric longwave candidates and admitted the FSM2 canopy route. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-MARKS1999` | Marks et al. (1999), *Hydrological Processes* 13:1935-1959, doi: `10.1002/(SICI)1099-1085(199909)13:12/13<1935::AID-HYP868>3.0.CO;2-C` | Two-layer SNOBAL energy balance, active-layer thermal state, conductive exchange, and progressively smaller shallow-layer timesteps. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-LIBSNOBAL` | CC0 libsnobal at `/home/workdir/pysnobal`, commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`; `_calc_layers.c`, `_adj_layers.c`, `_e_bal.c`, `g_snow.c`, `_divide_tstep.c`, `_below_thold.c`, `snobal.h`, `pysnobal/ipysnobal.py`, and `test_data_point/inheight.input` | Equation-reference implementation for `z_s_0`, `G_0`, harmonic two-layer transfer, the `60/10/1 kg m^-2` mass-dependent `60/15/1 minute` timestep hierarchy, exact total-`<=`/lower-`<` terminal-layer ordering, residual-snow phase disposition, and the point-forcing `5 m` thermodynamic/wind virtual heights plus `0.005 m` snow roughness. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-GRIDMET` | Google Earth Engine asset catalog `IDAHO_EPSCOR/GRIDMET`, accessed 2026-08-07; Abatzoglou (2013), DOI `10.1002/joc.3413`; NASA GSFC NLDAS-2 forcing documentation, accessed 2026-08-07 | General authority that distributed GRIDMET `vs` is daily nominal `10 m` wind on an approximately `4 km` grid and derives from gridded land-data forcing. It does not identify retained fixture pixels, transforms, or exposure. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-WIND-CUSTODY` | `docs/work-packages/20260807-snow-stage3-wind-source-custody-and-exposure-authority-001/`, its result-blind freeze, retained CLI hashes, provider recovery, custody ledger, consumer proof, and exposure matrix | Custody/claim authority proving retained WEPPpy value lineage and distinguishing it from statically reconstructed request/transform semantics, while separating raw CLI/Stage 3 wind, PMET-local `2 m` adjustment, virtual `5 m` transfer geometry, and missing deployed/server/exposure authority without fitting or production correction. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-LUTE2022` | Lute et al. (2022), *Geoscientific Model Development* 15:5045-5071, doi: `10.5194/gmd-15-5045-2022`, section 2.2.7 | Independent documentation that Marks et al. address shallow-snow energy instability with progressively smaller timesteps. SnowClim's alternative temperature replacement and fitted cold-content tax are not admitted. | `[DIRECT][Static]` |
| `REF-SNOWENERGY-PHYSICAL` | Stefan-Boltzmann law and bounded-fraction physical invariants | Thermal emission, finite-temperature, and bounded-transmission requirements. | `[INFERENCE][Static]` |
| `REF-SNOWENERGY-21N` | `docs/work-packages/20260804-snow-coe-stage3-melt-owner-authority-reconciliation-001/` with frozen 21M evidence and pinned libsnobal commit `bf8b41c71e3e54ae654ae04005ddf72566c47ee6` (`_e_bal.c`, `_snowmelt.c`, `_advec.c`, `_mass_bal.c`, `_runoff.c`, `envphys.h`, `snow.h`) | Result-blind CoE-envelope adjudication; energy-to-melt derivation; exact energy, solid-to-liquid, and liquid-disposition chronology; current-runtime hold. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-USER-OFE-GROUND-V15` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts/science-contracts/SC-SNOWENERGY-001/authority-decision.md` | Direct prospective user selection of one persistent Stage 3 column per lane on OFE-ground basis, with complete tile-ground flux summation, no covered-subset renormalization, uniform-depth terminal identity, and topology-bound restart semantics. Repository state/terminal architecture supports the selection; future per-tile/routing-cell ownership requires a new version. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-PRECIP-CUSTODY-V17` | `SC-VEGETATION-001@28`; `SC-LANDSURFACEENERGY-001`; user-directed Child-1 covered physical-custody checkpoint | Imports the admitted vegetation liquid interception/release chronology and typed LSE destination topology, then binds their terminal parcels to the OFE-ground Stage 3 lane without adding canopy-snow or interception physics. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-SOIL-BOUNDARY-V18` | pinned `dac3c950...:src/frostn.for`, lines 476-607; `src/tmpadj.for`, lines 266-353; `SC-LANDSURFACEENERGY-001@8` | Legacy WEPP establishes additive snow/soil thermal resistance and harmonic conduction; current LSE supplies the authoritative node-centered two-half-layer interface, Crank--Nicolson endpoint evaluation, and OFE soil-thermal owner. Legacy zero-flux fallbacks, calibrated factors, and frost-front approximations are not imported. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V32` | Owner-authorized `WGHL-FULL-001D-V32` amendment in `docs/work-packages/20260830-workspace-gate-hold-lift-001/` at governance commits `c1cfd6e4bb5d4f28f85fae538b70c75747f207a6` and `8ec04440d`; frozen captured `1860..1920 s` support operands in `artifacts/science-contracts/SC-SNOWENERGY-001/v32-active-set/source-freeze.md` | Direct numerical-transition authority for one pure opposite-sign vapor root and zero-to-one-sided unpublished branch entry; no constitutive or adaptive-time-policy authority. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V33` | Owner-authorized `WGHL-FULL-001D-V33` amendment at governance commit `0dc1ef0070430314c67a9c8964eb4bd883cde7ba`; retained exact 60/120-second v32 transition capture `/tmp/wghl_001d_v32/dff_ws2_transition_capture.log` with SHA-256 `1e76362f229ebc8dbe41f481c91c69f69689858b771338ffe54b23fc8bfd9590`; frozen source/evidence identities under `artifacts/science-contracts/SC-SNOWENERGY-001/v33-phase-consistent-coupled/` | Direct numerical-solver authority for exact authentic-cycle detection, reduced unchanged physical residual evaluation, and coupled-authentic final replay; no new process physics or adaptive-time authority. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V34` | Owner-directed continuation of `WGHL-FULL-001D` in `docs/work-packages/20260830-workspace-gate-hold-lift-001/`; retained canonical one-day evidence shows stable raw Picard contraction consuming 93 of the unchanged 96 evaluations on a 60-second support without an active-set transition. | Prospective numerical-trigger authority to invoke the already-admitted equation-level v33 solver after eight stable monotone raw authentic maps; no new process physics, tolerance, or adaptive-time authority. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V35` | Owner-directed `WGHL-FULL-001D-V35` successor after retained r83 canonical failure `/tmp/wghl_001d_v34_64m_r83.log`, SHA-256 `bd091a4154eafff60309677e38bbbf598da6199cb15b3b84da93e7e23977b909`, on exact support `1800..1860 s` with typed `phase-consistent authentic replay/reseal` refusal after `298.91 s`. | Prospective exact receipt-stabilization authority after a coupled physical root; the first reseal is a charged probe under a different receipt input, not exact replay evidence. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V36` | Owner-directed `WGHL-FULL-001D-V36` successor after retained r88 field audit `/tmp/wghl_001d_v35_64m_r88_field_audit.log`, SHA-256 `55a904fbbb35126a00f50af60ba3c7d296e3298c575a784cbd3eedaa7f24ec65`; lane-1 thickness evolved from bits `4569208177783694401` (`3.06254815664577839e-3 m`) to `4569208162027237604` (`3.06254132337190239e-3 m`), delta `6.833273876e-9 m`, while LSE/boundary closed and Stage 3 rejected. | Prospective geometry-complete residual authority adding the terminal density coordinate and unchanged constitutive density residual; no new process physics or tolerance. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V37` | Owner-directed `WGHL-FULL-001D-V37` successor after retained r93 finalization audit `/tmp/wghl_001d_v36_64m_r93_finalization_audit.log`, SHA-256 `c4ddfef9dc52bdd085ff43d97f1179406486795696425c9aa949097fd756b0a5`; the v36 root closed `R_W` at the existing `1e-6 kg m^-2` bound and `R_rho` exactly, but its derived lane-1 thickness changed `4569208177783694401 -> 4569208162027237604` (`6.833273876e-9 m`), exceeding the unchanged `1e-9 m` depth bound because low density amplifies an otherwise-admissible ice-mass residual. | Direct numerical-merit authority requiring derived physical `R_z` closure from canonical `I/rho` without adding a coordinate, physics call, equation, tolerance, or adaptive-time change. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V38` | Owner-authorized `WGHL-FULL-001D-V38` successor after retained r95 canonical audit `/tmp/wghl_001d_v37_64m_r95_result_audit.log`, SHA-256 `a69e6d16b176cdf29015d55c01834f41a32def624774cf736f8f182884b5571c`; exact support `1800..1860 s` closed solver `R_z=0` at budget 21 and stabilized same-input replay `R_z=0` at budget 26, but independent authentic finalization still changed lane-1 thickness bits `4569208177783694401 -> 4569208162027237604`. | Direct numerical-map authority requiring every charged coupled evaluation to evaluate the authentic finalization-equivalent endpoint map rather than a provisional-map proxy, with one Stage 3 map per charge and unchanged final replay. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V39` | Owner-authorized `WGHL-FULL-001D-V39` successor after retained r102 canonical failure `/tmp/wghl_001d_v38_64m_r102.log`, SHA-256 `962716679a499b4e3bd23f87ed2e6ceabda1435081cc590fd9c06f21ed52548b`; the composed second child retained outer surface-ingress/source transaction 42 while its authenticated soil target owned transaction 43, and the former one-transaction operand builder refused `V2 soil support identity`. Retained r104 `/tmp/wghl_001d_v39_64m_r104.log`, SHA-256 `5749d657761615d139576daddad578c2996532695d768b816c06f677216c6959`, proved the same forbidden outer-42 substitution remained in covered initial/private coordinate support preparation after V8, operand, and continuation joins passed. | Direct custody authority separating the immutable source transaction from the authenticated soil-target transaction in V2 soil-energy operand construction and every private/finalization-equivalent soil support preparation without changing any physical energy amount, owner, support, receipt, or acceptance rule. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V40` | Owner-authorized `WGHL-FULL-001D-V40` successor after retained r107 canonical audit `/tmp/wghl_001d_v39_64m_r107.log`, SHA-256 `2fdfcb0c54e1845670f9d95a4b3770f3f43ab129e297901fc9e7b786dafa6c75`; the corrected rolling exact reset first dispatched at shared budget 96, so the unchanged solver correctly refused its first evaluation with `EvaluationBudget`. | Direct numerical-trigger authority for an earlier finite parity-monotone active-set reset window using exact static joins and strictly decreasing nonzero root drift; no residual, tolerance, physical solver, budget-cap, or acceptance change. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V41` | Owner-authorized `WGHL-FULL-001D-V41` successor after retained r109 canonical audit `/tmp/wghl_001d_v40_64m_r109.log`, SHA-256 `da9ebf633cb9194a91c55dc7679ac3ecd34da3a3bb3750c1dc0c77a538cb3770`; four exact-static rolling windows formed a strictly one-way constant-water enthalpy sweep across the canonical zero-enthalpy phase boundary while V40 root-drift descent and fixed-phase predicates correctly refused. | Direct numerical-trigger authority for one exact single canonical enthalpy-boundary crossing under constant exact water, promoted-root chaining, two-map cadence, and the unchanged solver/budget/admission path; no tolerance, root admission, residual, physics, or cap change. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V42` | Owner-authorized `WGHL-FULL-001D-V42` successor after the full persisted-restart fixture gate reached 69/71 and the two before/after snow-reappearance cases failed exact V32 endpoint-coordinate closure on support `72000..72060 s`. A bounded capture reconstructed `H_support=-33741.202423291608 J m^-2` and `H_state=-33738.881304185808 J m^-2`; their exact `+2.3211191058 J m^-2` difference is the Stage 3 cold-content export from removed sublimated ice, already exposed by the authentic evaluation but omitted from the V31/V32 support image. | Direct numerical-coordinate authority adding the already-physical authentic cold-content-export operand to canonical `W/H` reconstruction and private nonlatent contraction; no new physics, ledger math, tolerance, cap, floor, or acceptance authority. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V43` | Owner-authorized `WGHL-FULL-001D-V43` successor after retained r113 canonical failure `/tmp/wghl_001d_v42_64m_r113.log`, SHA-256 `9c1d35d0f34991bec6386cbef9b6ca1295f6ca2e281e9774ba4b6bade5df3188`; the exact first charged map at `1860..1920 s` failed before physics because the carrier reconstructed a V38 numerical-coordinate projection as an ordinary base unpublished trial. Exact identity/support passed, while the first unequal sealed field was predecessor custody: reconstructed `AcceptedReceiptChain` versus retained `NumericalCoordinateProjection`. | Direct private-custody authority retaining a distinct typed numerical-coordinate fixed-point posture; no acceptance, sequential-continuation, physics, exact-carry, tolerance, budget, or publication change. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V44` | Owner-authorized `WGHL-FULL-001D-V44` successor after retained r116 canonical audit `/tmp/wghl_001d_v43_64m_r116.log`, SHA-256 `b4046720e3719f4736408833bfa1fc32a23e9bfc95aca8219c760bef9c59aadb`; the first `1860..1920 s` solver coordinate reached strict weighted-OFE validation before reciprocal-longwave exchange was rebuilt. The forest tile residual was `423.500682899798 J m^-2 tile` (`160.9302595019235 J m^-2` after exact `0.38` weighting), while later trials changed sign and magnitude, proving a nonlinear uncommitted boundary mismatch rather than a fixed missing ledger term. | Direct evaluation-posture authority deferring aggregate weighted-OFE closure only for the uncommitted private LSE exchange, then requiring unchanged strict closure for authentic receipt probes, same-input replay, and finalization; also binds exact-once projected soil-coordinate consumption to the CN path and forbids V8 double application. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V45` | Owner-directed `WGHL-FULL-001D-V45` successor after retained r117 `/tmp/wghl_001d_v44_64m_r117.log`, SHA-256 `33ac890a9dbe05962363a0a5838b992d7ca2ad3c13e9fe2912f5555968748c5e`, and r118 `/tmp/wghl_001d_v44_64m_r118.log`, SHA-256 `fb65dfbfd53d4f587a416ffc97d6e1aca9a4d4a8cfd0ac2b49e925c265edc858`. The exact `1860..1920 s` support reached fully tolerance-closed Picard images at iterations 11 and 12, then the coupled root's authentic receipt tail exhausted the shared budget at exactly `used=maximum=96`; static charge tracing proves solver entry at used 13, no enforced post-root reserve, and a tolerance-closed but not bit-exact CN receipt fixed point. | Direct numerical authority for private authentic residual polishing plus hard shared-budget probe/replay reservation; no new residual, tolerance, receipt comparison, physical map, cap, or acceptance rule. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V46` | Owner-directed `WGHL-FULL-001D-V46` successor after retained r119 `/tmp/wghl_001d_v45_64m_r119.log`, SHA-256 `c9c7e3f19c46ee69815033c9734b17bb873f2ff545f879ad170dfbf94209fab1`, and r120 `/tmp/wghl_001d_v45_64m_r120.log`, SHA-256 `e00b4d4059560359f17c5c919834957d693bde2b2f7d2d55996ae1be0cb0fc53`. The exact failure reports solver entry at used 10, 67 ordinary-solver evaluations to used 77, 17 polishing evaluations to the two-slot reserve at used 94, then one nonstable receipt probe at used 95 with the independent replay slot protected. Static source tracing proves a safeguarded step begins charging finite-difference columns before it knows whether capacity remains for the trust trial that alone can update the carried root; aggregate evidence does not determine the exact reverse-column/backtrack cadence or number of wasted tail charges. | Direct budget authority for dimension-complete safeguarded-step preflight before any Jacobian column charge, retaining the unchanged generalized Jacobian, Newton/trust map, exact receipt/replay, maximum 96, and all physical/custody/publication rules. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V47` | Owner-directed `WGHL-FULL-001D-V47` successor after retained r121 `/tmp/wghl_001d_v46_64m_r121.log`, SHA-256 `bf703a976e5852a17b1a922d2086a9b2ce7786c4f459aa3cb79d2a346d3cca47`. Version 46 cleared the prior direct `60 s` receipt-stabilization blocker, then the composed `1800..1980 s` owner install rejected the authenticated soil target transaction because its atomic guard still required the outer vegetation/LSE/BGC source transaction to equal the soil target. Static owner tracing proves the second child retains exact source transaction 42 and authenticated soil target transaction 43 with exact expected predecessor 42. | Direct custody authority for a typed atomic complete-owner transaction posture: exact same source/target or exact authenticated soil predecessor equal to the mutually equal source owners, with soil state sealed to target and no adjacency inference. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V48` | Owner-directed `WGHL-FULL-001D-V48` successor after retained r122 `/tmp/wghl_001d_v47_64m_r122.log`, SHA-256 `20f5b118b43f69a35ce3e0ed03576bd916b3b4a9cb579692727f0438fb5de2bc`. V47 made the typed split posture exact, but the real ordinary fixed-point finalizer at composed `1800..1980 s` still invoked the generic same-ID installer after reconstructing a lawful prepared target 43 with predecessor/source 42, thereby erasing the explicit V39 authority before the guard. | Direct custody-propagation authority for one authenticated-prepared-beginning final install that validates the complete beginning/accepted/seal chain and carries an explicit source/soil-target authority; generic/public installation remains same-ID-only. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V49` | Owner-directed `WGHL-FULL-001D-V49` successor after retained r123 `/tmp/wghl_001d_v48_64m_r123.log`, SHA-256 `8c8a665317d06863b8d612780eb0b0280b5de977802487b5cdacbc81d466ee7b`, and exact r124 identity capture `/tmp/wghl_001d_v49_64m_r124.log`, SHA-256 `f596a10676bed83c1bc360ccaf034982e583922eac158f0d15468b0c98fbfd60`. At `1920..2040 s`, the fixed V11 parent retains mutually equal outer source 42 while the exact authenticated soil resident is 43 and its prepared successor is target 44 with predecessor 43. | Direct custody authority for an opaque three-domain authenticated prepared-install posture binding outer source, exact resident/predecessor, and prepared target without adjacency inference or outer-owner rebasing. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V50` | Owner-directed `WGHL-FULL-001D-V50` successor after retained r125 `/tmp/wghl_001d_v49_64m_r125.log`, SHA-256 `20c7e38a0a1aed2470b943d96b343749be1725db62edcee7ef3aba3cd9e34823`, and exact r129 failure capture `/tmp/wghl_001d_v50_64m_r129.log`. The composed finalizer's authenticated constitutive beginning is vegetation 41/LSE 40/BGC 41/soil 41, so applying the ending complete-owner mutual-source join to it is invalid. The separately validated covered V8 envelope owns the exact candidate ending source. | Direct custody authority binding the candidate's mutually equal ending source to the explicit validated `UncommittedCoveredV8OwnerEnvelope.transaction_id()` while retaining the heterogeneous beginning solely for exact soil resident/prepared custody; no adjacency, source copying, or owner rebasing. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V51` | Owner-directed `WGHL-FULL-001D-V51` successor after retained r130 `/tmp/wghl_001d_v50_64m_r130.log`, SHA-256 `43aee720db2758e47b166f96e726e307152c4fa14c82321564422062b9df728a`, and exact r132 capture `/tmp/wghl_001d_v51_64m_r132.log`, SHA-256 `db16c87e296f1a4756d9467e38fb1b36d7611df51b8275a483c3c33584600dbf`. At exact floor support `1920..1980 s`, four exact-static windows retain constant `W=0.3272909355676788 kg m^-2` and one adjacent canonical predicate crossing `0 -> 1`, then remain in predicate 1 while enthalpy corrections alternate and contract with binary64 endpoint-derived exact magnitudes `5069.96060020145, 2965.9686717161603, 997.5636528494651, 340.4305842210815 J m^-2`; V41 incorrectly requires one signed enthalpy direction even after the phase crossing. | Direct numerical-trigger authority admitting only the captured single adjacent direction-consistent phase crossing followed by finite nonstagnant alternating within-phase contraction with strictly decreasing exact step magnitude; unchanged solver, budget, authentic replay/finalization, closure, event, and 60-second floor. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V52` | Owner-directed `WGHL-FULL-001D-V52` successor after retained r133 `/tmp/wghl_001d_v51_64m_r133.log`, SHA-256 `6291dab02a435a46c4f13646fe8898ade184029ec9cbca75bb7739bab4b2ebcb`, and exact r134 capture `/tmp/wghl_001d_v52_64m_r134.log`, SHA-256 `cf276951616c509f71bf2f33dc2192e096d5367768ee43062e66f9e37a8d39f0`. At exact floor support `2100..2160 s`, the tolerance-closed private root exhausted its receipt reserve in an exact two-cycle: ending snow temperature `263.2042297771622/263.20422977716225 K`, ending flux `-88.24563334437782/-88.24563334437724 W m^-2`, and snow-candidate heat, positive into snow, `+5340.494294593449/+5340.494294593433 J m^-2`. The private map eliminated CN heat algebraically, whereas the authentic map consumed the sealed receipt heat; that omitted continuous coordinate is the exact cause. | Direct equation authority adding one snow-candidate CN heat coordinate `Q_cn,l` in `J m^-2` (positive into snow) and physical residual `R_Q,l=Q_cn,l-Q_cn,physical,l`, using the unchanged lane energy tolerance and exact typed CN operand; exact receipt equality/replay remains the only authentic admission. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V53` | Owner-directed `WGHL-FULL-001D-V53` successor after r135 `/tmp/wghl_001d_v52_64m_r135.log`, SHA-256 `38949328585ea32604a9d7de637540012c478377a6b62d171b2b9486f37e328f`, and exact r136 `/tmp/wghl_001d_v53_64m_r136.log`, SHA-256 `0963a182818d2f0791a4eb6d14da53df746807c29fa8dcd516296d672860e0e1`. At exact floor support `1860..1920 s`, the six-coordinate solve began with cross-map retained `Q=12481.284398406831 J m^-2` and exhausted at `88/96`; its latest charged coordinate/output pair was `8133.824322945977/8133.824323886676 J m^-2`, giving `R_Q=-9.406994649907574e-7 J m^-2`, while aggregate merit remained `2.992164809256792`. Static trace proved that fresh and legacy V52 seeds alone sourced Q from the retained accepted receipt instead of the already-produced Stage 3/soil endpoint associated with solver dispatch. | Direct representational seed authority requiring Q to be reconstructed from that exact already-produced endpoint candidate pair, without a physics call or charge; the non-Q seed coordinates and every V52 equation, residual, tolerance, map, budget, receipt, and admission rule remain unchanged. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V54` | Owner-directed `WGHL-FULL-001D-V54` successor after r137 and exact r138 `/tmp/wghl_001d_v54_64m_r138.log`, SHA-256 `28f55b679c8eaae874fdfead55c992e16ceaff559925c71b78cd63b49068a6e7`. At exact floor support `2100..2160 s`, the private root was already tolerance-closed (`scaled_merit=1.9099388737231493e-5`, `R_Q=-1.9099388737231493e-11 J m^-2`) but three authentic probes detected an exact receipt cycle at budget `84/96`. One observed edge changed snow temperature by one binary64 ULP, `263.204229777162197 -> 263.204229777162254 K`, while soil temperature was bit-equal; receipt Q changed `5340.494294593449 -> 5340.49429459343264 J m^-2` and both ending-owner and receipt digests changed. | Narrow exact finite-cycle authority: reconstruct each cycle member's own complete endpoint coordinate from its already-charged artifacts and output receipt, run a charged full authentic map with that own receipt, retain only exact fixed points under unchanged closure, then require independent exact replay. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V55` | Owner-directed `WGHL-FULL-001D-V55` successor after r139 and exact r140 `/tmp/wghl_001d_v55_64m_r140.log`, SHA-256 `3482ada5075aa921fc1e71d0f5fa253765b009fa4e833b05f3e9edc598147628`. At exact floor support `2100..2160 s`, the V54 own-artifact search evaluated both exact cycle members at shared budget `84 -> 86/96`, but neither was a fixed point. Member 0 mapped positive-into-snow Q `5340.494294593433 -> 5340.494294593502 J m^-2`; member 1 mapped `5340.494294593449 -> 5340.494294593433 J m^-2`. Both retained tiny full residuals, exact derived-z, and the unchanged branch. Exact r142 `/tmp/wghl_001d_v55_64m_r142.log`, SHA-256 `4c4d63a75b39494b005cacc21d1d2777c03faeebff7af503a44d80ca1ebf7473`, then proved a different valid root at budget `30/96`, merit `0.010248...`, with a 1394-member Q interval requiring 1396 charges: the finite specialization was inapplicable while unchanged V45 polishing still had capacity. | Narrow pre-polish representational authority to exhaustively enumerate a finite positive binary64 Q lattice only when the complete interval and authentic reserve fit atomically; a valid zero-charge specialization miss preserves the unchanged V45 polish path, while any committed attempt remains fail-closed. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V56` | Owner-directed `WGHL-FULL-001D-V56` successor after exact r144 `/tmp/wghl_001d_v55_64m_r144.log`, SHA-256 `161712621295b503da41b065846304ce0e0198a26a9d9b97efa6d4012fa36c65`, wall `6:46.42`, RSS `442360 KiB`. At exact-floor support `2100..2160 s`, the first tolerance-closed private root entered V55 near shared budget 63 and exhaustively charged all 21 representable Q candidates through budget 84. No exact witness existed: the last candidate had Q bits `4662593950276069748`, reconstructed-output bits `4662593950276069730`, nonzero `R_Q` bits `4445615782168100864`, and merit `2.1827872842550278e-5`. This disproves promotion of a transient Q-lattice or cycle artifact as authenticated material authority; it does not disprove an exact frozen material state with retained enthalpy remainder. | Direct representational, custody, and restart authority for a strictly frozen/noncrossing temperature-primary solve, exact snow enthalpy high-plus-carry, authenticated compound snow material ownership, additive carry receipts, and exact whole-receipt replay/finalization. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |
| `REF-SNOWENERGY-WGHL-V57` | Owner-directed `WGHL-FULL-001D-V57` successor after exact r147 `/tmp/wghl_001d_v56_64m_r147.log`, SHA-256 `1b95e317d65cf831933ef7778f20c7295ef2e590a199b986fcfee8dc97b759fc`, wall `6:44.93`, RSS `442964 KiB`. At exact-floor support `2100..2160 s`, V56 remained zero-charge ineligible solely because the immutable external-liquid ledger operand was `2.7404676319867775e-15` to `5.480935263973555e-15 kg m^-2 OFE-ground`; the legacy root then reached budget 63 and V55 lawfully found no Q witness. | Direct numerical-dispatch authority for an eligibility-only `1.0e-12 kg m^-2` external-liquid roundoff bound and a zero-charge pre-V55 transition from the already-produced tolerance-closed legacy root. The exact liquid operand remains unchanged in all physics and ledgers. | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` |

## Variables and Units

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `T_a` | `K` | Hourly above-canopy air temperature. | hourly climate forcing | atmospheric longwave |
| `e_a` | `kPa` | Daily actual vapor pressure held across the declared hourly evaluations. | daily climate forcing | precipitable-water proxy |
| `w` | `kg m^-2` | Precipitable-water proxy, `4650 e_a / T_a`. | atmospheric longwave | clear-sky equation |
| `L_clear` | `W m^-2` | Clear-sky downward longwave irradiance. | atmospheric longwave | clear-sky emissivity |
| `epsilon_clear` | `dimensionless` | Effective clear-sky emissivity. | atmospheric longwave | cloud mixture |
| `R_s` | `MJ m^-2 d^-1` | Daily incident above-canopy shortwave radiation. | climate forcing | clearness index |
| `R_a` | `MJ m^-2 d^-1` | Daily extraterrestrial radiation on the horizontal surface. | solar geometry | clearness index |
| `k_t` | `dimensionless` | Daily clearness index, `R_s/R_a`. | cloud inference | cloud fraction |
| `c` | `fraction` | Effective daily cloud fraction. | cloud inference | all-sky emissivity |
| `epsilon_all` | `dimensionless` | Effective all-sky emissivity. | atmospheric longwave | atmospheric irradiance |
| `L_atm` | `W m^-2` | Hourly all-sky downward atmospheric longwave above canopy. | atmospheric longwave | sub-canopy mixture |
| `C` | `fraction` | Effective plan-view overhead canopy interception fraction. | plant/canopy state | sky-view translation |
| `P_0` | `fraction` | Vertical canopy gap fraction, `1-C`. | sky-view translation | Beer-law elimination |
| `f_sky` | `fraction` | Hemispherical diffuse sky-view/transmission factor. | sky-view translation | sub-canopy mixture |
| `T_c` | `K` | Effective radiating canopy temperature. | EB-03 provider boundary | canopy emission |
| `L_can` | `W m^-2` | Effective canopy longwave emission, `sigma T_c^4`. | canopy emission | sub-canopy mixture |
| `L_sub` | `W m^-2` | Downward longwave incident at the snow surface. | sub-canopy mixture | snow energy carrier |
| `T_s` | `K` | Snow-surface radiating temperature. | EB-03 provider boundary | snow emission |
| `L_out` | `W m^-2` | Upward longwave emitted by snow. | snow emission | net longwave |
| `L_net` | `W m^-2` | Net longwave, positive toward snow. | longwave balance | snow energy carrier |
| `z_0` | `m` | Active thermal-layer depth, normally `min(z_s, 0.25 m)`; whole-pack depth when `INV-SNOWENERGY-026` collapses a lower volume with `0 < m_l < 1 kg m^-2`. | Stage 3 thermal partition | shared energy carrier |
| `z_T` | `m` | Virtual air-temperature transfer height above the instantaneous modeled snow surface; `5 m` for current Stage 3 geometry, not a physical measurement-height claim. | model geometry | turbulent sensible exchange |
| `z_q` | `m` | Virtual humidity/vapor-pressure transfer height above the instantaneous modeled snow surface; `5 m` for current Stage 3 geometry, not a forcing reference-height claim. | model geometry | turbulent latent exchange |
| `z_u` | `m` | Virtual momentum transfer height above the instantaneous modeled snow surface; `5 m` in current Stage 3 geometry and distinct from nominal GRIDMET `10 m` wind reference height. | model geometry | turbulent momentum exchange |
| `z_u,source` | `m` | Physical/source-product wind reference height relative to the source-product land/model surface; nominal `10 m` for GRIDMET `vs` only when exact GRIDMET source identity is established. | forcing-source custody | source applicability |
| `u_cli` | `m s^-1` | Daily CLI `w-vl`, parsed as raw `vwind`/`vwind_m_s` and consumed unchanged by Stage 3. Retained provider evidence proves daily parquet-to-CLI equality; nearby historical code statically reconstructs watershed-centroid request, run-level sharing, and one-decimal formatting. Deployed request and server-side pixel/version/timezone/fill semantics remain `AUTHORITY_MISSING`. | CLI/runtime boundary | Stage 3 and PMET input |
| `u_2,PMET` | `m s^-1` | PMET-local FAO-56 adjustment of raw wind to `2 m`; never a Stage 3 input or exposure authority. | PMET local | evapotranspiration only |
| `z_0,aero` | `m` | Aerodynamic roughness length of the exposed snow surface; fixed `0.005 m` for the admitted Stage 3 snow surface. This is distinct from active-layer depth `z_0`. | snow-surface authority | turbulent exchange |
| `m_0` | `kg m^-2` | Snow-ice mass contained in `z_0`. | Stage 3 thermal partition | active-layer heat capacity |
| `T_0` | `K` | Heat-capacity-weighted active-layer temperature. | active-layer cold content | radiation and turbulent exchange |
| `T_l` | `K` | Heat-capacity-weighted lower-pack temperature when `z_s > z_0`. | lower-pack cold content | interface conduction |
| `G_0` | `W m^-2` | Conductive exchange, positive from the lower pack into the active layer. | coupled thermal provider | active/lower energy balances |
| `p_a` | `Pa` | Atmospheric pressure derived from run elevation. | climate metadata projection | SNOBAL effective snow conductivity |
| `k_d`, `k_eff` | `W m^-1 K^-1` | Dry Yen snow conductivity and Anderson pore-vapor-enhanced effective conductivity. | density, temperature, pressure | active/lower series resistance and `G_0` |
| `Q_cc` | `J m^-2` | Positive active-layer cold-content deficit relative to `0 degC` ice. | Stage 3 thermal partition | shared energy carrier |
| `m_v,raw` | `kg m^-2` | Signed vapor exchange opportunity integrated from turbulent mass flux before snow-ice availability bounding; deposition positive, sublimation negative. | evaluation-only schema-v6 carrier | attribution diagnostic; never actual S/F debit |
| `m_v` | `kg m^-2` | Signed bounded vapor transfer; deposition positive, sublimation negative. | bounded Stage 3 transfer | actual snow mass and canonical latent-energy ledgers |
| `alpha_v` | `dimensionless` | Unique strict-convex fraction from a pure one-sided current vapor image to an opposite-sided authentic image at which signed actual vapor is exact zero. | version-32 unpublished active-set localizer | numerical operand reconstruction only |
| `Q_v` | `J m^-2` | Already-once latent-energy component linked to actual vapor by positive finite specific latent heat at each authentic one-sided endpoint; exact positive zero at the localized interface. | bounded Stage 3 transfer | complete-energy operand vector and version-32 active-set guard |
| `X_c` | `J m^-2` | Nonnegative cold-content magnitude exported from the represented snow control volume with removed ice during the authentic Stage 3 evaluation. It is already physical result data, not a new energy source or ledger correction. | authentic Stage 3 result | V31/V32 and successor canonical endpoint enthalpy reconstruction |
| `L_s,auth` | `J kg^-1` | Positive finite authentic endpoint specific latent heat retained only when entering from the exact-zero interface toward that fresh authentic one-sided branch. | fresh authentic map image | version-32 unpublished branch entry |
| `W_1,l`, `H_1,l` | `kg m^-2`, `J m^-2` | Reduced ending total represented snow water and snow enthalpy for affected lane `l`, both relative to the immutable support beginning. | version-33 private coupled solver | canonical `Pi(W,H)` and exact water/energy residuals |
| `T_s,1,l` | `K` | Version-56 temperature-primary ending snow temperature for affected frozen lane `l`; it is finite, positive, strictly below exact `273.15 K`, and never crosses the authenticated Stage 3 phase branch. | version-56 private coupled solver | exact frozen snow enthalpy reconstruction and unchanged Stage 3/CN map |
| `H_hi,l`, `R_Hcarry,l` | `J m^-2`, exact dyadic `J m^-2` | Canonical round-nearest-even binary64 high word and exact additive remainder of `H_exact,l=-exact(W_1,l)*exact(c_ice)*(exact(273.15 K)-exact(T_s,1,l))`; their exact sum is the material enthalpy. | version-56 exact reconstruction | authenticated compound snow owner, exact energy ledger, carry receipt, restart |
| `Q_cn,l` | `J m^-2` | Complete-support snow-candidate Crank--Nicolson heat, positive into snow. Version 56 derives it exactly once from unchanged CN physics and the temperature-primary snow/soil coordinates; it is not an independent V56 coordinate. | unchanged CN snow--soil operation | snow `+Q`, soil `-Q`, receipt stabilization/replay |
| `Pi(W,H)` | typed phase projection | Unchanged version-22 canonical projection from total water/enthalpy to ice, liquid, cold content, and terminal unallocated energy, including exact `H=0` and `H=L_f W` sides. | `SC-SNOWENERGY-001@22` | every version-33 residual and final replay evaluation |
| `E_soil,1,n`, `T_soil,1,n` | `J m^-2`, `K` | Only the coupled ending soil-node enthalpy/temperature coordinates already required by the existing soil thermal state and CN snow--soil block; no unrelated soil or LSE coordinate is an unknown. | existing LSE soil-thermal owner | version-33 reduced residual and final replay |
| `R_W,l`, `R_H,l`, `R_E,n`, `R_T,n` | `kg m^-2`, `J m^-2`, `J m^-2`, `K` | Exact lane-water, complete ordered lane-energy, soil-CN endpoint enthalpy, and soil enthalpy--temperature residual carriers returned together by one sealed version-33 physical evaluation. | `CoveredPhaseConsistentResidualEvaluationV1` | safeguarded semismooth coupled solve, algebraic side constraints, and final root guard |
| `T_ca`, `q_ca` | `K`, `kg kg^-1` | Shared canopy-air temperature and specific humidity solved by the coupled carrier. | shared carrier transaction | V11 canopy and Stage 3 snow turbulent exchange |
| `H_i`, `V_i` | `W m^-2`, `kg m^-2 s^-1` | Sensible and vapor exchange between participant `i` and the shared carrier. | shared carrier transaction | canopy/snow flux ledgers |
| `L_can`, `L_snow<->canopy` | `W m^-2` | Area-weighted component canopy emission and reciprocal canopy/snow longwave exchange. | V11/LSE canopy components | snow longwave and reciprocal ledger |
| `Q_E` | `J m^-2` | Hour-integrated applied surface energy, positive toward snow. | shared energy carrier | cold-content update |
| `Q_complete` | `J m^-2` | Exact-one sum of all admitted external and ground/interlayer energy operands for the declared substep/control volume, before phase-change allocation. | future complete Stage 3 energy carrier | cold-content and phase allocation |
| `Q_cold_required` | `J m^-2` | Non-negative energy required to bring the melt-owning ice to its phase threshold after active/lower allocation. | future Stage 3 thermal state | positive-excess derivation |
| `Q_excess` | `J m^-2` | Non-negative energy remaining after active/lower cold-content satisfaction in the same stability substep. | future complete Stage 3 energy ledger | bounded phase change |
| `delta_E_cold` | `J m^-2` | Signed increase in below-phase-threshold snow internal energy over the substep; positive means warming. | future Stage 3 energy ledger | independent energy closure |
| `Q_refreeze` | `J m^-2` | Latent energy released by same-substep refreeze, exactly `L_f m_refrozen`. | future Stage 3 phase ledger | cold-content/energy closure |
| `L_f` | `J kg^-1` | Latent heat of fusion used by the admitted phase conversion. | fixed physical constant | bounded phase change |
| `m_ice_available` | `kg m^-2` | Ice available for melt after same-substep solid precipitation and reservation of the already evaluated bounded sublimation mass; later deposition does not enlarge it. | future Stage 3 phase state | joint melt/vapor availability bound |
| `m_melt` | `kg m^-2` | Ice converted to liquid by Stage 3 in one substep, bounded by available ice. | future Stage 3 melt owner | same-substep liquid disposition |
| `m_liquid_external_in` | `kg m^-2` | Liquid entering the Stage 3 control volume during the substep, excluding the retained store already present at substep start. | precipitation/upstream handoff | liquid-disposition ledger |
| `delta_m_retained` | `kg m^-2` | Signed retained-liquid store change, `m_liquid_phase_end-m_liquid_phase_start`. | Stage 3 liquid state | liquid-disposition ledger |
| `m_refrozen`, `m_routed` | `kg m^-2` | Same-substep liquid refrozen to ice and liquid exported after holding-capacity disposition. | Stage 3 phase/liquid solve | linked mass ledgers |
| `m_solid_precip`, `m_deposition`, `m_sublimation` | `kg m^-2` | Same-substep solid precipitation, vapor deposition to ice, and sublimation from ice, each non-negative and distinct. | precipitation and signed vapor exchange | complete solid ledger |
| `P_phase` | ordered set | Sealed precipitation phase parcels for one exact support, in canonical destination then phase/source order. Every parcel binds lane/OFE, destination tile, phase, mass basis, support, atmospheric or vegetation producer-state identity, temperature/enthalpy provider, and receipt identity. | atmospheric forcing plus vegetation terminal-liquid owner | Stage 3 precipitation mass and advection consumers |
| `m_precip,p`, `Q_adv,p` | `kg m^-2 tile-ground`, `J m^-2 tile-ground` | Parcel mass and precipitation-advected heat reconstructed from the same parcel identity; OFE-ground lane values are `sum_p(f_destination,p * value_p)` with each parcel consumed exactly once. | sealed phase parcel | Stage 3 mass and complete-energy ledgers |
| `T_sb,0`, `T_sb,1` | `K` | Beginning and candidate-ending temperatures of the bottom represented Stage 3 snow thermal volume. | Stage 3 lane owner | snow side of lower boundary |
| `dz_sb`, `lambda_sb` | `m`, `W m^-1 K^-1` | Positive physical thickness and conductivity of that bottom snow thermal volume. | Stage 3 configuration/state | lower-boundary resistance |
| `T_1,0`, `T_1,1` | `K` | Beginning and candidate-ending temperatures of the first ordered OFE soil-thermal node. | soil-thermal owner | soil side of lower boundary |
| `dz_1`, `lambda_1` | `m`, `W m^-1 K^-1` | Positive thickness and conductivity of the first ordered OFE soil-thermal node. | LSE/soil-thermal configuration | lower-boundary resistance |
| `G_ss` | `W m^-2 OFE-ground` | Snow--soil conductive heat, positive downward from snow to soil; Stage 3 is debited and soil thermal is credited by the identical accepted amount. | joined Stage 3/soil-thermal candidate | complete energy and soil ledgers |
| `Q_unallocated_after_exhaustion` | `J m^-2` | `Q_excess-L_f m_melt`, non-negative energy remaining only when the available-ice bound saturates. | future energy/phase ledger | terminal meltout hold |
| `m_res` | `kg m^-2` | Total ice-mass boundary above which the Marks/SNOBAL Stage 3 thermal layer is resolved. | fixed libsnobal threshold | Stage 3 domain branch |
| `t_unres` | `s` | Duration for which CoE snow exists below the resolved Stage 3 thermal domain. | Stage 3 domain branch | diagnostics and runtime evidence |
| `m_l` | `kg m^-2` | Ice mass in the selected lower thermal volume. | Stage 3 thermal partition | one/two-volume branch |
| `t_collapse` | `s` | Duration for which a resolved pack uses one thermal volume because `0 < m_l < m_res`. | Stage 3 layer branch | diagnostics and runtime evidence |
| `SWE_layer` | `m` | Persistent layer water-equivalent mass depth. | typed `DirectSnowLayerState` | named lifecycle conversion |
| `m_layer` | `kg m^-2` | Persistent layer areal water mass used for lifecycle selection. | named SWE-to-area-mass conversion | density lifecycle |
| `z_layer` | `m` | Persistent layer physical thickness. | typed `DirectSnowLayerState` | density closure and Stage 3 |
| `sigma` | `W m^-2 K^-4` | Stefan-Boltzmann constant. | fixed constant | emission equations |

## Algorithm State Surfaces

### Required inputs

| Surface | Required state |
|---|---|
| Above-canopy meteorology | hourly finite `T_a > 0 K`; daily finite `e_a >= 0 kPa` and `R_s >= 0 MJ m^-2 d^-1` |
| Turbulent forcing geometry | typed positive virtual `z_T`, `z_q`, and `z_u` plus typed positive `z_0,aero`, all relative to the instantaneous modeled snow surface; current geometry is exactly `5 m`, `5 m`, `5 m`, and `0.005 m`; none asserts physical forcing reference height |
| Solar geometry | finite `R_a >= 0 MJ m^-2 d^-1` plus an explicit daylight/polar-night classification |
| Canopy | finite effective daily `C` in `[0, 1)` |
| Thermal provider | supported internal `layered_thermal_liquid_v1`; finite active-layer `T_0 > 0 K`, non-negative finite active/lower cold content, conservative depositional-to-thermal partition, and `T_c=T_a` with the named approximation identity |
| Future melt-owner energy producers | finite, unit-explicit, same-substep net shortwave/longwave radiation, sensible heat, bounded latent exchange, ground/interlayer conduction, and precipitation-advected heat with exact-one lineage; unavailable components block cutover |
| Future phase/liquid inputs | active/lower ice and retained liquid at substep start, solid/liquid precipitation, and one signed vapor exchange, all in explicit area-mass units and chronology |
| Stage 3 precipitation custody | One sealed ordered `P_phase` for every support, including a complete empty set for zero precipitation; exact LSE destination topology and fractions; open raw-liquid or covered vegetation-terminal-liquid source selected exclusively per destination; solid atmospheric precipitation bypasses canopy and remains ground-snow precipitation. |
| Version-32 numerical transition | Two independently closed pure one-sided complete-support images from one immutable beginning, exact identical support at least `60_000_000_000 ns`, exact identical structural/event/topology/custody/receipt identities, strictly opposite finite nonzero actual vapor, linked finite latent components, finite external liquid and ordered nonlatent energy components, and raw authentic history. |
| Version-33 reduced coupled solve | Identical immutable support/source/event/topology/custody/receipt joins and an exact transition record `root/interface -> one-sided branch-entry -> opposite pure-vapor raw-authentic -> same root/interface/reset coordinates and branch predicates with opposite pure vapor disposition`; no bitwise equality requirement on asymptotically changing raw-authentic continuous owner fields; affected-lane `W_1/H_1`; only coupled soil endpoint enthalpy/temperature coordinates; sealed `CoveredPhaseConsistentResidualInputsV1`; existing covered-LSE/CN/receipt operands and side constraints; and one shared `CoveredPhysicalEvaluationBudgetV1` with the remaining count from the unchanged 96 total. |
| Version-34 stable-monotone eligibility | Eight consecutive raw authentic Picard maps on one event-free terminal-one-volume covered support `>=60_000_000_000 ns`; exact unchanged support/source/event/topology/custody/static-receipt/phase-branch/carry-authority-and-representation joins while physical receipt digests and exact `H_hi/R` coordinates may evolve; exact `E=exact(H_hi)+R` reconstruction; finite `R_W/R_H/R_E/R_T`; strictly decreasing governed residual merit; no `A/B/A`, active-set transition, or finalization restart; and the same `CoveredPhysicalEvaluationBudgetV1` already charged for all eight maps. |
| Version-35 authentic receipt stabilization | A coupled physical root plus immutable beginning owners and sealed sources; canonical input receipt set `R_n`; independently reconstructed/resealed output `R_(n+1)`; exact canonical receipt bytes and authenticated fields; physical residual and candidate-artifact bytes; and the same `CoveredPhysicalEvaluationBudgetV1` already charged by all prior maps/solver evaluations. The first root reseal is a probe under a different input and is not an equality candidate. |
| Version-36 geometry-complete solve | Per affected terminal-one-volume lane: one `rho_1,l` coordinate; canonical `I_1,l` from `Pi(W,H)`; reconstructed `z_1,l=I_1,l/rho_1,l`; unchanged layer/order, settling-authority, and density-model-branch joins; physical `R_rho,l`; generalized `R_W/R_H/R_rho/R_E/R_T`; and the same shared budget and v35 receipt-stabilization inputs. |
| Version-56 frozen temperature-primary solve | An authentic terminal-one-volume Stage 3 branch proven strictly frozen and noncrossing for the complete support; exact unchanged lane/order/density/phase/event/topology/custody/static-receipt joins; finite ordered coordinates `(W_1,l,T_s,1,l,rho_1,l)` followed by top-soil `(E_soil,1,n,T_soil,1,n)`; exact `c_ice` and `273.15 K`; unchanged CN operands; authenticated beginning compound snow material owner and additive carry-receipt lineage; and the same already-charged maximum-96 physical-evaluation account. |

### Required outputs

`w`, `L_clear`, `epsilon_clear`, `k_t`, `c`, `epsilon_all`, `L_atm`,
`P_0`, `f_sky`, `L_can`, `L_sub`, `L_out`, and `L_net`, each with the
units and lineage declared above.

The future melt-owner path additionally publishes `Q_excess`, `m_melt`,
`m_refrozen`, `delta_m_retained`, `m_routed`, refreeze latent energy, terminal
unallocated energy, and independently reconstructable complete-energy,
solid-to-liquid, and liquid-disposition ledger operands. Their absence is a
cutover blocker, not permission to infer them from adjacent state or output.

Version 32 produces only unpublished numerical iterate images. It adds no
runtime output, schema, persisted field, diagnostic, receipt, or owner surface.
Version 33 likewise produces no public or persisted numerical surface. A
private trial may expose residuals only within the solve call; production must
not retain or persist cycle, trust-region, Jacobian, or residual diagnostics.
Version 56 adds no solver diagnostic or public witness. Only an authentically
finalized compound snow material owner may expose ordered `H_hi/R_Hcarry`, and
only its versioned restart/checkpoint schema may persist committed, pending, or
in-progress compound-owner state.

### Mutated state surfaces

The longwave evaluator is pure. It may not mutate canopy, snow mass, snow
temperature, cold content, or forcing state. The shared Stage 3 energy carrier
consumes every admitted component exactly once. In the version-7 target it is
solely responsible for mutating cold content and converting bounded positive
excess to melt. Optional sublimation uses the same pre-exchange `T_s`; signed
vapor exchange and latent heat are two views of one transfer.
Version-32 interface and branch-entry images may replace only the next private
covered fixed-point iterate; they may not mutate an accepted owner or any
publication, receipt, restart, checkpoint, or rollback surface.
On an exact v33 cycle they are historical oracles only and do not control the
next production iterate. Version-33 trial coordinates and projected states are
private and non-mutating. Only the successful coupled-authentic final replay
may stage ordinary candidate owners and receipts for unchanged atomic commit.
Version-56 private coordinates, exact intermediate enthalpy, and derived CN
heat remain non-mutating. A compound snow owner and additive carry receipt may
replace accepted state only after exact whole-receipt stabilization,
independent same-input replay, and unchanged strict finalization; every failure
restores prior owner and restart bytes atomically.

## Algorithm Specification

Required evaluation order:

1. Validate cadence, units, finiteness, and physical input domains.
2. Once per daylight day, calculate `k_t` and the bounded cloud fraction `c`;
   otherwise take the explicit polar-night unavailable branch.
3. For each hour, evaluate `w`, `L_clear`, `epsilon_clear`,
   `epsilon_all`, and `L_atm` using hourly `T_a` and held daily `e_a`/`c`;
   enforce the no-clamp derived-emissivity guard.
4. Translate the current effective canopy cover to `P_0` and `f_sky`.
5. Partition the current snow column at
   `z_0=min(total_snow_depth, 0.25 m)` except for the `INV-SNOWENERGY-026`
   strict lower-volume collapse, integrating mass, heat capacity, cold
   content, and thermal resistance across depositional-layer boundaries.
   Obtain `T_s=T_0` from the active control volume and set `T_c=T_a` under the
   named homogeneous-stand approximation; stop on missing or invalid state.
6. Evaluate `L_can`, `L_sub`, `L_out`, and `L_net` in the specified order.
7. If sublimation is enabled, evaluate one signed vapor exchange at the same
   `T_s`, bound its loss against post-precipitation ice, derive latent heat from
   that exact bounded exchange, and reserve its sublimation mass before
   calculating melt availability. Define
   `m_deposition=max(m_v,0)`, `m_sublimation=max(-m_v,0)`, and
   `m_ice_available=max(m_ice_after_solid_precip-m_sublimation,0)`.
   For current Stage 3 transfer, use the version-10 virtual geometry
   `z_T=z_q=z_u=5 m` and `z_0,aero=0.005 m`; validate each typed length and the
   logarithmic displacement/roughness domain before flux arithmetic.
8. Select the `60`, `15`, or `1 minute` stability substep from the
   Marks/SNOBAL active/lower mass thresholds and reevaluate `T_0`, vapor
   exchange, radiation, and `G_0` at every substep.
9. For the admitted target, sum complete net radiation, sensible heat, latent
   heat, ground/interlayer conduction, and precipitation-advected heat once in
   the applicable active/lower balances. Apply energy to cold content first.
10. Define `Q_excess=max(Q_complete-Q_cold_required,0)` after the active/lower
    allocation and convert only that positive remainder to bounded melt:
    `m_melt = min(Q_excess/L_f, m_ice_available)`. Debit that exact ice mass,
    credit the same liquid mass, and apply same-substep refreeze with its latent
    energy explicitly coupled to cold-content state.
11. Apply the previously evaluated signed vapor mass exchange after
    melt/refreeze, then wet compaction and holding-capacity retention/routing;
    repartition state before the next substep. Publish independently
    reconstructable energy, solid-to-liquid, and liquid-disposition ledgers
    after all guards pass.

The normative mass chronology is precipitation -> energy balance ->
melt/refreeze -> vapor mass mutation -> wet compaction -> retention/runoff.
With all mass operands non-negative except signed store change, the target
identities are:

```text
Q_complete + Q_refreeze - delta_E_cold - L_f*m_melt - Q_unallocated_after_exhaustion = 0
m_ice_start + m_solid_precip + m_deposition - m_ice_end - m_sublimation - m_melt + m_refrozen = 0
m_liquid_external_in + m_melt - m_refrozen - delta_m_retained - m_routed = 0
```

`Q_refreeze=L_f*m_refrozen`. The retained-store operand is
`delta_m_retained=m_liquid_phase_end-m_liquid_phase_start`; its initial level
is not also counted as external input. Define
`Q_unallocated_after_exhaustion=Q_excess-L_f*m_melt`. It must be zero in every
currently admitted resolved substep. A positive value identifies the
unresolved terminal meltout/remaining-energy boundary: it may not be discarded,
carried, or routed by proxy and blocks cutover until a contract amendment
defines its physical recipient and next-state chronology.

Steps 9-11 describe target authority, not current runtime conformance. The
current implementation omits complete sensible and precipitation-advected
heat from this carrier, does not convert `unused_positive_energy_j_m2`, and
keeps CoE as the melt generator. It must remain on `IMPLEMENTATION_HOLD` until
one implementation package closes every component, thin-pack, selector,
default, real-consumer, and rollback gate atomically. A partial energy-melt
path or simultaneous CoE/Stage 3 generation is forbidden.

### Atmospheric longwave

For each hourly evaluation, use hourly air temperature `T_a` in kelvin and
the daily actual vapor pressure `e_a` in kilopascals:

```text
w = 4650 e_a / T_a
L_clear = 59.38
          + 113.7 (T_a / 273.16)^6
          + 96.96 sqrt(w / 25)
epsilon_clear = L_clear / (sigma T_a^4)
```

The constants and units are inseparable from this equation. A vapor pressure
in pascals or temperature in degrees Celsius is invalid.

Evaluate `w`, `L_clear`, `epsilon_clear`, `epsilon_all`, and `L_atm` at the
hourly `T_a`; do not substitute daily mean temperature into the nonlinear
`T_a^6` and `T_a^4` equations. The daily clearness-derived `c` and daily
`e_a` are held constant across that day's hourly evaluations. A future
subdaily humidity/cloud route requires a contract amendment.

When daylight permits a daily clearness index and
`R_a > R_a,min = 1e-9 MJ m^-2 d^-1`:

```text
k_t = R_s / R_a
c = clamp((0.80 - k_t) / (0.80 - 0.15), 0, 1)
epsilon_all = (1 - 0.84 c) epsilon_clear + 0.84 c
L_atm = epsilon_all sigma T_a^4
```

`R_a,min` is a numeric divide/branch threshold, not a user coefficient or
empirical calibration parameter. The clamp belongs only to the declared
empirical cloud mapping. It must not repair a non-finite input or an invalid
radiation unit.

When `R_a <= R_a,min`, the clearness route is unavailable. Version 2 has no
independently authoritative polar-night cloud producer, so an enabled
longwave cell returns typed `CloudForcingUnavailable`. It must not reuse the
legacy SIMIMPL28 cloud fraction or a prior daylight value. Disabled longwave
cells do not require this operand.

Flerchinger et al. report approximately `24.5 W m^-2` subdaily RMSD and
`14.9 W m^-2` daily RMSD for the Dilley-Unsworth combination across their
development comparison. These are model-form context, not openWEPP
calibration tolerances or guaranteed errors. A daily clearness operator cannot
recover observed subdaily cloud variation, and solar-index inference is
undefined during polar night.

The reviewed authority does not provide a transferable numeric
temperature/humidity envelope for every openWEPP climate. Therefore each
evaluation must require finite `L_clear`, `epsilon_clear`, `epsilon_all`, and
`L_atm`, with `0 <= epsilon_clear <= 1` and `0 <= epsilon_all <= 1`.
Out-of-range derived emissivity is typed `out-of-authority`; it is not clamped
to one. Passing this physical output guard does not assert site validation.

### Effective canopy cover to diffuse sky view

`C` is the effective overhead interception state already produced by the
canopy model. Identify its complement with vertical Beer-law gap fraction:

```text
P_0 = 1 - C = exp(-k_ext VAI_eff)
```

FSM2's hemispherical diffuse transmission is:

```text
f_sky = exp(-1.6 k_ext VAI_eff)
```

Eliminating the unobserved product `k_ext VAI_eff` gives the canonical
openWEPP translation:

```text
f_sky = P_0^1.6 = (1 - C)^1.6
```

This is a model-state translation, not a fitted coefficient. The exponent is
the FSM2 diffuse-transmission factor. The effective extinction coefficient
cancels algebraically, so openWEPP does not request it from the user.

The translation treats the complete effective cover state—including the
native structural floor—as equivalent vertical optical depth under the same
homogeneous, randomly oriented canopy and isotropic diffuse-sky regime. It
does not assert that structural cover is a measured stem-area index.

`LAI`, native structural cover, and canopy height are not separately added:

- daily `C` already includes the leaf-on/leaf-off canopy trajectory and the
  structural-cover floor;
- structural cover is a fraction, not stem-area index, so adding it to LAI
  would be dimensionally and semantically invalid;
- adding LAI again would double count foliage represented in `C`;
- homogeneous Beer-law gap transmission contains no independent height term.

Height and LAI remain diagnostic provenance for `C` and may support future
adjudication; they do not enter this canonical one-layer translation.

### Complementary sub-canopy exchange

`L_atm` arrives as the already evaluated incident atmospheric flux with
variable effective `epsilon_all`. With effective canopy and snow emissivities
fixed to one:

```text
L_can = sigma T_c^4
L_sub = f_sky L_atm + (1 - f_sky) L_can
L_out = sigma T_s^4
L_net = L_sub - L_out
```

Positive `L_net` supplies energy to snow. Negative `L_net` removes energy.
Sky and canopy weights are complementary and must sum to exactly one within
the numeric tolerance.

The admitted first implementation may use above-canopy `T_a` for `T_c` only
as an explicitly named homogeneous-stand approximation. It must not be
described as a prognostic canopy energy balance. Stable nocturnal inversions,
forest edges, large gaps, and strongly sunlit or intercepted-snow canopies are
known limitations.

### Shared Stage 3 active-layer thermal and sublimation composition

The sole version-3 thermal provider is the Marks/SNOBAL active control volume
constructed from the layers carried by
`snow_stage3_liquid_routing_model=layered_thermal_liquid_v1`:

```text
z_0 = min(sum_i(z_i), 0.25 m)
m_0 = integral over [0, z_0] of layer mass
Q_cc,0 = integral over [0, z_0] of layer cold content
T_s = T_0 = 273.15 K - Q_cc,0 / (m_0 c_i)
T_c = T_a
```

The normal `0.25 m` maximum is fixed Marks/SNOBAL structural authority, not a
user coefficient. The sole exception is the exact version-4 lower-volume
collapse, which makes the complete resolved pack active for that substep.
Depositional boundaries do not define the radiating/turbulent
heat capacity. A depositional layer intersected by `z_0` is partitioned
conservatively; mass, depth, liquid, refrozen mass, and cold content must close
before flux evaluation. All material within the active thermal volume shares
the resulting `T_0` after projection. When the pack is no deeper than
`0.25 m`, the complete pack is the active control volume.

When a lower volume exists, derive its aggregate temperature and effective
conductivity from its mass, cold content, and series thermal resistance.
Thermal partitions persist across substeps; a boundary-intersected
depositional layer must not be recombined in a way that erases the current
active/lower temperature gradient.

Use the exact libsnobal `KTS` plus `efcon` conductivity formulation, not the
Sturm density-only frost-insulation relation:

```text
p_a = 101300 Pa * (1 - 0.0065 z_elev / 293 m)^5.26
rho_r = rho_s / (1000 kg m^-3)
k_d = 4.186798188 * 0.0077 * rho_r^2
D_e = 0.65 * (101324.6 Pa / p_a) * (T_s / 273.16 K)^14 * 1e-4 m^2 s^-1
w_s = (18.0153 / 28.9644) * e_si(T_s) / (p_a - e_si(T_s))
k_eff = k_d + L_s(T_s) D_e w_s
```

Here `e_si` is the admitted SNOBAL ice-saturation relation already used by
the vapor provider. Elevation is existing climate metadata; `p_a` is derived
internally and is not a user coefficient. Each volume's series resistance
uses its current shared temperature. The SNOBAL harmonic interface exchange
is:

```text
G_0 = 2 k_0 k_l (T_l - T_0) / (k_l z_0 + k_0 z_l)
```

where positive `G_0` supplies heat to the active layer. The same transfer is
`-G_0` in the lower-layer balance; it cancels exactly in the whole-pack
ledger. A transfer first satisfies the receiving control volume's available
cold content. Under the version-7 target, any positive whole-pack residual then
enters the single bounded phase-change ledger; it is neither discarded nor
also supplied to a CoE generator. The current runtime's reported-but-
unconverted excess is an explicit implementation hold.

Use the minimum active/lower control-volume mass to select the stability
timestep:

```text
m_min >= 60 kg m^-2  -> 60 minute substeps
10 <= m_min < 60     -> 15 minute substeps
m_min < 10           -> 1 minute substeps
```

If no lower volume exists, `m_min=m_0`. The `1 kg m^-2` small-timestep
threshold is also the exact libsnobal terminal resolved-pack boundary. Before
partitioning or constructing `T_0`, conductivity, or the next carrier,
calculate total represented ice mass `m_s`. If
`m_s <= m_res = 1 kg m^-2`, the Stage 3 thermal and exchange domain is
unresolved for the remainder of the hour:

```text
Q_shortwave = Q_longwave = Q_latent = 0
m_v = m_sub = 0
G_0 = Q_E = 0
t_unres += remaining hour duration
```

When `m_s > m_res`, select the normal active/lower partition. If a lower volume
exists and `0 < m_l < m_res`, do not evaluate that sub-resolution volume.
Collapse the thermal partition to one whole-pack active volume for the current
substep and continue the ordinary surface-energy solve. Existing conservative
projection may coalesce thermally identical fragments, but total mass, liquid,
refrozen mass, and cold content must remain closed:

```text
active thermal volume = complete represented pack
lower thermal volume = none
t_collapse += substep duration
```

The lower-volume comparison is strict: `m_l = 1 kg m^-2` remains a resolved
two-volume solve. This reproduces libsnobal `_calc_layers.c` ordering and
branch sides; it is distinct from the `m_s <= 1 kg m^-2` no-layer branch.

The current implementation adopts libsnobal's resolved-layer boundary but not
its residual-snow-to-water conversion. During the implementation hold, CoE
remains the compatibility runtime owner, so persistent layer mass, liquid,
refrozen mass, and cold content remain unchanged by suspended Stage 3
exchange. The version-7 target does not authorize carrying that behavior into
cutover: exact residual-snow disposition at `m_s <= 1 kg m^-2` must be derived,
implemented, and independently closed before activation. Until then the
unresolved branch must not fabricate temperature, conductivity, vapor
exchange, or phase change, and no partial Stage 3 melt path may activate.

All four current `B/L/S/LS` cells use this provider and the same compatibility
CoE melt, density, phase, liquid-routing, forcing, and albedo selections. That
fact records current implementation, not target authority. Longwave and
sublimation are separate default-off selectors. Enabling either without the
Stage 3 provider is a typed missing-provider error. Enabling the new
sublimation selector together with legacy
`coe_open_sublimation_stage_a_v1` or
`coe_open_sublimation_stage_b_v1` is a typed incompatible-selection error,
preventing double mass loss.

For the retained Marks/SNOBAL-lineage neutral exchange, calculate one
loss-positive substep sublimation amount `m_sub >= 0` from the same `T_s` used
by longwave, then define:

```text
m_v = -m_sub
q_v = m_v / delta_t
Q_latent = q_v L_s(T_s)
```

where `delta_t` is the selected substep duration, `q_v` is in
`kg m^-2 s^-1`, and `L_s(T_s)` is the temperature-appropriate latent heat of
sublimation in `J kg^-1`. Thus
`Q_latent <= 0` during sublimation. The implementation must derive both mass
and energy from `q_v`; it may not independently recompute either view.

The current compatibility substep carrier is:

```text
Q_surface = Q_shortwave
            + I_L L_net
            + I_S Q_latent
Q_E,0,potential = (Q_surface + G_0) delta_t
Q_E,l,potential = -G_0 delta_t
```

where `I_L` and `I_S` are the independent longwave and sublimation selector
indicators. Before the atomic version-7 cutover, Stage 3 may apply only the
portion that changes cold content; unused positive potential is reported and
is not converted to melt. Sublimated ice is removed from the
active surface downward after the coupled update. Cold content associated
with that removed ice is a separately reported energy export. Active and
lower ledgers retain `G_0` with opposite signs; it cancels in whole-pack
closure:

```text
Q_surface,applied + Q_refreeze + Q_cc,export
    = Q_cc,before - Q_cc,after
```

and post-CoE vapor mass closure is:

```text
M_ice,before - M_ice,after = M_sublimation
```

Sublimation must not enter routed melt, retained liquid, released liquid, or
refreeze operands.

### Raw opportunity versus bounded vapor transfer

The current evaluation-only schema-v6 carrier exposes
`vapor_mass_exchange_kg_m2 = m_v,raw`. For sequential `Q`, independently derive

```text
m_deposition = max(m_v,raw, 0)
m_sublimation = min(max(-m_v,raw, 0), m_active_ice_before)
m_v = m_deposition - m_sublimation
```

and compare both bounded components to producer fields before aggregation.
Same-state `S` and frozen-active `F` prohibit mutation, so actual bounded
transfer is N/A even when raw opportunity is nonzero. Numeric zero cannot
replace N/A.

The evaluation carrier's `latent_flux_w_m2` is raw turbulent latent-energy
opportunity paired with `m_v,raw`. When the availability bound is inactive,
raw and bounded mass/latent views coincide. When sublimation opportunity
exceeds active ice, raw latent opportunity is not latent energy of actual
bounded transfer. Report that valid capacity truncation as
`VAPOR_OPPORTUNITY_TRANSFER_MISMATCH`; do not hide it in melt, liquid,
deposition, a median, or an endpoint residual. `INV-SNOWENERGY-017/029`
continue to require one bounded `m_v` for the future production target's
exact-one mass and latent-energy ledgers. Version 9 authorizes
characterization only and makes no production correction.

The independent consumer reconstructs the current evaluation chronology per Q
tuple, distinct from the future bounded-latent production target:

```text
Q_latent_raw = latent_flux_w_m2 * duration_seconds = m_v,raw * L_s(T_s)
Q_latent_bounded = m_v * L_s(T_s)
Q_latent_truncation = Q_latent_raw - Q_latent_bounded
C1 = C0 - G
E_raw = independently reconstructed external_flux_w_m2 * duration_seconds
surface_change = if E_raw >= 0 then min(E_raw, C1) else E_raw
active_cold_change = G + surface_change
lower_cold_change = -G
Q_complete_raw = E_raw + G
Q_excess_raw = max(Q_complete_raw - active_cold_change, 0)
m_ice_available = max(m_active_ice_before - m_sublimation, 0)
m_melt_raw_carrier = min(Q_excess_raw / L_f, m_ice_available)
Q_unallocated_raw = max(Q_excess_raw - L_f * m_melt_raw_carrier, 0)
```

Here `C0` is active cold content before the substep and `G` is the emitted
active-side internal-conduction primitive; schema v6 does not independently
recompute layerwise conductivity. The consumer compares each reconstructed
cold, melt, and closure operand to producer fields, then checks mass endpoints.
It preserves that the as-built melt chronology consumes `E_raw`; it must not
relabel `Q_latent_raw` or `m_melt_raw_carrier` as conformance to the future
bounded-latent production target.

### Version 56 frozen temperature-primary exact-carry specialization

Before V55 eligibility, validate an authentic Stage 3 terminal-one-volume
history as strictly frozen and noncrossing: every affected lane remains below
exact `273.15 K`, has no liquid/melt/refreeze/terminal-unallocated event, and
retains identical phase predicate, density branch, layer order, support,
event, topology, custody, and static receipt authority. If this specialization
is inapplicable before a charge, continue the unchanged V55/V45 chronology.
Once a V56 physical attempt charges, every error is typed and fail-closed.

The canonical private coordinates are ordered lane `(W_1,T_s,1,rho_1)` then
ordered top-soil `(E_soil,1,T_soil,1)`. For every charged map:

1. compute `H_exact=-exact(W_1)*exact(c_ice)*(exact(273.15)-exact(T_s,1))`;
2. round `H_exact` once to finite binary64 `H_hi` by IEEE-754
   round-nearest-even and retain exact dyadic
   `R_Hcarry=H_exact-exact(H_hi)`, so reconstruction is exact;
3. derive `Q_cn` once from the unchanged CN operation and the proposed
   `T_s,1/T_soil,1`, applying snow `+Q` and soil `-Q` exactly once;
4. run the unchanged finalization-equivalent Stage 3/soil map and evaluate all
   existing water, density, soil, thickness, side, branch, custody, and ledger
   guards, evaluating energy tolerance from the exact high-plus-carry total;
5. seal only an authentic fresh result as an
   `AuthenticatedCoveredSnowMaterialOwnerV1`, pairing the unchanged Stage 3
   material owner with ordered high/carry state and a
   `CoveredSnowEnthalpyCarryReceiptV1` that binds schema/version, support,
   transaction/predecessor, lane/order, beginning/ending high-plus-carry,
   exact ordered energy operands, base-owner digest, receipt chain, branch,
   topology, and custody;
6. stabilize the complete receipt set, including CN and additive carry
   receipts, to exact input/output equality; then require one independent
   same-input replay and unchanged strict finalization equality before any
   admission or publication.

Every physical/Jacobian/trust/rejected/probe/replay map uses the same already
charged `CoveredPhysicalEvaluationBudgetV1` with maximum 96. Exact
reconstruction and CN derivation add no physical call. The adaptive minimum
support remains exactly 60 seconds. No V54/V55 transient cycle member, Q
lattice candidate, or solver artifact may be promoted into a compound owner
or carry receipt.

`DirectSnowStage3V11SnowEnthalpyRestartV5` persists the authenticated compound
owner and carry-receipt chronology for committed, pending, and in-progress
state, including the current in-progress support. Migration from V4 is allowed
only by assigning canonical exact zero carry. Downgrade from V5 to V4 is
allowed only when every carried snow remainder is exact zero; any nonzero
carry refuses without mutation: nonzero carry blocks downgrade. Restore must
revalidate and reproduce the
complete owner/receipt chronology before execution resumes.

## Branch and Guard Table

| Branch/condition | Required behavior | Guard class | Failure class |
|---|---|---|---|
| V56 authentic Stage 3 history is strictly frozen/noncrossing with exact static joins and budget capacity | Dispatch the temperature-primary specialization before V55; derive exact H high-plus-carry and unchanged CN Q, then require exact compound-receipt stabilization, replay, and finalization. | runtime/numerical/custody | typed refusal after first charge; zero-charge not-applicable may continue V55 |
| V56 history crosses phase, changes branch/order/event/topology/custody, contains liquid/melt/refreeze/terminal energy, or has invalid exact-carry/restart lineage | Refuse V56 without normalization, carry deletion, or transient-witness promotion. | runtime/custody/restart | typed eligibility, owner, receipt, or restart failure with exact rollback |
| Any required scalar is non-finite | Reject before arithmetic. | runtime | typed invalid forcing/state |
| Any turbulent height or `z_0,aero` is non-finite/non-positive, or a measurement height does not exceed the displacement/roughness boundary | Reject before logarithms or stability iteration; do not substitute another forcing profile. | runtime | typed invalid turbulent geometry |
| `T_a <= 0 K`, `T_c <= 0 K`, or `T_s <= 0 K` | Reject. | runtime | typed invalid temperature |
| `e_a < 0 kPa`, `R_s < 0`, or `R_a < 0` | Reject. | runtime | typed invalid forcing |
| Derived `L_clear`, `epsilon_clear`, `epsilon_all`, or `L_atm` is non-finite, or either emissivity is outside `[0,1]` | Reject without clamping. | runtime | typed out-of-authority atmospheric state |
| Daylight and `R_a > R_a,min` | Calculate `k_t`, clamp only the empirical cloud mapping, and continue. | runtime | none |
| Polar night or `R_a <= 1e-9 MJ m^-2 d^-1` with longwave enabled | Do not divide, reuse SIMIMPL28 cloud fraction, or carry a prior value; return typed unavailable state. | runtime | typed cloud-forcing unavailable |
| `C < 0` or `C >= 1` | Reject; do not silently clamp. | runtime | typed invalid canopy state |
| `C = 0` | Require `f_sky = 1` and `L_sub = L_atm`. | test | blocked promotion on mismatch |
| `C -> 1` within valid domain | Require `f_sky -> 0` and `L_sub -> L_can`. | test | blocked promotion on mismatch |
| Stage 3 thermal provider absent while longwave or sublimation is enabled | Do not publish or mutate candidate energy/mass state. | runtime | typed missing thermal provider |
| New sublimation selector plus legacy Stage A/B melt variant | Reject before hourly processing. | runtime | typed incompatible selector |
| Sublimation demand exceeds available ice | Bound the transfer to available ice and derive latent energy from the bounded transfer. | runtime | none; bounded physical availability |
| Melt and sublimation both demand the same substep ice | Reserve bounded `m_sublimation` first, bound melt by the remaining `m_ice_available`, and apply the already evaluated vapor mutation after melt/refreeze. Deposition cannot enlarge melt availability. | runtime/test | typed joint-availability or mass-closure failure |
| Total snow depth is `<= 0.25 m` | Use the complete pack as the active thermal control volume. | runtime | none |
| Total represented ice mass is `<= 1 kg m^-2` | Before thermal partition, preserve persistent CoE/layer state and suspend all Stage 3 thermal, radiation, conduction, and vapor exchange without constructing temperature or conductivity. | runtime/model domain | explicit unresolved-duration and total-mass diagnostics; no typed thermal failure |
| Total mass is `> 1 kg m^-2` and `0 < m_l < 1 kg m^-2` | Conservatively project to one whole-pack thermal volume and continue normal exchange; thermally identical fragments may coalesce only with closed aggregate state. | runtime/model domain | explicit collapse-duration and lower-mass diagnostics |
| Lower thermal mass is exactly `1 kg m^-2` | Retain the resolved active/lower two-volume solve. | runtime/model domain | no collapse diagnostic; ordinary coupled-state guards |
| A depositional layer crosses `z_0` | Partition/project conservatively; reject nonclosing state. | runtime | typed thermal-partition closure failure |
| Active/lower mass selects a smaller timestep | Execute every required substep and reevaluate the coupled state; do not retain an hourly energy debit. | runtime | typed cadence/closure failure |
| Coupled update would require `T <= 0 K` | Reject; no clamp, temperature replacement, or cold-content tax is allowed. | runtime | typed invalid thermal state / blocked campaign |
| `T_c = T_a` approximation active | Emit/retain explicit approximation identity in configuration or diagnostics. | profile | blocked promotion if unlabeled |
| Canopy is outside equivalent homogeneous/random-orientation/isotropic-diffuse regime | Do not expand the claim; retain a diagnostic/model-limitation classification. | governance | model limitation |
| Any target energy component is unavailable, duplicated, non-finite, or lacks exact-one lineage | Do not activate the Stage 3 melt owner. | governance before cutover; runtime after cutover | hard `IMPLEMENTATION_HOLD`; typed energy-input failure after cutover |
| `Q_excess > 0` with available ice | Apply `m_melt=min(Q_excess/L_f,m_ice_available)` once after cold-content satisfaction. | runtime/test | typed energy/phase closure failure |
| CoE and Stage 3 melt generation both selected or reachable | Reject the configuration/cutover; never blend outputs. | governance/runtime | hard `IMPLEMENTATION_HOLD`; typed incompatible-owner failure after cutover |
| Any energy, solid, or liquid ledger does not reconstruct from exact operands | Reject without residual acceptance, alias substitution, or clamp. | runtime/test | hard `IMPLEMENTATION_HOLD`; typed closure failure after cutover |
| Available-ice bound saturates and `Q_unallocated_after_exhaustion > 0` | Do not discard, carry, or route the energy by proxy; target cutover remains blocked until canonical physical recipient and next-state chronology are amended. | governance/model boundary | hard `IMPLEMENTATION_HOLD` |
| `m_s <= 1 kg m^-2` or terminal meltout would enter unresolved residual snow | Preserve current compatibility behavior only; do not activate a partial target phase path. | governance/model boundary | hard `IMPLEMENTATION_HOLD` |
| shared carrier lacks sealed exposure, has duplicate flux lineage, or has invalid participant/support receipt | reject before trial state or ledger mutation | runtime | `SNOWENERGY-E-WIND-001` / `SNOWENERGY-E-CARRIER-001` |
| snow flux is requested after the accepted event or in a snow-free regime | reject before evaluation | runtime | `SNOWENERGY-E-REGIME-001` |
| canopy-intercepted snow is supplied to this surface carrier | reject as out of scope | runtime | `SNOWENERGY-E-SCOPE-001` |
| component-weighted canopy longwave or reciprocal ledger does not close | reject without candidate publication | runtime | `SNOWENERGY-E-LW-001` |
| An authentic finalization rebuild misses an unchanged convergence predicate and support-scaled relaxation is admitted | Advance only the unpublished Stage 3 iterate with the existing guarded candidate weight; retain authentic final LSE and complete boundaries plus converged authentic soil. | runtime/numerical | typed nonconvergence on cap exhaustion; relaxed iterate never publishes |
| A relaxed finalization restart is followed by the first otherwise-converged provisional Picard image | Consume exactly one additional guarded support-scaled Picard update before finalization is retried. | runtime/numerical | typed nonconvergence on cap exhaustion; no tolerance bypass |
| Finalization relaxation encounters a schema, terminal-model, lane, interval, layer, settling, initial/input, represented-mass, aggregate-posture, reconstruction, or closure mismatch | Decline contraction and retain the raw authentic final candidate as the next unpublished iterate. Candidate density is always copied bitwise and never interpolated; its difference remains nonconverged until an authentic image satisfies the exact comparison. | runtime/numerical | typed nonconvergence/refinement; no state repair |
| At exactly 60 seconds, a finite raw authentic terminal-one-volume open-snow image is below the existing `200 K` boundary domain while the current unpublished image lies across the same canonical phase kink | Retain the raw authentic history; form exactly one `w=0.5` coordinated complete-support operand midpoint from the common immutable beginning, reconstruct `W` and `H`, and apply the canonical phase projection to obtain only the next unpublished iterate. | runtime/numerical | typed refinement/nonconvergence for any identity, support, vapor-sign/disposition, component closure, phase-capacity, finiteness, structure, or independent ledger failure; midpoint never finalizes, accepts, replays, or publishes |
| Two independently closed terminal-one-volume support images on an exact support `>=60_000_000_000 ns` are pure one-sided, have strictly opposite finite nonzero actual vapor, share every immutable identity, and ordinary guarded contraction is blocked only by the vapor active-set crossing | Retain raw authentic history; localize the unique strict-convex signed-vapor root, set `V=D=S=Q_v=+0`, interpolate only external liquid and ordered nonlatent energy components, recompute complete energy, and apply the canonical `W/H` phase projection to obtain one unpublished interface image. | runtime/numerical | typed refinement/nonconvergence for mixed `D/S`, same-sign or zero endpoint dispatch, nonfinite/root/component/latent/identity/event/topology/custody/receipt/capacity failure, or any mutation; interface never converges, finalizes, replays, accepts, persists, or publishes |
| An exact version-32 zero-vapor interface is followed by a fresh authentic pure one-sided endpoint on the same support and identities | Enter that one-sided branch using the existing bounded support-scaled Picard weight; derive `V`, exclusive `D` or `S`, and `Q_v=V*L_s,auth` with the authentic positive finite specific latent heat; contract only external liquid and ordered nonlatent energy, recompute `Q`, and project `W/H`. | runtime/numerical | typed refinement/nonconvergence for zero/zero, mixed disposition, bad latent sign/linkage, nonfinite operand, identity/event/topology/custody/receipt/capacity mismatch, cap exhaustion, rollback or publication mutation |
| One event-free covered support repeats the exact v32 active-set transition sequence `root/interface -> one-sided branch-entry -> opposite pure-vapor raw-authentic -> same root/interface/reset coordinates and branch predicates with opposite pure vapor disposition`, while support/source/event/topology/custody/receipt joins remain identical | Preserve the transition record and invoke `phase_consistent_coupled_solve_v1` on only lane `W_1/H_1` and the coupled soil endpoint enthalpy/temperature coordinates. Do not require the two asymptotically changing raw-authentic continuous owner images to compare bitwise equal. Reevaluate concrete `R_W/R_H/R_E/R_T` through `covered_phase_consistent_residual_evaluate_v1` from immutable beginnings and sealed sources. Version-31/32 synthetic states remain diagnostic evidence, not residuals or publication candidates. | runtime/numerical | typed nonconvergence for any transition-order/root-coordinate/branch-predicate/pure-vapor/join mismatch or unless a finite domain-valid coupled physical root is found within the one shared unchanged 96-evaluation budget |
| Eight consecutive raw authentic maps on one event-free terminal-one-volume covered support have identical support/source/event/topology/custody/static-receipt/phase-branch/carry-authority-and-representation joins, exact `E=exact(H_hi)+R` reconstruction, finite `R_W/R_H/R_E/R_T`, and strictly decreasing governed residual merit, with no `A/B/A`, active-set transition, or finalization restart; physical receipt digests and `H_hi/R` coordinate values may evolve under unchanged closure | Charge all eight maps to the existing shared 96-evaluation budget and invoke the unchanged `phase_consistent_coupled_solve_v1` through `covered_stable_monotone_solve_eligibility_v1`; no private trial is a candidate for acceptance. | runtime/numerical | any eligibility or pre-root solver refusal discards every private trial and resumes raw authentic Picard within the remaining shared budget; exhaustion or any changed static join/authority/representation/branch, failed exact reconstruction, nonfinite/nondecreasing merit, active-set change, `A/B/A`, restart, or private-trial admission fails closed |
| A coupled physical root is available under immutable receipt input `R_n` | Treat the fresh physical evaluation and reconstructed `R_(n+1)` as a charged `covered_authentic_receipt_stabilization_probe_v1`; discard its private artifacts after extracting the immutable next receipt input. Never compare root/probe artifacts for exact acceptance across different receipt inputs. | runtime/numerical | typed refusal on nonfinite state, side-constraint failure, malformed receipt, in-place mutation, digest/tolerance repair, artifact retention, or budget exhaustion |
| A receipt-stabilization probe reconstructs output receipts exactly equal to its input receipts | Perform one independent `covered_authentic_receipt_stabilization_replay_v1` with the same stabilized receipt set and require exact residual, candidate-artifact, and reconstructed-receipt equality before `CoupledAuthentic` or finalization. | runtime/numerical/publication | typed refusal with all private/probe artifacts discarded on receipt oscillation, replay disagreement, nonfinite state, side-constraint failure, or budget exhaustion |
| Stable terminal-one-volume contraction retains exact layer/order, settling authority, and density-model branch but physical density/thickness evolves | Add one `rho_1,l` coordinate, reconstruct `z_1,l=I_1,l/rho_1,l`, and evaluate physical `R_rho,l` through `covered_terminal_density_geometry_residual_evaluate_v1` from immutable beginnings, sealed forcing, and unchanged Stage 3 constitutive operands. | runtime/numerical | typed refusal for branch/authority mutation, nonpositive/nonfinite density or thickness, mass-depth mismatch, generic map-difference residual, interpolation/repair, uncharged evaluation, budget exhaustion, or any v35/finalization bypass |
| A v33 coupled trial crosses a sealed event boundary | Partition at the existing event boundary before solving, or refuse the unpartitioned support. The phase kink is internal complementarity and creates no event. | runtime/event | typed event/support failure with exact rollback |
| The v33 reduced solve reaches a root under every unchanged residual, algebraic side-constraint, closure, capacity, identity, topology, custody, receipt, and soil guard | Charge fresh evaluation and final replay/reseal to the same `CoveredPhysicalEvaluationBudgetV1`; reconstruct from immutable beginnings and sealed sources; then enter `CoveredConvergenceAdmissionV1::CoupledAuthentic`. This admission bypasses only ordinary Picard iterate equality/convergence and retains every finalization, reseal, rollback, and publication check. | runtime/numerical/publication | typed budget/replay/reseal/finalization failure; no affine or synthetic state may finalize or publish |

`R_a,min` is the numerically explicit `1e-9 MJ m^-2 d^-1`
divide/branch threshold.

## Invariants and Guard Map

### Invariants

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-SNOWENERGY-001` | Temperatures entering fourth-power emission are finite kelvin values greater than zero. | `REF-SNOWENERGY-PHYSICAL` | `[INFERENCE][Static]` | pre-arithmetic temperature guard | typed invalid temperature |
| `INV-SNOWENERGY-002` | Vapor pressure is finite, non-negative, and expressed in `kPa`. | `REF-SNOWENERGY-FLERCHINGER` | `[DIRECT][Static]` | forcing/unit boundary | typed invalid forcing |
| `INV-SNOWENERGY-003` | `0 <= c <= 1`; clamping occurs only after a valid finite daylight `k_t` exists. | `REF-SNOWENERGY-FLERCHINGER` | `[DIRECT][Static]` | cloud branch guard | typed invalid or unavailable cloud forcing |
| `INV-SNOWENERGY-004` | `0 <= C < 1`, `P_0=1-C`, and `f_sky=(1-C)^1.6`. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PLANT` | `[DIRECT][Static] + [INFERENCE][Static]` | canopy-domain guard and vectors | typed invalid canopy state |
| `INV-SNOWENERGY-005` | `0 < f_sky <= 1` and sky view decreases monotonically with `C` on the valid domain. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | analytical property test | blocked contract/runtime promotion |
| `INV-SNOWENERGY-006` | Sky and canopy weights are complementary: `f_sky+(1-f_sky)=1`. | `REF-SNOWENERGY-ESSERY2008`, `REF-SNOWENERGY-RUTTER2023` | `[DIRECT][Static]` | independent reconstruction | blocked contract/runtime promotion |
| `INV-SNOWENERGY-007` | At `C=0`, `L_sub=L_atm`; as `C->1`, `L_sub->sigma T_c^4`. | `REF-SNOWENERGY-FSM2`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | limiting-case vectors | blocked contract/runtime promotion |
| `INV-SNOWENERGY-008` | `L_net=L_sub-sigma T_s^4`, positive toward snow. | `REF-SNOWENERGY-ESSERY2008`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static]` | sign and reconstruction tests | typed energy-evaluation failure |
| `INV-SNOWENERGY-009` | LAI, structural cover, and height are not independently added to effective cover in this translation. | `REF-SNOWENERGY-PLANT`, `REF-SNOWENERGY-FSM2` | `[DIRECT][Static] + [INFERENCE][Static]` | contract/source review | hard `HOLD` on alternate composition |
| `INV-SNOWENERGY-010` | No production snow-energy flux is published before EB-03 supplies coherent `T_c`, `T_s`, and cold-content coupling. | `REF-SNOWENERGY-EB01A` | `[DIRECT][Static]` | runtime-consumer closure gate | hard `HOLD` |
| `INV-SNOWENERGY-011` | Polar-night cloud inference never divides by zero or silently reuses unrelated legacy cloud state. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-EB01A` | `[DIRECT][Static] + [INFERENCE][Static]` | daylight/cloud branch guard | typed cloud-forcing unavailable |
| `INV-SNOWENERGY-012` | Package-local analytical code is evidence only and is never imported by production crates. | ADR-0011 and package scope | `[DIRECT][Static]` | source/write-set inventory | blocked package closure |
| `INV-SNOWENERGY-013` | Atmospheric longwave uses hourly `T_a`; daily `e_a` and clearness-derived `c` may be held across the day. Daily-mean-temperature substitution is prohibited. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-EB01A` | `[DIRECT][Static] + [INFERENCE][Static]` | cadence/aggregation test | typed cadence mismatch / hard `HOLD` |
| `INV-SNOWENERGY-014` | Derived atmospheric fluxes are finite and both effective emissivities lie in `[0,1]`; out-of-range results fail without clamping. | `REF-SNOWENERGY-FLERCHINGER`, `REF-SNOWENERGY-PHYSICAL` | `[DIRECT][Static] + [INFERENCE][Static]` | derived-domain guard | typed out-of-authority state |
| `INV-SNOWENERGY-015` | All `B/L/S/LS` cells use the same Stage 3 top-layer `T_s`/cold-content provider; `T_c=T_a` is explicitly identified. | `REF-SNOWENERGY-EB01A`, `REF-SNOWENERGY-RUTTER2023`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-085` | `[DIRECT][Static] + [INFERENCE][Static]` | selector/provider guard and real-consumer test | typed missing provider / blocked campaign |
| `INV-SNOWENERGY-016` | In the current compatibility `B/L/S/LS` cells, longwave and sublimation are orthogonal default-off selectors and neither changes the CoE melt-model selector. This current-runtime selector invariant does not authorize CoE after the atomic `INV-SNOWENERGY-029` cutover. | snow-surface EB roadmap, `REF-SNOWENERGY-21N` | `[DIRECT][Static]` | selector matrix test | blocked EB-04 admission or partial cutover |
| `INV-SNOWENERGY-017` | Signed vapor mass and latent heat are derived from one bounded exchange at the shared `T_s`; sublimation is negative latent energy and cannot be debited twice. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-085`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | independent latent/mass reconstruction | typed closure failure |
| `INV-SNOWENERGY-018` | Sublimation reduces ice storage only and never aliases routed melt, retained/released liquid, or refreeze. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-073`, `#INV-SNOWFREEZE-076` | `[DIRECT][Static]` | independent mass closure and alias-separation test | typed closure failure |
| `INV-SNOWENERGY-019` | Cold-content change closes from applied surface energy, interlayer conduction, refreeze energy, and exported cold content on the declared control volume. | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-080`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | independent energy reconstruction | typed closure failure |
| `INV-SNOWENERGY-020` | The radiating/turbulent control volume is normally the upper `min(z_s,0.25 m)` of snow and is independent of snowfall-event boundaries; `INV-SNOWENERGY-026` exclusively authorizes whole-pack depth for the strict sub-resolution-lower-volume collapse. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static]` | active-layer partition and anti-alias test | typed partition failure |
| `INV-SNOWENERGY-021` | Active/lower mass, depth, cold content, and thermal resistance reconstruct the persistent column exactly before and after projection. | physical conservation, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static] + [INFERENCE][Static]` | independent partition reconstruction | typed closure failure |
| `INV-SNOWENERGY-022` | `G_0` is positive into the active layer, appears as `-G_0` in the lower balance, and cancels from the whole-pack ledger. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL` | `[DIRECT][Static]` | sign, limiting, and reconstruction tests | typed closure failure |
| `INV-SNOWENERGY-023` | Mass-dependent `60/15/1 minute` substeps are selected from the `60/10/1 kg m^-2` Marks/SNOBAL thresholds; substep fluxes are reevaluated from current state. | `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-LIBSNOBAL`, `REF-SNOWENERGY-LUTE2022` | `[DIRECT][Static]` | cadence and thin-pack tests | typed cadence failure / blocked campaign |
| `INV-SNOWENERGY-024` | No active/lower update may use an absolute-zero clamp, air-temperature replacement, fitted cold-content tax, or user limiter. | `REF-SNOWENERGY-PHYSICAL`, EB-03A authority envelope | `[INFERENCE][Static]` | source scan and physical-domain tests | hard `HOLD` |
| `INV-SNOWENERGY-025` | Active and lower partitions retain distinct shared temperatures across substeps, and `G_0` uses libsnobal `KTS+efcon` effective conductivity with elevation-derived pressure; the Sturm frost-insulation relation is not an admissible substitute. | `REF-SNOWENERGY-LIBSNOBAL`, `REF-SNOWENERGY-MARKS1999`, `REF-SNOWENERGY-ANDERSON1976` | `[DIRECT][Static]` | unequal-temperature persistence and conductivity vectors | typed conductivity/projection failure |
| `INV-SNOWENERGY-026` | At total represented ice mass `m_s <= 1 kg m^-2`, the current Stage 3 thermal/exchange domain is unresolved before partition: compatibility CoE and persistent-layer mass, liquid, refrozen mass, cold content, and topology are preserved; no temperature, conductivity, surface energy, conduction, vapor exchange, sublimation, or melt alias is produced. For `m_s > 1`, `0 < m_l < 1 kg m^-2` collapses to one whole-pack thermal volume and continues exchange, while `m_l = 1` remains two-volume. Version 7 retains this as current-runtime behavior but blocks target cutover until residual-snow phase authority is complete. | `REF-SNOWENERGY-LIBSNOBAL`, `REF-SNOWENERGY-21N`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | exact total-mass threshold sides, strict lower-layer collapse/equality, state-preservation, resume, and real-consumer trace tests | typed closure failure above boundary / blocked campaign on alias or mutation |
| `INV-SNOWENERGY-027` | Persistent density layers are retained or removed by the density model's mass-unit lifecycle boundary: `m_layer = rho_w * SWE_layer > 1e-9 kg m^-2` is represented and `m_layer <= 1e-9 kg m^-2` is zero mass. The independent `1e-9 m` SWE and physical-depth aggregate tolerances test residual closure only; neither may delete a represented layer. Retained mass, depth, liquid, refrozen mass, cold content, density, and settle state remain coupled. | `INV-SNOWENERGY-021`, physical conservation, dimensional consistency | `[DIRECT][Static] + [INFERENCE][Static]` | named SWE-to-mass predicate, exact-side tests, independent aggregate reconstruction, typed mismatch outside tolerance | typed aggregate mismatch / blocked campaign on cross-unit filtering or state deletion |
| `INV-SNOWENERGY-028` | A `1e-9 m` SWE closure residual is exactly `1e-6 kg m^-2` when the same residual is expressed as area mass through `rho_w=1000 kg m^-3`; vapor-to-sublimation transfer closure uses that `1e-6 kg m^-2` bound. This conversion does not alter the separately governed `1e-9 kg m^-2` hourly/daily vapor-aggregation reconstruction tolerance or the `1e-9 kg m^-2` represented-layer lifecycle boundary (`1e-12 m` SWE). | `INV-SNOWENERGY-017`, `INV-SNOWENERGY-018`, `INV-SNOWENERGY-027`, named unit conversion, dimensional consistency | `[DIRECT][Static] + [INFERENCE][Static]` | operand-specific independent reconstruction and named SWE-to-area-mass conversion | typed closure failure / blocked adjudication on cross-predicate substitution |
| `INV-SNOWENERGY-029` | The admitted future melt owner is Stage 3 alone. In each resolved stability substep, complete net radiation, sensible heat, latent heat, ground/interlayer conduction, and precipitation-advected heat satisfy cold content first. Split the already bounded signed vapor exchange exactly as `m_deposition=max(m_v,0)` and `m_sublimation=max(-m_v,0)`; reserve sublimation from post-precipitation ice; define `m_ice_available=max(m_ice_after_solid_precip-m_sublimation,0)`; then define `Q_excess=max(Q_complete-Q_cold_required,0)` after active/lower allocation and convert only that remainder as `m_melt=min(Q_excess/L_f,m_ice_available)`. `Q_unallocated_after_exhaustion=Q_excess-L_f*m_melt` must be zero; a positive value is an unresolved terminal boundary and blocks cutover. The CoE `A/B/C/D`, `C_canopy`, daily midpoint gate, embedded albedo, and rain-heat terms are compatibility diagnostics only and cannot generate melt after cutover. | `REF-SNOWENERGY-21N`, `REF-SNOWENERGY-LIBSNOBAL`, physical energy/phase conservation | `[DIRECT][Static] + [INFERENCE][Static]` | complete-component, cold-content-first, joint vapor/melt availability, latent-fusion, terminal-energy, and exact-one-owner gates | hard `IMPLEMENTATION_HOLD` until complete; typed energy/mass closure failure after cutover |
| `INV-SNOWENERGY-030` | Stage 3-generated liquid is debited from ice and credited to the single liquid handoff exactly once in the same substep, then passes through refreeze, retention, and routing before thermal repartition. The energy ledger includes latent heat released by refreeze; the solid ledger credits refrozen liquid back to ice; and the liquid ledger debits that same refrozen mass. All three reconstruct independently from exact operands. Simultaneous CoE/Stage 3 melt, discarded positive energy, delayed duplicate routing, or an unresolved `m_s <= 1 kg m^-2` phase proxy is prohibited. | `REF-SNOWENERGY-21N`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-091`, physical conservation | `[DIRECT][Static] + [INFERENCE][Static]` | same-substep chronology, linked-ledger reconstruction, thin-pack authority, and real-consumer cutover gates | hard `IMPLEMENTATION_HOLD`; typed closure failure after cutover |
| `INV-SNOWENERGY-031` | Stage 3 uses explicit virtual transfer heights `z_T=z_q=z_u=5 m` above the instantaneous modeled snow surface and exposed-snow aerodynamic roughness `z_0,aero=0.005 m`. These are model geometry, not physical measurement/reference heights, observations, calibration parameters, or aliases of active thermal-layer depth `z_0`. All four values cross a typed runtime boundary and satisfy the logarithmic displacement/roughness domain before evaluation. | `REF-SNOWENERGY-LIBSNOBAL`, user authority dated 2026-08-05, `INV-SNOWENERGY-033` | `[DIRECT][Static]` | exact-value projection, typed-domain, source-height/geometry non-alias, and sensitivity tests | typed invalid turbulent geometry / blocked cutover |
| `INV-SNOWENERGY-032` | Evaluation schema-v6 preserves `m_v,raw` and `Q_latent_raw=m_v,raw L_s(T_s)` as raw opportunities and actual sequential transfer separately as bounded deposition/sublimation with `Q_latent_bounded=m_v L_s(T_s)`. `S/F` actual transfer is N/A. For Q, the consumer reconstructs bounded transfer plus the exact characterization-only `C0/G/C1/E_raw/surface_change/active_change/lower_change/Q_complete_raw/Q_excess_raw/availability/melt/unallocated` chronology before producer and endpoint checks. Producer disagreement, simultaneous transfer, wrong direction, numeric-zero N/A, melt/liquid aliasing, or nonclosure is invalid evidence. Valid capacity truncation and `Q_latent_truncation` are `VAPOR_OPPORTUNITY_TRANSFER_MISMATCH` and block passage/persistence. The raw-latent chronology is not future bounded-latent target conformance. | `INV-SNOWENERGY-017/018/029`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-096`, physical mass/energy conservation | `[DIRECT][Static] + [INFERENCE][Static]` | independent tuple-level raw/bounded mass/latent reconstruction, operator-order chronology, anti-alias tests, and endpoint/energy closure | evidence hard-fail on malformed/alias/nonclosure; governance hold on physical passage |
| `INV-SNOWENERGY-033` | WIND-SOURCE-CUSTODY-AND-EXPOSURE: retained CLI `w-vl` is parsed as raw `vwind` and reaches Stage 3 as raw `vwind_m_s`; PMET alone creates `u_2,PMET` and that value cannot feed snow. GRIDMET `vs` product metadata describes daily nominal `10 m` wind, while Stage 3 `z_u=5 m` is virtual snow-surface-relative transfer geometry. Surviving WEPPpy runs directly prove byte-identical CLI lineage, retained watershed centroids and GRIDMET-enabled flags, complete daily parquet wind, and exact parquet-to-CLI equality. The nearest pre-build code statically reconstructs watershed-centroid GRIDMET `vs` requests, shared run-level wind, and one-decimal formatting; it is not deployed-code or request evidence. Exact deployed identity/request/response, product version/status, server-side pixel/sampling, day boundary, missing policy, source datum, and physical exposure remain `AUTHORITY_MISSING` unless directly retained. Modeled forest/`cancov=0.9` is target intent, not physical exposure or linkage. Neither values, residuals, a height conversion, nor a desired energy balance can establish forest/sub-canopy applicability, fit attenuation, license a canopy operator, or authorize production correction. | `REF-SNOWENERGY-GRIDMET`, `REF-SNOWENERGY-WIND-CUSTODY`, `INV-SNOWENERGY-031`, ADR-0042 | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | literal source/consumer alias tests, provider/fixture hash and parquet/CLI equality, static provider-code reconstruction, custody ledger, independent neutral-height diagnostic, and two-sided exposure-authority matrix | governance `HOLD` on remaining custody/applicability; no production correction |
| `INV-SNOWENERGY-034` | SNOW-TERMINAL-ENTHALPY-EVENT-NUMERICS: only `persistent_accumulation_shadow_v1` may enter an evaluation-only terminal snow domain when post-precipitation represented ice is `0 < m_i <= 1 kg m^-2`. Collapse the complete snow column to one enthalpy-bearing control volume without deleting mass: canonical cold-content deficit `Q_cc >= 0 J m^-2`, retained liquid `m_l >= 0 kg m^-2`, and material enthalpy `H=-Q_cc+L_f m_l` relative to 0 C ice. Use the existing complete bounded Stage 3 carrier and its current-state surface temperature; do not introduce a heat-capacity epsilon, clamp, fitted threshold, cold-content tax, or new flux equation. A deterministic first-order transition map is integrated with step doubling: compare one trial of `h` with two sequential trials of `h/2`, accept the two-half state only when the componentwise scaled ice/liquid/cold-content/energy norm satisfies `TOL-SNOWENERGY-001`, otherwise halve `h`; `h <= 60 s`, `h_min=1e-9 s`, at most 64 consecutive rejections, and any nonfinite/domain/nonconvergence result is typed failure with no state commit. Each trial reevaluates the carrier from its start state. Define `Delta H_cc=Q_cc,start-Q_cc,end`, positive for warming and negative for cooling. Apply energy to `Q_cc` and refreeze first, reserve bounded sublimation before melt availability, define `Q_excess=max(Q_complete+Q_refreeze-Delta H_cc,0)` and `m_melt=min(Q_excess/L_f,max(m_i-m_sublimation,0))`, then apply deposition after same-trial melt availability. Because entry is explicitly post-precipitation, the actual endpoint-solid function is `g(tau)=m_i,start+m_refrozen(tau)+m_deposition(tau)-m_sublimation(tau)-m_melt(tau)`; deposition/refreeze cannot retroactively enlarge same-trial melt availability. Require `g(0)=m_i,start`, bounded `g>=0`, and no event while deposited or refrozen solid remains. When an accepted trial first reaches the mass-root tolerance, replay from the immutable pretrial state and localize the earliest event by safeguarded bisection; preserve the positive/terminal bracket, require monotonically nonincreasing candidate solid, and stop only when both bracket width and endpoint solid satisfy `TOL-SNOWENERGY-001`, otherwise fail after 64 iterations. At the accepted upper endpoint, the complete solid identity—not a debit clamp—must establish zero ice; terminal liquid equals retained/external liquid plus melt less refreeze. The snow energy identity closes through `Delta H_cc`, fusion, refreeze, and `Q_terminal_unallocated=Q_complete+Q_refreeze-Delta H_cc-L_f m_melt >= 0`, which is explicitly censored and may be positive when sublimation exhausts solid first. No snow-domain state receives energy and no snow flux is evaluated after the event. Publish `evaluated_seconds=t_event`, `unevaluated_seconds=requested-t_event`, and censored terminal liquid/energy handoffs; neither is a land-surface recipient. This mechanics-only exception supersedes the `INV-SNOWENERGY-026` no-evaluation branch only for the named operator and terminal domain; compatibility/default paths and historical schemas remain exact. | `REF-SNOWENERGY-LIBSNOBAL`, `INV-SNOWENERGY-017/023/026/029/030`, physical conservation, deterministic numerical analysis | `[DIRECT][Static] + [INFERENCE][Static]` | exact boundary sides, step-doubling refinement, event bracket/order, joint vapor/melt, deposition/refreeze no-false-event, cooling/no-event, typed nonconvergence, atomicity, independent schema-v8 reconstruction, and production isolation | evaluation hard-fail + governance claim limit |

| `INV-SNOWENERGY-035` | A default-off terminal receiver may consume only the earliest closed INV-034 event with in-tolerance unallocated energy, exact half-open support, and one atomic retained/rain/melt/refreeze liquid debit-credit-consumed join; INV-034 remains evaluation-only and CoE remains production owner. | `INV-SNOWENERGY-030/034`, physical conservation | `[INFERENCE][Static]` | receipt, energy, support, and production-isolation guards | typed terminal-receiver failure; no recipient/commit |

### Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-SNOWENERGY-001` | future atmospheric/exchange evaluator; analytical invalid-temperature cases | runtime | typed invalid temperature | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-002` | future forcing boundary; negative/non-finite vapor tests | runtime | typed invalid forcing | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-003` | future daylight cloud operator; endpoint/clamp cases | runtime | typed invalid/unavailable cloud forcing | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-004` | future canopy translator; valid/invalid cover cases | runtime | typed invalid canopy state | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-005` | monotonic analytical vector/property gate | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-006` | independently fixed complementary-mixture vectors | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-007` | open/near-closed canopy limiting vectors | test | blocked promotion | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-008` | future shared energy carrier and fixed net-flux vectors | runtime | typed evaluation failure | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-009` | contract review and production source inspection | governance | hard `HOLD` | EB-02 `canopy-sky-view-derivation.md` |
| `INV-SNOWENERGY-010` | EB-03 provider decision and real-consumer closure gate | governance | hard `HOLD` | `GAP-SNOWENERGY-001` |
| `INV-SNOWENERGY-011` | explicit polar-night unavailable branch | runtime | typed cloud-forcing unavailable | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-012` | intended/exact write-set reconciliation | governance | blocked package closure | EB-02 `exact-diff-reconciliation.md` |
| `INV-SNOWENERGY-013` | hourly-forcing/provider cadence assertion | profile | typed cadence mismatch / hard `HOLD` | EB-02 `operand-lineage.csv`; EB-03 evidence required |
| `INV-SNOWENERGY-014` | future atmospheric derived-domain guard; out-of-authority vector | runtime | typed out-of-authority state | EB-02 `analytical-test-vectors.csv` |
| `INV-SNOWENERGY-015` | Stage 3 selector/provider validation and hourly diagnostics | runtime/profile | typed missing provider or blocked campaign | EB-03 runtime tests |
| `INV-SNOWENERGY-016` | direct-production selector matrix | runtime/test | typed selector failure or blocked EB-04 | EB-03 consumer-path evidence |
| `INV-SNOWENERGY-017` | shared vapor/latent operands and reconstruction residual | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-018` | snow-layer/aggregate mutation and liquid-alias separation | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-019` | Stage 3 cold-content closure ledger | runtime/test | typed closure failure | EB-03 conservation evidence |
| `INV-SNOWENERGY-020` | active-volume constructor and surface-provider diagnostics | runtime/test | typed partition failure | EB-03A evidence |
| `INV-SNOWENERGY-021` | independent persistent/thermal partition reconstruction | runtime/test | typed closure failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-022` | coupled active/lower energy update | runtime/test | typed closure failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-023` | mass-selected substep scheduler | runtime/test | typed cadence failure | EB-03A real-consumer evidence |
| `INV-SNOWENERGY-024` | production-source scan and invalid-state vectors | governance/test | hard `HOLD` | EB-03A review and verification |
| `INV-SNOWENERGY-025` | SNOBAL effective-conductivity primitive and persistent unequal-temperature runtime vector | runtime/test | typed conductivity/projection failure | EB-03A conservation evidence |
| `INV-SNOWENERGY-026` | pre-temperature total-ice-mass branch plus unresolved-domain trace diagnostics | runtime/test/profile | preserve state and emit zero exchange below boundary; typed guards remain above boundary | EB-04C contract, replay, and conservation evidence |
| `INV-SNOWENERGY-027` | multilayer density initialization and typed aggregate replay | runtime/test/profile | retain mass-resolved layers; independently reject mass/depth residual beyond tolerance | EB-04D contract, exact-side tests, replay, and conservation evidence |
| `INV-SNOWENERGY-028` | independent snow-mass, vapor aggregation, and vapor-to-sublimation reconstructions | test/profile/governance | apply each operand's unit-explicit tolerance; reject cross-predicate substitution | EB-04E prospective protocol and EB-04S result-blind authority reconciliation |
| `INV-SNOWENERGY-029` | future Stage 3 complete energy carrier and sole melt generator | runtime/test/governance | no partial activation; typed closure failure after cutover | 21N authority decision and future implementation evidence |
| `INV-SNOWENERGY-030` | future same-substep phase/liquid pipeline and three independently reconstructed ledgers | runtime/test/governance | no dual owner, alias, delay, or unresolved thin-pack proxy | 21N chronology and future real-consumer evidence |
| `INV-SNOWENERGY-032` | package-local independent schema-v6 consumer | test/governance | reject invalid evidence; preserve valid capacity truncation as a physical finding and keep persistence held | Stage 3 evolving-carrier plausibility package |
| `INV-SNOWENERGY-033` | contract-derived alias/source checks and wind-custody package evidence | governance | `AUTHORITY_MISSING` / persistence hold; no production correction | Stage 3 wind source-custody package |
| `INV-SNOWENERGY-034` | terminal one-volume evaluator, adaptive step controller, event bracket/localizer, and schema-v8 consumer | evaluation runtime/test/governance | typed domain, step-underflow, rejection-limit, bracket, iteration, or closure failure; no state commit | terminal enthalpy-event package |
| `INV-SNOWENERGY-035` | terminal receiver event/receipt join and independent reconstruction | default-off runtime/test/governance | typed no-recipient failure and exact rollback | terminal handoff implementation package |
| `INV-SNOWENERGY-054` | guarded finalization contraction plus stateful stabilization transition | runtime/numerical/test | typed nonconvergence/refinement; no relaxed publication | WGHL v28 contract-derived transitions and authentic-publication source binding |
| `INV-SNOWENERGY-055` | exact-floor phase-aware complete-support projection | runtime/numerical/test | typed refinement/nonconvergence with atomic state retention | WGHL v31 captured vector, closure oracles, refusal vectors, and publication source binding |
| `INV-SNOWENERGY-056` | pure opposite-sign root and zero-to-one-sided entry with independent component/latent/`W/H` reconstruction | runtime/numerical/test | typed refinement/nonconvergence with exact rollback and no synthetic publication | `v32-active-set` captured/root/direct-support/refusal evidence and fresh-authentic-only source binding |
| `INV-SNOWENERGY-057` | rolling exact active-set transition-reset detector, equation-level `R_W/R_H/R_E/R_T` evaluator, one shared physical-evaluation budget, and `CoupledAuthentic` admission/reseal | runtime/numerical/test | typed nonconvergence/refinement with exact rollback and no trial publication | `v33-phase-consistent-coupled` rolling transition-reset, evaluator, known-root, side-constraint, budget, admission, refusal, replay, and one-day qualification evidence |
| `INV-SNOWENERGY-058` | eight-map stable-monotone eligibility for the unchanged equation-level solver, with exact static receipt/phase/carry-authority-and-representation joins, evolving physical receipt/carry coordinates under exact reconstruction and closure, and the existing shared physical-evaluation budget | runtime/numerical/test | discard private trials and resume raw authentic Picard within remaining budget on any pre-root refusal; no private-trial admission | `v34-stable-monotone` eligibility, merit, static-join/evolving-coordinate, budget, fallback, replay, refusal, and one-day qualification evidence |
| `INV-SNOWENERGY-059` | exact authentic receipt fixed-point stabilization after a coupled root, followed by one independent same-input exact replay under the existing shared physical-evaluation budget | runtime/numerical/test | discard every private/probe artifact and refuse oscillation, nonfinite/constraint/replay mismatch, or budget exhaustion; no cross-input exact comparison | `v35-authentic-receipt-stabilization` probe/input/output/replay/budget/refusal and retained-r83 evidence |
| `INV-SNOWENERGY-060` | geometry-complete coupled solve with one terminal density coordinate per affected lane, canonical mass-depth reconstruction, physical `R_rho`, unchanged density/settling branch authority, shared budget, and v35 finalization | runtime/numerical/test | typed refusal with exact rollback; no map-difference residual, interpolation/repair, uncharged physics, or bypass | `v36-geometry-complete` retained-r88, residual, branch, budget, poison, receipt, and finalization evidence |
| `INV-SNOWENERGY-061` | derived terminal thickness closure from canonical `I/rho` participates in coupled root merit/admission without becoming an independent coordinate | runtime/numerical/test | typed refusal with exact rollback; no omitted depth merit, independent-z solve, interpolation/repair, uncharged physics, or bypass | `v37-derived-thickness-closure` retained-r93, exact `I/rho`, depth-merit, budget, replay, poison, and finalization evidence |
| `INV-SNOWENERGY-062` | every charged coupled residual/replay evaluates the authentic finalization-equivalent endpoint map directly | runtime/numerical/test | typed refusal with exact rollback; no provisional-map admission, second or uncharged Stage 3 map, repair, or finalization bypass | `v38-finalization-equivalent-map` retained-r95, exact operand/posture, one-map budget, receipt replay, and independent-finalization evidence |
| `INV-SNOWENERGY-063` | V2 soil-energy operands bind distinct exact outer source and authenticated soil-target transactions | runtime/custody/test | typed identity refusal with exact rollback; no adjacent-successor inference, source/target substitution, receipt reseal, or publication | `v39-soil-energy-transaction-separation` retained-r102, first/second-child chain, digest, source/target poison, and canonical-consumer evidence |
| `INV-SNOWENERGY-064` | four-window parity-monotone active-set eligibility for the unchanged equation-level solver under the existing shared budget | runtime/numerical/test | typed refusal on nonfinite, stagnant, reversed, nonconsecutive, nonexact-static, late-budget, or publication posture; no approximate root admission | `v40-parity-monotone-active-set` retained-r107, rolling-window, drift, reserve, poison, replay, and one-day qualification evidence |
| `INV-SNOWENERGY-065` | exact one-way constant-water canonical enthalpy-boundary bracket eligibility for the unchanged equation-level solver | runtime/numerical/test | typed refusal on nonfinite, water drift, stagnation, reversal, multiple/no crossing, nonexact-static, late-budget, or publication posture; no tolerance or root admission | `v41-one-way-phase-boundary` retained-r109, exact bracket, cadence, reserve, poison, replay, and one-day qualification evidence |
| `INV-SNOWENERGY-066` | exact cold-content-export-complete canonical `W/H` support coordinates | runtime/numerical/test | typed endpoint/refinement refusal on missing, nonfinite, negative, reordered, substituted, or nonclosing export; no ledger repair or tolerance | `v42-cold-content-export-coordinate` snow-reappearance capture, zero-export byte lock, interpolation, omission/substitution/order/closure poisons, and persisted-restart evidence |
| `INV-SNOWENERGY-067` | typed V2 numerical-coordinate fixed-point custody distinct from ordinary base unpublished physics | runtime/custody/test | typed owner-closure refusal on erased/mixed posture, foreign authority, receipt/support/order/carry substitution, nonempty credits, sequential use, acceptance, or publication | `v43-projected-base-custody` retained-r113, exact custody first-difference, positive endpoint-carrier vector, substitution poisons, rollback, and no-publication evidence |
| `INV-SNOWENERGY-069` | tolerance-closed private coupled-root polishing plus protected authentic probe/replay capacity inside the unchanged shared budget | runtime/numerical/test | typed refusal on above-tolerance stagnation/non-descent, side/branch/finite/budget/replay failure; no tolerance or receipt repair | `v45-authentic-receipt-root-polishing` retained-r117/r118, reserve, carried-best, polishing, exact-replay, poison, rollback, and no-publication evidence |
| `INV-SNOWENERGY-070` | dimension-complete safeguarded-step budget preflight before any finite-difference column is charged | runtime/numerical/test | private sub-tolerance stop or typed above-tolerance budget refusal before partial-step work; no new numerical method or reserve theft | `v46-complete-step-budget-preflight` retained-r119/r120, exact dimension/reserve, zero-partial-charge, probe-chain/replay, poison, rollback, and no-publication evidence |
| `INV-SNOWENERGY-071` | typed native-V2 atomic complete-owner transaction posture for same-ID first children and exact authenticated predecessor/target second children | runtime/custody/test | typed identity refusal before mutation; no inferred adjacency, source/target substitution, rollback mutation, or private publication | `v47-atomic-complete-owner-transaction-posture` retained-r121, first/second-child chain, owner-disagreement, substitution poison, rollback, and no-publication evidence |
| `INV-SNOWENERGY-072` | explicit authenticated-prepared-beginning transaction authority reaches the real native-V2 fixed-point final install without weakening the generic same-ID path | runtime/custody/test | typed identity refusal before mutation; no erased authority, generic split admission, adjacency inference, rollback mutation, or private publication | `v48-fixed-point-final-install-authority` retained-r122 real-path, prepared-beginning, accepted/seal, generic-refusal, rollback, and no-publication evidence |
| `INV-SNOWENERGY-073` | opaque three-domain prepared-install authority binds mutually equal outer source, exact authenticated soil resident/predecessor, and exact prepared target across repeated same-parent soil children | runtime/custody/test | typed identity refusal before mutation; no predecessor/source alias, adjacency inference, owner rebasing, rollback mutation, or private publication | `v49-multi-child-prepared-install-authority` retained-r123/r124 exact source42/resident43/target44/predecessor43 path, repeated-child, full substitution, rollback, and no-publication evidence |
| `INV-SNOWENERGY-074` | V49 ending source is anchored to the exact validated covered V8 envelope transaction, not the lawfully heterogeneous constitutive beginning | runtime/custody/test | typed identity refusal before mutation; no beginning mutual-source join, envelope/source substitution, adjacency inference, owner rebasing, rollback mutation, or private publication | `v50-envelope-source-transition-authority` retained-r125/r129 mixed-beginning capture, exact envelope-source path, poisons, rollback, and no-publication evidence |
| `INV-SNOWENERGY-075` | exact one-crossing canonical phase eligibility with strictly contracting alternating corrections only inside the already-entered phase | runtime/numerical/test | typed refusal on pre-crossing reversal, predicate reversal/skip/recross, multiple/no crossing, noncontracting/equal/stagnant/nonfinite correction, water/static/cadence/side/owner/budget poison, or publication posture | `v51-post-crossing-contraction` retained-r130/r132 exact chain, poisons, unchanged solver/replay/finalization, rollback, and no-publication evidence |
| `INV-SNOWENERGY-076` | explicit per-lane snow-candidate CN heat coordinate and physical residual close the authentic receipt-governing endpoint map | runtime/numerical/test | typed refusal on missing/reordered/substituted/nonfinite/sign-wrong heat, static receipt geometry, branch, budget, replay, or publication poison; no receipt repair | `v52-cn-heat-coordinate` retained-r133/r134 exact two-cycle, physical coordinate root, exact receipt/replay, poisons, rollback, and no-publication evidence |
| `INV-SNOWENERGY-077` | same-map endpoint CN heat seeds every fresh and legacy V52 coordinate vector | runtime/numerical/test | typed refusal on prior/retained receipt substitution, endpoint mismatch, lane reorder/cardinality, nonfinite heat, or charged seed reconstruction; no seed repair | `v53-same-map-cn-heat-seed` retained-r135/r136 budget capture, exact endpoint reconstruction, fresh/legacy parity, poisons, unchanged solver and receipt evidence |
| `INV-SNOWENERGY-078` | bounded exact receipt-cycle artifact endpoint witness search under unchanged physical closure and replay | runtime/numerical/test | typed refusal on cycle length, budget, projection, residual, branch, receipt, replay, custody, or publication poison; no lattice interpolation or repair | `v54-representable-receipt-cycle-witness` retained-r137/r138 exact cycle, endpoint reconstruction, full-map witness, replay, poisons, rollback, and no-publication evidence |
| `INV-SNOWENERGY-079` | optional bounded exhaustive private binary64 Q-lattice fixed-point witness before residual polishing | runtime/numerical/test | hard typed refusal on invalid Q shape/finiteness/lineage and every post-commit error; zero-charge specialization miss for domain/cardinality/capacity preserves unchanged V45 polishing | `v55-private-q-lattice-witness` retained-r139/r140 exact two-member miss, r142 overcapacity fallthrough, atomic commit boundary, full-map candidates, authentic stabilization/replay/finalization, poisons, rollback, and no-publication evidence |
| `INV-SNOWENERGY-080` | frozen/noncrossing temperature-primary snow solve with exact enthalpy high-plus-carry compound material ownership | runtime/numerical/custody/restart/test | zero-charge ineligibility may continue V55; every committed solve, owner, receipt, replay, finalization, migration, downgrade, or restore error is typed fail-closed | `v56-frozen-temperature-primary` r144 transient-witness rejection; exact dyadic/RN-even vectors; CN exact-once; compound owner/carry receipt; committed/pending/in-progress restart; migration/downgrade poisons; rollback/no-publication evidence |
| `INV-SNOWENERGY-081` | bounded external-liquid residue is eligibility-neutral without ledger normalization, with zero-charge post-root V57 transition before V55 | runtime/numerical/custody/test | negative, nonfinite, or greater-than-bound liquid is ineligible before charge; every post-charge error is typed fail-closed | retained r147 residue/budget evidence; exact boundary/one-bit-above/negative/nonfinite vectors; unchanged operand reconstruction and ledger closure; post-root budget and V55 non-dispatch evidence |
| `INV-SNOWENERGY-036` | One shared canopy-air node jointly closes reference, all V11 canopy, and Stage 3 snow sensible/vapor exchange. | Child 2C carrier authority | `[INFERENCE][Static]` | carrier residual | `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-037` | Wind is a sealed exposure-projected operand at declared transfer geometry; raw 10 m wind and fixed attenuation are not substitutes. | Child 2C exposure authority | `[INFERENCE][Static]` | exposure join | `SNOWENERGY-E-WIND-001` |
| `INV-SNOWENERGY-038` | Canopy--snow--sky longwave uses one reciprocal current-trial state and exact-one exchange. | Child 2C longwave authority | `[INFERENCE][Static]` | radiation lineage | `SNOWENERGY-E-LW-001` |
| `INV-SNOWENERGY-039` | Snow fluxes stop at the accepted event and snow-free fluxes begin only on admitted successor support. | Child 2C chronology authority | `[INFERENCE][Static]` | event/regime join | `SNOWENERGY-E-REGIME-001` |
| `INV-SNOWENERGY-040` | Canopy-intercepted snow is outside this carrier and cannot enter its mass or energy ledgers. | Child 2C scope authority | `[INFERENCE][Static]` | scope guard | `SNOWENERGY-E-SCOPE-001` |
| `INV-SNOWENERGY-042` | One persistent Stage 3 lane owner is OFE-ground. Complete typed tile-ground snow-surface fluxes aggregate exactly once as `sum_i(f_i X_i)` over an ordered tile set closing to one within `TOL-SNOWENERGY-002`; the tolerance never authorizes renormalization. Every contribution binds the same beginning lane-state identity, snow-surface temperature, and latent heat. Missing, duplicate, wrong-class, wrong-model, covered-subset-normalized, or restart topology/basis substitutions fail closed. Uniform terminal liquid preserves `sum_i(f_i M_i)=M_lane`; dividing the complete lane amount by every tile fraction is prohibited. | `REF-SNOWENERGY-USER-OFE-GROUND-V15`; single-column Stage 3 and terminal-receiver state semantics | `[DIRECT][Static] + [INFERENCE][Static]` | lane topology/source-set/common-state/restart guards | `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-043` | Covered fixed-point acceptance first reconstructs and validates each candidate fingerprint independently. Schema, terminal-event model, lane, interval, layer cardinality/order, density, and stored count-like settling chronology compare exactly. Numeric state compares only under the physical-class absolute bounds in `TOL-SNOWENERGY-003`; candidate fingerprint equality is neither required nor a substitute for those comparisons. | existing Stage 3 closure scales and covered carrier temperature policy | `[DIRECT][Static] + [INFERENCE][Static]` | typed convergence/nonconvergence and stale-fingerprint guards | `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-054` | A failed authentic finalization rebuild applies the existing guarded support-scaled contraction only to unpublished continuous Stage 3 state, retains authentic final LSE/boundaries and converged soil, and consumes exactly one additional guarded Picard crossing before retry. Authentic candidate density is copied bitwise and never interpolated; a density difference still prevents convergence and final acceptance remains authentic-map-only. | version-23 bounded Picard authority, version-28 finalization diagnosis, physical closure | `[DIRECT][Static] + [INFERENCE][Ran]` | finalization restart/refusal, exactly-once stabilization state, candidate-density, cumulative-closure, and authentic-publication guards | typed nonconvergence/refinement; no relaxed publication |
| `INV-SNOWENERGY-055` | At exactly `60_000_000_000 ns`, a finite authentic terminal-one-volume open-snow image below the existing `200 K` boundary domain may seed exactly one unpublished `w=0.5` phase-aware contraction only when current and authentic images derive from the same immutable beginning and exact support, retain identical schema/model/lane/cursor/layer/density/settling/initial/input/support/topology/custody/receipt identities, use complete finite support operands with the same signed-vapor disposition and independently closed component energy, and straddle only the canonical phase kink. For each image `j`, reconstruct `D_j=max(V_j,0)`, `S_j=min(max(-V_j,0),I0)`, `W_j=I0+L0+D_j-S_j+Lin_j`, and `H_j=-C0+Lf*(L0+Lin_j)+Q_j`. Form the coordinated midpoint operand vector, require its reconstruction to equal `W_*=(W_current+W_authentic)/2` and `H_*=(H_current+H_authentic)/2`, and apply the existing version-22 projection exactly: `H_*<0 -> (I,L,C,U)=(W_*,0,-H_*,0)`; `0<=H_*<Lf W_* -> (W_*-H_*/Lf,H_*/Lf,0,0)`; `H_*>=Lf W_* -> (0,W_*,0,max(H_*-Lf W_*,0))`, with exact zero-enthalpy and fusion-capacity boundary sides. Reconstruct melt, refreeze, temperature, thickness, cumulative mass/energy fields, and fingerprint from immutable beginning plus the coordinated operands; independent interpolation of ice, liquid, cold content, melt, refreeze, or any cumulative field is forbidden. Raw authentic history is retained. The midpoint cannot satisfy convergence, finalization, replay, acceptance, or publication; only a later fresh authentic image can do so under every unchanged predicate. Any poison or failed reconstruction is typed failure under the unchanged 96-iteration cap, tolerances, exact floor, rollback, and ledger rules. | version-22 canonical phase projection, `INV-SNOWENERGY-048/049/052/054`, physical conservation | `[DIRECT][Static] + [INFERENCE][Ran]` | captured `1860..1980 s` parent composed-trial mixed/frozen vector and its exact 60-second child midpoint; independent water/enthalpy and mass/energy oracles; zero-`H`/fusion-capacity exact sides; vapor-sign, nonfinite, structure, identity, receipt, and component-closure poisons; no-intermediate-publication source binding | typed refinement/nonconvergence; no component repair, tolerance, cap, floor, equation, event, custody, receipt, rollback, or publication change |
| `INV-SNOWENERGY-056` | Version 32 retains `INV-SNOWENERGY-055` unchanged for same-disposition endpoints. Only when ordinary guarded contraction is blocked by a terminal-one-volume vapor active-set reversal on an exact covered support `h>=60_000_000_000 ns` may two independently closed, pure one-sided endpoint images with identical immutable beginning/support/schema/model/lane/cursor/layer/density/settling/initial/input/event/topology/custody/receipt identities and strictly opposite finite nonzero signed actual vapor enter this branch. Orient endpoints `0 -> 1` and derive the unique `alpha_v=-V_0/(V_1-V_0)` with `0<alpha_v<1`; require each endpoint to satisfy exactly one of `(V=D>0,S=0,Q_v=V L_s)` or `(V=-S<0,D=0,Q_v=V L_s)` with finite `L_s>0`. At the interface set `V=D=S=Q_v=+0` exactly, interpolate only external liquid and each ordered nonlatent energy component at `alpha_v`, recompute complete energy without latent energy, reconstruct canonical `W=I0+L0+D-S+Lin` and `H=-C0+Lf*(L0+Lin)+Q`, and apply the unchanged version-22 phase projection. Affine interpolation of endpoint latent energy is forbidden because differing endpoint `L_s` does not preserve `Q_v=V L_s`; for the captured `1860..1920 s` images it yields `+45.77845449909091 J m^-2` although vapor is zero. From an exact interface, a later fresh authentic pure one-sided endpoint may seed one numerical branch entry with the existing `w_p=max(0.25,min(0.5,120 s/h))`: set `V_*=w_p V_auth`, derive exclusive `D_*` or `S_*`, preserve the authentic finite `L_s,auth>0` and set `Q_v,*=V_* L_s,auth`, contract external liquid and ordered nonlatent components with the same `w_p`, recompute `Q`, and project `W/H`. The interface and branch-entry images are synthetic numerical states: they cannot be authentic, converged, finalizable, replayable, acceptable, persistable, or publishable; raw authentic history is retained and only a later fresh authentic image may satisfy unchanged finalization and publication. Mixed `D/S`, same-sign dispatch, either zero endpoint, zero/zero entry, nonfinite or out-of-range `alpha_v`, bad latent sign/linkage, incomplete components, capacity/closure failure, or any support/identity/event/topology/custody/receipt/rollback/publication mutation fails typed without state mutation under the unchanged 96-iteration cap. | `REF-SNOWENERGY-WGHL-V32`, version-22 canonical phase projection, `INV-SNOWENERGY-017/048/049/052/054/055`, physical mass/energy conservation | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | exact captured opposite-sign operands and root; independent nonlatent/root/`W/H` oracle; affine-latent wrong-formula rejection; exact-floor and `>60 s` direct-support vectors; same-sign v31 route; mixed/zero/nonfinite/alpha/latent/component/identity/event/topology/custody/receipt/capacity/cap/rollback/publication poisons; fresh-authentic-only source binding | typed refinement/nonconvergence; no tolerance, cap, floor, constitutive equation, adaptive policy, event, custody, receipt, rollback, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-044` | Additive restart must not restore `OPENWEPP_LANE_STAGE3_BOUNDARY_RECEIPT_V1`. Before restart implementation, a normative V2 successor must exclude numerical initial-guess identity and exactly join configured lane/OFE and tile topology, covered/open final boundaries, component carrier, installed LSE owner, complete snow owner, wet-liquid authorization, and canonical parent/restart framing. Until its exact fields, framing, ordering, and test vectors are admitted, the successor is `SCHEMA_UNDEFINED / IMPLEMENTATION_BLOCKED`. | canonical receipt/restart governance; v15 adopter-wire limitation | `[INFERENCE][Static]` | schema/version/topology/owner replay guards | restart hard `HOLD` |
| `INV-SNOWENERGY-045` | Each accepted support owns exactly one sealed, canonically ordered precipitation phase-parcel set. A parcel binds support, lane/OFE, destination tile, phase, tile-ground mass basis, source and destination identities, temperature/enthalpy provider, producer beginning-state identity, and receipt identity. Zero precipitation is a present, complete empty set, never an omitted owner. | `REF-SNOWENERGY-PRECIP-CUSTODY-V17` | `[DIRECT][Static] + [INFERENCE][Static]` | parcel-set schema/order/cardinality/identity guard | `SNOWENERGY-E-PRECIP-001` |
| `INV-SNOWENERGY-046` | At each ground destination, liquid custody is exclusive: an open destination receives its sealed raw atmospheric rain parcel, while a covered destination receives only terminal throughfall/drainage and stemflow parcels produced under `SC-VEGETATION-001@28`. Raw rain and vegetation release cannot both be delivered to one destination. Solid atmospheric precipitation bypasses vegetation, remains ground-snow precipitation, and canopy-intercepted snow remains excluded. | `SC-VEGETATION-001@28`; `REF-SNOWENERGY-PRECIP-CUSTODY-V17` | `[DIRECT][Static]` | destination/source/phase exclusivity guard | `SNOWENERGY-E-PRECIP-001` |
| `INV-SNOWENERGY-047` | Stage 3 reconstructs precipitation mass and precipitation-advected heat from the identical accepted parcel identities. Tile-ground parcel operands aggregate once to the lane as `sum_p(f_destination,p * X_p)` on the existing OFE-ground basis, without covered-fraction renormalization. Missing, duplicate, substituted, differently ordered, or mass-only/advection-only parcel use fails before candidate publication. | `INV-SNOWENERGY-042`; `REF-SNOWENERGY-PRECIP-CUSTODY-V17` | `[INFERENCE][Static]` | same-set/area-basis/exact-once reconstruction guard | `SNOWENERGY-E-PRECIP-001` |
| `INV-SNOWENERGY-048` | The persistent lower boundary is exactly one OFE/lane interface from the bottom represented snow thermal volume to the first ordered OFE soil-thermal node. No tile soil temperature participates; first-tile selection, tile averaging, covered-only averaging, tile-fraction weighting, duplicated lane flux, or silent zero heat is prohibited. | `REF-SNOWENERGY-SOIL-BOUNDARY-V18`; `SC-LANDSURFACEENERGY-001@8` | `[DIRECT][Static] + [INFERENCE][Static]` | topology/node/owner/basis guard | `SNOWENERGY-E-SOIL-HEAT-001` |
| `INV-SNOWENERGY-049` | With positive finite `dz_sb,lambda_sb,dz_1,lambda_1`, `g_ss=1/(dz_sb/(2*lambda_sb)+dz_1/(2*lambda_1))=2/(dz_sb/lambda_sb+dz_1/lambda_1)`. `G_ss,e=g_ss*(T_sb,e-T_1,e)` for endpoint `e in {0,1}` and the accepted support flux is `bar(G_ss)=0.5*(G_ss,0+G_ss,1)`, positive downward. Both ending temperatures participate in the covered fixed point; beginning values come only from sealed beginning owners. | `REF-SNOWENERGY-SOIL-BOUNDARY-V18`; LSE Crank--Nicolson authority | `[DIRECT][Static] + [INFERENCE][Static]` | physical-operand/endpoint/convergence guard | `SNOWENERGY-E-SOIL-HEAT-001` |
| `INV-SNOWENERGY-050` | The Stage 3 candidate records exactly `-bar(G_ss)` and the first-node soil-thermal candidate records exactly `+bar(G_ss)` on the same support and OFE-ground basis. One sealed `SnowSoilHeatReceiptV1` binds support, lane/OFE, topology/configuration digests, both beginning-owner identities, the four resistance operands, both endpoint temperature pairs, accepted flux, both candidate-ending identities, and a reconstructable digest. Independent validation reconstructs the receipt and equal/opposite debits from primitives before atomic publication; any omission, substitution, sign/basis error, nonconvergence, or later failure rolls back both owners and all receipt state. | physical conservation; `SC-LANDSURFACEENERGY-001@8` | `[INFERENCE][Static]` | receipt/reconstruction/atomic-owner guard | `SNOWENERGY-E-SOIL-HEAT-001` |

| `INV-SNOWENERGY-057` | When immutable raw authentic covered evaluations reproduce an exact finite `A/B/A` period-two phase/vapor active-set cycle on the same event-free support, identity, topology, custody, and receipt set, version 33 supersedes version-31/32 synthetic production control for that cycle. `phase_consistent_coupled_solve_v1` solves only each affected lane's ending total-water/enthalpy coordinates `(W_1,H_1)` and the soil endpoint enthalpy/temperature coordinates required by the existing coupled Crank--Nicolson equations. Every trial applies the unchanged canonical `Pi(W,H)` phase projection, reevaluates the existing covered LSE trial, and reconstructs `R_W` from the exact ordered water ledger, `R_H` from the complete ordered energy ledger including `Q_v=V L_s` and the unchanged snow--soil Crank--Nicolson term exactly once, plus the existing soil Crank--Nicolson residuals. A deterministic safeguarded semismooth Newton/trust-region method uses the active side of `Pi` and remains within the single unchanged cumulative 96-evaluation budget, including authentic cycle detection, residual/generalized-Jacobian evaluations, rejected trust trials, and final replay/reseal. An internal sealed event boundary forces prior partition or typed refusal; the phase kink is internal complementarity and is not a new event. A root is eligible only after `phase_consistent_coupled_authentic_final_evaluation_v1` and `phase_consistent_coupled_authentic_final_replay_reseal_v1` freshly reproduce and reseal the physical result from immutable beginning owners and sealed inputs under every unchanged convergence, exact identity, water/energy closure, phase-capacity, LSE, soil, event, topology, custody, receipt, rollback, and publication guard. Affine or synthetic states, including every v31/v32 midpoint/interface/branch-entry image, remain ineligible for acceptance or publication; v31/v32 evidence remains a diagnostic/refusal oracle. Nonfinite coordinates/residuals, a non-exact cycle, same-side dispatch, singular or non-descent generalized systems, trust-region stagnation, domain or phase-capacity failure, incomplete/reordered components, bad latent linkage, soil/LSE failure, identity/event/topology/custody/receipt mutation, budget exhaustion, replay mismatch, or attempted intermediate mutation fails typed and atomically rolls back. | `REF-SNOWENERGY-WGHL-V33`, version-22 canonical phase projection, `INV-SNOWENERGY-017/043/048/049/050/052/054/055/056`, physical mass/energy conservation | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | exact retained 60/120-second v32 period-two capture; independent cold/mixed-phase/fusion-boundary roots; complete water/energy/soil residual reconstruction; root distinctness from v31/v32 affine/synthetic states; event, active-set, nonfinite, singularity, stagnation, domain, capacity, component, soil, LSE, identity, topology, custody, receipt, budget, replay, rollback, and publication refusals; canonical one-day performance qualification | typed refinement/nonconvergence; no equation, tolerance, cap, floor, adaptive controller, event, topology, custody, receipt, rollback, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-058` | Version 34 may invoke the unchanged version-33 `phase_consistent_coupled_solve_v1` without an active-set reset only after `covered_stable_monotone_solve_eligibility_v1` validates eight consecutive raw authentic maps on the same event-free terminal-one-volume covered support `>=60_000_000_000 ns`. Every map must retain exact support, sealed source, event, topology, custody, canonical phase branch, static receipt joins (schema, support, topology, ordered operand lineage, and beginning-owner/authority identities), and exact V2 carry authority/representation (signed-dyadic schema/version, normalization, units, and layer/owner/order); expose finite concrete `R_W/R_H/R_E/R_T`; reconstruct each evolving soil enthalpy exactly as `E=exact(H_hi)+R`; and reduce the governed residual merit strictly relative to its predecessor. The physical applied-energy value, candidate-ending identity, resealed receipt digest, `H_hi`, and `R` coordinate values may evolve across authentic maps; byte equality of those physical coordinates is neither required nor a substitute for exact custody, carry representation, reconstruction, residual, closure, replay, and reseal. An `A/B/A` record, any active-set or branch change, any finalization restart, nonfinite or nondecreasing merit, changed static join/authority/representation, or failed exact reconstruction refuses stable-monotone eligibility. All eight raw maps and every solver residual, generalized-Jacobian, trust, rejected, fresh, and final-replay evaluation charge one unchanged `CoveredPhysicalEvaluationBudgetV1` capped at 96. A pre-root refusal destroys all private trials and resumes ordinary raw authentic Picard with only the remaining budget; it cannot reset the account or admit a private state. A root is eligible only through the unchanged fresh `CoveredConvergenceAdmissionV1::CoupledAuthentic` evaluation/replay/reseal path, preserving exact receipt custody and carry, every physical and algebraic residual tolerance, mass/energy ledger closure, phase/capacity, LSE, soil, event, topology, finalization, rollback, and publication guard. | `REF-SNOWENERGY-WGHL-V34`, `INV-SNOWENERGY-057`, exact V2 soil-enthalpy carry authority, physical mass/energy conservation | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | eight-map exact-side trigger; evolving physical receipt digest and `H_hi/R` coordinates; changed static join/phase/carry authority/representation, failed exact reconstruction, nonfinite, nondecreasing-merit, `A/B/A`, active-set, restart, budget, private-admission, fallback, replay, reseal, rollback, and publication refusal vectors; canonical one-day performance and closure qualification | typed refinement/nonconvergence or raw-Picard continuation within remaining budget; no equation, tolerance, cap, floor, adaptive controller, event, topology, custody, receipt, rollback, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-059` | After version 33/34 obtains a coupled physical root, version 35 stabilizes the complete authentic physical receipt input before admission. For immutable input receipt set `R_n`, `covered_authentic_receipt_stabilization_probe_v1` reconstructs the physical candidate and canonical output receipts `R_(n+1)` from immutable beginning owners and sealed sources. Every probe charges the existing shared `CoveredPhysicalEvaluationBudgetV1`; no probe resets or owns a separate budget. `R_(n+1)` becomes the next immutable authentic physical input without tolerance comparison, digest repair, or in-place mutation. The first private-root-to-authentic reseal is a probe evaluated under `R_n`; it cannot be compared for exact acceptance to root or prior artifacts produced under a different receipt input. Stabilization occurs only when canonical input and reconstructed output receipt bytes and every authenticated receipt field are exactly equal. The implementation then performs exactly one independent `covered_authentic_receipt_stabilization_replay_v1` with that same stabilized receipt input and requires exact equality of finite physical residuals, complete candidate artifacts, and reconstructed receipts with the stabilizing probe. Only that same-input exact replay may proceed to `CoveredConvergenceAdmissionV1::CoupledAuthentic` and unchanged finalization. Any exact receipt period/oscillation, nonfinite coordinate/residual, side-constraint or receipt reconstruction failure, budget exhaustion, replay disagreement, digest/tolerance repair, in-place mutation, or attempted retention/publication of a private/probe artifact fails typed and discards every private/probe artifact. | `REF-SNOWENERGY-WGHL-V35`, `INV-SNOWENERGY-057/058`, exact receipt custody and rollback authority | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | r83 typed replay/reseal refusal; exact input-output stabilization; first cross-input probe classification; multiple-probe budget charging; exact oscillation/nonfinite/side-constraint/budget/refusal vectors; same-input independent replay exact residual/artifact/receipt equality; artifact disposal and no-publication proofs | typed refinement/nonconvergence with exact rollback; no equation, tolerance, cap, floor, adaptive controller, event, topology, custody, receipt meaning, exact carry, rollback, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-060` | Version 36 extends the version-33/34 reduced coordinate vector by exactly one terminal-one-volume density coordinate `rho_1,l` for each affected lane. `I_1,l` remains the ice mass produced by canonical `Pi(W_1,l,H_1,l)` and thickness is derived, never independently solved or interpolated, as `z_1,l=I_1,l/rho_1,l` on the existing mass-depth basis. `covered_terminal_density_geometry_residual_evaluate_v1` reconstructs `R_rho,l` from the unchanged Stage 3 density/settling constitutive map using immutable beginning state, sealed forcing, and current physical operands. The solver evaluates generalized physical residuals `R_W/R_H/R_rho/R_E/R_T`; `F(x)-x`, density/thickness interpolation, tolerance repair, and post-hoc geometry repair are forbidden. Stable eligibility preserves exact layer cardinality/order/identity, settling authority, and density-model branch while allowing finite positive `rho_1,l` and derived `z_1,l` to evolve. Every new physical, Jacobian, trust, rejected, fresh, v35 receipt-probe, and replay evaluation charges the same unchanged `CoveredPhysicalEvaluationBudgetV1` capped at 96. A root remains private until v35 exact receipt stabilization, same-input replay, `CoupledAuthentic`, and every unchanged authentic finalization guard pass. | `REF-SNOWENERGY-WGHL-V36`, `INV-SNOWENERGY-057/058/059`, unchanged Stage 3 density/settling authority and mass-depth basis | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | r88 exact thickness bits/delta; known geometry root; independent `I/rho` reconstruction; physical `R_rho` distinct from map difference; evolving rho/z with exact branch joins; zero/negative/nonfinite density, mass-depth, branch, settling, layer, budget, interpolation, repair, uncharged-physics, receipt-bypass, finalization-bypass, rollback, and publication poisons | typed refinement/nonconvergence with exact rollback; no equation, tolerance, cap, floor, event, topology, custody, receipt, exact carry, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-061` | Version 37 extends the geometry-complete evaluation result with derived physical `R_z,l=z_1,l-z_phys,l` for each affected terminal-one-volume lane. Both thicknesses are reconstructed on the existing mass-depth basis from canonical ice mass and the unchanged Stage 3 density: `z_1,l=I_1,l/rho_1,l` and `z_phys,l=I_phys,l/rho_phys,l`. `R_z` enters the safeguarded merit and root-admission predicate under the unchanged `TOL-SNOWENERGY-003` `depth_abs_m`; it does not add an unknown or replace the solved `R_W/R_H/R_rho/R_E/R_T` vector. Thus low-density amplification of an admissible water-mass residual cannot pass while the derived physical depth remains outside its existing bound. Every private, Jacobian, trust, rejected, fresh, v35 receipt-probe, and replay evaluation derives `R_z` from the same already-charged unchanged physical map. V35 exact receipt stabilization, same-input replay, `CoupledAuthentic`, and authentic finalization remain unchanged. | `REF-SNOWENERGY-WGHL-V37`, `INV-SNOWENERGY-057/058/059/060`, canonical mass-depth basis and `TOL-SNOWENERGY-003` | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r93 bits/delta; exact root-to-finalization `I/rho` reconstruction; low-density amplified-water vector; one-coordinate-only guard; derived-depth merit/admission; budget/no-extra-map; receipt/replay/finalization closure; omission/independent-z/interpolation/repair/bypass poisons | typed refinement/nonconvergence with exact rollback; no equation, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-062` | Version 38 requires every v33--v37 charged physical evaluation to execute the authentic finalization-equivalent endpoint map, never the provisional/outer-iteration proxy exposed by r95. For each canonical proposed `W/H/rho/E/T` vector, `covered_phase_consistent_finalization_equivalent_map_v1` reconstructs unpublished terminal snow and exact high-plus-carry soil operands from immutable beginnings. Soil `E/T` coordinates govern exactly the first ordered snow-coupled node of each OFE; every deeper soil layer retains bit-exact high term, exact carry, temperature, identity, order, and custody, without a one-layer-column restriction, rerounding, zero-carry substitution, or additional solve coordinate. The map builds the non-provisional carrier with the same covered/open endpoint boundaries, precipitation custody, and soil beginning as authentic finalization; applies the same corrected LSE exchange and coordinate-ending boundary merge; consumes the selected immutable CN receipt input; and performs exactly one Stage 3 physical evaluation for that shared-budget charge. Residuals, derived `R_z`, artifacts, and reconstructed receipts all come from that one map. A provisional carrier or outer Picard iterate cannot establish root closure, receipt stabilization, replay equality, or admission. V35 stabilization still requires exact input/output receipts and one exact same-input replay. After `CoupledAuthentic`, independent authentic finalization reevaluates that same endpoint map and requires exact Stage 3, LSE, boundary, receipt, and ledger equality before publication. | `REF-SNOWENERGY-WGHL-V38`, `INV-SNOWENERGY-057/058/059/060/061`, immutable-owner and authentic-finalization authority | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r95 solver/stabilization/finalization tuple and exact bits; finalization-equivalent operand identity; multi-layer top-coordinate projection with deeper exact-carry preservation; one charge/one Stage 3 map; provisional-map negative; exact receipt replay; independent finalization exact equality; budget, extra-map, repair, rollback, and no-publication poisons | typed refinement/nonconvergence with exact rollback; no equation, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-063` | Version 39 requires `PhysicalSoilEnergyTransactionAuthorityV2` to carry two exact, nonzero transaction identities into `physical_soil_energy_operands_v2`: `source_transaction_id` authenticates the immutable surface-ingress candidate and every source receipt, while `soil_thermal_transaction_id` authenticates the prepared/unpublished V2 soil target and seals the soil-internal and infiltration debit-credit operand digests. The two values are equal for an ordinary first child and may differ for an authenticated later child; inequality alone is neither permission nor refusal. The exact source candidate must validate against `source_transaction_id`, the exact prepared soil beginning must independently validate against `soil_thermal_transaction_id`, and every operand digest must bind both values, the exact support, source owner, candidate/receipt lineage, layer, ordinal, basis, and energy. Initial unpublished, physical-beginning, same-support continuation, and finalization-equivalent private coordinate-projection custody must use the transaction from an independently validated `PreparedSoilThermalSupportV2` beginning; they must never compare or prepare the soil successor using the outer source transaction. Initial and finalization-equivalent projection supports obtain that prepared owner only through the resident's authenticated next-support custody chain for the exact child support. Legacy V1 candidate posture is not superseded: source and soil transaction remain one exact identity, the V1 beginning identity remains exact, and the legacy one-transaction digest domains and bytes remain unchanged; mixing V1 and V2 posture or identities fails typed. No caller may infer a target by incrementing the source transaction, copy either transaction into the other, accept a merely adjacent value, rewrite a source receipt, or publish a private operand set. Any zero, stale, foreign, swapped, missing, out-of-order, support, owner, receipt, prepared-beginning, continuation, projection, or digest mismatch fails before soil acceptance with exact rollback. | `REF-SNOWENERGY-WGHL-V39`, `INV-SNOWENERGY-062`, exact V1/V2 owner/receipt/source custody | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r102 exact outer-42/soil-43 failure; retained r103 downstream continuation failure; retained r104 initial/private-support failure; parent/first/second-child vectors; same-source/different-target digest vector; second-child prepared-beginning and private-projection replay; legacy V1 single-transaction digest vector; zero/stale/foreign/swapped/out-of-order/support/owner/receipt/prepared-beginning/continuation/projection/digest and mixed-posture poisons; canonical composed consumer | typed identity refusal with exact rollback; no equation, energy amount, V1 identity/digest, tolerance, cap, floor, event, topology, receipt meaning, exact carry, publication, persistence, or diagnostic change |
| `INV-SNOWENERGY-064` | Version 40 may invoke only the unchanged version-38/39 finalization-equivalent physical solver after exactly four consecutive failed-but-valid rolling active-set reset windows. Every window must be event-free terminal-one-volume on one exact support at or above 60 seconds; carry identical exact source/event/topology/custody/static-receipt/density-model/carry-authority joins and identical phase predicates; retain exact opposite pure-vapor side vectors; chain each promoted root bit-exactly from the preceding reset; contain finite root/reset `W/H` coordinates and finite raw authentic owner coordinates; and remain private. Each root/reset drift is the finite positive dimensionless Euclidean norm of coordinate differences scaled only by `max(abs(root),abs(reset),1)`, never by a physical tolerance. Drift must decrease strictly across all four windows, raw authentic parity coordinates must not stagnate or reproduce an `A/B/A` within the observed parity subsequence, and physical evaluation ordinals must be consecutive for the two-map window cadence. Eligibility additionally requires enough unused capacity in the same existing 96-evaluation budget for one initial map, one generalized-Jacobian column per solve coordinate, one trust trial, one authentic receipt probe, and one independent replay; this is a refusal guard, not a second budget or guaranteed admission. Eligibility supplies only the current authentic/interface seed to `phase_consistent_coupled_solve_v1`; it cannot establish residual closure, convergence, receipt equality, replay, finalization, acceptance, or publication. Every evaluation remains charged once to the shared budget and every unchanged V35--V39 physical residual, side constraint, exact receipt stabilization/replay, authentic finalization, rollback, and no-publication guard remains mandatory. Any nonfinite, zero/increasing drift, exact-reset (which takes the V33 route), join/predicate/side/support/cadence/parity mutation, insufficient remaining budget, solver refusal, or private publication fails typed without repair or fallback admission. | `REF-SNOWENERGY-WGHL-V40`, `INV-SNOWENERGY-057/058/059/060/061/062/063`, unchanged physical solver and shared budget | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r107 budget-96 dispatch; four-window decreasing drift; exact rolling chain; no-parity-A/B/A; static join/phase/side/support/cadence/nonfinite/stagnation/reversal/late-budget poisons; shared-budget monotonicity; unchanged receipt replay/finalization/rollback/no-publication; canonical one-day counts/width/runtime/reasons/ledger evidence | typed refinement/nonconvergence with exact rollback; no equation, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-065` | Version 41 may invoke only the unchanged version-38/39 finalization-equivalent physical solver when the same four consecutive failed-but-valid rolling windows refused V40 yet form one exact canonical enthalpy phase-boundary bracket. Every V40 support, source/event/topology/custody/static-receipt/density-model/carry-authority, opposite-pure-vapor-side, promoted-root-chain, finite raw-owner, private-posture, exact two-map cadence, and shared-budget reserve guard remains mandatory, except fixed phase and root-drift descent are not eligibility predicates for this distinct route. For each affected lane, parse the exact ordered `(W,H)` pairs from the promoted root followed by each bit-exact chained reset. `W` must be finite, nonnegative, and bit-identical at all five points. `H` must be finite and strictly one-way at every transition, with one direction shared by all affected lanes; equality, reversal, or direction change refuses. Recompute the canonical phase predicate at every point solely from exact `H<=0`, `0<H<L_f W`, and `H>=L_f W` boundaries and require it to equal the recorded predicate. Exactly one adjacent transition across the entire observed lane set may change by exactly one canonical branch; no missing crossing, direct cold-to-liquid jump, second boundary crossing, or crossing reversal is eligible. The bracket is only a deterministic early-dispatch witness: it applies no tolerance, interpolation, bisection, repair, phase projection, convergence, or root admission. Eligibility supplies only the current authentic/interface seed to the unchanged solver. Every unchanged V35--V40 physical evaluation, shared charge, residual, side constraint, exact receipt stabilization/replay, authentic finalization, rollback, and no-publication guard remains mandatory. | `REF-SNOWENERGY-WGHL-V41`, `INV-SNOWENERGY-057/058/059/060/061/062/063/064`, unchanged canonical phase projection, physical solver, and shared budget | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r109 exact `W=0.3168113... kg m^-2`, monotone `H=-3327 -> -2445 -> -957 -> +1454 -> +2782 J m^-2`, and single zero-boundary crossing; reversal, stagnation, multiple/no crossing, direct branch jump, water drift, nonfinite, join, side, chain, cadence, budget, publication, replay, and rollback poisons; canonical one-day qualification | typed refinement/nonconvergence with exact rollback; no equation, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-066` | Version 42 corrects the incomplete V31/V32 canonical support-coordinate image without changing Stage 3 physics or ledger math. Every authentic support image carries the exact finite nonnegative `X_c=complete_arm_cold_content_export_j_m2` produced by that same Stage 3 evaluation. Canonical total water remains `W=I_0+L_0+P_s+D-S+L_in`. Canonical ending enthalpy is `H=-C_0+L_f(L_0+L_in)+Q+X_c`: removing ice exports its positive cold-content magnitude, so omission would make the retained owner's enthalpy too negative. The zero-`X_c` case is byte-identical to V31/V32. An unpublished midpoint, vapor interface, or branch-entry image contracts `X_c` with the same single scalar weight and endpoint order as external liquid and each ordered nonlatent energy component; `X_c` is never folded into `Q` or the latent component. The synthetic state's existing cold-energy-change ledger is independently reconstructed as retained cold-content change minus refreeze energy and `X_c`, exactly matching the already-governed physical export accounting rather than adding or repairing energy. Endpoint closure must independently reconstruct the authentic state coordinate from immutable beginning, exact support operands, and exact `X_c` before any synthetic image is used. Missing, nonfinite, negative, reordered, omitted, independently weighted, substituted, or nonclosing `X_c`, and any source/support/receipt mismatch, fails typed with exact rollback. V33--V41 residual, trigger, shared-budget, receipt stabilization/replay, authentic finalization, and no-publication guards consume only this corrected coordinate image and remain otherwise unchanged. | `REF-SNOWENERGY-WGHL-V42`, `INV-SNOWENERGY-019/055/056/057/065`, authentic Stage 3 complete-arm result | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | captured snow-reappearance `+2.3211191058 J m^-2` endpoint delta; zero-export byte lock; exact current/authentic/interface/branch coordinate vectors; missing/nonfinite/negative/order/weight/substitution/closure poisons; persisted-restart 71/71 and canonical one-day qualification | typed refinement/nonconvergence with exact rollback; no physical equation, ledger mutation, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, schema, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-067` | Version 43 preserves the V38 numerical soil coordinate image as a typed private fixed-point posture rather than reconstructing it with the ordinary base-unpublished physical constructor. The typed posture must validate the sealed trial, exact resident/prepared beginning, transaction, predecessor, support, beginning-state digest, accepted receipt-chain custody, numerical-coordinate authority and coordinate-set digests, ordered OFE/layer topology, top-layer finite high/temperature with canonical zero exact carry, bit-exact unchanged lower layers and lineage, and an empty physical-credit set. Ordinary base trials still require exact reconstruction from authenticated operands; numerical projections remain forbidden as sequential beginnings, accepted candidates, installed owners, or publication evidence. Erasing or mixing the posture, substituting authority/coordinate/receipt/support/order/high/carry/temperature/lower-layer state, adding credits, or passing the projection to base, sequential, acceptance, or publication APIs fails typed with unchanged owner bytes. | `REF-SNOWENERGY-WGHL-V43`, `INV-SNOWENERGY-062/063`, native V2 numerical-coordinate custody | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r113 first-difference; exact top-layer projection fixed-point validation; ordinary base byte lock; foreign/missing/mixed posture, authority, coordinate-set, receipt, support, order, high/carry/temperature, deeper-layer, nonempty-credit, sequential, acceptance, rollback, and publication poisons | typed owner-closure/refinement refusal with exact rollback; no physical equation, exact-carry arithmetic, tolerance, cap, floor, event, topology, receipt meaning, acceptance, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-068` | Version 44 supersedes V38's non-provisional requirement only for a charged `PrivateTrial` whose LSE reciprocal-longwave/shortwave/sensible/vapor exchange is still uncommitted. That trial uses the existing provisional LSE evaluation posture to obtain corrected boundary exchange and one Stage 3 image; its deferred aggregate weighted-OFE result cannot establish authentic closure, `CoupledAuthentic`, acceptance, or publication. Every authentic receipt-stabilization probe and independent same-input replay uses strict non-provisional posture and must pass the unchanged weighted-OFE energy decomposition/tolerance before admission; independent finalization repeats the same strict closure and exact image comparison. The projected top-soil `E/T` coordinate supplies exactly the existing snow--soil CN receipt/credit calculation once, while Stage3-covered V8 retains its admitted soil beginning with zero LSE ground heat/storage; coordinate omission or V8 double application fails typed. Every evaluation remains one charge/one Stage 3 map under the unchanged shared 96 cap. | `REF-SNOWENERGY-WGHL-V44`, `INV-SNOWENERGY-062/063/067`, unchanged weighted-OFE ledger and snow--soil CN authority | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r116 trial-dependent residuals; private-posture positive vector; strict probe/replay/finalization vectors; reciprocal-LW corrected-boundary vector; projected-coordinate CN exact-once and V8-double-use poisons; budget/rollback/no-publication evidence | typed refinement/nonconvergence with exact rollback; no physical equation, ledger operand/tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, acceptance, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-069` | Version 45 treats the first tolerance-closed private coupled solution as a private root, not proof of binary64 receipt fixed-point exactness. Before constructing `R_0`, `phase_consistent_coupled_root_polish_v1` carries one complete matching physical bundle—residual, artifacts, and finalization inputs—and continues the unchanged generalized-Jacobian/Newton/trust calculation through `R_W/R_H/R_rho/R_E/R_T` plus derived `R_z` while each admitted trial strictly lowers the existing canonical scaled merit. It never forms `F(x)-x`, receipt differences, digest distance, or a new residual. Exact-zero residual bits may stop polishing but are not required. A zero direction or twelve-trial non-descent may stop polishing only when the carried best bundle remains finite, side-valid, same-branch, and within every unchanged tolerance; that stop can seed V35 receipt stabilization but cannot satisfy `CoupledAuthentic`, acceptance, or publication. The ordinary private solver must refuse an evaluation that would leave fewer than three charges; polishing must refuse an evaluation that would leave fewer than two; a nonstable receipt probe must refuse before consuming the last charge reserved for the independent same-input replay. The single budget owner, charge site, and maximum 96 remain unchanged. Exact receipt oscillation, nonfinite/side/branch/singularity failure, above-tolerance stagnation/non-descent, budget exhaustion, stale rejected-trial artifact substitution, replay mismatch, or any private/probe publication fails typed with exact rollback. | `REF-SNOWENERGY-WGHL-V45`, `INV-SNOWENERGY-057/059/060/061/062/068`, unchanged residual/tolerance/receipt/finalization authority | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | r117 tolerance-closed receipt tail and r118 exact budget exhaustion; complete carried-root bundle; exact-zero and sub-tolerance stationary stops; above-tolerance/side/branch/singular/nonfinite poisons; private/polish/probe replay reservations; stable-at-final-slot replay; evolving-at-final-slot refusal; exact replay, rollback, and no-publication evidence | typed refinement/nonconvergence with exact rollback; no equation, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, acceptance, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-070` | Version 46 makes each finite-difference safeguarded step budget-atomic. After validating a nonempty `d`-coordinate residual and the role reserve `r`, the engine must prove at least `d+1+r` unconsumed charges before evaluating the first generalized-Jacobian column. The `d` columns and one trust trial remain ordinary separately charged full-physics maps; this preflight creates no uncharged evaluation or guarantee that a reverse column or later backtrack will fit. If the preflight fails above tolerance, the solver returns typed `EvaluationBudget` without a new charge. If it fails during finite side-valid same-branch sub-tolerance polishing, the exact carried best bundle stops with `ReceiptEntryReserve`, also without a new charge, and enters unchanged V35 stabilization. Existing per-map reserve checks still protect reverse perturbations, rejected trust trials, authentic probes, and independent replay. Coordinate cardinality overflow, malformed shape, enlarged/reset/separate budget, reserve subtraction underflow, partial-column admission, or use of any perturbation/rejected artifact fails typed with exact rollback. | `REF-SNOWENERGY-WGHL-V46`, `INV-SNOWENERGY-057/059/069`, unchanged finite-difference generalized Jacobian, trust map, receipt stabilization, and finalization authority | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | r120 exact budget decomposition; dimension 1/5/multi-coordinate preflight vectors; zero-charge private stop and above-tolerance refusal; full-step boundary; reverse/backtrack per-map reserve; evolving multi-probe receipt chain plus protected replay; overflow/reset/enlarged/separate-budget, stale-artifact, rollback, and no-publication poisons | typed refinement/nonconvergence with exact rollback; no equation, Jacobian method, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, acceptance, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-071` | Version 47 makes native-V2 complete-owner installation transaction-posture exact. First derive one source transaction only if vegetation, LSE, and biogeochemistry source owners are mutually equal. Independently require the accepted soil owner to validate, its state and every accepted layer to be sealed to the owner's exact target transaction, and its accepted custody to remain unchanged. The ordinary/public install admits only `source == target`. Only the authenticated unpublished-continuation install may receive an explicit native-V2 `PhysicalSoilEnergyTransactionAuthorityV2`; it admits a split only when that authority exactly equals the mutually equal source owners and the continuation/prepared target, and the owner's exact authenticated `expected_predecessor_transaction_id == Some(source)` while `source != target`. Missing predecessor on a split, a predecessor equal to a foreign/swapped source, source-owner disagreement, target/state mismatch, erased accepted custody, or any numeric adjacency inference fails typed before cloning or mutation. The first-child same-ID and composed second-child split are explicit alternatives, never inferred successors. Exact accepted no-op and ordinary install both retain the same validation, atomic clone-then-replace, rollback, support/receipt/carry custody, and no-publication behavior. | `REF-SNOWENERGY-WGHL-V47`, `INV-SNOWENERGY-063`, unchanged V2 accepted-owner validation and atomic installation authority | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r121 source42/target43/predecessor42 chain; same-ID first child; explicit-authority exact split second child; foreign, swapped, missing-predecessor, source-owner-disagreement, target/state, support/receipt/custody, rollback, no-op, and no-publication poisons | typed identity refusal with exact rollback; no inferred adjacency and no physics, equation, tolerance, budget, floor, event, topology, receipt meaning, exact carry, serialization, persistence, diagnostic, or publication change |
| `INV-SNOWENERGY-072` | Version 48 propagates V47's exact split posture through the real ordinary fixed-point finalizer. The strict generic/public installer remains `source==target` only. A distinct authenticated-beginning final install must receive the authoritative native-V2 resident and the exact prepared beginning, validate that resident's complete model/run/state/receipt-chain custody and exact prepared target/predecessor/support, reconstruct an explicit native-V2 `PhysicalSoilEnergyTransactionAuthorityV2` from the mutually equal outer source owners and that prepared target, validate the exact accepted result plus orchestrator seals against the same prepared beginning, and only then call the unchanged atomic posture/install. For a split, the prepared beginning and accepted owner must both retain `expected_predecessor_transaction_id==Some(source)` and the accepted state plus every layer must be sealed to target. The real fixed-point finalization call site must use this typed path; calling the generic installer after a prepared split is an authority-erasure failure. Foreign, missing, swapped, stale, support/receipt/state/seal-substituted, legacy-V1, source-owner-disagreeing, inferred-adjacent, or generic-split input fails before clone or mutation. Exact rollback, no publication, owner bytes, receipts, carries, physics, ledgers, tolerances, budget, and finalization remain unchanged. | `REF-SNOWENERGY-WGHL-V48`, `INV-SNOWENERGY-063/071`, unchanged prepared-beginning and accepted-owner validation | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r122 real finalizer; exact source42/target43/predecessor42 prepared-beginning final install; same-ID path; generic split refusal; foreign source/target/predecessor/support/receipt/state/seal and erased-authority poisons; rollback, no-op, no-publication, retained V39/V46/V47 gates | typed identity refusal with exact rollback; no physics, equation, tolerance, budget, floor, event, topology, receipt meaning, exact carry, serialization, persistence, diagnostic, adaptive, or publication change |
| `INV-SNOWENERGY-073` | Version 49 separates three already-authenticated transaction roles across repeated ordinary soil children inside one fixed V11 parent: mutually equal outer vegetation/LSE/BGC source, exact installed native-V2 soil resident/predecessor, and exact prepared soil target. An opaque `DirectSoilThermalPreparedBeginningInstallAuthorityV2` must retain the complete validated authoritative resident and prepared beginning together with the unchanged explicit outer-source/soil-target operand authority. Construction and install both revalidate the resident's model/run/owner/state/layer/receipt/support/latest-accepted custody, the prepared target/exact predecessor/contiguous support/state/receipt joins, mutually equal source owners, and the accepted ending plus seals. Atomic installation may admit `source != predecessor != target` only when the supplied opaque authority equals the independently reconstructed authority byte-for-byte; generic installation and the V47/V48 predecessor-equals-source posture remain strict. No numeric adjacency, transaction arithmetic, owner rebasing, source/predecessor/target copying, or receipt repair supplies authority. Every missing, foreign, stale, swapped, support/receipt/state/layer/seal/authority substitution refuses before clone or mutation with exact rollback and no publication. | `REF-SNOWENERGY-WGHL-V49`, `INV-SNOWENERGY-063/071/072`, unchanged native-V2 resident/prepared/accepted validation | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r123/r124 real finalizer; exact source42/resident43/predecessor43/target44 `1920..2040 s` vector and a further same-parent successor; generic/V48 strictness; complete resident/prepared/accepted/authority poisons; exact no-op, rollback, and no-publication | typed identity refusal with exact rollback; no physics, equation, tolerance, budget, floor, event, topology, receipt meaning, exact carry, serialization, persistence, diagnostic, adaptive, or publication change |
| `INV-SNOWENERGY-074` | Version 50 distinguishes a heterogeneous authenticated constitutive beginning from an ending complete-owner source. `UncommittedCoveredV8OwnerEnvelope::validate` must first prove exact envelope transaction equality across vegetation, physical/hydrology, and BGC owners plus its material receipts. The V11 finalizer must independently require native-V2 envelope transaction equality to the accepted candidate transaction and normalize the candidate vegetation/LSE/BGC ending owners to that exact source. Only that explicit `envelope.transaction_id()` may enter V49 authority construction and installation. The V49 constructor must require the candidate's mutually equal vegetation/LSE/BGC source to equal the supplied envelope source, retain the exact authenticated native-V2 soil resident/prepared custody unchanged, and reconstruct the physical source/soil-target authority from that source and prepared target. It must not call the complete-owner mutual-source validator on the heterogeneous beginning. Construction and install repeat the envelope-source/candidate join and exact V49 custody equality before atomic mutation. A substituted envelope source, jointly rebased or individually changed candidate owner, invalid envelope vegetation/physical/BGC/material join, beginning/resident/prepared/accepted/receipt/seal substitution, missing authority, or inferred adjacency fails typed with exact rollback and no publication. | `REF-SNOWENERGY-WGHL-V50`, `INV-SNOWENERGY-063/071/072/073`, existing covered V8 envelope validation and accepted vegetation transaction-lineage guard | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r125/r129; exact beginning veg41/LSE40/BGC41/soil41 to validated envelope/candidate source42; real helper positive/no-op; envelope source, vegetation/physical/BGC/material receipt, candidate owner, resident/prepared/accepted/seal/authority poisons; rollback/no-publication | typed identity refusal with exact rollback; no physics, equation, tolerance, budget, floor, event, topology, receipt meaning, exact carry, serialization, persistence, diagnostic, adaptive, or publication change |
| `INV-SNOWENERGY-075` | Version 51 narrows V41's signed-enthalpy monotonicity to the canonical phase transition it governs. All V41 exact static joins, promoted-root chaining, two-map cadence, constant bit-exact water, opposite vapor sides, finite/nonstagnant raw owner, no-A/B/A, private posture, and shared-budget reserve remain mandatory. Across the complete validated observed-lane set there must be exactly one adjacent canonical phase-predicate crossing; the crossing direction must agree with every pre-crossing enthalpy step and predicates may never reverse, skip, or recross. Lanes that do not contain that unique crossing remain on one unchanged predicate and retain the same exact signed pre-crossing direction. After the crossing, enthalpy may reverse only on the crossing lane while its canonical predicate remains unchanged, every exact absolute step magnitude is finite, positive, and strictly smaller than the preceding magnitude, and successive within-phase correction directions alternate after the first correction. No tolerance or bit-distance comparison is introduced. Only eligibility to dispatch the unchanged physical solver is granted; authentic residual closure, receipt stabilization, same-input replay, finalization, custody, rollback, event correctness, shared maximum 96, and exact 60-second floor remain unchanged. | `REF-SNOWENERGY-WGHL-V51`, `INV-SNOWENERGY-057/064/065`, unchanged physical coupled solver and authentic admission | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r130/r132 exact `H`/predicate/magnitude chain; pre-cross reversal, nonalternating/noncontracting/equal magnitude, stagnation/nonfinite, predicate reversal/skip/recross, zero/multiple crossing, water/static/join/cadence/side/raw-owner/A-B-A/budget/publication poisons; unchanged solver/replay/finalization | typed adaptive refinement or nonconvergence with exact rollback; no narrower support, tolerance, equation, physics, budget, receipt, closure, event, topology, custody, persistence, diagnostic, acceptance, or publication change |
| `INV-SNOWENERGY-076` | Version 52 extends every affected lane coordinate from ordered `(W_1,H_1,rho_1)` to `(W_1,H_1,rho_1,Q_cn,1)`, where `Q_cn,1` is the complete-support snow-candidate Crank--Nicolson heat in `J m^-2 OFE-ground`, positive into snow. A `PrivateTrial` charged map must feed that exact coordinate once through the existing typed unpublished CN operand as snow `+Q` and soil `-Q`, never through a fabricated sealed receipt. A `ReceiptStabilizationProbe` or `ReceiptStabilizationReplay` must instead consume its supplied sealed receipt Q unchanged; the coordinate may not overwrite or reseal that authentic input. Every posture reconstructs `Q_cn,physical` from its same physical Stage 3/soil endpoint and evaluates `R_Q=Q_cn,1-Q_cn,physical`; it enters the canonical residual vector and merit with the unchanged existing lane energy tolerance. Coordinate order is all lane `W/H/rho/Q`, then ordered soil `E/T`. Initial `Q` is the unchanged canonical CN formula. Every physical/Jacobian/trust/rejected/polish/probe/replay map charges the same maximum-96 budget and retains exact branch/static receipt-geometry/custody joins. After the private root, V35 sealing, exact immutable receipt iteration, exact stabilized same-input replay, `CoupledAuthentic`, and authentic finalization remain mandatory. Digest distance, receipt-difference residuals, `F(x)-x`, averaging, interpolation, canonical repair, uncharged maps, tolerance/cap/floor change, or private publication is forbidden. | `REF-SNOWENERGY-WGHL-V52`, `INV-SNOWENERGY-048/049/050/057/059/060/061/069/070`, unchanged CN physics and lane energy tolerance | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r133/r134 exact A/B/A receipt heat cycle; authentic-shaped `Q` root; zero-heat parity; multi-lane order/cardinality; omission/sign/one-bit/static-geometry/branch/budget/replay poisons; rollback and no-publication | typed refinement/nonconvergence with exact rollback; no constitutive physics, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, persistence, diagnostic, acceptance, or publication change |
| `INV-SNOWENERGY-077` | Version 53 requires the initial Q coordinate for every affected lane to be the snow-candidate CN heat reconstructed from the exact already-produced endpoint Stage 3 and endpoint soil candidates associated with that solver dispatch. The ordered endpoint receipt reconstruction is representational and uncharged: it invokes the unchanged canonical CN receipt formula but no Stage 3, soil, carrier, LSE, or other physical map. Both a fresh active-set seed and augmentation of a retained legacy `(W,H,rho,E,T)` seed must use that endpoint Q; an accepted, retained, predecessor, or otherwise cross-map receipt may not supply it. V53 does not alter how any non-Q coordinate is selected. Exact lane order/cardinality/support/topology/node/custody/seal and finite heat remain mandatory. After seed assembly every V52 equation, coordinate order, physical map, residual, tolerance, complete-step preflight, maximum-96 budget, exact receipt stabilization/replay/finalization, rollback, and no-publication rule is unchanged. | `REF-SNOWENERGY-WGHL-V53`, `INV-SNOWENERGY-048/049/050/059/069/070/076`, unchanged canonical receipt reconstruction | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r135/r136 cross-map-Q budget failure; exact endpoint candidate reconstruction; fresh/legacy dispatch parity; cross-map substitution, lane order/cardinality, nonfinite, extra-map/charge, rollback, and no-publication poisons | typed refinement/nonconvergence with exact rollback; no equation, non-Q seed, tolerance, cap, floor, event, topology, custody, receipt meaning, physics, persistence, diagnostic, acceptance, or publication change |
| `INV-SNOWENERGY-078` | Version 54 narrowly supersedes V35's immediate oscillation refusal only after the unchanged authentic receipt iteration proves an exact finite cycle of one to three members. Retain only the private probe bundles needed to name that cycle. In deterministic first-seen cycle order, reconstruct each member's representable endpoint vector solely from its own already-produced authentic artifacts and its own reconstructed output receipt: ordered Stage 3 `W/H/rho`, that receipt's positive-into-snow Q, then ordered V2 top-soil exact high-plus-carry E rounded once and same-candidate T. No coordinate may be copied from another member. Atomically preflight one shared-budget charge for every observed member plus one protected replay. Pair each vector with its own sealed output receipt and execute one charged finalization-equivalent authentic full map. A witness is eligible only if unchanged full `R_W/R_H/R_rho/R_Q/R_E/R_T` merit, derived-z, finite/side, phase/density branch, support/topology/custody/static-receipt/exact-carry joins pass and the input receipt equals the reconstructed output bit-for-bit. Then the existing same-coordinate/same-input independent replay must reproduce residuals, artifacts, finalization inputs, branch, and receipt exactly before `CoupledAuthentic` admission. If no member is stable, cycle length exceeds three, preflight fails, or any projection/map/replay poison occurs, discard every bundle and fail with exact rollback. Q-only enumeration is forbidden because authentic maps consume sealed receipt Q and coordinate Q cannot alter their physical endpoint. Midpoint, averaging, interpolation, `nextafter`, arbitrary ULP search, receipt/digest distance, receipt repair/reseal, uncharged physics, tolerance/cap/floor change, or private publication is forbidden. | `REF-SNOWENERGY-WGHL-V54`, `INV-SNOWENERGY-048/049/050/059/062/069/070/076/077`, unchanged exact receipt and physical closure authority | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r138 finite cycle; exact first-seen chronology; own-artifact W/H/rho/Q/E/T projection; Q-invariance; complete cycle-plus-replay budget; exact witness and no-solution vectors; mixing/order/nonfinite/residual/z/branch/carry/receipt/digest/replay poisons; rollback and no-publication | typed refinement/nonconvergence with exact rollback; no equation, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, physics, persistence, diagnostic, acceptance, or publication change |
| `INV-SNOWENERGY-079` | Version 55 acts only on the first complete, finite, side-valid, branch-bound private physical root that already satisfies every unchanged V52 full residual and derived-thickness tolerance, and before V45 finite-difference polishing spends another physical-map charge. Valid Q vector shape, finite coordinates/physical outputs, and exact canonical endpoint-receipt lineage are hard requirements: source the same charged map's own physical `Q_out` directly from the retained canonical reconstructed endpoint receipt and require `Q_coordinate - Q_out` to reproduce `R_Q` bit-for-bit. Residual subtraction alone may not source the endpoint. V55 is an optional specialization only when exactly one ordered lane Q residual is unresolved, both interval endpoints are positive and in the same binary64 domain, checked root-exclusive/own-output-inclusive cardinality succeeds, and the complete interval plus protected authentic probe and independent replay fits the remaining shared maximum-96 budget. Any miss of those specialization predicates returns `NotApplicable` before a callback or charge, leaves budget and root bundle unchanged, and continues the pre-existing V45 polishing path. After successful atomic preflight, the attempt commits immediately before its first candidate charge and may never fall back. The interval is traversed completely in deterministic root-to-output bit order with all W/H/rho, every other lane Q, all soil E/T, support, topology, custody, static receipt geometry, exact carry, phase, and density branches bit-identical. Every member receives exactly one charged `PrivateTrial` full map. Retain the first exact positive-zero `R_Q`/own-output-Q/full-residual/z/side/branch/custody witness privately but evaluate every preflighted member. No witness or any post-commit evaluation, charge, coordinate, closure, authentic receipt, replay, or finalization error fails typed with exact rollback and no fallback. The unchanged exact whole-receipt iteration, independent replay, and strict downstream finalization remain mandatory. Authentic probes/replay consume their supplied sealed receipt Q unchanged. Averaging, midpoint, interpolation, `nextafter`, sparse search, receipt/digest distance, receipt mutation/repair/reseal, uncharged maps, tolerance/cap/floor change, or private publication is forbidden. | `REF-SNOWENERGY-WGHL-V55`, `INV-SNOWENERGY-048/049/050/059/060/062/069/070/076/077/078`, unchanged V45 polishing, V52 residuals, and V35 authentic admission | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r139/r140 two-member miss and r142 1394-member overcapacity; hard shape/finiteness/lineage; zero-charge unresolved-count/domain/cardinality/capacity misses; behavior-identical V45 continuation; ascending/descending exact-fit attempt; atomic commit; exact-once traversal; post-charge/no-witness/residual/z/branch/side/custody poisons; authentic stabilization/replay/finalization; rollback and no-publication | typed hard refusal or unchanged V45 continuation with exact rollback; no equation, tolerance, cap, floor, event, topology, custody, receipt meaning, exact carry, physics, persistence, diagnostic, acceptance, or publication change |
| `INV-SNOWENERGY-080` | Version 56 dispatches before V55 only for an authentic terminal-one-volume Stage 3 support whose complete observed branch is strictly frozen and noncrossing: finite positive water and density, finite `0<T_s<273.15 K`, no liquid, melt, refreeze, terminal-unallocated energy, phase-predicate transition, density/settling-branch change, layer/order change, event, topology, custody, support, or static-receipt change. Its ordered private coordinates are lane `(W,T_s,rho)` followed by top-soil `(E_soil,T_soil)`; neither snow H nor CN Q is an independent coordinate. Every charged map derives `H_exact=-exact(W)*exact(c_ice)*(exact(273.15)-exact(T_s))`, rounds `H_hi` once by IEEE-754 round-nearest-even, retains exact dyadic `R_Hcarry=H_exact-exact(H_hi)`, and evaluates unchanged energy closure from the exact sum. It derives Q once through unchanged CN physics, applies snow `+Q` and soil `-Q` once, and charges the same maximum-96 budget for every physical/Jacobian/trust/rejected/probe/replay map. Only a fresh authentic result may form an `AuthenticatedCoveredSnowMaterialOwnerV1`: unchanged Stage 3 material plus ordered high/carry, sealed by a `CoveredSnowEnthalpyCarryReceiptV1` over schema/version, support, transaction/predecessor, lanes/order, exact beginning/ending high-plus-carry, ordered energy operands, base-owner digest, receipt chain, branch, topology, and custody. The complete CN-plus-carry receipt set must stabilize exactly, then reproduce exact artifacts/receipts under one independent same-input replay and unchanged strict finalization before admission. A V54/V55 transient witness or private artifact may never become material/carry authority. `DirectSnowStage3V11SnowEnthalpyRestartV5` preserves the compound owner and carry-receipt chronology for committed, pending, and in-progress state including current support; V4 migration supplies only canonical exact zero carry, and V5 downgrade refuses unless every snow carry is exact zero. Restore revalidates the complete chronology. Ineligibility before charge may continue unchanged V55; any committed evaluation, owner, receipt, replay, finalization, migration, downgrade, or restore failure is typed with exact rollback and no publication. The 60-second floor, constitutive physics, `c_ice`, CN operation, physical ledger terms/tolerances, event chronology, topology, custody, and diagnostics are unchanged. | `REF-SNOWENERGY-WGHL-V56`, `INV-SNOWENERGY-048/049/050/059/062/069/070/076/079`, unchanged Stage 3/CN physics, exact-dyadic authority, and restart custody | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r144 exhaustive 21-candidate no-witness; strict frozen/noncrossing sides; exact formula/RN-even/tie/cancellation/high-plus-carry ledger vectors; Q exact-once; compound-owner and carry-receipt substitution/order/digest poisons; whole-receipt stabilization/replay/finalization; V4->V5 zero-carry migration; nonzero V5->V4 downgrade refusal; committed/pending/in-progress restore; rollback/no-publication/no-diagnostics | typed not-applicable before charge or fail-closed after commit; no equation, constitutive physics, tolerance, cap, floor, event, topology, custody weakening, receipt repair, transient-witness promotion, or diagnostic/publication change |
| `INV-SNOWENERGY-081` | Version 57 classifies only a finite nonnegative `external_liquid_kg_m2 <= 1.0e-12 kg m^-2 OFE-ground` as neutral to the V56 strictly-frozen eligibility predicate. Equality is eligible. This predicate must not change the operand bits: the exact binary64 amount remains in `physical_delta_water_kg_m2`, every ordered energy/advection operand, receipt, replay, finalization, and independent mass/energy reconstruction. Any corresponding refreeze is eligible only when it is the same bounded external operand completely routed into existing frozen material with exact-zero ending liquid, no melt or terminal event, and unchanged phase predicate; its exact mass and latent energy remain ledgered. A negative, nonfinite, or one-bit-greater amount, unmatched refreeze, ending liquid, melt, or event is ineligible before charge. V57 may also perform the same zero-charge eligibility and coordinate conversion on the first tolerance-closed legacy root immediately before V55; it retains the shared budget already used, derives `(W,T_s,rho,E_soil,T_soil)` without a physical map, and commits before its first charged V56 evaluation. Zero-charge ineligibility continues unchanged V55; after the first V57/V56 charge, any error is typed and no fallback is permitted. The `1.0e-12 kg m^-2` eligibility bound is the unchanged minimum terminal physical snow-closure scale, one millionth of the unchanged `1.0e-6 kg m^-2` covered mass tolerance, and one thousandth of the `1.0e-9 kg m^-2` represented-layer lifecycle boundary. No external liquid or refreeze is zeroed, clamped, dropped, rerouted, repaired, or excluded from closure. Constitutive physics, physical ledger tolerances, exact high-plus-carry closure, bulk phase/event chronology, maximum 96, exact 60-second floor, topology, custody, receipts, rollback, persistence, diagnostics, and final admission are unchanged. | `REF-SNOWENERGY-WGHL-V57`, `INV-SNOWENERGY-027/048/049/050/059/062/069/070/080`, unchanged Stage 3 liquid and ledger authority | `[DIRECT][Static] + [DIRECT][Ran] + [INFERENCE][Static]` | retained r147 exact residue and budget; zero/equality/one-bit-above/negative/nonfinite eligibility; exact bounded refreeze/no-ending-liquid identity; bit-exact operand retention in independent mass/energy closure; zero-charge post-root transition at retained budget; V55 callback zero after commit; rollback/no-publication/no-diagnostics | typed not-applicable before charge or fail-closed after commit; no operand normalization, equation, physical tolerance, cap, floor, bulk phase/event, topology, custody, receipt, persistence, diagnostic, acceptance, or publication change |

For `INV-SNOWENERGY-057`, the exact transition detector is a rolling
three-transition window. If `interface -> branch entry -> interface reset`
fails any exact reset join, coordinate, phase predicate, or opposite-side
predicate, it must not dispatch: the current already-validated interface
becomes the next root anchor and prior branch-entry state is cleared. Only a
later complete window whose reset exactly equals that promoted root may
dispatch; a stale first root may never remain latched across later windows.
This changes no reset equality, tolerance, physical residual, budget,
acceptance, replay, finalization, receipt, rollback, or publication rule.

The following corrective definition has normative precedence over any shorthand
use of "exact authentic A/B/A" in `INV-SNOWENERGY-057`: exactness attaches to
the support/source/event/topology/custody/receipt joins, transition order,
root/interface/reset coordinates, branch predicates, and pure-vapor sides. It
does not require the first and later raw-authentic continuous owner images to
be bitwise equal. The sole admitted detector sequence is
`root/interface -> branch-entry -> opposite raw-authentic -> same
root/interface/reset`, with opposite pure vapor disposition and all named joins
unchanged.

For one sealed `CoveredPhaseConsistentResidualInputsV1`,
`covered_phase_consistent_residual_evaluate_v1` performs one complete physical
Stage-3/LSE/soil evaluation and returns exactly one
`CoveredPhaseConsistentResidualEvaluationV1`:

```text
R_W,l = W_1,l - W_0,l - DeltaW_physical,l
R_H,l = H_1,l - H_0,l - Q_complete,l
R_E,n = E_soil,1,n - E_soil,0,n - DeltaE_CN+other,n
R_T,n = T_soil,1,n - T_soil_owner(E_soil,1,n, sealed soil state)
```

`DeltaW_physical` is the unchanged exact ordered snow-water ledger.
`Q_complete` is the unchanged complete ordered snow-energy ledger, with
`Q_v=V L_s` and the snow debit of the existing snow--soil CN receipt exactly
once. `DeltaE_CN+other` is the existing soil owner's ordered CN storage/flux
equation, including the equal/opposite receipt credit exactly once.
`T_soil_owner` is the existing soil enthalpy--temperature relation, not a new
closure or fit. Canonical `Pi(W,H)`, physical-domain and capacity validity,
existing covered-LSE balance, CN endpoint/conductance equality, snow/soil
equal-opposite custody, exact receipt reconstruction/reseal, and every identity
join are algebraic side constraints. A trial that fails any side constraint has
no residual merit and is refused. Constructing residuals as coordinate-map
differences `F(x)-x` is forbidden because it neither exposes these physical
equations nor proves their ledgers and receipts.

One `CoveredPhysicalEvaluationBudgetV1` is created for the entire covered solve
with the unchanged maximum 96. Each complete physical evaluator call requested
for trigger confirmation, baseline residual, generalized-Jacobian direction,
trust-region trial, rejected trial, fresh root evaluation, or final replay is
charged exactly once before evaluation. No nested counter, solve restart, or
finalization path may reset or exceed it. A successful fresh/replayed root may
enter `CoveredConvergenceAdmissionV1::CoupledAuthentic`, which bypasses only the
ordinary Picard current/candidate equality and convergence decision. It does
not bypass residual tolerances, algebraic constraints, authentic finalization,
receipt reseal, event/topology/custody/identity checks, rollback, or atomic
publication.

## Producer and Consumer Obligations

| Obligation ID | Role | Requirement |
|---|---|---|
| `OBL-SNOWENERGY-P-001` | climate producer | Publish hourly `T_a` plus daily `e_a` and `R_s` with declared units, cadence, and finite-domain validation. |
| `OBL-SNOWENERGY-P-002` | solar-geometry producer | Publish `R_a` and explicit daylight/polar-night classification. |
| `OBL-SNOWENERGY-P-003` | canopy producer | Publish one effective daily plan-view canopy cover `C`; preserve its leaf-on/leaf-off and structural-floor semantics. |
| `OBL-SNOWENERGY-P-004` | Stage 3 thermal producer | Above `m_res`, publish active-layer `T_s`, mass, depth, cold content, lower state when present, and explicitly identified `T_c=T_a`, or a typed unavailable result. At or below `m_res`, publish unresolved duration/mass without fabricating thermal state. |
| `OBL-SNOWENERGY-P-005` | sublimation exchange | Publish one bounded signed vapor mass exchange and derive its latent heat using the same `T_s`. |
| `OBL-SNOWENERGY-P-006` | complete energy producer | Before melt-owner cutover, publish finite, unit-explicit, same-substep net radiation, sensible heat, latent heat, ground/interlayer conduction, and precipitation-advected heat with exact-one composition and independently reconstructable lineage. |
| `OBL-SNOWENERGY-P-007` | evaluation evidence producer | Preserve raw vapor/latent opportunity separately from bounded deposition/sublimation and state endpoints; use N/A for S/F actual transfer; do not relabel raw opportunity as actual snow loss. |
| `OBL-SNOWENERGY-P-008` | terminal event producer | For enabled persistent evaluation only, publish request/version, start/end enthalpy state, accepted/rejected trials, error norm, bracket bounds, event/evaluated/unevaluated seconds, complete carrier, `Delta H_cc`, refreeze, deposition, sublimation, melt, endpoint-solid identity, terminal liquid/energy handoffs, and scale-aware closure without naming a receiving surface. |
| `OBL-SNOWENERGY-C-001` | longwave evaluator | Apply the equations and guards in the specified order without silent unit conversion or fallback. |
| `OBL-SNOWENERGY-C-002` | shared energy carrier | Consume `L_net` exactly once with the positive-toward-snow convention. |
| `OBL-SNOWENERGY-C-003` | sublimation/melt consumers | Use the same EB-03 snow state as longwave; do not reconstruct an independent surface temperature. |
| `OBL-SNOWENERGY-C-004` | diagnostics | Preserve component operands sufficient to reconstruct `L_atm`, `f_sky`, `L_sub`, `L_out`, and `L_net`. |
| `OBL-SNOWENERGY-C-005` | configuration/reporting | Identify when `T_c=T_a` is used and communicate its approximation limits. |
| `OBL-SNOWENERGY-C-006` | runtime implementation package | Prove the real common `B/L/S/LS` Stage 3 consumer reads this path before claiming activation. |
| `OBL-SNOWENERGY-C-007` | snow state | Remove sublimated mass from layer and aggregate ice state only, publish cold-content export, and reject double-selector composition. |
| `OBL-SNOWENERGY-C-008` | thermal partition | Reconstruct the Marks/SNOBAL active volume independently of depositional boundaries and conserve projection operands. |
| `OBL-SNOWENERGY-C-009` | coupled solver | Apply equal-and-opposite `G_0` within each selected stability substep and close active/lower and whole-pack energy. |
| `OBL-SNOWENERGY-C-010` | stability scheduler | Select `60/15/1 minute` substeps from the fixed mass thresholds and reevaluate fluxes from current substep state. |
| `OBL-SNOWENERGY-C-011` | conductive provider | Publish current active/lower state, applied `G_0`, cadence, and separate active/lower/cancellation residuals from the production solve. |
| `OBL-SNOWENERGY-C-012` | density-layer handoff | Convert layer SWE to `kg m^-2` before applying the density model's zero-mass lifecycle boundary; preserve all coupled state for retained layers and keep aggregate residual tolerances independent. |
| `OBL-SNOWENERGY-C-013` | melt-owner implementation and cutover | Atomically replace CoE generation with bounded Stage 3 positive-energy phase conversion; close same-substep refreeze/retention/routing, residual-snow and terminal-unallocated-energy authority, selectors/defaults/rollback, real-consumer use, and independent energy plus linked mass ledgers before claiming conformance. |
| `OBL-SNOWENERGY-C-014` | evaluation evidence consumer | Reconstruct raw vapor from turbulent primitives; verify `Q_latent_raw=m_v,raw L_s(T_s)`; derive bounded transfer and `Q_latent_bounded=m_v L_s(T_s)`; report their difference; reconstruct the exact characterization-only raw-latent cold/melt chronology; and reject producer disagreement plus all vapor/melt/liquid/N/A aliases before reduction. Never claim the as-built raw-latent chronology conforms to the future bounded-latent target. |
| `OBL-SNOWENERGY-C-015` | terminal event evidence consumer | Independently reconstruct endpoint solid, terminal liquid, enthalpy/energy, event support, and bracket/error acceptance from schema-v8 primitives. Reject full-step melt/sublimation, omitted deposition/refreeze, post-event snow flux, poisoned producer residuals, request/state mismatch, terminal recipient claims, and any event outside the terminal domain. |
| `OBL-SNOWENERGY-P-010` | shared carrier producer | Emit one carrier candidate with complete operand lineage, residuals, current-trial temperatures, and owner/support identities. |
| `OBL-SNOWENERGY-C-017` | shared carrier consumer | Independently reconstruct snow, vapor, liquid, energy, longwave, and event-time closure and reject any alias or duplicate flux. |
| `OBL-SNOWENERGY-C-018` | OFE-ground lane-boundary consumer | Independently reconstruct the complete ordered typed tile contribution set, all retained source-receipt-set identities, common Stage 3 state/temperature/latent heat, OFE-ground flux sums, terminal-liquid handoff, and topology/basis identity; reject omission, duplication, class/model substitution, covered-subset normalization, or restart topology substitution. |
| `OBL-SNOWENERGY-C-019` | covered fixed-point consumer | Reconstruct each candidate fingerprint, compare structural and count-like state exactly, and apply only the physical-class absolute convergence bounds admitted by `TOL-SNOWENERGY-003`; reject stale fingerprints and nonconvergence without state repair. |
| `OBL-SNOWENERGY-C-023` | covered fixed-point consumer | At the exact floor only, retain the raw authentic below-domain terminal-one-volume image, reconstruct the single phase-aware midpoint admitted by `INV-SNOWENERGY-055` from the common immutable beginning and complete support operands, and use it only as the next unpublished iterate. Refuse vapor-sign/disposition changes, incomplete or nonclosing operands, structural or receipt substitutions, nonfinite values, failed canonical projection, or any route by which the midpoint could enter finalization, replay, acceptance, or publication. |
| `OBL-SNOWENERGY-C-024` | covered fixed-point consumer | On any exact covered support at or above the unchanged 60-second floor, route only pure strictly opposite-sign independently closed actual-vapor images through `INV-SNOWENERGY-056`; localize the zero interface without affine latent interpolation, preserve authentic specific latent heat on zero-to-one-sided entry, reconstruct complete `W/H` and every exact join, retain raw authentic history, and refuse every route by which a synthetic image could converge, finalize, replay, accept, persist, publish, or mutate rollback state. Same-sign images retain the version-31 route. |
| `OBL-SNOWENERGY-C-020` | additive-restart consumer | Refuse V1 lane receipts and any inferred successor wire. Restore only after a normative V2 schema and vectors exist and the restored receipt is rejoined to static topology, destination/lane/component receipts, installed LSE and snow owners, and wet-liquid authorization. |
| `OBL-SNOWENERGY-P-011` | precipitation parcel-set producer | Seal the complete ordered phase-parcel set, including a present empty set, only after joining atmospheric phase custody, vegetation terminal-liquid custody, support, topology, and producer beginning-state identities. |
| `OBL-SNOWENERGY-C-021` | Stage 3 precipitation consumer | Independently validate destination exclusivity and reconstruct OFE-ground precipitation mass and precipitation-advected heat from the same exact parcel set before accepting a physical candidate. |
| `OBL-SNOWENERGY-P-012` | joined snow/soil boundary producer | Build one lane-level lower-boundary receipt from immutable Stage 3 and OFE soil-thermal beginnings plus the accepted coupled candidate; publish neither candidate independently. |
| `OBL-SNOWENERGY-C-022` | snow/soil boundary consumer | Independently reconstruct half-layer series conductance, both endpoint fluxes, Crank--Nicolson accepted heat, exact snow debit/soil credit, receipt digest, and candidate-ending joins before the complete-owner commit. |

| `OBL-SNOWENERGY-C-025` | covered fixed-point consumer | Detect only the exact active-set transition reset `root/interface -> branch-entry -> opposite raw-authentic -> same root/interface/reset`, with exact root coordinates/branch predicates, opposite pure vapor disposition, and unchanged support/source/event/topology/custody/receipt joins; never require bitwise equality of asymptotically changing raw-authentic continuous owner fields. Invoke `PhaseConsistentCoupledSolveV1`/`phase_consistent_coupled_solve_v1` only through sealed `CoveredPhaseConsistentResidualInputsV1` and `covered_phase_consistent_residual_evaluate_v1`. Require concrete `R_W/R_H/R_E/R_T`, canonical phase, existing LSE/CN equations, exact water/complete energy including linked vapor latent heat and equal/opposite snow--soil receipt, soil enthalpy--temperature closure, and exact receipt/identity side constraints at every physical evaluation; reject `F(x)-x` as a residual substitute. Charge trigger, residual, Jacobian, trust, rejection, fresh, and final replay physical evaluations to one `CoveredPhysicalEvaluationBudgetV1` bounded by the unchanged 96 total. Finalize only through fresh evaluation/replay/reseal and private `CoveredConvergenceAdmissionV1::CoupledAuthentic`, which bypasses only Picard equality/convergence and retains every residual tolerance, constraint, finalization, reseal, event, identity, rollback, and publication guard. Qualification must bind the retained 60/120-second reset capture, cold/mixed-phase/fusion known roots, root distinctness from v31/v32 affine states, full refusal vectors, and canonical one-day accepted/rejected counts, width distribution, evaluator/budget counts, wall time, limiting rejection reasons, maximum ledger residuals, and absence of repeated 96-evaluation ceilings. The target remains material reduction from about 1,435 accepted and 1,500 rejected trials, provisionally at least fourfold in each with at least 900-second accepted-time fraction and stable maximum width; these are performance gates, not new numerical tolerances. No microstepping diagnostic may persist in production. |
| `OBL-SNOWENERGY-C-026` | covered fixed-point consumer | Implement `CoveredStableMonotoneSolveEligibilityV1` and `covered_stable_monotone_solve_eligibility_v1` over exactly eight consecutive raw authentic maps. Require one event-free terminal-one-volume covered support at or above the exact 60-second floor; exact unchanged support/source/event/topology/custody/static-receipt/phase-branch/carry-authority-and-representation joins; exact `E=exact(H_hi)+R` reconstruction for each map while physical receipt digests and `H_hi/R` coordinate values evolve; finite concrete `R_W/R_H/R_E/R_T`; strictly decreasing governed residual merit; no `A/B/A`, active-set transition, or finalization restart; and one already-charged `CoveredPhysicalEvaluationBudgetV1` for all prior Picard and subsequent solver/fresh/replay evaluations. Invoke only the unchanged equation-level v33 solver. Never use evolving-coordinate byte inequality to relax exact receipt custody, signed-dyadic carry representation, ordered reconstruction, residual, closure, replay, or reseal. Before a root, discard every private trial and resume raw authentic Picard within the remaining budget on any refusal; never reset the budget or accept a private trial. Retain fresh `CoupledAuthentic` replay/reseal plus every tolerance, ledger, phase, event, topology, custody, receipt, rollback, finalization, and publication guard. Qualification must report canonical one-day accepted/rejected counts, step-width distribution, runtime, limiting rejection reasons, maximum ledger closure, solver/budget counts, repeated-96 outcomes, and prove that no microstepping diagnostic persists in production. |
| `OBL-SNOWENERGY-C-027` | covered fixed-point consumer | Implement `CoveredAuthenticReceiptStabilizationV1`, `covered_authentic_receipt_stabilization_probe_v1`, and `covered_authentic_receipt_stabilization_replay_v1`. After a coupled root, treat each immutable canonical receipt set `R_n` as the next authentic physical input, reconstruct/reseal `R_(n+1)` from immutable beginnings and sealed sources, and charge every probe to the same unchanged shared 96-evaluation budget. Do not tolerance-compare, repair a digest, mutate a receipt in place, retain a probe artifact, or exact-compare artifacts produced under different receipt inputs; the first root reseal is only a probe. Continue until canonical input receipts exactly equal reconstructed output receipts. Then perform one independent physical replay with that same stabilized receipt input and require exact finite residual, complete candidate-artifact, and reconstructed-receipt equality before `CoupledAuthentic` or finalization. Refuse exact receipt oscillation, nonfinite values, side-constraint/reconstruction failure, budget exhaustion, replay disagreement, or private/probe artifact retention with exact rollback. Preserve every equation, tolerance, floor, event, topology, custody, receipt meaning, exact carry, rollback, finalization, publication, and no-persisted-diagnostic guard. |
| `OBL-SNOWENERGY-C-028` | covered fixed-point consumer | Implement `CoveredTerminalDensityGeometryCoordinateV1` and `covered_terminal_density_geometry_residual_evaluate_v1`. Add exactly one `rho_1,l` coordinate per affected terminal-one-volume lane, derive `I_1,l` only from canonical `Pi(W,H)`, reconstruct `z_1,l=I_1,l/rho_1,l`, and return physical `R_rho,l` from the unchanged Stage 3 density/settling constitutive map using immutable beginning, sealed forcing, and current physical operands. Solve `R_W/R_H/R_rho/R_E/R_T`; preserve exact layer/order, settling authority, and density-model branch while allowing finite positive rho/z to evolve. Charge all geometry physics to the same unchanged shared 96 budget and retain v35 receipt stabilization/authentic finalization. Refuse generic `F(x)-x`, interpolation/repair, tolerance/cap/floor change, uncharged physics, branch/geometry poison, private acceptance, or receipt/finalization bypass with exact rollback. |
| `OBL-SNOWENERGY-C-029` | covered fixed-point consumer | Implement `CoveredDerivedThicknessClosureV1` and `covered_derived_thickness_closure_evaluate_v1`. Reconstruct proposed and physical thickness only from canonical `I/rho`, expose physical `R_z`, and require its unchanged `depth_abs_m` closure in scaled merit and root admission while retaining exactly the `W/H/rho/E/T` unknowns and solved `R_W/R_H/R_rho/R_E/R_T` residual vector. Derive `R_z` from every already-charged unchanged physical evaluation, including fresh, v35 receipt probes, and replay, without an extra map. Retain exact receipt stabilization, same-input replay, `CoupledAuthentic`, and authentic finalization. Refuse omitted depth merit, independent-z coordinates, interpolation/copy/repair, tolerance/cap/floor change, uncharged physics, private acceptance, receipt/replay/finalization bypass, or rollback mutation. |
| `OBL-SNOWENERGY-C-030` | covered fixed-point consumer | Implement `covered_phase_consistent_finalization_equivalent_map_v1` as the sole physical map for private, Jacobian, trust, rejected, receipt-probe, and same-input replay evaluations. Reconstruct proposed terminal snow and exact-carry soil operands from immutable beginnings; build the same non-provisional endpoint carrier, LSE exchange, open/covered boundary merge, precipitation set, and immutable CN receipt input as authentic finalization; and call Stage 3 exactly once per shared-budget charge. Derive every residual/artifact/reconstructed receipt from that one image. Forbid provisional-map closure/admission, outer-iterate substitution, duplicate or uncharged Stage 3 calls, repair/copy, private publication, and replay/finalization bypass. Retain exact v35 receipt stabilization, one independent same-input replay, and one independent authentic-finalization reevaluation that must reproduce exact state/LSE/boundary/receipt/ledger images. |
| `OBL-SNOWENERGY-C-031` | V2 soil-energy operand, unpublished-continuation, and private-support consumer | Construct `PhysicalSoilEnergyTransactionAuthorityV2` from the already-authenticated outer source transaction and exact prepared/unpublished soil-target transaction. Validate ingress and its receipts only against the source transaction; validate V2 target support/owner lineage only against the soil transaction; bind both transactions into every soil-internal and infiltration operand digest; and pass the unchanged physical energy values and source-owner/receipt lineage into the exact soil-target acceptance path. Construct every initial unpublished trial, unpublished physical beginning, same-support continuation, and finalization-equivalent coordinate projection from an independently validated prepared soil owner, use that prepared owner's exact transaction and predecessor chain, and never feed the outer source transaction into these soil-only joins. Prepare initial and finalization-equivalent exact child supports only through the resident's authenticated next-support custody chain. Refuse zero, stale, foreign, swapped, missing, merely adjacent, out-of-order, support, owner, receipt, prepared-beginning, continuation, projection, or digest substitution before acceptance with exact rollback and no private publication. |
| `OBL-SNOWENERGY-C-032` | covered fixed-point parity-monotone trigger consumer | Observe exactly four consecutive nonexact rolling active-set resets with exact static joins, exact promoted-root chaining, fixed phase/opposite-vapor predicates, finite positive strictly decreasing tolerance-independent root drift, nonstagnant/non-`A/B/A` parity owner coordinates, exact two-map ordinal cadence, and enough unused capacity in the unchanged shared 96-evaluation budget for the minimum initial/Jacobian/trust/probe/replay path. Dispatch only the unchanged equation-level solver from the current authentic/interface seed. Refuse exact-reset routing, nonfinite/stagnant/reversed drift, parity repetition, join/support/predicate/side/cadence/budget mutation, solver failure, or private publication; retain all V35--V39 receipt, finalization, rollback, and authentic-only acceptance guards. |
| `OBL-SNOWENERGY-C-033` | covered fixed-point one-way phase-boundary trigger consumer | After the same four exact-static rolling windows refuse V40, require bit-exact constant finite nonnegative `W`, a strictly one-way finite five-point `H` chain, recorded predicates independently equal to canonical phase predicates, exactly one adjacent one-branch boundary crossing across the observed affected lanes, exact promoted-root/static-join/opposite-side/two-map cadence, private posture, and the unchanged shared-budget reserve. Refuse water drift, nonfinite/stagnant/reversed enthalpy, no/multiple/direct-jump crossing, join/side/chain/cadence/budget mutation, solver failure, or publication. Dispatch only the unchanged physical solver from the current authentic/interface seed; retain every V35--V40 receipt, replay, finalization, rollback, and authentic-only admission guard. |
| `OBL-SNOWENERGY-C-034` | covered fixed-point cold-content-export coordinate consumer | Carry exact finite nonnegative authentic `X_c` in every support image; reconstruct `H=-C_0+L_f(L_0+L_in)+Q+X_c`; contract `X_c` in endpoint order with the same single private weight as external liquid and ordered nonlatent energy; independently require authentic endpoint W/H closure before use; preserve zero-export byte identity, raw authentic history, shared budget, receipts, rollback, authentic-only finalization, and no publication; refuse omission, nonfinite/negative/order/weight/substitution/closure poisons. |
| `OBL-SNOWENERGY-C-035` | V2 private numerical-coordinate fixed-point consumer | Preserve a typed numerical-coordinate posture through the carrier; validate its exact sealed prepared-owner, transaction/predecessor/support/beginning-state, receipt-chain, numerical authority/set, topology/order, top-layer high/zero-carry/temperature, unchanged lower-layer, and empty-credit custody; allow it only as the current same-support private numerical image. Retain ordinary base reconstruction unchanged and refuse posture erasure/mixing, substitution, sequential continuation, acceptance, installation, publication, or owner mutation. |
| `OBL-SNOWENERGY-C-036` | covered coupled-map LSE closure-posture consumer | Select evaluation posture solely from the sealed coupled evaluation kind. A `PrivateTrial` may defer aggregate weighted-OFE validation only through the existing uncommitted provisional LSE path needed to rebuild reciprocal-longwave/shortwave/sensible/vapor exchange; it remains charged, private, non-publishable, and incapable of authentic admission. Receipt-stabilization probes and the one independent same-input replay must use strict non-provisional LSE posture and pass the unchanged weighted-OFE decomposition and tolerance before `CoupledAuthentic`; independent finalization must repeat strict closure and exact replay. Consume projected top-soil `E/T` exactly once in the snow--soil CN receipt/credit path and never as Stage3-covered V8 soil storage/ground heat. Refuse posture substitution, deferred-closure admission, strict-probe bypass, coordinate omission/double use, extra Stage 3 calls, budget reset, owner mutation, or private publication. |
| `OBL-SNOWENERGY-C-037` | covered coupled-root polishing and authentic receipt consumer | Implement `CoveredCoupledPolishedRootV1`, `phase_consistent_coupled_root_polish_v1`, and the purpose-specific shared-budget reserve guard. Carry the exact complete physical bundle for the best private root so rejected Jacobian/trust artifacts cannot substitute for it. Continue only the existing physical generalized-Jacobian/Newton/trust calculation while the existing scaled merit decreases strictly; permit exact-zero or finite side-valid sub-tolerance stagnation/non-descent to stop polishing only as a private seed for unchanged V35. Preserve three later charges before every ordinary private-solver map, two before every polishing map, and the final replay charge before every nonstable authentic probe; all maps still charge the same budget once and the maximum remains 96. Require exact immutable `R_n -> R_(n+1)` chaining, exact input/output receipt equality, and one independent same-input exact residual/artifact/receipt replay before `CoupledAuthentic`. Refuse above-tolerance stagnation/non-descent, singular/nonfinite/side/branch mutation, stale rejected artifacts, separate/reset budget, reserved-slot theft, receipt repair/interpolation/digest distance, oscillation, replay mismatch, rollback mutation, or private/probe publication. |

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract | Required interpretation |
|---|---|---|---|---|---|
| `T_a` | `air_temperature_k` | hourly climate to longwave | `K` -> `K` | `SC-CLIMATE-001` | Hourly above-canopy air temperature. |
| `e_a` | `vapor_pressure_kpa` | daily climate to hourly longwave | `kPa` -> `kPa` | `SC-CLIMATE-001` | Daily actual vapor pressure held across the day; not pascals or relative humidity. |
| `R_s` | `solar_radiation_mj_m2_day` | climate to cloud mapping | `MJ m^-2 d^-1` -> same | `SC-CLIMATE-001` | Incident daily shortwave. |
| `R_a` | `extraterrestrial_radiation_mj_m2_day` | solar geometry to cloud mapping | `MJ m^-2 d^-1` -> same | `SC-CLIMATE-001` / `SC-SNOWENERGY-001` | Same daily energy units as `R_s`. |
| `C` | `canopy_cover_fraction` | canopy to sky-view translation | `fraction` -> `fraction` | `SC-PLANT-001` | Effective overhead interception; not LAI or sky-view factor. |
| `f_sky` | `subcanopy_sky_view_fraction` | sky view to longwave mixture | `fraction` -> `fraction` | `SC-SNOWENERGY-001` | Derived diffuse transmission `(1-C)^1.6`; never alias directly to `1-C`. |
| `T_c` | `canopy_temperature_k` | thermal provider to canopy emission | `K` -> `K` | `SC-SNOWENERGY-001` | Effective radiating canopy temperature selected by EB-03. |
| `T_s`, `T_0` | `snow_surface_temperature_k` | active thermal provider to snow emission | `K` -> `K` | `SC-SNOWFREEZE-001` / `SC-SNOWENERGY-001` | Heat-capacity-weighted Marks/SNOBAL active-layer temperature. |
| `G_0` | `snow_active_lower_conduction_w_m2` | lower pack to active thermal volume | `W m^-2` -> same | `SC-SNOWENERGY-001` | Positive into active; equal negative lower operand. |
| `L_atm` | `atmospheric_longwave_w_m2` | atmosphere to sub-canopy mixture | `W m^-2` -> same | `SC-SNOWENERGY-001` | Hourly all-sky downward longwave above canopy. |
| `L_sub` | `subcanopy_longwave_w_m2` | mixture to snow energy | `W m^-2` -> same | `SC-SNOWENERGY-001` | Downward longwave incident at snow. |
| `L_net` | `net_longwave_w_m2` | longwave to shared energy carrier | `W m^-2` -> same | `SC-SNOWENERGY-001` | Positive toward snow. |
| `Q_cc` | `cold_content_j_m2` | Stage 3 layer to shared energy carrier | `J m^-2` -> same | `SC-SNOWFREEZE-001` | Positive energy deficit relative to `0 degC` ice. |
| `m_v,raw` | `vapor_mass_exchange_kg_m2` | evaluation turbulent opportunity | `kg m^-2` -> same | `SC-SNOWENERGY-001` | Raw signed opportunity; never actual S/F transfer or bounded snow loss. |
| `m_v` | `deposition_kg_m2 - sublimation_kg_m2` | bounded Q transfer reconstructed from separate schema-v6 fields | `kg m^-2` -> same | `SC-SNOWFREEZE-001` / `SC-SNOWENERGY-001` | Actual signed bounded transfer; not the raw vapor field. |
| `alpha_v`, `Q_v`, `L_s,auth` | private version-32 numerical operands only; no boundary/API field | unpublished covered fixed-point active-set localization | `dimensionless`, `J m^-2`, `J kg^-1` | `SC-SNOWENERGY-001` | Never persisted, published, serialized, or exposed as diagnostics; `Q_v=V L_s` on a one-sided image and all four vapor terms are exact zero at the interface. |
| `Q_E` | `applied_surface_energy_j_m2` | shared carrier to cold content | `J m^-2` -> same | `SC-SNOWFREEZE-001` | Positive toward snow. |
| `Q_complete`, `Q_cold_required`, `Q_excess`, `delta_E_cold`, `Q_refreeze`, `Q_unallocated_after_exhaustion` | future typed Stage 3 energy-ledger fields | substep energy/phase closure | `J m^-2` -> same | `SC-SNOWENERGY-001` / `SC-SNOWFREEZE-001` | Exact named operands; `Q_refreeze=L_f m_refrozen` and none may be inferred from residual output. |
| `m_ice_available`, `m_melt`, `m_liquid_external_in`, `delta_m_retained`, `m_refrozen`, `m_routed`, `m_solid_precip`, `m_deposition`, `m_sublimation` | future typed Stage 3 phase/liquid-ledger fields | same-substep mass closure | `kg m^-2` -> same | `SC-SNOWENERGY-001` / `SC-SNOWFREEZE-001` | Exact named operands with retained change defined as end minus start. |
| `SWE_layer` | `snow.layer.mass_swe_m` | persistent snow layer to density lifecycle | `m` -> `m` | `SC-SNOWENERGY-001` | Typed vector element converted to `m_layer`; strict mass-unit lifecycle boundary. |
| `z_layer` | `snow.layer.thickness_m` | persistent snow layer to density closure | `m` -> `m` | `SC-SNOWENERGY-001` | Typed vector element; physical-depth aggregate only. |
| `T_ca`, `q_ca` | `SharedCanopyAirNodeV1.temperature_k`, `.specific_humidity` | shared carrier trial state | `K`, `kg kg^-1` | `SC-VEGETATIONTRANSACTION-001` | no independent canopy-air alias |
| `H_i`, `V_i` | typed carrier turbulent flux entries | surface/node exchange | `W m^-2`, `kg m^-2 s^-1` | Child 2C carrier | exact-once flux lineage |
| `L_can`, `L_snow<->canopy` | typed longwave ledger entries | component emission/reciprocal exchange | `W m^-2` | V11/LSE and Stage 3 | equal/opposite closure |

## Constants and Parameters

| Name | Value | Units | Status/provenance |
|---|---:|---|---|
| `sigma` | `5.670374419e-8` | `W m^-2 K^-4` | fixed SI Stefan-Boltzmann constant |
| Dilley intercept | `59.38` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley temperature coefficient | `113.7` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley temperature reference | `273.16` | `K` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley water coefficient | `96.96` | `W m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| Dilley water reference | `25` | `kg m^-2` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| vapor-pressure conversion | `4650` | `K kg m^-2 kPa^-1` | fixed by `REF-SNOWENERGY-FLERCHINGER` |
| clear clearness bound | `0.80` | `dimensionless` | EB-01A mapping |
| overcast clearness bound | `0.15` | `dimensionless` | EB-01A mapping |
| cloud mixture weight | `0.84` | `dimensionless` | `REF-SNOWENERGY-FLERCHINGER` |
| diffuse extinction multiplier | `1.6` | `dimensionless` | `REF-SNOWENERGY-FSM2`, Eq. 14 |
| canopy/snow emissivity | `1` | `dimensionless` | effective-unity exchange convention admitted by EB-01A/FSM2; atmospheric emissivity remains variable |
| `R_a,min` | `1e-9` | `MJ m^-2 d^-1` | EB-03 numeric divide/branch threshold; not a fit parameter |
| hourly duration | `3600` | `s` | typed hourly forcing cadence and named time conversion |
| latent heat of fusion, `L_f` | `333600` | `J kg^-1` | exact pinned libsnobal `LH_FUS(FREEZE)=3.336e5 J kg^-1` phase-change constant; not a calibration coefficient |
| normal maximum active-layer depth | `0.25` | `m` | fixed Marks/SNOBAL `max_z_s_0`; exceeded only by the exact `INV-SNOWENERGY-026` lower-volume collapse; not a user coefficient |
| libsnobal sea-level pressure | `101324.6` | `Pa` | fixed `SEA_LEVEL` constant used by `efcon` |
| libsnobal dry-snow conductivity factor | `4.186798188 * 0.0077` | `W m^-1 K^-1` | exact `CAL_TO_J(0.0077)` factor in `KTS`; density enters as `(rho/1000)^2` |
| normal mass threshold | `60` | `kg m^-2` | fixed Marks/SNOBAL timestep threshold |
| medium mass threshold | `10` | `kg m^-2` | fixed Marks/SNOBAL timestep threshold |
| minimum resolved thermal mass, `m_res` | `1` | `kg m^-2` | exact libsnobal threshold: total mass `<=` suspends; lower-volume mass `<` collapses to one volume; lower-volume equality remains two-volume |
| density-layer zero mass | `1e-9` | `kg m^-2` | existing density-model lifecycle boundary; equivalent to `1e-12 m` SWE through `rho_w`, not the aggregate closure tolerance |
| layer aggregate closure tolerance | `1e-9` | `m` | existing independent SWE and physical-depth residual bound; never a layer-deletion threshold |
| same-residual area-mass closure tolerance | `1e-6` | `kg m^-2` | exact area-mass equivalent of `1e-9 m` SWE through `rho_w=1000 kg m^-3`; applies to vapor-to-sublimation transfer closure |
| hourly/daily vapor-aggregation tolerance | `1e-9` | `kg m^-2` | separate aggregation reconstruction predicate; not the vapor-to-sublimation transfer tolerance or layer lifecycle boundary |
| medium duration | `900` | `s` | fixed `15 minute` Marks/SNOBAL level |
| small duration | `60` | `s` | fixed `1 minute` Marks/SNOBAL level |

None of these constants is a user calibration coefficient in the admitted
model.

## Unit Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `T_a` | `K` | typed `TemperatureCelsius` boundary with named kelvin conversion | temperature helper | none | internal diagnostic only |
| `e_a` | `kPa` | typed non-negative vapor-pressure wrapper | daily dewpoint-to-vapor helper | none | internal diagnostic only |
| `R_s`, `R_a` | `MJ m^-2 d^-1` | typed non-negative daily-radiation wrapper | named hourly-sum and solar producer | none | internal diagnostic only |
| `k_t`, `c` | `dimensionless fraction` | typed bounded fraction | none | none | internal diagnostic only |
| `C` | `dimensionless fraction` | existing native canopy state; registry disposition owned by `SC-PLANT-001` | none | existing typed producer semantics; no new exception | existing canopy publication unchanged |
| `P_0`, `f_sky` | `dimensionless fraction` | typed bounded fraction | none | none | internal diagnostic only |
| `w` | `kg m^-2` | internal derived operand; no boundary entry in EB-02 | equation-local `4650 e_a/T_a` only | package evidence scalar only; future typed internal value or recorded exception | not published by EB-02 |
| `epsilon_clear`, `epsilon_all` | `dimensionless` | typed bounded fraction | none | none | internal diagnostic only |
| `T_c`, `T_s` | `K` | typed `TemperatureCelsius` provider | named kelvin conversion | none | internal diagnostic only |
| `L_clear`, `L_atm`, `L_can`, `L_sub`, `L_out`, `L_net` | `W m^-2` | typed energy/radiative flux wrappers | named hourly integration helper | none | internal diagnostic only |
| `Q_cc`, `Q_E`, `Q_complete`, `Q_cold_required`, `Q_excess`, `delta_E_cold`, `Q_refreeze`, `Q_unallocated_after_exhaustion` | `J m^-2` | Stage 3 scalar with contract-bound guard | no conversion | retained/future scalar exception: internal area-normalized energy ledger | required cutover evidence for future operands |
| `alpha_v`, `Q_v`, `L_s,auth` | `dimensionless`, `J m^-2`, `J kg^-1` | no boundary entry; private unpublished version-32 numerical operands | no conversion | contract-bound internal scalar only | prohibited from publication, schema, persistence, or diagnostics |
| `m_melt` | `kg m^-2` | future Stage 3 solid-to-liquid ledger | named `Q_excess/L_f` conversion bounded by `m_ice_available` | internal scalar with linked-ledger guards | required cutover evidence |
| `m_ice_available`, `m_liquid_external_in`, `delta_m_retained`, `m_refrozen`, `m_routed`, `m_solid_precip`, `m_deposition`, `m_sublimation` | `kg m^-2` | future Stage 3 phase/liquid ledgers | named phase-state and exact handoff conversions only | internal scalars with linked-ledger guards | required cutover evidence |
| `m_v,raw` | `kg m^-2` | schema-v6 `vapor_mass_exchange_kg_m2` diagnostic scalar | named turbulent mass-flux duration integration | retained evaluation-only scalar exception | internal diagnostic only |
| `m_v` | `kg m^-2` | schema-v6 bounded `deposition_kg_m2` and `sublimation_kg_m2` fields or future typed Stage 3 transfer | named availability bound and signed recomposition | internal scalar with mass/latent ledger guards | required bounded-transfer evidence |
| `m_res` | `kg m^-2` | fixed internal model-domain constant | named SWE-to-mass conversion using `rho_w` | no user boundary or scalar exception | contract and environment-gated trace metadata |
| `m_layer` | `m` SWE at the typed state seam; `kg m^-2` for lifecycle comparison | `SWE_layer` (`snow.layer.mass_swe_m`) | `snow_water_equivalent_meters_to_area_mass_kg_m2` | typed `DirectSnowLayerState` vector-element scalar exception | internal state only |
| `z_layer` | `m` | `z_layer` (`snow.layer.thickness_m`) | identity | typed `DirectSnowLayerState` vector-element scalar exception | internal state only |
| `t_unres` | `s` | Stage 3 diagnostic scalar | accumulated explicit substep duration | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `m_l` | `kg m^-2` | Stage 3 lower-volume diagnostic scalar | named SWE-to-mass conversion using `rho_w` | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `t_collapse` | `s` | Stage 3 diagnostic scalar | accumulated explicit substep duration | internal scalar exception with contract-bound non-negative guard | environment-gated research trace |
| `p_a` | `Pa` | typed positive pressure wrapper | named elevation-to-pressure projection | none | environment-gated research trace |
| `k_d`, `k_eff` | `W m^-1 K^-1` | typed positive thermal-conductivity wrapper | exact named `KTS+efcon` helper | none | environment-gated research trace operands |
| `T_ca`, `q_ca` | `K`, `kg kg^-1` | typed shared carrier node | identity at trial state | none | carrier receipt |
| `H_i`, `V_i` | `W m^-2`, `kg m^-2 s^-1` | typed flux wrappers | named duration integration | no raw 10 m wind alias | carrier ledger |
| `L_can`, `L_snow<->canopy` | `W m^-2` | typed longwave ledger | Stefan--Boltzmann and complementary exchange | none | reciprocal ledger |

Energy-carrier integration from `W m^-2` uses the explicit typed hourly
duration of `3600 s`; a hidden daily factor of `86400` is prohibited.

## Tolerance and Numeric Notes

INV-035 inherits the INV-034 event tolerances for numeric closure only.
Fingerprint, half-open support, receipt cardinality, debit/credit/consumed
state, CoE exclusion, and recipient absence on failure are exact.

INV-056 introduces no tolerance. Endpoint purity, signed-vapor opposition,
exact-zero interface terms, latent linkage/sign, support lower bound, component
ordering, identities, events, topology, custody, receipts, synthetic status,
rollback, and publication eligibility are exact predicates. Existing physical
mass/energy closure tolerances validate independently reconstructed operands;
they do not repair a root, component, phase, or linkage failure. The
`60_000_000_000 ns` value remains only the minimum exact covered-support floor;
version 32 is explicitly eligible on larger exact supports and changes no
adaptive duration proposal or acceptance rule.

- `TOL-SNOWENERGY-001` governs terminal numerics only. The step-doubling LTE
  norm is `max_i |fine_i-coarse_i| / (a_i + 1e-8 *
  max(|fine_i|,|coarse_i|))`, with `a_mass=1e-9 kg m^-2` and
  `a_energy=1e-6 J m^-2`; acceptance requires norm `<=1`. Proposed duration is
  at most `60 s` and at least `1e-9 s`; 64 consecutive rejections fail typed.
  Event bisection requires bracket width `<=1e-6 s`, endpoint solid
  `<=max(1e-12 kg m^-2,1e-12*m_i,start)`, and at most 64 iterations. Snow-side
  mass/energy closure remains independently scale-aware at
  `max(1e-12 kg m^-2,1e-12*sum_abs_mass_operands)` and
  `max(1e-6 J m^-2,1e-12*sum_abs_energy_operands)`. LTE, event-root, and
  closure tolerances are not interchangeable and authorize no state clamp.

| Tolerance ID | Binding rule | Guard |
|---|---|---|
| `TOL-SNOWENERGY-001` | Terminal step-doubling, event-root, and independent mass/energy closure tolerances are distinct and never repair identity or state. | typed numerical failure |
| `TOL-SNOWENERGY-002` | OFE tile-fraction closure residual `abs(sum_i(f_i)-1) <= 1e-12` (dimensionless) admits only floating-point summation roundoff; it never changes, rescales, or renormalizes any fraction or flux. Identity, ordering, cardinality, duplication, area basis, boundary class, model definition, and state joins remain exact. | typed topology failure |
| `TOL-SNOWENERGY-003` | Covered fixed-point state convergence uses absolute bounds only: `1e-9 m` for SWE, thickness, liquid, and refrozen depth; `1e-8 K` for temperature difference; `1e-6 kg m^-2` for cumulative/detached mass; and `1e-6 J m^-2` for cold content and cumulative energy. Density and all structural/count-like fields are exact. No relative term, clamp, canonicalization, or cross-unit substitution is admitted. | typed nonconvergence |
| `TOL-SNOWENERGY-004` | `OPENWEPP_STAGE3_ADAPTIVE_OWNER_TOLERANCE_V1` governs only direct-versus-composed complete-owner truncation error. Snow depth uses `1e-9 m + 5e-3*max_abs`, snow mass uses `5e-6 kg m^-2 + 5e-3*max_abs`, snow energy uses `1e-6 J m^-2 + 5e-3*max_abs`, and snow temperature uses `1e-2 K + 1e-8*max_abs`. Soil-thermal energy alone uses `1e-6 J m^-2 + 1.5e-2*max_abs`; LSE energy uses `1e-6 J m^-2 + 5e-3*max_abs`. Other complete-owner fields retain their owning-contract absolute floor and `1e-3` relative tolerance for extensive mass/depth/energy or `1e-6` for other intensive state; temperature uses `1e-2 K + 1e-8*max_abs`. These constants do not govern or relax independent mass/energy ledger closure. Exact topology, active-set, posture, ordering, schema, and thermodynamic-provider identities remain exact. | typed adaptive refinement or qualification hold |
| `TOL-SNOWENERGY-005` | Final snow--soil receipt nonlinear termination admits only finite reconstructed-endpoint residuals `<=1e-9 J m^-2` for applied energy and `<=1e-8 K` for each ending temperature. The receipt retains the exact equal/opposite energy actually consumed and reseals only exact installed candidate identities. The energy bound is 1,000 times smaller than the unchanged physical-ledger closure threshold; it cannot excuse a sign, node, topology, support, owner, digest, mass/energy-ledger, or discrete-phase mismatch. | bounded fixed-point retry; cap exhaustion remains typed nonconvergence |
| `TOL-SNOWENERGY-006` | A refreeze-capacity operation that selects `m_refreeze=Q_cc/L_f` may canonicalize only its finite binary64 quotient/product remainder `Q_cc-L_f*m_refreeze` in `[-1e-9,0) J m^-2` to exact `+0 J m^-2`. This threshold is 1,000 times smaller than the unchanged `1e-6 J m^-2` physical energy-ledger closure bound. It changes neither refrozen mass nor fusion energy; any more-negative result, any nonfinite operand, a non-capacity-limited branch, or arbitrary negative cold content is a typed domain failure. | exact zero canonicalization at the constitutive operation; otherwise typed domain failure |

- Analytical evidence uses an absolute tolerance of `1e-9` for dimensionless
  identity checks and `1e-6 W m^-2` for independently reconstructed fluxes.
- Runtime snow-mass closure uses `1e-9 m` water equivalent. Through the named
  `snow_water_equivalent_meters_to_area_mass_kg_m2` conversion and
  `rho_w=1000 kg m^-3`, the same residual is exactly `1e-6 kg m^-2`; the
  vapor-to-sublimation transfer identity uses this area-mass bound. Energy
  closure uses `1e-6 J m^-2`. These tolerances do not relax physical domains.
- Hourly/daily vapor-aggregation reconstruction separately uses
  `1e-9 kg m^-2`. The density-layer lifecycle boundary also has the numeric
  value `1e-9 kg m^-2` (`1e-12 m` SWE), but it is a representation predicate,
  not a residual-acceptance threshold. Neither predicate may be substituted
  for the `1e-6 kg m^-2` vapor-to-sublimation transfer closure, and that
  transfer tolerance may not be generalized to other mass checks.
- Evaluate fourth powers in finite `f64`; reject non-finite intermediate
  values.
- The canonical canopy producer currently caps effective cover below one.
  The longwave consumer nevertheless must guard `C >= 1` rather than inventing
  its own epsilon clamp.
- The limit `C -> 1` is a scientific test; `C=1` is outside the admitted
  finite inversion domain.

## Calibration and Identifiability

Disposition: `CALIBRATION_NOT_APPLICABLE`.

Version 32 adds no calibratable parameter. `alpha_v` is uniquely determined by
the two signed endpoint masses and `w_p` is the unchanged numerical controller
weight; neither is an observation, fitted coefficient, constitutive constant,
or empirical calibration surface.

```text
science_implementation_status = IMPLEMENTED
stage3_melt_owner_status = AUTHORITY_ADMITTED_IMPLEMENTATION_HOLD
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

`IMPLEMENTED` applies to the canonical longwave equations, their default-off
diagnostic/reproduction seam, and the version-3 active-layer coupled provider.
EB-03A production, analytical reconstruction, and real B/L/S/LS consumer gates
pass. The two `NOT_APPLICABLE` fields reflect that this contract defines no
empirically estimated parameter surface.

`AUTHORITY_ADMITTED_IMPLEMENTATION_HOLD` applies specifically to the
version-7 melt-owner target. It is not a runtime-conformance claim. Current
CoE melt remains byte-identical compatibility behavior until
`GAP-SNOWENERGY-011` is closed.

The admitted equations use fixed literature constants and existing forcing or
state variables. EB-03 performs no fitting and introduces no tunable
sky-view, extinction, canopy-temperature, or emissivity coefficient.

| Candidate | Calibration status | Reason |
|---|---|---|
| `1.6` diffuse multiplier | fixed | literature equation constant |
| `0.15`, `0.80`, `0.84` cloud mapping | fixed for canonical route | changing them would require a new authority/calibration package |
| `C` | externally produced state | governed by `SC-PLANT-001`, not fit here |
| `T_c`, `T_s` | provider states | Stage 3 / air-temperature approximation, not fitted |
| `R_a,min` | fixed numeric guard | divide/branch threshold, not empirical calibration |

Identifiability warning: fitting an extinction coefficient after deriving
`f_sky` from effective cover would confound canopy-state calibration with the
radiative translation and is prohibited in this version.

## Test Vector Obligations

The implementation increment must reproduce package artifact
`analytical-test-vectors.csv` and include at least:

1. open canopy: `C=0`, `f_sky=1`, `L_sub=L_atm`;
2. intermediate cover values independently evaluated as `(1-C)^1.6`;
3. near-closed-canopy limiting behavior;
4. monotonic decrease of `f_sky` with increasing `C`;
5. clear and overcast clearness endpoints and both clamp sides;
6. independent reconstruction of Dilley clear-sky and all-sky fluxes;
7. complementary sky/canopy mixing and net-longwave sign;
8. typed rejection of invalid temperatures, cover, and forcing;
9. typed unavailable polar-night cloud inference;
10. distinct hourly-temperature fluxes under held daily vapor/cloud state and
    an explicit nonzero daily-mean-temperature substitution bias; and
11. typed failure when the Stage 3 thermal provider is absent;
12. all four orthogonal selector cells with identical non-candidate settings;
13. independent latent/mass equivalence and wrong-sign rejection;
14. sublimation mass closure with explicit non-aliasing to liquid/melt; and
15. cold-content energy closure including exported cold content;
16. active depth and mass reconstructed across at least three depositional
    layers with a boundary-crossing split;
17. a shallow pack whose complete mass, rather than a thin snowfall-event
    layer, supplies the active heat capacity;
18. `G_0` sign, equal-and-opposite active/lower ledgers, isothermal zero-flux,
    and harmonic-conductivity reconstruction;
19. exact `60/15/1 minute` transitions around `60/10/1 kg m^-2`; and
20. a thin-pack chronology proving substep reevaluation and rejecting hourly
    debit, absolute-zero clamp, air-temperature replacement, and cold-content
    tax alternatives; and
21. unequal depositional temperatures proving one shared active `T_0`, a
    distinct persistent lower temperature, nonzero correctly signed `G_0`,
    exact active/lower cancellation, and exact libsnobal `KTS+efcon`
    conductivity rather than the Sturm frost relation; and
22. exact total `m_s < 1`, `m_s = 1`, and `m_s > 1 kg m^-2` vectors proving
    pre-partition zero exchange and unchanged persistent state below/at the
    boundary; plus lower-volume `m_l < 1` collapse, `m_l = 1` two-volume
    equality, normal evaluation/resume, explicit runner diagnostics, and
    rejection of forced melt, deletion, temperature clamp, epsilon vapor
    pressure, and one-more-flux aliases.
23. exact density-layer mass-boundary vectors below, at, and above
    `1e-9 kg m^-2`; captured fragment vectors proving state retention and
    independent mass/depth reconstruction; and negative vectors proving
    cross-unit filtering, tolerance inflation, fragment deletion, and a
    material aggregate mismatch are rejected.
24. a complete-component energy vector proving cold-content-first application,
    `m_melt=min(Q_excess/L_f,m_ice_available)`, and exact energy closure;
25. same-substep solid debit, liquid credit, refreeze, retention, and routing
    vectors with three independent ledger reconstructions; and
26. negative cutover vectors rejecting simultaneous CoE/Stage 3 generation,
    omitted sensible or precipitation-advection terms, discarded positive
    energy, delayed/duplicate liquid disposition, and an unauthorized
    `m_s <= 1 kg m^-2` proxy.
27. raw-versus-bounded vapor vectors covering inactive and active availability
    bounds; `S/F` N/A; independent Q deposition/sublimation; same-sign wrong
    magnitude; direction reversal; simultaneous deposition/sublimation;
    endpoint-preserving melt/vapor aliases; active/total endpoint substitution;
    and raw latent opportunity versus bounded-transfer latent energy.
28. terminal-domain exact sides at total ice below, equal to, and above
    `1 kg m^-2`; cold/warming no-event; pure melt; pure sublimation; joint
    melt/sublimation; positive deposition and refreeze no-false-event; and an
    exact hour-end event.
29. first-order step-doubling discrepancy/refinement and event-time convergence
    against an independent constant-carrier analytical or tighter bisection
    oracle, plus typed nonfinite, step-underflow, rejection-limit, invalid
    bracket, nonmonotone candidate, and iteration-limit failures with atomic
    state identity.
30. schema-v8 independent reconstruction of endpoint solid, enthalpy, liquid,
    terminal energy, evaluated/unevaluated time, and bracket/error evidence;
    negative vectors reject full-step flux, omitted deposition/refreeze,
    post-event snow energy, producer residuals, request/state mismatch, and any
    receiving-surface interpretation.
31. finalization-restart vectors at 60-second and ordinary supports proving raw
    exact-floor behavior, support-scaled contraction, cumulative closure,
    bitwise authentic candidate density without interpolation, and continued
    exact-density nonconvergence until an authentic candidate matches;
32. refusal vectors for terminal-model, topology/cardinality, settling,
    initial/input, represented-mass, aggregate-posture, invalid reconstruction,
    and cumulative-closure changes, each retaining the raw authentic candidate;
    and
33. a stateful seam vector proving a relaxed finalization restart survives
    intervening nonconvergence, consumes exactly one otherwise-converged Picard
    crossing, permits the next authentic finalization retry, and never routes
    the relaxed intermediate to publication or replay; and
34. exact-floor terminal-one-volume phase-aware vectors covering the captured
    `1_860_000_000_000..1_980_000_000_000 ns` mixed-`0 C` to dry-frozen
    authentic image, its exact coordinated midpoint, independent `W/H` and
    mass/energy reconstruction from immutable beginning plus complete support
    operands, exact `H=0` and `H=Lf W` boundary projection, unchanged density
    and structural identities, raw authentic history, vapor-sign/disposition,
    component-closure, nonfinite, support, structure, topology/custody/receipt
    poisons, explicit rejection of independently interpolated cumulative melt,
    refreeze, and energy fields, and source proof that the midpoint cannot
    finalize, replay, accept, or publish; and
35. version-32 active-set vectors using the exact captured `1860..1920 s`
    deposition/sublimation and latent-energy operands, independently deriving
    `alpha_v=0.04393657257739406`, exact positive-zero interface vapor and
    latent energy, and rejecting the affine-latent
    `+45.77845449909091 J m^-2` wrong result; zero-to-deposition and
    zero-to-sublimation entries with preserved authentic positive finite
    specific latent heat; same-sign dispatch through version 31; exact-floor
    and `>60 s` direct supports; mixed `D/S`, same-sign-to-v32, endpoint-zero,
    zero/zero, nonfinite/root/component/latent/capacity/identity/event/topology/
    custody/receipt/cap/rollback/publication poisons; raw authentic history;
    synthetic ineligibility; and later fresh-authentic-only finalization; and
36. version-33 exact 60/120-second active-set transition-reset detection over
    `root/interface -> branch-entry -> opposite raw-authentic -> same
    root/interface/reset`, proving exact joins/root coordinates/branch predicates
    and opposite pure vapor disposition while permitting asymptotically changing
    raw-authentic continuous owner fields;
    cold, mixed-phase, and exact-fusion known roots through canonical `Pi(W,H)`;
    sealed evaluator carriers and independent concrete `R_W/R_H/R_E/R_T` from
    exact water, complete ordered energy including `Q_v=V L_s`, unchanged
    snow--soil Crank--Nicolson exchange, soil enthalpy--temperature closure, LSE,
    CN, identity, custody, and receipt side constraints; explicit rejection of
    coordinate-map `F(x)-x`; deterministic safeguarded semismooth/trust-region
    progress; one shared `CoveredPhysicalEvaluationBudgetV1` covering trigger,
    residual, generalized-Jacobian, rejected trust, fresh, and final replay;
    `CoveredConvergenceAdmissionV1::CoupledAuthentic` bypass of Picard equality
    only, with every finalization/reseal/rollback/publication check retained;
    root distinction from every version-31/32 affine or synthetic image; exact
    event partition/refusal and phase-complementarity classification; nonfinite,
    noncycle, same-side, singular, stagnation, domain, capacity, component,
    latent-linkage, soil, LSE, identity, topology, custody, receipt, budget,
    replay, rollback, and publication poisons; source binding for
    `PhaseConsistentCoupledSolveV1`, `phase_consistent_coupled_solve_v1`,
    `phase_consistent_coupled_authentic_final_evaluation_v1`, and
    `phase_consistent_coupled_authentic_final_replay_reseal_v1`; plus canonical
    one-day accepted/rejected counts, width distribution, wall time, limiting
    rejection reasons, maximum ledger residuals, and zero repeated 96-evaluation
    ceilings without production persistence of microstepping diagnostics; and
37. version-34 stable-monotone eligibility over exactly eight consecutive raw
    authentic maps on one event-free terminal-one-volume covered support at or
    above the exact 60-second floor: exact support/source/event/topology/
    custody/static-receipt/phase-branch/carry-authority-and-representation
    joins, exact per-map `E=exact(H_hi)+R` reconstruction while physical
    receipt digests and `H_hi/R` coordinates evolve, finite concrete
    `R_W/R_H/R_E/R_T`, strictly decreasing governed residual merit, no
    `A/B/A`, active-set transition, or finalization restart, and one shared
    already-charged 96-evaluation budget. Exact-side tests must prove that any
    changed static join/phase/carry authority or representation, failed exact
    reconstruction, nonfinite or nondecreasing merit, active-set change,
    cycle, restart, or budget exhaustion refuses the trigger; pre-root
    solver refusal discards every private trial and resumes raw authentic
    Picard only within the remaining budget; no private trial may accept; and
    only fresh `CoupledAuthentic` replay/reseal may proceed through unchanged
    finalization/publication guards. Canonical one-day qualification must
    report accepted/rejected counts, step-width distribution, runtime,
    limiting rejection reasons, maximum ledger closure, evaluator/budget
    counts, repeated-96 outcomes, and absence of production-persisted
    microstepping diagnostics; and
38. version-35 authentic receipt stabilization after a coupled physical root:
    immutable canonical input `R_n`; charged physical reconstruction/reseal of
    `R_(n+1)` from immutable owners and sealed sources; next-input adoption
    without tolerance, digest repair, or in-place mutation; explicit treatment
    of the first root reseal as a cross-input probe rather than exact replay;
    exact input/output receipt equality as the sole stabilization predicate;
    and one independent same-stabilized-input replay requiring exact finite
    residual, complete candidate-artifact, and reconstructed-receipt equality
    before `CoupledAuthentic` or finalization. Vectors must cover multiple
    probes on the one unchanged 96-evaluation budget, exact receipt
    period/oscillation, nonfinite state, side-constraint/reconstruction
    failure, budget exhaustion, replay disagreement, private/probe artifact
    disposal, rollback, and no intermediate acceptance/publication. Retain the
    r83 log and hash as the pre-amendment real-fixture refusal; and
39. version-36 geometry-complete vectors: retained r88 thickness bits
    `4569208177783694401 -> 4569208162027237604`, independent decimal delta
    `6.833273876e-9 m`, exact canonical `I/rho` reconstruction, physical
    `R_rho` from unchanged density/settling operands, generalized
    `R_W/R_H/R_rho/R_E/R_T`, evolving rho/z under exact layer/settling/model
    branch authority, and the same shared budget plus v35 stabilization.
    Refuse zero/negative/nonfinite density, mass-depth mismatch, branch/layer/
    settling poison, `F(x)-x`, interpolation/repair, tolerance/cap/floor
    change, uncharged physics, private acceptance, receipt/finalization bypass,
    rollback mutation, and intermediate publication.
40. version-37 derived-thickness vectors: retained r93 root-to-finalization
    lane-1 thickness bits `4569208177783694401 -> 4569208162027237604`, exact
    `I/rho` reconstruction of both sides, unchanged `R_W` and `depth_abs_m`
    bounds, and a low-density case where `R_W` passes while derived `R_z`
    correctly blocks root admission. Positive vectors must close `R_z` through
    the existing water-mass solve without adding a coordinate or constitutive
    call, retain v35 exact receipt stabilization/same-input replay and unchanged
    authentic finalization, and prove the final Stage 3 image passes its
    existing depth comparison. Refuse omitted depth merit, independent-z solve,
    map difference, interpolation/copy/repair, tolerance/cap/floor change,
    uncharged or duplicate physics, receipt/replay/finalization bypass, rollback
    mutation, and intermediate publication; and
41. version-46 complete-step budget vectors: residual dimensions one, five,
    and multiple lanes/soil nodes; exact `d+1+r` boundary admission; one-charge
    shortfall refusing before any Jacobian column; above-tolerance typed budget
    failure versus sub-tolerance private receipt-entry stop; retained per-map
    reserve on reverse perturbations and twelve trust backtracks; malformed or
    overflowing cardinality; enlarged/reset/separate budget; partial-column or
    rejected-artifact substitution; a multi-probe evolving receipt chain whose
    final exact probe plus independent replay ends at shared used 96; exact
    rollback and no private/probe publication. Retain r119/r120 identities and
    state explicitly that canonical sufficiency is pending; and
42. version-56 frozen temperature-primary vectors: exact strictly frozen and
    noncrossing eligibility sides; exact `(W,T_s,rho,E_soil,T_soil)` order;
    independent exact-dyadic reconstruction of
    `H=-exact(W)*exact(c_ice)*(exact(273.15)-exact(T_s))`; round-nearest-even
    tie, cancellation, subnormal, overflow, and high-plus-carry ledger cases;
    unchanged CN Q derived and consumed exactly once; exact shared-budget
    accounting and dispatch before V55; compound snow material owner and
    additive carry-receipt order/schema/support/transaction/predecessor/base-
    digest/branch/topology/custody poisons; complete CN-plus-carry receipt
    stabilization, independent same-input replay, and strict finalization;
    committed, pending, in-progress-day, and in-progress-support restart
    round trips; V4-to-V5 canonical-zero migration and nonzero V5-to-V4
    downgrade refusal; exact rollback/no-publication/no-diagnostics; and r144
    proof that the 21 evaluated transient Q candidates are not carry authority.
    Phase crossing, liquid/melt/refreeze, event, branch/density/order change,
    carry omission/substitution/repair, receipt repair, uncharged physics,
    tolerance/cap/floor change, and V54/V55 witness promotion must refuse.

Producer-only analytical vectors cannot close runtime activation. EB-03 must
prove the real shared Stage 3 snow-energy consumer reads the contracted
operands.

### Child 2C canonical obligation IDs

| ID | Binding requirement | Enforcement |
|---|---|---|
| `TOL-SNOWENERGY-001` | Terminal step-doubling, event-root, and independent mass/energy closure tolerances are distinct and never repair identity or state. | typed numerical failure |
| `TOL-SNOWENERGY-002` | OFE tile fractions close within `1e-12` dimensionless summation residual without normalization; every nonnumeric topology join remains exact. | typed topology failure |
| `TOL-SNOWENERGY-003` | Covered Stage 3 fixed-point state uses physical-class absolute bounds only; fingerprints are reconstructed per candidate, while density and structural/count-like state remain exact. | typed nonconvergence |
| `TOL-SNOWENERGY-006` | Only a refreeze-capacity `Q_cc/L_f` quotient/product remainder in `[-1e-9,0) J m^-2` canonicalizes to exact zero; mass, fusion energy, and physical closure thresholds are unchanged. | constitutive refreeze validator |
| `OBL-SNOWENERGY-P-009` | Emit one immutable terminal receiver receipt or a typed failure; never partially commit. | receiver transaction |
| `OBL-SNOWENERGY-C-016` | Reconstruct snow, liquid, vapor, fusion energy, and time without post-event snow operands or aliases. | receiver validator |
| `INV-SNOWENERGY-041` | Terminal numerical tolerances are typed, distinct, and never repair identity, support, or state. | numerical validator |
| `INV-SNOWENERGY-043` | Covered convergence reconstructs candidate fingerprints and separates exact structure/count/density from unit-specific absolute numeric residuals. | convergence validator |
| `INV-SNOWENERGY-044` | Lane receipt V1 is non-restorable; V2 restart schema remains undefined and blocked pending exact normative framing and replay joins. | restart schema/version guard |
| `INV-SNOWENERGY-054` | Failed finalization uses only the guarded unpublished Stage 3 contraction, retains authentic LSE/boundary/soil operands, copies candidate density bitwise without interpolation, consumes one stabilization crossing, and publishes only an authentic map image. | fixed-point finalization and publication guards |
| `INV-SNOWENERGY-055` | Exact-floor terminal-one-volume contraction reconstructs one coordinated `W/H` midpoint from immutable beginning plus complete support operands, projects phase canonically, retains every identity and independent closure, and remains unpublished until a fresh authentic image passes. | covered fixed-point phase-aware midpoint and publication guards |
| `INV-SNOWENERGY-056` | Any exact support at or above 60 seconds may localize only a pure opposite-sign actual-vapor interface, set linked vapor/latent terms to exact zero, enter one authentic side under the existing support-scaled weight and authentic latent heat, reconstruct canonical `W/H`, and retain synthetic-ineligible/fresh-authentic-only publication posture. | covered fixed-point vapor-active-set, closure, identity, and publication guards |
| `INV-SNOWENERGY-057` | Rolling exact active-set transition resets, not bitwise-equal raw owners, invoke the reduced solve through concrete `R_W/R_H/R_E/R_T`, algebraic side constraints, one shared 96-physical-evaluation budget, and fresh `CoupledAuthentic` admission that bypasses Picard equality only. A failed exact reset promotes the current validated interface and clears the branch-entry window; it never leaves a stale first root latched. | covered fixed-point trigger, evaluator, event, identity, closure, budget, admission, reseal, rollback, and publication guards |
| `INV-SNOWENERGY-058` | Eight stable monotone raw authentic maps may invoke only the unchanged equation-level solver when exact static receipt/phase/carry-authority-and-representation joins, exact evolving `E=exact(H_hi)+R` reconstruction, finite residuals, strict merit decrease, and shared-budget conditions hold; physical receipt digests and carry coordinates may evolve under unchanged custody/closure, and any pre-root refusal discards private trials and resumes raw Picard only within the remaining budget. | covered fixed-point stable-monotone trigger, evaluator, custody, exact carry, budget, fallback, admission, reseal, rollback, and publication guards |
| `INV-SNOWENERGY-059` | A coupled root enters charged authentic receipt probes until exact input/output receipt equality, then one independent replay under the identical stabilized receipt input must reproduce exact residuals, complete artifacts, and receipts before admission; cross-input root/reseal comparison and probe retention are forbidden. | covered fixed-point authentic receipt stabilization, budget, exact replay, artifact-disposal, rollback, admission, and publication guards |
| `INV-SNOWENERGY-060` | The geometry-complete solver adds one terminal density coordinate, derives thickness exactly as `I/rho`, evaluates physical `R_rho` from unchanged density/settling authority, solves `R_W/R_H/R_rho/R_E/R_T`, shares the unchanged budget, and retains v35 receipt/authentic finalization. | covered fixed-point geometry, physical residual, branch, budget, receipt, finalization, rollback, and publication guards |
| `INV-SNOWENERGY-061` | Derived physical `R_z` from canonical `I/rho` must close under the unchanged depth bound before a coupled root is admitted, while `z` remains derived and the solved vector remains `R_W/R_H/R_rho/R_E/R_T`. | covered fixed-point derived-depth merit, budget, receipt, replay, finalization, rollback, and publication guards |
| `INV-SNOWENERGY-062` | Every charged coupled physical evaluation must execute the authentic finalization-equivalent endpoint map directly and exactly once; provisional-map closure cannot admit a root. | covered fixed-point map posture, operand identity, budget, receipt replay, independent finalization, rollback, and publication guards |
| `INV-SNOWENERGY-063` | Every V2 soil-energy operand binds the exact outer source transaction and exact authenticated soil-target transaction independently; later-child inequality never permits inferred adjacency or substitution. | V2 source/target transaction, support, owner, receipt, digest, rollback, and publication guards |
| `INV-SNOWENERGY-064` | Four exact-static rolling active-set reset windows with strictly decreasing finite positive tolerance-independent root drift may trigger only the unchanged physical solver while preserving the same shared budget and authentic-only admission. | covered parity trigger, exact joins, drift, cadence, budget, replay, rollback, and publication guards |
| `INV-SNOWENERGY-065` | Four exact-static rolling reset windows that retain bit-exact water and sweep enthalpy strictly one-way across exactly one adjacent canonical phase boundary may trigger only the unchanged physical solver, without tolerance or root admission. | covered one-way enthalpy bracket, exact joins/cadence/budget, replay, rollback, and publication guards |
| `INV-SNOWENERGY-066` | Canonical support enthalpy includes exact authentic cold-content export `X_c`, and every private contraction carries that nonlatent operand with the same ordered weight before exact endpoint closure. | covered support-coordinate reconstruction, authentic result lineage, rollback, and no-publication guards |
| `INV-SNOWENERGY-067` | A native V2 numerical-coordinate trial remains a typed same-support private fixed-point image and cannot be reconstructed or authenticated as ordinary base unpublished physics. | V2 projection authority/set, prepared-owner/support/receipt/topology/carry, base/sequential/acceptance/publication, and rollback guards |
| `INV-SNOWENERGY-069` | A tolerance-closed private coupled root is polished only through stricter progress in the existing physical residual merit, with a matching carried artifact bundle and hard reservation of authentic probe/replay capacity; only exact receipt equality plus independent same-input replay admits. | covered physical residual, budget, receipt, replay, finalization, rollback, and publication guards |
| `OBL-SNOWENERGY-C-019` | Apply only `TOL-SNOWENERGY-003` after independent fingerprint validation. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-020` | Reject V1/inferred restart wire and require the complete V2 topology/owner replay join. | additive-restart consumer |
| `OBL-SNOWENERGY-C-024` | Independently reconstruct the version-32 root, nonlatent components, linked one-sided latent energy, canonical `W/H`, all exact identities, raw authentic history, and synthetic-ineligibility on exact-floor and larger direct supports. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-025` | Bind rolling exact 60/120-second transition-reset detection without raw-owner bit equality: a nonexact reset must rearm from the current validated interface and clear branch-entry state, while only a later exact complete window dispatches. Retain equation-level residual carriers, CN/LSE/receipt side constraints, one shared physical-evaluation budget, Picard-only `CoupledAuthentic` bypass, full refusal/replay behavior, and canonical one-day performance/ledger evidence without persistent diagnostics. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-026` | Bind the exact eight-map stable-monotone trigger, unchanged static receipt/phase/carry authority and representation with evolving physical receipt/carry coordinates and exact high-plus-carry reconstruction, finite strictly decreasing physical merit, the existing shared 96-evaluation budget, private-trial disposal plus raw-Picard fallback, fresh-only `CoupledAuthentic` replay/reseal, full refusal behavior, and canonical one-day performance/ledger evidence without persistent diagnostics. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-027` | Bind immutable receipt-input probes, exact input/output receipt stabilization, the first reseal's cross-input probe posture, same shared budget charging, one independent same-stabilized-input exact residual/artifact/receipt replay, oscillation/nonfinite/constraint/budget refusal, private/probe artifact disposal, and fresh-only admission/finalization without persistent diagnostics. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-028` | Bind the one-coordinate density geometry extension, canonical `I/rho` thickness, physical `R_rho`, exact branch authority with evolving rho/z, generalized residuals, shared budget, v35 receipt/finalization path, and complete map-difference/interpolation/repair/uncharged/bypass poisons. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-029` | Bind derived physical `R_z` from canonical proposed/physical `I/rho` into scaled merit and root admission under unchanged `depth_abs_m`, without adding a z coordinate, physics call, equation, or tolerance; retain v35 receipt stabilization, same-input replay, and authentic finalization with omission/repair/uncharged/bypass poisons. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-030` | Bind the finalization-equivalent endpoint map as the sole one-Stage-3-call physical source for every shared-budget-charged coupled evaluation, with canonical proposed snow and first-node soil coordinates, bit-exact preservation of every deeper soil layer including carry/custody, non-provisional carrier posture, exact receipt replay, and independent authentic-finalization equality; reject a one-layer soil restriction, deeper-layer rerounding or zero-carry substitution, provisional-map admission, duplicate/uncharged physics, repair, rollback mutation, or private publication. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-031` | Bind distinct exact source and soil-target transactions in V2 soil-energy operand construction, validate ingress/receipts against the source, validate prepared soil against the target, seal both into each operand digest, and reject every zero/stale/foreign/swapped/inferred/out-of-order/support/owner/receipt/digest substitution before acceptance. | V2 soil-energy operand consumer |
| `OBL-SNOWENERGY-C-032` | Bind the exact four-window parity-monotone active-set trigger, tolerance-independent strict root-drift descent, exact static/promoted-root/phase/side/cadence joins, shared-budget reserve refusal, unchanged physical solve, authentic receipt replay/finalization, full poisons, rollback, and no private publication. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-033` | Bind the exact four-window one-way canonical enthalpy-boundary bracket trigger, constant exact water, independently reconstructed single adjacent phase crossing, exact static/promoted-root/side/cadence joins, shared-budget reserve, unchanged physical solve, authentic receipt replay/finalization, poisons, rollback, and no private publication. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-034` | Bind exact authentic cold-content export into canonical W/H endpoint reconstruction and every ordered private nonlatent contraction; retain zero-export byte identity, endpoint closure, shared budget, authentic replay/finalization, rollback, and omission/substitution/order/closure poisons. | covered fixed-point consumer |
| `OBL-SNOWENERGY-C-035` | Bind the typed V2 numerical-coordinate projection through same-support fixed-point validation with exact prepared-owner, transaction/predecessor/support, receipt-chain, authority/set, topology/order, top-layer zero-carry and lower-layer bit preservation; retain ordinary base reconstruction and refuse erased/mixed posture, substitutions, credits, sequential use, acceptance, installation, publication, or rollback mutation. | V2 private projected-base fixed-point consumer |
| `OBL-SNOWENERGY-C-037` | Bind complete carried-root polishing through the unchanged physical residual/Jacobian/trust map, strict exact merit descent, sub-tolerance-only stationary transition, three/two/one shared-budget reserve roles, unchanged exact receipt chaining, and one protected independent same-input replay; reject `F(x)-x`, tolerance/digest repair, stale rejected artifacts, budget reset/slot theft, and private/probe publication. | covered coupled-root polishing and authentic receipt consumer |
| `OBL-SNOWENERGY-C-038` | Preflight every safeguarded finite-difference step against exact coordinate cardinality plus one mandatory trust trial and the unchanged downstream reserve before any column charge; preserve per-map guards and exact V35 receipt/replay, and reject partial-step work, overflow, budget reset/enlargement/separation, stale artifacts, rollback mutation, or private publication. | covered safeguarded solver and authentic receipt consumer |
| `OBL-SNOWENERGY-C-039` | Bind native-V2 atomic complete-owner installation to one typed exact posture: mutually equal vegetation/LSE/BGC source owners plus soil state sealed to target, with either exact source==target or exact authenticated soil predecessor==source. Reject missing/foreign/swapped/out-of-order identities, source-owner disagreement, inferred adjacency, custody substitution, rollback mutation, or private publication before install. | V2 soil accepted-owner atomic installation consumer |
| `OBL-SNOWENERGY-C-040` | Carry one explicit authenticated prepared-beginning source/soil-target authority through the real native-V2 fixed-point final install while retaining strict same-ID generic installation. Validate the authoritative resident, prepared target/predecessor/support/receipt, accepted result, state/layer target seals, and orchestrator seals before atomic mutation; refuse authority erasure, generic split admission, substitution, adjacency inference, rollback mutation, or private publication. | V11 covered fixed-point finalizer and V2 soil accepted-owner consumer |
| `OBL-SNOWENERGY-C-041` | Bind one opaque three-domain authority for repeated same-parent native-V2 soil installation: mutually equal outer source owners, exact authenticated resident/predecessor bundle, exact contiguous prepared target, and unchanged source/target operand authority. Reconstruct it at install, preserve generic and V48 strictness, and reject missing/foreign/stale/swapped/substituted custody, inferred adjacency, owner rebasing, rollback mutation, or private publication. | V11 covered fixed-point finalizer and V2 soil accepted-owner atomic installation consumer |
| `OBL-SNOWENERGY-C-042` | Bind the V49 mutually equal candidate ending source to the explicit validated covered V8 envelope transaction at the real finalizer. Retain the heterogeneous authenticated beginning solely for exact soil resident/prepared custody; reject beginning mutual-source validation, envelope/source/owner/receipt substitution, inferred adjacency, rollback mutation, or private publication. | V11 covered V8 envelope/fixed-point finalizer and V2 soil accepted-owner consumer |
| `OBL-SNOWENERGY-C-043` | Bind the exact R132 single adjacent canonical phase crossing plus post-crossing alternating strict contraction trigger under unchanged V41 static/water/side/cadence/raw-owner/shared-budget guards. Dispatch only the unchanged authentic coupled solver and reject pre-cross reversal, predicate reversal/skip/recross, nonalternating/noncontracting/equal/stagnant/nonfinite corrections, substitution, rollback mutation, or private publication. | covered fixed-point eligibility and authentic coupled-solver consumer |
| `OBL-SNOWENERGY-C-044` | Add exactly one ordered per-lane snow-candidate CN heat coordinate in `J m^-2`, positive into snow. Only `PrivateTrial` feeds coordinate Q exactly once through the typed unpublished CN operand as snow `+Q` and soil `-Q`; authentic probes/replay consume the supplied sealed receipt Q unchanged and never overwrite or reseal it from the coordinate. Reconstruct physical endpoint heat from every same charged map and solve `R_Q=Q-Q_physical` under the unchanged lane energy tolerance. Preserve shared maximum 96, complete-step preflight, V35 exact receipt stabilization/replay/finalization, rollback, and no-publication; reject omission, reorder, sign/substitution/static-geometry/branch/budget/replay poison, receipt repair, averaging, `F(x)-x`, digest residual, or uncharged physics. | covered coupled physical evaluator, solver, and authentic receipt consumer |
| `OBL-SNOWENERGY-C-045` | Assemble every fresh active-set or retained-legacy V52 initial coordinate with Q reconstructed from the exact already-produced endpoint Stage 3 and soil candidates associated with solver dispatch, under exact receipt lane/support/topology/node/custody/seal order. Reject cross-map accepted/retained/predecessor receipt Q, reorder/cardinality/nonfinite substitution, or any added physical evaluation/charge; preserve non-Q seed selection and every V52 residual, tolerance, budget, receipt replay/finalization, rollback, and no-publication guard. | covered coupled physical seed assembler |
| `OBL-SNOWENERGY-C-046` | On only a proven exact receipt cycle of at most three members, reconstruct every member's complete own-artifact W/H/rho plus own-receipt Q and exact-carry soil E/T endpoint vector; atomically reserve all member full-map charges plus replay, evaluate in first-seen order, and admit only an unchanged-tolerance/branch/side/custody exact receipt fixed point followed by exact same-input/same-coordinate replay. Reject mixing, Q-only or arbitrary-ULP search, no-solution, budget, projection, residual, branch, receipt, replay, rollback, or publication poison. | covered authentic receipt-cycle endpoint witness consumer |
| `OBL-SNOWENERGY-C-047` | At the first unchanged-tolerance private root, hard-validate Q shape/finiteness/canonical endpoint-receipt lineage. Return zero-charge `NotApplicable` with root and budget unchanged when unresolved-lane count, positive same-domain interval, checked cardinality, or complete interval-plus-probe/replay capacity is ineligible, then continue unchanged V45 polishing. Otherwise commit immediately before the first charge, exhaustively evaluate every candidate exactly once, retain the first exact positive-zero `R_Q` own-output witness, and fail typed without fallback on every post-commit error or no witness. Require unchanged exact whole-receipt stabilization, replay, and finalization; reject repair, sparse search, rollback mutation, authentic Q substitution, or publication. | covered private coupled-root polish and authentic receipt consumer |
| `OBL-SNOWENERGY-C-048` | Before V55, admit the temperature-primary specialization only from exact authentic strictly frozen/noncrossing Stage 3 authority. Reconstruct exact snow enthalpy from `(W,T_s)` with round-nearest-even high plus exact dyadic carry; derive unchanged CN Q once; preserve exact ledger tolerance; seal only fresh authentic Stage 3 material plus ordered carry as one compound owner and additive carry receipt; stabilize the complete CN/carry receipt set and require independent exact replay/finalization. Persist committed, pending, and in-progress compound-owner chronology in restart V5; migrate V4 only with exact zero carry and refuse nonzero-carry downgrade. Reject transient V54/V55 witness promotion, carry/receipt repair, uncharged maps, rollback mutation, or publication. | covered frozen temperature-primary solver, Stage 3 owner/finalizer, receipt consumer, and persisted restart |
| `OBL-SNOWENERGY-C-049` | Treat only finite nonnegative external liquid through `1.0e-12 kg m^-2 OFE-ground` as neutral to frozen-specialization eligibility while retaining its exact bits and exact contribution in every physical mass/energy operand, receipt, replay, finalization, and independent closure reconstruction. Any corresponding refreeze must equal that bounded operand, leave exact-zero ending liquid, and preserve the frozen phase/event predicates while its exact latent energy remains ledgered. Allow zero-charge coordinate conversion from the first tolerance-closed legacy root immediately before V55 with the already-used shared budget; commit before the first charged V56 map and prohibit fallback thereafter. Reject negative, nonfinite, or one-bit-above amounts, unmatched refreeze, ending liquid/melt/event, operand normalization or omission, uncharged physics, budget reset, V55 dispatch after V57 commit, rollback mutation, diagnostics, or publication. | covered frozen-specialization eligibility, post-root transition, physical ledger, and authentic receipt consumer |

## Binding Exposure Index

The package rows below map active package-local binding residue through version
16 to authority promoted into this canonical core.

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `SNOWENERGY-CHILD2C-CARRIER` | `docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-036, INV-SNOWENERGY-037, INV-SNOWENERGY-038, INV-SNOWENERGY-039, INV-SNOWENERGY-040, OBL-SNOWENERGY-P-010, OBL-SNOWENERGY-C-017` | `flagged-binding-addition` | Shared carrier topology, sealed exposure, weighted component longwave, typed flux lineage, and wrong-regime/scope rejection. |
| `SNOWENERGY-V15-OFE-GROUND-LANE` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-042, OBL-SNOWENERGY-C-018` | `flagged-binding-addition` | Direct user selection of one-column-per-lane OFE-ground storage under `TOL-SNOWENERGY-002`, complete typed tile-surface flux aggregation without covered-subset renormalization, common lane snow state, terminal identity, and topology-bound restart posture; dual review and verification required. |
| `SNOWENERGY-V16-COVERED-CONVERGENCE-RESTART` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-043, INV-SNOWENERGY-044, OBL-SNOWENERGY-C-019, OBL-SNOWENERGY-C-020` | `flagged-binding-addition` | Separates covered fixed-point comparisons under `TOL-SNOWENERGY-003`, reconstructs candidate fingerprints, and holds additive restart until a normative lane-receipt V2 wire and complete topology/owner replay join are admitted. |
| `SNOWENERGY-V32-TERMINAL-PHASE-VAPOR-ACTIVE-SET` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `historical` | `maps-to-existing-INV` | `INV-SNOWENERGY-055, INV-SNOWENERGY-056, OBL-SNOWENERGY-C-023, OBL-SNOWENERGY-C-024` | `flagged-binding-addition` | Retains v31/v32 midpoint, vapor-root, and branch-entry authority as diagnostic/refusal evidence. Version 33 supersedes their production-control role only after the exact active-set root/interface/branch-entry/opposite-authentic/reset transition record repeats under unchanged joins. |
| `SNOWENERGY-V33-PHASE-CONSISTENT-COUPLED` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-057, OBL-SNOWENERGY-C-025` | `flagged-binding-addition` | The exact transition reset invokes a private reduced solve through concrete physical residual carriers and algebraic constraints under one shared 96-evaluation budget; only fresh `CoupledAuthentic` replay/reseal may bypass Picard equality and proceed to unchanged finalization/publication guards. |
| `SNOWENERGY-V34-STABLE-MONOTONE-COUPLED` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-058, OBL-SNOWENERGY-C-026` | `flagged-binding-addition` | Eight stable monotone raw authentic maps may invoke the unchanged v33 physical solver under exact static receipt/phase/carry-authority-and-representation joins and exact evolving high-plus-carry reconstruction, without requiring physical receipt digests or carry coordinates to remain byte-equal, under the same already-charged 96-evaluation budget; pre-root refusal destroys private trials and resumes raw authentic Picard within the remaining budget. |
| `SNOWENERGY-V35-AUTHENTIC-RECEIPT-STABILIZATION` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-059, OBL-SNOWENERGY-C-027` | `flagged-binding-addition` | A coupled root enters charged immutable-input receipt probes until exact input/output receipt equality, then one independent same-input replay must reproduce exact residuals, artifacts, and receipts before admission; cross-input equality and probe retention are forbidden. |
| `SNOWENERGY-V36-GEOMETRY-COMPLETE-COUPLED` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-060, OBL-SNOWENERGY-C-028` | `flagged-binding-addition` | Adds one terminal density coordinate, canonical `I/rho` thickness, and unchanged constitutive `R_rho` to the generalized physical solve under the same budget and v35 receipt/finalization path. |
| `SNOWENERGY-V37-DERIVED-THICKNESS-CLOSURE` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-061, OBL-SNOWENERGY-C-029` | `flagged-binding-addition` | Requires canonical derived `R_z` to participate in merit and admission under the unchanged depth bound while retaining the v36 coordinate vector, one charged map, v35 receipt replay, and authentic finalization. |
| `SNOWENERGY-V38-FINALIZATION-EQUIVALENT-MAP` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-062, OBL-SNOWENERGY-C-030` | `flagged-binding-addition` | Requires every charged coupled residual/replay to execute the authentic finalization-equivalent endpoint map directly with one Stage 3 call, projects only each OFE's first snow-coupled soil node while preserving deeper layers and exact carries bit-for-bit, and retains exact receipt stabilization plus an independent exact finalization replay. |
| `SNOWENERGY-V39-SOIL-ENERGY-TRANSACTION-SEPARATION` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-063, OBL-SNOWENERGY-C-031` | `flagged-binding-addition` | Separates the exact outer surface-ingress/source transaction from the exact authenticated V2 soil-target transaction in soil-energy operand custody and seals both identities into each operand digest without changing physical energy or acceptance semantics. |
| `SNOWENERGY-V40-PARITY-MONOTONE-ACTIVE-SET` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-064, OBL-SNOWENERGY-C-032` | `flagged-binding-addition` | Four consecutive exact-static nonexact rolling reset windows with strictly decreasing finite positive root drift may invoke the unchanged finalization-equivalent solver early enough to retain the same shared-budget minimum solve/replay path; no drift threshold admits a root. |
| `SNOWENERGY-V41-ONE-WAY-PHASE-BOUNDARY` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-065, OBL-SNOWENERGY-C-033` | `flagged-binding-addition` | Four exact-static rolling windows with bit-identical water and a strictly one-way enthalpy chain crossing exactly one adjacent canonical phase boundary may invoke only the unchanged solver under the same reserve; the bracket is not a tolerance, projection, or root admission. |
| `SNOWENERGY-V42-COLD-CONTENT-EXPORT-COORDINATE` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-066, OBL-SNOWENERGY-C-034` | `flagged-binding-addition` | Adds the exact already-physical authentic cold-content-export operand to canonical support enthalpy and ordered private nonlatent contraction, preserving zero-export bytes and every unchanged solver, ledger, replay, rollback, and publication guard. |
| `SNOWENERGY-V43-PROJECTED-BASE-CUSTODY` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-067, OBL-SNOWENERGY-C-035` | `flagged-binding-addition` | Preserves the V38 top-layer numerical-coordinate sibling as a typed private fixed-point posture with exact projection custody instead of misclassifying it as ordinary base unpublished physics; every base, sequential, acceptance, installation, rollback, and publication guard remains unchanged. |
| `SNOWENERGY-V44-UNCOMMITTED-LSE-CLOSURE-POSTURE` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-068, OBL-SNOWENERGY-C-036` | `flagged-binding-addition` | Defers weighted-OFE closure only for the charged uncommitted private LSE exchange, then requires unchanged strict receipt-probe/replay/finalization closure; projected soil coordinates remain exact-once CN operands and cannot be double-applied to the frozen Stage3-covered V8 soil control volume. |
| `SNOWENERGY-V45-AUTHENTIC-RECEIPT-ROOT-POLISHING` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-069, OBL-SNOWENERGY-C-037` | `flagged-binding-addition` | Continues a tolerance-closed private root only through strict descent of the existing physical residual merit, carries matching artifacts, reserves authentic probe/replay capacity inside the unchanged shared 96 cap, and retains exact receipt equality plus independent same-input replay as the sole admission. |
| `SNOWENERGY-V46-COMPLETE-STEP-BUDGET-PREFLIGHT` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-070, OBL-SNOWENERGY-C-038` | `flagged-binding-addition` | Requires dimension-complete capacity for every finite-difference Jacobian plus one trust trial before the first column charge, preventing an unusable partial polishing tail while retaining the unchanged solver, exact receipt/replay, and maximum 96. |
| `SNOWENERGY-V47-ATOMIC-COMPLETE-OWNER-TRANSACTION-POSTURE` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-071, OBL-SNOWENERGY-C-039` | `flagged-binding-addition` | Admits native-V2 accepted-owner installation only for exact same-ID first-child custody or an exact authenticated soil-target successor whose predecessor equals the mutually equal outer source owners; no numeric adjacency, substitution, or partial install. |
| `SNOWENERGY-V48-FIXED-POINT-FINAL-INSTALL-AUTHORITY` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-072, OBL-SNOWENERGY-C-040` | `flagged-binding-addition` | Propagates explicit authenticated prepared-beginning source/soil-target authority through the real fixed-point final install while leaving generic/public installation same-ID-only. |
| `SNOWENERGY-V49-MULTI-CHILD-PREPARED-INSTALL-AUTHORITY` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-073, OBL-SNOWENERGY-C-041` | `flagged-binding-addition` | Binds exact outer source, independently advancing authenticated resident/predecessor, and exact prepared target through repeated same-parent final installs without adjacency inference or owner rebasing. |
| `SNOWENERGY-V50-ENVELOPE-SOURCE-TRANSITION-AUTHORITY` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-074, OBL-SNOWENERGY-C-042` | `flagged-binding-addition` | Anchors V49's mutually equal candidate ending source to the already validated covered V8 envelope transaction while preserving the exact heterogeneous constitutive beginning as soil custody only. |
| `SNOWENERGY-V51-POST-CROSSING-CONTRACTION` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-075, OBL-SNOWENERGY-C-043` | `flagged-binding-addition` | Extends V41 only for the exact r132 single canonical crossing followed by alternating, strictly shrinking within-phase corrections, retaining every solver, budget, closure, receipt, event, floor, rollback, and publication guard. |
| `SNOWENERGY-V52-CN-HEAT-COORDINATE` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-076, OBL-SNOWENERGY-C-044` | `flagged-binding-addition` | Adds the omitted continuous snow-candidate CN heat coordinate/equation to the charged coupled map while retaining exact receipt stabilization, replay, finalization, shared budget, floor, closure, rollback, and no-publication. |
| `SNOWENERGY-V53-SAME-MAP-CN-HEAT-SEED` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-077, OBL-SNOWENERGY-C-045` | `flagged-binding-addition` | Seeds the V52 Q coordinate from the exact already-produced endpoint candidate pair associated with solver dispatch, without changing non-Q seeds or adding a map/budget charge; all V52 solve and authentic admission rules remain unchanged. |
| `SNOWENERGY-V54-REPRESENTABLE-RECEIPT-CYCLE-WITNESS` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-078, OBL-SNOWENERGY-C-046` | `flagged-binding-addition` | Searches only the finite cycle's already-produced exact endpoint artifacts with their own receipts through charged full authentic maps and exact replay; no receipt or numerical repair is admitted. |
| `SNOWENERGY-V55-PRIVATE-Q-LATTICE-WITNESS` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-079, OBL-SNOWENERGY-C-047` | `flagged-binding-addition` | Exhaustively evaluates the finite one-lane private Q lattice at the first tolerance-closed root before V45 polish, with atomic shared-budget reservation and exact positive-zero physical Q closure; authentic receipt stabilization, replay, and finalization remain unchanged and exact. |
| `SNOWENERGY-V56-FROZEN-TEMPERATURE-PRIMARY-CARRY` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-080, OBL-SNOWENERGY-C-048` | `flagged-binding-addition` | Dispatches a strictly frozen/noncrossing temperature-primary solve before V55, derives exact enthalpy high-plus-carry and unchanged CN Q, and admits only a fresh authenticated compound snow owner after whole-receipt stabilization, independent replay, strict finalization, and versioned restart custody. |
| `SNOWENERGY-V57-BOUNDED-LIQUID-ELIGIBILITY` | `docs/work-packages/20260830-workspace-gate-hold-lift-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-081, OBL-SNOWENERGY-C-049` | `flagged-binding-addition` | Keeps at most `1.0e-12 kg m^-2` external-liquid roundoff eligibility-neutral without changing its physical ledger operand and permits a zero-charge, same-budget temperature-primary transition from the first legacy root immediately before V55. |
| `SNOWENERGY-V17-PRECIPITATION-CUSTODY` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-045, INV-SNOWENERGY-046, INV-SNOWENERGY-047, OBL-SNOWENERGY-P-011, OBL-SNOWENERGY-C-021` | `flagged-binding-addition` | Seals the ordered precipitation phase-parcel set, binds open-versus-covered liquid exclusivity and solid bypass, and requires mass/advection same-set reconstruction on the OFE-ground lane basis. |
| `SNOWENERGY-V18-SNOW-SOIL-BOUNDARY` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-048, INV-SNOWENERGY-049, INV-SNOWENERGY-050, OBL-SNOWENERGY-P-012, OBL-SNOWENERGY-C-022` | `flagged-binding-addition` | Binds one OFE/lane bottom-snow-to-first-soil-node Crank--Nicolson interface, exact equal/opposite custody, reconstructable receipt, and atomic rollback without tile aggregation or duplication. |
| `SNOWENERGY-EB02-AUTHORITY` | `docs/work-packages/20260730-snow-surface-eb-02-subcanopy-longwave-contract-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-001, INV-SNOWENERGY-002, INV-SNOWENERGY-003, INV-SNOWENERGY-004, INV-SNOWENERGY-005, INV-SNOWENERGY-006, INV-SNOWENERGY-007, INV-SNOWENERGY-008, INV-SNOWENERGY-009, INV-SNOWENERGY-010, INV-SNOWENERGY-011, INV-SNOWENERGY-012, INV-SNOWENERGY-013, INV-SNOWENERGY-014` | `none` | Package-local source reconciliation and analytical artifacts are evidence; all binding equations, guards, and obligations are in this canonical contract. |
| `SNOWENERGY-EB03-COMPOSITION` | `docs/work-packages/20260730-snow-surface-eb-03-shared-thermal-energy-composition-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-015, INV-SNOWENERGY-016, INV-SNOWENERGY-017, INV-SNOWENERGY-018, INV-SNOWENERGY-019` | `none` | Package evidence binds the Stage 3 provider, orthogonal selectors, and mass/energy composition implemented by version 2. |
| `SNOWENERGY-EB03A-COUPLING` | `docs/work-packages/20260730-snow-surface-eb-03a-active-layer-thermal-coupling-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-020, INV-SNOWENERGY-021, INV-SNOWENERGY-022, INV-SNOWENERGY-023, INV-SNOWENERGY-024, INV-SNOWENERGY-025` | `none` | Package evidence must implement and verify the version-3 active thermal control volume and coupled substep solver. |
| `SNOWENERGY-EB04C-THERMAL-DOMAIN` | `docs/work-packages/20260731-snow-surface-eb-04c-thin-pack-thermal-domain-closure-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-023, INV-SNOWENERGY-024, INV-SNOWENERGY-026` | `dual review and verification required` | Package evidence must implement and verify the exact minimum-resolved-mass branch without importing libsnobal's phase conversion or weakening typed guards. |
| `SNOWENERGY-EB04D-LAYER-RECONCILIATION` | `docs/work-packages/20260731-snow-surface-eb-04d-layer-thickness-reconciliation-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-021, INV-SNOWENERGY-027` | `dual review and verification required` | Package evidence must separate mass-unit lifecycle selection from meter-unit aggregate residual tolerances and preserve coupled layer state. |
| `SNOWENERGY-EB04S-TOLERANCE-RECONCILIATION` | `docs/work-packages/20260801-snow-surface-eb-04s-authority-reconciliation-retained-adjudication-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-017, INV-SNOWENERGY-018, INV-SNOWENERGY-027, INV-SNOWENERGY-028` | `dual review and verification required` | Result-blind authority reconciliation binds the SWE-to-area-mass equivalence while preserving distinct vapor-aggregation and layer-lifecycle predicates. |
| `SNOWENERGY-21N-MELT-OWNER` | `docs/work-packages/20260804-snow-coe-stage3-melt-owner-authority-reconciliation-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-029, INV-SNOWENERGY-030, OBL-SNOWENERGY-P-006, OBL-SNOWENERGY-C-013` | `dual review and verification required` | Stage 3 is the sole future melt owner; the unchanged CoE runtime remains compatibility-only until complete energy, residual-snow, same-substep liquid, real-consumer, and cutover gates pass atomically. |
| `SNOWENERGY-STAGE3-COMPLETE-CARRIER` | `docs/work-packages/20260805-snow-stage3-complete-carrier-shadow-melt-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-029, INV-SNOWENERGY-030, INV-SNOWENERGY-031` | `dual review and verification required` | User authority binds explicit CLIGEN virtual-instrument geometry and lifts the turbulent-input authority hold; carrier and shadow evidence remain required before atomic cutover. |
| `SNOWENERGY-STAGE3-EVOLVING-CARRIER-PLAUSIBILITY` | `docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-017, INV-SNOWENERGY-029, INV-SNOWENERGY-032, OBL-SNOWENERGY-P-007, OBL-SNOWENERGY-C-014` | `dual review and verification required` | Distinguishes evaluation-only raw vapor/latent opportunity from actual bounded sequential transfer; no production correction or persistence authority. |
| `SNOWENERGY-STAGE3-WIND-SOURCE-CUSTODY` | `docs/work-packages/20260807-snow-stage3-wind-source-custody-and-exposure-authority-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-031, INV-SNOWENERGY-033` | `dual review and verification required` | Separates nominal source height, raw CLI wind, PMET-local adjustment, and virtual Stage 3 geometry; provider recovery directly proves retained output equality and statically reconstructs the local path while deployed/server and exposure authority remain missing. |
| `SNOWENERGY-TERMINAL-ENTHALPY-EVENT` | `docs/work-packages/20260807-snow-terminal-enthalpy-event-numerics-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-034, INV-SNOWENERGY-041, OBL-SNOWENERGY-P-008, OBL-SNOWENERGY-C-015` | `flagged-binding-addition` | Admits evaluation-only shallow-pack enthalpy/error-control/event mechanics while keeping liquid, energy, and remaining-time receiving-surface custody censored. |
| `SNOWENERGY-TERMINAL-RECEIVER-TRANSACTION` | `docs/work-packages/20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001/` | `historical` | `maps-to-existing-INV` | `INV-SNOWENERGY-035, OBL-SNOWENERGY-P-009, OBL-SNOWENERGY-C-016` | `historical` | Preserves the default-off receiver evidence superseded by version 22. |
| `SNOWENERGY-ADAPTIVE-COMPOSITIONAL-V22` | `docs/work-packages/20260826-snow-stage3-adaptive-compositional-microstepping-001/` | `active` | `maps-to-existing-INV` | `INV-SNOWENERGY-048, INV-SNOWENERGY-049, INV-SNOWENERGY-050` | `flagged-binding-addition` | Owner-selected bounded vapor, same-step phase equilibrium, adaptive complete-owner composition, and accepted-microstep terminal transfer. Rejected versions 19-21 remain historical. |

## Adaptive compositional Stage-3 phase-equilibrium successor

Version 22 supersedes `INV-SNOWENERGY-034` and the terminal portions of
`INV-SNOWENERGY-029/030/035` where they prescribe melt-before-deposition,
continuous root localization, sub-60-second attempts, censored terminal energy, or
an evaluation-only/default-off receiver. Their historical evidence remains
binding for the behavior it observed and as negative regression evidence; it is
not active production authority after cutover. Versions 19 through 21 remain historical rejected candidates and are not restored.

For every accepted Stage-3 microstep let beginning ice, retained liquid, and
nonnegative cold content be `I0`, `L0`, and `C0`; actual signed vapor mass be
`V`; external liquid entering snow custody be `Lin`; complete bounded energy be
`Q`; and fusion latent heat be `Lf`. Define `D=max(V,0)` and
`S=min(max(-V,0),I0)`. Raw sublimation opportunity and its latent energy are
truncated together before `Q` is assembled. Deposition is retained exactly.
The vapor latent term is already one component of `Q` and is never credited a
second time.

The phase projection uses enthalpy relative to all water being ice at 0 C:

    W = I0 + L0 + D - S + Lin
    E = -C0 + Lf * (L0 + Lin) + Q

If `E<0`, the endpoint is `(I1,L1,C1,U)=(W,0,-E,0)`. If
`0<=E<Lf*W`, it is `(W-E/Lf,E/Lf,0,0)`. If `E>=Lf*W`, it is
`(0,W,0,max(E-Lf*W,0))`; equality produces exact zero ice and exact zero
unallocated energy. With `Lpre=L0+Lin`, define
`melt=max(L1-Lpre,0)` and `refreeze=max(Lpre-L1,0)`.

The accepted endpoint independently closes:

    I0 + D + refreeze - S - melt - I1 = 0
    L0 + Lin + melt - refreeze - L1 = 0
    Q - (C0-C1) - Lf*melt + Lf*refreeze - U = 0

Material positive `I1` with material positive `U` is forbidden. Positive solid
is never tolerance-deleted. Same-microstep deposited ice participates in phase
equilibrium; after disappearance atmospheric vapor belongs to the snow-free
LSE owner and only authorized solid precipitation may recreate snow.

`INV-SNOWENERGY-048` — Actual vapor custody is a typed record carrying raw and
actual mass, raw and actual latent energy, positive finite specific latent heat
for nonzero transfer, and deposition/sublimation/none disposition. Complete
energy replaces raw latent opportunity with actual bounded latent energy once.
Raw-as-actual, mass-only truncation, energy-only truncation, sign inversion,
vapor-as-liquid, or duplicate latent credit fails closed.

`INV-SNOWENERGY-049` — Every accepted adaptive step installs the more-resolved
composed complete-owner result. Positive supports are exact integer multiples
of the 60-second (`60_000_000_000 ns`) temporal floor; stable ordinary
supports must accept steps substantially larger than that floor. Direct and composed trials reevaluate every
state-dependent carrier and owner from their own immutable beginning state and
exact projected support. Trial states are unpublished. Physical comparison uses
the named `OPENWEPP_STAGE3_ADAPTIVE_OWNER_TOLERANCE_V1` authority in
`TOL-SNOWENERGY-004`, owner-specific absolute/relative tolerances, and exact discrete predicates; a
discrete mismatch rejects and refines the joint step.

For covered supports strictly above the 60-second floor, the bounded outer
solve applies under-relaxed Picard to the current Stage 3 and soil iterates,
`x_next=x+w*(F(x)-x)`, with
`w=max(0.25,min(0.5,120 s/support_duration))` and at most 96 iterations. The
exact 60-second floor begins with the raw authentic Picard update. It switches
to `w=0.5` only after authentic Stage 3 candidates exhibit an `A/B/A` cycle:
candidate `n` and `n-2` must pass the existing `TOL-SNOWENERGY-003` comparison,
including exact density and every structural/discrete predicate, while
candidate `n` and `n-1` do not. The switch is monotonic for the remainder of
that unpublished bounded solve. Acceptance still requires every
native-unit `TOL-SNOWENERGY-003` norm plus LSE and boundary norms; exhaustion
is a typed refinable trial failure and never publishes a candidate. The exact
Stage-3 lower-boundary/column operand-join closure is likewise refinable only
above the floor because it can identify a coarse support spanning a phase
transition; the same failure at the floor remains fail-closed.

The same candidate weight applies when the converged provisional map is
authentically rebuilt for finalization but does not yet pass the unchanged LSE,
boundary, and Stage 3 convergence predicates. That finalization mismatch is
another unpublished iteration of the same bounded coupled solve, not authority
to discard the current damped iterate and restart from the raw final candidate.
Only Stage 3 continuous state admitted by the relaxation guards is blended;
the finalization LSE and complete boundaries remain authentic map operands and
soil retains its converged authentic candidate. If relaxation is declined, the
raw authentic final candidate remains the next iterate. Publication still
requires an authentic finalization image that passes every unchanged
predicate; a relaxed intermediate can never be accepted or replayed.
After a damped finalization restart, the first provisional Stage 3 comparison
that enters `TOL-SNOWENERGY-003` takes one further support-scaled Picard update
before another finalization rebuild. This single stabilization update prevents
the tolerance crossing from disabling damping and immediately recreating the
same finalization residual. It is active only where relaxation is already
enabled, introduces no new tolerance, and does not bypass any authentic-map
comparison.

Relaxation is prohibited across any schema, terminal-model, layer-cardinality,
settling, initial/input, per-layer represented-mass, or aggregate
resolved/terminal/dormant posture change. Density is never interpolated: every
unpublished relaxed layer copies the authentic candidate density bit-for-bit,
and thickness is reconstructed from relaxed mass and that exact candidate
density. A density difference remains nonconverged under
`TOL-SNOWENERGY-003` and therefore cannot be accepted until an authentic map
image matches; it does not disable damping of otherwise-admitted continuous
mass/energy coordinates. Cumulative water and energy close independently. Soil transaction
lineage is exact and both nested owner digests are canonically resealed. Every
snow--soil heat receipt is joined to the actual numerical iterate before its
equal/opposite credit is consumed. A failed reconstruction, posture, digest,
or receipt join declines relaxation and retains typed refinement.

Final snow--soil receipt sealing retains bit-for-bit the equal/opposite heat
that the snow and soil solvers actually consumed. It then reconstructs the
Crank--Nicolson receipt from the installed endpoint temperatures. Only when
the reconstructed-versus-consumed energy residual is finite and at most
`1e-9 J m^-2` and both endpoint-temperature residuals are finite and at most
`1e-8 K` may the consumed receipt bind the exact installed candidate digests
and reseal its complete hash. This bounded nonlinear termination roundoff is
`TOL-SNOWENERGY-005`; it is three orders of magnitude smaller than the
unchanged `1e-6 J m^-2` physical-ledger closure threshold and does not change
the applied snow debit or soil credit. A larger residual consumes another
bounded fixed-point iteration and cap exhaustion remains fail-closed. Final
publication remains exact owner-identity replay and receipt reseal from the
admitted map image; an unpublished numerical iterate never becomes accepted
owner authority.

When an accepted terminal support is the ordered composition of multiple exact
physical children, its precipitation parcel set retains every child parcel
exactly once. Child supports are contiguous and cover the accepted envelope;
lane topology, destination identities, and OFE basis are identical. Parcel
support is rebound to the enclosing support, and semantic ordinals are assigned
in physical-child order within each destination/phase/source route before the
complete canonical set is sorted and resealed. Mass and enthalpy-provider bits,
source identity, producer beginning identity, and destination remain unchanged.
The accepted physical ledger consumes this enclosing set, while final boundary
receipts retain the distinct final-child set. Omission, duplication, order or
topology substitution, noncontiguous support, or a changed physical operand
fails before publication.

The terminal snow--soil receipt chain is summed in exact physical-child order.
Its aggregate must equal the terminal event's independently accumulated heat;
only finite absolute binary64 regrouping residual within
`TOL-SNOWENERGY-005` (`1e-9 J m^-2`) is admitted and recorded by the transient
qualification audit. One bit over the bound fails closed. The accepted heat
remains the exact ordered receipt sum, and the snow/soil equal-opposite ledger
threshold is unchanged.

`INV-SNOWENERGY-052` — Exact-floor period-two contraction is a conditional
numerical iterate policy only. At `60_000_000_000 ns`, raw Picard remains the
default; after and only after the authentic `A/B/A` predicate above, the
candidate weight is exactly `0.5` for the remainder of that solve. A raw
convergent map never activates damping. The authentic candidate density and
every schema, event, topology, posture, settling, initial/input, and receipt
identity remain unblended and exact; mass/energy closure is reconstructed on
every unpublished iterate. Final acceptance and replay use only an authentic
map image. A discrete change, invalid reconstruction, receipt mismatch, or
96-iteration exhaustion remains fail-closed, and no support below the exact
floor is admitted.

`INV-SNOWENERGY-053` — The persistent per-layer `refrozen_liquid_m` field is
material-provenance history within one trial factorization, not an independent
prognostic truncation-error coordinate between direct `H` and composed
`H/2 + H/2` trials. Each trial must still carry, validate, fingerprint, and
replay the complete field, and the accepted composed owner must install and
persist its exact value. The adaptive cross-factorization scalar projection
therefore excludes only `snow.lanes[*][1].layers[*].refrozen_liquid_m`.

This classification does not exclude or loosen any current snow mass, liquid,
cold-content, temperature, cumulative-water, cumulative-energy, phase, event,
topology, custody, or receipt coordinate. Before committed publication, actual
accepted refreeze remains independently reconstructed from beginning and ending
ice plus snowfall, deposition, sublimation, and melt under the unchanged
`1e-9 m` physical-ledger threshold; the liquid and energy ledgers remain
independently closed. A changed accepted-path tracer still changes canonical
owner identity and persisted restart bytes. No adaptive diagnostic surface is
persisted.

`INV-SNOWENERGY-054` — A nonconverged authentic finalization rebuild advances
the unpublished Stage 3 iterate with the same support-scaled candidate weight
as outer Picard. The convex update is permitted only across the exact schema,
terminal-event, lane, interval, layer, settling, initial/input,
represented-mass, and aggregate phase-posture identities already required for
Picard relaxation. Density is copied bitwise from the authentic candidate and
never interpolated; a density difference remains an exact convergence failure
until authentic-map equality is reached. Every reconstructed intermediate must retain valid
cumulative mass/energy closure. Authentic final LSE and boundary operands are
retained for the next evaluation; soil is not blended across this transition.
If any relaxation guard fails, the raw authentic final candidate is used.
Following an applied finalization restart, the first otherwise-converged
provisional Stage 3 revisit performs one additional guarded update before
finalization can be retried. Neither path changes `TOL-SNOWENERGY-003`, the
96-iteration cap, receipt termination, exact-floor cycle policy, or final
authentic-map replay.

`INV-SNOWENERGY-055` — This exception is narrower than ordinary Picard
relaxation and applies only to an exact-floor terminal-one-volume authentic
open-snow image that is finite but cannot enter the existing `>=200 K`
boundary domain because it lies across the canonical ice/liquid/cold-content
kink from the current unpublished image. Current and authentic images must be
independently reproducible from one immutable beginning owner and two complete
same-support operand images whose identities, source receipts, component sums,
and vapor disposition validate exactly. Form a single coordinated midpoint of
the signed vapor, external-liquid, and ordered energy-component vector. Its
derived `W` and `H` must equal the midpoint of the independently reconstructed
endpoint `W/H` coordinates. Apply the version-22 phase projection once, then
derive ice, liquid, cold content, temperature, thickness, melt, refreeze,
cumulative deposition/sublimation/external liquid/melt/unresolved liquid,
complete energy, cold-energy change, terminal unallocated energy, and the state
fingerprint from the immutable beginning and that midpoint operand image.
Independent interpolation of any persistent component or cumulative ledger is
forbidden. Exact `H=0` remains all ice with zero cold content; exact `H=Lf W`
is all liquid with zero unallocated energy. Crossing the vapor sign/disposition,
exceeding sublimation capacity, nonfinite input, a mismatched support or
identity, failed component or cumulative closure, or a phase projection outside
the canonical domain fails typed without mutation. The raw authentic image is
retained in history, the phase-aware midpoint is marked ineligible for every
convergence/finalization/replay/acceptance/publication branch, and a later fresh
authentic image must pass every unchanged predicate before owner installation.
No diagnostic from this numerical path is persisted.

`INV-SNOWENERGY-056` — Version 32 is a private numerical active-set
localizer, not a new vapor or energy constitutive relation. Let endpoint `0`
be the current unpublished support image and endpoint `1` the raw authentic
image after orienting the pair so their signed actual vapor satisfies
`V_0*V_1<0`. Both endpoints must be finite, independently closed, pure
one-sided (`D>0,S=0` or `D=0,S>0`), and identical in every immutable beginning,
support, structure, event, topology, custody, and receipt identity. Define

```text
alpha_v = -V_0/(V_1-V_0),                    0 < alpha_v < 1
V_alpha = D_alpha = S_alpha = Q_v,alpha = +0
Lin_alpha = Lin_0 + alpha_v*(Lin_1-Lin_0)
Q_k,alpha = Q_k,0 + alpha_v*(Q_k,1-Q_k,0)   for each ordered nonlatent k
Q_alpha = ordered_sum_k(Q_k,alpha)
W_alpha = I0 + L0 + D_alpha - S_alpha + Lin_alpha
H_alpha = -C0 + L_f*(L0+Lin_alpha) + Q_alpha
```

Apply the version-22 phase projection to `(W_alpha,H_alpha)` exactly. The
latent component is deliberately not affine-interpolated. At the captured
`1860..1920 s` support,
`V_0=D_0=+2.12159691239571346e-4 kg m^-2`, `S_0=0`,
`Q_v,0=+649.057936925197964 J m^-2`, while
`V_1=-4.61661230425127085e-3 kg m^-2`, `D_1=0`,
`S_1=+4.61661230425127085e-3 kg m^-2`, and
`Q_v,1=-13081.6326253264015 J m^-2`. The unique binary64 root fraction is
`alpha_v=0.04393657257739406`; affine latent interpolation would retain the
captured wrong value `+45.77845449909091 J m^-2` at zero vapor and is rejected.

If that exact interface is the current private iterate and a later fresh
authentic image is pure one-sided with all exact joins unchanged, the solver
may enter the authentic side with the existing support-scaled bounded weight
`w_p=max(0.25,min(0.5,120 s/support_duration))`:

```text
V_* = w_p V_auth
D_* = max(V_*,0); S_* = max(-V_*,0)
Q_v,* = V_* L_s,auth,                         L_s,auth finite and > 0
Lin_* = Lin_interface + w_p*(Lin_auth-Lin_interface)
Q_k,* = Q_k,interface + w_p*(Q_k,auth-Q_k,interface)  for nonlatent k
Q_* = Q_v,* + ordered_sum_k(Q_k,*)
```

Reconstruct `W_*` and `H_*` from immutable beginning plus these operands and
apply the same canonical phase projection. The interface and entry images
remain explicitly synthetic and retain raw authentic history. A fresh
authentic map evaluation is still required before convergence, finalization,
replay, owner acceptance, persistence, or publication. The branch is
ineligible for same-sign endpoints, either zero endpoint, zero-to-zero entry,
mixed deposition/sublimation, nonfinite or out-of-range root, nonpositive or
unlinked authentic latent heat, missing/reordered components, failed exact
identity/event/topology/custody/receipt/capacity/closure, or
any rollback/publication mutation. Every refusal is typed and atomic.

`INV-SNOWENERGY-057` — Version 33 replaces version-31/32 production control
only after the exact active-set transition-reset sequence
`root/interface -> one-sided branch-entry -> opposite pure-vapor raw-authentic
-> same root/interface/reset coordinates and branch predicates`, on one
event-free covered support with identical immutable beginning, sealed source,
support, event, topology, custody, and receipt joins. The first and later
raw-authentic continuous owner coordinates may change asymptotically and are
not required to compare bitwise equal. Exactness applies to the transition
record, root/reset coordinates, branch predicates, joins, and opposite pure
vapor disposition. The retained 60- and 120-second v32 captures are mandatory
detector vectors. Version-31/32 midpoint, vapor-interface, and branch-entry
images remain historical diagnostic and refusal evidence; none is a physical
residual sample, accepted root, or publication candidate.

For each affected lane, the reduced unknown vector contains only ending total
water and enthalpy plus the coupled first-soil-node endpoint coordinates already
required by the unchanged snow--soil Crank--Nicolson equations. One sealed
`CoveredPhaseConsistentResidualInputsV1` carries immutable beginnings, support,
sources, LSE inputs, CN inputs, receipts, candidate coordinates, and the shared
budget handle. One call to `covered_phase_consistent_residual_evaluate_v1`
performs the complete physical trial and returns one
`CoveredPhaseConsistentResidualEvaluationV1`:

```text
x = (W_1,l, H_1,l, E_soil,1,n, T_soil,1,n)_affected
(I_1,l, L_1,l, C_1,l, U_1,l) = Pi(W_1,l, H_1,l)
R_W,l = W_1,l - W_0,l - DeltaW_physical,l
R_H,l = H_1,l - H_0,l - Q_complete,l
R_E,n = E_soil,1,n - E_soil,0,n - DeltaE_CN+other,n
R_T,n = T_soil,1,n - T_soil_owner(E_soil,1,n, sealed soil state)
Q_v,l = V_l * L_s,l
```

`Pi` is exactly the version-22 piecewise phase projection, including the cold,
mixed-phase, and exact fusion-capacity sides. `DeltaW_physical` is the unchanged
ordered water ledger. `Q_complete` is the unchanged ordered energy ledger,
including linked `Q_v` and the snow debit of the existing snow--soil CN receipt
exactly once. `DeltaE_CN+other` is the existing soil owner's CN storage/flux
equation with the equal/opposite credit exactly once. `T_soil_owner` is the
existing soil enthalpy--temperature relation. Canonical phase and capacity,
covered-LSE balance, CN endpoints and conductance, exact equal/opposite custody,
receipt reconstruction/reseal, and every identity join are algebraic side
constraints: failure refuses the trial before merit. The covered LSE physical
trial supplies dependent carrier state and is not another free coordinate.
Affine, interpolated, surrogate, synthetic, or coordinate-map-difference
`F(x)-x` residuals are forbidden.

`phase_consistent_coupled_solve_v1` is deterministic safeguarded semismooth
Newton with a bounded trust region. Its generalized system uses the active
one-sided derivative of canonical phase and the concrete physical residuals.
A trial is admitted to the solver only when finite, within every existing
domain/capacity and algebraic side constraint, and strictly improving the
existing scaled residual merit; singular, non-descent, or stagnant systems
shrink the trust region deterministically and ultimately refuse typed. One
`CoveredPhysicalEvaluationBudgetV1` owns the unchanged maximum 96 across the
entire covered solve. Each trigger-confirmation, baseline-residual,
generalized-Jacobian, trust-region, rejected-trial, fresh-root, and final-replay
physical evaluator invocation charges it exactly once before evaluation. No
nested solve, rejection, restart, or finalization seam may reset or enlarge it.
A sealed internal event boundary forces existing partitioning or typed refusal;
a phase kink is internal complementarity and creates no event.

After a root satisfies every unchanged residual tolerance and side constraint,
`phase_consistent_coupled_authentic_final_evaluation_v1` reevaluates it from
immutable beginnings and sealed physical sources, and
`phase_consistent_coupled_authentic_final_replay_reseal_v1` independently
reconstructs and reseals its source/receipt lineage. Both calls charge the same
budget. Only that fresh result may enter the private
`CoveredConvergenceAdmissionV1::CoupledAuthentic` branch. This branch bypasses
only ordinary Picard current/candidate equality and convergence; it retains
every physical residual tolerance, side constraint, authentic finalization,
receipt reseal, event/topology/custody/identity guard, rollback, and atomic
publication check. Any mismatch, budget exhaustion, poisoned component, event
crossing, capacity/domain failure, or intermediate mutation rolls back exactly.
The method introduces no equation, tolerance, iteration cap, 60-second floor,
adaptive controller, event, topology, custody, receipt, rollback, schema,
persistence, diagnostic, or publication change.

The persistent-layer refreeze operation preserves nonnegative cold content at
its generating seam. When liquid availability reaches the exact capacity
`Q_cc/L_f`, binary64 division followed by multiplication is not generally
bit-reversible even though the constitutive remainder is algebraically zero.
`TOL-SNOWENERGY-006` therefore canonicalizes only the resulting finite
remainder in `[-1e-9,0) J m^-2` to exact positive zero. The selected refrozen
mass and its `L_f*m_refreeze` energy remain unchanged and are retained in the
ordinary water and energy ledgers; the independently reconstructed raw
remainder must remain within the named bound. A material negative remainder,
or negative cold content from any other operation, fails closed and is never
clamped during persistence.

`INV-SNOWENERGY-050` — A terminal event is owned only by an accepted microstep
whose ending ice is exact zero and whose mass, liquid, energy, owner, topology,
and receipt closures pass. Its liquid endpoint is transferred exactly once to
surface-liquid/WB14 custody; the remaining parent support is reevaluated under
the snow-free owner. No separate root solver, post-event snow flux, censored
snow energy, or alternate snow model is admitted.

`INV-SNOWENERGY-051` — Stage 3 terminal liquid is a zero-Celsius phase
reference. At committed publication only, a mass-weighted temperature
bit-identical to the first binary64 value above `273.15 K`
(`0x4071126666666667`) canonicalizes to exact `273.15 K`; exact reference is
unchanged and every other value is unchanged and remains subject to the
existing nonpositive-Celsius outcome guard. This exact representation rule
changes no mass, energy, support, phase decision, or ledger tolerance.

Test obligations include all cold/mixed/melt phase branches, bounded
sublimation with paired latent truncation, deposition below/at/above melt
balance, beginning/external liquid and refreeze, raw/actual poisons, the real
deposition-at-meltout endpoint and perturbations, direct/composed consistency,
floor acceptance/failure, deterministic replay, and independent linked-ledger
reconstruction. Calibration is not applicable: the projection is conservation
and controller tolerances are numerical constants, not fitted parameters.

The 2026-08-27 owner amendment changes temporal admission only, replacing the
provisional 600-ms floor with an exact 60-second floor. Bounded-vapor custody,
phase-equilibrium and closure equations, topology, receipt identity/order,
exact rollback, and all fail-closed obligations remain unchanged. Every prior
floor-dependent result, attempt count, event tick, trace, and performance
claim is superseded and requires fresh execution. The 2026-08-28 performance
amendment retains that floor and authorizes only the result-blind solver and
controller policies stated above; it does not relax ledger closure or exact
discrete-event authority. The version-25 exact-floor contraction amendment
supersedes only version 23's undamped exact-floor sentence; all convergence
tolerances, exact density/discrete predicates, ledgers, events, receipts,
rollback, cap exhaustion, and authentic final replay remain unchanged.

## Gap Register

| Gap ID | Gap | Owner | Required closure | Current disposition |
|---|---|---|---|---|
| `GAP-SNOWENERGY-001` | Shared `T_s` and cold-content provider selection. | `SNOW-SURFACE-EB-03` | Stage 3 top-layer provider plus common-consumer proof. | candidate selected; common-consumer proof failed / runtime `HOLD` |
| `GAP-SNOWENERGY-002` | Canopy temperature uses no prognostic canopy energy balance. | future adjudication after EB-04 | Assess factorial sensitivity and field/literature evidence; retain `T_c=T_a` only within stated homogeneous-stand limits. | accepted approximation |
| `GAP-SNOWENERGY-003` | Polar-night cloud state cannot be inferred from daily clearness index. | future authoritative cloud producer | Version 2 returns typed unavailable when longwave is enabled and `R_a <= R_a,min`; a future producer requires contract amendment. | bounded runtime limitation |
| `GAP-SNOWENERGY-004` | `R_a,min` numeric threshold. | `SNOW-SURFACE-EB-03` | Unit-explicit `1e-9 MJ m^-2 d^-1` and transition tests. | resolved in version 2 |
| `GAP-SNOWENERGY-005` | Effective-cover translation has not been evaluated against hemispherical photography across heterogeneous stands. | future validation campaign | Compare without making observations a runtime prerequisite. | non-blocking research gap |
| `GAP-SNOWENERGY-006` | The Dilley-Unsworth review does not establish a transferable numeric meteorological input envelope for every openWEPP climate. | future validation | Enforce the no-clamp derived-emissivity guard; report extrapolation diagnostics and evaluate climate-envelope adequacy. | implemented guard / validation gap |
| `GAP-SNOWENERGY-007` | The Stage 3 cold-content carrier used a snowfall-event top layer instead of the Marks/SNOBAL active thermal control volume and applied hourly surface energy outside a mass-dependent coupled active/lower substep. | `SNOW-SURFACE-EB-03A` | Implement and independently test version-3 active-volume construction, `G_0`, conservative projection, and stability substeps without a clamp, fitted limiter, new user coefficient, or changed frozen controls. | resolved in version 3; real B/L/S/LS and rollback cells pass |
| `GAP-SNOWENERGY-008` | Stage 3 continued thermal/exchange evaluation below libsnobal's minimum resolved layer mass, producing 17 impossible temperatures and five valid-Kelvin vapor-pressure underflows. | `SNOW-SURFACE-EB-04C` | Apply the exact fixed resolved-layer boundary before temperature/conductivity evaluation while preserving CoE mass and persistent cold content; prove all 22 captured thermal failures pass their original boundary. | resolved in version 4; 22/22 captured failures pass their formerly rejected processing day with zero forbidden thermal errors |
| `GAP-SNOWENERGY-009` | Multilayer density initialization used the `1e-9 m` aggregate SWE tolerance as a layer-deletion threshold, omitting represented fragments whose physical depth remained in the expected aggregate. | `SNOW-SURFACE-EB-04D` | Apply the existing `1e-9 kg m^-2` density-model zero-mass boundary after named SWE-to-mass conversion; preserve coupled state and prove both captured geometry failures pass. | resolved in version 5; both 16,437-day trajectories complete with independently reconstructed layer mass/depth closure |
| `GAP-SNOWENERGY-010` | EB-04R transcribed the `1e-9 m` SWE-equivalent vapor-to-sublimation closure as `1e-9 kg m^-2`, conflating it with separate mass-unit predicates. | `SNOW-SURFACE-EB-04S` | Reconcile from pre-result authority, state every operand-specific tolerance in canonical units, and preserve EB-04R as an unchanged HOLD. | resolved in version 6; result-blind dimensional authority frozen before retained-output adjudication |
| `GAP-SNOWENERGY-011` | CoE remains the production melt generator. Version 12 admits evaluation-only residual-snow enthalpy and localized solid exhaustion, but receiving-surface liquid/energy custody and post-event flux chronology remain unresolved for the target. | land-surface authority plus future atomic cutover | Implement a first-class receiving surface, exact-one ownership, linked ledgers, selectors/defaults/rollback, and real downstream consumption. | terminal snow numerics admitted; production/cutover `IMPLEMENTATION_HOLD` remains |
| `GAP-SNOWENERGY-012` | Current evaluation schema-v6 can apply raw latent-energy opportunity while actual sublimation is availability-bounded. | this plausibility package and future production implementation | Quantify tuple-level capacity truncation without aliasing; production target must derive latent energy and mass from one bounded `m_v`. | characterization admitted; physical passage and persistence held when active |
| `GAP-SNOWENERGY-013` | Surviving WEPPpy runs directly recover retained centroids, GRIDMET-enabled intent, daily parquet wind, and exact CLI equality; nearby historical code only statically reconstructs the likely centroid/GRIDMET/share/format path. Deployed identity/request/response, exact GRIDMET asset version/status, server pixel/sampling, missing/day-boundary policy, and physical exposure linkage remain absent. | source generator owner / future authority package | Supply immutable deployed/server receipt fields and two-sided forcing-to-target exposure authority; a height conversion or modeled forest class alone is insufficient. | narrowed `AUTHORITY_MISSING`; persistence held; no canopy or production correction authorized |

## Default-Off Terminal Receiver Transaction Amendment

`INV-SNOWENERGY-035` — `terminal_receiver_v1` is a fresh, internal,
default-off authority layered after, and not inside, the evaluation-only
`INV-SNOWENERGY-034` event solver. From immutable interval-beginning snow and
receiver snapshots, it consumes the earliest complete solid-exhaustion event
`t*`, where `0 < t* <= dt_interval`, and exposes exactly one receipt containing
the event fingerprint, `t*`, `dt_remaining = dt_interval - t*`, zero terminal
represented ice and cold content, bounded vapor transfer, and all retained plus
newly generated liquid. Liquid temperature is exactly `273.15 K`; sensible
enthalpy relative to the contract reference is exactly zero. Fusion energy
remains closed by the snow ledger and is neither liquid sensible enthalpy nor a
soil-energy credit.

The receipt transfers liquid and zero sensible enthalpy exactly once to
`SC-SURFACELIQUID-001#INV-SURFACELIQUID-010`. After `t*`, no snow albedo,
surface temperature, roughness, radiation, turbulence, evaporation,
precipitation heat, soil heat, or unallocated terminal energy is admissible.
The actual snow-free receiver rebuilds every flux over only `dt_remaining`.
Zero remaining duration commits closure/receipt without a receiver physics
step; all other cases have neither time overlap nor gap.

The transferred amount is explicitly
`m_terminal_liquid = m_liquid,retained,start + m_rain,snow_support +
m_melt,new - m_refreeze`, with every term nonnegative,
finite, on the same OFE-ground basis, and independently present in the event
ledger. `m_rain,snow_support` is the only authorized external snow-support
liquid and contains only rain whose absolute support is
`[wall_start,wall_t*)`; receiver-side rain begins at `wall_t*` and is excluded.
Runon is receiver-side only and is prohibited on snow support in this
transaction because no existing snow authority admits it.
Snow candidate commit sets retained liquid to zero and records
`terminal_receipt_consumed=true` under the event fingerprint while the surface-
liquid candidate credits exactly `m_terminal_liquid`. Debit, credit, and the
consumed marker are one atomic join; any omission, duplication, stale marker,
negative result, or receipt replay is typed failure and restores all three
beginning values.

Handoff additionally requires
`Q_terminal_unallocated <= TOL-SNOWENERGY-001` on the independently reconstructed
event ledger. A larger positive value is `SNOWENERGY-E-TERMINAL-UNALLOCATED` and
rejects the receiver transaction with no recipient, disposition, carry, soil
assignment, or state commit. This does not relax the schema-v8 event-only
censoring: INV-034 still reports the value only as evaluation evidence.

This mechanics authority is unreachable from production defaults/selectors;
CoE remains the sole production snow mass/melt generator. Turbulent-carrier and
forest-exposure authority, physical efficacy, qualification, assurance
approval, production ownership, and cutover remain held. `INV-SNOWENERGY-034`
retains schema-v8 evaluation-only, no-recipient, no-commit semantics.

- `OBL-SNOWENERGY-P-009`: emit one immutable fingerprint-bound receipt or a
  typed snow-side failure; never partially commit.
- `OBL-SNOWENERGY-C-016`: independently reconstruct snow mass, liquid, vapor,
  fusion energy, and time, rejecting duplicates, aliases, post-event snow
  operands, or nonzero liquid sensible enthalpy.

| Obligation ID | Binding requirement | Enforcement |
|---|---|---|
| `OBL-SNOWENERGY-P-009` | Emit one immutable terminal receiver receipt or a typed failure; never partially commit. | receiver transaction |
| `OBL-SNOWENERGY-C-016` | Reconstruct snow, liquid, vapor, fusion energy, and time without post-event snow operands or aliases. | receiver validator |

Required unequal-operand vectors distinguish retained liquid, new melt, rain,
runon, vapor, and receiver storage and poison full-step flux reuse,
liquid/store aliases, fusion-energy-to-soil assignment, and dual melt owners.

| Canonical surface | INV-035 binding |
|---|---|
| Algorithm | localize earliest `t*`; reconstruct event; reject unallocated energy; form explicit liquid debit/credit/consumed join; invoke receiver for remaining half-open support |
| Branch/guard | no event: remain snow; `t*=wall_end`: zero receiver step; positive out-of-tolerance unallocated energy: typed no-recipient failure |
| Invariant/alias | event fingerprint, retained/rain/melt/refreeze/vapor terms are distinct; fusion energy is not liquid sensible enthalpy or soil heat |
| Unit/tolerance | liquid `kg m^-2 OFE-ground`; energy `J m^-2`; time `s`; only named event closure tolerance applies, never to identity/cardinality |
| Tests/gap | unequal operands, endpoint allocation, replay/alias poisons, unallocated-energy rejection; production/carrier/efficacy/cutover gaps stay held |

## Child 2C shared snow--canopy turbulent carrier amendment

This amendment binds one default-off carrier for a forest-covered V11 canopy
and Stage 3 ground snow. It does not activate the carrier, select a production
default, admit canopy-intercepted snow, or qualify an exposure or seasonal
consumer. The carrier fails closed when any required sealed forcing, exposure,
thermal, or support receipt is missing.

### Carrier topology, inputs, and ownership

The topology is:

```text
sealed reference atmosphere -> shared canopy-air node
shared canopy-air node -> V11 canopy surfaces
shared canopy-air node -> Stage 3 ground snow surface
```

The sealed half-hour forcing owns reference wind, temperature, humidity, and
pressure. It must provide an exposure-projected wind at the admitted virtual
transfer geometry; a nominal raw `10 m` wind is not a subcanopy operand. V11
owns canopy structure, leaf/stem surfaces, their temperatures and transfer
conductances. Stage 3 owns snow surface temperature, SWE, liquid, cold
content, roughness, emissivity, and albedo. The carrier transaction owns one
shared canopy-air temperature/humidity node and its coupled residual. Coupled
time owns the segment support receipt and one complete-owner parent commit.

Each production lane owns exactly one persistent Stage 3 snow column whose
mass, depth, liquid, cumulative mass terms, cold content, energy, fluxes, and
terminal liquid are expressed per unit OFE ground. Tile-level surface operands
remain per unit tile ground and enter that column exactly once as
`X_lane = sum_i(f_i * X_i)`, where the ordered complete snow-surface tile set
closes to `sum_i(f_i) = 1` within the admitted topology tolerance. The sum is
not divided by the covered fraction. A mixed open/covered OFE therefore
requires both covered-canopy and open-snow boundary receipts; a missing,
duplicate, or incomplete tile contribution is typed failure. Lane-wide
precipitation enters once on the OFE-ground basis and any tile partition must
independently reconstruct it. The area basis, lane/OFE identity, ordered tile
fractions, and topology digest are restart identity. A future per-tile or
per-routing-cell snow owner requires a new versioned topology and cannot
reinterpret this lane state.

The source authority is `SC-VEGETATION-001@26` for V11 shared-tile air,
neutral momentum and canopy-surface conductances; `SC-SNOWENERGY-001` for
Stage 3 virtual `z_T=z_q=z_u=5 m`, exposed-snow `z_0,aero=0.005 m`, and
reciprocal sky/canopy/snow longwave; `SC-COUPLEDTIME-001@3` for support and
event chronology; and `SC-LANDSURFACEENERGY-001@7` for the snow-free
successor. No fixed forest attenuation multiplier or fitted proxy is
authority.

### Turbulent carrier equations

All turbulent fluxes use the same current trial shared-node state. Define
surface-to-node fluxes as positive from a canopy or snow surface into the
shared node:

```text
H_i = rho_a * c_p * g_H,i * (T_i - T_ca)
V_i = rho_a * g_q,i * (q_i - q_ca)
H_ref = rho_a * c_p * g_H,ref * (T_ref - T_ca)
V_ref = rho_a * g_q,ref * (q_ref - q_ca)
R_T = H_ref + sum_i(H_i) = 0
R_q = V_ref + sum_i(V_i) = 0
```

The unknown shared node is solved or iterated from these complete residuals;
the linear trial update, when every conductance is fixed, is the weighted
closure `T_ca=(g_H,ref*T_ref+sum(g_H,i*T_i))/(g_H,ref+sum(g_H,i))` and the
same expression for `q_ca`. Nonlinear conductances, saturation, and V11
surface temperatures are reevaluated until both residuals and the owning
constitutive tolerances pass. The carrier exports snow sensible and vapor
terms as `H_snow=-H_s` and `V_snow=-V_s`, positive into snow. It exports
canopy terms with the same sign reversal into each canopy owner. There is one
reference exchange and one snow exchange; no flux is copied into a second
owner.

Reference wind is first converted through the sealed exposure/transfer
receipt and the named neutral log-law in V11. Surface conductances remain
owner-specific. Raw `10 m` wind, a fixed attenuation factor, a hidden wind
floor, or independent canopy-air nodes is typed failure.

### Canopy--snow--sky longwave

The carrier imports the reciprocal V11 rank recurrence and the Stage 3
complementary longwave boundary. For the one-layer snow boundary:

```text
L_can       = sum_j(w_j * sigma*T_can,j^4)
L_snow,down = f_sky * L_atm + (1-f_sky) * L_can
L_snow,net  = L_snow,down - sigma*T_s^4
L_snow<->canopy = (1-f_sky) * (sigma*T_s^4 - L_can)
```

The canopy-side term is the equal-and-opposite exchange in the V11 recurrence;
atmospheric and canopy component terms remain separate operands. `L_can` is
reconstructed from the current V11 canopy component temperatures and
emissive-area weights; it is not the shared-air temperature and cannot be
copied from a stale or aggregate diagnostic. `f_sky` is
the existing effective-cover Beer-law translation, not a new input. Canopy
and snow emissivity are exactly one under the admitted one-layer domain.
Longwave is evaluated from the same current trial temperatures as the
turbulent residual; a stale or post-event snow temperature is invalid.

### Carrier algorithm and guards

### Lane-boundary receipt identity

`OPENWEPP_LANE_STAGE3_BOUNDARY_RECEIPT_V1` is an adopter-specific,
deterministic receipt wire and is not a coupled-time parent/restart identity.
Its SHA-256 preimage is the ASCII domain including its trailing NUL, followed
positionally by: little-endian `u32 lane_id`; `u64`-length-framed UTF-8 OFE ID;
little-endian `u128 start_ns,end_ns`; one byte area basis (`0=OfeGround`); five
raw 32-byte aggregate source-set digests; little-endian `u64` contribution
count; then each tile in strict ID order as `u64`-length-framed UTF-8 tile ID,
little-endian IEEE-754 fraction bits, one-byte boundary class
(`0=V11CanopyCovered`, `1=OpenSnow`), raw 32-byte model definition, beginning
Stage 3 state, provisional carrier, optical, reciprocal-longwave, and final
boundary digests, followed by little-endian IEEE-754 bits for the seven named
physical operands in schema order. The seven aggregate physical operands
follow in the same scalar encoding. No optional or extra fields exist.

Each aggregate source-set digest uses domain
`OPENWEPP_LANE_STAGE3_SOURCE_SET_V1\0`, one byte source index (`0` provisional,
`1` optical, `2` reciprocal longwave, `3` final), little-endian `u64` count,
and for each ordered contribution its framed tile ID, fraction bits, class,
model-definition digest, and selected raw source digest. Validation reconstructs
all four sets. This explicit alternative wire may not enter additive restart
or a coupled parent receipt; that requires a future contract amendment adopting
the repository canonical framed helper/domain and fixed vectors.

1. Validate forcing, exposure, virtual transfer geometry, active participant
   set, owner state, and support receipts without mutating owners.
2. Construct one current-trial shared node from all active V11 surfaces and
   the Stage 3 snow surface; do not split the node by stratum or regime.
3. Evaluate reciprocal canopy--snow--sky longwave and all turbulent residuals
   from the same trial state.
4. Iterate the complete coupled system until the carrier, V11, and Stage 3
   tolerances pass; reject incomplete or wrong-regime operands.
5. Independently reconstruct vapor mass, sensible/latent energy, longwave,
   cold-content, liquid, and event-time ledgers before producing a typed
   owner candidate. Commit only through the coupled-time complete-owner
   transaction.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-SNOWENERGY-036` | One shared canopy-air node jointly closes reference, all V11 canopy, and Stage 3 snow sensible/vapor exchange. | carrier residual / `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-037` | Wind is a sealed exposure-projected operand at declared transfer geometry; raw 10 m wind and fixed attenuation are not substitutes. | forcing/exposure join / `SNOWENERGY-E-WIND-001` |
| `INV-SNOWENERGY-038` | Canopy--snow--sky longwave uses one reciprocal current-trial state and exact-one exchange. | radiation lineage / `SNOWENERGY-E-LW-001` |
| `INV-SNOWENERGY-039` | Snow fluxes stop at the accepted event and snow-free fluxes begin only on admitted successor support. | chronology / `SNOWENERGY-E-REGIME-001` |
| `INV-SNOWENERGY-040` | Canopy-intercepted snow is outside this carrier and cannot enter its mass or energy ledgers. | scope guard / `SNOWENERGY-E-SCOPE-001` |
| `INV-SNOWENERGY-042` | The single persistent Stage 3 lane owner is OFE-ground. Complete typed tile-ground snow-surface fluxes aggregate only as `sum_i(f_i X_i)` over an ordered tile set closing to one under `TOL-SNOWENERGY-002`; covered-subset renormalization, inconsistent common snow state, missing/open-surface omission, duplicate tiles, class/model substitution, or restart topology/basis substitution is prohibited. Uniform terminal-liquid projection preserves `sum_i(f_i M_i) = M_lane`; dividing the complete lane amount by every tile fraction is prohibited. | topology/area/source/state/restart guard / `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-043` | Covered fixed-point acceptance reconstructs each candidate fingerprint independently; schema, terminal-event model, lane, interval, layer cardinality/order, density, and count-like settling state compare exactly. Numeric state uses only `TOL-SNOWENERGY-003` by physical class; fingerprints need not equal when admitted numeric state differs. | typed convergence/nonconvergence guard / `SNOWENERGY-E-CARRIER-001` |
| `INV-SNOWENERGY-044` | Lane receipt V1 is non-restorable. Additive restart is blocked until a normative V2 schema excludes initial-guess identity and defines exact canonical framing, topology/owner joins, and test vectors. | restart schema/version guard / hard `HOLD` |

`TOL-SNOWENERGY-003` governs only covered outer fixed-point convergence. SWE,
physical thickness, liquid depth, and refrozen depth use `1e-9 m` absolute;
temperature difference uses `1e-8 K` absolute; cumulative/detached mass uses
`1e-6 kg m^-2` absolute; cold-content and cumulative energy use `1e-6 J m^-2`
absolute. The depth and corresponding water-equivalent area-mass scales, energy
reconstruction scale, and covered-carrier temperature scale already exist in
this contract or implementation authority. Density is bitwise exact. The
stored `settle_day_count` chronology operand is bitwise exact despite its
`f64` representation. No relative tolerance, state repair, or cross-unit
substitution is admitted.

A future `OPENWEPP_LANE_STAGE3_BOUNDARY_RECEIPT_V2` shall be the
parent/restart successor. It must use repository canonical framing, omit all
initial numerical guess identity, bind static topology context and ordered
covered/open final-boundary sources, and require exact lane/OFE, map-key,
class/model, fraction, and complete-destination joins. V1 remains historical
adopter evidence and must never be restored. Restart replay must join V2,
destination and component receipts, installed LSE and complete snow owners,
static tile/occupancy topology, and wet-liquid authorization. The V2 schema is
currently `SCHEMA_UNDEFINED / IMPLEMENTATION_BLOCKED`: no implementation may
infer its exact fields, framing, ordering, or vectors from in-memory V1 types.

### Child 2C obligations and gaps

`OBL-SNOWENERGY-P-010`: emit one carrier candidate with complete operand
lineage, residuals, current-trial temperatures, and owner/support identities.
`OBL-SNOWENERGY-C-017`: independently reconstruct snow, vapor, liquid, energy,
longwave, and event-time closure and reject any alias or duplicate flux.
`OBL-SNOWENERGY-C-018`: independently reconstruct each lane boundary and
terminal-liquid handoff on OFE-ground basis from the complete ordered
tile-ground contribution set, and reject any non-closing topology, covered-
subset normalization, or restart area/topology substitution.
`OBL-SNOWENERGY-C-019`: independently reconstruct every candidate fingerprint,
then compare exact structure/count/density and only the unit-specific absolute
state residuals admitted by `TOL-SNOWENERGY-003`.
`OBL-SNOWENERGY-C-020`: reject V1 and inferred successor restart wires; require
the normative V2 schema and the complete static-topology and ending-owner join
before additive restore.

The carrier is `AUTHORITY_ADMITTED / IMPLEMENTATION_MISSING` until the later
default-off implementation package proves a real V11/Stage 3 consumer. The
existing wind-custody/exposure gap remains a typed precondition, not a license
for attenuation. Calibration is `CALIBRATION_NOT_APPLICABLE` here; no
efficacy, qualification, or empirical claim follows.

| ID | Gap | Disposition |
|---|---|---|
| `GAP-SNOWENERGY-014` | Default-off runtime carrier and actual V11 snow-covered consumer are not implemented. | later implementation package; current authority remains promotable |
| `GAP-SNOWENERGY-015` | Deployed/server exposure receipt is not available for every retained forcing value. | typed runtime precondition; no proxy or attenuation admitted |
| `GAP-SNOWENERGY-016` | Exact lane-receipt V2 fields, canonical framing, ordering, and test vectors are not yet defined. | `SCHEMA_UNDEFINED / IMPLEMENTATION_BLOCKED`; V1 is never restart authority |

## Version 17 precipitation phase-parcel custody amendment

This approved amendment governs only the mass/enthalpy handoff into the
persistent Stage 3 support. `SC-VEGETATION-001@28` remains sole authority for
liquid interception, persistent canopy storage, initial and second drainage,
throughfall, and stemflow. `SC-LANDSURFACEENERGY-001` remains authority for
the ordered destination topology. No raw precipitation amount may be passed
around the parcel set, and no rule here intercepts solid precipitation in a
canopy.

The canonical parcel key is
`(lane_id, destination_topology_index, phase_rank, source_rank,
semantic_receipt_ordinal)`.
`phase_rank` is `0=solid`, `1=liquid`; `source_rank` is
`0=atmospheric_ground_snow`, `1=open_raw_rain`,
`2=vegetation_terminal_throughfall`,
`3=vegetation_initial_drainage`,
`4=vegetation_second_drainage`, and
`5=vegetation_terminal_stemflow`. Each vegetation route remains a distinct
parcel with its own mass, enthalpy provider, producer-state identity, and
receipt identity; aggregating drainage into throughfall is prohibited. Keys
are unique and strictly increasing.
The semantic ordinal distinguishes repeated parcels from the same route in
their producer order. Receipt hashes authenticate already ordered semantic
records; hash value order is never the semantic or arithmetic order. The
sealed set binds its schema, exact half-open
support, lane/OFE, OFE-ground basis, ordered destination topology/fractions,
parcel count, ordered keys and parcel receipts, and producer beginning-state
identities. An empty ordered vector sealed under the same set schema and
support/topology identity is the only representation of zero precipitation.

For each open destination, atmospheric solid and liquid are separately sealed:
solid uses `atmospheric_ground_snow`, and liquid uses `open_raw_rain`. For each
covered destination, solid likewise uses `atmospheric_ground_snow`, while all
ground-reaching liquid is imported only from the vegetation terminal release
owner as the distinct admitted throughfall, initial-drainage, second-drainage,
and stemflow parcels. A covered raw
rain parcel, an open vegetation-release parcel, or both liquid source classes
at one destination is invalid. Persistent canopy storage is not ground
precipitation. Canopy-intercepted snow remains outside the supported domain.

Each parcel carries finite non-negative tile-ground mass and a finite
temperature or specific-enthalpy provider sufficient to reconstruct its
advected heat. The Stage 3 candidate independently validates the set and then
uses its parcel identities exactly once for both mass and advected energy.
It aggregates each physical value to the lane with the destination fractions
from the sealed topology. The mass consumer set and advection consumer set
must be identical in key order and cardinality; equality of aggregate numbers
or set digests cannot excuse a missing, duplicated, or substituted parcel.
Validation completes before snow or soil owner mutation, and any failure uses
`SNOWENERGY-E-PRECIP-001` with complete rollback.

## Change Log

| Version | Date | Change | Evidence |
|---:|---|---|---|
| 56 | 2026-09-01 | Added a strictly frozen/noncrossing temperature-primary coupled specialization before V55. Ordered coordinates are lane `W/T_s/rho` plus top-soil `E/T`; snow H is reconstructed exactly as `-exact(W)*exact(c_ice)*(exact(273.15)-exact(T_s))`, rounded once to a binary64 high word by round-nearest-even, and retained with exact dyadic carry. Unchanged CN heat is derived and consumed once. Only fresh authentic Stage 3 material plus ordered carry may become a compound snow owner; additive carry receipts join the whole exact stabilization, independent replay, and finalization chronology. Restart V5 preserves committed, pending, and in-progress compound-owner state, admits V4 migration only with canonical zero carry, and refuses nonzero-carry downgrade. Every physical call retains the shared maximum 96 and exact 60-second floor. No transient V54/V55 witness, receipt repair, constitutive physics, ledger term/tolerance, event, topology, custody, rollback, publication, persistence diagnostic, cap, or floor change is admitted. | Exact r144 `/tmp/wghl_001d_v55_64m_r144.log`, SHA-256 `161712621295b503da41b065846304ce0e0198a26a9d9b97efa6d4012fa36c65`; exhaustive 21-member no-witness proof; contract/source expected pre-red |
| 55 | 2026-09-01 | Added a bounded exhaustive private binary64 Q-lattice witness at the first tolerance-closed physical root, before V45 polishing. The search changes exactly one unresolved ordered Q coordinate, evaluates every representable value through a charged full `PrivateTrial` map, and atomically reserves the entire lattice plus the shared-budget authentic probe/replay charges; strict downstream finalization remains mandatory outside that physical-evaluation account. An unresolved-count, positive-domain, checked-cardinality, or capacity miss is zero-charge `NotApplicable` and continues unchanged V45 polishing. A fitting attempt commits immediately before its first candidate charge and may never fall back. Only an exact positive-zero `R_Q` own-output fixed point under every unchanged residual, derived-z, branch, side, custody, and closure guard may proceed to exact receipt stabilization, independent replay, and strict finalization. No authentic-Q enumeration, sparse search, averaging, interpolation, nextafter, receipt repair, tolerance/cap/floor change, or publication is admitted. | Retained r139; exact r140 SHA-256 `3482ada5075aa921fc1e71d0f5fa253765b009fa4e833b05f3e9edc598147628`; exact r142 SHA-256 `4c4d63a75b39494b005cacc21d1d2777c03faeebff7af503a44d80ca1ebf7473`; contract/source expected pre-red |
| 54 | 2026-09-01 | Added a bounded exact receipt-cycle endpoint witness search after unchanged iteration proves a cycle of at most three members. Every member reconstructs complete W/H/rho/Q/E/T from its own already-produced artifact/receipt, is evaluated through one charged authentic full map under unchanged closure, and may proceed only on bit-exact receipt equality plus independent same-input/same-coordinate exact replay. Search preflights all members plus replay inside the unchanged maximum 96. No Q enumeration, averaging, interpolation, nextafter/ULP search, receipt repair, uncharged physics, tolerance/cap/floor change, or private publication. | Retained r137; exact r138 SHA-256 `28f55b679c8eaae874fdfead55c992e16ceaff559925c71b78cd63b49068a6e7`; contract/source expected pre-red |
| 53 | 2026-09-01 | Corrected fresh active-set and legacy V52 initial Q seeding to reconstruct snow-candidate CN heat from the exact already-produced endpoint Stage 3 and soil candidates associated with solver dispatch. This representational seed assembly performs no physical map, consumes no shared-budget charge, and changes no non-Q seed coordinate. The V52 coordinate equation, physical residual, tolerance, complete-step preflight, maximum 96, exact receipt stabilization/replay/finalization, exact 60-second floor, closure, custody, events, rollback, and no-publication remain unchanged. | Retained r135 SHA-256 `38949328585ea32604a9d7de637540012c478377a6b62d171b2b9486f37e328f`; exact r136 SHA-256 `0963a182818d2f0791a4eb6d14da53df746807c29fa8dcd516296d672860e0e1`; contract/source expected pre-red |
| 52 | 2026-09-01 | Added one ordered per-lane snow-candidate CN heat coordinate `Q_cn` (positive into snow) and physical residual `R_Q=Q_cn-Q_cn,physical` under the unchanged lane energy tolerance. `PrivateTrial` consumes coordinate Q exactly once through the typed unpublished CN operand as snow `+Q` and soil `-Q`; authentic receipt probes/replay consume their supplied sealed receipt Q unchanged while evaluating the same physical residual and exact input/output receipt equality. Exact V35 receipt stabilization, same-input replay, authentic finalization, shared maximum 96, exact 60-second floor, closure, custody, events, rollback, and no-publication remain mandatory. No receipt repair, averaging, digest residual, `F(x)-x`, constitutive physics, tolerance, cap, floor, persistence, or diagnostic change. | Retained r133 SHA-256 `6291dab02a435a46c4f13646fe8898ade184029ec9cbca75bb7739bab4b2ebcb`; exact r134 two-cycle SHA-256 `cf276951616c509f71bf2f33dc2192e096d5367768ee43062e66f9e37a8d39f0`; contract/source expected pre-red |
| 51 | 2026-09-01 | Extended V41 eligibility only for one direction-consistent adjacent canonical phase crossing followed by finite, nonstagnant, same-phase corrections whose directions alternate and whose exact absolute enthalpy-step magnitudes strictly decrease. Every V41 static/water/chain/cadence/side/raw-owner/no-A-B-A/budget guard and the unchanged authentic solver, receipt replay, finalization, closure, event, rollback, no-publication, shared 96-evaluation cap, and exact 60-second floor remain binding. | Retained r130 SHA-256 `43aee720db2758e47b166f96e726e307152c4fa14c82321564422062b9df728a`; exact r132 SHA-256 `db16c87e296f1a4756d9467e38fb1b36d7611df51b8275a483c3c33584600dbf`; contract/source expected pre-red and focused behavior/poison evidence |
| 50 | 2026-09-01 | Replaced the invalid complete-owner mutual-source validation of a lawful heterogeneous constitutive beginning with an explicit validated covered-V8-envelope ending-source anchor. V49 still requires mutually equal candidate vegetation/LSE/BGC owners, exact soil resident/prepared/accepted custody, reconstruction at install, rollback, and no publication; no adjacency inference, owner rebasing, physics, tolerance, budget, floor, receipt, persistence, or diagnostic change. | Retained r125 SHA-256 `20c7e38a0a1aed2470b943d96b343749be1725db62edcee7ef3aba3cd9e34823`; exact r129 vegetation41/LSE40/BGC41/soil41 capture; contract/source expected pre-red |
| 49 | 2026-09-01 | Added an opaque three-domain prepared-install authority for repeated native-V2 soil children inside one fixed V11 parent. Exact outer source, authenticated resident/predecessor bundle, prepared target/support/receipt/state, accepted ending, and seals are independently revalidated; generic and V48 predecessor-equals-source installs remain strict. No adjacency inference, owner rebasing, transaction repair, physics, tolerance, budget, floor, receipt, rollback, publication, persistence, or diagnostic change. | Retained r123 SHA-256 `8c8a665317d06863b8d612780eb0b0280b5de977802487b5cdacbc81d466ee7b`; exact r124 source42/resident43/predecessor43/target44 capture SHA-256 `f596a10676bed83c1bc360ccaf034982e583922eac158f0d15468b0c98fbfd60`; contract/source expected pre-red |
| 48 | 2026-09-01 | Propagated one explicit authenticated prepared-beginning source/soil-target authority through the real native-V2 fixed-point final install. The generic/public installer remains exact same-ID-only; the typed finalizer validates the complete authoritative resident, prepared target/predecessor/support/receipt, accepted result, state/layer target seals, and orchestrator seals before unchanged atomic replacement. Authority erasure, generic split admission, substitution, adjacency inference, partial mutation, or private publication remains forbidden. Physics, ledgers, tolerances, budget, floor, events, topology, receipts, exact carry, rollback, persistence, and diagnostics are unchanged. | Retained r122 SHA-256 `20f5b118b43f69a35ce3e0ed03576bd916b3b4a9cb579692727f0438fb5de2bc`; exact ordinary fixed-point finalizer call-path trace; contract/source expected pre-red |
| 47 | 2026-09-01 | Added a typed native-V2 atomic complete-owner transaction posture. Mutually equal vegetation/LSE/BGC source owners may install either an exact same-ID soil target or an exact authenticated soil target whose expected predecessor equals that source, while the soil accepted state remains sealed to the target. Foreign, swapped, missing, disagreeing, inferred-adjacent, or target/state-substituted identities fail before mutation. Atomic clone-then-install, exact no-op, rollback, custody, receipts, exact carry, physics, tolerances, budget, floor, publication, persistence, and diagnostics remain unchanged. | Retained r121 SHA-256 `bf703a976e5852a17b1a922d2086a9b2ce7786c4f459aa3cb79d2a346d3cca47`; exact source42/target43/predecessor42 static owner trace; contract/source expected pre-red |
| 46 | 2026-09-01 | Added dimension-complete safeguarded-step budget preflight before the first finite-difference column charge. A private sub-tolerance step that cannot fund all canonical columns, one trust trial, and the unchanged receipt-entry reserve now carries the prior complete root bundle without spending an unusable partial-step tail; above tolerance remains typed budget failure. The Jacobian/Newton/trust method, physical residuals, tolerances, shared maximum 96, exact receipt equality/replay, custody, rollback, finalization, publication, persistence, and diagnostics remain unchanged. | Retained r119 SHA-256 `c9c7e3f19c46ee69815033c9734b17bb873f2ff545f879ad170dfbf94209fab1` and r120 SHA-256 `e00b4d4059560359f17c5c919834957d693bde2b2f7d2d55996ae1be0cb0fc53`; exact dimension/budget control-flow trace; contract/source expected pre-red |
| 45 | 2026-09-01 | Added private coupled-root polishing after tolerance closure using only the existing physical residual/Jacobian/trust map and strict canonical merit descent. Sub-tolerance representational stagnation/non-descent may only hand the carried best physical bundle to unchanged exact V35 receipt stabilization. Private solver, polishing, and nonstable receipt probes preserve three, two, and one later charges respectively within the same shared maximum 96, protecting the independent same-input replay. Exact receipt equality, physics, tolerances, equations, floor, cap, custody, rollback, finalization, publication, persistence, and diagnostics remain unchanged. | Retained r117 SHA-256 `33ac890a9dbe05962363a0a5838b992d7ca2ad3c13e9fe2912f5555968748c5e` and r118 SHA-256 `fb65dfbfd53d4f587a416ffc97d6e1aca9a4d4a8cfd0ac2b49e925c265edc858`; static charge/reserve trace; contract/source expected pre-red |
| 44 | 2026-08-31 | Corrected coupled evaluation posture so an uncommitted private coordinate can rebuild reciprocal-longwave/shortwave/sensible/vapor exchange through the existing provisional LSE path before the aggregate weighted-OFE guard is applied. Receipt probes, same-input replay, and independent finalization remain strict and must pass the unchanged weighted-energy decomposition/tolerance. Top-soil numerical coordinates remain exact-once snow--soil CN operands and are forbidden from double application in the Stage3-covered V8 soil control volume. No physics, ledger operand, tolerance, cap, floor, event, topology, custody, receipt, exact-carry, acceptance, publication, persistence, or diagnostic policy changed. | Retained r116 SHA-256 `b4046720e3719f4736408833bfa1fc32a23e9bfc95aca8219c760bef9c59aadb`; exact trial-dependent weighted residuals; contract/source expected pre-red |
| 43 | 2026-08-31 | Preserved the V38 numerical soil-coordinate sibling as a distinct typed same-support private fixed-point posture. Exact projection seal, prepared-owner, transaction/predecessor/support, receipt-chain, numerical authority/set, topology/order, top-layer zero-carry, unchanged lower layers, and empty credits are required; ordinary base reconstruction and every sequential/acceptance/installation/publication refusal remain unchanged. No physics, exact-carry arithmetic, tolerance, cap, floor, ledger, receipt meaning, persistence, diagnostic, or publication policy changed. | Retained r113 canonical audit SHA-256 `9c1d35d0f34991bec6386cbef9b6ca1295f6ca2e281e9774ba4b6bade5df3188`; exact predecessor-custody first difference; contract/source expected pre-red |
| 42 | 2026-08-31 | Corrected the canonical V31/V32 support enthalpy image to include the exact Stage 3 cold-content magnitude exported with removed ice. Authentic support images carry finite nonnegative `X_c`; endpoint, midpoint, vapor-interface, branch-entry, and V33--V41 successor coordinate reconstruction use `H=-C_0+L_f(L_0+L_in)+Q+X_c`. Zero-export behavior is byte-identical. No physical equation, ledger math, tolerance, cap, floor, event, topology, custody, receipt, exact carry, acceptance, publication, persistence, or diagnostic policy changed. | Persisted-restart 69/71 snow-reappearance failure; captured exact `+2.3211191058 J m^-2` omitted export; contract/source expected pre-red |
| 41 | 2026-08-31 | Added a distinct early-dispatch witness for the retained r109 one-way constant-water enthalpy sweep across exactly one canonical phase boundary. It requires exact static joins, promoted-root chaining, two-map cadence, independently reconstructed phase predicates, one adjacent crossing, and the unchanged shared-budget reserve; it performs no interpolation, tolerance, projection, or root admission and dispatches only the unchanged authentic solver/replay/finalization path. | Retained r109 canonical audit SHA-256 `da9ebf633cb9194a91c55dc7679ac3ecd34da3a3bb3750c1dc0c77a538cb3770`; exact enthalpy chain and predicate evidence; contract/source expected pre-red |
| 40 | 2026-08-31 | Added an earlier four-window parity-monotone trigger with exact static/promoted-root/phase/side/cadence joins, strictly decreasing finite positive tolerance-independent root drift, nonstagnant parity coordinates, and the unchanged shared-budget reserve. It dispatches only the unchanged physical solver and cannot admit a root. | Retained r107 canonical audit SHA-256 `2fdfcb0c54e1845670f9d95a4b3770f3f43ab129e297901fc9e7b786dafa6c75`; contract/source expected pre-red; r109 retained counterexample to fixed-phase/drift eligibility |
| 39 | 2026-08-31 | Separated V2 soil-energy operand source and target transaction authority. Surface ingress and its receipts retain the exact outer source transaction; prepared/unpublished soil operands retain the exact authenticated soil-child transaction; both identities are sealed into every debit-credit digest and independently validated. No energy amount, equation, tolerance, cap, floor, event, topology, owner, receipt meaning, exact carry, rollback, publication, persistence, or diagnostic policy changed. | Retained r102 canonical failure SHA-256 `962716679a499b4e3bd23f87ed2e6ceabda1435081cc590fd9c06f21ed52548b`; exact outer-42/soil-43 evidence; contract/source expected pre-red |
| 38 | 2026-08-31 | Replaced provisional-map coupled residual/replay evaluation with the authentic finalization-equivalent endpoint map. Every shared-budget charge reconstructs proposed snow and exact-carry soil operands, uses the non-provisional endpoint carrier/LSE/boundary/receipt posture, and performs exactly one Stage 3 map; v35 exact receipt stabilization and independent authentic finalization remain mandatory. No equation, tolerance, cap, floor, event, custody, receipt meaning, exact carry, rollback, publication, persistence, or diagnostic policy changed. | Retained r95 canonical audit SHA-256 `a69e6d16b176cdf29015d55c01834f41a32def624774cf736f8f182884b5571c`; exact solver/stabilization `R_z=0` and finalization thickness divergence; contract/source expected pre-red |
| 37 | 2026-08-31 | Added derived physical thickness closure `R_z=z_1-z_phys` from canonical proposed/physical `I/rho` to coupled scaled merit and root admission under the unchanged `depth_abs_m`. Thickness remains derived, the unknowns and solved residual vector remain `W/H/rho/E/T` and `R_W/R_H/R_rho/R_E/R_T`, and every evaluation derives `R_z` from its already-charged unchanged map. V35 receipt stabilization, exact same-input replay, and authentic finalization remain mandatory. No equation, tolerance, cap, floor, event, custody, receipt, carry, rollback, publication, persistence, or diagnostic policy changed. | Retained r93 finalization-audit SHA-256 `c4ddfef9dc52bdd085ff43d97f1179406486795696425c9aa949097fd756b0a5`; exact root-to-finalization thickness bits/delta; contract/source-bound expected pre-red |
| 36 | 2026-08-31 | Made the reduced coupled solver geometry-complete with exactly one terminal density coordinate per affected lane, canonical `z=I/rho` reconstruction, physical `R_rho` from the unchanged density/settling map, generalized `R_W/R_H/R_rho/R_E/R_T`, exact branch authority with evolving rho/z, the same shared 96 budget, and retained v35 receipt stabilization/authentic finalization. Generic map difference, interpolation/repair, uncharged physics, and bypass remain forbidden; no physics, tolerance, cap, floor, event, custody, receipt, carry, rollback, publication, persistence, or diagnostic policy changed. | Retained r88 field-audit SHA-256 `55a904fbbb35126a00f50af60ba3c7d296e3298c575a784cbd3eedaa7f24ec65`; exact thickness bits/delta; contract/source-bound expected pre-red |
| 35 | 2026-08-31 | Added exact authentic receipt stabilization after a coupled physical root. Each immutable input receipt set feeds one charged authentic probe whose reconstructed receipts become the next input; exact input/output receipt equality is required before one independent same-stabilized-input replay must reproduce exact finite residuals, complete artifacts, and receipts. The first root reseal is a cross-input probe, not replay evidence. Oscillation, nonfinite/constraint/budget/replay failure discards every private/probe artifact. Equations, tolerances, 60-second floor, 96-evaluation cap, events, topology, custody, receipt meaning, exact carry, rollback, finalization, publication, persistence, and diagnostics remain unchanged. | Owner-directed `WGHL-FULL-001D-V35` successor; retained r83 canonical log SHA-256 `bd091a4154eafff60309677e38bbbf598da6199cb15b3b84da93e7e23977b909`; contract/source-bound expected pre-implementation red |
| 34 | 2026-08-31 | Extended eligibility for the unchanged equation-level v33 solver to eight consecutive stable monotone raw authentic maps on an event-free terminal-one-volume covered support, with exact support/source/event/topology/custody/static-receipt/phase-branch/carry-authority-and-representation joins, exact per-map `E=exact(H_hi)+R` reconstruction while physical receipt digests and `H_hi/R` coordinates evolve, finite residuals, strictly decreasing merit, no cycle/active-set transition/finalization restart, and one already-charged unchanged 96-evaluation budget. Pre-root refusal discards private trials and resumes raw Picard only within the remaining budget; fresh `CoupledAuthentic` replay/reseal and every equation, tolerance, ledger, floor, event, custody, receipt, exact-carry, rollback, finalization, publication, persistence, and diagnostic guard remain unchanged. | Owner-directed `WGHL-FULL-001D-V34` continuation and contract-first clarification; canonical one-day 60-second support trace reaching raw Picard closure at iteration 93; contract/source-bound expected pre-implementation red |
| 33 | 2026-08-30 | Superseded v31/v32 production control only after an exact active-set root/interface -> branch-entry -> opposite-authentic -> same-root/reset transition record under unchanged joins, explicitly without bitwise equality of asymptotically changing raw-authentic continuous owners. Bound equation-level `R_W/R_H/R_E/R_T`, algebraic CN/LSE/receipt constraints, one shared unchanged 96-physical-evaluation budget, and private `CoupledAuthentic` admission bypassing only Picard equality while retaining finalization/reseal/rollback. No equation, tolerance, cap, floor, adaptive policy, event, topology, custody, receipt, rollback, schema, persistence, diagnostic, or publication surface changed. | Governance commit `0dc1ef007`; retained exact 60/120-second v32 reset capture; design-review correction rejecting `F(x)-x`; evaluator/budget/admission source obligations and corrective isolated pre-red |
| 32 | 2026-08-30 | Retained v31 same-disposition `W/H` authority and added one pure opposite-sign vapor-active-set interface plus zero-to-one-sided unpublished branch entry on any exact covered support at or above the unchanged 60-second floor. The interface makes linked vapor/deposition/sublimation/latent terms exact zero and interpolates only external liquid and ordered nonlatent energy; branch entry uses the unchanged support-scaled weight and authentic positive finite specific latent heat. Synthetic images remain ineligible and only a later fresh authentic image may finalize or publish. No tolerance, cap, constitutive equation, adaptive policy, event, topology, custody, receipt, rollback, schema, persistence, diagnostic, or publication surface changed. | Governance commits `c1cfd6e4b`/`8ec04440d`; frozen captured `1860..1920 s` operands; independent root/nonlatent/`W/H` oracle; affine-latent `+45.77845449909091 J m^-2` rejection; exact-floor and larger direct-support obligations; pre-implementation expected red |
| 31 | 2026-08-30 | Admitted one exact-floor terminal-one-volume phase-aware unpublished midpoint in canonical total-water/enthalpy coordinates, reconstructed from immutable beginning state and a coordinated complete-support operand vector through the existing version-22 phase projection. Rejected v30 component/cumulative interpolation remains non-authoritative. Raw authentic history, exact identities, independent closure, the 60-second floor, tolerances, 96-iteration cap, rollback, and fresh-authentic-only finalization/acceptance/publication remain unchanged. | Captured `1860..1980 s` mixed/frozen vector; independent mass/energy oracle; zero-enthalpy/fusion-capacity exact sides; vapor-sign/nonfinite/structure/receipt poisons; no-intermediate-publication source obligation; real DFF-WS2 rerun pending implementation |
| 29 | 2026-08-30 | Reconciled the version-23 exact-candidate-density rule with finalization relaxation: density is never interpolated, remains bitwise authentic-candidate state, and continues to block convergence until an authentic image matches, while otherwise-admitted continuous coordinates may remain damped. Added finalization/stabilization branches, the primary invariant/guard binding, and formal vector obligations; replaced the boolean stabilization predicate with a stateful exactly-once seam. No tolerance, floor, cap, physical ledger, discrete guard, or publication rule changed. | Independent review RA-001/RA-003/RB-003; candidate-density finalization vector; stateful exactly-once stabilization vector; affected contract and canonical qualification reruns |
| 28 | 2026-08-30 | Applied the existing support-scaled convex Stage 3 iterate policy to a nonconverged authentic finalization rebuild, preventing the finalization transition from discarding damping and repeatedly restarting a cold-content residual. After such a restart, one further guarded Picard update stabilizes the first provisional tolerance crossing before finalization is retried. Retained authentic final LSE/boundary operands, converged soil, every discrete/posture/refusal guard, unchanged convergence tolerances and iteration cap, and authentic-map-only publication. | Default-off iteration-history trace distinguishing 100 cold-only finalization/Picard caps from 27 materially nonconverged maps and one receipt-replay cap; contract-derived transition/refusal/stabilization vectors; replacement focused and canonical one-day qualifications |
| 27 | 2026-08-30 | Classified only the persistent per-layer `refrozen_liquid_m` tracer as within-trial factorization lineage for adaptive direct-versus-composed error estimation. The accepted composed value remains exact owner/restart state, while independent ice-flux refreeze reconstruction and all mass, liquid, cold-content, temperature, cumulative, phase, event, topology, custody, receipt, and energy guards remain unchanged. | One-day cap-signature audit; direct/composed factorization-history vectors; physical-scalar anti-evasion matrix; accepted-publication ledger reconstruction; replacement one-day qualification |
| 26 | 2026-08-29 | Bound accepted terminal composition to every exact physical child's contiguous precipitation and snow--soil receipt custody. The enclosing parcel set preserves physical operands and child order while resealing envelope support; final-child boundary evidence remains distinct. Terminal receipt/event regrouping uses only existing `TOL-SNOWENERGY-005`, with one-bit-over rejection and unchanged physical-ledger closure. | Multi-child terminal reconstruction, material omission diagnosis, exact-side receipt test, EROD16 progression, and canonical one-day replacement qualification |
| 25 | 2026-08-28 | Admitted conditional exact-floor period-two contraction: 60-second solves remain raw unless authentic Stage 3 candidates satisfy the existing exact-discrete/native-unit `A/B/A` predicate, after which `w=0.5` applies for that solve only. This replaces the briefly tested unconditional exact-floor damping that regressed the canonical one-day controller to near-floor stepping. Density and every discrete authority remain unblended; the floor, tolerances, 96-iteration cap, authentic final replay, ledgers, events, receipts, and rollback are unchanged. | P102 exact-floor limiting-coordinate audit; focused raw-convergent, exact-density-cycle, cumulative-closure, event/topology-poison vectors; replacement one-day telemetry |
| 24 | 2026-08-28 | Admitted only exact-zero canonicalization for finite binary64 refreeze-capacity quotient/product cancellation under `TOL-SNOWENERGY-006`; retained the selected refrozen mass, fusion energy, unchanged physical-ledger tolerance, and typed failure for every material or unrelated negative cold-content result. | P102 real-consumer boundary failure operands; focused exact-side and poison vectors with independent water/energy reconstruction; bounded P102 replay |
| 23 performance amendment | 2026-08-28 | Replaced the covered-support period-2 Picard failure with bounded support-scaled under-relaxation and a `0.25` long-support contraction floor; authentic candidate density remains unblended and bitwise-exact while continuous state is damped. Retained undamped exact-floor outer-Picard behavior and typed cap exhaustion, admitted audited complete-owner truncation-error controller bounds, and added causal receipt resealing under `TOL-SNOWENERGY-005`. Exact equal/opposite applied energy, ledger thresholds, exact installed-owner identities, and every discrete predicate remain unchanged. | Default-off real-fixture fixed-point/comparison/receipt audits, focused 60/120/420/1800-second convergence and exact-side runs, and canonical one-day replacement qualification |
| 22 owner amendment | 2026-08-27 | Replaced the provisional 600-ms temporal floor with exact 60-second (`60_000_000_000 ns`) adaptive admission. Conservation, bounded-vapor/phase, custody, topology, receipt, rollback, and fail-closed authority are unchanged; stable ordinary supports must accept substantially larger steps. | Direct owner amendment; floor-dependent execution evidence superseded, replacement runs pending |
| 22 | 2026-08-26 | Replaced the old terminal phase ordering/root exception with bounded vapor-energy custody, same-microstep deposited-ice phase equilibrium, linked closure, adaptive composed-result ownership, and accepted-microstep terminal transfer. Versions 19-21 remain rejected historical candidates. | Direct owner decision in the adaptive compositional microstepping package; physical conservation and existing complete-carrier authority |
| 18 | 2026-08-24 | Admitted persistent snow--soil boundary authority: one OFE/lane interface, bottom snow volume to first OFE soil node, two-half-layer series resistance, Crank--Nicolson beginning/end evaluation inside the covered fixed point, exact equal/opposite candidate custody, reconstructable receipt, and atomic rollback. | Pinned `frostn.for`/`tmpadj.for` series-resistance provenance; `SC-LANDSURFACEENERGY-001@8`; Child-1 contract-derived guards |
| 17 | 2026-08-24 | Admitted persistent-support precipitation custody: sealed ordered phase-parcel sets, present empty-set zero, open raw-rain versus covered route-distinct vegetation-terminal-liquid exclusivity, solid ground-snow bypass, OFE-ground aggregation, and exact same-set mass/advection consumption. No interception or canopy-snow physics was added. | Direct Child-1 checkpoint authority; `SC-VEGETATION-001@28`; `SC-LANDSURFACEENERGY-001`; contract-derived source guards |
| 13 | 2026-08-19 | Added fresh default-off `terminal_receiver_v1` authority (`INV-SNOWENERGY-035`) for earliest-event closure, exact-one 0 C liquid/enthalpy custody, remaining-time support, and post-event snow-operand exclusion while preserving all carrier/efficacy/production holds and evaluation-only INV-034 semantics. | Contract-first terminal handoff package |
| 12 | 2026-08-07 | Admitted a persistent-evaluation-only one-volume shallow-snow enthalpy solve, deterministic step-doubling policy, safeguarded earliest solid-exhaustion event, schema-v8 reconstruction, and explicit censoring of terminal liquid, energy, and remaining time. | CC0 libsnobal shallow-pack/cadence/phase chronology, conservation, and independent numerical review |
| 11 | 2026-08-07 | Recovered byte-identical WEPPpy lineage, retained centroids/flags, and exact parquet-to-CLI equality; separately labeled the nearby historical centroid/GRIDMET/share/format code path as static reconstruction and narrowed missing authority to deployed request/server semantics and two-sided aerodynamic exposure linkage. | Surviving `/wc1` run records, provider source/history, retained parquet/CLI equality, and independent custody/exposure reviews |
| 10 | 2026-08-07 | Separated nominal GRIDMET `10 m` source height, raw CLI/Stage 3 wind, PMET-local `2 m` adjustment, and virtual Stage 3 `5 m` transfer geometry; retained missing source/exposure authority and prohibited fitted attenuation or production correction. | Stage 3 wind source-custody result-blind authority reconciliation |
| 9 | 2026-08-07 | Distinguished evaluation-only raw vapor/latent opportunity `m_v,raw` from actual bounded transfer `m_v`; bound independent tuple-level transfer reconstruction, N/A and alias rejection, and capacity-truncation plausibility hold without changing production physics. | Stage 3 evolving-state carrier result-blind authority reconciliation |
| 8 | 2026-08-05 | Bound typed CLIGEN/openWEPP virtual-instrument heights `z_T=z_q=z_u=5 m` above the instantaneous modeled snow surface and exposed-snow aerodynamic roughness `z_0,aero=0.005 m`; distinguished aerodynamic roughness from active-layer depth and retained all carrier/cutover gates. | Direct user authority plus pinned libsnobal point-input defaults and fixture |
| 7 | 2026-08-04 | Admitted Stage 3 as the sole future melt owner after CoE failed the frozen specific-validation and enforceable-envelope predicates. Bound cold-content-first complete energy, bounded latent-fusion conversion, same-substep linked mass/liquid ledgers, no-dual-owner guards, and an atomic implementation hold covering incomplete fluxes and residual snow. Runtime CoE behavior remains unchanged as compatibility implementation, not target authority. | `SNOW-COE-STAGE3-MELT-OWNER-AUTHORITY-RECONCILIATION` frozen adjudication and pinned libsnobal chronology |
| 6 | 2026-08-01 | Made closure tolerances operand- and unit-explicit: `1e-9 m` SWE equals `1e-6 kg m^-2` for the same residual and governs vapor-to-sublimation transfer closure; hourly/daily vapor aggregation and represented-layer lifecycle retain their distinct `1e-9 kg m^-2` predicates. | `SNOW-SURFACE-EB-04S` result-blind authority freeze and independent authority reviews |
| 5 | 2026-07-31 | Separated represented density-layer mass lifecycle from aggregate SWE/depth residual tolerances. Layers above `1e-9 kg m^-2` after named SWE conversion remain represented with all coupled state; the independent `1e-9 m` closure guards remain unchanged. | `SNOW-SURFACE-EB-04D` authority reconciliation and required runtime replay |
| 4 | 2026-07-31 | Defined the exact libsnobal `1 kg m^-2` branches. Total mass `<=1` suspends before partition while CoE retains snow state; in a resolved pack, lower mass `<1` collapses to one thermal volume and continues, while lower equality remains two-volume. Both branches publish explicit diagnostics. | `SNOW-SURFACE-EB-04C` authority reconciliation and required runtime replay |
| 3 | 2026-07-30 | Replaced the failed snowfall-event top-layer provider with the Marks/SNOBAL upper-`0.25 m` active thermal control volume, harmonic active/lower `G_0`, conservative depositional-layer projection, and mass-dependent `60/15/1 minute` substeps. The amendment retains CoE snow existence/melt authority and prohibits shallow-pack temperature replacement, cold-content tax, fitted limiter, or new user coefficient. | `SNOW-SURFACE-EB-03A` contract-first authority trace |
| 2 | 2026-07-30 | Selected the Stage 3 top-layer thermal provider; bound `T_c=T_a`, polar-night typed unavailability, `R_a,min`, orthogonal default-off selectors, exact-one vapor/latent composition, snow-state mutation, and mass/energy closure obligations. Real S/LS execution then retained the seam as diagnostic/reproduction-only and opened `GAP-SNOWENERGY-007` because the common provider reaches `0 K` with material SWE remaining. | `SNOW-SURFACE-EB-03` contract-first implementation and terminal consumer evidence |
| 1 | 2026-07-30 | Initial contract: atmospheric longwave, effective-cover-derived diffuse sky view, complementary canopy exchange, runtime hold, and analytical obligations. | `SNOW-SURFACE-EB-01A` and `SNOW-SURFACE-EB-02` static/analytical evidence |
| 14 | 2026-08-20 | Bound the default-off shared V11/Stage 3 canopy-air carrier, complete turbulent residuals, reciprocal canopy--snow--sky longwave, sealed exposure wind, wrong-regime guards, and implementation-only disposition. | Child 2C authority package |
| 15 | 2026-08-22 | Selected the prospective one-column-per-lane OFE-ground area basis, complete tile-set weighted boundary, uniform-depth terminal identity, and topology-bound restart rule; prohibited covered-subset renormalization. | Direct user authority in Stage-3/V11 covered consumer package |
| 16 | 2026-08-22 | Admitted unit-specific covered fixed-point comparisons with independently reconstructed fingerprints and exact structural/density/count fields; reserved canonical lane receipt V2 without initial-guess identity and froze restart semantic joins. | Direct user authority and covered replay review |
