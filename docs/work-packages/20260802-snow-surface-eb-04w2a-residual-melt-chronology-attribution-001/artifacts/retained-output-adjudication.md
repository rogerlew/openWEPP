# Retained-Output Adjudication

Evidence mode: **Ran**.

Independent review found two aggregation defects in the frozen runner's
human-facing flags after execution. The runner, freeze, receipt, and raw output
remain unchanged; no model or harness cell was rerun.

The raw files are quarantined as immutable evidence:

| File | SHA-256 |
|---|---|
| `melt-chronology-diagnostic-results.raw.json` | `96e0b40a879bdb47404fc48bdce66e3d9904f1f0974b9b257fdd90cce800365c` |
| `melt-chronology-diagnostic-summary.raw.csv.gz` | Archive: `dcfebaa34f5932d1d8dc848c19c16f649bcfbd2070c6267898808a27f232802e`; uncompressed CSV: `f82c72f6aded10216fcea9966c8398a6cc8828ad7bbab7ccafdab308e5fa71b9` |

The raw CSV is stored with deterministic gzip metadata so its exact CRLF bytes
remain recoverable without publishing a whitespace-invalid text diff. The
authoritative same-base-name JSON and CSV are adjudicated publication
views. They set harness chronology, trajectory, and albedo results to null and
publish only admissible direct-production fields.

## Cold-Content Reconstruction

The raw site flag separately takes the median melt fraction and median melt
depth, then applies the frozen `fraction >= 0.10 OR depth >= 0.010 m` rule.
Independent adjudication instead applies that unchanged disjunction to every
operator window and reports prevalence:

- Mica Creek: `8/23`;
- Niwot: `16/40`;
- Paradise: `0/19`; and
- Snowbird: `12/22`.

The raw false site flags remain provenance, not the controlling interpretation.

## Late-Input Reconstruction

The raw runner included early, tied, and late modeled peak windows even though
the prospective rule applies only to an early modeled peak. Filtering to
`modeled_date < observed_date` yields:

- Niwot: `0/27` windows meet the deficit rule; and
- Snowbird: `5/16` windows meet the deficit rule.

The site-level Boolean direction survives, but the corrected population and
prevalence control. Observed SWE gain is a net storage change, so these screens
neither prove nor rule out precipitation error.

## Term-Balance Naming

The raw field `TURBULENT_EMPIRICAL_TERMS_DOMINANT` is retained only for frozen
provenance. Its actual comparison is empirical `|b + c| > |a + d|`, where `b`
mixes temperature/cloud and `c` mixes wind/dew point. Adjudicated artifacts use
the equation rather than calling it a pure turbulent-flux partition.
