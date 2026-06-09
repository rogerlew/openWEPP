#!/usr/bin/env python3
"""Suite manifest helpers for owcmp."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

SCHEMA_VERSION = "owcmp-suite-manifest-v1"
ENV_SCHEMA_VERSION = "owcmp-env-v1"
TOOL_DIR = Path(__file__).resolve().parent
REPO_ROOT = TOOL_DIR.parents[1]
DEFAULT_SUITE_DIR = TOOL_DIR / "suites"
DEFAULT_TOLERANCE_CONFIG = TOOL_DIR / "configs" / "pl14s_wat_tolerances.json"


def load_manifest(path: Path) -> dict[str, Any]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if payload.get("schema_version") != SCHEMA_VERSION:
        raise RuntimeError(
            f"unsupported owcmp suite manifest schema in {path}: {payload.get('schema_version')!r}"
        )
    return payload


def list_manifest_paths(suite_dir: Path = DEFAULT_SUITE_DIR) -> list[Path]:
    if not suite_dir.is_dir():
        return []
    return sorted(suite_dir.glob("*.json"))


def list_manifests(suite_dir: Path = DEFAULT_SUITE_DIR) -> list[dict[str, Any]]:
    manifests = []
    for path in list_manifest_paths(suite_dir):
        payload = load_manifest(path)
        manifests.append(
            {
                "suite_id": payload.get("suite_id"),
                "title": payload.get("title"),
                "lane": payload.get("lane"),
                "run_root": payload.get("run_root"),
                "manifest": str(path),
            }
        )
    return manifests


def _resolve_manifest_path(manifest_path: Path, path_text: str) -> Path:
    path = Path(path_text)
    if path.is_absolute():
        return path
    return (manifest_path.parent / path).resolve()


def _check_path(path: Path, kind: str) -> dict[str, Any]:
    if kind == "file":
        exists = path.is_file()
    elif kind == "directory":
        exists = path.is_dir()
    else:
        raise RuntimeError(f"unsupported manifest check kind: {kind!r}")
    return {
        "kind": kind,
        "path": str(path),
        "status": "PASS" if exists else "FAIL",
    }


def _check_pattern_range(manifest_path: Path, check: dict[str, Any]) -> dict[str, Any]:
    pattern = check["path_pattern"]
    start = int(check.get("start", 1))
    end = int(check["end"])
    missing = []
    present = 0
    for value in range(start, end + 1):
        path = _resolve_manifest_path(manifest_path, pattern.format(h=value, H=f"H{value}"))
        if path.is_file():
            present += 1
        else:
            missing.append(str(path))
    return {
        "kind": "pattern-range",
        "path_pattern": pattern,
        "start": start,
        "end": end,
        "present": present,
        "expected": end - start + 1,
        "missing": missing,
        "status": "PASS" if not missing else "FAIL",
    }


def check_manifest(manifest_path: Path) -> dict[str, Any]:
    manifest = load_manifest(manifest_path)
    checks = []
    for item in manifest.get("checks", []):
        kind = item.get("kind")
        if kind in {"file", "directory"}:
            path = _resolve_manifest_path(manifest_path, item["path"])
            result = _check_path(path, kind)
        elif kind == "pattern-range":
            result = _check_pattern_range(manifest_path, item)
        else:
            raise RuntimeError(f"unsupported manifest check kind: {kind!r}")
        result["name"] = item.get("name")
        result["required"] = bool(item.get("required", True))
        checks.append(result)

    required_failures = [
        item for item in checks if item["required"] and item["status"] != "PASS"
    ]
    return {
        "schema_version": "owcmp-suite-check-v1",
        "manifest": str(manifest_path),
        "suite_id": manifest.get("suite_id"),
        "title": manifest.get("title"),
        "lane": manifest.get("lane"),
        "run_root": manifest.get("run_root"),
        "status": "PASS" if not required_failures else "FAIL",
        "checks": checks,
    }


def env_summary(manifest_path: Path | None = None) -> dict[str, Any]:
    python_check = {
        "name": "python_executable",
        "path": sys.executable,
        "status": "PASS",
    }
    venv_python = REPO_ROOT / ".venv" / "bin" / "python"
    venv_check = {
        "name": "repo_venv_python",
        "path": str(venv_python),
        "status": "PASS" if venv_python.is_file() else "FAIL",
    }
    try:
        import pyarrow  # type: ignore

        pyarrow_check = {
            "name": "pyarrow",
            "status": "PASS",
            "version": getattr(pyarrow, "__version__", "unknown"),
        }
    except ImportError as exc:  # pragma: no cover - environment dependent
        pyarrow_check = {
            "name": "pyarrow",
            "status": "FAIL",
            "error": str(exc),
        }

    tolerance_check = {
        "name": "default_tolerance_config",
        "path": str(DEFAULT_TOLERANCE_CONFIG),
        "status": "PASS" if DEFAULT_TOLERANCE_CONFIG.is_file() else "FAIL",
    }
    suite_count = len(list_manifest_paths())
    summary: dict[str, Any] = {
        "schema_version": ENV_SCHEMA_VERSION,
        "status": "PASS",
        "checks": [python_check, venv_check, pyarrow_check, tolerance_check],
        "suite_manifest_count": suite_count,
    }
    if manifest_path is not None:
        manifest_check = check_manifest(manifest_path)
        summary["manifest_check"] = manifest_check
        if manifest_check["status"] != "PASS":
            summary["status"] = "FAIL"
    if any(item["status"] != "PASS" for item in summary["checks"]):
        summary["status"] = "FAIL"
    return summary


def _print_human_env(summary: dict[str, Any]) -> None:
    print(f"owcmp env: {summary['status']}")
    for item in summary["checks"]:
        details = item.get("path") or item.get("version") or item.get("error") or ""
        print(f"- {item['name']}: {item['status']} {details}".rstrip())
    print(f"- suite_manifest_count: {summary['suite_manifest_count']}")
    manifest_check = summary.get("manifest_check")
    if manifest_check:
        print(f"- manifest {manifest_check['suite_id']}: {manifest_check['status']}")
        for item in manifest_check["checks"]:
            detail = item.get("path") or item.get("path_pattern")
            print(f"  - {item['name']}: {item['status']} {detail}")


def env_main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(
        description="Check owcmp environment and optional suite manifest"
    )
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv)
    summary = env_summary(args.manifest)
    if args.json:
        print(json.dumps(summary, indent=2, sort_keys=True))
    else:
        _print_human_env(summary)
    if summary["status"] != "PASS":
        raise SystemExit(1)


def list_main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(description="List owcmp suite manifests")
    parser.add_argument("--suite-dir", type=Path, default=DEFAULT_SUITE_DIR)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv)
    manifests = list_manifests(args.suite_dir)
    if args.json:
        payload = {"schema_version": "owcmp-suite-list-v1", "suites": manifests}
        print(json.dumps(payload, indent=2, sort_keys=True))
        return
    for manifest in manifests:
        print(f"{manifest['suite_id']}\t{manifest['lane']}\t{manifest['manifest']}")


def show_main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(description="Show an owcmp suite manifest")
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv)
    manifest = load_manifest(args.manifest)
    if args.json:
        print(json.dumps(manifest, indent=2, sort_keys=True))
        return
    print(f"{manifest.get('suite_id')}: {manifest.get('title')}")
    print(f"lane: {manifest.get('lane')}")
    print(f"run_root: {manifest.get('run_root')}")
