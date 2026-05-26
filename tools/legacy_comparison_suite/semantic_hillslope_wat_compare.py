#!/usr/bin/env python3
"""Semantic comparator for legacy/openWEPP hillslope water-balance outputs.

Input support:
- legacy/openWEPP ASCII `.dat` rows (20-column or 25-column variants),
- openWEPP parquet candidate rows (`.parquet`) with WB13/CLI04-aligned fields.

Output:
- JSON report with structural deltas, per-column statistics, tolerance verdicts,
  and top divergent row keys for investigation.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Mapping, Tuple

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

PARQUET_TO_CANONICAL = {
    "OFE": "OFE",
    "julian": "J",
    "year": "Y",
    "P": "P",
    "RM": "RM",
    "Q": "Q",
    "Ep": "Ep",
    "Es": "Es",
    "Er": "Er",
    "Dp": "Dp",
    "UpStrmQ": "UpStrmQ",
    "SubRIn": "SubRIn",
    "latqcc": "latqcc",
    "Total-Soil": "Total-Soil",
    "Total-Soil Water": "Total-Soil",
    "frozwt": "frozwt",
    "Snow-Water": "Snow-Water",
    "QOFE": "QOFE",
    "Tile": "Tile",
    "Irr": "Irr",
    "Area": "Area",
    "SoilWaterTotal": "SoilWaterTotal",
    "ProfileDepth": "ProfileDepth",
    "ProfilePorosityCap": "ProfilePorosityCap",
    "ProfileFCStore": "ProfileFCStore",
    "ProfileWPStore": "ProfileWPStore",
}

DEFAULT_INVESTIGATION_COLUMNS = [
    "P",
    "Q",
    "Ep",
    "Es",
    "Er",
    "Dp",
    "Total-Soil",
    "frozwt",
    "Snow-Water",
    "SoilWaterTotal",
]

KEY_FIELDS = ("OFE", "J", "Y")
REPORT_SCHEMA_VERSION = "pl14s-semantic-wat-v2"


@dataclass
class Tolerance:
    abs_tol: float
    rel_tol: float


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def is_int_like(value: float) -> bool:
    return float(int(value)) == value


def parse_dat_rows(
    path: Path, row_year_offset: int = 0
) -> tuple[Dict[Tuple[int, int, int], Dict[str, float]], List[int]]:
    row_map: Dict[Tuple[int, int, int], Dict[str, float]] = {}
    widths_seen: List[int] = []

    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        parts = line.strip().split()
        if not parts:
            continue
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
        key_year = int(row["Y"]) + row_year_offset
        row["Y"] = float(key_year)
        key = (int(row["OFE"]), int(row["J"]), key_year)
        if key in row_map:
            raise RuntimeError(f"duplicate row key {key} in dat input {path}")
        row_map[key] = row
        if len(values) not in widths_seen:
            widths_seen.append(len(values))

    widths_seen.sort()
    return row_map, widths_seen


def parse_parquet_rows(
    path: Path,
    candidate_partition_value: int | None,
    candidate_partition_column: str,
    candidate_year_offset: int,
) -> tuple[
    Dict[Tuple[int, int, int], Dict[str, float]], List[int], Dict[str, List[str]]
]:
    try:
        import pyarrow.parquet as pq
    except Exception as exc:  # pragma: no cover - environment dependent
        raise RuntimeError(
            "parquet input requires pyarrow; install pyarrow or pass .dat candidate"
        ) from exc

    table = pq.read_table(path)
    column_names = table.column_names
    arrays = {name: table[name].to_pylist() for name in column_names}

    if candidate_partition_value is not None and candidate_partition_column not in arrays:
        raise RuntimeError(
            "candidate partition column "
            f"{candidate_partition_column!r} not present in parquet input {path}"
        )

    row_map: Dict[Tuple[int, int, int], Dict[str, float]] = {}
    widths_seen: List[int] = []
    alias_sources: Dict[str, set[str]] = {}
    row_count = table.num_rows

    for idx in range(row_count):
        if candidate_partition_value is not None:
            raw_partition = arrays[candidate_partition_column][idx]
            if raw_partition is None:
                continue
            try:
                partition_value = int(raw_partition)
            except (TypeError, ValueError):
                continue
            if partition_value != candidate_partition_value:
                continue

        row: Dict[str, float] = {}

        for src, dst in PARQUET_TO_CANONICAL.items():
            if src not in arrays:
                continue
            value = arrays[src][idx]
            if value is None:
                continue
            row[dst] = float(value)
            alias_sources.setdefault(dst, set()).add(src)

        if "OFE" not in row and "ofe_id" in arrays and arrays["ofe_id"][idx] is not None:
            row["OFE"] = float(arrays["ofe_id"][idx])
        if "Y" not in row and "year" in arrays and arrays["year"][idx] is not None:
            row["Y"] = float(arrays["year"][idx])
        if "J" not in row and "julian" in arrays and arrays["julian"][idx] is not None:
            row["J"] = float(arrays["julian"][idx])

        missing = [field for field in KEY_FIELDS if field not in row]
        if missing:
            continue

        key_year = int(row["Y"]) + candidate_year_offset
        row["Y"] = float(key_year)
        key = (int(row["OFE"]), int(row["J"]), key_year)
        if key in row_map:
            raise RuntimeError(f"duplicate row key {key} in parquet input {path}")
        row_map[key] = row
        width = len(row)
        if width not in widths_seen:
            widths_seen.append(width)

    widths_seen.sort()
    alias_sources_sorted = {
        canonical: sorted(sources) for canonical, sources in sorted(alias_sources.items())
    }
    return row_map, widths_seen, alias_sources_sorted


def load_rows(
    path: Path,
    candidate_partition_value: int | None,
    candidate_partition_column: str,
    candidate_year_offset: int,
) -> tuple[
    Dict[Tuple[int, int, int], Dict[str, float]], List[int], str, Dict[str, List[str]]
]:
    suffix = path.suffix.lower()
    if suffix == ".parquet":
        row_map, widths, alias_sources = parse_parquet_rows(
            path,
            candidate_partition_value,
            candidate_partition_column,
            candidate_year_offset,
        )
        return row_map, widths, "parquet", alias_sources

    if candidate_partition_value is not None:
        raise RuntimeError(
            "--candidate-partition-value requires parquet candidate input (.parquet)"
        )

    row_map, widths = parse_dat_rows(path, row_year_offset=candidate_year_offset)
    return row_map, widths, "dat", {}


def load_tolerance_config(path: Path | None, default_abs: float, default_rel: float) -> tuple[Tolerance, Dict[str, Tolerance]]:
    default = Tolerance(abs_tol=default_abs, rel_tol=default_rel)
    by_col: Dict[str, Tolerance] = {}
    if path is None:
        return default, by_col

    payload = json.loads(path.read_text(encoding="utf-8"))
    default_payload = payload.get("default", {})
    default = Tolerance(
        abs_tol=float(default_payload.get("abs", default_abs)),
        rel_tol=float(default_payload.get("rel", default_rel)),
    )
    columns = payload.get("columns", {})
    for name, config in columns.items():
        by_col[name] = Tolerance(abs_tol=float(config["abs"]), rel_tol=float(config["rel"]))
    return default, by_col


def value_within_tolerance(a_val: float, b_val: float, tolerance: Tolerance) -> tuple[bool, float, float]:
    abs_diff = abs(a_val - b_val)
    denom = max(abs(a_val), abs(b_val), 1e-30)
    rel_diff = abs_diff / denom
    within = abs_diff <= tolerance.abs_tol or rel_diff <= tolerance.rel_tol
    return within, abs_diff, rel_diff


def compare_rows(
    baseline_rows: Mapping[Tuple[int, int, int], Mapping[str, float]],
    candidate_rows: Mapping[Tuple[int, int, int], Mapping[str, float]],
    default_tol: Tolerance,
    per_column_tol: Mapping[str, Tolerance],
    top_n: int,
) -> Dict[str, object]:
    baseline_keys = set(baseline_rows)
    candidate_keys = set(candidate_rows)
    common_keys = sorted(baseline_keys & candidate_keys)
    only_baseline = sorted(baseline_keys - candidate_keys)
    only_candidate = sorted(candidate_keys - baseline_keys)

    baseline_cols = set().union(*(row.keys() for row in baseline_rows.values())) if baseline_rows else set()
    candidate_cols = set().union(*(row.keys() for row in candidate_rows.values())) if candidate_rows else set()
    shared_cols = sorted((baseline_cols & candidate_cols) - set(KEY_FIELDS))
    baseline_only_cols = sorted((baseline_cols - candidate_cols) - set(KEY_FIELDS))
    candidate_only_cols = sorted((candidate_cols - baseline_cols) - set(KEY_FIELDS))

    column_stats: List[Dict[str, object]] = []
    semantic_pass = True

    for column in shared_cols:
        tol = per_column_tol.get(column, default_tol)
        compared = 0
        fail_count = 0
        sum_abs = 0.0
        sum_sq = 0.0
        max_abs = 0.0
        max_rel = 0.0
        max_key: Tuple[int, int, int] | None = None

        for key in common_keys:
            base_row = baseline_rows[key]
            cand_row = candidate_rows[key]
            if column not in base_row or column not in cand_row:
                continue
            compared += 1
            within, abs_diff, rel_diff = value_within_tolerance(base_row[column], cand_row[column], tol)
            if not within:
                fail_count += 1
            sum_abs += abs_diff
            sum_sq += abs_diff * abs_diff
            if abs_diff > max_abs:
                max_abs = abs_diff
                max_key = key
            if rel_diff > max_rel:
                max_rel = rel_diff

        mean_abs = (sum_abs / compared) if compared else 0.0
        rmse = math.sqrt(sum_sq / compared) if compared else 0.0
        column_pass = fail_count == 0
        semantic_pass = semantic_pass and column_pass

        column_stats.append(
            {
                "column": column,
                "compared_points": compared,
                "fail_count": fail_count,
                "pass": column_pass,
                "abs_tolerance": tol.abs_tol,
                "rel_tolerance": tol.rel_tol,
                "mean_abs_diff": mean_abs,
                "rmse": rmse,
                "max_abs_diff": max_abs,
                "max_rel_diff": max_rel,
                "max_abs_key": list(max_key) if max_key is not None else None,
            }
        )

    if only_baseline or only_candidate or baseline_only_cols:
        semantic_pass = False

    investigation_columns = [name for name in DEFAULT_INVESTIGATION_COLUMNS if name in shared_cols]
    investigation_columns_missing = sorted(
        name for name in DEFAULT_INVESTIGATION_COLUMNS if name not in shared_cols
    )
    row_scores: List[Tuple[Tuple[int, int, int], float]] = []
    for key in common_keys:
        score = 0.0
        used = 0
        for name in investigation_columns:
            base_val = baseline_rows[key].get(name)
            cand_val = candidate_rows[key].get(name)
            if base_val is None or cand_val is None:
                continue
            score += abs(base_val - cand_val)
            used += 1
        if used:
            row_scores.append((key, score / used))

    row_scores.sort(key=lambda item: item[1], reverse=True)
    top_rows = []
    for key, score in row_scores[:top_n]:
        row_delta = {"key": list(key), "mean_abs_diff_score": score, "columns": {}}
        for name in investigation_columns:
            base_val = baseline_rows[key].get(name)
            cand_val = candidate_rows[key].get(name)
            if base_val is None or cand_val is None:
                continue
            row_delta["columns"][name] = {
                "baseline": base_val,
                "candidate": cand_val,
                "abs_diff": abs(base_val - cand_val),
            }
        top_rows.append(row_delta)

    return {
        "semantic_pass": semantic_pass,
        "shared_column_count": len(shared_cols),
        "shared_columns": shared_cols,
        "baseline_only_columns": baseline_only_cols,
        "candidate_only_columns": candidate_only_cols,
        "common_row_count": len(common_keys),
        "only_baseline_count": len(only_baseline),
        "only_candidate_count": len(only_candidate),
        "only_baseline_examples": [list(key) for key in only_baseline[:25]],
        "only_candidate_examples": [list(key) for key in only_candidate[:25]],
        "investigation_columns_used": investigation_columns,
        "investigation_columns_missing": investigation_columns_missing,
        "column_stats": column_stats,
        "top_divergent_rows": top_rows,
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline-wat", type=Path, required=True)
    parser.add_argument("--candidate-wat", type=Path, required=True)
    parser.add_argument("--report-json", type=Path, required=True)
    parser.add_argument("--tolerance-config", type=Path, default=None)
    parser.add_argument("--abs-tol-default", type=float, default=0.1)
    parser.add_argument("--rel-tol-default", type=float, default=0.02)
    parser.add_argument("--top-n", type=int, default=25)
    parser.add_argument("--candidate-partition-value", type=int, default=None)
    parser.add_argument("--candidate-partition-column", type=str, default="wepp_id")
    parser.add_argument("--candidate-year-offset", type=int, default=0)
    return parser.parse_args()


def main() -> None:
    args = parse_args()

    baseline_rows, baseline_widths, baseline_format, baseline_alias_sources = load_rows(
        args.baseline_wat, None, args.candidate_partition_column, 0
    )
    candidate_rows, candidate_widths, candidate_format, candidate_alias_sources = load_rows(
        args.candidate_wat,
        args.candidate_partition_value,
        args.candidate_partition_column,
        args.candidate_year_offset,
    )

    if not baseline_rows:
        raise SystemExit(f"no baseline rows parsed from {args.baseline_wat}")
    if not candidate_rows:
        raise SystemExit(f"no candidate rows parsed from {args.candidate_wat}")

    default_tol, per_col_tol = load_tolerance_config(
        args.tolerance_config, args.abs_tol_default, args.rel_tol_default
    )

    comparison = compare_rows(
        baseline_rows,
        candidate_rows,
        default_tol=default_tol,
        per_column_tol=per_col_tol,
        top_n=args.top_n,
    )

    report = {
        "report_schema_version": REPORT_SCHEMA_VERSION,
        "inputs": {
            "baseline_wat": str(args.baseline_wat),
            "candidate_wat": str(args.candidate_wat),
            "baseline_format": baseline_format,
            "candidate_format": candidate_format,
            "row_key_fields": list(KEY_FIELDS),
            "width_diagnostic_mode": "observed_row_field_count",
            "baseline_numeric_widths": baseline_widths,
            "candidate_numeric_widths": candidate_widths,
            "baseline_column_alias_sources": baseline_alias_sources,
            "candidate_column_alias_sources": candidate_alias_sources,
            "candidate_partition_value": args.candidate_partition_value,
            "candidate_partition_column": args.candidate_partition_column,
            "candidate_year_offset": args.candidate_year_offset,
            "baseline_sha256": sha256_file(args.baseline_wat),
            "candidate_sha256": sha256_file(args.candidate_wat),
        },
        "tolerances": {
            "default": {
                "abs": default_tol.abs_tol,
                "rel": default_tol.rel_tol,
            },
            "column_overrides": {
                name: {"abs": tol.abs_tol, "rel": tol.rel_tol}
                for name, tol in sorted(per_col_tol.items())
            },
        },
        "comparison": comparison,
    }

    args.report_json.parent.mkdir(parents=True, exist_ok=True)
    args.report_json.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
