# Management Fixture Provenance

Primary seed corpus:
`/home/workdir/wepppy/wepppy/wepp/management/data`

Curated canonical fixtures:
- `canonical_cropland_nonzero_98_4.man`
  - source: `Agriculture/corn-no till.man`
  - transform: none
- `canonical_cropland_nonzero_95_7.man`
  - source: `Agriculture/corn-no till.man`
  - transform: `datver` header set from `98.4` to `95.7`
- `canonical_cropland_nonzero_2016_3.man`
  - source: `Agriculture/corn-no till.man`
  - transform: `datver` header set from `98.4` to `2016.3`
- `canonical_cropland_nonzero_2017_1.man`
  - source: `Agriculture/corn-no till.man`
  - transform: `datver` header set from `98.4` to `2017.1`
- `canonical_rotation_nonzero_98_4.man`
  - source: `Palouse/ww_barley_fallow_Int_Precip_ct.man`
  - transform: none
- `compat_trailing_tokens_ok.man`
  - source: `canonical_cropland_nonzero_95_7.man`
  - transform: `datver` line changed from `95.7` to `95.7 format`

Negative fixtures derived from canonical sources:
- `malformed_dangling_yearly_ref.man`
  - source: `canonical_cropland_nonzero_98_4.man`
  - transform: first management `manindx` changed from `1` to `99`
- `malformed_invalid_surface_date.man`
  - source: `canonical_cropland_nonzero_98_4.man`
  - transform: first surface `mdate` changed from `130` to `367`
- `malformed_rangeland_unsupported.man`
  - source: `canonical_cropland_nonzero_98_4.man`
  - transform: yearly `landuse` changed from `1` to `2`
- `malformed_dangling_op_ref.man`
  - source: `canonical_cropland_nonzero_98_4.man`
  - transform: first surface operation reference `op` changed from `1` to `9`
- `malformed_total_year_mismatch.man`
  - source: `canonical_cropland_nonzero_98_4.man`
  - transform: info-section total years changed from `1` to `2`
