# Official Source Map

Status: frozen before result-bearing access

| Authority | Binding use |
|---|---|
| [ERA5 hourly single-level time series](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels-timeseries?tab=overview) | ERA5 hourly point time series, 0.25-degree source grid, NetCDF/CSV access, and dataset request identity. |
| [ERA5-Land hourly time series](https://cds.climate.copernicus.eu/datasets/reanalysis-era5-land-timeseries?tab=overview) | 0.1-degree hourly land time series, NetCDF/CSV availability, variable names and units. |
| [ERA5-Land data documentation](https://confluence.ecmwf.int/pages/viewpage.action?pageId=402639006) | Distinguishes the gridded-product accumulation convention from the selected time-series product. |
| [ERA5-Land parameter documentation](https://confluence.ecmwf.int/pages/viewpage.action?pageId=222471664) | Instantaneous versus accumulated variables, geopotential units, and ERA5-Land forcing-field status. |

The live CDS time-series forms label their radiation variables de-accumulated.
Terminal comparison converts hourly `J m^-2` once to `MJ m^-2 h^-1` and does
not difference. The 00 UTC cumulative convention applies to the distinct
gridded ERA5-Land product and must not be transferred to these point series.
The live ERA5-Land catalogue form offers both shortwave and downward longwave
as de-accumulated time-series fields.
Temperature and dewpoint convert from kelvin by subtracting `273.15`; wind speed
is reconstructed from the 10 m vector. Grid elevation is a separately validated
ancillary; if supplied as geopotential, elevation is `z/9.80665`.
