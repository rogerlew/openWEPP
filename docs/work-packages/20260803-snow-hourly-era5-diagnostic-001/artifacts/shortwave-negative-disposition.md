# Shortwave Negative-Value Disposition

Status: `DISPOSITIONED / BOUNDED SOURCE-INGRESS NORMALIZATION`

## Physical Invariant And Source-Uncertainty Boundary

Surface solar radiation downwards is nonnegative. The
[ECMWF ecCodes GRIB FAQ](https://confluence.ecmwf.int/display/UDOC/Why%2Bare%2Bthere%2Bsometimes%2Bsmall%2Bnegative%2Bprecipitation%2Baccumulations%2B-%2BecCodes%2BGRIB%2BFAQ)
documents packing artifacts for start-of-forecast accumulations but explicitly
says ERA5 short-forecast fields archived after de-accumulation do not suffer
that described mechanism. It therefore does not establish GRIB-packing
causation here. These authenticated point-series negatives are classified more
narrowly as empirically bounded, nonphysical near-zero values of unresolved
source encoding/conversion origin. The rule rests on physical nonnegativity,
the checksum-bound observed range, and negligible correction magnitude—not a
claimed mechanism. It is limited to these source fields; it is not a
production-kernel clamp or permission to repair materially negative radiation.

## Ran: Complete Four-Site Characterization

The characterized cohort is exactly these eight long-range files; the five
retained Mica annual downloads from acquisition debugging are excluded:

| File suffix | SHA-256 |
|---|---|
| `era5-land...mica...1986-2024.nc` | `f7189f11280359c3450191421d469c88d51994ec504c097c7d95f0c6d6bd4233` |
| `era5-land...niwot...1980-2024.nc` | `47ba7b499f41c1fa58e4f26423551a579926358ea8a07ef279e4a3c8ac1e9329` |
| `era5-land...paradise...1980-2024.nc` | `0c4d3701b2e33b88b3e11cac84d4001f6c7e2171c99a6f8e26dffb547c0ced24` |
| `era5-land...snowbird...1986-2024.nc` | `5147f27b0e8e0a52b2309cb5181fa877040fc36994bb4c03ce47d7da6327e8cf` |
| `era5...mica...1986-2024.nc` | `6d8a00ddf0b1200702de2c6ef51f884177435815b78555cb6eb11bcf9f6200cc` |
| `era5...niwot...1980-2024.nc` | `97709b9f7d3c231f50e5bc07a62776421ffe313a01f1a420d08efd88c5058582` |
| `era5...paradise...1980-2024.nc` | `9a43da9ce6de331ed9f6d52e1710800ca0683240eed1bcca67bea8df281b75d9` |
| `era5...snowbird...1986-2024.nc` | `b8b665d22eb3389613dfc497a6786cedf96d6bc6625e0a707e4b0ec04b08adbc` |

| Product | Hours | Negative hours | Fraction | Minimum (`J m^-2`) | Sum of negatives (`J m^-2`) |
|---|---:|---:|---:|---:|---:|
| ERA5-Land point time series | 1,472,736 | 22,921 | 1.55635% | -4.0 | -21,590.2421875 |
| ERA5 point time series | 1,472,736 | 4 | 0.0002716% | -0.0014299653 | -0.0057198610 |

The ERA5-Land correction adds `0.0215902422 MJ m^-2` when summed across all
four complete multi-decade series (`5.99729 Wh m^-2`); ERA5 adds
`5.72e-9 MJ m^-2`. Maximum hourly correction is `4e-6 MJ m^-2`, equivalent to
`0.001111... W m^-2` over one hour. Every file has an exact one-hour UTC axis;
no missing or duplicate timestamp was found.

## Frozen Rule

At authenticated ERA5/ERA5-Land diagnostic ingress only:

1. require finite `ssrd` with source unit exactly `J m**-2`;
2. map values in the closed-open interval `[-4.0, 0.0) J m^-2` to exact zero;
3. retain values `>= 0` unchanged;
4. reject any value `< -4.0 J m^-2`, nonfinite value, unit mismatch, or a new
   product/file identity outside the validated inventory;
5. publish per-file corrected count, minimum, and energy delta before any
   comparison.

The threshold is the exact worst-case bound in this checksum-bound cohort, not
a general ERA5 tolerance. A future cohort exceeding it requires new source
characterization and review; threshold inflation is forbidden.

## Snow-Model Consequence

The correction affects only impossible near-zero source values and is
energetically immaterial at this cohort scale. It prevents
negative downward shortwave from tripping the diagnostic physical-domain guard
without altering daylight radiation, precipitation, calibrated multipliers,
temperature, longwave, observations, or production snow physics. It cannot be
credited as an improvement in snow timing or magnitude.
