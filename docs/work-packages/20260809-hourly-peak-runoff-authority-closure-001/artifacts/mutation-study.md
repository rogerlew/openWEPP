# openWEPP Topanga Mutation Study

Status: `corrected exact-anchor probe passed; complete cohort running`

Evidence mode: `Ran (bounded probe)`

The terminal study retains both Topanga strata and the frozen ±1% first-layer
Ksat / ±0.01 paired-cover design. Legacy execution and solver replay are not
part of this artifact.

The release hillslope executable at implementation anchor `949349e70` has
SHA-256 `569f586516283c633cf4a2c99eb4c89725f8c57c476047b7b03a0b59e327ca88`.
A fresh one-baseline/one-mutation probe paired 1,832 event rows with finite
positive peaks, no invalid maximum-hour fractions, no runoff/peak zero-topology
mismatch, and peak-response decomposition residual `2.22e-16`. The maximum
shape ratio was `1.05878` and p99 was `1.00106`; peak-ratio p99 was `1.04473`.
The raw maximum peak ratio (`1.94e10`) is a near-zero-denominator diagnostic,
not a volume-stable doubling: the count with runoff volume within 5% and peak
change at least 2x was zero. External probe evidence is under
`/home/workdir/openwepp-hourly-peak-topanga-probe-v2`.

This probe does not satisfy the 1,088-trial acceptance criterion; the complete
cohort is executing in a fresh provenance-bound evidence root.
