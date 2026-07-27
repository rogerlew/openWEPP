# VPD and Photoperiod Method

Evidence class: `Static equation binding + Ran deterministic derivation`

Daymet supplies daily `Tmax`, `Tmin` in degrees Celsius, actual vapor pressure
`VP` in pascals, and day length in seconds. Derived VPD uses the native
runner's saturation and daily-mean saturation algebra:

`es(T) = 0.6108 * exp(17.27*T/(T+237.3))` kPa

`VPD = 1000 * (0.5*(es(Tmax)+es(Tmin)) - VP/1000)` Pa.

The native climate path obtains actual vapor pressure as `es(dewpoint)`,
whereas this analysis uses Daymet's supplied daily-average actual VP directly.
Thus the saturation/deficit algebra is native-equivalent, but the actual-VP
source is not identical.

The derivation rejects negative or non-finite VPD; it does not clamp. All
118,260 plot-days passed.

Native photoperiod mirrors `openwepp-plant-phenology` FAO-56 geometry:
declination `0.409*sin(2*pi*d/365 - 1.39)`, bounded sunset cosine, and
`24*omega/pi` hours. Daymet day length is retained separately for comparison.

Daymet uses 365 records in every year: leap day is present and December 31 is
discarded in leap years. Spring P3 joins use Daymet `yday`, which matches
Gregorian ordinal days through the spring observation period.
