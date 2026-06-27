#!/usr/bin/env python3
"""Install canopy-stratified Harvard and Marcell snow observations.

The normalized fixture products are long tables under
``tests/fixtures/cancov_forest/observations``. Raw downloads are cached under
``target/cancov_stratified_observed/raw`` and referenced from provenance files.
"""

from __future__ import annotations

import csv
import datetime as dt
import hashlib
import json
import math
import shutil
import urllib.request
import zipfile
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
RAW_ROOT = REPO_ROOT / "target/cancov_stratified_observed/raw"
OUT_ROOT = REPO_ROOT / "tests/fixtures/cancov_forest/observations"
ACCESS_DATE = "2026-06-26"

HARVARD_DEPTH_URL = (
    "https://harvardforest1.fas.harvard.edu/data/p23/hf237/"
    "hf237-01-snow-depth.csv"
)
HARVARD_DENSITY_URL = (
    "https://harvardforest1.fas.harvard.edu/data/p23/hf237/"
    "hf237-02-snow-density.csv"
)
MARCELL_ZIP_URL = (
    "https://www.fs.usda.gov/rds/archive/products/RDS-2021-0016/"
    "RDS-2021-0016.zip"
)

EXPECTED_HASHES = {
    "hf237-01-snow-depth.csv": (
        "2c80d505952350879df3993d61262608ee0f5a695f5a9409c1213edd1d0271ff"
    ),
    "hf237-02-snow-density.csv": (
        "b57eca8cad0d2f8e8e909a825dbba22b0f1feab7ad073ba01e2f9585853232e9"
    ),
    "RDS-2021-0016.zip": (
        "a507b825a705caeb32c928228b07fb431ea8ad3093d927f22adacb6eb03b76d3"
    ),
}

HARVARD_STRATA = {
    "shaler": {
        "observed_stratum": "open",
        "binding_status": "bound",
        "model_fixture": "harvard_open_ma",
        "quality": "source_daily_depth;source_interpolated_density",
    },
    "lph": {
        "observed_stratum": "hardwood",
        "binding_status": "bound",
        "model_fixture": "harvard_deciduous_ma",
        "quality": "source_daily_depth;source_interpolated_density",
    },
    "hemlock": {
        "observed_stratum": "hemlock",
        "binding_status": "unbound_no_pure_conifer_fixture",
        "model_fixture": "",
        "quality": "source_daily_depth;source_interpolated_density;unbound_hemlock",
    },
}

MARCELL_STRATA = {
    "S11": ("deciduous", "marcell_deciduous_mn"),
    "S21": ("deciduous", "marcell_deciduous_mn"),
    "S22": ("deciduous", "marcell_deciduous_mn"),
    "S51": ("deciduous", "marcell_deciduous_mn"),
    "S23": ("conifer", "marcell_conifer_mn"),
    "S54": ("conifer", "marcell_conifer_mn"),
    "PINE": ("conifer", "marcell_conifer_mn"),
    "S5WS": ("open", "marcell_open_mn"),
    "S52": ("open", "marcell_open_mn"),
    "JUNC": ("open", "marcell_open_mn"),
}


@dataclass(frozen=True)
class FileSummary:
    path: str
    bytes: int
    sha256: str


def main() -> int:
    raw_files = acquire_raw_files()
    output_files = normalize_observations(raw_files)
    write_readme()
    manifest = build_manifest(raw_files, output_files)
    write_json(OUT_ROOT / "manifest.json", manifest)
    return 0


def acquire_raw_files() -> dict[str, Path]:
    harvard_raw = RAW_ROOT / "harvard"
    marcell_raw = RAW_ROOT / "marcell"
    harvard_raw.mkdir(parents=True, exist_ok=True)
    marcell_raw.mkdir(parents=True, exist_ok=True)

    depth = download(
        HARVARD_DEPTH_URL,
        harvard_raw / "hf237-01-snow-depth.csv",
        EXPECTED_HASHES["hf237-01-snow-depth.csv"],
    )
    density = download(
        HARVARD_DENSITY_URL,
        harvard_raw / "hf237-02-snow-density.csv",
        EXPECTED_HASHES["hf237-02-snow-density.csv"],
    )
    marcell_zip = download(
        MARCELL_ZIP_URL,
        marcell_raw / "RDS-2021-0016.zip",
        EXPECTED_HASHES["RDS-2021-0016.zip"],
    )
    extracted = marcell_raw / "extracted"
    if extracted.exists():
        shutil.rmtree(extracted)
    extracted.mkdir(parents=True)
    with zipfile.ZipFile(marcell_zip) as archive:
        archive.extractall(extracted)
    return {
        "harvard_depth": depth,
        "harvard_density": density,
        "marcell_zip": marcell_zip,
        "marcell_snow_swe": extracted / "Data/MEF_snowSWE_biweekly.csv",
        "marcell_coordinates": extracted / "Supplements/MEF_snowcourse_coordinates.csv",
        "marcell_metadata_html": extracted / "_metadata_RDS-2021-0016.html",
        "marcell_file_index_html": extracted / "_fileindex_RDS-2021-0016.html",
    }


def download(url: str, path: Path, expected_sha256: str) -> Path:
    if not path.is_file():
        with urllib.request.urlopen(url, timeout=60) as response:
            path.write_bytes(response.read())
    actual = sha256_file(path)
    if actual != expected_sha256:
        raise RuntimeError(
            f"unexpected hash for {path}: observed {actual}, expected {expected_sha256}"
        )
    return path


def normalize_observations(raw_files: dict[str, Path]) -> list[FileSummary]:
    sites_dir = OUT_ROOT / "sites"
    profiles_dir = OUT_ROOT / "profiles"
    provenance_dir = OUT_ROOT / "provenance"
    for directory in (sites_dir, profiles_dir, provenance_dir):
        directory.mkdir(parents=True, exist_ok=True)

    products = [
        normalize_harvard_depth(raw_files["harvard_depth"], sites_dir),
        normalize_harvard_profiles(raw_files["harvard_density"], profiles_dir),
        normalize_marcell_points(raw_files["marcell_snow_swe"], sites_dir),
        normalize_marcell_snowcourse_means(raw_files["marcell_snow_swe"], sites_dir),
        normalize_marcell_stratum_means(raw_files["marcell_snow_swe"], sites_dir),
    ]
    provenance_files = [
        write_harvard_provenance(raw_files, provenance_dir),
        write_marcell_provenance(raw_files, provenance_dir),
    ]
    return [summarize_file(path) for path in products + provenance_files]


def normalize_harvard_depth(path: Path, sites_dir: Path) -> Path:
    rows: list[dict[str, str]] = []
    for raw in read_csv(path):
        date = dt.date.fromisoformat(raw["date"])
        for prefix, meta in HARVARD_STRATA.items():
            depth_m = cm_to_m(parse_float(raw[f"{prefix}.depth"]))
            swe_mm = cm_to_mm(parse_float(raw[f"{prefix}.swe"]))
            density = parse_float(raw[f"{prefix}.density"])
            rows.append(
                {
                    "source_id": "harvard_hf237",
                    "observation_site": "harvard_forest",
                    "observed_stratum": meta["observed_stratum"],
                    "binding_status": meta["binding_status"],
                    "model_fixture": meta["model_fixture"],
                    "date": date.isoformat(),
                    "water_year": str(water_year(date)),
                    "observed_snow_depth_m": format_optional(depth_m, 6),
                    "observed_swe_mm": format_optional(swe_mm, 3),
                    "observed_density_kg_m3": format_optional(density, 3),
                    "sample_count": "1",
                    "source_record_id": f"HF237-01:{prefix}:{date.isoformat()}",
                    "quality_flag": meta["quality"],
                }
            )
    out = sites_dir / "harvard_hf237_strata.csv"
    write_csv(out, rows)
    return out


def normalize_harvard_profiles(path: Path, profiles_dir: Path) -> Path:
    rows: list[dict[str, str]] = []
    for raw in read_csv(path):
        date = dt.date.fromisoformat(raw["date"])
        for prefix, meta in HARVARD_STRATA.items():
            for depth_cm in ("07", "14", "21", "28", "35", "42", "49", "56", "63", "70"):
                density = parse_float(raw[f"{prefix}.{depth_cm}"])
                if density is None:
                    continue
                rows.append(
                    {
                        "source_id": "harvard_hf237",
                        "observation_site": "harvard_forest",
                        "observed_stratum": meta["observed_stratum"],
                        "binding_status": meta["binding_status"],
                        "model_fixture": meta["model_fixture"],
                        "date": date.isoformat(),
                        "water_year": str(water_year(date)),
                        "profile_depth_m": format_optional(cm_to_m(float(depth_cm)), 6),
                        "observed_density_kg_m3": format_optional(density, 3),
                        "source_record_id": (
                            f"HF237-02:{prefix}.{depth_cm}:{date.isoformat()}"
                        ),
                        "quality_flag": "source_profile_density"
                        + (";unbound_hemlock" if prefix == "hemlock" else ""),
                    }
                )
    out = profiles_dir / "harvard_hf237_density_profiles.csv"
    write_csv(out, rows)
    return out


def normalize_marcell_points(path: Path, sites_dir: Path) -> Path:
    rows: list[dict[str, str]] = []
    for raw in read_csv(path):
        date = dt.date.fromisoformat(raw["Date"])
        snowcourse = raw["SnowcourseID"]
        stratum, model_fixture, binding_status = marcell_binding(snowcourse)
        depth_m = cm_to_m(parse_float(raw["SnowDEPTH"]))
        swe_mm = cm_to_mm(parse_float(raw["SWE"]))
        rows.append(
            {
                "source_id": "marcell_rds_2021_0016",
                "observation_site": "marcell_experimental_forest",
                "observed_stratum": stratum,
                "binding_status": binding_status,
                "model_fixture": model_fixture,
                "watershed": raw["Watershed"],
                "snowcourse_id": snowcourse,
                "point": raw["Point"],
                "date": date.isoformat(),
                "water_year": str(water_year(date)),
                "observed_snow_depth_m": format_optional(depth_m, 6),
                "observed_swe_mm": format_optional(swe_mm, 3),
                "observed_density_kg_m3": format_optional(density_from(depth_m, swe_mm), 3),
                "source_record_id": (
                    "RDS-2021-0016:MEF_snowSWE_biweekly:"
                    f"{snowcourse}:{date.isoformat()}:point{raw['Point']}"
                ),
                "quality_flag": marcell_quality(snowcourse, depth_m, swe_mm),
            }
        )
    out = sites_dir / "marcell_rds_2021_0016_points.csv"
    write_csv(out, rows)
    return out


def normalize_marcell_snowcourse_means(path: Path, sites_dir: Path) -> Path:
    groups: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for raw in read_csv(path):
        groups[(raw["Date"], raw["SnowcourseID"])].append(raw)

    rows: list[dict[str, str]] = []
    for (date_text, snowcourse), group_rows in sorted(groups.items()):
        date = dt.date.fromisoformat(date_text)
        stratum, model_fixture, binding_status = marcell_binding(snowcourse)
        depth_values = [cm_to_m(parse_float(row["SnowDEPTH"])) for row in group_rows]
        swe_values = [cm_to_mm(parse_float(row["SWE"])) for row in group_rows]
        depth_mean = mean([value for value in depth_values if value is not None])
        swe_mean = mean([value for value in swe_values if value is not None])
        rows.append(
            {
                "source_id": "marcell_rds_2021_0016",
                "observation_site": "marcell_experimental_forest",
                "observed_stratum": stratum,
                "binding_status": binding_status,
                "model_fixture": model_fixture,
                "watershed": group_rows[0]["Watershed"],
                "snowcourse_id": snowcourse,
                "date": date.isoformat(),
                "water_year": str(water_year(date)),
                "observed_snow_depth_m": format_optional(depth_mean, 6),
                "observed_swe_mm": format_optional(swe_mean, 3),
                "observed_density_kg_m3": format_optional(
                    density_from(depth_mean, swe_mean), 3
                ),
                "point_count": str(len(group_rows)),
                "source_record_id": (
                    "RDS-2021-0016:MEF_snowSWE_biweekly:"
                    f"{snowcourse}:{date.isoformat()}:snowcourse_mean"
                ),
                "quality_flag": marcell_quality(snowcourse, depth_mean, swe_mean)
                + ";snowcourse_mean",
            }
        )
    out = sites_dir / "marcell_rds_2021_0016_snowcourse_means.csv"
    write_csv(out, rows)
    return out


def normalize_marcell_stratum_means(path: Path, sites_dir: Path) -> Path:
    groups: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for raw in read_csv(path):
        stratum, _, binding_status = marcell_binding(raw["SnowcourseID"])
        if binding_status != "bound":
            continue
        groups[(raw["Date"], stratum)].append(raw)

    rows: list[dict[str, str]] = []
    for (date_text, stratum), group_rows in sorted(groups.items()):
        date = dt.date.fromisoformat(date_text)
        model_fixture = {
            "conifer": "marcell_conifer_mn",
            "deciduous": "marcell_deciduous_mn",
            "open": "marcell_open_mn",
        }[stratum]
        depth_values = [cm_to_m(parse_float(row["SnowDEPTH"])) for row in group_rows]
        swe_values = [cm_to_mm(parse_float(row["SWE"])) for row in group_rows]
        depth_mean = mean([value for value in depth_values if value is not None])
        swe_mean = mean([value for value in swe_values if value is not None])
        snowcourse_ids = sorted({row["SnowcourseID"] for row in group_rows})
        rows.append(
            {
                "source_id": "marcell_rds_2021_0016",
                "observation_site": "marcell_experimental_forest",
                "observed_stratum": stratum,
                "binding_status": "bound",
                "model_fixture": model_fixture,
                "date": date.isoformat(),
                "water_year": str(water_year(date)),
                "observed_snow_depth_m": format_optional(depth_mean, 6),
                "observed_swe_mm": format_optional(swe_mean, 3),
                "observed_density_kg_m3": format_optional(
                    density_from(depth_mean, swe_mean), 3
                ),
                "snowcourse_count": str(len(snowcourse_ids)),
                "point_count": str(len(group_rows)),
                "snowcourse_ids": ";".join(snowcourse_ids),
                "source_record_id": (
                    "RDS-2021-0016:MEF_snowSWE_biweekly:"
                    f"{stratum}:{date.isoformat()}:stratum_mean"
                ),
                "quality_flag": "stratum_mean;density_derived_from_mean_swe_depth",
            }
        )
    out = sites_dir / "marcell_rds_2021_0016_stratum_means.csv"
    write_csv(out, rows)
    return out


def write_harvard_provenance(raw_files: dict[str, Path], provenance_dir: Path) -> Path:
    path = provenance_dir / "harvard_hf237.json"
    data = {
        "source_id": "harvard_hf237",
        "title": "Snowpack in Hemlock, Hardwood, and Open Sites at Harvard Forest 2008-2014",
        "citation": (
            "Hellstrom R. 2023. Snowpack in Hemlock, Hardwood, and Open Sites at "
            "Harvard Forest 2008-2014. Harvard Forest Data Archive: HF237 (v.4). "
            "Environmental Data Initiative. https://doi.org/10.6073/pasta/"
            "be69b1f46b57354a25d85a437c0679c8."
        ),
        "license_or_terms": "Creative Commons Zero v1.0 Universal (CC0-1.0).",
        "access_date": ACCESS_DATE,
        "source_url": "https://harvardforest1.fas.harvard.edu/exist/apps/datasets/showData.html?id=HF237",
        "raw_files": [
            summarize_file(raw_files["harvard_depth"]).__dict__,
            summarize_file(raw_files["harvard_density"]).__dict__,
        ],
        "parser_version": "cancov-stratified-observations-v1",
        "parser_assumptions": [
            "HF237 shaler columns are normalized as the open stratum.",
            "HF237 lph columns are normalized as the hardwood stratum.",
            "HF237 hemlock columns are installed but marked unbound because no pure Harvard hemlock model fixture exists.",
            "Depth and SWE in HF237-01 are centimeters; density is kg m^-3.",
            "HF237-01 density is source-provided daily interpolation.",
        ],
    }
    write_json(path, data)
    return path


def write_marcell_provenance(raw_files: dict[str, Path], provenance_dir: Path) -> Path:
    path = provenance_dir / "marcell_rds_2021_0016.json"
    data = {
        "source_id": "marcell_rds_2021_0016",
        "title": "Marcell Experimental Forest biweekly snow depth, frost depth, and snow water equivalent, 1962 - ongoing",
        "citation": (
            "Sebestyen, Stephen D.; Verry, Elon S.; Elling, Arthur E.; "
            "Kyllander, Richard L.; Roman, Daniel T.; Burdick, Jacob M.; "
            "Lany, Nina K.; Kolka, Randall K. 2021. Marcell Experimental Forest "
            "biweekly snow depth, frost depth, and snow water equivalent, 1962 - "
            "ongoing. Fort Collins, CO: Forest Service Research Data Archive. "
            "https://doi.org/10.2737/RDS-2021-0016"
        ),
        "license_or_terms": (
            "Collected using U.S. Government funding and usable without additional "
            "permissions or fees; cite the data publication."
        ),
        "access_date": ACCESS_DATE,
        "source_url": "https://www.fs.usda.gov/rds/archive/Catalog/RDS-2021-0016",
        "download_url": MARCELL_ZIP_URL,
        "raw_files": [
            summarize_file(raw_files["marcell_zip"]).__dict__,
            summarize_file(raw_files["marcell_snow_swe"]).__dict__,
            summarize_file(raw_files["marcell_coordinates"]).__dict__,
            summarize_file(raw_files["marcell_metadata_html"]).__dict__,
            summarize_file(raw_files["marcell_file_index_html"]).__dict__,
        ],
        "parser_version": "cancov-stratified-observations-v1",
        "parser_assumptions": [
            "SnowDEPTH and SWE are centimeters and are normalized to meters and millimeters.",
            "Density is derived as SWE(mm) / snow_depth(m) where depth is positive.",
            "S11/S21/S22/S51 map to deciduous; S23/S54/PINE map to conifer; S5WS/S52/JUNC map to open.",
            "S53 appears in the snow/SWE table but is not described in the RDS metadata or coordinate supplement; it is retained in point and snowcourse tables as unknown and excluded from stratum means.",
        ],
    }
    write_json(path, data)
    return path


def build_manifest(raw_files: dict[str, Path], output_files: list[FileSummary]) -> dict[str, Any]:
    return {
        "schema": "cancov-stratified-observations-manifest-v1",
        "access_date": ACCESS_DATE,
        "contract": "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-063",
        "normal_depth_units": "m",
        "normal_swe_units": "mm water equivalent",
        "normal_density_units": "kg m^-3",
        "parser_version": "cancov-stratified-observations-v1",
        "sites": [
            site_summary("harvard_hf237", OUT_ROOT / "sites/harvard_hf237_strata.csv"),
            site_summary(
                "marcell_rds_2021_0016_points",
                OUT_ROOT / "sites/marcell_rds_2021_0016_points.csv",
            ),
            site_summary(
                "marcell_rds_2021_0016_snowcourse_means",
                OUT_ROOT / "sites/marcell_rds_2021_0016_snowcourse_means.csv",
            ),
            site_summary(
                "marcell_rds_2021_0016_stratum_means",
                OUT_ROOT / "sites/marcell_rds_2021_0016_stratum_means.csv",
            ),
        ],
        "profiles": [
            site_summary(
                "harvard_hf237_density_profiles",
                OUT_ROOT / "profiles/harvard_hf237_density_profiles.csv",
            )
        ],
        "sources": [
            {
                "source_id": "harvard_hf237",
                "source_url": "https://harvardforest1.fas.harvard.edu/exist/apps/datasets/showData.html?id=HF237",
                "normalized_files": [
                    summarize_file(OUT_ROOT / "sites/harvard_hf237_strata.csv").__dict__,
                    summarize_file(
                        OUT_ROOT / "profiles/harvard_hf237_density_profiles.csv"
                    ).__dict__,
                ],
                "provenance_file": "provenance/harvard_hf237.json",
            },
            {
                "source_id": "marcell_rds_2021_0016",
                "source_url": "https://www.fs.usda.gov/rds/archive/Catalog/RDS-2021-0016",
                "download_url": MARCELL_ZIP_URL,
                "normalized_files": [
                    summarize_file(
                        OUT_ROOT / "sites/marcell_rds_2021_0016_points.csv"
                    ).__dict__,
                    summarize_file(
                        OUT_ROOT / "sites/marcell_rds_2021_0016_snowcourse_means.csv"
                    ).__dict__,
                    summarize_file(
                        OUT_ROOT / "sites/marcell_rds_2021_0016_stratum_means.csv"
                    ).__dict__,
                ],
                "provenance_file": "provenance/marcell_rds_2021_0016.json",
            },
        ],
        "output_files": [summary.__dict__ for summary in sorted(output_files, key=lambda s: s.path)],
        "raw_cache_root": str(RAW_ROOT),
        "raw_inputs": {key: str(path) for key, path in sorted(raw_files.items())},
    }


def site_summary(site_id: str, path: Path) -> dict[str, Any]:
    rows = read_csv(path)
    dates = sorted({row["date"] for row in rows if "date" in row and row["date"]})
    strata = sorted({row["observed_stratum"] for row in rows if row.get("observed_stratum")})
    return {
        "site_id": site_id,
        "observation_file": str(path.relative_to(OUT_ROOT)),
        "row_count": len(rows),
        "start_date": dates[0] if dates else None,
        "end_date": dates[-1] if dates else None,
        "observed_strata": strata,
        "normalized_file_bytes": path.stat().st_size,
        "normalized_file_sha256": sha256_file(path),
    }


def write_readme() -> None:
    text = """# Canopy-Stratified Snow Observations

Normalized observation tables for the `tests/fixtures/cancov_forest/` canopy
gradient. These are external-authority observations for SNOWDENSITY-10.3.3 and
later snow/frost fidelity work.

## Files

- `sites/harvard_hf237_strata.csv`: Harvard Forest HF237 daily open,
  hardwood, and hemlock snow depth/SWE/density.
- `profiles/harvard_hf237_density_profiles.csv`: Harvard HF237 vertical density
  profile observations.
- `sites/marcell_rds_2021_0016_points.csv`: Marcell RDS-2021-0016 point-level
  snow depth/SWE observations.
- `sites/marcell_rds_2021_0016_snowcourse_means.csv`: Marcell snowcourse means.
- `sites/marcell_rds_2021_0016_stratum_means.csv`: Marcell conifer,
  deciduous, and open stratum means.

## Binding Notes

- Harvard `open` binds to `harvard_open_ma`.
- Harvard `hardwood` binds to `harvard_deciduous_ma`.
- Harvard `hemlock` is installed but remains unbound because the current
  Harvard fixture set has no pure hemlock/conifer hillslope.
- Marcell `conifer`, `deciduous`, and `open` bind to
  `marcell_conifer_mn`, `marcell_deciduous_mn`, and `marcell_open_mn`.
- Marcell snowcourse `S53` is retained as `unknown` in point/snowcourse files
  but excluded from stratum means because it is not described in the RDS
  metadata or coordinate supplement.

## Regeneration

```sh
.venv/bin/python tools/snowfreeze_observed/cancov_stratified_observations.py
```
"""
    (OUT_ROOT / "README.md").write_text(text, encoding="utf-8")


def marcell_binding(snowcourse: str) -> tuple[str, str, str]:
    if snowcourse in MARCELL_STRATA:
        observed_stratum, model_fixture = MARCELL_STRATA[snowcourse]
        return observed_stratum, model_fixture, "bound"
    return "unknown", "", "unbound_metadata_unmapped"


def marcell_quality(snowcourse: str, depth_m: float | None, swe_mm: float | None) -> str:
    flags = []
    if snowcourse not in MARCELL_STRATA:
        flags.append("metadata_snowcourse_unmapped")
    if depth_m is not None and swe_mm is not None and depth_m > 0.0:
        flags.append("density_derived_from_swe_depth")
    else:
        flags.append("density_unavailable")
    return ";".join(flags)


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8-sig") as handle:
        return list(csv.DictReader(handle))


def write_csv(path: Path, rows: list[dict[str, str]]) -> None:
    if not rows:
        raise RuntimeError(f"refusing to write empty CSV: {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(
            handle, fieldnames=list(rows[0].keys()), lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)


def write_json(path: Path, data: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def summarize_file(path: Path) -> FileSummary:
    return FileSummary(
        path=str(path.relative_to(REPO_ROOT) if path.is_relative_to(REPO_ROOT) else path),
        bytes=path.stat().st_size,
        sha256=sha256_file(path),
    )


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def parse_float(value: str) -> float | None:
    value = value.strip()
    if value == "" or value.upper() == "NA":
        return None
    parsed = float(value)
    if not math.isfinite(parsed):
        return None
    return parsed


def cm_to_m(value: float | None) -> float | None:
    return None if value is None else value / 100.0


def cm_to_mm(value: float | None) -> float | None:
    return None if value is None else value * 10.0


def density_from(depth_m: float | None, swe_mm: float | None) -> float | None:
    if depth_m is None or swe_mm is None or depth_m <= 0.0:
        return None
    return swe_mm / depth_m


def mean(values: list[float]) -> float | None:
    if not values:
        return None
    return sum(values) / len(values)


def format_optional(value: float | None, digits: int) -> str:
    if value is None:
        return ""
    return f"{value:.{digits}f}"


def water_year(date: dt.date) -> int:
    return date.year + 1 if date.month >= 10 else date.year


if __name__ == "__main__":
    raise SystemExit(main())
