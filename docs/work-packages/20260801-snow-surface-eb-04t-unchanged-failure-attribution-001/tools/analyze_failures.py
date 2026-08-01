#!/usr/bin/env python3
"""Attribute EB-04S unchanged failures from immutable retained evidence."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

sys.dont_write_bytecode = True

REPO = Path(__file__).resolve().parents[4]
PACKAGE = Path(__file__).resolve().parents[1]
ARTIFACTS = PACKAGE / "artifacts"
FIGURES = ARTIFACTS / "figures"
EB04S_REPORT = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04s-authority-reconciliation-"
    "retained-adjudication-001/artifacts/retained-adjudication.json"
)
EB04R_PROTOCOL = REPO / (
    "docs/work-packages/20260801-snow-surface-eb-04r-fresh-factorial-execution-"
    "adjudication-001/artifacts/prospective-decision-protocol.md"
)
SNOWENERGY = REPO / "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md"
SNOWFREEZE = REPO / "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md"
PRIOR_TOOL = REPO / "tools/snowfreeze_observed/post_partition_residual_decomposition.py"
PRIOR_REPORT = REPO / (
    "docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-"
    "decomposition-001/artifacts/post-partition-residual-decomposition.json"
)
INPUTS = (EB04S_REPORT, EB04R_PROTOCOL, SNOWENERGY, SNOWFREEZE, PRIOR_TOOL, PRIOR_REPORT)
CELLS = ("B", "L", "S", "LS")

ATTRIBUTION = {
    "seasonal_densification_trajectory": {
        "family": "density_trajectory",
        "primary_process_owner": "snow density / compaction structure",
        "target_sensitivity": "indirect",
        "criterion_role": "adjacent-process collateral",
        "authority": "SC-SNOWFREEZE-001#INV-SNOWFREEZE-050/#INV-SNOWFREEZE-058; prior post-partition density_trajectory_diffuse cluster",
        "primary_metric": "kge",
        "target": 1.0,
    },
    "seasonal_depth_swe_slope": {
        "family": "depth_swe_geometry",
        "primary_process_owner": "density/layer geometry with canopy interception ambiguity",
        "target_sensitivity": "mixed_ambiguous",
        "criterion_role": "mixed density/interception geometry guard",
        "authority": "SC-SNOWFREEZE-001#INV-SNOWFREEZE-050; prior cancov_depth_swe_slope_geometry cluster",
        "primary_metric": "slope_ratio",
        "target": 1.0,
    },
    "seasonal_peak_swe_date": {
        "family": "peak_timing",
        "primary_process_owner": "accumulation/phase/redistribution and snow energy timing",
        "target_sensitivity": "sublimation_sensitive_open_control_mixed",
        "criterion_role": "LS-blocking open-control timing; does not identify canopy longwave",
        "authority": "SC-SNOWFREEZE-001#INV-SNOWFREEZE-050; SC-SNOWENERGY-001#INV-SNOWENERGY-017/#INV-SNOWENERGY-019",
        "primary_metric": "median_offset_days",
        "target": 0.0,
    },
    "seasonal_peak_depth_date": {
        "family": "peak_timing",
        "primary_process_owner": "density/geometry plus accumulation and snow energy timing",
        "target_sensitivity": "sublimation_sensitive_open_control_mixed",
        "criterion_role": "LS-blocking open-control timing with density coupling; does not identify canopy longwave",
        "authority": "SC-SNOWFREEZE-001#INV-SNOWFREEZE-050; SC-SNOWENERGY-001#INV-SNOWENERGY-019",
        "primary_metric": "median_offset_days",
        "target": 0.0,
    },
    "seasonal_ablation_meltout_date": {
        "family": "meltout_timing",
        "primary_process_owner": "snow energy/mass loss plus forcing representativeness",
        "target_sensitivity": "sublimation_sensitive_open_control",
        "criterion_role": "LS-blocking open-control timing; does not identify canopy longwave",
        "authority": "SC-SNOWFREEZE-001#INV-SNOWFREEZE-050; SC-SNOWENERGY-001#INV-SNOWENERGY-017/#INV-SNOWENERGY-018/#INV-SNOWENERGY-019",
        "primary_metric": "median_offset_days",
        "target": 0.0,
    },
}
EXPECTED = Counter(
    {
        "seasonal_densification_trajectory": 9,
        "seasonal_depth_swe_slope": 2,
        "seasonal_peak_swe_date": 2,
        "seasonal_peak_depth_date": 1,
        "seasonal_ablation_meltout_date": 2,
    }
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rel(path: Path) -> str:
    return path.relative_to(REPO).as_posix()


def normalize_generated_text(path: Path) -> None:
    """Use LF endings and remove generator-introduced trailing whitespace."""
    lines = path.read_text(encoding="utf-8").splitlines()
    path.write_text("\n".join(line.rstrip() for line in lines) + "\n", encoding="utf-8")


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def rubric_cells(lane: dict[str, Any], cell: str) -> dict[str, dict[str, Any]]:
    return {
        item["cell_id"]: item
        for item in lane["cells"][cell]["rubric_profile"]["cells"]
        if item["forcing_robust"]
    }


def primary_value(item: dict[str, Any], rule: dict[str, Any]) -> float:
    value = float(item["metrics"][rule["primary_metric"]])
    if not math.isfinite(value):
        raise RuntimeError("non-finite primary failure metric")
    return value


def error_magnitude(value: float, rule: dict[str, Any]) -> float:
    return abs(value - float(rule["target"]))


def direction(b_error: float, candidate_error: float) -> str:
    if candidate_error < b_error:
        return "toward_observation"
    if candidate_error > b_error:
        return "away_from_observation"
    return "unchanged"


def reconstruct_rows(report: dict[str, Any]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for lane in report["lanes"]:
        if lane["role"] != "INDEPENDENT_VALIDATION":
            continue
        by_cell = {cell: rubric_cells(lane, cell) for cell in CELLS}
        for cell_id, baseline in by_cell["B"].items():
            if baseline["ordinal_label"] != "fail":
                continue
            if cell_id not in ATTRIBUTION:
                raise RuntimeError(f"unattributed baseline failure {cell_id}")
            rule = ATTRIBUTION[cell_id]
            values = {cell: primary_value(by_cell[cell][cell_id], rule) for cell in CELLS}
            errors = {cell: error_magnitude(value, rule) for cell, value in values.items()}
            labels = {cell: by_cell[cell][cell_id]["ordinal_label"] for cell in CELLS}
            if any(label != "fail" for label in labels.values()):
                raise RuntimeError(f"failure label changed unexpectedly: {lane['lane_id']}/{cell_id}")
            row = {
                "lane_id": lane["lane_id"],
                "stratum": lane["stratum"],
                "climate": lane["climate"],
                "cell_id": cell_id,
                **rule,
                "labels": labels,
                "primary_values": values,
                "error_magnitudes": errors,
                "directions_vs_b": {
                    cell: direction(errors["B"], errors[cell]) for cell in ("L", "S", "LS")
                },
                "error_effects": {
                    "longwave_main": errors["L"] - errors["B"],
                    "sublimation_main": errors["S"] - errors["B"],
                    "combined": errors["LS"] - errors["B"],
                    "interaction": errors["LS"] - errors["L"] - errors["S"] + errors["B"],
                },
                "metric_objects_exact_b_to_ls": baseline["metrics"] == by_cell["LS"][cell_id]["metrics"],
            }
            rows.append(row)
    rows.sort(key=lambda item: (item["family"], item["lane_id"], item["cell_id"]))
    return rows


def summarize(rows: list[dict[str, Any]]) -> dict[str, Any]:
    counts = Counter(row["cell_id"] for row in rows)
    if counts != EXPECTED or len(rows) != 16:
        raise RuntimeError(f"failure inventory mismatch: {counts}")
    direction_counts = Counter(row["directions_vs_b"]["LS"] for row in rows)
    by_family: dict[str, Any] = {}
    for family in sorted({row["family"] for row in rows}):
        subset = [row for row in rows if row["family"] == family]
        by_family[family] = {
            "count": len(subset),
            "ls_direction_counts": dict(Counter(row["directions_vs_b"]["LS"] for row in subset)),
            "mean_combined_error_change": sum(row["error_effects"]["combined"] for row in subset) / len(subset),
        }
    open_timing = [row for row in rows if row["family"] in {"peak_timing", "meltout_timing"}]
    adjacent = [row for row in rows if row["family"] in {"density_trajectory", "depth_swe_geometry"}]
    open_timing_counts = Counter(row["directions_vs_b"]["LS"] for row in open_timing)
    adjacent_counts = Counter(row["directions_vs_b"]["LS"] for row in adjacent)
    interaction_counts = Counter(
        "mitigating_additive_error" if row["error_effects"]["interaction"] < 0.0
        else "amplifying_additive_error" if row["error_effects"]["interaction"] > 0.0
        else "zero"
        for row in rows
    )
    return {
        "failure_count": len(rows),
        "counts_by_signature": dict(counts),
        "ls_direction_counts": dict(direction_counts),
        "sublimation_sensitive_open_control_count": len(open_timing),
        "sublimation_sensitive_open_control_ls_direction_counts": dict(open_timing_counts),
        "canopy_longwave_identifying_failure_count": 0,
        "adjacent_process_count": len(adjacent),
        "adjacent_process_ls_direction_counts": dict(adjacent_counts),
        "metric_objects_exact_b_to_ls_count": sum(row["metric_objects_exact_b_to_ls"] for row in rows),
        "primary_errors_unchanged_b_to_ls_count": sum(
            row["directions_vs_b"]["LS"] == "unchanged" for row in rows
        ),
        "interaction_counts": dict(interaction_counts),
        "nonzero_interaction_count": sum(row["error_effects"]["interaction"] != 0.0 for row in rows),
        "by_family": by_family,
        "criterion_fitness": {
            "total_failure_reduction": "CONSERVATIVE_BUT_MIXED_ALIGNMENT",
            "reason": (
                "Eleven failures test density/geometry or mixed interception geometry. Five open-control "
                "timing failures are sensitive to sublimation and block promotion of the combined LS "
                "bundle, but cannot identify canopy-longwave efficacy. None of those five errors moved "
                "toward observation under LS."
            ),
            "retroactive_promotion_supported": False,
            "supported_opt_in_science_status": "IMPLEMENTED_AND_PHYSICALLY_QUALIFIED_EMPIRICAL_BENEFIT_NARROW",
            "prospective_rule_recommendation": (
                "Separate canopy-longwave efficacy, sublimation efficacy, and adjacent-process "
                "noninferiority; require no ordinal regression; and validate on independent evidence "
                "before default activation."
            ),
        },
    }


def write_csv(rows: list[dict[str, Any]]) -> None:
    path = ARTIFACTS / "failure-response-matrix.csv"
    fields = [
        "lane_id", "stratum", "climate", "cell_id", "family",
        "primary_process_owner", "target_sensitivity", "criterion_role",
        "primary_metric", "target", "b_value", "l_value", "s_value", "ls_value",
        "b_error", "l_error", "s_error", "ls_error", "l_direction", "s_direction",
        "ls_direction", "longwave_main_error", "sublimation_main_error",
        "combined_error", "interaction_error", "authority",
    ]
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow(
                {
                    "lane_id": row["lane_id"], "stratum": row["stratum"],
                    "climate": row["climate"], "cell_id": row["cell_id"],
                    "family": row["family"], "primary_process_owner": row["primary_process_owner"],
                    "target_sensitivity": row["target_sensitivity"],
                    "criterion_role": row["criterion_role"], "primary_metric": row["primary_metric"],
                    "target": row["target"],
                    **{f"{cell.lower()}_value": row["primary_values"][cell] for cell in CELLS},
                    **{f"{cell.lower()}_error": row["error_magnitudes"][cell] for cell in CELLS},
                    "l_direction": row["directions_vs_b"]["L"],
                    "s_direction": row["directions_vs_b"]["S"],
                    "ls_direction": row["directions_vs_b"]["LS"],
                    "longwave_main_error": row["error_effects"]["longwave_main"],
                    "sublimation_main_error": row["error_effects"]["sublimation_main"],
                    "combined_error": row["error_effects"]["combined"],
                    "interaction_error": row["error_effects"]["interaction"],
                    "authority": row["authority"],
                }
            )


def make_figures(rows: list[dict[str, Any]], summary: dict[str, Any]) -> None:
    import matplotlib

    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    import numpy as np

    matplotlib.rcParams["svg.hashsalt"] = "snow-surface-eb04t-v1"

    FIGURES.mkdir(parents=True, exist_ok=True)
    labels = [f"{row['lane_id']}\n{row['cell_id'].replace('seasonal_', '')}" for row in rows]
    effect_names = ("longwave_main", "sublimation_main", "combined", "interaction")
    matrix = np.array([[row["error_effects"][name] for name in effect_names] for row in rows])
    scale = np.maximum(np.array([row["error_magnitudes"]["B"] for row in rows])[:, None], 1.0e-15)
    relative = 100.0 * matrix / scale
    limit = max(1.0, float(np.nanmax(np.abs(relative))))
    fig, ax = plt.subplots(figsize=(10.5, 9.5))
    image = ax.imshow(relative, cmap="RdYlGn_r", vmin=-limit, vmax=limit, aspect="auto")
    ax.set_xticks(range(4), ["Longwave", "Sublimation", "Combined LS", "L×S interaction"])
    ax.set_yticks(range(len(labels)), labels, fontsize=7)
    ax.set_title("Change in existing failure error magnitude\nnegative is toward observation; positive is away")
    for y in range(relative.shape[0]):
        for x in range(relative.shape[1]):
            ax.text(x, y, f"{relative[y, x]:+.1f}%", ha="center", va="center", fontsize=6)
    fig.colorbar(image, ax=ax, label="Change from baseline error magnitude (%)")
    fig.tight_layout()
    fig.savefig(FIGURES / "eb04t-failure-error-response.svg", format="svg", metadata={"Date": None})
    plt.close(fig)
    normalize_generated_text(FIGURES / "eb04t-failure-error-response.svg")

    families = list(summary["by_family"])
    directions = ("toward_observation", "unchanged", "away_from_observation")
    colors = {"toward_observation": "#2b8c5a", "unchanged": "#808080", "away_from_observation": "#c84b31"}
    fig, ax = plt.subplots(figsize=(9.5, 5.5))
    bottom = np.zeros(len(families))
    for name in directions:
        values = np.array([summary["by_family"][family]["ls_direction_counts"].get(name, 0) for family in families])
        ax.bar(families, values, bottom=bottom, label=name.replace("_", " "), color=colors[name])
        bottom += values
    ax.set_ylabel("Baseline robust failures")
    ax.set_title("Combined LS direction for the 16 unchanged failure labels")
    ax.legend(loc="upper right")
    ax.grid(axis="y", alpha=0.25)
    ax.set_axisbelow(True)
    ax.tick_params(axis="x", rotation=20)
    fig.tight_layout()
    fig.savefig(FIGURES / "eb04t-failure-family-directions.svg", format="svg", metadata={"Date": None})
    plt.close(fig)
    normalize_generated_text(FIGURES / "eb04t-failure-family-directions.svg")

    timing = [row for row in rows if row["family"] in {"peak_timing", "meltout_timing"}]
    fig, ax = plt.subplots(figsize=(10.0, 5.8))
    x = np.arange(len(timing))
    for cell, marker, color in (("B", "o", "#333333"), ("L", "s", "#377eb8"), ("S", "^", "#ff7f00"), ("LS", "D", "#984ea3")):
        ax.plot(x, [row["primary_values"][cell] for row in timing], marker=marker, label=cell, color=color)
    ax.axhline(0.0, color="black", linewidth=1.0)
    ax.set_xticks(x, [f"{row['lane_id']}\n{row['cell_id'].replace('seasonal_', '')}" for row in timing], fontsize=8)
    ax.set_ylabel("Median modeled − observed timing (days)")
    ax.set_title("Open-control timing failures remain early under LS")
    ax.legend(ncol=4, loc="lower right")
    ax.grid(alpha=0.25)
    ax.set_axisbelow(True)
    fig.tight_layout()
    fig.savefig(FIGURES / "eb04t-open-control-timing.svg", format="svg", metadata={"Date": None})
    plt.close(fig)
    normalize_generated_text(FIGURES / "eb04t-open-control-timing.svg")


def write_sidecars(rows: list[dict[str, Any]], summary: dict[str, Any]) -> None:
    sidecars = {
        "eb04t-failure-error-response": (
            "Error response of every baseline robust failure",
            "Each row is one of the 16 B failures. Cells show the percent change in the existing rubric metric's error magnitude under L, S, and LS; the fourth column is the L×S departure from an additive response. Green/negative values reduce error or mitigate the additive error; red/positive values increase it. No new threshold is applied.",
            "The combined response is predominantly away from the observation. Thirteen rows have nonzero interactions, so main-effect associations do not uniquely assign causality. One selected primary error is unchanged even though no complete metric object is identical between B and LS.",
        ),
        "eb04t-failure-family-directions": (
            "Direction of combined response by process family",
            "Counts partition the 16 unchanged fail labels by their existing primary metric's direction under LS relative to B.",
            f"LS directions are {summary['ls_direction_counts']}. The 11 density/geometry rows include two mixed, ambiguous depth-SWE guards. The five timing rows are open-control evidence about sublimation and combined-bundle promotion, not canopy-longwave identification.",
        ),
        "eb04t-open-control-timing": (
            "Modeled-minus-observed timing response in open-control failures",
            "Median modeled-minus-observed offsets are shown for B/L/S/LS. Zero is the observation target; negative values mean modeled peak or melt-out occurs too early.",
            "All five timing failures are open SNOTEL controls and remain early. They show that sublimation and the combined LS bundle do not resolve these errors; they do not identify canopy-longwave efficacy. At Niwot, S improves peak-depth timing by four days but a +4-day interaction cancels that gain in LS; for peak SWE, a −2-day interaction mitigates most of S's adverse 2.5-day shift.",
        ),
    }
    for stem, (title, caption, interpretation) in sidecars.items():
        text = f"""# {title}

## Caption

{caption}

## Data And Method

Source: committed EB-04S `retained-adjudication.json`, exact 10 independent-
validation lanes and their forcing-robust rubric cells. B/L/S/LS meanings and
observation operators remain frozen. Generated by `tools/analyze_failures.py`.

## Interpretation

{interpretation}

## Limitations

This is retained, result-aware diagnostic evidence. Exact direction does not
establish practical materiality because no materiality threshold was defined.
It cannot retroactively
change EB-04S, define a new pass threshold, prove unique causality, activate a
default, or establish warm-maritime conifer transfer. Direction is based only
on the pre-existing primary metric's exact distance from its observation target.
"""
        (FIGURES / f"{stem}.md").write_text(text, encoding="utf-8")


def write_summary(report: dict[str, Any]) -> None:
    summary = report["summary"]
    text = f"""# Unchanged-Failure Attribution

Evidence mode: `Static + Reused Ran`.

## Result

All 16 baseline forcing-robust failures remain `fail` under B/L/S/LS. No full
metric object is identical between B and LS, while one selected primary error
is unchanged. Under combined LS, exact primary-error direction is:

- toward observation: `{summary['ls_direction_counts'].get('toward_observation', 0)}`;
- unchanged: `{summary['ls_direction_counts'].get('unchanged', 0)}`;
- away from observation: `{summary['ls_direction_counts'].get('away_from_observation', 0)}`.

Eleven failures concern density trajectory or depth-SWE geometry; the two
depth-SWE rows have mixed, ambiguous density/interception ownership. Five
failures concern peak or melt-out timing, but all are open SNOTEL controls.
They are sensitive to sublimation and can block promotion of the combined LS
bundle, but they cannot identify canopy-longwave efficacy. None moves toward
the observation under LS.

## Why Failures Were Not Resolved

The retained factorial says the responses associated with S-enabled cells are
usually too small or directionally adverse relative to the dominant residuals.
It does not uniquely assign causality. Thirteen of 16 rows have nonzero L×S
interactions. At Niwot, S alone improves peak-depth timing by four days, but a
+4-day interaction cancels that improvement in LS. For peak SWE, S increases
error by 2.5 days and a −2-day interaction mitigates most of that increase.
Density KGE and forest depth-SWE slope remain unresolved density/geometry or
mixed interception-geometry debt. Prior residual authority also identifies
forcing representativeness or wind redistribution as plausible ownership for
mountain under-persistence.

## Criterion Fitness

Total robust-failure reduction was conservative and only partly aligned: 11/16
cells emphasize density/geometry or mixed interception-geometry debt. The five
open-control timing failures provide legitimate sublimation and LS-bundle
noninferiority evidence, but no canopy-longwave identification. Their lack of
improvement supports EB-04S nonpromotion of the combined default; it does not
show that canopy longwave itself failed. Exact direction is descriptive, not a
claim of material degradation, because no materiality threshold was frozen.

## Recommended Route

1. Treat LS as implemented and physically qualified with narrow empirical
   canopy benefit, still default-off.
2. Do not tune longwave/sublimation to repair density or early-persistence
   residuals.
3. If future promotion is desired, prospectively separate canopy-longwave
   efficacy, sublimation efficacy, and adjacent-process noninferiority, using
   independent evidence that can identify each mechanism.
4. Route density/geometry and mountain under-persistence to their owning snow
   process/forcing investigations before another promotion campaign.
5. Proceed to EB-05 assurance with this attribution and explicit support limits.
"""
    (ARTIFACTS / "scientific-synthesis.md").write_text(text, encoding="utf-8")


def self_check() -> None:
    if not all(path.is_file() for path in INPUTS):
        raise RuntimeError("required retained/authority input missing")
    report = read_json(EB04S_REPORT)
    rows = reconstruct_rows(report)
    summary = summarize(rows)
    if summary["failure_count"] != 16:
        raise RuntimeError("self-check did not recover exact failure inventory")
    mutated = json.loads(json.dumps(report))
    first = next(lane for lane in mutated["lanes"] if lane["role"] == "INDEPENDENT_VALIDATION")
    first_b = next(item for item in first["cells"]["B"]["rubric_profile"]["cells"] if item["forcing_robust"])
    first_b["ordinal_label"] = "fail" if first_b["ordinal_label"] != "fail" else "strong"
    try:
        summarize(reconstruct_rows(mutated))
    except RuntimeError:
        pass
    else:
        raise RuntimeError("failure-inventory mutation was not rejected")


def analyze() -> dict[str, Any]:
    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    report = read_json(EB04S_REPORT)
    rows = reconstruct_rows(report)
    summary = summarize(rows)
    result = {
        "schema": "snow-surface-eb04t-unchanged-failure-attribution-v1",
        "evidence_class": "Static + Reused Ran",
        "diagnostic_only": True,
        "model_subprocesses_launched": 0,
        "eb04s_outcome_changed": False,
        "default_activation_authorized": False,
        "inputs": {rel(path): sha256(path) for path in INPUTS},
        "summary": summary,
        "failures": rows,
    }
    (ARTIFACTS / "failure-attribution.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    write_csv(rows)
    make_figures(rows, summary)
    write_sidecars(rows, summary)
    write_summary(result)
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--self-check", action="store_true")
    group.add_argument("--analyze", action="store_true")
    args = parser.parse_args()
    if args.self_check:
        self_check()
        print("EB-04T frozen inventory and rejected-alias self-check: PASS")
        return 0
    result = analyze()
    print(json.dumps(result["summary"], indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
