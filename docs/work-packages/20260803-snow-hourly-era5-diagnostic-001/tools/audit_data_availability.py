#!/usr/bin/env python3
"""Audit whether ERA5 diagnostic execution has authenticated local evidence."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
MANIFEST = PACKAGE / "artifacts/acquisition-manifest.json"
OUTPUT = PACKAGE / "artifacts/data-availability-audit.json"
VALIDATED = PACKAGE / "artifacts/validated-source-inventory.json"
DATA_ROOTS = (
    REPO / "target/snow_hourly_era5_diagnostic",
    REPO / "data/era5",
    Path("/workdir/era5"),
)
READER_MODULES = ("cdsapi", "xarray", "netCDF4", "cfgrib")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    if OUTPUT.exists():
        raise RuntimeError("data availability audit already exists")
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    variable_union = {variable for dataset in manifest["datasets"] for variable in dataset["variables"]}
    if (
        len(manifest["sites"]) != 4
        or len(variable_union) != manifest["canonical_hourly_variable_union_count"]
        or manifest["ancillary_required"]["variable"] != "grid elevation or geopotential"
    ):
        raise RuntimeError("frozen acquisition inventory is incomplete")
    credential_file = Path.home() / ".cdsapirc"
    credential_locator_present = credential_file.is_file() or all(
        name in os.environ for name in ("CDSAPI_KEY", "CDSAPI_URL")
    )
    reader_capability = {
        name: importlib.util.find_spec(name) is not None for name in READER_MODULES
    }
    local_files = []
    for root in DATA_ROOTS:
        if not root.is_dir():
            continue
        for path in root.rglob("*"):
            if path.is_file() and path.suffix.lower() in {".grib", ".grib2", ".nc", ".nc4", ".csv"}:
                local_files.append(
                    {
                        "path": str(path),
                        "size_bytes": path.stat().st_size,
                        "sha256": sha256(path),
                    }
                )
    candidate_data_present = bool(local_files)
    validated_inventory_present = False
    if VALIDATED.is_file():
        validated = json.loads(VALIDATED.read_text(encoding="utf-8"))
        validated_inventory_present = bool(
            validated.get("status") == "VALIDATED_COMPLETE"
            and validated.get("manifest_sha256") == sha256(MANIFEST)
            and len(validated.get("files", [])) == 8
            and len(validated.get("elevations", [])) == 8
        )
    audit = {
        "schema": "snow-hourly-era5-data-availability-audit-v1",
        "manifest_sha256": sha256(MANIFEST),
        "credential_locator_present": credential_locator_present,
        "authenticated_credential_proven": False,
        "credential_values_inspected_or_recorded": False,
        "reader_capability": reader_capability,
        "searched_roots": [str(path) for path in DATA_ROOTS],
        "local_data_files": local_files,
        "candidate_data_present": candidate_data_present,
        "validated_source_inventory_present": validated_inventory_present,
        "acquisition_ready": False,
        "result_bearing_ready": False,
        "status": (
            "CANDIDATE_DATA_PRESENT_VALIDATION_REQUIRED"
            if candidate_data_present
            else "BLOCKED_EXTERNAL_DATA"
        ),
        "missing": [
            name
            for name, missing in (
                ("authenticated_cds_credential", True),
                ("retained_era5_or_era5_land_files", not local_files),
                ("separately_reviewed_complete_source_validator", not validated_inventory_present),
                (
                    "netcdf_reader",
                    not (reader_capability["xarray"] or reader_capability["netCDF4"]),
                ),
            )
            if missing
        ],
    }
    OUTPUT.write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(audit["status"], ",".join(audit["missing"]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
