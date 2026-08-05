# Physical Authority Matrix

Status: complete

Evidence mode: Static + Ran

## Requirement Map

| Full energy-balance requirement | Independent authority | Current CoE representation | Finding |
| --- | --- | --- | --- |
| Melt begins only after snow thermal deficit is satisfied and the melting surface reaches `0 C` | Marks et al. 1998 pp. 1576-1577; Marks et al. 1999 pp. 1937-1939; Ohmura 2001 pp. 754-756 | daily air-temperature midpoint gate; no surface temperature or cold-content operand | material authority gap |
| Net radiation resolves absorbed shortwave and incoming/outgoing longwave using surface state | Marks et al. 1998 pp. 1576-1579; Ohmura 2001 Eq. 2 and pp. 754-756; Walter et al. 2005 pp. 66-68 | `A` is radiation times a legacy coefficient/canopy factor; `B` mixes air temperature and clear-sky proxy | empirical analogy only |
| Sensible and latent exchange depend on surface-air temperature/vapor gradients, wind, roughness, and stability | Marks et al. 1998 pp. 1576-1579; Walter et al. 2005 Eqs. 12-13; Marks et al. 1999 | `C` uses a weighted air/dewpoint temperature sum, fixed height adjustment, and canopy air-temperature branch; no snow-surface gradients or stability | material authority gap |
| Precipitation heat uses precipitation phase, temperature, mass, and heat capacity | Marks et al. 1999 pp. 1938-1939; Walter et al. 2005 Eq. 15 | `D` uses rain depth and positive dewpoint, otherwise air temperature | empirical approximation; no defect authority |
| Canopy modifies radiation and turbulent exchange through explicit transfer geometry/state | Marks et al. 1998 forest/open comparison; `SC-SNOWENERGY-001:193-202,325-326` | linear canopy attenuation plus a large air-temperature-only `C_canopy` branch | material canopy-process authority gap |
| Energy and mass ledgers remain distinct and close | Marks et al. 1998 Eq. 1; Marks et al. 1999 output definitions; `SC-SNOWENERGY-001:547-552` | CoE depth terms close mass arithmetic; Stage 3 separately closes energy but cannot turn positive excess into melt | CoE closure passes here; receipt-bound 21L `target/snow_warm_mixed_prepeak_loss_energy_attribution_v2/results.json` reports independent Stage-3 maxima `2.6329172132452672e-17 m` and `2.4985638447105885e-08 J m^-2`, but melt ownership remains separate |

## Quantitative State Exposures

Ran evidence from `quantitative-audit.json` shows that `C` remains the largest
positive empirical component at every canonical site (`41.61-53.38%` of the
sum of positive `A/B/C/D` contributions). Yet its frozen subcomponents show
the net open wind/dewpoint branch is negative at all four sites
(`-4.368` to `-0.162 m`), while the canopy air-temperature branch is positive
and much larger (`4.924-14.423 m`). These signed all-hour sums do not
decompose the positive-only `C` sum or prove causation. They establish only
that `C_canopy` is the sole net-positive algebraic `C` subcomponent at site
scale and therefore the first post-handbook assumption needing authority.

Positive raw applied CoE melt also occurs under state combinations that a
physical energy solver would resolve using surface state:

| Site | Positive applied hours | Dewpoint `<=0 C` | Hourly air `<=0 C` | Same-hour snowfall | Interval-start density `<350 kg m^-3` |
| --- | ---: | ---: | ---: | ---: | ---: |
| Mica Creek | 27142 | 19068 | 218 | 6751 | 27136 |
| Niwot | 21917 | 19302 | 379 | 1379 | 21917 |
| Paradise | 59595 | 34844 | 615 | 27669 | 59568 |
| Snowbird | 33381 | 26093 | 362 | 4722 | 33367 |

These are raw-generation exposures, not routed-melt counts and not causal
defect proof. They show that air/dewpoint signs and interval-start density do
not encode the state used later for cap, retention, or routing. Same-hour
snowfall is mixed into an existing pack on a nonnegative daily-midpoint caller
path before the CoE call; a new pack from inactive interval-start depth is
deliberately not melted that hour.

## Authority Boundary

The contracts already contain the surface temperature, active/lower cold
content, radiation, vapor/latent, and conduction concepts used by their full
energy-balance Stage 3. They also intentionally state that CoE remains the
melt owner and that positive Stage 3 energy cannot become melt. That is a
coherent ownership decision, not evidence of a defect. The gap is the missing
independent validation and bounded transferability authority for the material
post-handbook CoE changes; the physical-state separation identifies the seam
a successor contract must adjudicate.

Ohmura 2001 explains why temperature-index methods can be practically accurate:
air temperature carries information about multiple energy terms. Walter et al.
2005 likewise notes contexts where added process resolution may not be
justified. Neither source validates the specific 2007/2008 CoE changes or
their canopy branch across openWEPP's claimed production domain.

No paper in scope supplies a drop-in replacement equation or transferable
numeric coefficient set for openWEPP. No counterfactual melt was calculated,
and no tuning or site-specific inference is authorized.
