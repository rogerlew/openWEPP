#!/usr/bin/env python3
"""Verify quality-observatory evidence and derive CQR module selection."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import os
import re
import subprocess
import sys
import tempfile
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Any


SCHEMA = "openwepp-cqr-quality-intake-v1"
AUTHORIZATION_SCHEMA = "openwepp-cqr-recollection-authorization-v1"
SHA256 = re.compile(r"^[0-9a-f]{64}$")
DIRECTIVE = re.compile(
    r"^execute cqr nightly(?: for [1-9][0-9]* modules)?$", re.IGNORECASE
)


class IntakeError(RuntimeError):
    """Raised when CQR evidence intake cannot be completed safely."""


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value,
        allow_nan=False,
        ensure_ascii=False,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.tmp")
    temporary.write_bytes(canonical_bytes(value) + b"\n")
    temporary.replace(path)


def ensure_safe_new_output(path: Path, forbidden_roots: tuple[Path, ...]) -> Path:
    absolute = path.absolute()
    parent = absolute.parent
    if not parent.is_dir() or parent.is_symlink():
        raise IntakeError("output parent must be an existing real directory")
    current = parent
    while True:
        if current.is_symlink():
            raise IntakeError("output path contains a symlink component")
        if current == current.parent:
            break
        current = current.parent
    if absolute.exists() or absolute.is_symlink():
        raise IntakeError("output must be a fresh path")
    for root in forbidden_roots:
        resolved_root = root.resolve()
        if absolute == resolved_root or absolute.is_relative_to(resolved_root):
            raise IntakeError("output must be outside evidence input roots")
    return absolute


def write_json_new(path: Path, value: Any, forbidden_roots: tuple[Path, ...]) -> None:
    absolute = ensure_safe_new_output(path, forbidden_roots)
    with tempfile.NamedTemporaryFile(
        mode="wb",
        dir=absolute.parent,
        prefix=".cqr-output-",
        delete=False,
    ) as stream:
        temporary = Path(stream.name)
        stream.write(canonical_bytes(value) + b"\n")
        stream.flush()
        os.fsync(stream.fileno())
    try:
        os.link(temporary, absolute)
    except FileExistsError as error:
        raise IntakeError("output path was created concurrently") from error
    finally:
        temporary.unlink(missing_ok=True)


def read_object(path: Path) -> dict[str, Any]:
    try:
        raw = path.read_bytes()
        value = json.loads(raw)
    except (OSError, json.JSONDecodeError) as error:
        raise IntakeError(f"cannot read canonical JSON {path}: {error}") from error
    if not isinstance(value, dict):
        raise IntakeError(f"JSON root is not an object: {path}")
    if raw != canonical_bytes(value) + b"\n":
        raise IntakeError(f"JSON is not canonical: {path}")
    return value


def load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise IntakeError(f"cannot load repository verifier: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def git_text(repo: Path, *arguments: str) -> str:
    result = subprocess.run(
        ["git", *arguments],
        cwd=repo,
        check=False,
        capture_output=True,
        text=True,
        timeout=10,
    )
    if result.returncode:
        raise IntakeError(result.stderr.strip() or "Git identity command failed")
    return result.stdout.strip()


def production_path(value: Any) -> str:
    if not isinstance(value, str):
        raise IntakeError("CRAP row file is not a string")
    path = Path(value).as_posix()
    parts = path.split("/")
    if (
        path != value
        or len(parts) < 4
        or parts[0] != "crates"
        or parts[2] != "src"
        or not path.endswith(".rs")
        or any(part in {"", ".", ".."} for part in parts)
    ):
        raise IntakeError(f"CRAP row is outside production Rust modules: {value}")
    return path


def row_key(row: dict[str, Any]) -> tuple[Any, ...]:
    fields = ("crate", "file", "function", "line", "cyclomatic", "coverage", "crap")
    if set(row) not in (
        set(fields),
        set(fields) | {"adjudication_id"},
    ):
        raise IntakeError("CRAP row fields are not exact")
    try:
        crate, file_name, function, line, cyclomatic, coverage, crap = (
            row[field] for field in fields
        )
    except KeyError as error:
        raise IntakeError(f"CRAP row lacks {error.args[0]}") from error
    if (
        not isinstance(crate, str)
        or not crate
        or not isinstance(function, str)
        or not function
        or isinstance(line, bool)
        or not isinstance(line, int)
        or line < 1
        or isinstance(cyclomatic, bool)
        or not isinstance(cyclomatic, (int, float))
        or not math.isfinite(float(cyclomatic))
        or coverage is not None
        and (
            isinstance(coverage, bool)
            or not isinstance(coverage, (int, float))
            or not math.isfinite(float(coverage))
        )
        or isinstance(crap, bool)
        or not isinstance(crap, (int, float))
        or not math.isfinite(float(crap))
        or float(crap) <= 30.0
    ):
        raise IntakeError("CRAP row contains invalid values")
    production_path(file_name)
    return (
        file_name,
        function,
        line,
        cyclomatic,
        coverage,
        crap,
    )


def reconstruct_selection(
    repo: Path,
    report: dict[str, Any],
    quality_module: Any,
    limit: int | None,
) -> dict[str, Any]:
    raw = report.get("raw_over_threshold")
    published_adjudicated = report.get("adjudicated")
    published_actionable = report.get("actionable")
    if not all(
        isinstance(rows, list)
        for rows in (raw, published_adjudicated, published_actionable)
    ):
        raise IntakeError("report lacks exact raw/adjudicated/actionable rows")
    raw_rows = [dict(row) for row in raw if isinstance(row, dict)]
    if len(raw_rows) != len(raw):
        raise IntakeError("raw CRAP rows contain a non-object")
    raw_keys = [row_key(row) for row in raw_rows]
    if len(raw_keys) != len(set(raw_keys)):
        raise IntakeError("raw CRAP rows contain duplicates")

    registry_path = repo / "tools/release/adjudicated_crap_exceptions.json"
    try:
        registry = json.loads(registry_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise IntakeError(f"cannot read adjudication registry: {error}") from error
    if not isinstance(registry, dict):
        raise IntakeError("adjudication registry root is not an object")
    crap_module = quality_module.load_crap_module(repo)
    try:
        adjudications, invalid = crap_module._load_adjudications(registry, repo)
    except (OSError, ValueError) as error:
        raise IntakeError(f"current adjudication registry is invalid: {error}") from error
    if invalid:
        raise IntakeError("current adjudication registry has invalid entries")
    accepted = {
        (item["file"], item["function"], float(item["cyclomatic"])): item["id"]
        for item in adjudications
    }

    reconstructed_adjudicated: list[dict[str, Any]] = []
    reconstructed_actionable: list[dict[str, Any]] = []
    for row in raw_rows:
        key = (row["file"], row["function"], float(row["cyclomatic"]))
        adjudication_id = accepted.get(key)
        if adjudication_id is None:
            reconstructed_actionable.append(row)
        else:
            reconstructed_adjudicated.append(
                {**row, "adjudication_id": adjudication_id}
            )
    sort_key = lambda row: (row["file"], row["line"], row["function"], row["crap"])
    reconstructed_adjudicated.sort(key=sort_key)
    reconstructed_actionable.sort(key=sort_key)
    if (
        reconstructed_adjudicated != published_adjudicated
        or reconstructed_actionable != published_actionable
    ):
        raise IntakeError("current registry does not reconstruct report partitions")

    modules: dict[str, dict[str, Any]] = {}
    for row in reconstructed_actionable:
        path = production_path(row["file"])
        module = modules.setdefault(
            path,
            {
                "module": path,
                "total_excess_crap": 0.0,
                "functions": set(),
                "maximum_crap": 0.0,
                "rows": [],
            },
        )
        module["total_excess_crap"] += max(float(row["crap"]) - 30.0, 0.0)
        module["functions"].add((row["function"], row["line"]))
        module["maximum_crap"] = max(module["maximum_crap"], float(row["crap"]))
        module["rows"].append(row)
    ranking = [
        {
            "module": value["module"],
            "total_excess_crap": value["total_excess_crap"],
            "function_count": len(value["functions"]),
            "maximum_crap": value["maximum_crap"],
            "rows": value["rows"],
        }
        for value in modules.values()
    ]
    ranking.sort(
        key=lambda item: (
            -item["total_excess_crap"],
            -item["function_count"],
            -item["maximum_crap"],
            item["module"],
        )
    )
    selected = ranking if limit is None else ranking[:limit]
    return {
        "raw_count": len(raw_rows),
        "adjudicated_count": len(reconstructed_adjudicated),
        "actionable_count": len(reconstructed_actionable),
        "actionable_module_count": len(ranking),
        "candidate_ranking": ranking,
        "candidate_selection": selected,
        "selection_review_status": "REQUIRED",
    }


def stale_reasons(
    repo: Path, payload: dict[str, Any], report: dict[str, Any], quality: Any
) -> list[str]:
    reasons: list[str] = []
    subject = payload.get("subject")
    crap = payload.get("crap")
    if not isinstance(subject, dict) or not isinstance(crap, dict):
        return ["identity payload is incomplete"]
    crap_module = quality.load_crap_module(repo)
    comparisons = [
        ("HEAD", payload.get("head_commit"), git_text(repo, "rev-parse", "HEAD")),
        ("source tree", subject.get("source_tree"), git_text(repo, "rev-parse", "HEAD^{tree}")),
        (
            "source manifest",
            payload.get("source_manifest_sha256"),
            quality.manifest_sha256(quality.source_manifest(repo)),
        ),
        (
            "registry",
            crap.get("registry_sha256"),
            sha256_file(repo / "tools/release/adjudicated_crap_exceptions.json"),
        ),
        (
            "collector",
            payload.get("control_inputs", {}).get("collector_sha256"),
            sha256_file(repo / "tools/local_ci/quality_observatory.py"),
        ),
        (
            "snowbench ledger",
            payload.get("control_inputs", {}).get("snowbench_ledger_sha256"),
            sha256_file(
                repo
                / "docs/work-packages/20260724-quality-observatory-merged-coverage-001"
                / "artifacts/snowbench-full-only-row-ledger.json"
            ),
        ),
        (
            "workflow digest",
            subject.get("workflow_sha256"),
            sha256_file(repo / ".github/workflows/quality-observatory.yml"),
        ),
        ("workflow revision", subject.get("workflow_revision"), git_text(repo, "rev-parse", "HEAD")),
        ("toolchain", payload.get("toolchain"), quality.identity_versions(repo)),
        ("profiles", payload.get("ordered_profiles"), list(quality.PROFILES)),
        (
            "production filter",
            report.get("production_filter"),
            crap_module.EXPECTED_FILTER,
        ),
        (
            "deduplication policy",
            report.get("deduplication_key"),
            crap_module.EXPECTED_DEDUPLICATION_KEY,
        ),
    ]
    for label, observed, current in comparisons:
        if observed != current:
            reasons.append(f"{label} differs from current repository identity")
    return reasons


def intake(args: argparse.Namespace) -> int:
    repo = args.repo.resolve()
    output = ensure_safe_new_output(
        args.output,
        (args.published_dir.absolute(), args.control_receipt.absolute().parent),
    )
    disposition = "INVALID"
    reasons: list[str] = []
    evidence_id: str | None = None
    selection: dict[str, Any] | None = None
    inputs: dict[str, Any] = {
        "published_locator": str(args.published_dir),
        "control_locator": str(args.control_receipt),
        "expected_quality_evidence_id": args.expected_id,
        "selection_limit": args.limit,
    }
    try:
        if not SHA256.fullmatch(args.expected_id):
            raise IntakeError("expected quality evidence ID is not one SHA-256")
        quality = load_module(
            repo / "tools/local_ci/quality_observatory.py",
            "openwepp_quality_observatory",
        )
        workflow = load_module(
            repo / "tools/local_ci/quality_observatory_workflow.py",
            "openwepp_quality_observatory_workflow",
        )
        quality.ensure_no_symlink_path(args.control_receipt.absolute())
        control_path = args.control_receipt.resolve()
        control = workflow.validate_control(control_path.parent)
        if (
            control_path.name != "quality-control-receipt.json"
            or control.get("disposition") != "COMPLETE"
            or control.get("quality_evidence_id") != args.expected_id
            or not isinstance(control.get("admission"), dict)
        ):
            raise IntakeError("complete control receipt identity is invalid")
        inputs["control_sha256"] = sha256_file(control_path)
        published = args.published_dir.resolve()
        payload = read_object(published / "quality-payload.json")
        evidence_id = sha256_bytes(canonical_bytes(payload))
        if evidence_id != args.expected_id:
            raise IntakeError("expected evidence ID differs from canonical payload")
        subject = payload.get("subject")
        if (
            not isinstance(subject, dict)
            or control.get("source_sha") != payload.get("head_commit")
            or control.get("source_tree") != subject.get("source_tree")
            or control.get("workflow_revision") != subject.get("workflow_revision")
            or control.get("workflow_sha256") != subject.get("workflow_sha256")
            or control.get("publication")
            != workflow.publication_manifest(published)
        ):
            raise IntakeError(
                "complete control receipt does not bind the supplied publication"
            )
        report = read_object(published / "adjudicated-crap-report.json")
        with tempfile.NamedTemporaryFile(
            mode="wb",
            dir=output.parent,
            prefix=".cqr-admission-",
            suffix=".json",
            delete=False,
        ) as stream:
            admission_path = Path(stream.name)
            stream.write(canonical_bytes(control["admission"]) + b"\n")
        try:
            internally_verified_id = quality.verify_published(
                repo,
                published,
                admission_path,
                independent_inventory=False,
                check_source=False,
                check_current_controls=False,
            )
        finally:
            admission_path.unlink(missing_ok=True)
        if internally_verified_id != evidence_id:
            raise IntakeError("historical verifier returned a different evidence ID")
        reasons = stale_reasons(repo, payload, report, quality)
        if reasons:
            disposition = "STALE"
        else:
            with tempfile.NamedTemporaryFile(
                mode="wb",
                dir=output.parent,
                prefix=".cqr-admission-",
                suffix=".json",
                delete=False,
            ) as stream:
                admission_path = Path(stream.name)
                stream.write(canonical_bytes(control["admission"]) + b"\n")
            try:
                verified_id = quality.verify_published(
                    repo,
                    published,
                    admission_path,
                    independent_inventory=True,
                    check_source=True,
                    check_current_controls=True,
                )
            finally:
                admission_path.unlink(missing_ok=True)
            if verified_id != evidence_id:
                raise IntakeError("canonical verifier returned a different evidence ID")
            selection = reconstruct_selection(repo, report, quality, args.limit)
            disposition = "CURRENT"
    except Exception as error:
        if disposition != "STALE":
            disposition = "INVALID"
            reasons = [str(error)]
    receipt = {
        "schema_version": SCHEMA,
        "disposition": disposition,
        "quality_evidence_id": evidence_id,
        "inputs": inputs,
        "reasons": reasons,
        "selection": selection,
        "collection_launched": False,
    }
    write_json_new(output, receipt, ())
    print(
        f"cqr-quality-intake: {disposition}"
        + (f" id={evidence_id}" if evidence_id else "")
    )
    return 0 if disposition == "CURRENT" else 2


def authorize_recollection(args: argparse.Namespace) -> int:
    receipt = read_object(args.intake_receipt)
    inputs = receipt.get("inputs")
    evidence_id = receipt.get("quality_evidence_id")
    if (
        set(receipt)
        != {
            "schema_version",
            "disposition",
            "quality_evidence_id",
            "inputs",
            "reasons",
            "selection",
            "collection_launched",
        }
        or receipt.get("schema_version") != SCHEMA
        or receipt.get("disposition") not in {"STALE", "INVALID"}
        or not isinstance(receipt.get("reasons"), list)
        or not receipt["reasons"]
        or receipt.get("selection") is not None
        or receipt.get("collection_launched") is not False
        or not isinstance(inputs, dict)
        or set(inputs)
        not in (
            {
                "published_locator",
                "control_locator",
                "expected_quality_evidence_id",
                "selection_limit",
            },
            {
                "published_locator",
                "control_locator",
                "expected_quality_evidence_id",
                "selection_limit",
                "control_sha256",
            },
        )
        or not all(
            isinstance(inputs.get(field), str) and inputs[field]
            for field in (
                "published_locator",
                "control_locator",
                "expected_quality_evidence_id",
            )
        )
        or not SHA256.fullmatch(inputs["expected_quality_evidence_id"])
        or (
            inputs["selection_limit"] is not None
            and (
                isinstance(inputs["selection_limit"], bool)
                or not isinstance(inputs["selection_limit"], int)
                or inputs["selection_limit"] < 1
            )
        )
        or (
            "control_sha256" in inputs
            and (
                not isinstance(inputs["control_sha256"], str)
                or not SHA256.fullmatch(inputs["control_sha256"])
            )
        )
        or (
            evidence_id is not None
            and (not isinstance(evidence_id, str) or not SHA256.fullmatch(evidence_id))
        )
        or not all(isinstance(reason, str) and reason for reason in receipt["reasons"])
        or (
            receipt.get("disposition") == "STALE"
            and (
                evidence_id is None
                or "control_sha256" not in inputs
            )
        )
    ):
        raise IntakeError("intake receipt cannot authorize recollection")
    directive = args.operator_directive.strip()
    if DIRECTIVE.fullmatch(directive) is None:
        raise IntakeError("operator directive is not an explicit CQR execution request")
    output = ensure_safe_new_output(args.output, (args.intake_receipt.absolute(),))
    with tempfile.TemporaryDirectory(
        prefix=".cqr-reinspect-", dir=output.parent
    ) as temporary:
        reproduced_path = Path(temporary) / "intake.json"
        reproduced_status = intake(
            argparse.Namespace(
                repo=args.repo,
                published_dir=Path(inputs["published_locator"]),
                control_receipt=Path(inputs["control_locator"]),
                expected_id=inputs["expected_quality_evidence_id"],
                limit=inputs["selection_limit"],
                output=reproduced_path,
            )
        )
        if reproduced_status == 0 or reproduced_path.read_bytes() != args.intake_receipt.read_bytes():
            raise IntakeError("retained intake receipt did not reproduce exactly")
    authorization = {
        "schema_version": AUTHORIZATION_SCHEMA,
        "status": "AUTHORIZED",
        "operator_directive": directive,
        "intake_receipt_sha256": sha256_file(args.intake_receipt),
        "quality_evidence_id": receipt.get("quality_evidence_id"),
        "disposition": receipt["disposition"],
        "reasons": receipt["reasons"],
    }
    write_json_new(output, authorization, ())
    print(f"cqr-recollection: AUTHORIZED disposition={receipt['disposition']}")
    return 0


def build_intake_fixture(
    repo: Path,
    root: Path,
    quality: Any,
    workflow: Any,
    *,
    subject_head: str,
    inventories: dict[str, dict[str, Any]] | None = None,
) -> tuple[Path, Path, str]:
    published = root / "published"
    control = root / "control"
    published.mkdir(parents=True)
    control.mkdir()
    if inventories is None:
        inventories = quality.independent_inventory_partition(repo)
    inventory_bindings: dict[str, Any] = {}
    junit_bindings: dict[str, Any] = {}
    for name, inventory in inventories.items():
        inventory_path = published / f"inventory-{name}.json"
        write_json(inventory_path, inventory)
        inventory_bindings[name] = {
            "count": inventory["count"],
            "identities_sha256": inventory["identities_sha256"],
            "artifact_sha256": sha256_file(inventory_path),
        }
        if name in quality.PROFILES:
            root_element = ET.Element(
                "testsuites",
                {
                    "tests": str(inventory["count"]),
                    "failures": "0",
                    "errors": "0",
                    "skipped": "0",
                },
            )
            suite = ET.SubElement(root_element, "testsuite", {"name": name})
            for identity in inventory["identities"]:
                classname, test_name = identity.rsplit("::", 1)
                ET.SubElement(
                    suite,
                    "testcase",
                    {"classname": classname, "name": test_name},
                )
            ET.indent(root_element, space="  ")
            junit_path = published / f"junit-{name}.xml"
            junit_path.write_bytes(
                b'<?xml version="1.0" encoding="UTF-8"?>\n'
                + ET.tostring(root_element, encoding="utf-8")
                + b"\n"
            )
            parsed = quality.parse_compact_junit(junit_path)
            parsed.pop("identities")
            junit_bindings[name] = {
                **parsed,
                "sha256": sha256_file(junit_path),
            }

    crap_module = quality.load_crap_module(repo)
    registry_sha = sha256_file(
        repo / "tools/release/adjudicated_crap_exceptions.json"
    )
    row = {
        "crate": "openwepp-sim-contract",
        "file": "crates/openwepp-sim-contract/src/lib.rs",
        "function": "fixture_actionable",
        "line": 1,
        "cyclomatic": 6.0,
        "coverage": 20.0,
        "crap": 42.0,
    }
    merged_lcov_sha = "1" * 64
    workspace_crap_sha = "2" * 64
    source_manifest_artifact_sha = "3" * 64
    report = {
        "schema_version": "openwepp-adjudicated-crap-report-v1",
        "status": "OBSERVATION-COMPLETE",
        "debt_status": "FAIL",
        "closure_eligible": False,
        "production_filter": crap_module.EXPECTED_FILTER,
        "deduplication_key": crap_module.EXPECTED_DEDUPLICATION_KEY,
        "raw_over_threshold_count": 1,
        "adjudicated_count": 0,
        "actionable_count": 1,
        "raw_over_threshold": [row],
        "adjudicated": [],
        "actionable": [row],
        "invalid_adjudications": [],
        "adjudication_registry_sha256": registry_sha,
        "crap_json_sha256": workspace_crap_sha,
        "lcov_sha256": merged_lcov_sha,
        "source_manifest_sha256": source_manifest_artifact_sha,
    }
    write_json(published / "adjudicated-crap-report.json", report)
    (published / "adjudicated-crap-report.md").write_text(
        "# Fixture CRAP Report\n", encoding="utf-8"
    )
    ledger_path = (
        repo
        / "docs/work-packages/20260724-quality-observatory-merged-coverage-001"
        / "artifacts/snowbench-full-only-row-ledger.json"
    )
    ledger = json.loads(ledger_path.read_text(encoding="utf-8"))
    snowbench_rows = []
    for historical in ledger["rows"]:
        measured = {
            "crate": historical["crate"],
            "file": historical["file"],
            "function": historical["function"],
            "line": historical["line"],
            "cyclomatic": historical["cyclomatic"],
            "coverage": 1.0,
        }
        snowbench_rows.append(
            {
                "historical": historical,
                "science_manual": measured,
                "merged": measured,
                "disposition": "SCIENCE_MANUAL_CONTRIBUTION",
                "science_manual_contributed": True,
                "retained_as_debt": False,
            }
        )
    write_json(
        published / "coverage-summary.json",
        {
            "schema_version": quality.COVERAGE_SCHEMA,
            "snowbench_gate_status": "PASS",
            "snowbench_ledger_sha256": sha256_file(ledger_path),
            "snowbench_rows": snowbench_rows,
        },
    )
    source_manifest = quality.source_manifest(repo)
    source_manifest_sha = quality.manifest_sha256(source_manifest)
    source_tree = git_text(repo, "rev-parse", "HEAD^{tree}")
    workflow_sha = sha256_file(repo / ".github/workflows/quality-observatory.yml")
    toolchain = quality.identity_versions(repo)
    nextest_config_sha = sha256_file(repo / ".config/nextest.toml")
    instrumented_build_id = "4" * 64
    build_identity = {
        "coverage_mode": "workspace-default-features-instrument-coverage-cfg-coverage",
        "features": [],
        "runtime_cargo_artifacts": [dict(item) for item in quality.RUNTIME_CARGO_ARTIFACTS],
        "toolchain": toolchain,
        "nextest_config_sha256": nextest_config_sha,
    }
    admission_payload = {
        "schema_version": quality.ADMISSION_SCHEMA,
        "status": "READY",
        "head_commit": subject_head,
        "source_tree": source_tree,
        "workflow_revision": subject_head,
        "workflow_sha256": workflow_sha,
        "source_manifest_sha256": source_manifest_sha,
        "ordered_profiles": list(quality.PROFILES),
        "inventories": inventory_bindings,
        "instrumented_build_id": instrumented_build_id,
        "build_identity": build_identity,
        "registry_sha256": registry_sha,
        "snowbench_ledger_sha256": sha256_file(ledger_path),
        "collector_sha256": sha256_file(
            repo / "tools/local_ci/quality_observatory.py"
        ),
        "runner": "fixture",
        "workflow": "quality-observatory",
        "run_id": "fixture",
        "run_attempt": "1",
    }
    admission_id = sha256_bytes(canonical_bytes(admission_payload))
    admission = {"admission_id": admission_id, "payload": admission_payload}
    artifact_names = quality.PUBLISHED_FILES - {
        "quality-envelope.json",
        "quality-payload.json",
        "run-status.json",
    }
    payload = {
        "schema_version": quality.SCHEMA,
        "head_commit": subject_head,
        "source_manifest_sha256": source_manifest_sha,
        "instrumented_build_id": instrumented_build_id,
        "ordered_profiles": list(quality.PROFILES),
        "coverage_mode": build_identity["coverage_mode"],
        "features": [],
        "runtime_cargo_artifacts": build_identity["runtime_cargo_artifacts"],
        "toolchain": toolchain,
        "inventories": inventory_bindings,
        "junit": junit_bindings,
        "crap": {
            "registry_sha256": registry_sha,
            "workspace_crap_sha256": workspace_crap_sha,
            "source_manifest_artifact_sha256": source_manifest_artifact_sha,
            "raw_count": 1,
            "adjudicated_count": 0,
            "actionable_count": 1,
        },
        "coverage": {"merged_lcov_sha256": merged_lcov_sha},
        "control_inputs": {
            "registry_sha256": registry_sha,
            "snowbench_ledger_sha256": sha256_file(ledger_path),
            "collector_sha256": admission_payload["collector_sha256"],
            "nextest_config_sha256": nextest_config_sha,
        },
        "execution": {
            "runner": "fixture",
            "workflow": "quality-observatory",
            "run_id": "fixture",
            "run_attempt": "1",
        },
        "subject": {
            "source_commit": subject_head,
            "source_tree": source_tree,
            "workflow_revision": subject_head,
            "workflow_sha256": workflow_sha,
            "current_main": subject_head == git_text(repo, "rev-parse", "HEAD"),
        },
        "admission_id": admission_id,
        "closure_eligible": False,
        "artifacts": quality.artifact_digest_map(published, artifact_names),
    }
    write_json(published / "quality-payload.json", payload)
    evidence_id = sha256_bytes(canonical_bytes(payload))
    write_json(
        published / "run-status.json",
        {
            "schema_version": quality.SCHEMA,
            "execution_integrity": "PASS",
            "debt_status": "FAIL",
            "closure_eligible": False,
            "admission_id": admission_id,
            "quality_evidence_id": evidence_id,
        },
    )
    envelope_files = quality.PUBLISHED_FILES - {"quality-envelope.json"}
    write_json(
        published / "quality-envelope.json",
        {
            "schema_version": quality.ENVELOPE_SCHEMA,
            "quality_evidence_id": evidence_id,
            "payload": payload,
            "publication": {
                "allowed_files": sorted(quality.PUBLISHED_FILES),
                "max_total_bytes": quality.MAX_PUBLISHED_BYTES,
                "files": quality.artifact_digest_map(published, envelope_files),
            },
        },
    )
    workflow.control_receipt(
        control,
        disposition="COMPLETE",
        source_sha=subject_head,
        source_tree=source_tree,
        workflow_revision=subject_head,
        workflow_sha256=workflow_sha,
        occupancy={"status": "CLEAR"},
        child_exit=0,
        publication=workflow.publication_manifest(published),
        admission=admission,
        quality_evidence_id=evidence_id,
    )
    return published, control / "quality-control-receipt.json", evidence_id


def self_test() -> int:
    report = {
        "raw_over_threshold": [
            {
                "crate": "demo",
                "file": "crates/demo/src/alpha.rs",
                "function": "alpha",
                "line": 7,
                "cyclomatic": 5.0,
                "coverage": 10.0,
                "crap": 50.0,
            },
            {
                "crate": "demo",
                "file": "crates/demo/src/beta.rs",
                "function": "beta",
                "line": 9,
                "cyclomatic": 4.0,
                "coverage": 20.0,
                "crap": 40.0,
            },
        ],
        "adjudicated": [],
        "actionable": [],
    }
    quality = load_module(
        Path(__file__).with_name("quality_observatory.py"),
        "quality_observatory_self_test",
    )

    class CrapFixture:
        @staticmethod
        def _load_adjudications(
            _registry: dict[str, Any], _repo: Path
        ) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
            return [], []

    original_crap_loader = quality.load_crap_module
    quality.load_crap_module = lambda _repo: CrapFixture
    report["actionable"] = list(report["raw_over_threshold"])
    selection = reconstruct_selection(Path.cwd(), report, quality, 1)
    assert selection["actionable_count"] == 2
    assert (
        selection["candidate_selection"][0]["module"]
        == "crates/demo/src/alpha.rs"
    )
    forged = {**report, "actionable": []}
    try:
        reconstruct_selection(Path.cwd(), forged, quality, None)
    except IntakeError:
        pass
    else:
        raise IntakeError("summary-only or forged partition was accepted")
    duplicate = dict(report["raw_over_threshold"][0])
    duplicate["crate"] = "alias"
    duplicate_report = {
        "raw_over_threshold": [report["raw_over_threshold"][0], duplicate],
        "adjudicated": [],
        "actionable": [report["raw_over_threshold"][0], duplicate],
    }
    try:
        reconstruct_selection(Path.cwd(), duplicate_report, quality, None)
    except IntakeError:
        pass
    else:
        raise IntakeError("canonical duplicate with crate drift was accepted")
    quality.load_crap_module = original_crap_loader
    repo = Path.cwd()
    crap_module = quality.load_crap_module(repo)
    current_head = git_text(repo, "rev-parse", "HEAD")
    current_payload = {
        "head_commit": current_head,
        "source_manifest_sha256": quality.manifest_sha256(
            quality.source_manifest(repo)
        ),
        "ordered_profiles": list(quality.PROFILES),
        "toolchain": quality.identity_versions(repo),
        "subject": {
            "source_tree": git_text(repo, "rev-parse", "HEAD^{tree}"),
            "workflow_revision": current_head,
            "workflow_sha256": sha256_file(
                repo / ".github/workflows/quality-observatory.yml"
            ),
        },
        "control_inputs": {
            "collector_sha256": sha256_file(
                repo / "tools/local_ci/quality_observatory.py"
            ),
            "snowbench_ledger_sha256": sha256_file(
                repo
                / "docs/work-packages/20260724-quality-observatory-merged-coverage-001"
                / "artifacts/snowbench-full-only-row-ledger.json"
            ),
        },
        "crap": {
            "registry_sha256": sha256_file(
                repo / "tools/release/adjudicated_crap_exceptions.json"
            )
        },
    }
    policy_report = {
        "production_filter": crap_module.EXPECTED_FILTER,
        "deduplication_key": crap_module.EXPECTED_DEDUPLICATION_KEY,
    }
    assert not stale_reasons(repo, current_payload, policy_report, quality)
    stale_payload = {**current_payload, "head_commit": "0" * 40}
    assert stale_reasons(repo, stale_payload, policy_report, quality) == [
        "HEAD differs from current repository identity"
    ]
    with tempfile.TemporaryDirectory(prefix="cqr-intake-self-test-") as raw:
        root = Path(raw)
        receipt_path = root / "intake.json"
        output = root / "authorization.json"
        intake(
            argparse.Namespace(
                repo=repo,
                published_dir=root / "missing-published",
                control_receipt=root
                / "missing-control/quality-control-receipt.json",
                expected_id="0" * 64,
                limit=None,
                output=receipt_path,
            )
        )
        authorize_recollection(
            argparse.Namespace(
                intake_receipt=receipt_path,
                repo=repo,
                operator_directive="execute cqr nightly for 2 modules",
                output=output,
            )
        )
        assert read_object(output)["status"] == "AUTHORIZED"
        current = read_object(receipt_path)
        current["disposition"] = "CURRENT"
        write_json(receipt_path, current)
        try:
            authorize_recollection(
                argparse.Namespace(
                    intake_receipt=receipt_path,
                    repo=repo,
                    operator_directive="execute cqr nightly",
                    output=output,
                )
            )
        except IntakeError:
            pass
        else:
            raise IntakeError("CURRENT evidence authorized recollection")
    workflow = load_module(
        repo / "tools/local_ci/quality_observatory_workflow.py",
        "quality_workflow_intake_self_test",
    )
    inventories = quality.independent_inventory_partition(repo)
    with tempfile.TemporaryDirectory(prefix="cqr-current-fixture-") as raw:
        root = Path(raw)
        current_root = root / "current"
        published, control_receipt, evidence_id = build_intake_fixture(
            repo,
            current_root,
            quality,
            workflow,
            subject_head=current_head,
            inventories=inventories,
        )
        current_receipt_path = root / "current-intake.json"
        if (
            intake(
                argparse.Namespace(
                    repo=repo,
                    published_dir=published,
                    control_receipt=control_receipt,
                    expected_id=evidence_id,
                    limit=1,
                    output=current_receipt_path,
                )
            )
            != 0
        ):
            raise IntakeError("valid exact-head fixture was not CURRENT")
        current_receipt = read_object(current_receipt_path)
        if (
            current_receipt["selection"]["candidate_selection"][0]["module"]
            != "crates/openwepp-sim-contract/src/lib.rs"
            or current_receipt["selection"]["selection_review_status"]
            != "REQUIRED"
            or current_receipt["collection_launched"] is not False
        ):
            raise IntakeError("CURRENT fixture selection parity failed")

        invalid_receipt_path = root / "invalid-intake.json"
        (published / "coverage-summary.json").write_text("{}\n", encoding="utf-8")
        if (
            intake(
                argparse.Namespace(
                    repo=repo,
                    published_dir=published,
                    control_receipt=control_receipt,
                    expected_id=evidence_id,
                    limit=1,
                    output=invalid_receipt_path,
                )
            )
            == 0
            or read_object(invalid_receipt_path)["disposition"] != "INVALID"
        ):
            raise IntakeError("artifact digest corruption was not INVALID")

        stale_root = root / "stale"
        stale_published, stale_control, stale_id = build_intake_fixture(
            repo,
            stale_root,
            quality,
            workflow,
            subject_head="0" * 40,
            inventories=inventories,
        )
        stale_receipt_path = root / "stale-intake.json"
        if (
            intake(
                argparse.Namespace(
                    repo=repo,
                    published_dir=stale_published,
                    control_receipt=stale_control,
                    expected_id=stale_id,
                    limit=1,
                    output=stale_receipt_path,
                )
            )
            == 0
            or read_object(stale_receipt_path)["disposition"] != "STALE"
        ):
            raise IntakeError("internally valid historical fixture was not STALE")
    print("cqr-quality-evidence-self-test: PASS")
    return 0


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser()
    commands = root.add_subparsers(dest="command", required=True)
    inspect = commands.add_parser("inspect")
    inspect.add_argument("--repo", type=Path, default=Path.cwd())
    inspect.add_argument("--published-dir", type=Path, required=True)
    inspect.add_argument("--control-receipt", type=Path, required=True)
    inspect.add_argument("--expected-id", required=True)
    inspect.add_argument("--limit", type=int)
    inspect.add_argument("--output", type=Path, required=True)
    inspect.set_defaults(function=intake)
    recollect = commands.add_parser("authorize-recollection")
    recollect.add_argument("--repo", type=Path, default=Path.cwd())
    recollect.add_argument("--intake-receipt", type=Path, required=True)
    recollect.add_argument("--operator-directive", required=True)
    recollect.add_argument("--output", type=Path, required=True)
    recollect.set_defaults(function=authorize_recollection)
    test = commands.add_parser("self-test")
    test.set_defaults(function=lambda _args: self_test())
    return root


def main() -> int:
    args = parser().parse_args()
    try:
        if getattr(args, "limit", None) is not None and args.limit < 1:
            raise IntakeError("selection limit must be positive")
        return args.function(args)
    except (IntakeError, OSError, ValueError, subprocess.SubprocessError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
