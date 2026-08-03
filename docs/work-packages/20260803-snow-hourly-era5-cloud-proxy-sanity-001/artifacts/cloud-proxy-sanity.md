# ERA5 Versus SIMIMPL Cloud-Proxy Sanity Check

Status: `COMPLETE / SANITY ASSOCIATION PASSES / NO PROMOTION`

Ran: `tools/compare_cloud_proxy.py` reconstructed the retained daily SIMIMPL
cloud proxy exactly from the checksum-bound diagnostic longwave/temperature
export and compared it with ERA5 total cloud cover on complete fixed-local-
standard days. The machine receipt is `cloud-proxy-results.json`.

The primary cloud statistic is ERA5 24-hour arithmetic-mean cloud, which is
independent of realized shortwave. Realized-`ssrd`-weighted cloud is retained
only as an outcome-dependent sensitivity: it downweights cloudy hours and is
mathematically coupled to the shortwave residual. Wet winter means complete
November-March days with unchanged retained precipitation greater than zero.

| Site | Wet n | ERA5 24 h cloud | SIMIMPL proxy | ERA - proxy | Daily r | MAE | Category agreement | Cloud residual vs SW residual r |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Mica | 3,120 | 0.879 | 0.960 | -0.080 | 0.321 | 0.113 | 83.4% | -0.510 |
| Paradise | 4,517 | 0.903 | 0.873 | +0.030 | 0.268 | 0.145 | 78.1% | -0.563 |
| Snowbird | 2,464 | 0.769 | 0.913 | -0.145 | 0.230 | 0.194 | 62.5% | -0.573 |
| Niwot | 2,247 | 0.654 | 0.722 | -0.068 | 0.121 | 0.295 | 42.8% | -0.764 |

Clear/mixed/overcast categories are `[0,0.25)`, `[0.25,0.75)`, and
`[0.75,1]`. Proxy reconstruction is numerically exact for practical purposes:
the largest within-day range is below `6.6e-14`.

## What We Learned

The sign test passes at all four sites. Mica, Snowbird, and Niwot have less ERA
cloud than the SIMIMPL proxy and more ERA wet-winter shortwave than retained
forcing. Paradise has slightly more ERA cloud and less ERA shortwave. The wet-
winter primary cloud-residual/shortwave-residual association is negative at
every site and ranges from `-0.510` to `-0.764`. This is a sanity association,
not independent validation or causation.

The proxy is not a close daily cloud-state surrogate. Wet-winter correlations
are only `0.121–0.321`; exact cloud-category agreement ranges from `42.8%` at
Niwot to `83.4%` at Mica. Snowbird shows the largest mean cloud discrepancy:
SIMIMPL is about `0.15` cloud-fraction units cloudier than 24-hour ERA5 during
wet winter days, consistent with its lower retained solar forcing.

This supports a limited sanity conclusion: SIMIMPL's transmissivity-derived
proxy has the expected inverse association with shortwave differences, but it
does not reproduce ERA5 daily total-cloud chronology. That is not proof the
proxy is wrong. ERA5 total cloud cover and a radiation-inferred effective cloud
proxy are different quantities, and neither is a colocated cloud observation.
No correction, provider admission, or snow-model improvement is claimed.

The realized-shortwave-weighted sensitivity preserves the four mean signs, but
it is not used to establish the sanity conclusion because realized `ssrd`
appears in both its weights and the radiation residual.
