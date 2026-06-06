#!/usr/bin/env python3
"""Classify HPHYS0306 melt-call mask gaps by branch-activation source lane."""

from __future__ import annotations

import json
from collections import Counter
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[4]
PACKAGE_DIR = Path(__file__).resolve().parents[1]
ARTIFACT_DIR = PACKAGE_DIR / "artifacts"
HPHYS0306_LEDGER = (
    REPO
    / "docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/branch-active-melt-term-ledger.json"
)
FIXED_COMMIT = "47ac4c32faeea81bb99081f955a14c38b815ef4d"
BASELINE_WINTER = "/workdir/wepp-forest_260430_baseline/src/winter.for"
BASELINE_SNOWD = "/workdir/wepp-forest_260430_baseline/src/snowd.for"
OPENWEPP_BRANCH_SOURCE = (
    "crates/openwepp-hillslope-orchestrator/src/hydrology/"
    "03_kernel_support_00_support_helpers.rs"
)


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def classify_row(row: dict[str, Any]) -> tuple[str, str, str, bool]:
    baseline_extra = int(row["baseline_only_active_count"])
    openwepp_extra = int(row["openwepp_only_active_count"])
    source = str(row["first_divergent_source"])

    if int(row.get("branch_active_conflict_count", 0)) > 0:
        return (
            "trace-parser-conflict",
            "trace-parser-conflict-hold",
            "Repair trace authority selection before branch activation evidence is usable.",
            False,
        )
    if baseline_extra > 0 and openwepp_extra > 0:
        return (
            "bidirectional-melt-call-mask",
            "bidirectional-melt-call-mask-hold",
            "Both baseline-extra and openWEPP-extra active keys exist; inspect source predicates before edits.",
            False,
        )
    if baseline_extra > 0:
        return (
            "baseline-extra-melt-call",
            "baseline-extra-melt-call-hold",
            (
                "Baseline reached melt.for for at least one key where openWEPP "
                "published snow_hourly_melt_branch_active=false; inspect and port "
                "baseline winter/snowd branch predicates before numeric term edits."
            ),
            False,
        )
    if openwepp_extra > 0:
        return (
            "openwepp-extra-melt-call",
            "openwepp-extra-melt-call-hold",
            (
                "openWEPP published branch-active for at least one key absent from "
                "fixed-baseline melt.for observations; inspect state ordering and "
                "baseline snowd branch predicates before edits."
            ),
            False,
        )
    if source.startswith("same-hour-multi-source"):
        return (
            "matched-branch-active-same-hour-multi-source",
            "same-hour-multi-source-hold",
            "Branch-active masks match; open same-hour source-ordering work for cmelt/snodpt.",
            False,
        )
    return (
        "matched-branch-active-requires-term-state-continuation",
        "matched-branch-active-term-state-hold",
        "Branch-active masks match; continue on first active-domain term/state source.",
        False,
    )


def build_ledger() -> list[dict[str, Any]]:
    upstream = read_json(HPHYS0306_LEDGER)
    ledger: list[dict[str, Any]] = []
    for row in upstream:
        classification, route, next_action, authorized = classify_row(row)
        ledger.append(
            {
                "hillslope_id": row["hillslope_id"],
                "window": row["window"],
                "baseline_extra_count": row["baseline_only_active_count"],
                "baseline_extra_examples": row["baseline_only_active_examples"],
                "openwepp_extra_count": row["openwepp_only_active_count"],
                "openwepp_extra_examples": row["openwepp_only_active_examples"],
                "shared_active_count": row["shared_active_count"],
                "upstream_branch_active_status": row["branch_active_status"],
                "upstream_first_divergent_source": row["first_divergent_source"],
                "source_classification": classification,
                "route": route,
                "required_next_action": next_action,
                "production_edit_authorized": authorized,
                "upstream_source_ledger": "HPHYS0306 branch-active melt-term ledger",
                "fixed_comparator_commit": FIXED_COMMIT,
                "baseline_branch_sources": [BASELINE_WINTER, BASELINE_SNOWD],
                "openwepp_branch_source": OPENWEPP_BRANCH_SOURCE,
            }
        )
    return ledger


def write_source_lineage() -> None:
    path = ARTIFACT_DIR / "melt-call-branch-activation-source-lineage.md"
    path.write_text(
        """# HPHYS0307 Melt-Call Branch Activation Source Lineage

Static:

- Fixed comparator commit: `47ac4c32faeea81bb99081f955a14c38b815ef4d`
- Baseline winter driver: `/workdir/wepp-forest_260430_baseline/src/winter.for`
- Baseline snow driver: `/workdir/wepp-forest_260430_baseline/src/snowd.for`
- openWEPP branch publisher: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`

## Baseline Predicate

- `/workdir/wepp-forest_260430_baseline/src/winter.for:366-373` calls
  `call snowd(iresd(1,iplane),denh2o,iplane,driftf,driftg,snodep,densgy,densgt,smelt,hour)`
  for each winter hour and then publishes `hrmlt(hour,iplane) = wmelt(iplane)`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:70-90` handles the
  no-existing-snowpack lanes without `call melt`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:116-174` handles the
  freezing daily-mean branch and new-snow/drift accumulation without
  `call melt`.
- `/workdir/wepp-forest_260430_baseline/src/snowd.for:180-193` enters the
  non-freezing daily-mean existing-snowpack branch and calls
  `call melt(irtype,wrain,hour)` when `snodep .gt. 0.0`.

## openWEPP Predicate

- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3887`
  initializes `melt_branch_active = 0.0`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3925-3936`
  enters the non-freezing snowpack branch and requires `snodep > WB11_ZERO_THRESHOLD`.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs:3937-3949`
  invokes `compute_simimpl29_melt_hour` and then publishes
  `melt_branch_active = 1.0`.

## Classification Rule

- `baseline-extra-melt-call`: fixed baseline reached `melt.for` for keys where
  openWEPP published inactive.
- `openwepp-extra-melt-call`: openWEPP published active keys with no paired
  fixed-baseline `melt.for` observation.
- `matched-branch-active-same-hour-multi-source`: active masks match but the
  first active-domain divergence has multiple sources.

No production code edit is authorized by classification alone; source-line proof
must identify an openWEPP branch-predicate defect first.
""",
        encoding="utf-8",
    )


def write_summary(ledger: list[dict[str, Any]]) -> None:
    classification_counts = Counter(row["source_classification"] for row in ledger)
    route_counts = Counter(row["route"] for row in ledger)
    lines = [
        "# HPHYS0307 Melt-Call Branch Activation Summary",
        "",
        "Ran:",
        "",
        f"- Fixed comparator commit: `{FIXED_COMMIT}`",
        f"- Ledger rows: `{len(ledger)}`",
        f"- Production edit authorized rows: `{sum(1 for row in ledger if row['production_edit_authorized'])}`",
        "",
        "| Hill | Window | Baseline Extra | openWEPP Extra | Shared | Classification | Route |",
        "| --- | --- | ---: | ---: | ---: | --- | --- |",
    ]
    for row in ledger:
        lines.append(
            "| {hill} | {window} | {baseline_extra} | {openwepp_extra} | {shared} | {classification} | {route} |".format(
                hill=row["hillslope_id"],
                window=row["window"],
                baseline_extra=row["baseline_extra_count"],
                openwepp_extra=row["openwepp_extra_count"],
                shared=row["shared_active_count"],
                classification=row["source_classification"],
                route=row["route"],
            )
        )
    lines.extend(["", "## Classification Counts", ""])
    lines.extend(f"- `{key}`: `{classification_counts[key]}`" for key in sorted(classification_counts))
    lines.extend(["", "## Route Counts", ""])
    lines.extend(f"- `{key}`: `{route_counts[key]}`" for key in sorted(route_counts))
    (ARTIFACT_DIR / "melt-call-branch-activation-summary.md").write_text(
        "\n".join(lines) + "\n",
        encoding="utf-8",
    )


def write_method() -> None:
    (ARTIFACT_DIR / "melt-call-branch-activation-method.md").write_text(
        """# HPHYS0307 Melt-Call Branch Activation Method

Ran:

- Loaded HPHYS0306 `branch-active-melt-term-ledger.json`.
- Preserved fixed comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
- Compared upstream `baseline_only_active_count` and
  `openwepp_only_active_count` for each H1/H7/H39 target window.
- Classified branch mask gaps as baseline-extra, openWEPP-extra, matched
  same-hour multi-source, or parser conflict lanes.
- Kept `production_edit_authorized=false` for every row because this package
  produced classification/source-lineage evidence only and did not prove an
  implementation target defect.

Static:

- Baseline branch predicate provenance is recorded in
  `melt-call-branch-activation-source-lineage.md`.
""",
        encoding="utf-8",
    )


def main() -> None:
    ledger = build_ledger()
    write_json(ARTIFACT_DIR / "melt-call-branch-activation-ledger.json", ledger)
    write_summary(ledger)
    write_method()
    write_source_lineage()
    print(json.dumps({"rows": len(ledger), "classification_counts": Counter(row["source_classification"] for row in ledger)}, sort_keys=True, default=dict))


if __name__ == "__main__":
    main()
