#!/usr/bin/env python3
"""Retrieve the frozen site/year ERA5 time-series requests via CDS."""

from __future__ import annotations

import json
import os
from pathlib import Path
from tempfile import TemporaryDirectory
from zipfile import BadZipFile, ZipFile, is_zipfile

import cdsapi
import xarray as xr


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
MANIFEST = PACKAGE / "artifacts/acquisition-manifest.json"
OUTPUT = REPO / "target/snow_hourly_era5_diagnostic"


def is_netcdf(path: Path) -> bool:
    with path.open("rb") as stream:
        magic = stream.read(8)
    return magic.startswith(b"CDF") or magic == b"\x89HDF\r\n\x1a\n"


def normalize_download(download: Path, target: Path) -> None:
    if is_netcdf(download):
        os.replace(download, target)
        return
    if not is_zipfile(download):
        raise RuntimeError(f"CDS response is neither NetCDF nor ZIP: {download}")
    try:
        with ZipFile(download) as archive:
            members = [item for item in archive.infolist() if not item.is_dir()]
            partial = target.with_suffix(".partial")
            if not members or any(
                not item.filename.endswith(".nc") or Path(item.filename).name != item.filename
                for item in members
            ):
                raise RuntimeError(f"expected safe NetCDF members in {download}")
            if len(members) == 1:
                with archive.open(members[0]) as source, partial.open("wb") as destination:
                    while chunk := source.read(1024 * 1024):
                        destination.write(chunk)
            else:
                with TemporaryDirectory(dir=OUTPUT) as temporary:
                    paths = []
                    for member in members:
                        path = Path(temporary) / member.filename
                        with archive.open(member) as source, path.open("wb") as destination:
                            while chunk := source.read(1024 * 1024):
                                destination.write(chunk)
                        if not is_netcdf(path):
                            raise RuntimeError(f"archive member is not NetCDF: {member.filename}")
                        paths.append(path)
                    datasets = [xr.open_dataset(path) for path in paths]
                    try:
                        merged = xr.merge(datasets, compat="no_conflicts", join="exact")
                        merged.to_netcdf(partial)
                    finally:
                        for dataset in datasets:
                            dataset.close()
            if not is_netcdf(partial):
                raise RuntimeError(f"extracted member is not NetCDF: {download}")
            os.replace(partial, target)
    except BadZipFile as error:
        raise RuntimeError(f"invalid ZIP response: {download}") from error
    if download != target:
        download.unlink()


def main() -> int:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    OUTPUT.mkdir(parents=True, exist_ok=True)
    client = cdsapi.Client()
    for dataset in manifest["datasets"]:
        for site in manifest["sites"]:
            period = f'{site["start"][:4]}-{site["end"][:4]}'
            target = OUTPUT / f'{dataset["dataset"]}__{site["site_id"]}__{period}.nc'
            if target.exists():
                if is_netcdf(target):
                    continue
                if is_zipfile(target):
                    normalize_download(target, target)
                    continue
                raise RuntimeError(f"refusing unrecognized existing file {target}")
            download = target.with_suffix(".download")
            if download.exists():
                if is_zipfile(download) or is_netcdf(download):
                    normalize_download(download, target)
                    continue
                raise RuntimeError(f"refusing unrecognized partial download {download}")
            request = {
                "variable": dataset["variables"],
                "location": {"longitude": site["longitude"], "latitude": site["latitude"]},
                "date": [f'{site["start"]}/{site["end"]}'],
                "data_format": "netcdf",
            }
            client.retrieve(dataset["dataset"], request, str(download))
            normalize_download(download, target)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
