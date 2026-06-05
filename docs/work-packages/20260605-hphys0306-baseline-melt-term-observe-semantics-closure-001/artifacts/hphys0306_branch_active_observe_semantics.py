#!/usr/bin/env python3
"""Classify HPHYS0305 melt-term evidence on branch-active observe domains."""

from __future__ import annotations

import importlib.util
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0305_ARTIFACT_DIR = (
    REPO
    / "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts"
)
HPHYS0305_RUNNER = HPHYS0305_ARTIFACT_DIR / "hphys0305_paired_melt_term_state.py"
FIXED_COMMIT = "47ac4c32faeea81bb99081f955a14c38b815ef4d"
TRACE_AUTHORITY_BOUNDARY = "post_wb13"
TRACE_AUTHORITY_PHASE = None


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HPHYS0305 = load_module(HPHYS0305_RUNNER, "hphys0305_paired_melt_term_state")


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def load_openwepp_trace_with_branch(
    path: Path,
) -> tuple[
    dict[tuple[int, int, int], dict[str, float]],
    dict[tuple[int, int, int], bool],
    int,
    int,
]:
    parsed: dict[tuple[int, int, int], dict[str, float]] = defaultdict(dict)
    branch_active: dict[tuple[int, int, int], bool] = {}
    branch_conflicts = 0
    selected_rows = 0
    if not path.exists():
        return parsed, branch_active, branch_conflicts, selected_rows
    for line in path.read_text(encoding="utf-8", errors="ignore").splitlines():
        row = json.loads(line)
        if (
            row.get("boundary") != TRACE_AUTHORITY_BOUNDARY
            or row.get("phase") != TRACE_AUTHORITY_PHASE
        ):
            continue
        selected_rows += 1
        year = int(row.get("calendar_year", row.get("simulation_year")))
        day = int(row["julian_day"])
        active_values = row.get("snow_hourly_melt_branch_active", {})
        if isinstance(active_values, dict):
            for hour_key, value in active_values.items():
                key = (year, day, int(hour_key))
                active = bool(value)
                previous = branch_active.get(key)
                if previous is not None and previous != active:
                    branch_conflicts += 1
                branch_active[key] = active
        for symbol, field in HPHYS0305.OPENWEPP_FIELDS.items():
            values = row.get(field)
            if not isinstance(values, dict):
                continue
            for hour_key, value in values.items():
                parsed[(year, day, int(hour_key))][symbol] = float(value)
    return parsed, branch_active, branch_conflicts, selected_rows


def window_keys(year: int, start: int, end: int) -> set[tuple[int, int, int]]:
    return {(year, day, hour) for day in range(start, end + 1) for hour in range(1, 25)}


def compare_symbol(
    symbol: str,
    domain: set[tuple[int, int, int]],
    baseline: dict[tuple[int, int, int], dict[str, float]],
    openwepp: dict[tuple[int, int, int], dict[str, float]],
) -> dict[str, Any]:
    missing_baseline = 0
    missing_openwepp = 0
    paired = 0
    baseline_sum = 0.0
    openwepp_sum = 0.0
    max_abs_delta = 0.0
    first_delta: dict[str, Any] | None = None
    tolerance = HPHYS0305.SYMBOL_TOLERANCE[symbol]
    for key in sorted(domain):
        base_present = symbol in baseline.get(key, {})
        open_present = symbol in openwepp.get(key, {})
        if not base_present:
            missing_baseline += 1
            continue
        if not open_present:
            missing_openwepp += 1
            continue
        base_value = baseline[key][symbol]
        open_value = openwepp[key][symbol]
        delta = base_value - open_value
        paired += 1
        baseline_sum += base_value
        openwepp_sum += open_value
        max_abs_delta = max(max_abs_delta, abs(delta))
        if first_delta is None and abs(delta) > tolerance:
            first_delta = {
                "year": key[0],
                "julian": key[1],
                "hour": key[2],
                "baseline": round(base_value, 9),
                "openwepp": round(open_value, 9),
                "delta": round(delta, 9),
            }
    return {
        "symbol": symbol,
        "openwepp_field": HPHYS0305.OPENWEPP_FIELDS[symbol],
        "unit": HPHYS0305.SYMBOL_UNITS[symbol],
        "tolerance": tolerance,
        "baseline_count": len(domain) - missing_baseline,
        "openwepp_count": len(domain) - missing_openwepp,
        "paired_count": paired,
        "missing_baseline_count": missing_baseline,
        "missing_openwepp_count": missing_openwepp,
        "baseline_sum": round(baseline_sum, 9),
        "openwepp_sum": round(openwepp_sum, 9),
        "delta_sum": round(baseline_sum - openwepp_sum, 9),
        "max_abs_delta": round(max_abs_delta, 9),
        "first_delta": first_delta,
        "baseline_source": HPHYS0305.BASELINE_SOURCES[symbol],
        "openwepp_source": HPHYS0305.OPENWEPP_SOURCE,
    }


def classify(
    baseline_only: set[tuple[int, int, int]],
    openwepp_only: set[tuple[int, int, int]],
    comparisons: dict[str, dict[str, Any]],
    branch_conflicts: int,
) -> tuple[str, str, str]:
    if branch_conflicts:
        return (
            "trace-parser-conflict",
            "branch-active-parser-conflict-hold",
            "selected openWEPP trace authority snapshot has conflicting branch-active values",
        )
    if baseline_only or openwepp_only:
        return (
            "melt-call-mask",
            "branch-active-mask-hold",
            "baseline melt-call keys differ from openWEPP snow_hourly_melt_branch_active keys",
        )
    for symbol in HPHYS0305.TARGET_SYMBOLS:
        item = comparisons[symbol]
        if item["missing_baseline_count"] or item["missing_openwepp_count"]:
            return (
                f"paired-active-surface-gap:{symbol}",
                "active-surface-gap-hold",
                f"paired branch-active baseline/openWEPP surface is incomplete for {symbol}",
            )
    symbol_groups = {
        symbol: group
        for group, symbols in HPHYS0305.DEPENDENCY_ORDER
        for symbol in symbols
    }
    first_deltas: list[tuple[tuple[int, int, int], str]] = []
    for symbol in HPHYS0305.TARGET_SYMBOLS:
        first_delta = comparisons[symbol]["first_delta"]
        if first_delta is None:
            continue
        first_deltas.append(
            (
                (
                    int(first_delta["year"]),
                    int(first_delta["julian"]),
                    int(first_delta["hour"]),
                ),
                symbol,
            )
        )
    if first_deltas:
        first_key = min(key for key, _symbol in first_deltas)
        first_symbols = sorted(symbol for key, symbol in first_deltas if key == first_key)
        if len(first_symbols) > 1:
            joined = ",".join(first_symbols)
            return (
                f"same-hour-multi-source:{joined}",
                "same-hour-multi-source-hold",
                f"first chronological branch-active divergence has multiple symbols at {first_key}: {joined}",
            )
        symbol = first_symbols[0]
        group = symbol_groups[symbol]
        return (
            f"{group}:{symbol}",
            f"{group}-hold",
            f"first chronological same-unit branch-active divergence is {symbol} in {group}",
        )
    return (
        "branch-active-term-state-within-tolerance",
        "branch-active-term-state-closed-return-to-post-raw",
        "branch-active HPHYS0306 term/state surfaces are within declared tolerances",
    )


def required_next_action(source: str) -> str:
    if source == "melt-call-mask":
        return (
            "Diagnose and correct/openly adjudicate openWEPP winter melt-call branch "
            "activation against fixed-baseline melt.for before numeric term corrections."
        )
    if source == "trace-parser-conflict":
        return (
            "Repair trace authority boundary selection before using branch-active "
            "mask or term-state evidence."
        )
    if source.startswith("same-hour-multi-source"):
        return (
            "Open a same-hour source-ordering package to separate melt-term and "
            "snow-state divergence at the first active-domain timestamp."
        )
    if source.startswith("paired-active-surface-gap"):
        return (
            "Repair paired active-domain instrumentation or record a typed HOLD; do not zero-impute "
            "inactive fixed-baseline melt terms."
        )
    if source.startswith("hourly-forcing"):
        return "Open a forcing-source package for the first active-domain forcing divergence."
    if source.startswith("snow-state"):
        return "Open a snow-state carry package for the first active-domain snow-state divergence."
    if source.startswith("melt-terms"):
        return "Open a melt-term producer package for the first active-domain melt-term divergence."
    return "Return to post-raw/routed melt lineage only after branch-active term-state evidence closes."


def first_keys(keys: set[tuple[int, int, int]], limit: int = 5) -> list[dict[str, int]]:
    return [
        {"year": year, "julian": day, "hour": hour}
        for year, day, hour in sorted(keys)[:limit]
    ]


def build_ledger() -> list[dict[str, Any]]:
    identity = read_json(HPHYS0305_ARTIFACT_DIR / "baseline-observe-identity.json")
    trace_audit = read_json(HPHYS0305_ARTIFACT_DIR / "openwepp-trace-field-audit.json")
    trace_paths = {int(item["hillslope_id"]): Path(item["trace_path"]) for item in trace_audit}
    ledger: list[dict[str, Any]] = []
    for hill in HPHYS0305.TARGET_HILLS:
        hill_identity = identity[str(hill)]
        if hill_identity["fixed_commit"] != FIXED_COMMIT:
            raise RuntimeError(
                f"H{hill} fixed comparator {hill_identity['fixed_commit']} != {FIXED_COMMIT}"
            )
        observe_log = Path(hill_identity["observe_on_lane"]["observe_log"])
        baseline = HPHYS0305.parse_baseline_log(observe_log)
        (
            openwepp,
            openwepp_branch_active,
            branch_conflicts,
            selected_rows,
        ) = load_openwepp_trace_with_branch(trace_paths[hill])
        for window_name, year, start, end in HPHYS0305.TARGET_WINDOWS[hill]:
            keys = window_keys(year, start, end)
            baseline_active_keys = {
                key for key in keys if "amelt" in baseline.get(key, {})
            }
            openwepp_active_keys = {
                key for key in keys if openwepp_branch_active.get(key, False)
            }
            active_domain = baseline_active_keys | openwepp_active_keys
            baseline_only = baseline_active_keys - openwepp_active_keys
            openwepp_only = openwepp_active_keys - baseline_active_keys
            comparisons = {
                symbol: compare_symbol(symbol, active_domain, baseline, openwepp)
                for symbol in HPHYS0305.TARGET_SYMBOLS
            }
            source, route, reason = classify(
                baseline_only, openwepp_only, comparisons, branch_conflicts
            )
            paired_complete = all(
                item["missing_baseline_count"] == 0 and item["missing_openwepp_count"] == 0
                for item in comparisons.values()
            )
            branch_status = (
                "branch-active-mask-closed"
                if not baseline_only and not openwepp_only
                else "branch-active-mask-gap"
            )
            ledger.append(
                {
                    "hillslope_id": hill,
                    "window": window_name,
                    "year": year,
                    "start_julian": start,
                    "end_julian": end,
                    "fixed_comparator_commit": FIXED_COMMIT,
                    "baseline_active_count": len(baseline_active_keys),
                    "openwepp_active_count": len(openwepp_active_keys),
                    "shared_active_count": len(baseline_active_keys & openwepp_active_keys),
                    "baseline_only_active_count": len(baseline_only),
                    "openwepp_only_active_count": len(openwepp_only),
                    "baseline_only_active_examples": first_keys(baseline_only),
                    "openwepp_only_active_examples": first_keys(openwepp_only),
                    "branch_active_conflict_count": branch_conflicts,
                    "trace_authority_boundary": TRACE_AUTHORITY_BOUNDARY,
                    "trace_authority_phase": TRACE_AUTHORITY_PHASE,
                    "trace_authority_selected_row_count": selected_rows,
                    "branch_active_status": branch_status,
                    "paired_surface_status": (
                        "paired-active-complete"
                        if paired_complete
                        else "paired-active-surface-gap"
                    ),
                    "first_divergent_source": source,
                    "route": route,
                    "classification_reason": reason,
                    "production_edit_authorized": False,
                    "prohibited_compensation_note": (
                        "No WB13/WB17/WB18/WB19/WB12 compensation is authorized by "
                        "branch-active observe diagnostics."
                    ),
                    "required_next_action": required_next_action(source),
                    "zero_imputation_policy": "no-zero-impute-inactive-baseline-melt-terms",
                    "comparisons": comparisons,
                }
            )
    return ledger


def write_summary(ledger: list[dict[str, Any]]) -> None:
    lines = [
        "# HPHYS0306 Branch-Active Melt-Term Observe Summary",
        "",
        "Ran:",
        "",
        f"- Fixed comparator commit: `{FIXED_COMMIT}`",
        f"- Ledger rows: `{len(ledger)}`",
        f"- Production edit authorized rows: `{sum(1 for row in ledger if row['production_edit_authorized'])}`",
        "",
        "| Hill | Window | Active Baseline | Active openWEPP | Shared | Baseline Only | openWEPP Only | Status | First Source | Route |",
        "| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | --- |",
    ]
    for row in ledger:
        lines.append(
            "| H{hillslope_id} | {window} | {baseline_active_count} | "
            "{openwepp_active_count} | {shared_active_count} | "
            "{baseline_only_active_count} | {openwepp_only_active_count} | "
            "{branch_active_status} | {first_divergent_source} | {route} |".format(
                **row
            )
        )
    lines.extend(["", "## Counts", ""])
    for label, counter in [
        ("Branch Active Status", Counter(row["branch_active_status"] for row in ledger)),
        ("First Source", Counter(row["first_divergent_source"] for row in ledger)),
        ("Route", Counter(row["route"] for row in ledger)),
    ]:
        lines.extend([f"### {label}", "", "| Value | Count |", "| --- | ---: |"])
        for value, count in sorted(counter.items()):
            lines.append(f"| {value} | {count} |")
        lines.append("")
    (ARTIFACT_DIR / "branch-active-melt-term-summary.md").write_text(
        "\n".join(lines).rstrip() + "\n", encoding="utf-8"
    )


def write_method() -> None:
    (ARTIFACT_DIR / "branch-active-observe-method.md").write_text(
        "\n".join(
            [
                "# HPHYS0306 Branch-Active Observe Method",
                "",
                "Static:",
                "",
                "- Baseline active keys are fixed-comparator `melt.for` observe keys where `amelt` was emitted.",
                "- openWEPP active keys are `snow_hourly_melt_branch_active == true` keys from the final `post_wb13` daily trace snapshot.",
                "- Inactive fixed-baseline hours are not zero-imputed.",
                "- Numeric forcing, snow-state, and melt-term comparisons are interpreted only after active-mask comparison and selected-snapshot conflict checks.",
                "- Any active-mask mismatch routes to `branch-active-mask-hold` before numeric term correction.",
                "- Any selected-snapshot branch-active conflict routes to `branch-active-parser-conflict-hold`.",
                "- Numeric first-source classification is chronological; same-hour multi-symbol divergences remain HOLD.",
            ]
        )
        + "\n",
        encoding="utf-8",
    )


def main() -> None:
    ledger = build_ledger()
    write_json(ARTIFACT_DIR / "branch-active-melt-term-ledger.json", ledger)
    write_summary(ledger)
    write_method()


if __name__ == "__main__":
    main()
