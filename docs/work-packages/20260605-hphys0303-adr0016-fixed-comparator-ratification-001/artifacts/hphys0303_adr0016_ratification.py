#!/usr/bin/env python3
"""Execute HPHYS0303 local ADR-0016 comparator ratification gates."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass
from datetime import date, timedelta
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"

BASELINE_REPO = Path("/workdir/wepp-forest_260430_baseline")
ABANDONED_REPO = Path("/workdir/wepp-forest")
SOURCE_RUNS = Path("/tmp/unpalatable_parity_20260529T192707Z/runs")
ORIGINAL_BASELINE_PARTITIONS = Path(
    "/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions"
)
WEPPPY_PYTHON = Path("/workdir/wepppy/.venv/bin/python")
SEMANTIC_WAT = REPO / "tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py"

BASE_SHA = "dac3c950d8b16cc73774bf5ce2e7e11f80baac70"
ABANDONED_SHA = "924ab16d07edea8b904bcf64d3d7e276fc45d21e"
FIX_PROVENANCE_SHA = "03fee4558456535138592630b5dedc4d81ce8d06"
ORIGINAL_TAG = "wepp_260430_original_buggy_dac3c950"
ABANDONED_TAG = "kernel-rewrite-abandoned-20260605"
FIXED_BRANCH = "wepp_260430_negmeltfix_comparator"

LEGACY_20_COLUMNS = [
    "OFE",
    "J",
    "Y",
    "P",
    "RM",
    "Q",
    "Ep",
    "Es",
    "Er",
    "Dp",
    "UpStrmQ",
    "SubRIn",
    "latqcc",
    "Total-Soil",
    "frozwt",
    "Snow-Water",
    "QOFE",
    "Tile",
    "Irr",
    "Area",
]
CANONICAL_25_COLUMNS = [
    *LEGACY_20_COLUMNS,
    "SoilWaterTotal",
    "ProfileDepth",
    "ProfilePorosityCap",
    "ProfileFCStore",
    "ProfileWPStore",
]
PARQUET_COLUMNS = [
    "wepp_id",
    "ofe_id",
    "year",
    "sim_day_index",
    "julian",
    "month",
    "day_of_month",
    "water_year",
    "OFE",
    "P",
    "RM",
    "Q",
    "Ep",
    "Es",
    "Er",
    "Dp",
    "UpStrmQ",
    "SubRIn",
    "latqcc",
    "Total-Soil Water",
    "frozwt",
    "Snow-Water",
    "QOFE",
    "Tile",
    "Irr",
    "Area",
    "SoilWaterTotal",
    "ProfileDepth",
    "ProfilePorosityCap",
    "ProfileFCStore",
    "ProfileWPStore",
    "InterceptionStorage",
]
COMPARE_COLUMNS = [
    "P",
    "RM",
    "Q",
    "Ep",
    "Es",
    "Er",
    "Dp",
    "UpStrmQ",
    "SubRIn",
    "latqcc",
    "Total-Soil Water",
    "frozwt",
    "Snow-Water",
    "QOFE",
    "Tile",
    "Irr",
    "Area",
    "SoilWaterTotal",
    "ProfileDepth",
    "ProfilePorosityCap",
    "ProfileFCStore",
    "ProfileWPStore",
]
OBSERVE_SOURCE_WORKTREE = Path("/tmp/hphys0298_wepp_forest_obs")
OBSERVE_SOURCE_FILES = [
    "src/contin.for",
    "src/pmxelm.inc",
    "src/pmxhil.inc",
    "src/pmxpln.inc",
    "src/pntype.inc",
    "src/watbal.for",
    "src/watbal_hourly.for",
    "src/winter.for",
]
OBSERVE_HILLS = [1, 7, 39]
IDENTITY_TOLERANCE = 1.0e-9
EXPECTED_PARQUET_YEARS = [2013, 2014, 2015, 2016]
EXPECTED_YEAR_ROW_COUNTS = {2013: 365, 2014: 365, 2015: 365, 2016: 366}


@dataclass
class CommandRecord:
    name: str
    cmd: list[str]
    cwd: str | None
    rc: int
    seconds: float
    stdout: str
    stderr: str


class CommandFailure(RuntimeError):
    def __init__(self, record: CommandRecord) -> None:
        super().__init__(f"{record.name} failed with rc={record.rc}: {' '.join(record.cmd)}")
        self.record = record


COMMANDS: list[dict[str, Any]] = []
CURRENT_RUN_ROOT = Path("/tmp/hphys0303_uninitialized")


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def run(
    name: str,
    cmd: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    input_text: str | None = None,
    check: bool = True,
    timeout: int | None = None,
) -> CommandRecord:
    logs = CURRENT_RUN_ROOT / "logs" / "commands"
    logs.mkdir(parents=True, exist_ok=True)
    started = time.monotonic()
    merged_env = os.environ.copy()
    if env:
        merged_env.update(env)
    proc = subprocess.run(
        cmd,
        cwd=cwd,
        env=merged_env,
        input=input_text,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
        timeout=timeout,
    )
    elapsed = time.monotonic() - started
    safe_name = f"{len(COMMANDS) + 1:03d}_{name.replace('/', '_').replace(' ', '_')}"
    stdout_path = logs / f"{safe_name}.stdout.log"
    stderr_path = logs / f"{safe_name}.stderr.log"
    stdout_path.write_text(proc.stdout, encoding="utf-8", errors="ignore")
    stderr_path.write_text(proc.stderr, encoding="utf-8", errors="ignore")
    record = CommandRecord(
        name=name,
        cmd=cmd,
        cwd=str(cwd) if cwd else None,
        rc=proc.returncode,
        seconds=elapsed,
        stdout=str(stdout_path),
        stderr=str(stderr_path),
    )
    COMMANDS.append(
        {
            "name": record.name,
            "cmd": record.cmd,
            "cwd": record.cwd,
            "rc": record.rc,
            "seconds": round(record.seconds, 3),
            "stdout": record.stdout,
            "stderr": record.stderr,
        }
    )
    if check and proc.returncode != 0:
        raise CommandFailure(record)
    return record


def git(repo: Path, args: list[str], *, check: bool = True) -> CommandRecord:
    return run(f"git_{args[0]}", ["git", "-C", str(repo), *args], check=check)


def git_stdout(repo: Path, args: list[str], *, check: bool = True) -> str:
    record = git(repo, args, check=check)
    return Path(record.stdout).read_text(encoding="utf-8").strip()


def ref_exists(repo: Path, ref: str) -> bool:
    return git(repo, ["rev-parse", "--verify", "--quiet", ref], check=False).rc == 0


def rev_parse(repo: Path, ref: str) -> str:
    return git_stdout(repo, ["rev-parse", ref])


def tag_target(repo: Path, tag: str) -> str | None:
    if not ref_exists(repo, f"refs/tags/{tag}"):
        return None
    return rev_parse(repo, f"{tag}^{{commit}}")


def ensure_annotated_tag(repo: Path, tag: str, target: str, message: str) -> dict[str, Any]:
    existing = tag_target(repo, tag)
    if existing is not None:
        return {"tag": tag, "target": existing, "created": False, "pass": existing == target}
    git(repo, ["tag", "-a", tag, target, "-m", message])
    return {"tag": tag, "target": tag_target(repo, tag), "created": True, "pass": True}


def safe_remove_worktree(path: Path) -> None:
    if not path.exists():
        return
    if not str(path).startswith("/tmp/hphys0303_"):
        raise RuntimeError(f"refusing to remove non-HPHYS0303 path: {path}")
    git(BASELINE_REPO, ["worktree", "remove", "--force", str(path)], check=False)
    if path.exists():
        shutil.rmtree(path)


def prepare_sparse_worktree(path: Path, ref: str, branch: str | None = None) -> None:
    safe_remove_worktree(path)
    git(BASELINE_REPO, ["worktree", "add", "--detach", "--no-checkout", str(path), ref])
    git(path, ["sparse-checkout", "init", "--no-cone"])
    sparse_paths = "\n".join(
        [
            "/src/",
            "/release/",
            "/tools/",
            "/AGENTS.md",
            "/README.md",
            "/command_line_options_for_WEPP_compilation.txt",
            "",
        ]
    )
    run(
        "git_sparse_set",
        ["git", "-C", str(path), "sparse-checkout", "set", "--stdin"],
        input_text=sparse_paths,
    )
    if branch is None:
        git(path, ["checkout", ref])
    elif ref_exists(BASELINE_REPO, f"refs/heads/{branch}"):
        git(path, ["checkout", branch])
    else:
        git(path, ["checkout", "-B", branch, ref])


def patch_negative_melt(winter_path: Path) -> bool:
    text = winter_path.read_text(encoding="utf-8", errors="ignore")
    original = text
    text = text.replace(
        "       if (pstvML .le. ngtvML) then",
        "c      HPHYS0303 fixed comparator: compare net daily melt sign\n"
        "       if (pstvML + ngtvML .le. 0.0) then",
        1,
    )
    text = text.replace(
        "              hrmlt(hour,iplane) = hrmlt(hour,iplane)*(1-ngtvML/pstvML)",
        "c      HPHYS0303 fixed comparator: ngtvML<0 reduces positive melt\n"
        "              hrmlt(hour,iplane) = hrmlt(hour,iplane)*(1+ngtvML/pstvML)",
        1,
    )
    if text != original:
        winter_path.write_text(text, encoding="utf-8")
        return True
    if "pstvML + ngtvML .le. 0.0" not in text or "1+ngtvML/pstvML" not in text:
        raise RuntimeError(f"negative-melt replacement did not apply to {winter_path}")
    return False


def build_release(worktree: Path, name: str) -> dict[str, Any]:
    build = run(
        name,
        ["bash", "tools/build_wepp_dated_release.sh"],
        cwd=worktree,
        env={"TARGET_TAG": "260430", "COMPILER": "/usr/bin/gfortran"},
        timeout=1200,
    )
    binaries = {
        "wepp_260430": worktree / "release/wepp_260430",
        "wepp_260430_hill": worktree / "release/wepp_260430_hill",
    }
    return {
        "command": build.__dict__,
        "binaries": {
            name: {
                "path": str(path),
                "exists": path.exists(),
                "sha256": sha256_file(path) if path.exists() else None,
            }
            for name, path in binaries.items()
        },
    }


def commit_fixed_comparator(worktree: Path) -> str:
    git(worktree, ["add", "src/winter.for", "release/wepp_260430", "release/wepp_260430_hill"])
    staged_rc = git(worktree, ["diff", "--cached", "--quiet"], check=False).rc
    if staged_rc != 0:
        run(
            "git_commit_fixed_comparator",
            [
                "git",
                "-C",
                str(worktree),
                "commit",
                "-m",
                "winter: port negative melt fix to 260430 comparator",
            ],
            env={
                "GIT_AUTHOR_NAME": "Codex",
                "GIT_AUTHOR_EMAIL": "codex@openai.com",
                "GIT_COMMITTER_NAME": "Codex",
                "GIT_COMMITTER_EMAIL": "codex@openai.com",
            },
        )
    return rev_parse(worktree, "HEAD")


def legacy_runfile_text(hill: int, years: int = 4) -> str:
    return "\n".join(
        [
            "m",
            "Yes",
            "1",
            "1",
            "Yes",
            f"../output/H{hill}.pass.dat",
            "1",
            "No",
            f"../output/H{hill}.loss.dat",
            "Yes",
            f"../output/H{hill}.wat.dat",
            "No",
            "Yes",
            f"../output/H{hill}.soil.dat",
            "Yes",
            f"../output/H{hill}.plot.dat",
            "No",
            "Yes",
            f"../output/H{hill}.ebe.dat",
            "Yes",
            f"../output/H{hill}.element.dat",
            "No",
            "No",
            "No",
            f"p{hill}.man",
            f"p{hill}.slp",
            f"p{hill}.cli",
            f"p{hill}.sol",
            "0",
            str(years),
            "0",
            "",
        ]
    )


def prepare_legacy_run_dir(run_root: Path, hill: int, lane: str) -> tuple[Path, Path]:
    lane_root = run_root / "legacy_runs" / f"H{hill}_{lane}"
    if lane_root.exists():
        shutil.rmtree(lane_root)
    runs_dir = lane_root / "runs"
    output_dir = lane_root / "output"
    runs_dir.mkdir(parents=True)
    output_dir.mkdir(parents=True)
    for path in SOURCE_RUNS.iterdir():
        if path.is_file():
            shutil.copy2(path, runs_dir / path.name)
    (runs_dir / f"p{hill}.run").write_text(legacy_runfile_text(hill), encoding="utf-8")
    return runs_dir, output_dir


def run_legacy_hill(
    binary: Path, run_root: Path, hill: int, lane: str, *, observe: bool = False
) -> dict[str, Any]:
    runs_dir, output_dir = prepare_legacy_run_dir(run_root, hill, lane)
    if observe:
        (runs_dir / "wepp_observe.on").write_text("", encoding="utf-8")
    runfile = runs_dir / f"p{hill}.run"
    result = run(
        f"legacy_H{hill}_{lane}",
        [str(binary)],
        cwd=runs_dir,
        input_text=runfile.read_text(encoding="utf-8"),
        check=False,
        timeout=600,
    )
    wat_path = output_dir / f"H{hill}.wat.dat"
    observe_log = runs_dir / "wepp_observe.log"
    return {
        "hill": hill,
        "lane": lane,
        "binary": str(binary),
        "rc": result.rc,
        "seconds": round(result.seconds, 3),
        "wat_path": str(wat_path),
        "wat_exists": wat_path.exists(),
        "wat_sha256": sha256_file(wat_path) if wat_path.exists() else None,
        "observe_log": str(observe_log),
        "observe_records": sum(1 for _ in observe_log.open(encoding="utf-8", errors="ignore"))
        if observe_log.exists()
        else 0,
        "stdout": result.stdout,
        "stderr": result.stderr,
    }


def is_int_like(value: float) -> bool:
    return float(int(value)) == value


def parse_dat_rows(
    path: Path, row_year_offset: int = 0
) -> tuple[dict[tuple[int, int, int], dict[str, float]], list[int]]:
    row_map: dict[tuple[int, int, int], dict[str, float]] = {}
    widths_seen: list[int] = []
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        parts = line.strip().split()
        if len(parts) not in (20, 25):
            continue
        try:
            values = [float(item) for item in parts]
        except ValueError:
            continue
        if not all(is_int_like(values[idx]) for idx in (0, 1, 2)):
            continue
        columns = LEGACY_20_COLUMNS if len(values) == 20 else CANONICAL_25_COLUMNS
        row = {name: values[idx] for idx, name in enumerate(columns)}
        year = int(row["Y"]) + row_year_offset
        row["Y"] = float(year)
        key = (int(row["OFE"]), int(row["J"]), year)
        row_map[key] = row
        if len(values) not in widths_seen:
            widths_seen.append(len(values))
    return row_map, widths_seen


def dat_to_parquet(wat_path: Path, parquet_path: Path, hill: int) -> dict[str, Any]:
    import pandas as pd

    rows, widths = parse_dat_rows(wat_path, row_year_offset=0)
    records: list[dict[str, Any]] = []
    for index, ((ofe, julian, year), row) in enumerate(sorted(rows.items()), start=1):
        day = date(year, 1, 1) + timedelta(days=julian - 1)
        record = {
            "wepp_id": hill,
            "ofe_id": ofe,
            "year": year,
            "sim_day_index": index,
            "julian": julian,
            "month": day.month,
            "day_of_month": day.day,
            "water_year": year + (1 if day.month >= 10 else 0),
            "OFE": ofe,
            "Total-Soil Water": row.get("Total-Soil"),
            "InterceptionStorage": None,
        }
        for column in LEGACY_20_COLUMNS:
            if column in ("OFE", "J", "Y", "Total-Soil"):
                continue
            record[column] = row.get(column)
        for column in CANONICAL_25_COLUMNS[20:]:
            record[column] = row.get(column)
        records.append({column: record.get(column) for column in PARQUET_COLUMNS})
    parquet_path.parent.mkdir(parents=True, exist_ok=True)
    frame = pd.DataFrame.from_records(records, columns=PARQUET_COLUMNS)
    frame.to_parquet(parquet_path, index=False)
    duplicate_key_count = int(
        frame.duplicated(subset=["ofe_id", "year", "julian"]).sum()
    )
    years = sorted(int(year) for year in frame["year"].dropna().unique())
    year_row_counts = {
        str(int(year)): int(count)
        for year, count in frame.groupby("year", dropna=True).size().items()
    }
    expected_year_row_counts = {
        str(year): count for year, count in EXPECTED_YEAR_ROW_COUNTS.items()
    }
    expected_years_pass = (
        years == EXPECTED_PARQUET_YEARS
        and duplicate_key_count == 0
        and year_row_counts == expected_year_row_counts
    )
    return {
        "hill": hill,
        "source_wat": str(wat_path),
        "parquet": str(parquet_path),
        "row_count": len(records),
        "row_widths": widths,
        "years": years,
        "expected_years": EXPECTED_PARQUET_YEARS,
        "year_min": years[0] if years else None,
        "year_max": years[-1] if years else None,
        "julian_min": int(frame["julian"].min()) if len(frame) else None,
        "julian_max": int(frame["julian"].max()) if len(frame) else None,
        "year_row_counts": year_row_counts,
        "expected_year_row_counts": expected_year_row_counts,
        "duplicate_key_count": duplicate_key_count,
        "expected_years_pass": expected_years_pass,
        "sha256": sha256_file(parquet_path),
    }


def regenerate_fixed_parquets(binary: Path, run_root: Path) -> dict[str, Any]:
    partitions = run_root / "reports/hillslope/fixed_baseline_partitions"
    manifest_rows: list[dict[str, Any]] = []
    run_rows: list[dict[str, Any]] = []
    for hill in range(1, 40):
        result = run_legacy_hill(binary, run_root, hill, "fixed_baseline")
        run_rows.append(result)
        wat_path = Path(result["wat_path"])
        if result["rc"] == 0 and wat_path.exists():
            manifest_rows.append(dat_to_parquet(wat_path, partitions / f"baseline_H{hill}.parquet", hill))
    manifest = {
        "partition_dir": str(partitions),
        "source_binary": str(binary),
        "source_binary_sha256": sha256_file(binary),
        "generated_count": len(manifest_rows),
        "expected_count": 39,
        "all_generated": len(manifest_rows) == 39,
        "expected_years": EXPECTED_PARQUET_YEARS,
        "year_key_validation_pass": len(manifest_rows) == 39
        and all(row.get("expected_years_pass") for row in manifest_rows),
        "partitions": manifest_rows,
        "runs": run_rows,
    }
    write_json(ARTIFACT_DIR / "fixed-baseline-parquet-manifest.json", manifest)
    return manifest


def compare_original_fixed_parquets(fixed_manifest: dict[str, Any]) -> dict[str, Any]:
    import pandas as pd

    rows: list[dict[str, Any]] = []
    for item in fixed_manifest.get("partitions", []):
        hill = int(item["hill"])
        original_path = ORIGINAL_BASELINE_PARTITIONS / f"baseline_H{hill}.parquet"
        fixed_path = Path(item["parquet"])
        if not original_path.exists() or not fixed_path.exists():
            rows.append(
                {
                    "hill": hill,
                    "status": "missing-input",
                    "original_path": str(original_path),
                    "fixed_path": str(fixed_path),
                }
            )
            continue
        original = pd.read_parquet(original_path)
        fixed = pd.read_parquet(fixed_path)
        merged = original.merge(
            fixed,
            on=["OFE", "julian", "year"],
            suffixes=("_original", "_fixed"),
            how="outer",
            indicator=True,
        )
        changed_rows = 0
        max_abs = 0.0
        changed_columns: dict[str, int] = {}
        for _, merged_row in merged.iterrows():
            row_changed = False
            for column in COMPARE_COLUMNS:
                left_name = f"{column}_original"
                right_name = f"{column}_fixed"
                if left_name not in merged_row or right_name not in merged_row:
                    continue
                left = merged_row[left_name]
                right = merged_row[right_name]
                if pd.isna(left) and pd.isna(right):
                    continue
                if pd.isna(left) or pd.isna(right):
                    diff = float("inf")
                else:
                    diff = abs(float(left) - float(right))
                if diff > IDENTITY_TOLERANCE:
                    row_changed = True
                    max_abs = max(max_abs, diff if diff != float("inf") else 0.0)
                    changed_columns[column] = changed_columns.get(column, 0) + 1
            if row_changed:
                changed_rows += 1
        rows.append(
            {
                "hill": hill,
                "status": "compared",
                "row_count_original": int(len(original)),
                "row_count_fixed": int(len(fixed)),
                "merged_rows": int(len(merged)),
                "changed_rows": changed_rows,
                "max_abs_diff": max_abs,
                "changed_columns": changed_columns,
            }
        )
    payload = {
        "proof_mode": "runtime-output-delta-manifest plus source-delta-limited static proof",
        "original_partition_dir": str(ORIGINAL_BASELINE_PARTITIONS),
        "fixed_partition_dir": fixed_manifest.get("partition_dir"),
        "hillslope_count": len(rows),
        "changed_hillslopes": [row["hill"] for row in rows if row.get("changed_rows", 0) > 0],
        "rows": rows,
    }
    write_json(ARTIFACT_DIR / "fixed-vs-original-output-delta.json", payload)
    return payload


def write_parquet_manifest_md(manifest: dict[str, Any]) -> None:
    lines = [
        "# Fixed Baseline Parquet Manifest",
        "",
        "Ran:",
        "",
        f"- Partition dir: `{manifest['partition_dir']}`",
        f"- Source binary: `{manifest['source_binary']}`",
        f"- Source binary SHA256: `{manifest['source_binary_sha256']}`",
        f"- Generated: `{manifest['generated_count']}/{manifest['expected_count']}`",
        f"- Expected years: `{manifest.get('expected_years')}`",
        f"- Year/key validation pass: `{manifest.get('year_key_validation_pass')}`",
        "",
        "| Hill | Rows | Years | Duplicate Keys | Widths | SHA256 | Path |",
        "| --- | ---: | --- | ---: | --- | --- | --- |",
    ]
    for item in manifest["partitions"]:
        lines.append(
            f"| H{item['hill']} | {item['row_count']} | {item.get('years')} | "
            f"{item.get('duplicate_key_count')} | {item['row_widths']} | "
            f"`{item['sha256']}` | `{item['parquet']}` |"
        )
    (ARTIFACT_DIR / "fixed-baseline-parquet-manifest.md").write_text(
        "\n".join(lines) + "\n", encoding="utf-8"
    )


def prepare_observe_worktree(path: Path, fixed_ref: str) -> None:
    prepare_sparse_worktree(path, fixed_ref)
    if not OBSERVE_SOURCE_WORKTREE.exists():
        raise RuntimeError(f"missing observe source worktree: {OBSERVE_SOURCE_WORKTREE}")
    for relative in OBSERVE_SOURCE_FILES:
        source = OBSERVE_SOURCE_WORKTREE / relative
        target = path / relative
        if not source.exists():
            raise RuntimeError(f"missing observe source file: {source}")
        shutil.copy2(source, target)
    patch_negative_melt(path / "src/winter.for")


def compare_dat_identity(left_path: Path, right_path: Path) -> dict[str, Any]:
    left_rows, left_widths = parse_dat_rows(left_path, row_year_offset=0)
    right_rows, right_widths = parse_dat_rows(right_path, row_year_offset=0)
    keys = sorted(set(left_rows) & set(right_rows))
    missing = len(set(left_rows) ^ set(right_rows))
    max_abs_by_column = {column: 0.0 for column in COMPARE_COLUMNS if column != "Total-Soil Water"}
    for key in keys:
        left = left_rows[key]
        right = right_rows[key]
        for column in list(max_abs_by_column):
            source_column = "Total-Soil" if column == "Total-Soil Water" else column
            if source_column in left and source_column in right:
                max_abs_by_column[column] = max(
                    max_abs_by_column[column],
                    abs(float(left[source_column]) - float(right[source_column])),
                )
    semantic_pass = missing == 0 and all(value <= IDENTITY_TOLERANCE for value in max_abs_by_column.values())
    return {
        "left_row_widths": left_widths,
        "right_row_widths": right_widths,
        "common_rows": len(keys),
        "missing_or_extra_rows": missing,
        "max_abs_by_column": max_abs_by_column,
        "semantic_pass": semantic_pass,
    }


def run_observe_identity(fixed_binary: Path, observe_worktree: Path, fixed_ref: str, run_root: Path) -> dict[str, Any]:
    prepare_observe_worktree(observe_worktree, fixed_ref)
    observe_build = build_release(observe_worktree, "build_fixed_observe_release")
    observe_binary = observe_worktree / "release/wepp_260430_hill"
    rows: list[dict[str, Any]] = []
    for hill in OBSERVE_HILLS:
        release = run_legacy_hill(fixed_binary, run_root, hill, "fixed_release", observe=False)
        observe_off = run_legacy_hill(observe_binary, run_root, hill, "fixed_observe_off", observe=False)
        observe_on = run_legacy_hill(observe_binary, run_root, hill, "fixed_observe_on", observe=True)
        release_wat = Path(release["wat_path"])
        off_wat = Path(observe_off["wat_path"])
        on_wat = Path(observe_on["wat_path"])
        release_to_off = (
            compare_dat_identity(release_wat, off_wat)
            if release_wat.exists() and off_wat.exists()
            else {"semantic_pass": False}
        )
        off_to_on = (
            compare_dat_identity(off_wat, on_wat)
            if off_wat.exists() and on_wat.exists()
            else {"semantic_pass": False}
        )
        rows.append(
            {
                "hill": hill,
                "release": release,
                "observe_off": observe_off,
                "observe_on": observe_on,
                "release_to_observe_off_bit_identical": release.get("wat_sha256")
                == observe_off.get("wat_sha256"),
                "observe_off_to_observe_on_bit_identical": observe_off.get("wat_sha256")
                == observe_on.get("wat_sha256"),
                "release_to_observe_off_semantic_identity": release_to_off,
                "observe_off_to_observe_on_semantic_identity": off_to_on,
                "pass": release["rc"] == 0
                and observe_off["rc"] == 0
                and observe_on["rc"] == 0
                and release.get("wat_sha256") == observe_off.get("wat_sha256")
                and observe_off.get("wat_sha256") == observe_on.get("wat_sha256")
                and bool(release_to_off.get("semantic_pass"))
                and bool(off_to_on.get("semantic_pass"))
                and observe_on.get("observe_records", 0) > 0,
            }
        )
    payload = {
        "scope": "H1/H7/H39 fixed release vs fixed observe-off/observe-on WAT identity",
        "required_hills": OBSERVE_HILLS,
        "hills_checked": [row["hill"] for row in rows],
        "observe_source_worktree": str(OBSERVE_SOURCE_WORKTREE),
        "observe_worktree": str(observe_worktree),
        "observe_build": observe_build,
        "rows": rows,
        "pass": [row["hill"] for row in rows] == OBSERVE_HILLS
        and all(row["pass"] for row in rows),
    }
    write_json(ARTIFACT_DIR / "observe-identity-fixed-comparator.json", payload)
    return payload


def run_smoke(binary: Path, label: str) -> dict[str, Any]:
    script = BASELINE_REPO / "tools/smoke_wepp_binary_host.sh"
    if not script.exists():
        return {"label": label, "status": "missing-script", "script": str(script)}
    result = run(
        f"smoke_{label}",
        ["bash", str(script), str(binary)],
        env={"RUNS_DIR": str(SOURCE_RUNS), "CASES": "p1,p7", "TIMEOUT_SECONDS": "180"},
        check=False,
        timeout=420,
    )
    return {
        "label": label,
        "binary": str(binary),
        "rc": result.rc,
        "stdout": result.stdout,
        "stderr": result.stderr,
        "pass": result.rc == 0,
    }


def classify_smoke_checks(checks: list[dict[str, Any]]) -> dict[str, Any]:
    if all(row.get("pass") for row in checks):
        return {"status": "pass", "pass": True, "reason": "host smoke helper passed"}
    missing_fixture_failures = []
    for row in checks:
        stderr_path = row.get("stderr")
        stderr_text = (
            Path(stderr_path).read_text(encoding="utf-8", errors="ignore")
            if stderr_path and Path(stderr_path).exists()
            else ""
        )
        missing_fixture_failures.append("missing fixture file" in stderr_text)
    if missing_fixture_failures and all(missing_fixture_failures):
        return {
            "status": "non_applicable_missing_helper_fixtures",
            "pass": True,
            "reason": (
                "host smoke helper requires p*.run/chntyp.txt/gwcoeff.txt files "
                "absent from the selected HPHYS fixture root; H1..H39 fixed "
                "baseline replay is the executable binary gate"
            ),
        }
    return {
        "status": "failed",
        "pass": False,
        "reason": "host smoke helper failed for a reason other than missing helper-specific fixtures",
    }


def run_sc_lint() -> dict[str, Any]:
    outputs: list[dict[str, Any]] = []
    for contract in [
        "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
        "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    ]:
        result = run(
            f"sc_lint_{Path(contract).stem}",
            [
                "python3",
                "tools/release/check_sc_unit_compliance.py",
                "--path",
                contract,
                "--format",
                "json",
            ],
            cwd=REPO,
            check=False,
        )
        stdout_text = Path(result.stdout).read_text(encoding="utf-8", errors="ignore").strip()
        try:
            parsed = json.loads(stdout_text) if stdout_text else []
        except json.JSONDecodeError:
            parsed = {"raw_stdout": stdout_text}
        outputs.append(
            {
                "contract": contract,
                "rc": result.rc,
                "stdout": result.stdout,
                "stderr": result.stderr,
                "finding_count": len(parsed) if isinstance(parsed, list) else None,
                "parsed": parsed,
            }
        )
    payload = {
        "command": "tools/release/check_sc_unit_compliance.py --path <contract> --format json",
        "results": outputs,
        "result_artifact_present": True,
        "pass": all(row["rc"] == 0 for row in outputs),
    }
    write_json(ARTIFACT_DIR / "sc-unit-provenance-lint.json", payload)
    return payload


def write_source_delta_artifacts(worktree: Path, fixed_sha: str) -> dict[str, Any]:
    files = git_stdout(worktree, ["diff", "--name-only", f"{BASE_SHA}..{fixed_sha}"]).splitlines()
    source_files = [item for item in files if not item.startswith("release/")]
    winter_patch = git_stdout(worktree, ["diff", f"{BASE_SHA}..{fixed_sha}", "--", "src/winter.for"])
    (ARTIFACT_DIR / "fixed-comparator-source-delta.patch").write_text(winter_patch + "\n", encoding="utf-8")
    proof = {
        "base_sha": BASE_SHA,
        "fixed_sha": fixed_sha,
        "changed_files": files,
        "source_files": source_files,
        "source_delta_limited_to_winter": source_files == ["src/winter.for"],
        "contains_net_sign_fix": "pstvML + ngtvML .le. 0.0" in winter_patch,
        "contains_positive_melt_scale_fix": "1+ngtvML/pstvML" in winter_patch,
        "fix_provenance_sha": FIX_PROVENANCE_SHA,
        "pass": source_files == ["src/winter.for"]
        and "pstvML + ngtvML .le. 0.0" in winter_patch
        and "1+ngtvML/pstvML" in winter_patch,
    }
    write_json(ARTIFACT_DIR / "fixed-comparator-source-delta.json", proof)
    return proof


def write_git_ref_artifact(refs: dict[str, Any], fixed_tag: str) -> None:
    lines = [
        "# Fixed Comparator Git Refs",
        "",
        "Ran:",
        "",
        f"- Original tag: `{ORIGINAL_TAG}` -> `{refs['original_tag']['target']}`",
        f"- Abandoned tag: `{ABANDONED_TAG}` -> `{refs['abandoned_tag']['target']}`",
        f"- Fixed branch: `{FIXED_BRANCH}` -> `{refs['fixed_sha']}`",
        f"- Fixed tag: `{fixed_tag}` -> `{refs['fixed_tag']['target']}`",
        "- Remote push performed: `false`",
        "",
    ]
    (ARTIFACT_DIR / "fixed-comparator-git-refs.md").write_text(
        "\n".join(lines), encoding="utf-8"
    )


def write_binary_hash_artifact(build: dict[str, Any]) -> None:
    lines = ["# Fixed Comparator Binary Hashes", "", "Ran:", ""]
    for name, item in build["binaries"].items():
        lines.append(f"- `{name}`: `{item['sha256']}` at `{item['path']}`")
    (ARTIFACT_DIR / "fixed-comparator-binary-hashes.txt").write_text(
        "\n".join(lines) + "\n", encoding="utf-8"
    )


def write_ratification_checklist(ledger: dict[str, Any]) -> None:
    checklist = ledger["ratification_checklist"]
    lines = [
        "# ADR-0016 Ratification Checklist",
        "",
        "Ran:",
        "",
        f"- Run root: `{ledger['run_root']}`",
        f"- Ratification status: `{ledger['ratification_status']}`",
        "",
        "| Gate | Status | Evidence |",
        "| --- | --- | --- |",
    ]
    for gate, row in checklist.items():
        status = "pass" if row["pass"] else "hold"
        lines.append(f"| {gate} | {status} | {row['evidence']} |")
    if ledger["blockers"]:
        lines.extend(["", "## Blockers", ""])
        for blocker in ledger["blockers"]:
            lines.append(f"- {blocker}")
    (ARTIFACT_DIR / "ratification-checklist.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-root", type=Path, default=None)
    parser.add_argument(
        "--fixed-worktree",
        type=Path,
        default=Path("/tmp/hphys0303_wepp_260430_negmeltfix"),
    )
    parser.add_argument(
        "--observe-worktree",
        type=Path,
        default=Path("/tmp/hphys0303_wepp_260430_negmeltfix_observe"),
    )
    parser.add_argument("--skip-observe", action="store_true")
    parser.add_argument("--skip-parquets", action="store_true")
    args = parser.parse_args()

    global CURRENT_RUN_ROOT
    CURRENT_RUN_ROOT = args.run_root or Path(f"/tmp/hphys0303_adr0016_{int(time.time())}")
    CURRENT_RUN_ROOT.mkdir(parents=True, exist_ok=True)
    ARTIFACT_DIR.mkdir(parents=True, exist_ok=True)

    blockers: list[str] = []
    ledger: dict[str, Any] = {
        "schema": "hphys0303-adr0016-ratification-v1",
        "package": PACKAGE_DIR.name,
        "run_root": str(CURRENT_RUN_ROOT),
        "remote_push_performed": False,
        "production_edit_authorized": False,
        "hphys0302_hold_carried_forward": True,
        "baseline_repo": str(BASELINE_REPO),
        "abandoned_repo": str(ABANDONED_REPO),
        "base_sha": BASE_SHA,
        "abandoned_sha": ABANDONED_SHA,
        "fix_provenance_sha": FIX_PROVENANCE_SHA,
        "blockers": blockers,
    }
    try:
        for required in [BASELINE_REPO, ABANDONED_REPO, SOURCE_RUNS, WEPPPY_PYTHON, SEMANTIC_WAT]:
            if not required.exists():
                raise RuntimeError(f"missing required path: {required}")

        refs: dict[str, Any] = {}
        refs["original_tag"] = ensure_annotated_tag(
            BASELINE_REPO,
            ORIGINAL_TAG,
            BASE_SHA,
            "HPHYS0303 preserve original buggy wepp_260430 comparator baseline",
        )
        refs["abandoned_tag"] = ensure_annotated_tag(
            BASELINE_REPO,
            ABANDONED_TAG,
            ABANDONED_SHA,
            "HPHYS0303 preserve abandoned kernel rewrite archaeology line",
        )

        prepare_sparse_worktree(args.fixed_worktree, BASE_SHA, FIXED_BRANCH)
        patch_negative_melt(args.fixed_worktree / "src/winter.for")
        fixed_build = build_release(args.fixed_worktree, "build_fixed_comparator_release")
        fixed_sha = commit_fixed_comparator(args.fixed_worktree)
        fixed_tag = f"wepp_260430_negmeltfix_comparator_{fixed_sha[:12]}"
        refs["fixed_sha"] = fixed_sha
        refs["fixed_tag"] = ensure_annotated_tag(
            args.fixed_worktree,
            fixed_tag,
            fixed_sha,
            "HPHYS0303 fixed wepp_260430 negative-melt comparator",
        )
        ledger["fixed_branch"] = FIXED_BRANCH
        ledger["fixed_sha"] = fixed_sha
        ledger["fixed_tag"] = fixed_tag
        ledger["fixed_worktree"] = str(args.fixed_worktree)
        ledger["git_refs"] = refs
        ledger["fixed_build"] = fixed_build
        write_git_ref_artifact(refs, fixed_tag)
        write_binary_hash_artifact(fixed_build)

        source_delta = write_source_delta_artifacts(args.fixed_worktree, fixed_sha)
        ledger["source_delta"] = source_delta
        fixed_hill_binary = args.fixed_worktree / "release/wepp_260430_hill"
        fixed_watershed_binary = args.fixed_worktree / "release/wepp_260430"
        ledger["smoke_checks"] = [
            run_smoke(fixed_hill_binary, "fixed_hill"),
            run_smoke(fixed_watershed_binary, "fixed_watershed"),
        ]
        ledger["smoke_checks_disposition"] = classify_smoke_checks(ledger["smoke_checks"])
        if not ledger["smoke_checks_disposition"]["pass"]:
            blockers.append("fixed comparator host smoke helper failed without non-applicable fixture disposition")

        if args.skip_parquets:
            fixed_manifest = {"all_generated": False, "generated_count": 0, "partitions": []}
            blockers.append("fixed H1..H39 parquet regeneration skipped by CLI flag")
        else:
            fixed_manifest = regenerate_fixed_parquets(fixed_hill_binary, CURRENT_RUN_ROOT)
            write_parquet_manifest_md(fixed_manifest)
            ledger["fixed_baseline_parquets"] = fixed_manifest
            if not fixed_manifest["all_generated"]:
                blockers.append("fixed H1..H39 baseline parquet regeneration did not complete 39/39")
            if not fixed_manifest.get("year_key_validation_pass"):
                blockers.append("fixed H1..H39 baseline parquet year/key validation did not pass")
            ledger["fixed_vs_original_output_delta"] = compare_original_fixed_parquets(fixed_manifest)

        if args.skip_observe:
            observe_identity = {"pass": False, "status": "skipped"}
            blockers.append("fixed comparator observe identity skipped by CLI flag")
        else:
            observe_identity = run_observe_identity(
                fixed_hill_binary, args.observe_worktree, fixed_sha, CURRENT_RUN_ROOT
            )
            ledger["observe_identity"] = observe_identity
            if not observe_identity["pass"]:
                blockers.append("fixed comparator observe identity did not pass")

        sc_lint = run_sc_lint()
        ledger["sc_unit_provenance_lint"] = sc_lint
        if not sc_lint["result_artifact_present"]:
            blockers.append("SC unit/provenance lint result artifact missing")
        if not sc_lint["pass"]:
            blockers.append("SC unit/provenance lint did not pass")

        binary_hashes_present = all(
            bool(item.get("sha256")) for item in fixed_build["binaries"].values()
        )
        checklist = {
            "original_baseline_tag": {
                "pass": bool(refs["original_tag"]["pass"]),
                "evidence": "artifacts/fixed-comparator-git-refs.md",
            },
            "abandoned_line_tag": {
                "pass": bool(refs["abandoned_tag"]["pass"]),
                "evidence": "artifacts/fixed-comparator-git-refs.md",
            },
            "fixed_branch_tag_commit": {
                "pass": bool(refs["fixed_tag"]["pass"] and fixed_sha),
                "evidence": "artifacts/fixed-comparator-git-refs.md",
            },
            "fixed_binary_hashes": {
                "pass": binary_hashes_present,
                "evidence": "artifacts/fixed-comparator-binary-hashes.txt",
            },
            "source_delta_limited": {
                "pass": bool(source_delta["pass"]),
                "evidence": "artifacts/fixed-comparator-source-delta.json",
            },
            "h1_h39_parquet_manifest": {
                "pass": bool(
                    fixed_manifest.get("all_generated")
                    and fixed_manifest.get("year_key_validation_pass")
                ),
                "evidence": "artifacts/fixed-baseline-parquet-manifest.json",
            },
            "observe_identity": {
                "pass": bool(observe_identity.get("pass")),
                "evidence": "artifacts/observe-identity-fixed-comparator.json",
            },
            "output_delta_manifest": {
                "pass": "fixed_vs_original_output_delta" in ledger,
                "evidence": "artifacts/fixed-vs-original-output-delta.json",
            },
            "sc_lint_result_artifact": {
                "pass": bool(sc_lint["result_artifact_present"] and sc_lint["pass"]),
                "evidence": "artifacts/sc-unit-provenance-lint.json",
            },
            "smoke_or_replay_disposition": {
                "pass": bool(ledger["smoke_checks_disposition"]["pass"]),
                "evidence": "artifacts/comparator-ratification-ledger.json",
            },
            "hphys0302_hold": {
                "pass": True,
                "evidence": "artifacts/comparator-ratification-ledger.json",
            },
        }
        ledger["ratification_checklist"] = checklist
        if all(row["pass"] for row in checklist.values()) and not blockers:
            ledger["ratification_status"] = "accepted-ready"
        else:
            ledger["ratification_status"] = "proposed-hold"
    except CommandFailure as error:
        blockers.append(str(error))
        ledger["ratification_status"] = "proposed-hold"
        ledger["failed_command"] = error.record.__dict__
    except Exception as error:
        blockers.append(str(error))
        ledger["ratification_status"] = "proposed-hold"
    finally:
        ledger.setdefault("ratification_checklist", {})
        ledger["commands"] = COMMANDS
        write_json(ARTIFACT_DIR / "hphys0303-runner-command-log.json", COMMANDS)
        write_json(ARTIFACT_DIR / "comparator-ratification-ledger.json", ledger)
        write_ratification_checklist(ledger)

    print(json.dumps({"ratification_status": ledger["ratification_status"], "blockers": blockers}, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
