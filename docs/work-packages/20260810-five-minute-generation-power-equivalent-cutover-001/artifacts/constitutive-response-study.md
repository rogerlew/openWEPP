# Constitutive Response Study

Status: `executed — bounded screen rejects every fixed exponent`

Evidence mode: `Ran`

The compact machine result is `constitutive-response-study.json`, SHA-256
`c22ed2096fd88e78cff52f783572328899322a619645f9d590068a92276e56b9`.
No Topanga mutation outcome was opened.

For the only structurally admissible two-moment reduction, relative-error
summaries were:

| p | Shear p95 / max | Transport p95 / max | Detachment driver p95 / max | Deposition driver p95 / max |
|---|---:|---:|---:|---:|
| 4/3 | 6.03% / 6.56% | 1.54% / 2.25% | 32.65% / 33.54% | 59.56% / 59.56% |
| 3/2 | 6.85% / 7.41% | 1.73% / 2.72% | 34.03% / 34.91% | 60.80% / 60.80% |
| 2 | 8.90% / 9.56% | 2.75% / 3.92% | 37.41% / 38.29% | 63.81% / 63.81% |

All three two-moment candidates close volume and their selected `g^p` moment
to floating-point precision. The fixed-hour candidates preserve `g^p` but
have maximum volume errors of 86%, 129%, and 246% for `p=4/3,3/2,2`, so they
are structurally inadmissible for the current rate-duration consumer.

The `p=4/3` two-moment candidate approximates the screened shear and Yalin
capacity responses well on this library, but fails both screened normalized
detachment and reciprocal deposition factors materially. This matters because
the real Wave-1 consumer uses those drivers; selecting transport alone would
ignore another contract-active nonlinear response. The coarse any-transport
flag did not change in this finite library. That is not a full threshold-
topology or proximity claim and is insufficient to overcome the material
screened errors or missing equation-level exponent authority.

Rill width is a persistent state, not a flux. Its reset-seed integrated diagnostic was
retained only as sensitivity evidence; it is not used as standalone acceptance
authority. End-slope/`ktrato`, critical-shear and erodibility states, `qin`,
full continuity, invalid-domain census, persistent-state chronology, Rust
parity, and Topanga outcomes were not run. The screen is adequate only for the
conservative rejection; it cannot admit a candidate.
