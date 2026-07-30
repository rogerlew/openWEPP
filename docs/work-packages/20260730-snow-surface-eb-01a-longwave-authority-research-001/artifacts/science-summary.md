# Science Summary

Sub-canopy longwave is not an extra canopy heat term pasted onto atmospheric
longwave. It is a view-weighted replacement: visible sky supplies atmospheric
longwave, and the obstructed hemisphere supplies canopy emission. Snow then
emits longwave upward, so the snow energy balance consumes their difference.

The authoritative stand-scale model is:

`L_net = f_sky L_atm_down + (1-f_sky) sigma T_c^4
         - sigma T_s^4`.

This selected formulation fixes effective canopy and snow emissivity at one.
That is the coherent Rutter/Sicart stand-scale simplification after canopy
multiple scattering. Using non-unity emissivities would also require reflected
longwave and gray-surface exchange and is outside this candidate.

Three findings control implementation:

1. Radiometric sky-view fraction is first order. It is a hemispherical,
   angle-weighted quantity and is not interchangeable with plan-view canopy
   cover. EB-02 will derive it internally from existing canopy cover, LAI,
   structural cover, and scientifically relevant height information using the
   FSM diffuse-transmission lineage; it will not request another user
   coefficient.
2. Effective canopy temperature can be represented by sub-canopy air
   temperature in homogeneous, continuous stands away from gaps and edges.
   Available open/above-canopy air is a weaker proxy during clear, calm stable
   nights, when the lower canopy and sub-canopy air cool.
3. Sunlit trunks matter near gaps, edges, and individual trees, but their view
   influence is highly localized. A separate trunk component would add
   unsupported geometry and temperature operands to a hillslope stand model.

Flerchinger et al. support a Dilley clear-sky atmospheric estimator combined
with several cloud treatments across broad climates. This package selects the
fully specified Unsworth-Monteith correction. Current openWEPP hourly air
temperature, daily dewpoint, and daily solar geometry can support its
contract research, but the legacy daily cloud fraction repeated across hours
is not presumed equivalent to the source clearness mapping. Hourly uncertainty
is material and the extreme-latitude winter limitation remains.

Outcome: the missing formulations are resolved. EB-02 is admitted
contract-first, but production implementation remains blocked until it admits
the deterministic canopy-state-to-sky-view mapping, a contract-matched cloud
fraction, fixed effective-unity treatment, and one active authoritative
snow-surface temperature. Remote sensing and observations are optional
validation evidence, not implementation prerequisites. No additional article
acquisition is currently needed.
