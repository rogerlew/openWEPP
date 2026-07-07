# WEPPpy Implementation Evidence

Status: EXECUTED. Evidence mode: Static + Ran.

Changed WEPPpy files:

- `wepppy/wepp/management/managements.py`
- `wepppy/wepp/management/managements.pyi`
- `stubs/wepppy/wepp/management/managements.pyi`
- `wepppy/nodb/core/wepp_prep_service.py`
- `wepppy/nodb/mods/disturbed/route_coefficients.py`
- `wepppy/nodb/mods/disturbed/disturbed.py`
- `wepppy/nodb/mods/disturbed/__init__.py`
- `wepppy/nodb/mods/disturbed/data/extended_land_soil_lookup.csv`
- `wepppy/nodb/mods/disturbed/README.md`
- `wepppy/nodb/mods/disturbed/ENDUSER.md`
- `docs/adrs/ADR-0014-disturbed-openwepp-route-coefficients.md`
- `tests/test_managements_module.py`
- `tests/disturbed/test_route_coefficients.py`

Management API:

- `RoutingCoefficients` validates five finite values plus physical/coupling
  domains.
- `ow-lanuse-1` now treats `landuse=4` as native cropland and reuses cropland
  grammar. Legacy `landuse=4` remains Roads.
- Cropland plant records parse optional `routing_coefficients` only under
  native cropland. Legacy datvers fail closed if the marker appears.
- `Management.as_openwepp_native_cropland(...)` returns a copy with
  `ow-lanuse-1`, `landuse=4`, and route coefficients.
- `Management.apply_openwepp_native_cropland(...)` mutates an existing parsed
  management only when explicitly called.

Disturbed producer:

- `route_coefficients.py` owns the active class matrix, provenance values,
  enrichment, and validation.
- `build_extended_land_soil_lookup()` enriches every generated extended row.
- `Disturbed.build_openwepp_native_management(...)` validates a lookup row and
  returns a native management through the management API.
- `Disturbed.write_openwepp_native_management(...)` writes a native file for
  explicit producer calls.
- `wepp_prep_service.py` honors the opt-in
  `disturbed.openwepp_native_managements_enabled` flag and converts the
  per-hillslope `pN.man` write path to native only when enabled.
- Static extended CSV was updated for visibility and regression coverage.

Legacy isolation:

- Default string serialization of legacy managements is unchanged unless native
  conversion is explicitly called.
- Focused test proves the source management text remains equal after building a
  native copy.

Native output evidence:

- Generated smoke file:
  `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/generated-native-smoke/p1.man`
- openWEPP fixture:
  `tests/fixtures/disturbed_native_route_coefficients/p1.man`

Ran:

```text
cd /home/workdir/wepppy
wctl run-pytest tests/test_managements_module.py -q
15 passed, 2 warnings

cd /home/workdir/wepppy
wctl run-pytest tests/disturbed/test_route_coefficients.py -q
6 passed, 2 warnings

cd /home/workdir/wepppy
wctl run-pytest tests/disturbed/test_disturbed_matrix.py tests/disturbed/test_route_coefficients.py tests/test_2_validate_managements.py tests/test_managements_module.py -q
112 passed, 2 warnings
```
