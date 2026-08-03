# Radiation-First Comparison

Status: `EXECUTED / DUAL REVIEW PASS / DUAL VERIFICATION PASS`

Ran: `tools/compare_radiation_first.py` compared all eight validated reanalysis
series with the checksum-bound SIMIMPL28 hourly exports. The machine-readable
result is `radiation-first-results.json`; it binds the acquisition manifest,
validated receipt, comparison manifest, eight NetCDF identities, and four
retained comparator identities.

This is an A2/A6 diagnostic comparison, not observational validation. The
primary shortwave magnitude lane compares ERA horizontal flux with the retained
daily Daymet/gridMET horizontal `rad` field. The hourly chronology lane uses
SIMIMPL28 synthesis reconstructed before its fixed `0.80` net-shortwave factor,
but that synthesis is slope/aspect transformed. It is therefore explicitly
geometry-confounded and cannot support a like-for-like magnitude/provider claim.
Retained longwave is a SIMIMPL28 diagnostic temperature/cloud emissivity
estimate, not a measured or independently observed field.

Each ERA de-accumulated radiation value is labeled by the start of its preceding
hour (`valid_time - 1 h`) before conversion to fixed local standard time. This
corrects the first attempted direct-validity-time join, which review found was
one hour out of phase with the retained export labels. The exact intersection
excludes 8 hours per boundary at Snowbird/Niwot and 9 at Mica/Paradise. Daily
and fixed-local-standard peak metrics use complete 24-hour paired days only.
Winter events are prospectively defined as wet retained-comparator days in
November through March. Precipitation only selects those days; no precipitation
byte or multiplier changed.

| Product | Site | Horizontal daily r | Horizontal bias | Winter daily r | Winter bias | Sloped hourly r | Peak abs h | Winter hourly r | Winter peak abs h |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| ERA5 | Mica | 0.945 | +11.8% | 0.810 | +12.8% | 0.959 | 0.63 | 0.889 | 0.82 |
| ERA5 | Paradise | 0.896 | -5.0% | 0.760 | -6.0% | 0.925 | 0.91 | 0.859 | 0.95 |
| ERA5 | Snowbird | 0.902 | +12.8% | 0.795 | +28.5% | 0.896 | 0.95 | 0.872 | 1.04 |
| ERA5 | Niwot | 0.893 | +1.5% | 0.760 | +23.1% | 0.948 | 0.56 | 0.920 | 0.63 |
| ERA5-Land | Mica | 0.945 | +12.4% | 0.803 | +12.9% | 0.959 | 0.63 | 0.885 | 0.84 |
| ERA5-Land | Paradise | 0.897 | -4.3% | 0.762 | -3.8% | 0.925 | 0.89 | 0.863 | 0.93 |
| ERA5-Land | Snowbird | 0.903 | +13.0% | 0.797 | +28.4% | 0.896 | 0.96 | 0.872 | 1.05 |
| ERA5-Land | Niwot | 0.893 | +1.5% | 0.761 | +23.0% | 0.948 | 0.56 | 0.920 | 0.63 |

The JSON receipt retains the corresponding longwave rows for all products,
sites, and winter-event subsets. Across all-site longwave lanes, reanalysis is
lower than the diagnostic comparator by roughly 5–14% for the full record and
5–15% on wet winter days; hourly correlations fall from `0.674–0.774` overall
to `0.398–0.502` on wet winter days, with fixed-local-standard mean absolute
peak offsets near `3.8–6.4 h`.

## Interpretation

The scientific outcome is `DIVERGES` from the retained synthesis and remains an
investigation signal. In the comparable horizontal daily lane, Snowbird has the
largest wet-winter difference at about `+28.5%`; Niwot is about `+23%` despite
only `+1.5%` full-period bias. Mica is about `+13%`, while Paradise is `-4%` to
`-6%`. The former `+84%` Snowbird estimate is withdrawn as a plane-confounded
comparison between horizontal ERA flux and hillslope-transformed SIMIMPL
shortwave. After interval-start alignment, the sloped hourly chronology lane
has correlations of `0.896–0.959` overall and fixed-local-standard mean
absolute peak offsets of about `0.56–0.96 h`; those remain geometry-confounded.

ERA5 and ERA5-Land produce nearly identical comparison metrics at these four
points, so the finer land grid does not resolve the retained-radiation
discrepancy. No optional-provider admission, causal attribution, or snow-model
improvement claim follows from this result. The temperature and joint-
meteorology lanes remain isolated and unexecuted, and radiation-only snow
chronology requires a separately reviewed diagnostic handoff that preserves
precipitation exactly.

## Figures

- [Horizontal daily shortwave relative bias](figures/radiation-horizontal-daily-bias.md)
- [Wet-winter shortwave correlation and bias](figures/radiation-winter-correlation-bias.md)
- [Geometry-confounded hourly shortwave chronology](figures/radiation-hourly-shortwave-chronology.md)
- [Diagnostic longwave relative bias](figures/radiation-longwave-diagnostic-bias.md)
