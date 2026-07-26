# Parameter Lineage

Evidence class: `Executed operand audit`

Bill starts from the WEPP Windows perennial grass template, prevents winter
plant death with `tmpmin=-40 C`, establishes senescence on Julian day 270, and
then tunes the growth, transfer, and decay controls against standing-stock
targets. The delivered plants are encoded as cropland perennials because the
98.4 management format has no active forest-plant parameter block.

| Operand | Hardwood | Santee mixed | Bill’s stated role | Authority status |
| --- | ---: | ---: | --- | --- |
| `beinp` | 13 | 17 | Control equilibrium live biomass. | Iteratively fitted; Santee change is not independently derived. |
| `dropfc` | 0.95 delivered; 0.92 report | 0.93 | Fraction of live biomass retained after senescence. | Hubbard is an unresolved two-branch discrepancy. |
| `decfct` | 0.50 | 0.50 | Fraction of canopy remaining after senescence. | Delivered operand; not discussed as a calibrated canopy target. |
| `spriod` | 30 d | 60 d | Spread senescence/litter transfer; longer for needle loss. | Bill-derived phenology assumption. |
| `jdharv` | 270 | 270 | Fixed start of senescence. | Bill-derived Northern Hemisphere calendar assumption. |
| `oratea` | 0.0021 d^-1 | 0.0021 d^-1 | Decay surface residue until equilibrium stock matches forest floor. | Iteratively fitted stock-control operand. |
| `orater` | 0.0021 d^-1 | 0.0021 d^-1 | Root-residue decay. | Copied numeric value; no root-data lineage. |
| `bb` | 5 m2/kg | 5 m2/kg | Biomass-to-canopy-cover coefficient. | Bill revised grass value 14; no field fit reported. |
| `xmxlai` | 6 | 10 | Maximum LAI cap. | Hubbard literature-aligned; Santee is unsupported. |
| `hmax` | 51 m | 51 m | Maximum canopy height. | Plausibility edit from grass; not calibrated. |
| `rdmax` | 3 m | 3 m | Maximum root depth. | Plausibility edit from grass; not calibrated. |
| `rtmmax` | 3.4 kg/m2 | 3.4 kg/m2 | Maximum perennial root mass. | Plausibility edit; no root-stock target. |
| `pltsp` | 6 m | 6 m | Plant spacing. | Plausibility edit from grass. |
| `diam` | 0.5 m | 0.5 m | Stem diameter at maturity. | Plausibility edit from grass. |
| `tmpmin` | -40 C | -40 C | Avoid annual winter mortality. | Numerical device, not a site frost threshold. |

## Leaf litter and residue transfer

WEPP applies `dropfc` to total modeled live aboveground biomass during the
senescence period. It does not know that one portion is leaf litter and another
is twig/branch litter. Bill’s report branch uses an 8% annual live-biomass
transfer: nominally 4% leaf drop plus an extra 4% for twigs and branches. The
delivered `dropfc=0.95` instead transfers 5%. Both send one aggregate material
into WEPP’s current residue pool, which ages into previous and old pools while
all pools decompose.

The `4% leaf drop` premise is not supported by the cited Yang paper. Yang’s
`7.6 Mg/ha` is standing foliage biomass. Relative to the Fahey
`189.9 Mg/ha` aboveground stock it is indeed about 4%, but annual abscission of
that standing deciduous foliage is a separate inference. Adding another 4% for
twigs and branches is Bill’s explicit judgment, not a measurement. Gresham
supports the general fact that aggregate litterfall includes roughly
one-quarter non-foliage material in two coastal pine stands, but it does not
support a universal 4% of live biomass twig transfer at Hubbard Brook.

`oratea` then closes the stock-flow loop by forcing aggregate residue toward
the observed forest-floor stock. This can match total mass while compensating
for an incorrect input flux, incorrect material mixture, or incorrect decay
kinetics. CANOPY-CAL-02 should reproduce that calculation; later calibration
must independently test flux, pool age, material class, and stock.

## Relationship to current WEPPcloud parameters

The existing WEPPcloud deciduous/mixed work used canopy-gradient fixtures to
obtain winter canopy separation. Its values (`dropfc=0.20/0.55`,
`spriod=45`, `xmxlai=5/9.5`) were chosen for canopy behavior, not fitted to
biomass and forest-floor stocks. Bill’s `dropfc=0.92-0.95/0.93` has a different
meaning and calibration surface. Directly transplanting either set into the
other pathway would conflate canopy retention with whole-plant biomass
retention.
