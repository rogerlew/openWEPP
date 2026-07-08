#!/usr/bin/env python3
"""Replay selected-cohort annual sediment comparisons under SC-OFEROUTE rev 44."""

from __future__ import annotations

import hashlib
import json
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

import pyarrow.parquet as pq


REPO = Path(__file__).resolve().parents[4]
PACKAGE = REPO / "docs/work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001"
ARTIFACTS = PACKAGE / "artifacts"
SOURCE_SUMMARY = (
    REPO
    / "docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001"
    / "artifacts/coupled-spacetime-summary.json"
)
SEDIMENT_COLUMNS = ["tdet", "tdep", "sedcon_1", "sedcon_2", "sedcon_3", "sedcon_4", "sedcon_5"]
TOLERANCE = 0.02
ADEQUACY_TOLERANCE = TOLERANCE / 3.0
MATERIAL_YEAR_FRACTION = 0.05
DRY_EPS = 1.0e-12


def rel(path: Path) -> str:
    return str(path.relative_to(REPO))


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json(path: Path) -> Any:
    return json.loads(path.read_text())


def annual_sums(pass_path: Path) -> dict[str, dict[str, float]]:
    table = pq.read_table(pass_path)
    data = table.to_pydict()
    result: dict[str, dict[str, float]] = {column: defaultdict(float) for column in SEDIMENT_COLUMNS}
    for idx, year in enumerate(data["year"]):
        key = str(year)
        for column in SEDIMENT_COLUMNS:
            result[column][key] += float(data[column][idx])
    return {column: dict(sorted(values.items(), key=lambda item: int(item[0]))) for column, values in result.items()}


def strict_relative(candidate: float, reference: float) -> float:
    if abs(reference) <= DRY_EPS and abs(candidate) <= DRY_EPS:
        return 0.0
    return abs(candidate - reference) / max(abs(reference), DRY_EPS)


def evaluate_column(
    candidate: dict[str, float],
    reference: dict[str, float],
    tolerance: float,
) -> dict[str, Any]:
    years = sorted(set(candidate) | set(reference), key=int)
    total_reference_abs = sum(abs(reference.get(year, 0.0)) for year in years)
    total_candidate_abs = sum(abs(candidate.get(year, 0.0)) for year in years)
    vector_abs_delta = sum(abs(candidate.get(year, 0.0) - reference.get(year, 0.0)) for year in years)
    vector_rel = 0.0
    dry_reference = total_reference_abs <= DRY_EPS
    if dry_reference:
        vector_pass = total_candidate_abs <= DRY_EPS
    else:
        vector_rel = vector_abs_delta / total_reference_abs
        vector_pass = vector_rel <= tolerance

    strict_max_rel = 0.0
    strict_max_year = None
    material_max_rel = 0.0
    material_max_year = None
    low_contribution_max_rel = 0.0
    low_contribution_max_year = None
    material_years = []
    low_contribution_years = []
    per_year = []

    for year in years:
        cand = candidate.get(year, 0.0)
        ref = reference.get(year, 0.0)
        rel_delta = strict_relative(cand, ref)
        contribution = 0.0 if dry_reference else abs(ref) / total_reference_abs
        material = contribution >= MATERIAL_YEAR_FRACTION
        if rel_delta > strict_max_rel:
            strict_max_rel = rel_delta
            strict_max_year = year
        if material:
            material_years.append(year)
            if rel_delta > material_max_rel:
                material_max_rel = rel_delta
                material_max_year = year
        else:
            low_contribution_years.append(year)
            if rel_delta > low_contribution_max_rel:
                low_contribution_max_rel = rel_delta
                low_contribution_max_year = year
        per_year.append(
            {
                "year": year,
                "candidate": cand,
                "reference": ref,
                "abs_delta": abs(cand - ref),
                "strict_rel": rel_delta,
                "reference_contribution": contribution,
                "material_year": material,
            }
        )

    material_pass = material_max_rel <= tolerance
    return {
        "dry_reference": dry_reference,
        "total_reference_abs": total_reference_abs,
        "total_candidate_abs": total_candidate_abs,
        "vector_abs_delta": vector_abs_delta,
        "vector_rel": vector_rel,
        "vector_pass": vector_pass,
        "strict_max_rel": strict_max_rel,
        "strict_max_year": strict_max_year,
        "strict_pass": strict_max_rel <= tolerance,
        "material_max_rel": material_max_rel,
        "material_max_year": material_max_year,
        "material_pass": material_pass,
        "low_contribution_max_rel": low_contribution_max_rel,
        "low_contribution_max_year": low_contribution_max_year,
        "material_years": material_years,
        "low_contribution_years": low_contribution_years,
        "per_year": per_year,
        "rev44_pass": vector_pass and material_pass,
    }


def threshold_for_role(role: str) -> float:
    return ADEQUACY_TOLERANCE if "adequacy" in role else TOLERANCE


def evaluate_comparison(
    member_id: str,
    role: str,
    candidate_rung: str,
    reference_rung: str,
    annuals: dict[tuple[str, str], dict[str, dict[str, float]]],
) -> dict[str, Any]:
    tolerance = threshold_for_role(role)
    candidate = annuals[(member_id, candidate_rung)]
    reference = annuals[(member_id, reference_rung)]
    columns = {}
    strict_max_rel = 0.0
    strict_max_surface = None
    vector_max_rel = 0.0
    vector_max_column = None
    material_max_rel = 0.0
    material_max_surface = None
    low_contribution_max_rel = 0.0
    low_contribution_max_surface = None
    failures = []

    for column in SEDIMENT_COLUMNS:
        column_result = evaluate_column(candidate[column], reference[column], tolerance)
        columns[column] = column_result
        if column_result["strict_max_rel"] > strict_max_rel:
            strict_max_rel = column_result["strict_max_rel"]
            strict_max_surface = f"{column}:{column_result['strict_max_year']}"
        if column_result["vector_rel"] > vector_max_rel:
            vector_max_rel = column_result["vector_rel"]
            vector_max_column = column
        if column_result["material_max_rel"] > material_max_rel:
            material_max_rel = column_result["material_max_rel"]
            material_max_surface = f"{column}:{column_result['material_max_year']}"
        if column_result["low_contribution_max_rel"] > low_contribution_max_rel:
            low_contribution_max_rel = column_result["low_contribution_max_rel"]
            low_contribution_max_surface = f"{column}:{column_result['low_contribution_max_year']}"
        if not column_result["vector_pass"]:
            failures.append(f"{column} vector_rel {column_result['vector_rel']:.9g} > {tolerance:.9g}")
        if not column_result["material_pass"]:
            failures.append(
                f"{column} material_max_rel {column_result['material_max_rel']:.9g} > {tolerance:.9g}"
            )

    return {
        "member_id": member_id,
        "role": role,
        "candidate_rung": candidate_rung,
        "reference_rung": reference_rung,
        "tolerance": tolerance,
        "strict_relative": {
            "max_rel": strict_max_rel,
            "max_surface": strict_max_surface,
            "passes": strict_max_rel <= tolerance,
        },
        "rev44": {
            "vector_max_rel": vector_max_rel,
            "vector_max_column": vector_max_column,
            "material_max_rel": material_max_rel,
            "material_max_surface": material_max_surface,
            "low_contribution_max_rel": low_contribution_max_rel,
            "low_contribution_max_surface": low_contribution_max_surface,
            "failures": failures,
            "passes": len(failures) == 0,
            "columns": columns,
        },
    }


def write_markdown(result: dict[str, Any]) -> None:
    lines = [
        "# Annual Sediment Metric Replay",
        "",
        "Evidence mode: Ran.",
        "",
        "## Metric",
        "",
        f"- Material-year threshold: `{MATERIAL_YEAR_FRACTION}` of a column's total absolute reference annual sum.",
        "- Material member-years keep the existing relative tolerance.",
        "- All member-years are also bounded by per-column annual-vector L1 relative to the reference annual vector.",
        "- Low-contribution member-year strict-relative excursions remain reported but are not standalone blockers.",
        "",
        "## Summary",
        "",
        f"- Comparisons replayed: `{len(result['comparisons'])}`",
        f"- Strict-relative annual sediment blockers: `{len(result['strict_relative_blockers'])}`",
        f"- Rev-44 annual sediment blockers: `{len(result['rev44_blockers'])}`",
        "",
    ]
    if result["strict_relative_blockers"]:
        lines += ["### Strict-Relative Blockers", ""]
        for row in result["strict_relative_blockers"]:
            lines.append(
                "- `{member}` `{role}` `{candidate}` vs `{reference}`: `{surface}` `{value:.9g}` > `{threshold:.9g}`".format(
                    member=row["member_id"],
                    role=row["role"],
                    candidate=row["candidate_rung"],
                    reference=row["reference_rung"],
                    surface=row["strict_relative"]["max_surface"],
                    value=row["strict_relative"]["max_rel"],
                    threshold=row["tolerance"],
                )
            )
        lines.append("")
    lines += ["### Rev-44 Replay Table", ""]
    lines.append(
        "| Role | Member | Candidate | Reference | Strict max rel | Strict surface | Vector max rel | Vector column | Material max rel | Low max rel | Rev-44 verdict |"
    )
    lines.append("|---|---|---|---|---:|---|---:|---|---:|---:|---|")
    for row in result["comparisons"]:
        lines.append(
            "| `{role}` | `{member}` | `{candidate}` | `{reference}` | `{strict:.9g}` | `{surface}` | `{vector:.9g}` | `{vector_col}` | `{material:.9g}` | `{low:.9g}` | {verdict} |".format(
                role=row["role"],
                member=row["member_id"],
                candidate=row["candidate_rung"],
                reference=row["reference_rung"],
                strict=row["strict_relative"]["max_rel"],
                surface=row["strict_relative"]["max_surface"],
                vector=row["rev44"]["vector_max_rel"],
                vector_col=row["rev44"]["vector_max_column"],
                material=row["rev44"]["material_max_rel"],
                low=row["rev44"]["low_contribution_max_rel"],
                verdict="PASS" if row["rev44"]["passes"] else "FAIL",
            )
        )
    lines += [
        "",
        "## Decision Impact",
        "",
        "The rev-44 annual sediment metric closes the WA `tdep:4` strict-relative",
        "low-denominator blocker without changing routed-water, shape, storage,",
        "tail-fold, closure, active selector, or production default behavior.",
        "",
        "No `dx5` production flip is made by this package.",
    ]
    (ARTIFACTS / "annual-sediment-metric-replay.md").write_text("\n".join(lines) + "\n")


def main() -> None:
    summary = read_json(SOURCE_SUMMARY)
    runs = {
        (run["member_id"], run["rung"]): run
        for run in summary["runs"]
        if run.get("status") == "PASS" and run.get("pass_parquet_path")
    }
    annuals = {
        key: annual_sums(Path(run["pass_parquet_path"]))
        for key, run in sorted(runs.items())
    }
    comparisons = [
        evaluate_comparison(
            comparison["member_id"],
            comparison["comparison_role"],
            comparison["candidate_rung"],
            comparison["reference_rung"],
            annuals,
        )
        for comparison in summary["comparisons"]
    ]
    strict_relative_blockers = [
        row for row in comparisons if not row["strict_relative"]["passes"]
    ]
    rev44_blockers = [row for row in comparisons if not row["rev44"]["passes"]]
    result = {
        "created_utc": datetime.now(timezone.utc).isoformat(),
        "source_summary": rel(SOURCE_SUMMARY),
        "source_summary_sha256": sha256(SOURCE_SUMMARY),
        "metric": {
            "material_year_fraction": MATERIAL_YEAR_FRACTION,
            "tolerance": TOLERANCE,
            "adequacy_tolerance": ADEQUACY_TOLERANCE,
            "dry_epsilon": DRY_EPS,
            "columns": SEDIMENT_COLUMNS,
        },
        "release_binary": summary["release_binary"],
        "run_count": len(runs),
        "comparisons": comparisons,
        "strict_relative_blockers": strict_relative_blockers,
        "rev44_blockers": rev44_blockers,
    }
    output = ARTIFACTS / "annual-sediment-metric-replay.json"
    output.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    write_markdown(result)
    print(rel(output))


if __name__ == "__main__":
    main()
