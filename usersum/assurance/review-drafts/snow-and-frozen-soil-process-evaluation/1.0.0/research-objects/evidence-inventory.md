# ASSURE-06 Evidence Inventory

Evidence class: Static

## Primary Quantitative Sources

| Evidence | Assessed role | Exact SHA-256 | Main quantities | Important boundary |
| --- | --- | --- | --- | --- |
| `docs/work-packages/20260627-snowdensity-10-3-5b-hourly-partition-jennings-validation-001/artifacts/jennings-validation-report.json` | Observed precipitation-phase comparison | `f5d261379f2aaed862a4ad6734e76e3d8123f56df46ca7739fa2fab86c2d6ef8` | 17,810,805 read rows; 11,711,058 scored rows; 6,883 stations; confusion matrices; threshold errors | The corpus informed selection of the phase model; this is not untouched post-selection validation. |
| `docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/artifacts/harder-pomeroy-default-activation.json` | Current-default snow profile and production trace | `f511c11d73b2a0b03cb7ef8f573ddc9309ffd336f2790cd1218514a74565747a` | Ten comparison surfaces; 90 available robust cells; selector counts; 53,711 precipitation rows; partition residual | Ordinal cells supported development/activation and are not a universal performance score. |
| `docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/artifacts/post-partition-residual-decomposition.json` | Snow residual diagnosis | `0225ff80580ef352b2b91720da947f7f16f909a48827fb47e6c207da5b4e8875` | Signed SWE/depth/density directions; 15 residual robust cells; density/timing/geometry families | Diagnostic interpretation; forcing-limited absolute magnitudes do not carry mechanism verdicts. |
| `docs/work-packages/20260625-snowfrost-fidelity-i0-non-snotel-rubric-baseline-001/artifacts/non_snotel_rubric_baseline.json` | Frozen-soil observational comparison | `b3806ced25cf01eb4c7558eee8e9d7f3f486633aa708e93dbe63b115e76a8930` | Three frost-tube and two soil-temperature sites; matched counts; residual extrema; snow-control counts | Historical assessed realization; all sites were snow-confounded or lacked paired snow observations. |
| `docs/work-packages/20260713-integrated-validation-campaign-001/artifacts/final-conservation-and-consumer-evidence.md` | Production conservation and consumer verification | `306b96a1d45fca85d5604b16fe8ce4b814df48d2fc15ecb910e198085ee81f18` | Snow and frozen-soil WAT storage residuals and consumer hashes | Software/integration verification, not environmental accuracy. |

## Scientific And Dataset Authority

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
  SHA-256 `f7364f33ab446479b9160c5cf0bafef13826fd2637e140c823dfde2204cb1a16`.
- `tests/fixtures/precip_phase_observed/jennings2018/` retains the admitted
  Jennings observations, station thresholds, source code, and documentation.
- `tests/fixtures/snotel_observed/` retains five normalized station records,
  provenance records, manifests, and data-quality characterization.
- `tests/fixtures/cancov_forest/` retains the canopy-site comparison surfaces.
- `tests/fixtures/snowfreeze_observed/` retains three frost-tube and two
  soil-temperature sites with observation provenance.
- `usersum/snow-frost-modeling-and-validation.md`, SHA-256
  `6c861e573d0b087c1b49cea90a2ce2b62511d4bbfdbae000a38e30763f2588fe`,
  supplies the durable model rationale but is not a result source.

## Prior-Knowledge Sources To Cite

The manuscript must cite the primary sources already anchored by
`SC-SNOWFREEZE-001`, including Harder and Pomeroy for hydrometeor-temperature
phase partition, Anderson/SNOBAL lineage for bulk snow evolution, Jennings et
al. for observed precipitation phase, Sturm for snow-climate/density context,
and the admitted frost/soil-temperature dataset publications. Exact citations
and immutable identifiers must be checked against the contract/reference
records rather than reconstructed from memory.

## Excluded As Claim Authority

- The retired V1 SNOTEL dossier and method.
- Package disposition labels such as `ACTIVATED`, `PASS`, or `HOLD` when not
  accompanied by underlying operands.
- Legacy WEPP or PySnobal agreement as a truth criterion.
- Candidate mechanism profiles that were rejected or not activated, except as
  negative or alternative-mechanism evidence.
- Temporary trace paths that are not retained or content identified.
- Test counts as substitutes for scientific results.
