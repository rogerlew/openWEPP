# Daymet Calibration Forcing

Source-native Daymet Single Pixel Extraction responses for the nine Hubbard
Brook phenology plots admitted by EDI package `knb-lter-hbr.51.16`.

Product citation:

Thornton, M.M., R. Shrestha, Y. Wei, P.E. Thornton, S-C. Kao, and B.E. Wilson.
2022. Daymet: Daily Surface Weather Data on a 1-km Grid for North America,
Version 4 R1. ORNL DAAC. <https://doi.org/10.3334/ORNLDAAC/2129>

Requests select `tmax`, `tmin`, `vp`, and `dayl` for 1989–2024. Daymet returns
daily maximum/minimum 2-m air temperature in degrees Celsius, daily average
water vapor pressure in pascals, day length in seconds, returned grid
coordinates, tile, and grid elevation.

These objects define forcing support and gridded elevation context. They are
not in-situ plot microclimate measurements and do not replace the protected
openWEPP climate fixture.

