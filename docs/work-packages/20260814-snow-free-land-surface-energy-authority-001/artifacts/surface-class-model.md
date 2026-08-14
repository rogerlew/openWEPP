# Surface Class Model

## Bare mineral soil

The CLM5 soil humidity, dry-surface-layer resistance, thermal conductivity,
heat capacity, and Crank--Nicolson column equations are selected. Caller
configuration supplies complete texture, porosity, organic fraction, layer
geometry, optics, emissivity, and initial temperatures. No soil color or
thermal property is inferred.

## Forest litter

ISBA-MEB Appendix A equations (A1)--(A14) are selected with one deliberate
ownership substitution: `W_l` is a hydrology-owned typed mass store observed
by LSE, not a second LSE mass. LSE retains `T_l`; its heat capacity is
`C_l=dz_l*rho_ld*C_ld+W_l*C_w`. The snow/frozen terms are unavailable in this
first model. Litter water capacity, dry density, heat capacity, thickness,
VIS/NIR albedo and emissivity are required configuration; published site
defaults are not executable defaults.

When litter is configured, direct mineral-soil evaporation is zero. Litter
drainage reaches hydrology, which alone partitions infiltration/runoff.
