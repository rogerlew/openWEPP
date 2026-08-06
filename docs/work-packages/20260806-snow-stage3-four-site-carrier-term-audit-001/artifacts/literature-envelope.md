# Literature Context Freeze

Status: `frozen before result execution`

These are source-specific comparison ranges, not universal forest-snow
climatology, calibration targets, independent validation bounds, or science
authority for changing openWEPP.

| Source | Comparable quantity | Frozen context | Use and limit |
| --- | --- | --- | --- |
| Marks et al. (1998), doi `10.1002/(SICI)1099-1085(199808/09)12:10/11<1569::AID-HYP682>3.0.CO;2-L`, Figure 7 | forest climate-period net all-wave, combined turbulent, advection, soil heat, and total | respectively about `[-4,+6]`, `[-2,+7]`, `[0,+10]`, `[0,+2]`, and `[-5,+20] W m^-2`, visually read at plot precision | Event/climate-period order-of-magnitude context. It is not a seasonal probability interval. The text says snow development and cold periods remain at or near zero and the forest rain-on-snow period is distinctly positive. |
| Roth and Nolin (2017), doi `10.5194/hess-21-5427-2017` | seasonal energy partition at low-, mid-, and high-elevation forest sites | net longwave accounts for `82%`, `88%`, and `59%` of total energy inputs; turbulent fluxes are episodic and not significant at monthly/annual scales outside the exposed high-open site | Direction/partition context only. Their maritime sites demonstrate that subcanopy longwave need not be strongly negative. |
| Webster et al. (2016), doi `10.1002/2015JD024099` | short spring subcanopy net-longwave events | approximately `+40 W m^-2` upper event context | Event bound only; never a seasonal target. |
| Marks et al. (1999), doi `10.1002/(SICI)1099-1085(199909)13:12/13<1935::AID-HYP868>3.0.CO;2-C` | complete two-layer snow balance | net all-wave, sensible, latent, ground, and precipitation-advection are distinct external terms; active/lower exchange is internal | Equation/topology context, not a numeric range. |

The prospective accumulation near-balance screen is `[-5,+5] W m^-2` for
annual resolved-support external carrier means. The broader Marks climate-period
context is `[-5,+20] W m^-2`. These `ASSUMED_FOR_EXECUTION` screens implement
the campaign's registered expectation without claiming that every real forest
season must satisfy them.

Per-term comparison rules:

- compare `shortwave + longwave` with Marks net all-wave;
- compare `sensible + latent` with Marks combined turbulent heat;
- compare precipitation advection directly;
- compare complete external total without internal active/lower redistribution;
- report shortwave and longwave separately and compare their partition with
  Roth and Nolin descriptively; and
- report the Marks soil-heat range beside an explicit `NOT_IMPLEMENTED` model
  field. Never fill it with internal conduction or zero and call the budget
  complete.
