#!/usr/bin/env python3
"""Summarize selected-cohort active plain-vs-hybrid output evidence."""

from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
from typing import Any

import pyarrow.parquet as pq

PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE_DIR / "artifacts"
MATERIALIZATION = ARTIFACTS / "selected-cohort-materialization.json"
SUMMARY_JSON = ARTIFACTS / "active-suite-summary.json"
SUMMARY_MD = ARTIFACTS / "active-suite-summary.md"


def sha256(path: Path) -> str | None:
    if not path.is_file():
        return None
    digest = hashlib.sha256()
    with path.open("rb") as fp:
        for chunk in iter(lambda: fp.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_manifest(output_dir: Path) -> dict[str, Any] | None:
    path = output_dir / "openwepp_hillslope_run_manifest.json"
    if not path.is_file():
        return None
    return json.loads(path.read_text())


def relative_delta(plain: float | None, hybrid: float | None) -> float | None:
    if plain is None or hybrid is None or not math.isfinite(plain) or plain == 0.0:
        return None
    return (hybrid - plain) / plain


def pass_summary(path: Path | None) -> dict[str, Any] | None:
    if path is None or not path.is_file():
        return None
    table = pq.read_table(path)
    names = set(table.column_names)
    result: dict[str, Any] = {
        "rows": table.num_rows,
        "columns": table.column_names,
    }
    for column in ["tdet", "tdep", "sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4", "sedcon_5"]:
        if column in names:
            arr = table[column].combine_chunks()
            total = 0.0
            nonzero = 0
            for i in range(len(arr)):
                value = arr[i].as_py()
                if value is None:
                    continue
                numeric = float(value)
                total += numeric
                if numeric != 0.0:
                    nonzero += 1
            result[f"{column}_sum"] = total
            result[f"{column}_nonzero"] = nonzero
    return result


def output_file_from_manifest(manifest: dict[str, Any], suffix: str) -> Path | None:
    checksums = manifest.get("output_checksums", {})
    for raw_path in checksums:
        if raw_path.endswith(suffix):
            return Path(raw_path)
    return None


def member_summary(member: dict[str, Any]) -> dict[str, Any]:
    run_dir = Path(member["run_dir"])
    plain_dir = run_dir / "output-plain"
    hybrid_dir = run_dir / "output-hybrid"
    plain_manifest = read_manifest(plain_dir)
    hybrid_manifest = read_manifest(hybrid_dir)
    out: dict[str, Any] = {
        "member_id": member["member_id"],
        "wepp_id": member["wepp_id"],
        "plain_manifest_exists": plain_manifest is not None,
        "hybrid_manifest_exists": hybrid_manifest is not None,
    }
    if plain_manifest is None or hybrid_manifest is None:
        return out

    plain_laned = plain_manifest.get("execution_provenance", {}).get("laned_active", {})
    hybrid_laned = hybrid_manifest.get("execution_provenance", {}).get("laned_active", {})
    out["plain_laned_active"] = plain_laned
    out["hybrid_laned_active"] = hybrid_laned

    metric_deltas = {}
    for key in [
        "total_source_m3",
        "total_routed_outlet_m3",
        "total_end_window_storage_m3",
        "total_clamp_m3",
        "total_tail_fold_m3",
        "total_latqcc_outlet_m3",
        "max_supply_reconstruction_rel",
        "max_day_cascade_residual_rel",
        "max_day_seam_residual_rel",
        "max_day_identity_residual_rel",
    ]:
        plain = plain_laned.get(key)
        hybrid = hybrid_laned.get(key)
        if isinstance(plain, (int, float)) and isinstance(hybrid, (int, float)):
            metric_deltas[key] = {
                "plain": plain,
                "hybrid": hybrid,
                "delta": hybrid - plain,
                "relative_delta": relative_delta(float(plain), float(hybrid)),
            }
    out["laned_active_deltas"] = metric_deltas

    plain_hbp = output_file_from_manifest(plain_manifest, ".hbp")
    hybrid_hbp = output_file_from_manifest(hybrid_manifest, ".hbp")
    plain_pass = output_file_from_manifest(plain_manifest, ".pass.parquet")
    hybrid_pass = output_file_from_manifest(hybrid_manifest, ".pass.parquet")
    out["output_hashes"] = {
        "plain_hbp": sha256(plain_hbp) if plain_hbp else None,
        "hybrid_hbp": sha256(hybrid_hbp) if hybrid_hbp else None,
        "plain_pass_parquet": sha256(plain_pass) if plain_pass else None,
        "hybrid_pass_parquet": sha256(hybrid_pass) if hybrid_pass else None,
    }
    plain_pass_summary = pass_summary(plain_pass)
    hybrid_pass_summary = pass_summary(hybrid_pass)
    out["plain_pass_summary"] = plain_pass_summary
    out["hybrid_pass_summary"] = hybrid_pass_summary
    pass_deltas = {}
    if plain_pass_summary and hybrid_pass_summary:
        for key, plain in plain_pass_summary.items():
            if not key.endswith("_sum") or key not in hybrid_pass_summary:
                continue
            hybrid = hybrid_pass_summary[key]
            pass_deltas[key] = {
                "plain": plain,
                "hybrid": hybrid,
                "delta": hybrid - plain,
                "relative_delta": relative_delta(float(plain), float(hybrid)),
            }
    out["pass_deltas"] = pass_deltas
    return out


def format_pct(value: float | None) -> str:
    if value is None:
        return "n/a"
    return f"{value * 100:.6g}%"


def hash_equal(hashes: dict[str, Any], left: str, right: str) -> str:
    left_value = hashes.get(left)
    right_value = hashes.get(right)
    if left_value is None or right_value is None:
        return "n/a"
    return str(left_value == right_value)


def write_markdown(summary: list[dict[str, Any]]) -> None:
    complete = all(
        item.get("plain_manifest_exists") and item.get("hybrid_manifest_exists")
        for item in summary
    )
    lines = [
        "# Active Suite Summary",
        "",
        "Status: {status}. Evidence mode: Ran.".format(
            status="EXECUTED" if complete else "EXECUTED-HOLD-ACTIVE-RUN"
        ),
        "",
        "| Member | Plain manifest | Hybrid manifest | Outlet delta | HBP hash equal | Pass hash equal |",
        "|---|---:|---:|---:|---:|---:|",
    ]
    for item in summary:
        deltas = item.get("laned_active_deltas", {})
        outlet = deltas.get("total_routed_outlet_m3", {})
        hashes = item.get("output_hashes", {})
        lines.append(
            "| {member} | {plain} | {hybrid} | {delta} | {hbp} | {pass_hash} |".format(
                member=item["member_id"],
                plain="yes" if item.get("plain_manifest_exists") else "no",
                hybrid="yes" if item.get("hybrid_manifest_exists") else "no",
                delta=format_pct(outlet.get("relative_delta")),
                hbp=hash_equal(hashes, "plain_hbp", "hybrid_hbp"),
                pass_hash=hash_equal(
                    hashes, "plain_pass_parquet", "hybrid_pass_parquet"
                ),
            )
        )
    lines.extend(
        [
            "",
            "Detailed JSON:",
            "",
            "- `artifacts/active-suite-summary.json`",
            "",
        ]
    )
    SUMMARY_MD.write_text("\n".join(lines))


def main() -> None:
    members = json.loads(MATERIALIZATION.read_text())
    summary = [member_summary(member) for member in members]
    SUMMARY_JSON.write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    write_markdown(summary)
    print(json.dumps({"summary_json": str(SUMMARY_JSON), "members": len(summary)}))


if __name__ == "__main__":
    main()
