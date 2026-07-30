# Atmospheric Longwave Formulation

Decision: select the Dilley-O'Brien clear-sky equation with the
Unsworth-Monteith cloud correction as evaluated by Flerchinger et al. (2009).
This gives EB-02 one complete equation route without an unresolved cloud-base
temperature cycle.

## Clear sky

For screen-level air temperature `T_0` in kelvin and actual vapor pressure
`e_0` in kilopascals:

`w = 4650 e_0 / T_0`

where `w` is precipitable water in `kg m^-2`, numerically equivalent to
millimeters of liquid water. Equivalently, `w_cm = 465 e_0 / T_0` in
centimeters and the reference in the next equation is `2.5 cm`.

The corrected Dilley-O'Brien form reported in Flerchinger et al. (2009),
Table 1, is:

`L_clear = 59.38 + 113.7 (T_0 / 273.16)^6
           + 96.96 sqrt(w / 25)`

`L_clear` is in `W m^-2`; the three numeric flux coefficients carry
`W m^-2`. Define clear-sky effective emissivity for the cloud correction as:

`epsilon_clear = L_clear / (sigma T_0^4)`.

## Cloud correction

For cloud-cover fraction `c` in `[0,1]`, Flerchinger et al. (2009), Table 2,
give the Unsworth-Monteith correction:

`epsilon_all = (1 - 0.84 c) epsilon_clear + 0.84 c`

and:

`L_atm_down = epsilon_all sigma T_0^4`.

Flerchinger et al. found Dilley combined with Unsworth, Kimball, or Crawford
among the best all-sky methods and recommend those cloud corrections for most
sites. This package selects Unsworth because its complete correction uses only
a bounded cloud fraction.

## Cloud-fraction binding

The Flerchinger evaluation estimated cloud fraction from a solar clearness
index and optimized the complete-cloud and clear-sky limits. For the
Dilley-Unsworth pair, Table 9 reports `k_cld = 0.15` and `k_clr = 0.80`.
Between those limits:

`c = clamp((k_clr - k) / (k_clr - k_cld), 0, 1)`.

Here `k` is observed solar radiation divided by corresponding extraterrestrial
horizontal solar radiation over the declared window. Flerchinger found a
24-hour solar window appropriate for midnight estimates; shorter windows can
improve some daytime periods. Because openWEPP begins with daily solar
forcing, EB-02 should evaluate and bind the daily-window operator, not invent
subdaily cloud variation.

The existing `winter.hourly.cloud_fraction` is a daily SIMIMPL28 scalar
repeated across 24 hours. It is bounded but does not use the mapping above.
EB-02 must prove equivalence or publish a separate typed contract-matched
operand.

## Demonstrated uncertainty and limits

- Dilley-Unsworth had about `24.5 W m^-2` half-hourly/hourly RMSD and
  `14.9 W m^-2` daily RMSD in the Flerchinger development-site comparison.
- Cloud-cover uncertainty was a major error source, and the algorithms had
  difficulty reproducing diurnal variation.
- Those errors do not transfer automatically to openWEPP's legacy cloud
  operand or open-air temperature forcing.
- Solar-index cloud inference becomes poorly constrained or undefined during
  extreme-latitude winter with little or no daylight.
- These errors are model-form uncertainty, not calibration tolerances.

Provenance: Flerchinger et al. (2009), DOI `10.1029/2008WR007394`, corrected
version-of-record Table 1, Tables 2 and 9, sections 2.3-2.4, 4.6, 4.8, and 5.
