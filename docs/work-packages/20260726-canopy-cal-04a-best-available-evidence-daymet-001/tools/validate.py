#!/usr/bin/env python3
"""Validate CAL-04A source, derivation, role, and deterministic-design invariants."""

from __future__ import annotations

import csv
import hashlib
import math
import subprocess
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ART = PACKAGE / "artifacts"
RAW = ROOT / "references/canopy_phenology/daymet_calibration"
TIMING = ROOT / "docs/work-packages/20260726-canopy-cal-04-05-authority-evidence-admission-001/artifacts/cal04-timing-windows.csv"


def rows(name: str) -> list[dict[str, str]]:
    with (ART / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    subprocess.run(["sha256sum", "-c", "SHA256SUMS"], cwd=RAW, check=True)
    daymet = rows("daymet-daily-derived.csv")
    fixture = rows("fixture-daily-derived.csv")
    joined = rows("phenology-forcing-join.csv")
    support = rows("forcing-support-summary.csv")
    grid = rows("proposed-domain-grid.csv")
    manifest = rows("source-and-request-manifest.csv")
    receipts = rows("retrieval-receipt.csv")
    assert len(manifest) == 9
    assert len(receipts) == 9
    assert {r["plot_id"] for r in manifest} == {"1B", "4B", "4T", "5B", "5T", "6T", "7B", "7T", "HQ"}
    assert len(daymet) == 118260 and len(fixture) == 13149
    assert len(joined) == 932 and len(support) == 21 and len(grid) == 63
    assert all(float(r["derived_vpd_pa"]) >= 0 for r in daymet)
    assert all("Harvard" not in "|".join(r.values()) for r in joined)
    assert len([r for r in grid if r["family"] == "temperature"]) == 21
    assert len([r for r in grid if r["family"] == "vpd"]) == 21
    assert len([r for r in grid if r["family"] == "photoperiod"]) == 21
    assert 21**3 == 9261

    def es(value: float) -> float:
        return 0.6108 * math.exp(17.27 * value / (value + 237.3))

    for value, expected in [
        (-20.0, 0.124619111841318),
        (0.0, 0.6108),
        (20.0, 2.338281270927446),
    ]:
        assert abs(es(value) - expected) <= 1e-12

    def photo(latitude: float, yday: int) -> float:
        lat = math.radians(latitude)
        dec = 0.409 * math.sin(2.0 * math.pi * yday / 365.0 - 1.39)
        omega = math.acos(max(-1.0, min(1.0, -math.tan(lat) * math.tan(dec))))
        return 24.0 * omega / math.pi

    for yday, expected in [
        (60, 10.936679061903382),
        (120, 13.946398518663878),
        (180, 15.254691710597676),
    ]:
        assert abs(photo(43.94, yday) - expected) <= 1e-12

    latitudes = {r["plot_id"]: float(r["requested_latitude"]) for r in manifest}
    daymet_index: dict[tuple[str, str, str], dict[str, str]] = {}
    for r in daymet:
        expected_vpd = 1000.0 * (
            0.5 * (es(float(r["tmax_c"])) + es(float(r["tmin_c"])))
            - float(r["vp_pa"]) / 1000.0
        )
        assert abs(float(r["derived_vpd_pa"]) - expected_vpd) <= 1e-6
        assert abs(
            float(r["native_photoperiod_hours"])
            - photo(latitudes[r["plot_id"]], int(r["yday"]))
        ) <= 1e-9
        assert date.fromisoformat(r["date"]).timetuple().tm_yday == int(r["yday"])
        key = (r["plot_id"], r["year"], r["yday"])
        assert key not in daymet_index
        daymet_index[key] = r

    with TIMING.open(newline="", encoding="utf-8") as stream:
        calibration = [
            r for r in csv.DictReader(stream) if r["role"] == "CALIBRATION"
        ]
    assert len({r["record_id"] for r in calibration}) == len(calibration) == 932
    assert {r["record_id"] for r in joined} == {
        r["record_id"] for r in calibration
    }
    timing_by_id = {r["record_id"]: r for r in calibration}
    for r in joined:
        source = timing_by_id[r["record_id"]]
        assert r["plot_id"] == source["site"] and r["year"] == source["year"]
        assert r["interval_start_doy"] == source["interval_start_doy"]
        assert r["interval_end_doy"] == source["interval_end_doy"]
        for prefix, yday in [
            ("start", r["interval_start_doy"]),
            ("end", r["interval_end_doy"]),
        ]:
            forcing = daymet_index[(r["plot_id"], r["year"], yday)]
            assert r[f"{prefix}_tmin_c"] == forcing["tmin_c"]
            assert r[f"{prefix}_vpd_pa"] == forcing["derived_vpd_pa"]
            assert (
                r[f"{prefix}_photoperiod_hours"]
                == forcing["native_photoperiod_hours"]
            )

    generated = [
        "daymet-daily-derived.csv", "fixture-daily-derived.csv",
        "phenology-forcing-join.csv", "forcing-support-summary.csv",
        "proposed-domain-grid.csv", "fixture-daymet-comparison.csv",
        "correlation-and-confounding.md", "elevation-analysis.md",
        "vpd-and-photoperiod-method.md", "phenology-anomaly-association.md",
    ]
    before = {name: digest(ART / name) for name in generated}
    subprocess.run([str(ROOT / ".venv/bin/python"), str(PACKAGE / "tools/analyze.py")], check=True)
    after = {name: digest(ART / name) for name in generated}
    assert before == after, "analysis rebuild was not byte-deterministic"
    print("PASS CAL-04A: sources, roles, counts, VPD, grid, and deterministic rebuild")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
