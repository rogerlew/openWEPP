from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT = REPO_ROOT / "tools" / "release" / "check_adjudicated_crap.py"
SPEC = importlib.util.spec_from_file_location("check_adjudicated_crap", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
gate = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = gate
SPEC.loader.exec_module(gate)


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _row(
    repo: Path,
    *,
    file: str = "crates/example/src/lib.rs",
    function: str = "example",
    line: int = 1,
    cyclomatic: float = 6,
    coverage: float | None = 0,
    crap: float = 42,
) -> dict[str, object]:
    return {
        "file": str(repo / file),
        "function": function,
        "line": line,
        "cyclomatic": cyclomatic,
        "coverage": coverage,
        "crap": crap,
        "crate": "example",
    }


def _initialize_repo(repo: Path) -> None:
    source = repo / "crates" / "example" / "src" / "lib.rs"
    source.parent.mkdir(parents=True)
    source.write_text("fn example() {}\n", encoding="utf-8")
    evidence = repo / "evidence"
    evidence.mkdir()
    evidence_content = (
        "# TEST-EVIDENCE\n\n"
        "example R-OBSERVABILITY SOURCE_HASH accepted-test-evidence\n"
    )
    for name in ("adjudication.md", "review-a.md", "review-b.md"):
        (evidence / name).write_text(evidence_content, encoding="utf-8")
    for authority_path in gate.EXPECTED_AUTHORITY.values():
        authority = repo / authority_path
        authority.parent.mkdir(parents=True, exist_ok=True)
        authority.write_text("# authority\n", encoding="utf-8")
    subprocess.run(["git", "init", "-q"], cwd=repo, check=True)
    subprocess.run(["git", "add", "."], cwd=repo, check=True)
    subprocess.run(
        [
            "git",
            "-c",
            "user.name=OpenWEPP Test",
            "-c",
            "user.email=openwepp-test@example.invalid",
            "commit",
            "-qm",
            "baseline",
        ],
        cwd=repo,
        check=True,
    )


def _configure_measurement_workspace(repo: Path, *, root_depends_on_example: bool) -> None:
    dependency = (
        'example = { path = "crates/example" }\n'
        if root_depends_on_example
        else ""
    )
    (repo / "Cargo.toml").write_text(
        "[package]\n"
        'name = "measurement-root"\n'
        'version = "0.1.0"\n'
        'edition = "2024"\n\n'
        "[workspace]\n"
        'members = ["crates/example"]\n'
        'resolver = "2"\n\n'
        "[dependencies]\n"
        f"{dependency}",
        encoding="utf-8",
    )
    root_source = repo / "src" / "lib.rs"
    root_source.parent.mkdir(exist_ok=True)
    root_source.write_text("//! Measurement-only test aggregator.\n", encoding="utf-8")
    (repo / "crates" / "example" / "Cargo.toml").write_text(
        "[package]\n"
        'name = "example"\n'
        'version = "0.1.0"\n'
        'edition = "2024"\n',
        encoding="utf-8",
    )
    subprocess.run(
        ["cargo", "generate-lockfile", "--offline"],
        cwd=repo,
        check=True,
        capture_output=True,
    )


def _registry(repo: Path) -> dict[str, object]:
    source = repo / "crates" / "example" / "src" / "lib.rs"
    source_hash = _sha256(source)
    commit = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=repo,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()
    evidence_paths = [
        repo / "evidence" / "adjudication.md",
        repo / "evidence" / "review-a.md",
        repo / "evidence" / "review-b.md",
    ]
    for evidence_path in evidence_paths:
        evidence_path.write_text(
            (
                "# TEST-EVIDENCE\n\n"
                f"example R-OBSERVABILITY {source_hash} accepted-test-evidence\n"
            ),
            encoding="utf-8",
        )
    return {
        "schema_version": gate.SCHEMA_VERSION,
        "threshold": 30,
        "production_filter": gate.EXPECTED_FILTER,
        "deduplication_key": gate.EXPECTED_DEDUPLICATION_KEY,
        "authority": gate.EXPECTED_AUTHORITY,
        "adjudications": [
            {
                "id": "TEST-001",
                "status": "accepted",
                "classification": "R-OBSERVABILITY",
                "file": "crates/example/src/lib.rs",
                "function": "example",
                "cyclomatic": 6,
                "file_sha256": source_hash,
                "adjudicated_at_commit": commit,
                "evidence_key": "TEST-EVIDENCE",
                "adjudication_evidence": {
                    "path": "evidence/adjudication.md",
                    "sha256": _sha256(evidence_paths[0]),
                    "acceptance_token": "accepted-test-evidence",
                },
                "review_evidence": [
                    {
                        "path": "evidence/review-a.md",
                        "sha256": _sha256(evidence_paths[1]),
                        "acceptance_token": "accepted-test-evidence",
                    },
                    {
                        "path": "evidence/review-b.md",
                        "sha256": _sha256(evidence_paths[2]),
                        "acceptance_token": "accepted-test-evidence",
                    },
                ],
            }
        ],
    }


def _payload(entries: list[dict[str, object]]) -> dict[str, object]:
    return {
        "$schema": gate.CARGO_CRAP_SCHEMA,
        "version": gate.SUPPORTED_CARGO_CRAP_VERSION,
        "entries": entries,
    }


class AdjudicatedCrapGateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.repo = Path(self.temporary.name)
        _initialize_repo(self.repo)

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def test_measurement_root_requires_global_quality(self) -> None:
        _configure_measurement_workspace(self.repo, root_depends_on_example=True)
        with self.assertRaisesRegex(gate.GateInputError, "requires global quality"):
            gate.resolve_measurement_packages(self.repo, {"measurement-root"})

    def test_production_measurement_package_resolves_to_itself(self) -> None:
        _configure_measurement_workspace(self.repo, root_depends_on_example=True)
        scope = gate.resolve_measurement_packages(self.repo, {"example"})
        self.assertEqual(scope["production_packages"], ["example"])

    def test_unknown_measurement_package_fails_closed(self) -> None:
        _configure_measurement_workspace(self.repo, root_depends_on_example=True)
        with self.assertRaisesRegex(gate.GateInputError, "unknown workspace packages"):
            gate.resolve_measurement_packages(self.repo, {"missing"})

    def test_scope_preflight_prints_resolved_identity(self) -> None:
        _configure_measurement_workspace(self.repo, root_depends_on_example=True)
        result = subprocess.run(
            [
                sys.executable,
                str(SCRIPT),
                "--repo-root",
                str(self.repo),
                "--validate-expected-packages",
                "--expected-package",
                "example",
            ],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            json.loads(result.stdout),
            {
                "measurement_packages": ["example"],
                "production_packages": ["example"],
            },
        )

    def test_driver_rejects_root_measurement_before_coverage(self) -> None:
        driver = REPO_ROOT / "tools/release/run_adjudicated_crap_gate.sh"
        with tempfile.TemporaryDirectory() as temporary_directory:
            output_directory = Path(temporary_directory) / "output"
            result = subprocess.run(
                [
                    "bash",
                    str(driver),
                    "--scope",
                    "affected",
                    "--package",
                    "openwepp",
                    "--nextest-profile",
                    "affected",
                    "--base-ref",
                    "HEAD",
                    "--output-dir",
                    str(output_directory),
                ],
                cwd=REPO_ROOT,
                check=False,
                capture_output=True,
                text=True,
                env={
                    key: value
                    for key, value in os.environ.items()
                    if key
                    not in {
                        "OPENWEPP_GATE_ARTIFACT_ROOT",
                        "OPENWEPP_GATE_NEXTEST_CONFIG",
                    }
                },
            )
            self.assertEqual(result.returncode, 2)
            self.assertIn("requires global quality", result.stderr)
            self.assertFalse((output_directory / "llvm-cov.log").exists())

    def test_exact_adjudication_closes_raw_row(self) -> None:
        report = gate.evaluate(
            _payload([_row(self.repo)]),
            _registry(self.repo),
            self.repo,
            ["crates/example/src/lib.rs"],
        )
        self.assertEqual(report["status"], "PASS")
        self.assertEqual(report["raw_over_threshold_count"], 1)
        self.assertEqual(report["adjudicated_count"], 1)
        self.assertEqual(report["actionable_count"], 0)
        self.assertEqual(
            report["touched_production_files"], ["crates/example/src/lib.rs"]
        )

    def test_new_workspace_row_blocks_even_when_untouched(self) -> None:
        other = self.repo / "crates" / "other" / "src" / "lib.rs"
        other.parent.mkdir(parents=True)
        other.write_text("fn new_debt() {}\n", encoding="utf-8")
        payload = _payload(
            [
                _row(self.repo),
                _row(
                    self.repo,
                    file="crates/other/src/lib.rs",
                    function="new_debt",
                    cyclomatic=8,
                    crap=72,
                ),
            ]
        )
        report = gate.evaluate(
            payload,
            _registry(self.repo),
            self.repo,
            ["crates/example/src/lib.rs"],
        )
        self.assertEqual(report["status"], "FAIL")
        self.assertEqual(report["actionable_count"], 1)
        self.assertEqual(report["touched_actionable_count"], 0)
        self.assertEqual(report["untouched_actionable_count"], 1)

    def test_worktree_touched_paths_include_tracked_and_untracked_files(self) -> None:
        base_ref = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=self.repo,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()

        tracked = self.repo / "crates" / "example" / "src" / "lib.rs"
        tracked.write_text("fn example() { println!(\"changed\"); }\n", encoding="utf-8")
        untracked = self.repo / "crates" / "new" / "src" / "lib.rs"
        untracked.parent.mkdir(parents=True)
        untracked.write_text("fn new_source() {}\n", encoding="utf-8")

        changed = gate._changed_paths(self.repo, base_ref, None)
        self.assertIn(
            {"path": "crates/example/src/lib.rs", "status": "M"}, changed
        )
        self.assertIn({"path": "crates/new/src/lib.rs", "status": "U"}, changed)

    def test_touched_paths_record_deletions_and_both_rename_paths(self) -> None:
        base_ref = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=self.repo,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        source = self.repo / "crates" / "example" / "src" / "lib.rs"
        source.unlink()
        deleted = gate._changed_paths(self.repo, base_ref, None)
        self.assertIn(
            {"path": "crates/example/src/lib.rs", "status": "D"}, deleted
        )

        subprocess.run(["git", "restore", "crates/example/src/lib.rs"], cwd=self.repo, check=True)
        subprocess.run(
            [
                "git",
                "mv",
                "crates/example/src/lib.rs",
                "crates/example/src/renamed.rs",
            ],
            cwd=self.repo,
            check=True,
        )
        renamed = gate._changed_paths(self.repo, base_ref, None)
        self.assertIn(
            {"path": "crates/example/src/lib.rs", "status": "R100-from"},
            renamed,
        )
        self.assertIn(
            {"path": "crates/example/src/renamed.rs", "status": "R100-to"},
            renamed,
        )

    def test_source_manifest_changes_when_production_source_changes(self) -> None:
        before = gate.production_source_manifest(self.repo)
        source = self.repo / "crates" / "example" / "src" / "lib.rs"
        source.write_text("fn example() { println!(\"changed\"); }\n", encoding="utf-8")
        after = gate.production_source_manifest(self.repo)
        self.assertNotEqual(before, after)

    def test_source_manifest_changes_when_toolchain_selector_changes(self) -> None:
        toolchain = self.repo / "rust-toolchain.toml"
        toolchain.write_text('[toolchain]\nchannel = "stable"\n', encoding="utf-8")
        before = gate.production_source_manifest(self.repo)
        self.assertIn(
            "rust-toolchain.toml",
            {item["path"] for item in before["measurement_inputs"]},
        )
        toolchain.write_text('[toolchain]\nchannel = "beta"\n', encoding="utf-8")
        after = gate.production_source_manifest(self.repo)
        self.assertNotEqual(before, after)

    def test_stale_source_hash_invalidates_adjudication(self) -> None:
        registry = _registry(self.repo)
        source = self.repo / "crates" / "example" / "src" / "lib.rs"
        source.write_text("fn example() { println!(\"changed\"); }\n", encoding="utf-8")
        report = gate.evaluate(_payload([_row(self.repo)]), registry, self.repo)
        self.assertEqual(report["status"], "FAIL")
        self.assertEqual(report["actionable_count"], 1)
        self.assertIn(
            "source hash is stale", report["invalid_adjudications"][0]["reason"]
        )

    def test_filter_and_exact_tuple_dedup_match_cqr_campaign(self) -> None:
        (self.repo / "crates" / "example" / "src" / "tests").mkdir()
        test_row = _row(
            self.repo, file="crates/example/src/tests/test.rs", function="test_only"
        )
        crate_test_row = _row(
            self.repo, file="crates/example/tests/test.rs", function="crate_test"
        )
        docs_row = _row(self.repo, file="docs/prototype.rs", function="prototype")
        exact = _row(self.repo)
        report = gate.evaluate(
            _payload([exact, dict(exact), test_row, crate_test_row, docs_row]),
            _registry(self.repo),
            self.repo,
        )
        self.assertEqual(report["status"], "PASS")
        self.assertEqual(report["raw_over_threshold_count"], 1)

    def test_under_evidenced_and_wildcard_adjudications_fail_closed(self) -> None:
        registry = _registry(self.repo)
        registry["adjudications"][0]["review_evidence"] = ["evidence/review-a.md"]
        with self.assertRaisesRegex(gate.GateInputError, "evidence object"):
            gate.evaluate(_payload([_row(self.repo)]), registry, self.repo)

        wildcard_registry = _registry(self.repo)
        wildcard_registry["adjudications"][0]["function"] = "*"
        with self.assertRaisesRegex(gate.GateInputError, "function must be exact"):
            gate.evaluate(_payload([_row(self.repo)]), wildcard_registry, self.repo)

        escaping_registry = _registry(self.repo)
        escaping_registry["adjudications"][0]["file"] = "../outside.rs"
        with self.assertRaisesRegex(gate.GateInputError, "escapes repository"):
            gate.evaluate(_payload([_row(self.repo)]), escaping_registry, self.repo)

    def test_missing_commit_and_changed_evidence_invalidate_adjudication(self) -> None:
        missing_commit = _registry(self.repo)
        missing_commit["adjudications"][0]["adjudicated_at_commit"] = "f" * 40
        report = gate.evaluate(_payload([_row(self.repo)]), missing_commit, self.repo)
        self.assertEqual(report["status"], "FAIL")
        self.assertTrue(
            any(
                "commit does not resolve" in item["reason"]
                for item in report["invalid_adjudications"]
            )
        )

        changed_evidence = _registry(self.repo)
        (self.repo / "evidence" / "review-a.md").write_text(
            "# unrelated review\n", encoding="utf-8"
        )
        report = gate.evaluate(_payload([_row(self.repo)]), changed_evidence, self.repo)
        self.assertEqual(report["status"], "FAIL")
        self.assertTrue(
            any(
                "evidence hash is stale" in item["reason"]
                for item in report["invalid_adjudications"]
            )
        )

    def test_production_crate_census_mismatch_fails_closed(self) -> None:
        with self.assertRaisesRegex(gate.GateInputError, "census mismatch"):
            gate.evaluate(
                _payload([_row(self.repo)]),
                _registry(self.repo),
                self.repo,
                expected_production_crates={"example", "missing-crate"},
            )

    def test_retained_mode_is_assessment_only(self) -> None:
        with tempfile.TemporaryDirectory() as output_directory:
            report_path = Path(output_directory) / "report.json"
            result = subprocess.run(
                [
                    sys.executable,
                    str(SCRIPT),
                    "--repo-root",
                    str(REPO_ROOT),
                    "--crap-json",
                    str(
                        REPO_ROOT
                        / "docs/work-packages/cqr-pre-integration-campaign-evidence/low/final/final-crap.json"
                    ),
                    "--acquisition-mode",
                    "retained",
                    "--retained-provenance",
                    "docs/work-packages/cqr-pre-integration-campaign-evidence/low/campaign-final-assessment.md",
                    "--report-json",
                    str(report_path),
                ],
                cwd=REPO_ROOT,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            report = json.loads(report_path.read_text(encoding="utf-8"))
            self.assertEqual(report["status"], "ASSESSMENT-PASS")
            self.assertEqual(report["debt_status"], "PASS")
            self.assertFalse(report["closure_eligible"])

    def test_fresh_mode_rejects_substitute_registry(self) -> None:
        alternate_registry = self.repo / "alternate.json"
        alternate_registry.write_text("{}\n", encoding="utf-8")
        dummy_report = self.repo / "report.json"
        dummy_report.write_text("{}\n", encoding="utf-8")
        result = subprocess.run(
            [
                sys.executable,
                str(SCRIPT),
                "--repo-root",
                str(self.repo),
                "--crap-json",
                str(dummy_report),
                "--acquisition-mode",
                "fresh",
                "--adjudications",
                str(alternate_registry),
            ],
            cwd=self.repo,
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 2)
        self.assertIn("canonical adjudication registry", result.stderr)

    def test_failed_driver_replaces_stale_pass_with_failure_envelope(self) -> None:
        driver = REPO_ROOT / "tools" / "release" / "run_adjudicated_crap_gate.sh"
        with tempfile.TemporaryDirectory() as temporary_directory:
            temporary_path = Path(temporary_directory)
            output_directory = temporary_path / "output"
            output_directory.mkdir()
            stale_report = output_directory / "adjudicated-crap-report.json"
            stale_report.write_text('{"status":"PASS"}\n', encoding="utf-8")
            malformed_report = temporary_path / "malformed.json"
            malformed_report.write_text("{}\n", encoding="utf-8")
            result = subprocess.run(
                [
                    "bash",
                    str(driver),
                    "--crap-json",
                    str(malformed_report),
                    "--retained-provenance",
                    "docs/work-packages/cqr-pre-integration-campaign-evidence/low/campaign-final-assessment.md",
                    "--output-dir",
                    str(output_directory),
                ],
                cwd=REPO_ROOT,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(result.returncode, 2)
            self.assertFalse(stale_report.exists())
            run_status = json.loads(
                (output_directory / "run-status.json").read_text(encoding="utf-8")
            )
            self.assertEqual(run_status["result"], "FAIL")
            self.assertEqual(run_status["exit_status"], 2)
            self.assertIsNone(run_status["adjudicated_crap_report_sha256"])
            checksum_manifest = (
                output_directory / "sha256sums.txt"
            ).read_text(encoding="utf-8")
            self.assertIn("run-status.json", checksum_manifest)

    def test_successful_driver_control_envelope_binds_numeric_report(self) -> None:
        driver = REPO_ROOT / "tools" / "release" / "run_adjudicated_crap_gate.sh"
        retained_report = (
            REPO_ROOT
            / "docs/work-packages/cqr-pre-integration-campaign-evidence/low/final/final-crap.json"
        )
        retained_provenance = (
            "docs/work-packages/cqr-pre-integration-campaign-evidence/low/"
            "campaign-final-assessment.md"
        )
        with tempfile.TemporaryDirectory() as temporary_directory:
            output_directory = Path(temporary_directory) / "output"
            result = subprocess.run(
                [
                    "bash",
                    str(driver),
                    "--crap-json",
                    str(retained_report),
                    "--retained-provenance",
                    retained_provenance,
                    "--output-dir",
                    str(output_directory),
                ],
                cwd=REPO_ROOT,
                check=False,
                capture_output=True,
                text=True,
            )

            self.assertEqual(result.returncode, 0, result.stderr)
            detailed_report = output_directory / "adjudicated-crap-report.json"
            self.assertIsInstance(
                json.loads(detailed_report.read_text(encoding="utf-8"))["threshold"],
                float,
            )
            run_status = json.loads(
                (output_directory / "run-status.json").read_text(encoding="utf-8")
            )
            self.assertEqual(run_status["result"], "PASS")
            self.assertEqual(run_status["exit_status"], 0)
            self.assertEqual(
                run_status["adjudicated_crap_report_sha256"],
                _sha256(detailed_report),
            )

    def test_pre_acquisition_failure_replaces_stale_pass_envelope(self) -> None:
        driver = REPO_ROOT / "tools" / "release" / "run_adjudicated_crap_gate.sh"
        with tempfile.TemporaryDirectory() as temporary_directory:
            temporary_path = Path(temporary_directory)
            output_directory = temporary_path / "output"
            output_directory.mkdir()
            stale_report = output_directory / "adjudicated-crap-report.json"
            stale_report.write_text('{"status":"PASS"}\n', encoding="utf-8")
            stale_checksums = output_directory / "sha256sums.txt"
            stale_checksums.write_text("stale PASS checksums\n", encoding="utf-8")
            retained_report = temporary_path / "retained.json"
            retained_report.write_text("{}\n", encoding="utf-8")

            result = subprocess.run(
                [
                    "bash",
                    str(driver),
                    "--crap-json",
                    str(retained_report),
                    "--output-dir",
                    str(output_directory),
                ],
                cwd=REPO_ROOT,
                check=False,
                capture_output=True,
                text=True,
            )

            self.assertEqual(result.returncode, 2)
            self.assertIn("--crap-json requires --retained-provenance", result.stderr)
            self.assertFalse(stale_report.exists())
            run_status = json.loads(
                (output_directory / "run-status.json").read_text(encoding="utf-8")
            )
            self.assertEqual(run_status["result"], "FAIL")
            self.assertEqual(run_status["acquisition_mode"], "retained")
            self.assertEqual(run_status["exit_status"], 2)
            self.assertIsNone(run_status["adjudicated_crap_report_sha256"])
            checksum_manifest = stale_checksums.read_text(encoding="utf-8")
            self.assertNotIn("stale PASS checksums", checksum_manifest)
            self.assertIn("run-status.json", checksum_manifest)

    def test_empty_or_nonproduction_report_cannot_pass(self) -> None:
        with self.assertRaisesRegex(gate.GateInputError, "report schema"):
            gate.evaluate({"entries": [_row(self.repo)]}, _registry(self.repo), self.repo)
        with self.assertRaisesRegex(gate.GateInputError, "non-empty entries"):
            gate.evaluate(_payload([]), _registry(self.repo), self.repo)
        with self.assertRaisesRegex(gate.GateInputError, "no production rows"):
            gate.evaluate(
                _payload([_row(self.repo, file="tests/example.rs")]),
                _registry(self.repo),
                self.repo,
            )

    def test_completed_cqr_campaign_reproduces_empty_actionable_set(self) -> None:
        crap_payload = json.loads(
            (
                REPO_ROOT
                / "docs/work-packages/cqr-pre-integration-campaign-evidence/low/final/final-crap.json"
            ).read_text(encoding="utf-8")
        )
        registry = json.loads(
            (REPO_ROOT / "tools/release/adjudicated_crap_exceptions.json").read_text(
                encoding="utf-8"
            )
        )
        report = gate.evaluate(crap_payload, registry, REPO_ROOT)
        self.assertEqual(report["status"], "PASS")
        self.assertEqual(report["raw_over_threshold_count"], 2)
        self.assertEqual(report["adjudicated_count"], 2)
        self.assertEqual(report["actionable_count"], 0)


if __name__ == "__main__":
    unittest.main()
