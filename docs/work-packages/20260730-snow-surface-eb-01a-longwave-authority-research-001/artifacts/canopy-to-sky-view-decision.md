# Canopy-To-Sky-View Decision

Decision: effective hemispherical sky view is an internal derived state. The
first EB-02 implementation must not require users to enter a new sky-view
coefficient or obtain hemispherical photography, LiDAR, or other remote
sensing.

## Authority-backed base

FSM2 derives diffuse canopy transmission as:

`tau_d = exp(-1.6 k_ext VAI_eff)`.

With its published randomly oriented canopy default `k_ext = 0.5`:

`f_sky_candidate = exp(-0.8 VAI_eff)`.

FSM2 uses the same diffuse transmission for atmospheric longwave through the
canopy. This is the preferred base for EB-02 rather than a direct
canopy-cover/sky-view alias.

## Available openWEPP state

The derived operator must use coefficients already carried by the canopy
model:

- dynamic leaf area index;
- dynamic canopy-cover fraction, including the structural canopy floor for
  native forest;
- structural canopy-cover fraction where the derivation needs to distinguish
  persistent woody structure from seasonal foliage; and
- canopy height only when an admitted formulation gives height a defined
  geometric role.

Height must not be inserted merely to make use of an available coefficient.
For a homogeneous Beer-law canopy, area density and extinction control diffuse
transmission; height becomes necessary only if EB-02 admits a finite-crown,
gap, or other explicit geometric correction.

## Required properties

The EB-02 mapping from existing canopy state to `VAI_eff` and then `f_sky`
must:

- be deterministic and bounded in `[0,1]`;
- equal one for a genuinely open surface;
- decrease monotonically as effective obstructing area increases;
- retain woody obstruction during deciduous leaf-off rather than allowing
  leaf-only LAI to imply a completely open sky;
- distinguish radiometric diffuse transmission from plan-view canopy cover;
- state the roles and units of LAI, structural cover, dynamic cover, and
  height;
- use literature constants without site fitting; and
- fail explicitly on invalid existing coefficients rather than request a new
  user input.

The exact `VAI_eff` composition remains an EB-02 canonical-contract decision.
This package does not authorize an improvised algebraic blend of cover and
LAI.

## Evidence role

Hemispherical photographs, LiDAR, canopy-height models, and observed
sub-canopy radiation are valuable independent validation or uncertainty
evidence. They are not required runtime inputs and their absence does not
block contract authoring or implementation under ADR-0042.

## Admission effect

The prior “typed user/GIS sky-view provider” prerequisite is withdrawn.
EB-02 contract research is admitted to bind the deterministic derived
operator. Runtime implementation remains held only until that operator, the
cloud mapping, and the active snow-temperature provider are admitted into the
canonical contract.

Primary formulation authority: Essery et al. (2008) for hemispherical
radiative weighting and Essery et al. (2025), FSM2 version 2.1.1, Equation 14,
for diffuse transmission
`tau_d = exp(-1.6 k_ext VAI_eff)`.
