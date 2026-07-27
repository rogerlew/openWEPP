#!/usr/bin/env python3
"""Authenticate inputs and build the frozen CAL-04B configuration inventory."""

from __future__ import annotations

import csv
import hashlib
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
CAL04A = ROOT / "docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/artifacts"
TIMING = CAL04A / "phenology-forcing-join.csv"
DAYMET_ROOT = ROOT / "references/canopy_phenology/daymet_calibration"
HUBBARD = ROOT / "tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh"
FORCING_AUTHORITY = ARTIFACTS / "calibration-forcing-authority-resolution.md"
FORCING_AUTHORITY_SHA256 = (
    "13c715046dd1ef700796d1651efa09b9e541f51cbb3c1a8feb2ebd45f072781d"
)


def digest(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def committed_digest(path: Path) -> str:
    """Return the SHA-256 of the committed object, not the working-tree object."""

    relative = path.relative_to(ROOT).as_posix()
    result = subprocess.run(
        ["git", "show", f"HEAD:{relative}"],
        cwd=ROOT,
        capture_output=True,
        check=True,
    )
    return hashlib.sha256(result.stdout).hexdigest()


def pinned_daymet() -> dict[str, str]:
    rows: dict[str, str] = {}
    sums = DAYMET_ROOT / "SHA256SUMS"
    for line in sums.read_text(encoding="ascii").splitlines():
        expected, relative = line.split(maxsplit=1)
        relative = relative.lstrip("*")
        if relative.startswith("raw/"):
            rows[relative] = expected
    if len(rows) != 9:
        raise ValueError(f"expected nine pinned Daymet sources, observed {len(rows)}")
    return rows


def write_csv(path: Path, fields: list[str], rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def factor(family: str, value: float, lower: float, upper: float) -> float:
    increasing = 0.0 if value <= lower else 1.0 if value >= upper else (value - lower) / (upper - lower)
    return 1.0 - increasing if family == "vpd" else increasing


def main() -> int:
    grid_path = CAL04A / "proposed-domain-grid.csv"
    with grid_path.open(newline="", encoding="utf-8") as stream:
        grid = list(csv.DictReader(stream))
    if len(grid) != 63:
        raise ValueError(f"expected 63 domain rows, observed {len(grid)}")
    groups: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in grid:
        if float(row["lower_value"]) >= float(row["upper_value"]):
            raise ValueError(f"unordered grid row {row['pair_id']}")
        groups[row["family"]].append(row)
    if [len(groups[name]) for name in ("temperature", "vpd", "photoperiod")] != [21, 21, 21]:
        raise ValueError("domain family counts differ from 21/21/21")
    (ARTIFACTS / "gsi-domain-grid.csv").write_bytes(grid_path.read_bytes())

    upper: dict[tuple[str, int], int] = {}
    with TIMING.open(newline="", encoding="utf-8") as stream:
        for row in csv.DictReader(stream):
            key = (row["plot_id"], int(row["year"]))
            upper[key] = max(upper.get(key, 0), int(row["interval_end_doy"]))
    forcing: dict[str, list[float]] = {"temperature": [], "vpd": [], "photoperiod": []}
    selected_counts: dict[tuple[str, int], int] = defaultdict(int)
    with (CAL04A / "daymet-daily-derived.csv").open(newline="", encoding="utf-8") as stream:
        for row in csv.DictReader(stream):
            key = (row["plot_id"], int(row["year"]))
            yday = int(row["yday"])
            if key not in upper or not (60 <= yday <= upper[key]):
                continue
            forcing["temperature"].append(float(row["tmin_c"]))
            forcing["vpd"].append(float(row["derived_vpd_pa"]))
            forcing["photoperiod"].append(float(row["native_photoperiod_hours"]))
            selected_counts[key] += 1
    if set(selected_counts) != set(upper):
        missing = sorted(set(upper) - set(selected_counts))
        extra = sorted(set(selected_counts) - set(upper))
        raise ValueError(f"saturation windows differ from timing ledger: missing={missing} extra={extra}")
    source_manifest_path = CAL04A / "source-and-request-manifest.csv"
    with source_manifest_path.open(newline="", encoding="utf-8") as stream:
        source_rows = {row["plot_id"]: row for row in csv.DictReader(stream)}
    if set(source_rows) != {plot_id for plot_id, _year in upper}:
        raise ValueError("Daymet source identities differ from calibration plots")
    derived_path = CAL04A / "daymet-daily-derived.csv"
    derived_sha = committed_digest(derived_path)
    inventory: list[dict[str, object]] = []
    for (plot_id, year), maximum_yday in sorted(upper.items()):
        count = selected_counts[(plot_id, year)]
        expected_count = maximum_yday - 59
        if count != expected_count:
            raise ValueError(
                f"non-contiguous saturation window {plot_id}/{year}: "
                f"{count} rows, expected {expected_count}"
            )
        source = source_rows[plot_id]
        source_path = ROOT / source["path"]
        if source["state"] != "VERIFIED" or digest(source_path) != source["sha256"]:
            raise ValueError(f"unauthenticated Daymet source for plot {plot_id}")
        inventory.append({
            "plot_id": plot_id,
            "year": year,
            "minimum_yday": 60,
            "maximum_calibration_interval_upper_doy": maximum_yday,
            "selected_row_count": count,
            "derived_source_path": str(derived_path.relative_to(ROOT)),
            "derived_source_sha256": derived_sha,
            "raw_source_id": source["source_id"],
            "raw_source_path": source["path"],
            "raw_source_sha256": source["sha256"],
            "state": "PASS",
        })
    write_csv(
        ARTIFACTS / "saturation-window-inventory.csv",
        list(inventory[0]),
        inventory,
    )
    pair_stats: dict[str, tuple[float, float, int]] = {}
    for family, pairs in groups.items():
        for pair in pairs:
            values = [
                factor(family, value, float(pair["lower_value"]), float(pair["upper_value"]))
                for value in forcing[family]
            ]
            pair_stats[pair["pair_id"]] = (min(values), max(values), len(values))

    configs: list[dict[str, object]] = []
    saturation: list[dict[str, object]] = []
    serial = 0
    for temperature in groups["temperature"]:
        for vpd in groups["vpd"]:
            for photoperiod in groups["photoperiod"]:
                serial += 1
                candidate_id = f"GSI-{serial:04d}"
                pairs = (temperature, vpd, photoperiod)
                lower_boundary = any(row["lower_level"] == "q00" for row in pairs)
                upper_boundary = any(row["upper_level"] == "q100" for row in pairs)
                boundary = "DOUBLE_BOUNDARY" if lower_boundary and upper_boundary else "LOWER_SUPPORT_BOUNDARY" if lower_boundary else "UPPER_SUPPORT_BOUNDARY" if upper_boundary else "INTERIOR"
                sat_flags: list[str] = []
                for family, row in zip(("temperature", "vpd", "photoperiod"), pairs):
                    minimum, maximum, count = pair_stats[row["pair_id"]]
                    result = maximum - minimum <= 1e-12
                    if result:
                        sat_flags.append(family)
                    saturation.append({
                        "candidate_id": candidate_id,
                        "family": family,
                        "plot_year_windows": len(upper),
                        "population_rows": count,
                        "minimum_yday": 60,
                        "maximum_interval_upper_doy": max(upper.values()),
                        "factor_min": f"{minimum:.17g}",
                        "factor_max": f"{maximum:.17g}",
                        "factor_range": f"{maximum - minimum:.17g}",
                        "result": "SATURATED_ON_OBSERVED_FORCING" if result else "ACTIVE_ON_OBSERVED_FORCING",
                    })
                configs.append({
                    "candidate_id": candidate_id,
                    "temperature_pair_id": temperature["pair_id"],
                    "vpd_pair_id": vpd["pair_id"],
                    "photoperiod_pair_id": photoperiod["pair_id"],
                    "minimum_temperature_inactive_c": temperature["lower_value"],
                    "minimum_temperature_unconstrained_c": temperature["upper_value"],
                    "vapor_pressure_deficit_unconstrained_pa": vpd["lower_value"],
                    "vapor_pressure_deficit_inactive_pa": vpd["upper_value"],
                    "photoperiod_inactive_hours": photoperiod["lower_value"],
                    "photoperiod_unconstrained_hours": photoperiod["upper_value"],
                    "boundary_class": boundary,
                    "saturation_flags": ";".join(sat_flags) or "NONE",
                })
    if serial != 9261:
        raise ValueError(f"enumerated {serial} candidates")
    write_csv(ARTIFACTS / "candidate-configurations.csv", list(configs[0]), configs)
    write_csv(ARTIFACTS / "saturation-evidence.csv", list(saturation[0]), saturation)

    inputs = [
        ("domain_grid", grid_path, "CAL04A_FROZEN_GRID"),
        (
            "calibration_forcing_authority_resolution",
            FORCING_AUTHORITY,
            "RESULT_BLIND_BINDING_AUTHORITY",
        ),
        ("daymet_derived", derived_path, "SATURATION_FORCING"),
        (
            "daymet_sha256sums",
            DAYMET_ROOT / "SHA256SUMS",
            "PINNED_DAYMET_CUSTODY_LEDGER",
        ),
        (
            "daymet_source_request_manifest",
            source_manifest_path,
            "CAL04A_SOURCE_IDENTITY_LEDGER",
        ),
        (
            "hubbard_plot_geometry",
            CAL04A / "hubbard-plot-geometry.csv",
            "CAL04A_SOURCE_EML_PLOT_GEOMETRY",
        ),
        ("timing", TIMING, "ROLE_FILTERED_OPERATOR_INPUT"),
        (
            "operator",
            ROOT / "docs/work-packages/20260726-canopy-cal-04-process-calibration-identifiability-001/artifacts/objective-and-observation-operator.md",
            "FROZEN_OPERATOR",
        ),
    ]
    inputs.extend(
        (
            f"daymet_source_{relative.removeprefix('raw/').removesuffix('.csv')}",
            DAYMET_ROOT / relative,
            "PINNED_DAYMET_SOURCE",
        )
        for relative in sorted(pinned_daymet())
    )
    hubbard_names = [
        "gwcoeff.txt",
        "manifest.md",
        "p10.cli",
        "p10.man",
        "p10.man.yaml",
        "p10.native.run.toml",
        "p10.run",
        "p10.slp",
        "p10.sol",
        "pmetpara.txt",
        "snow.txt",
    ]
    inputs.extend(
        (f"hubbard_{name.replace('.', '_')}", HUBBARD / name, "HUBBARD_FULL_FIXTURE")
        for name in hubbard_names
    )
    daymet_expected = {
        (DAYMET_ROOT / relative).resolve(): expected
        for relative, expected in pinned_daymet().items()
    }
    manifest = []
    for input_id, path, role in inputs:
        if path == FORCING_AUTHORITY:
            expected = FORCING_AUTHORITY_SHA256
        elif path.resolve() in daymet_expected:
            expected = daymet_expected[path.resolve()]
        else:
            expected = committed_digest(path)
        observed = digest(path)
        manifest.append({
            "input_id": input_id,
            "path": str(path.relative_to(ROOT)),
            "role": role,
            "expected_sha256": expected,
            "observed_sha256": observed,
            "state": "PASS" if observed == expected else "FAIL",
        })
    failed = [row["input_id"] for row in manifest if row["state"] != "PASS"]
    if failed:
        raise ValueError(f"custody mismatch for inputs: {', '.join(failed)}")
    write_csv(ARTIFACTS / "input-and-authority-manifest.csv", list(manifest[0]), manifest)
    print(f"PASS candidates={len(configs)} saturation_rows={len(saturation)} forcing_rows={sum(len(v) for v in forcing.values())}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        sys.exit(1)
