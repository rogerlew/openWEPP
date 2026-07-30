# Formulation Decision

Decision: `GO_WITH_PREREQUISITES` for a contract-first EB-02.

## Selected stand-scale formulation

For homogeneous forest stands away from gaps and edges:

`L_sub_down = f_sky L_atm_down + (1 - f_sky) sigma T_c^4`

`L_net = L_sub_down - sigma T_s^4`

Fluxes are in `W m^-2`; positive net longwave is toward the snow. `f_sky` is
the hemispherical radiometric sky-view fraction. The canopy term displaces,
rather than supplements, the obscured sky term. The selected stand-scale path
fixes effective canopy and snow emissivity at exactly one. Rutter et al. (2023)
justify effective canopy unity through multiple scattering beneath the canopy;
Sicart et al. (2004) use the same full-emitter simplification for vegetation
and snow. Non-unity material emissivity would require reflected-longwave and
multiple-exchange terms and is not authorized by this candidate.

The preferred first candidate uses an effective canopy temperature equal to
the available hourly open-air temperature only as an explicit, named
homogeneous-stand approximation. Rutter et al. (2023) obtained average RMS
errors of `6.8` and `8.4 W m^-2` with measured `0.5 m` sub-canopy air
temperature and measured sky view at leaf-off birch and conifer sites. Those
numbers do not quantify the error of openWEPP's open-air forcing.
Above-canopy/open air temperature is less accurate during clear, calm stable
nights; that limitation must be a claim boundary and a diagnostic based on
cloud, wind, and nighttime state, not hidden by fitting.

## Why not an explicit trunk component

Webster et al. (2016) show that a third, measured trunk-view component improves
high-insolation conditions and can reduce RMSE by as much as `7.7 W m^-2`.
Musselman and Pomeroy (2017), however, show that trunk effects are highly
localized: the trunk view factor was about `0.05` even one meter from the
trunk, while canopy-emitted energy dominated. Explicit trunk temperature and
geometry are therefore appropriate for forest gaps, edges, and individual-tree
models, not the initial hillslope stand-scale candidate.

## Binding prerequisites

EB-02 may start contract-first only after it prospectively binds:

1. the deterministic canopy-state-to-sky-view operator specified in
   `canopy-to-sky-view-decision.md`, using existing canopy cover, LAI,
   structural cover, and height only where scientifically defined;
2. the corrected Dilley-O'Brien clear-sky equation and selected
   Unsworth-Monteith cloud correction in
   `atmospheric-longwave-formulation.md`;
3. a contract-matched cloud-fraction derivation rather than automatic reuse of
   the legacy hourly cloud surface;
4. fixed effective canopy and snow emissivity of exactly one;
5. one authoritative active snow-surface temperature;
6. the named air-temperature proxy and its homogeneous-stand, non-edge,
   stable-night claim limits; and
7. uncertainty diagnostics that do not masquerade as calibration.

Current canopy cover and LAI must feed an admitted derived operator; neither is
silently relabeled as `f_sky`. No new user coefficient or remote-sensing input
is allowed. The current legacy cloud fraction and competing snow-temperature
surfaces are also not silently selected. Consequently, this package admits an
EB-02 canonical-contract research/amendment step but retains a hold on runtime
implementation until the contract binds the derived mapping and those
providers.
