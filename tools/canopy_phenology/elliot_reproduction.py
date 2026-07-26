#!/usr/bin/env python3
"""Deterministic CANOPY-CAL-02 WEPP 2012 reproduction harness."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import re
import shutil
import stat
import subprocess
import tempfile
from dataclasses import asdict, dataclass
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
FIXTURES = REPO / "tests/fixtures/canopy_phenology/elliot_reproduction"
SOURCES = FIXTURES / "source/weppcloud"
REFERENCES = REPO / "references/canopy_phenology/elliot_2026"
EXECUTABLE_SHA256 = "6104a3440624ad54aa6c3660794280adfd600d4a11b98559c6205a73cd47fc3f"
LINUX_EXECUTABLE_SHA256 = "7e0ccad2a79cebf63ad821b140ef3007ca5846ca9b646e87559448c38e4d0d91"
REMOTE_EXE = r"C:\WEPP\wepp\wepp_2012.exe"
SAFE_REMOTE = re.compile(r"^C:/Users/roger/AppData/Local/Temp/openwepp-cal02-[A-Za-z0-9_-]+$")
ADMITTED_HOST = "BLARHG"


@dataclass(frozen=True)
class Arm:
    arm_id: str
    site: str
    source_dir: str
    stem: str
    management: str
    dropfc: float | None
    mukey: int
    effective_length_m: float


ARMS = (
    Arm("hubbard_constant", "HUBBARD_BROOK", "hubbard_brook_unassailable_sensuousness", "p1", "source", None, 665220, 251.8),
    Arm("hubbard_hardwood_095", "HUBBARD_BROOK", "hubbard_brook_unassailable_sensuousness", "p1", "hardwood", 0.95, 665220, 251.8),
    Arm("hubbard_hardwood_092", "HUBBARD_BROOK", "hubbard_brook_unassailable_sensuousness", "p1", "hardwood", 0.92, 665220, 251.8),
    Arm("santee_constant", "SANTEE", "santee_clean_burning_griddle", "p2", "source", None, 131976, 300.0),
    Arm("santee_mixed", "SANTEE", "santee_clean_burning_griddle", "p2", "mixed", 0.93, 131976, 300.0),
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def verify_source(arm: Arm) -> None:
    root = SOURCES / arm.source_dir
    subprocess.run(["sha256sum", "-c", "SHA256SUMS"], cwd=root, check=True, stdout=subprocess.DEVNULL)


def derive_management(arm: Arm, target: Path) -> dict[str, object]:
    if arm.management == "source":
        source = SOURCES / arm.source_dir / "inputs" / f"{arm.stem}.man"
        shutil.copyfile(source, target)
        return {"source": str(source.relative_to(REPO)), "transformation": "byte-identical"}
    source = REFERENCES / (
        "bill_elliot_2026_hardwood_forest.man"
        if arm.management == "hardwood"
        else "bill_elliot_2026_santee_mixed_forest.man"
    )
    data = source.read_text(encoding="utf-8")
    transformation = "byte-identical replacement"
    if arm.dropfc == 0.92:
        needle = "0.85000 0.95000 0.65000 0.99000"
        replacement = "0.85000 0.92000 0.65000 0.99000"
        if data.count(needle) != 1:
            raise ValueError("hardwood dropfc token is not unique")
        data = data.replace(needle, replacement)
        transformation = "single-token dropfc 0.95000 -> 0.92000"
    target.write_text(data, encoding="utf-8", newline="")
    return {"source": str(source.relative_to(REPO)), "transformation": transformation}


def derive_run_control(source: Path, target: Path, stem: str) -> None:
    lines = source.read_text(encoding="utf-8").splitlines()
    if len(lines) != 31 or lines[11].strip().lower() != "no":
        raise ValueError("unexpected source run-control layout")
    lines[11] = "Yes"
    lines.insert(12, f"../output/{stem}.crop.dat")
    target.write_text("\n".join(lines) + "\n", encoding="ascii")


def copy_regular_inputs(source_inputs: Path, run_dir: Path) -> None:
    for source in source_inputs.iterdir():
        mode = source.lstat().st_mode
        if not stat.S_ISREG(mode):
            raise ValueError(f"source fixture entry is not a regular file: {source}")
        shutil.copyfile(source, run_dir / source.name)


def prepare_arm(arm: Arm, root: Path, remote_root: str, soil_root: Path | None = None) -> Path:
    verify_source(arm)
    arm_root = root / arm.arm_id
    run_dir = arm_root / "run"
    output_dir = arm_root / "output"
    run_dir.mkdir(parents=True)
    output_dir.mkdir()
    source_inputs = SOURCES / arm.source_dir / "inputs"
    copy_regular_inputs(source_inputs, run_dir)
    soil_derivation: dict[str, object] = {"transformation": "byte-identical source-native soil"}
    if soil_root is not None:
        derived_soil = soil_root / f"{arm.mukey}.sol"
        if not derived_soil.is_file() or derived_soil.is_symlink():
            raise ValueError(f"missing regular WEPPpy 2006.2 soil: {derived_soil}")
        if derived_soil.read_text(encoding="utf-8").splitlines()[0].strip() != "2006.2":
            raise ValueError(f"derived soil is not WEPP 2006.2: {derived_soil}")
        shutil.copyfile(derived_soil, run_dir / f"{arm.stem}.sol")
        soil_derivation = {
            "transformation": "replace source-native 9002 soil with WEPPpy SSURGO 2006.2 serialization",
            "mukey": arm.mukey,
            "source": str(derived_soil),
            "sha256": sha256(derived_soil),
        }
    management = derive_management(arm, run_dir / f"{arm.stem}.man")
    derive_run_control(source_inputs / f"{arm.stem}.run", run_dir / f"{arm.stem}.run", arm.stem)
    remote_win = remote_root.replace("/", "\\") + "\\" + arm.arm_id
    command = (
        "@echo off\r\n"
        f"cd /d {remote_win}\\run\r\n"
        f"{REMOTE_EXE} < {arm.stem}.run > ..\\stdout.txt 2> ..\\stderr.txt\r\n"
        "echo %ERRORLEVEL% > ..\\exit_code.txt\r\n"
    )
    (arm_root / "run.cmd").write_text(command, encoding="ascii", newline="")
    inventory = {}
    for path in sorted(run_dir.iterdir()):
        inventory[path.name] = {"sha256": sha256(path), "size_bytes": path.stat().st_size}
    manifest = {
        "schema_version": 1,
        "arm": asdict(arm),
        "management_derivation": management,
        "soil_derivation": soil_derivation,
        "run_control_derivation": "source byte copy plus crop diagnostic No -> Yes and confined filename insertion",
        "executable": {"path": REMOTE_EXE, "sha256": EXECUTABLE_SHA256},
        "inputs": inventory,
    }
    (arm_root / "input-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    return arm_root


def remote_executable_sha256(host: str) -> str:
    if host != ADMITTED_HOST:
        raise ValueError(f"only the admitted host {ADMITTED_HOST} is allowed")
    completed = subprocess.run(
        ["ssh", "-o", "BatchMode=yes", host, 'cmd /d /c "certutil -hashfile C:\\WEPP\\wepp\\wepp_2012.exe SHA256"'],
        check=True,
        capture_output=True,
        text=True,
    )
    matches = re.findall(r"\b[0-9a-fA-F]{64}\b", completed.stdout)
    if len(matches) != 1:
        raise RuntimeError("could not parse a unique remote executable SHA-256")
    return matches[0].lower()


def run_matrix(results_root: Path, remote_root: str, host: str, soil_root: Path | None = None) -> None:
    if not SAFE_REMOTE.fullmatch(remote_root):
        raise ValueError("remote root is outside the admitted package-owned temp namespace")
    if host != ADMITTED_HOST:
        raise ValueError(f"only the admitted host {ADMITTED_HOST} is allowed")
    if remote_executable_sha256(host) != EXECUTABLE_SHA256:
        raise RuntimeError("remote executable identity mismatch before matrix")
    results_root.mkdir(parents=True, exist_ok=False)
    subprocess.run(
        ["ssh", "-o", "BatchMode=yes", host, f'cmd /d /c "mkdir {remote_root.replace("/", chr(92))}"'],
        check=True,
    )
    failures = []
    for arm in ARMS:
        arm_root = prepare_arm(arm, results_root, remote_root, soil_root)
        subprocess.run(["scp", "-q", "-r", str(arm_root), f"{host}:{remote_root}/"], check=True)
        remote_cmd = remote_root.replace("/", "\\") + "\\" + arm.arm_id + "\\run.cmd"
        subprocess.run(["ssh", "-o", "BatchMode=yes", host, f'cmd /d /c "{remote_cmd}"'], check=True)
        fetched = Path(tempfile.mkdtemp(prefix=f"cal02-{arm.arm_id}-"))
        subprocess.run(["scp", "-q", "-r", f"{host}:{remote_root}/{arm.arm_id}", str(fetched)], check=True)
        remote_copy = fetched / arm.arm_id
        for name in ("output", "stdout.txt", "stderr.txt", "exit_code.txt"):
            source = remote_copy / name
            target = arm_root / name
            if target.exists() and target.is_dir():
                shutil.rmtree(target)
            if source.is_dir():
                shutil.copytree(source, target)
            elif source.exists():
                shutil.copyfile(source, target)
        shutil.rmtree(fetched)
        exit_code = (arm_root / "exit_code.txt").read_text(encoding="ascii").strip()
        output_inventory = {
            path.name: {"sha256": sha256(path), "size_bytes": path.stat().st_size}
            for path in sorted((arm_root / "output").iterdir())
            if path.is_file()
        }
        (arm_root / "output-manifest.json").write_text(
            json.dumps({"schema_version": 1, "arm_id": arm.arm_id, "outputs": output_inventory}, indent=2) + "\n",
            encoding="utf-8",
        )
        if exit_code != "0":
            failures.append(f"{arm.arm_id}: exit {exit_code}")
    if remote_executable_sha256(host) != EXECUTABLE_SHA256:
        raise RuntimeError("remote executable identity mismatch after matrix")
    if failures:
        raise RuntimeError("matrix arm failures: " + ", ".join(failures))


def run_linux_matrix(results_root: Path, executable: Path) -> None:
    if not executable.is_file() or executable.is_symlink():
        raise ValueError(f"missing regular Linux WEPP executable: {executable}")
    if sha256(executable) != LINUX_EXECUTABLE_SHA256:
        raise RuntimeError("Linux WEPP executable identity mismatch before matrix")
    results_root.mkdir(parents=True, exist_ok=False)
    failures = []
    for arm in ARMS:
        arm_root = prepare_arm(
            arm,
            results_root,
            "C:/Users/roger/AppData/Local/Temp/openwepp-cal02-linux-placeholder",
        )
        run_dir = arm_root / "run"
        (run_dir / "wepp_ui.txt").write_text("", encoding="ascii")
        (run_dir / "wepp_observe.on").write_text("", encoding="ascii")
        input_manifest_path = arm_root / "input-manifest.json"
        input_manifest = json.loads(input_manifest_path.read_text(encoding="utf-8"))
        input_manifest["executable"] = {
            "path": str(executable),
            "sha256": LINUX_EXECUTABLE_SHA256,
        }
        for switch_name in ("wepp_ui.txt", "wepp_observe.on"):
            switch_path = run_dir / switch_name
            input_manifest["inputs"][switch_name] = {
                "sha256": sha256(switch_path),
                "size_bytes": switch_path.stat().st_size,
            }
        input_manifest["runtime_switches"] = {
            "wepp_ui.txt": "present: hourly water balance",
            "wepp_observe.on": "present: fixed-callsite observability",
        }
        input_manifest_path.write_text(
            json.dumps(input_manifest, indent=2) + "\n", encoding="utf-8"
        )
        with (run_dir / f"{arm.stem}.run").open("rb") as stdin, (
            arm_root / "stdout.txt"
        ).open("wb") as stdout, (arm_root / "stderr.txt").open("wb") as stderr:
            completed = subprocess.run(
                [str(executable)],
                cwd=run_dir,
                stdin=stdin,
                stdout=stdout,
                stderr=stderr,
                check=False,
            )
        (arm_root / "exit_code.txt").write_text(f"{completed.returncode}\n", encoding="ascii")
        output_inventory = {
            path.name: {"sha256": sha256(path), "size_bytes": path.stat().st_size}
            for path in sorted((arm_root / "output").iterdir())
            if path.is_file()
        }
        diagnostic_inventory = {
            path.name: {"sha256": sha256(path), "size_bytes": path.stat().st_size}
            for path in (run_dir / "wepp_observe.log", run_dir / "wepp_observe_bottom.csv", run_dir / "wepp_observe_pmet.csv")
            if path.is_file()
        }
        transcript_inventory = {
            path.name: {"sha256": sha256(path), "size_bytes": path.stat().st_size}
            for path in (
                arm_root / "stdout.txt",
                arm_root / "stderr.txt",
                arm_root / "exit_code.txt",
            )
        }
        (arm_root / "output-manifest.json").write_text(
            json.dumps(
                {
                    "schema_version": 2,
                    "arm_id": arm.arm_id,
                    "executable_sha256": LINUX_EXECUTABLE_SHA256,
                    "soil_format": "9002 source-native",
                    "hourly_water_balance": True,
                    "outputs": output_inventory,
                    "observe_outputs": diagnostic_inventory,
                    "transcripts": transcript_inventory,
                },
                indent=2,
            )
            + "\n",
            encoding="utf-8",
        )
        if completed.returncode != 0:
            failures.append(f"{arm.arm_id}: exit {completed.returncode}")
    if sha256(executable) != LINUX_EXECUTABLE_SHA256:
        raise RuntimeError("Linux WEPP executable identity mismatch after matrix")
    if failures:
        raise RuntimeError("Linux matrix arm failures: " + ", ".join(failures))


def parse_crop(path: Path, arm: Arm) -> list[dict[str, object]]:
    def fixed_width_float(value: str) -> tuple[float | None, bool]:
        if value and set(value) == {"*"}:
            return None, True
        return float(value), False

    rows = []
    for line in path.read_text(encoding="ascii", errors="strict").splitlines():
        fields = line.split()
        if len(fields) != 27 or not fields[0].isdigit():
            continue
        canopy_height, canopy_height_overflow = fixed_width_float(fields[3])
        lai, lai_overflow = fixed_width_float(fields[5])
        row = {
            "arm_id": arm.arm_id,
            "site": arm.site,
            "ofe": int(fields[0]),
            "jday": int(fields[1]),
            "year": int(fields[2]),
            "canopy_height_m": canopy_height,
            "canopy_height_overflow": canopy_height_overflow,
            "canopy_cover_fraction": float(fields[4]),
            "lai_m2_m2": lai,
            "lai_overflow": lai_overflow,
            "rill_cover_fraction": float(fields[6]),
            "interrill_cover_fraction": float(fields[7]),
            "live_biomass_kg_m2": float(fields[9]),
            "standing_residue_kg_m2": float(fields[10]),
            "current_flat_residue_kg_m2": float(fields[12]),
            "previous_flat_residue_kg_m2": float(fields[14]),
            "old_flat_residue_kg_m2": float(fields[16]),
            "buried_residue_kg_m2": sum(float(fields[i]) for i in (17, 18, 19)),
            "dead_root_kg_m2": sum(float(fields[i]) for i in (21, 23, 25)),
        }
        row["total_flat_residue_kg_m2"] = sum(
            float(row[key])
            for key in ("current_flat_residue_kg_m2", "previous_flat_residue_kg_m2", "old_flat_residue_kg_m2")
        )
        rows.append(row)
    if not rows:
        raise ValueError(f"no crop rows parsed from {path}")
    return rows


def parse_events(path: Path, arm: Arm) -> list[dict[str, object]]:
    rows = []
    for line in path.read_text(encoding="ascii", errors="strict").splitlines():
        fields = line.split()
        if len(fields) != 14 or not all(re.fullmatch(r"-?\d+(?:\.\d+)?", value) for value in fields):
            continue
        rows.append(
            {
                "arm_id": arm.arm_id,
                "site": arm.site,
                "day": int(fields[0]),
                "month": int(fields[1]),
                "year": int(fields[2]),
                "precipitation_mm": float(fields[3]),
                "runoff_mm": float(fields[4]),
                "sediment_delivery_kg_m": float(fields[12]),
            }
        )
    return rows


def parse_peak_runoff(path: Path) -> list[float]:
    values = []
    for line in path.read_text(encoding="ascii", errors="strict").splitlines():
        fields = line.split()
        if len(fields) in (24, 26) and fields[0].isdigit():
            values.append(float(fields[7]))
    if not values:
        raise ValueError(f"no element rows parsed from {path}")
    return values


def write_csv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        path.write_text("", encoding="utf-8")
        return
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(
            stream, fieldnames=list(rows[0]), lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)


def reconstruct_annual_litter_transfer(rows: list[dict[str, object]], arm: Arm) -> float:
    if arm.dropfc is None:
        return 0.0
    return max(float(row["live_biomass_kg_m2"]) for row in rows) * (1.0 - arm.dropfc)


def analyze(results_root: Path, artifact_root: Path) -> None:
    events: list[dict[str, object]] = []
    annual: list[dict[str, object]] = []
    equilibrium: list[dict[str, object]] = []
    return_periods: list[dict[str, object]] = []
    litter_transfer: list[dict[str, object]] = []
    daily_manifest: dict[str, object] = {"schema_version": 1, "arms": []}
    for arm in ARMS:
        output = results_root / arm.arm_id / "output"
        crop_rows = parse_crop(output / f"{arm.stem}.crop.dat", arm)
        event_rows = parse_events(output / f"H{arm.stem[1:]}.ebe.dat", arm)
        events.extend(event_rows)
        peak_runoff_values = sorted(
            parse_peak_runoff(output / f"H{arm.stem[1:]}.element.dat"), reverse=True
        )
        events_by_year: dict[int, list[dict[str, object]]] = {}
        for row in event_rows:
            events_by_year.setdefault(int(row["year"]), []).append(row)
        by_year: dict[int, list[dict[str, object]]] = {}
        for row in crop_rows:
            by_year.setdefault(int(row["year"]), []).append(row)
        if sorted(by_year) != list(range(1, 101)):
            raise ValueError(f"{arm.arm_id} does not contain years 1..100")
        for year, rows in sorted(by_year.items()):
            first, last = rows[0], rows[-1]
            live_decline = sum(
                max(0.0, float(previous["live_biomass_kg_m2"]) - float(current["live_biomass_kg_m2"]))
                for previous, current in zip(rows, rows[1:])
            )
            annual_litter_transfer = reconstruct_annual_litter_transfer(rows, arm)
            annual.append(
                {
                    "arm_id": arm.arm_id,
                    "site": arm.site,
                    "year": year,
                    "live_biomass_year_end_kg_m2": last["live_biomass_kg_m2"],
                    "flat_residue_year_end_kg_m2": last["total_flat_residue_kg_m2"],
                    "current_flat_residue_year_end_kg_m2": last["current_flat_residue_kg_m2"],
                    "previous_flat_residue_year_end_kg_m2": last["previous_flat_residue_kg_m2"],
                    "old_flat_residue_year_end_kg_m2": last["old_flat_residue_kg_m2"],
                    "live_biomass_min_kg_m2": min(float(r["live_biomass_kg_m2"]) for r in rows),
                    "live_biomass_max_kg_m2": max(float(r["live_biomass_kg_m2"]) for r in rows),
                    "flat_residue_min_kg_m2": min(float(r["total_flat_residue_kg_m2"]) for r in rows),
                    "flat_residue_max_kg_m2": max(float(r["total_flat_residue_kg_m2"]) for r in rows),
                    "annual_live_decline_sum_kg_m2": live_decline,
                    "annual_litter_transfer_kg_m2": annual_litter_transfer,
                    "annual_runoff_mm": sum(float(r["runoff_mm"]) for r in events_by_year.get(year, [])),
                    "annual_sediment_delivery_kg_m": sum(
                        float(r["sediment_delivery_kg_m"]) for r in events_by_year.get(year, [])
                    ),
                }
            )
        tail = [row for row in annual if row["arm_id"] == arm.arm_id and int(row["year"]) >= 91]
        equilibrium.append(
            {
                "arm_id": arm.arm_id,
                "site": arm.site,
                "window": "years_91_100",
                "mean_year_end_live_biomass_kg_m2": sum(float(r["live_biomass_year_end_kg_m2"]) for r in tail) / 10,
                "mean_year_end_flat_residue_kg_m2": sum(float(r["flat_residue_year_end_kg_m2"]) for r in tail) / 10,
                "mean_year_end_current_residue_kg_m2": sum(
                    float(r["current_flat_residue_year_end_kg_m2"]) for r in tail
                ) / 10,
                "mean_year_end_previous_residue_kg_m2": sum(
                    float(r["previous_flat_residue_year_end_kg_m2"]) for r in tail
                ) / 10,
                "mean_year_end_old_residue_kg_m2": sum(
                    float(r["old_flat_residue_year_end_kg_m2"]) for r in tail
                ) / 10,
                "live_biomass_range_kg_m2": max(float(r["live_biomass_year_end_kg_m2"]) for r in tail)
                - min(float(r["live_biomass_year_end_kg_m2"]) for r in tail),
                "flat_residue_range_kg_m2": max(float(r["flat_residue_year_end_kg_m2"]) for r in tail)
                - min(float(r["flat_residue_year_end_kg_m2"]) for r in tail),
            }
        )
        transfer_rows = [row for row in annual if row["arm_id"] == arm.arm_id]
        transfer_tail = transfer_rows[-10:]
        litter_transfer.append(
            {
                "arm_id": arm.arm_id,
                "site": arm.site,
                "method": (
                    "max daily live biomass * (1 - dropfc); grow.for perennial senescence delvd authority"
                    if arm.dropfc is not None
                    else "constant-cover arm; no senescence transfer"
                ),
                "dropfc": arm.dropfc,
                "mean_annual_transfer_kg_m2": sum(
                    float(row["annual_litter_transfer_kg_m2"]) for row in transfer_rows
                )
                / 100,
                "mean_years_91_100_transfer_kg_m2": sum(
                    float(row["annual_litter_transfer_kg_m2"]) for row in transfer_tail
                )
                / 10,
                "max_formula_vs_daily_decline_abs_diff_kg_m2": max(
                    abs(
                        float(row["annual_litter_transfer_kg_m2"])
                        - float(row["annual_live_decline_sum_kg_m2"])
                    )
                    for row in transfer_rows
                ),
            }
        )
        runoff_values = sorted(
            [float(row["runoff_mm"]) for row in event_rows] + [0.0] * (36525 - len(event_rows)),
            reverse=True,
        )
        for recurrence_years in (2, 5, 10, 20, 25, 50):
            rank = 100 // recurrence_years
            for surface, unit, values in (
                ("daily_hillslope_surface_runoff", "mm", runoff_values),
                ("peak_hillslope_runoff_rate", "mm/h", peak_runoff_values),
            ):
                return_periods.append(
                    {
                        "arm_id": arm.arm_id,
                        "site": arm.site,
                        "surface": surface,
                        "recurrence_years": recurrence_years,
                        "empirical_rank_100_years": rank,
                        "return_level": values[rank - 1],
                        "unit": unit,
                    }
                )
        daily_manifest["arms"].append(
            {
                "arm_id": arm.arm_id,
                "source": str((output / f"{arm.stem}.crop.dat").relative_to(results_root)),
                "source_sha256": sha256(output / f"{arm.stem}.crop.dat"),
                "normalized_rows": len(crop_rows),
                "canopy_height_overflow_rows": sum(bool(row["canopy_height_overflow"]) for row in crop_rows),
                "lai_overflow_rows": sum(bool(row["lai_overflow"]) for row in crop_rows),
            }
        )
    artifact_root.mkdir(parents=True, exist_ok=True)
    write_csv(artifact_root / "event-results.csv", events)
    write_csv(artifact_root / "annual-results.csv", annual)
    write_csv(artifact_root / "equilibrium-results.csv", equilibrium)
    write_csv(artifact_root / "return-period-results.csv", return_periods)
    write_csv(artifact_root / "litter-transfer-summary.csv", litter_transfer)
    (artifact_root / "daily-results-manifest.json").write_text(
        json.dumps(daily_manifest, indent=2) + "\n", encoding="utf-8"
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest="command", required=True)
    run = sub.add_parser("run")
    run.add_argument("--results-root", type=Path, required=True)
    run.add_argument("--remote-root", required=True)
    run.add_argument("--host", choices=[ADMITTED_HOST], default=ADMITTED_HOST)
    run.add_argument("--soil-root", type=Path)
    analysis = sub.add_parser("analyze")
    analysis.add_argument("--results-root", type=Path, required=True)
    analysis.add_argument("--artifact-root", type=Path, required=True)
    linux = sub.add_parser("run-linux")
    linux.add_argument("--results-root", type=Path, required=True)
    linux.add_argument("--executable", type=Path, required=True)
    prepare = sub.add_parser("prepare")
    prepare.add_argument("--arm", choices=[arm.arm_id for arm in ARMS], required=True)
    prepare.add_argument("--root", type=Path, required=True)
    prepare.add_argument("--remote-root", required=True)
    args = parser.parse_args()
    if args.command == "run":
        soil_root = args.soil_root.resolve() if args.soil_root else None
        run_matrix(args.results_root.resolve(), args.remote_root, args.host, soil_root)
    elif args.command == "analyze":
        analyze(args.results_root.resolve(), args.artifact_root.resolve())
    elif args.command == "run-linux":
        run_linux_matrix(args.results_root.resolve(), args.executable.resolve())
    else:
        if not SAFE_REMOTE.fullmatch(args.remote_root):
            raise ValueError("invalid remote root")
        arm = next(item for item in ARMS if item.arm_id == args.arm)
        args.root.mkdir(parents=True, exist_ok=True)
        prepare_arm(arm, args.root.resolve(), args.remote_root)


if __name__ == "__main__":
    main()
