#!/usr/bin/env python3
"""Directly validate acquired hourly and elevation content and emit a receipt."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

import numpy as np
import pyarrow.parquet as pq
import xarray as xr


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
MANIFEST = PACKAGE / "artifacts/acquisition-manifest.json"
OUTPUT = PACKAGE / "artifacts/validated-source-inventory.json"
DATA = REPO / "target/snow_hourly_era5_diagnostic"
G = 9.80665
VARIABLES = {
    "10m_u_component_of_wind": ("u10", "m s**-1"),
    "10m_v_component_of_wind": ("v10", "m s**-1"),
    "2m_dewpoint_temperature": ("d2m", "K"),
    "2m_temperature": ("t2m", "K"),
    "surface_pressure": ("sp", "Pa"),
    "surface_solar_radiation_downwards": ("ssrd", "J m**-2"),
    "surface_thermal_radiation_downwards": ("strd", "J m**-2"),
    "total_cloud_cover": ("tcc", "(0 - 1)"),
}
COORDINATE_TOLERANCE_DEGREES = 1.0e-4


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def normalized_longitude(value: float) -> float:
    return (value + 180.0) % 360.0 - 180.0


def nearest_grid_cell(value: float, spacing: float) -> float:
    return np.floor(value / spacing + 0.5) * spacing


def main() -> int:
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    dewpoint_tolerance_k = float(manifest["dewpoint_temperature_policy"]["accept_excess_through_k"])
    project_by_site = {item["site_id"]: item for item in manifest["project_elevation_sources"]}
    files = []
    elevations = []
    for dataset in manifest["datasets"]:
        expected = {VARIABLES[name][0]: VARIABLES[name][1] for name in dataset["variables"]}
        for site in manifest["sites"]:
            period = f'{site["start"][:4]}-{site["end"][:4]}'
            path = DATA / f'{dataset["dataset"]}__{site["site_id"]}__{period}.nc'
            expected_time = np.arange(
                np.datetime64(site["start"]),
                np.datetime64(site["end"]) + np.timedelta64(1, "D"),
                np.timedelta64(1, "h"),
            )
            with xr.open_dataset(path) as source:
                if source.sizes != {"valid_time": expected_time.size}:
                    raise RuntimeError(f"unexpected dimensions in {path}: {source.sizes}")
                if not np.array_equal(source.valid_time.values, expected_time):
                    raise RuntimeError(f"incomplete hourly UTC axis in {path}")
                if set(source.data_vars) != set(expected):
                    raise RuntimeError(f"variable mismatch in {path}")
                point_latitude = float(source.latitude.values.item())
                point_longitude = normalized_longitude(float(source.longitude.values.item()))
                if not np.isfinite([point_latitude, point_longitude]).all() or not -90 <= point_latitude <= 90:
                    raise RuntimeError(f"invalid point coordinate in {path}")
                spacing = 0.25 if dataset["dataset"] == "reanalysis-era5-single-levels-timeseries" else 0.1
                expected_latitude = nearest_grid_cell(float(site["latitude"]), spacing)
                expected_longitude = normalized_longitude(nearest_grid_cell(float(site["longitude"]), spacing))
                if not np.allclose(
                    [point_latitude, point_longitude],
                    [expected_latitude, expected_longitude],
                    rtol=0.0,
                    atol=COORDINATE_TOLERANCE_DEGREES,
                ):
                    raise RuntimeError(f"point series is not at the frozen nearest grid cell in {path}")
                for variable, unit in expected.items():
                    values = source[variable].values
                    if source[variable].attrs.get("units") != unit or not np.isfinite(values).all():
                        raise RuntimeError(f"unit/nonfinite failure {path}:{variable}")
                if not ((source.t2m.values > 0) & (source.d2m.values > 0)).all():
                    raise RuntimeError(f"Kelvin domain failure in {path}")
                dewpoint_excess = (source.d2m.values - source.t2m.values).astype(np.float64)
                if (dewpoint_excess > dewpoint_tolerance_k).any():
                    raise RuntimeError(f"dewpoint exceeds temperature tolerance in {path}")
                if not (source.sp.values > 0).all() or not (source.strd.values >= 0).all():
                    raise RuntimeError(f"pressure/longwave domain failure in {path}")
                if "tcc" in source and not ((source.tcc.values >= 0) & (source.tcc.values <= 1)).all():
                    raise RuntimeError(f"cloud fraction domain failure in {path}")
                shortwave = source.ssrd.values.astype(np.float64)
                if (shortwave < -4.0).any():
                    raise RuntimeError(f"shortwave below disposition bound in {path}")
                negative = shortwave < 0.0
                files.append({
                    "path": str(path),
                    "sha256": sha256(path),
                    "dataset": dataset["dataset"],
                    "site_id": site["site_id"],
                    "start": site["start"],
                    "end": site["end"],
                    "hours": int(shortwave.size),
                    "latitude": point_latitude,
                    "longitude": point_longitude,
                    "variables": sorted(source.data_vars),
                    "shortwave_negative_count": int(negative.sum()),
                    "shortwave_min_j_m2": float(shortwave.min()),
                    "shortwave_normalization_delta_j_m2": float(-shortwave[negative].sum()),
                    "dewpoint_above_temperature_count": int((dewpoint_excess > 0.0).sum()),
                    "dewpoint_max_excess_k": float(max(0.0, dewpoint_excess.max())),
                })
            elevation_path = DATA / f'grid-elevation__{dataset["dataset"]}__{site["site_id"]}.nc'
            with xr.open_dataset(elevation_path) as elevation_source:
                time_names = set(elevation_source.sizes) & {"time", "valid_time"}
                if len(time_names) != 1:
                    raise RuntimeError(f"elevation time coordinate failure in {elevation_path}")
                time_name = time_names.pop()
                if dict(elevation_source.sizes) != {time_name: 1, "latitude": 1, "longitude": 1}:
                    raise RuntimeError(f"unexpected elevation dimensions in {elevation_path}: {elevation_source.sizes}")
                if set(elevation_source.data_vars) != {"z"}:
                    raise RuntimeError(f"unexpected elevation variables in {elevation_path}")
                geopotential = float(elevation_source.z.values.item())
                grid_latitude = float(elevation_source.latitude.values.item())
                grid_longitude = normalized_longitude(float(elevation_source.longitude.values.item()))
                if elevation_source.z.attrs.get("units") != "m**2 s**-2":
                    raise RuntimeError(f"geopotential unit failure in {elevation_path}")
                if not np.isfinite([geopotential, grid_latitude, grid_longitude]).all() or geopotential <= 0:
                    raise RuntimeError(f"invalid geopotential or coordinate in {elevation_path}")
                if not -90 <= grid_latitude <= 90:
                    raise RuntimeError(f"invalid latitude in {elevation_path}")
                if not np.allclose(
                    [grid_latitude, grid_longitude],
                    [point_latitude, point_longitude],
                    rtol=0.0,
                    atol=COORDINATE_TOLERANCE_DEGREES,
                ):
                    raise RuntimeError(f"elevation/hourly grid-cell mismatch in {elevation_path}")
                grid_elevation_m = geopotential / G
            project = project_by_site[site["site_id"]]
            parquet = Path(project["run_root"]) / "watershed/hillslopes.parquet"
            dem = Path(project["run_root"]) / "dem/dem.tif"
            rows = [
                row for row in pq.read_table(parquet).to_pylist()
                if row["topaz_id"] == project["topaz_id"] and row["wepp_id"] == project["wepp_id"]
            ]
            if len(rows) != 1:
                raise RuntimeError(f"project hillslope identity failure for {site['site_id']}")
            row = rows[0]
            site_elevation_m = float(row["elevation"])
            fixture_elevation_m = float(site["fixture_elevation_m"])
            centroid_latitude = float(row["centroid_lat"])
            centroid_longitude = float(row["centroid_lon"])
            if (
                not np.isfinite(
                    [site_elevation_m, fixture_elevation_m, centroid_latitude, centroid_longitude]
                ).all()
                or site_elevation_m <= 0
                or fixture_elevation_m <= 0
            ):
                raise RuntimeError(f"invalid project elevation/centroid for {site['site_id']}")
            if not np.allclose(
                [centroid_latitude, centroid_longitude],
                [float(site["latitude"]), float(site["longitude"])],
                rtol=0.0,
                atol=1.0e-5,
            ):
                raise RuntimeError(f"fixture/project centroid mismatch for {site['site_id']}")
            if abs(site_elevation_m - fixture_elevation_m) > 0.1:
                raise RuntimeError(f"fixture/project elevation mismatch for {site['site_id']}")
            fixture_manifest = REPO / project["fixture_manifest"]
            fixture_text = fixture_manifest.read_text(encoding="utf-8")
            required_fixture_provenance = (
                f'Source run: `{project["run_root"]}`',
                f'TopazID {project["topaz_id"]} → `wepp_id` {project["wepp_id"]}',
            )
            if not all(value in fixture_text for value in required_fixture_provenance):
                raise RuntimeError(f"fixture manifest provenance mismatch for {site['site_id']}")
            delta_m = site_elevation_m - grid_elevation_m
            elevations.append({
                "dataset": dataset["dataset"],
                "site_id": site["site_id"],
                "grid_elevation_path": str(elevation_path),
                "grid_elevation_sha256": sha256(elevation_path),
                "grid_latitude": grid_latitude,
                "grid_longitude": grid_longitude,
                "grid_geopotential_m2_s2": float(grid_elevation_m * G),
                "grid_elevation_m": grid_elevation_m,
                "project_dem_path": str(dem),
                "project_dem_sha256": sha256(dem),
                "project_dem_role": "project identity evidence; hillslope elevation authority is the selected parquet row",
                "project_hillslopes_path": str(parquet),
                "project_hillslopes_sha256": sha256(parquet),
                "fixture_manifest_path": str(fixture_manifest),
                "fixture_manifest_sha256": sha256(fixture_manifest),
                "topaz_id": project["topaz_id"],
                "wepp_id": project["wepp_id"],
                "site_elevation_m": site_elevation_m,
                "site_centroid_latitude": centroid_latitude,
                "site_centroid_longitude": centroid_longitude,
                "site_minus_grid_elevation_m": delta_m,
                "fixed_lapse_temperature_offset_c": -6.5 * delta_m / 1000.0,
            })
    receipt = {
        "schema": "snow-hourly-era5-validated-source-inventory-v1",
        "status": "VALIDATED_COMPLETE",
        "manifest_sha256": sha256(MANIFEST),
        "sites": [site["site_id"] for site in manifest["sites"]],
        "datasets": [dataset["dataset"] for dataset in manifest["datasets"]],
        "complete_hourly_utc_inventory": True,
        "units_and_domains_validated": True,
        "shortwave_negative_policy_validated": True,
        "dewpoint_temperature_policy_validated": True,
        "files": files,
        "elevations": elevations,
    }
    OUTPUT.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("VALIDATED_COMPLETE", len(files), len(elevations))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
