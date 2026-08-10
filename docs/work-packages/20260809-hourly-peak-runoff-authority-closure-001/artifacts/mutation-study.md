# openWEPP Topanga Mutation Study

Status: `complete — PASS`

Evidence mode: `Ran (complete frozen cohort)`

The terminal study retains both Topanga strata and the frozen ±1% first-layer
Ksat / ±0.01 paired-cover design. Legacy execution and solver replay are not
part of this artifact.

The production-equivalent release hillslope executable has SHA-256
`ac8790faf32a5b98993427b636084c04ba468955458c4fc18f3874cea709c4c3`.
It was built from terminal implementation/contract/test commit
`33831787b7029b28b0716c8458f08a11899db446`. The terminal production delta
names the already-enforced absolute seconds duration-custody threshold; the
fresh binary and cohort were rebuilt rather than relying on prior byte identity.

The census executed all 1,088 eligible trials and 280 unique baselines from
the frozen plan. Its plan SHA-256 is
`32e6f5e99a77747fcdd93388302f2a5ffb496a87b764ac4505e09691955db756`.
The terminal run used eight jobs, took 498 seconds by package-log wall time,
and paired 1,913,199 event rows. Of those, 1,913,158 pairs had finite positive
peaks. Validation
found:

- zero invalid maximum-hour fractions;
- zero runoff/peak zero-topology mismatches;
- maximum peak-ratio decomposition relative residual of
  `4.440892098500626e-16`;
- maximum and p99 maximum-hour-fraction ratios of `2.755595239734283` and
  `1.0000004332094452`;
- maximum and p99 peak ratios of `12965889426731.332` and
  `1.0000000000000002`; and
- zero pairs with runoff volume within 5% and peak changing by at least 2x.

The raw maximum peak ratio is a near-zero-denominator diagnostic. It is not a
volume-stable discontinuity, and the independent decomposition closes it to
the corresponding hourly runoff-depth response. The accepted result is that
the frozen small mutations expose no unexplained branch discontinuity in the
hourly peak operator.

Authoritative external evidence is under
`/home/workdir/openwepp-hourly-peak-topanga-census-20260809-v5`; the event-pair
table is `topanga-openwepp-event-pairs.parquet`, and the terminal summary is
`summary.json`. The package log is
`artifacts/topanga-openwepp-census-full-v5.log`. Resume admitted only
schema-v3 receipts whose plan, binary, complete primary/sidecar inputs, row
count, and calendar hashes matched.

This is a hillslope mutation study, not calibration or validation against
observed flow. It makes no legacy-parity, instantaneous-peak, subhourly,
routed-watershed, or channel-flow claim.
