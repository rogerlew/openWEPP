#!/usr/bin/env python3
"""Retrieve gridded geopotential for each acquired point-series grid cell."""

from __future__ import annotations

import json
import os
from pathlib import Path

import cdsapi
import xarray as xr


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
MANIFEST = PACKAGE / "artifacts/acquisition-manifest.json"
DATA = REPO / "target/snow_hourly_era5_diagnostic"
GRIDDED = {
    "reanalysis-era5-single-levels-timeseries": "reanalysis-era5-single-levels",
    "reanalysis-era5-land-timeseries": "reanalysis-era5-land",
}


def point_path(dataset: str, site: dict[str, object]) -> Path:
    period = f'{str(site["start"])[:4]}-{str(site["end"])[:4]}'
    return DATA / f'{dataset}__{site["site_id"]}__{period}.nc'


def valid_geopotential(path: Path, latitude: float, longitude: float) -> bool:
    try:
        with xr.open_dataset(path) as source:
            time_name = "valid_time" if "valid_time" in source.sizes else "time"
            source_longitude = float(source.longitude.values.item())
            source_longitude = (source_longitude + 180.0) % 360.0 - 180.0
            return bool(
                set(source.sizes) >= {time_name, "latitude", "longitude"}
                and source.sizes[time_name] == 1
                and source.sizes["latitude"] == 1
                and source.sizes["longitude"] == 1
                and set(source.data_vars) == {"z"}
                and source["z"].attrs.get("units") == "m**2 s**-2"
                and abs(float(source.latitude.values.item()) - latitude) < 1e-4
                and abs(source_longitude - longitude) < 1e-4
                and float(source.z.values.item()) > 0.0
            )
    except (OSError, ValueError):
        return False


def main() -> int:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    client = cdsapi.Client()
    for dataset in manifest["datasets"]:
        point_dataset = dataset["dataset"]
        gridded_dataset = GRIDDED[point_dataset]
        for site in manifest["sites"]:
            with xr.open_dataset(point_path(point_dataset, site)) as point:
                latitude = float(point.latitude.values.item())
                longitude = float(point.longitude.values.item())
            target = DATA / f'grid-elevation__{point_dataset}__{site["site_id"]}.nc'
            if target.exists():
                if valid_geopotential(target, latitude, longitude):
                    continue
                raise RuntimeError(f"refusing invalid existing elevation file {target}")
            partial = target.with_suffix(".partial")
            if partial.exists():
                if valid_geopotential(partial, latitude, longitude):
                    os.replace(partial, target)
                    continue
                raise RuntimeError(f"refusing invalid existing partial file {partial}")
            request: dict[str, object] = {
                "variable": ["geopotential"],
                "year": ["2024"],
                "month": ["01"],
                "day": ["01"],
                "time": ["00:00"],
                "data_format": "netcdf",
                "download_format": "unarchived",
                "area": [latitude, longitude, latitude, longitude],
            }
            if gridded_dataset == "reanalysis-era5-single-levels":
                request["product_type"] = ["reanalysis"]
            client.retrieve(gridded_dataset, request, str(partial))
            if not valid_geopotential(partial, latitude, longitude):
                raise RuntimeError(f"retrieved elevation failed validation {partial}")
            os.replace(partial, target)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
