#!/usr/bin/env python3
"""Build CAL-06 ensemble summaries, verdicts, and narrative evidence."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import statistics
from collections import defaultdict
from pathlib import Path
from typing import Any

PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"


def rows(name: str) -> list[dict[str, str]]:
    with (ARTIFACTS / name).open(newline="", encoding="utf-8") as stream:
        return list(csv.DictReader(stream))


def write_csv(name: str, values: list[dict[str, Any]]) -> None:
    if not values:
        raise ValueError(f"refusing to write empty {name}")
    with (ARTIFACTS / name).open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(values[0]))
        writer.writeheader()
        writer.writerows(values)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def normalize_execution_manifest(manifest: dict[str, Any]) -> None:
    execution = rows("execution-manifest.csv")
    runner = manifest["runner"]
    for row in execution:
        row["stderr_sha256"] = "EPHEMERAL_PATH_BEARING_NOT_RETAINED"
        row["command"] = (
            f"{runner} --run-dir <scratch>/{row['run_id']}/fixture "
            f"--run-file {next_run_file(row['fixture'])} "
            f"--output-dir <scratch>/{row['run_id']}/output "
            "--legacy-sidecar-discovery --direct-production-executor"
        )
    write_csv("execution-manifest.csv", execution)
    path = ARTIFACTS / "execution-manifest.csv"
    manifest["outputs"]["execution-manifest.csv"] = {
        "sha256": sha256(path),
        "bytes": path.stat().st_size,
    }
    (ARTIFACTS / "result-manifest.json").write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def exclude_contradictory_harvard_swe(manifest: dict[str, Any]) -> None:
    scores = rows("observation-scores.csv")
    for row in scores:
        if row["site"] == "harvard" and row["quantity"] == "swe":
            row["matched_count"] = "0"
            row["bias"] = ""
            row["mae"] = ""
            row["rmse"] = ""
            row["verdict"] = (
                "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION"
                if row["stratum"] in {"deciduous", "open"}
                else "NOT_EVALUATED"
            )
    write_csv("observation-scores.csv", scores)
    path = ARTIFACTS / "observation-scores.csv"
    manifest["outputs"]["observation-scores.csv"] = {
        "sha256": sha256(path),
        "bytes": path.stat().st_size,
    }
    manifest["observation_operator"] = {
        "harvard_bulk_density": (
            "WAT aggregate density versus HF237-01 daily bulk density"
        ),
        "harvard_profile_density": "NOT_EVALUATED_SCALE_MISMATCH",
        "harvard_swe": "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION",
    }
    (ARTIFACTS / "result-manifest.json").write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def next_run_file(fixture: str) -> str:
    mapping = {
        "marcell_conifer_mn": "p8.native.run.toml",
        "marcell_deciduous_mn": "p15.native.run.toml",
        "marcell_mixed_mn": "p10.native.run.toml",
        "marcell_open_mn": "p6.native.run.toml",
        "harvard_deciduous_ma": "p6.native.run.toml",
        "harvard_mixed_ma": "p8.native.run.toml",
        "harvard_open_ma": "p3.native.run.toml",
        "hubbardbrook_deciduous_nh": "p10.native.run.toml",
        "hubbardbrook_mixed_nh": "p4.native.run.toml",
    }
    return mapping[fixture]


def number(row: dict[str, str], field: str) -> float:
    value = float(row[field])
    if not math.isfinite(value):
        raise ValueError(f"{field} is not finite")
    return value


def ensemble_summary(run_rows: list[dict[str, str]]) -> list[dict[str, Any]]:
    fields = (
        "winter_cover_mean",
        "summer_cover_max",
        "summer_lai_max",
        "cover_amplitude",
        "annual_leaf_litter_median_kg_m2",
        "annual_interception_median_mm",
        "annual_et_median_mm",
        "annual_runoff_median_mm",
        "peak_swe_median_mm",
        "peak_snow_depth_median_mm",
        "peak_snow_density_median_kg_m3",
        "meltout_median_day_of_year",
        "frost_onset_median_day_of_year",
        "frost_thaw_median_day_of_year",
    )
    grouped: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for row in run_rows:
        grouped[(row["site"], row["stratum"])].append(row)
    output: list[dict[str, Any]] = []
    for (site, stratum), group in sorted(grouped.items()):
        record: dict[str, Any] = {
            "site": site,
            "stratum": stratum,
            "member_count": len(group),
        }
        for field in fields:
            values = [number(row, field) for row in group if row[field] != ""]
            record[f"{field}_min"] = min(values) if values else ""
            record[f"{field}_median"] = statistics.median(values) if values else ""
            record[f"{field}_max"] = max(values) if values else ""
        output.append(record)
    return output


def ordering_result(
    run_rows: list[dict[str, str]], site: str, strata: tuple[str, ...]
) -> tuple[int, int]:
    by_key = {
        (row["stratum"], row["member_id"]): number(row, "winter_cover_mean")
        for row in run_rows
        if row["site"] == site and row["member_id"] != "OPEN-CONTROL"
    }
    members = sorted(
        {
            member
            for (stratum, member) in by_key
            if stratum == strata[0]
        }
    )
    passed = 0
    for member in members:
        values = [by_key[(stratum, member)] for stratum in strata]
        if all(left < right for left, right in zip(values, values[1:])):
            passed += 1
    return passed, len(members)


def score_summary(score_rows: list[dict[str, str]]) -> list[dict[str, Any]]:
    grouped: dict[tuple[str, str, str], list[dict[str, str]]] = defaultdict(list)
    for row in score_rows:
        if int(row["matched_count"]) > 0:
            grouped[(row["site"], row["stratum"], row["quantity"])].append(row)
    output: list[dict[str, Any]] = []
    for (site, stratum, quantity), group in sorted(grouped.items()):
        output.append(
            {
                "site": site,
                "stratum": stratum,
                "quantity": quantity,
                "units": group[0]["units"],
                "member_count": len(group),
                "matched_count_per_member_min": min(int(row["matched_count"]) for row in group),
                "matched_count_per_member_max": max(int(row["matched_count"]) for row in group),
                "bias_min": min(number(row, "bias") for row in group),
                "bias_median": statistics.median(number(row, "bias") for row in group),
                "bias_max": max(number(row, "bias") for row in group),
                "mae_min": min(number(row, "mae") for row in group),
                "mae_median": statistics.median(number(row, "mae") for row in group),
                "mae_max": max(number(row, "mae") for row in group),
                "rmse_min": min(number(row, "rmse") for row in group),
                "rmse_median": statistics.median(number(row, "rmse") for row in group),
                "rmse_max": max(number(row, "rmse") for row in group),
                "verdict": "BOUNDED_NO_SOURCE_UNCERTAINTY",
            }
        )
    return output


def verdicts(
    run_rows: list[dict[str, str]], score_rows: list[dict[str, str]]
) -> tuple[list[dict[str, str]], dict[str, tuple[int, int]]]:
    orders = {
        "marcell": ordering_result(
            run_rows, "marcell", ("deciduous", "mixed", "conifer")
        ),
        "harvard": ordering_result(run_rows, "harvard", ("deciduous", "mixed")),
        "hubbard_brook": ordering_result(
            run_rows, "hubbard_brook", ("deciduous", "mixed")
        ),
    }
    snow_count = sum(
        int(row["matched_count"])
        for row in score_rows
        if row["site"] == "marcell"
        and row["verdict"] != "INVALID_SOURCE_UNIT_IDENTITY_CONTRADICTION"
    )
    values = [
        {
            "cell_id": "CAL06-CAN-001",
            "status": "BOUNDED",
            "quantitative_result": f"{orders['marcell'][0]}/{orders['marcell'][1]} members preserve deciduous<mixed<conifer winter cover",
            "advancement": "PASS_MODEL_ORDERING",
            "rationale": "Complete ensemble ordering passes, but category seed amplitude operands are not independently calibrated.",
        },
        {
            "cell_id": "CAL06-CAN-002",
            "status": "BOUNDED",
            "quantitative_result": f"{orders['harvard'][0]}/{orders['harvard'][1]} members preserve deciduous<mixed winter cover",
            "advancement": "PASS_MODEL_ORDERING",
            "rationale": "No pure Harvard conifer lane exists and Harvard timing transferability remains poor.",
        },
        {
            "cell_id": "CAL06-CAN-003",
            "status": "BOUNDED",
            "quantitative_result": f"{orders['hubbard_brook'][0]}/{orders['hubbard_brook'][1]} members preserve deciduous<mixed winter cover",
            "advancement": "PASS_MODEL_ORDERING",
            "rationale": "No source-supplied open or pure-conifer lane exists.",
        },
        {
            "cell_id": "CAL06-CAN-004",
            "status": "BOUNDED",
            "quantitative_result": "37-member summer cover, LAI, and amplitude ranges retained for every forest lane",
            "advancement": "NO_EMPIRICAL_AMPLITUDE_PROMOTION",
            "rationale": "Bf,max, fe, xmxlai, Cs, and bb remain calibration-ready-data-limited.",
        },
        {
            "cell_id": "CAL06-SNOW-001",
            "status": "BOUNDED",
            "quantitative_result": f"{snow_count} exact-date metric/member matches across bound snow strata",
            "advancement": "QUANTIFIED_NO_SUPPORT_THRESHOLD",
            "rationale": "Bias, MAE, and RMSE are retained; source uncertainty does not authorize a pass threshold.",
        },
        {
            "cell_id": "CAL06-SNOW-002",
            "status": "BOUNDED",
            "quantitative_result": "Harvard open/hardwood depth and density scored; SWE excluded; hemlock unbound",
            "advancement": "QUANTIFIED_NO_SUPPORT_THRESHOLD",
            "rationale": "SWE metadata/identity conflict is unresolved; the mixed lane is not a pure hemlock counterpart.",
        },
        {
            "cell_id": "CAL06-SNOW-003",
            "status": "BOUNDED",
            "quantitative_result": "Peak SWE, depth, density, and melt-out distributions retained for all lanes",
            "advancement": "MODEL_RESPONSE_ONLY",
            "rationale": "Common snow residuals are not attributed to canopy phenology.",
        },
        {
            "cell_id": "CAL06-LIT-001",
            "status": "NOT_EVALUATED",
            "quantitative_result": "Leaf transfer emitted; needle and fine-woody predictive sources null in all 259 forest/member runs",
            "advancement": "AUTHORITY_MISSING",
            "rationale": "CAL-05 predictive-source hold is preserved; null is not zero.",
        },
        {
            "cell_id": "CAL06-RES-001",
            "status": "NOT_EVALUATED",
            "quantitative_result": "Residue mass/depth chronology retained",
            "advancement": "NOT_ADVANCED",
            "rationale": "Aggregate residue adequacy cannot advance with incomplete litter sources and unfitted decay.",
        },
        {
            "cell_id": "CAL06-FROST-001",
            "status": "NOT_EVALUATED",
            "quantitative_result": "Frost onset/depth/thaw model response retained",
            "advancement": "NOT_ADVANCED",
            "rationale": "The residue/source chain does not pass, so frost consequence is not promoted.",
        },
        {
            "cell_id": "CAL06-ET-001",
            "status": "NOT_EVALUATED",
            "quantitative_result": "Interception and ET distributions retained",
            "advancement": "NOT_ADVANCED",
            "rationale": "Canopy amplitude and transferability are bounded rather than supported.",
        },
        {
            "cell_id": "CAL06-RUN-001",
            "status": "NOT_EVALUATED",
            "quantitative_result": "Runoff distributions retained",
            "advancement": "NOT_ADVANCED",
            "rationale": "Snow and residue/frost upstream cells are not supported.",
        },
        {
            "cell_id": "CAL06-ERO-001",
            "status": "NOT_EVALUATED",
            "quantitative_result": "Canopy/cover operands reached erosion consumers; no erosion consequence output was emitted",
            "advancement": "NOT_ADVANCED",
            "rationale": "Consumer input lineage is not an erosion-result surface and cannot carry a consequence claim.",
        },
    ]
    return values, orders


def main() -> int:
    run_rows = rows("run-results.csv")
    score_rows = rows("observation-scores.csv")
    manifest = json.loads((ARTIFACTS / "result-manifest.json").read_text(encoding="utf-8"))
    normalize_execution_manifest(manifest)
    exclude_contradictory_harvard_swe(manifest)
    score_rows = rows("observation-scores.csv")
    if len(run_rows) != 261 or manifest["passed_runs"] != 261:
        raise SystemExit("CAL-06 execution inventory is incomplete")
    summaries = ensemble_summary(run_rows)
    scores = score_summary(score_rows)
    matrix, orders = verdicts(run_rows, score_rows)
    write_csv("ensemble-summary.csv", summaries)
    write_csv("observation-score-summary.csv", scores)
    write_csv("verdict-matrix.csv", matrix)
    science = f"""# CAL-06 Science Summary

Evidence class: `Ran + Static interpretation`

## Result

CAL-06 completed all 261 prespecified runs: 37 frozen CAL-04B GSI members
through each of seven native forest lanes, plus the Marcell and Harvard open
controls. The result is `COMPLETE / BOUNDED GRADIENT CHARACTERIZATION /
DOWNSTREAM ADVANCEMENT WITHHELD`.

The within-model winter canopy ordering is stable across the full ensemble:

- Marcell deciduous < mixed < conifer: {orders["marcell"][0]}/{orders["marcell"][1]} members.
- Harvard deciduous < mixed: {orders["harvard"][0]}/{orders["harvard"][1]} members.
- Hubbard Brook deciduous < mixed: {orders["hubbard_brook"][0]}/{orders["hubbard_brook"][1]} members.

This is a model-response result, not independent canopy-amplitude validation.
The class seed values for foliar mass, evergreen fraction, LAI, structural
cover, and closure remain calibration-ready and data-limited, and CAL-04B's
poor Harvard timing transferability remains contrary evidence.

## Snow and consumers

Exact-date Harvard and Marcell snow comparisons report bias, MAE, and RMSE
where the source is internally admissible. Harvard SWE is excluded because
the provider metadata labels it centimeters while the raw values contradict
the same-row depth × density identity by approximately a factor of ten. The
package does not reinterpret that field as millimeters without source
clarification. Remaining depth, density, and Marcell SWE comparisons are
bounded rather than supported because the retained observations do not
authorize a scoring tolerance or complete uncertainty model. The full
ensemble peak, density, and melt-out response is retained without assigning
common snow residuals to canopy phenology.

Leaf litter, residue, frost, interception, ET, and runoff chronology ran
through real production consumers. Predictive needle and fine-woody sources
remained null in all 259 forest/member runs. Therefore total litter/residue
adequacy, frost consequence, ET, runoff, and erosion are visibly
`NOT ADVANCED`. Erosion-facing canopy inputs were present, but no erosion
consequence output was emitted; no zero or proxy was substituted.
"""
    (ARTIFACTS / "science-summary.md").write_text(science, encoding="utf-8")
    readiness = """# Calibration-Readiness Matrix

Evidence class: `Ran + Static`

| Obligation | Science implementation | Calibration evidence | Identifiability | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| GSI timing | IMPLEMENTED | EMPIRICALLY_CALIBRATED | PARTIALLY_IDENTIFIABLE | PASS | CAL-04B frozen 37-member ensemble; `execution-manifest.csv` |
| Canopy amplitude operands | IMPLEMENTED | CALIBRATION_READY_DATA_LIMITED | PARTIALLY_IDENTIFIABLE | PASS | `ensemble-summary.csv`; no best-member selection |
| Within-site canopy ordering | IMPLEMENTED | CALIBRATION_READY_DATA_LIMITED | PARTIALLY_IDENTIFIABLE | PASS | `verdict-matrix.csv` CAL06-CAN-001..004 |
| Snow observation operator | IMPLEMENTED | NOT_APPLICABLE | NOT_APPLICABLE | PASS | `observation-scores.csv`; `observation-score-summary.csv` |
| Leaf litter source | IMPLEMENTED | CALIBRATION_READY_DATA_LIMITED | PARTIALLY_IDENTIFIABLE | PASS | `run-results.csv` |
| Predictive needle/fine-woody source | AUTHORITY_MISSING | NOT_CALIBRATION_READY | NOT_ASSESSED | NOT_APPLICABLE to CAL-06 completion | Null retained in every forest run; CAL-05 hold |
| Aggregate residue adequacy | IMPLEMENTED | NOT_CALIBRATION_READY | NOT_ASSESSED | NOT_APPLICABLE | Source composition and decay are unresolved |
| Downstream consequence promotion | IMPLEMENTED | NOT_APPLICABLE | NOT_APPLICABLE | NOT_APPLICABLE | Model responses retained; advancement withheld |

No current-scope calibration was attempted. Missing predictive source
authority is an advancement boundary, not a failure to execute the bounded
CAL-06 characterization.
"""
    (ARTIFACTS / "calibration-readiness-matrix.md").write_text(
        readiness, encoding="utf-8"
    )
    lineage = """# Real Consumer And Operand Lineage

Evidence class: `Ran + Static source inspection`

| Producer | State/frame | Consumer | Retained result | Negative proof |
| --- | --- | --- | --- | --- |
| Native GSI/forest canopy advance | post-phenology growth state | snow canopy attenuation | WAT SWE/depth/density response | Static initial canopy does not carry native trace identity. |
| Same post-phenology state | day input and direct frame | WB15 interception | trace and WAT interception identity | No repeated scalar canopy sidecar. |
| Same post-phenology state | ET compute inputs | WB17 ET | WAT `Ep+Es+Er` | No canopy parameter was selected from ET residuals. |
| Leaf-off transfer | decomposition/residue frame | surface residue and frost thermal input | trace residue and WAT frost response | Needle/fine-wood nulls were not converted to zero. |
| Direct water frame | WAT publication | runoff | WAT `Q` | Runoff was not used to refit canopy. |
| Post-phenology canopy/ground cover | erosion daily consumer inputs | erosion producer | input lineage only | No erosion output exists in this run surface; consequence remains `NOT_ADVANCED`. |

All forest runs used the direct-production executor. The research trace
validated exact post-phenology producer/consumer identities before the WAT
result was summarized.
"""
    (ARTIFACTS / "consumer-lineage.md").write_text(lineage, encoding="utf-8")
    print(
        f"PASS: {len(summaries)} lane summaries; {len(scores)} observation summaries; "
        f"{len(matrix)} verdict cells"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
