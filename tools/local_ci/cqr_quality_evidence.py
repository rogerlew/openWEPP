#!/usr/bin/env python3
"""Verify quality-observatory evidence and derive CQR module selection."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import re
import subprocess
import sys
import tempfile
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
    return (
        crate,
        production_path(file_name),
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
        "ranking": ranking,
        "selected": selected,
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
    disposition = "INVALID"
    reasons: list[str] = []
    evidence_id: str | None = None
    selection: dict[str, Any] | None = None
    inputs: dict[str, Any] = {
        "published_locator": str(args.published_dir),
        "control_locator": str(args.control_receipt),
        "expected_quality_evidence_id": args.expected_id,
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
        args.output.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.NamedTemporaryFile(
            mode="wb",
            dir=args.output.parent,
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
            admission_path = args.output.with_name(
                f".{args.output.name}.admission.tmp"
            )
            write_json(admission_path, control["admission"])
            try:
                verified_id = quality.verify_published(
                    repo,
                    published,
                    admission_path,
                    independent_inventory=False,
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
    write_json(args.output, receipt)
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
            },
            {
                "published_locator",
                "control_locator",
                "expected_quality_evidence_id",
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
    authorization = {
        "schema_version": AUTHORIZATION_SCHEMA,
        "status": "AUTHORIZED",
        "operator_directive": directive,
        "intake_receipt_sha256": sha256_file(args.intake_receipt),
        "quality_evidence_id": receipt.get("quality_evidence_id"),
        "disposition": receipt["disposition"],
        "reasons": receipt["reasons"],
    }
    write_json(args.output, authorization)
    print(f"cqr-recollection: AUTHORIZED disposition={receipt['disposition']}")
    return 0


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
    assert selection["selected"][0]["module"] == "crates/demo/src/alpha.rs"
    forged = {**report, "actionable": []}
    try:
        reconstruct_selection(Path.cwd(), forged, quality, None)
    except IntakeError:
        pass
    else:
        raise IntakeError("summary-only or forged partition was accepted")
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
        write_json(
            receipt_path,
            {
                "schema_version": SCHEMA,
                "disposition": "INVALID",
                "quality_evidence_id": None,
                "inputs": {
                    "published_locator": "missing",
                    "control_locator": "missing",
                    "expected_quality_evidence_id": "0" * 64,
                },
                "reasons": ["fixture is absent"],
                "selection": None,
                "collection_launched": False,
            },
        )
        authorize_recollection(
            argparse.Namespace(
                intake_receipt=receipt_path,
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
                    operator_directive="execute cqr nightly",
                    output=output,
                )
            )
        except IntakeError:
            pass
        else:
            raise IntakeError("CURRENT evidence authorized recollection")
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
