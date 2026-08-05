# Scientific Synthesis

Status: complete / reviewed and verified

Evidence mode: Static + Ran

## What changed after 21K

The corrected exact-one wet-compaction operand does not change upstream snow
mass. All `253` canonical guarded dry intervals reproduce the 21J pack-loss
values within `9.02e-17 m`. Therefore 21K removed a density, depth, and
downstream-liquid-disposition confounder but neither created nor removed the
pre-peak loss signal.

## When loss occurs

Across `154` canonical annual windows, the site-median fraction of pre-peak
pack loss assigned to mixed or warm days is:

| Site | Years | Median loss | Mixed/warm loss | Mixed/warm share |
| --- | ---: | ---: | ---: | ---: |
| Mica Creek | 34 | `0.3084 m` | `0.3083 m` | `0.9965` |
| Niwot | 44 | `0.2883 m` | `0.2880 m` | `0.9991` |
| Paradise | 41 | `0.7313 m` | `0.7266 m` | `0.9961` |
| Snowbird | 35 | `0.5296 m` | `0.5280 m` | `0.9987` |

Every site passes the frozen warm/mixed screen, and every eligible annual
window has at least half its loss in that class. This sharply localizes
chronology, but it is not surprising that a positive-melt routine loses snow
when some active-pack hours exceed freezing. The classification does not prove
that daily air temperature alone is biased or that its threshold is wrong.

## Which modeled driver family is implicated

Annual positive applied CoE melt and pack loss correlate at `r=0.9995-0.9998`
and Spearman `rho=0.9964-0.9994`. This is strong lineage localization, not
independent physical validation: pack loss is downstream of that generated
melt.

`cmelt` is the largest positive empirical warm/mixed term at all four sites;
annual-first medians are shown in the figure sidecar. It mixes wind, dewpoint,
canopy, temperature, and snow-state inputs. It cannot be called pure sensible
or turbulent heat. Radiation `amelt` is not the dominant positive term, so the
existing evidence does not support a shortwave-only explanation.

## Which forcing and state signals accompany material loss

An annual-first descriptive contrast compares each water year's median
material-loss day with its median eligible nonmaterial-loss day. Across Mica,
Niwot, Paradise, and Snowbird respectively, material-loss days are warmer by
`4.14`, `7.80`, `4.70`, and `7.19 C`, and have dew points higher by `3.80`,
`7.18`, `4.10`, and `6.85 C`; both directions occur in every eligible year.
Radiation is generally higher (`+0.64` to `+4.04 MJ m^-2`), whereas wind is not
systemically higher and the modeled cloud-proxy fraction is flat at Mica and
generally lower at the other sites. Pack density is higher by `100-147 kg m^-3`, depth is usually
lower, and retained liquid is generally higher.

These contrasts localize warm, moist, generally more radiative, denser-pack
chronology. They are strongly state- and chronology-confounded: they do not
validate the forcing, establish causation, or authorize a correction. The
evidence argues against a wind-only or shortwave-only account.

Stage-3 shortwave response is nonzero, while explicit longwave and latent
energy are zero under the executed selectors. Stage 3 remains downstream of
CoE generation and its mass/energy ledgers close; it cannot own the upstream
loss.

## Snowbird input sensitivity

Across `35` paired water years, the precipitation-scaled development lane adds
a median `0.1258 m` peak SWE and `0.1459 m` SWE on the observed peak date. It
also adds `0.0190 m` median pre-peak pack loss, with the same delta direction in
`94.3%` of nonzero years. More input therefore creates more stored snow and
some additional state-exposed loss; storage gain is much larger than loss gain.

This keeps Snowbird multifactor. The scaled lane is not forcing truth and does
not turn the result into a calibrated precipitation correction.

## Bounded conclusion

The frozen matrix returns `MULTIFACTOR_WARM_MIXED_AND_STATE_SIGNAL`. The next
scientifically efficient step is a read-only first-principles authority audit
of the active warm/mixed CoE melt formula, especially `cmelt` chronology and
its wind/dewpoint/canopy/temperature/snow-state operands. Such an audit must
compare formula assumptions with canonical chapter/legacy provenance and
independent physical energy-balance authority before any code change.
