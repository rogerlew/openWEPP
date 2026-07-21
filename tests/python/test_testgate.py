from __future__ import annotations

import importlib.util
import io
import json
import subprocess
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).parents[2]
MODULE_PATH = ROOT / "tools/local_ci/testgate.py"
SPEC = importlib.util.spec_from_file_location("testgate", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
TESTGATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TESTGATE)
QUALIFICATION_PATH = ROOT / "tools/local_ci/testgate_qualification.py"
QUALIFICATION_SPEC = importlib.util.spec_from_file_location(
    "testgate_qualification", QUALIFICATION_PATH
)
assert QUALIFICATION_SPEC is not None and QUALIFICATION_SPEC.loader is not None
QUALIFICATION = importlib.util.module_from_spec(QUALIFICATION_SPEC)
QUALIFICATION_SPEC.loader.exec_module(QUALIFICATION)


class TestGateTest(unittest.TestCase):
    def test_qualification_freeze_rejects_subject_mutation(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            repo = Path(directory)
            subject = repo / "subject.txt"
            subject.write_text("frozen\n", encoding="utf-8")
            freeze = {
                "schema_version": "openwepp-testgate-subject-freeze-v1",
                "subject_freeze_id": "0" * 64,
                "implementation_commit": "1" * 40,
                "paths": [
                    {
                        "path": "subject.txt",
                        "sha256": TESTGATE.hashlib.sha256(subject.read_bytes()).hexdigest(),
                    }
                ],
            }
            freeze["subject_freeze_id"] = QUALIFICATION._derived(
                freeze, "subject_freeze_id"
            )
            QUALIFICATION._verify_freeze(repo, freeze)
            subject.write_text("changed\n", encoding="utf-8")
            with self.assertRaises(QUALIFICATION.QualificationError):
                QUALIFICATION._verify_freeze(repo, freeze)

    def test_attempt_history_is_append_only_and_hash_chained(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "attempts.jsonl"
            first = TESTGATE._append_history(
                ledger, {"record_type": "ATTEMPT", "status": "CLOSED"}
            )
            second = TESTGATE._append_history(
                ledger, {"record_type": "ATTEMPT", "status": "CLOSED"}
            )
            records = [json.loads(line) for line in ledger.read_text().splitlines()]
        self.assertEqual(records[0]["entry_sha256"], first)
        self.assertIsNone(records[0]["previous_entry_sha256"])
        self.assertEqual(records[1]["previous_entry_sha256"], first)
        self.assertEqual(records[1]["entry_sha256"], second)

    def test_attempt_index_covers_pre_receipt_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "pre-receipt-failure.json").write_text("{}\n", encoding="utf-8")
            TESTGATE._write_attempt_index(root)
            index = json.loads((root / "attempt-index.json").read_text())
        self.assertEqual(index["files"][0]["path"], "pre-receipt-failure.json")

    def test_durable_history_snapshot_is_indexable_and_exact(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            evidence = root / "evidence"
            evidence.mkdir()
            ledger = root / "durable" / "attempts.jsonl"
            ledger.parent.mkdir()
            TESTGATE._append_history(
                ledger,
                {"record_type": "STAGE_ATTEMPT", "status": "FAILED"},
            )
            TESTGATE._snapshot_history(ledger, evidence)
            TESTGATE._write_attempt_index(evidence)
            self.assertEqual(
                (evidence / "attempts.jsonl").read_bytes(), ledger.read_bytes()
            )
            index = json.loads((evidence / "attempt-index.json").read_text())
            self.assertEqual(index["files"][0]["path"], "attempts.jsonl")

    def test_base_package_authorizes_only_declared_changed_paths(self) -> None:
        package = """# Package\n\nStatus: `READY / ACTIVE`\n\n## Declared Write Set\n\n- `src/**`\n- `docs/work-packages/example/package.md`\n\n## Next\n"""
        changed = ["docs/work-packages/example/package.md", "src/lib.rs"]
        with mock.patch.object(TESTGATE, "_base_text", return_value=package):
            authorization = TESTGATE._intent_authorization(
                ROOT, "a" * 40, changed, "docs/work-packages/example/package.md"
            )
        self.assertEqual(authorization["authorized_changed_paths"], changed)
        self.assertEqual(authorization["package_path"], changed[0])

        with mock.patch.object(TESTGATE, "_base_text", return_value=package):
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._intent_authorization(
                    ROOT,
                    "a" * 40,
                    [*changed, ".github/workflows/undeclared.yml"],
                    "docs/work-packages/example/package.md",
                )

    def test_intent_package_must_preexist_and_change(self) -> None:
        with self.assertRaises(TESTGATE.TestgateError):
            TESTGATE._intent_authorization(ROOT, "a" * 40, ["src/lib.rs"], None)

    def test_invoke_retains_valid_nonpass_result(self) -> None:
        completed = subprocess.CompletedProcess(
            args=["gate"], returncode=1, stdout='{"result":"FAIL"}', stderr=""
        )
        with mock.patch.object(TESTGATE.subprocess, "run", return_value=completed):
            value = TESTGATE._invoke(["gate"], ROOT, allow_nonpass=True)
        self.assertEqual(value["result"], "FAIL")

        with mock.patch.object(TESTGATE.subprocess, "run", return_value=completed):
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._invoke(["gate"], ROOT)

    def test_main_reports_verified_nonpass_as_failure_visible(self) -> None:
        observation = {
            "execution_requested": True,
            "execution_result": {"result": "BLOCKED"},
            "execution_error": None,
        }
        with (
            mock.patch.object(TESTGATE, "_parse_args", return_value=object()),
            mock.patch.object(TESTGATE, "observe", return_value=observation),
            redirect_stdout(io.StringIO()),
        ):
            self.assertEqual(TESTGATE.main(), 1)

        observation["execution_result"] = {"result": "PASS"}
        with (
            mock.patch.object(TESTGATE, "_parse_args", return_value=object()),
            mock.patch.object(TESTGATE, "observe", return_value=observation),
            redirect_stdout(io.StringIO()),
        ):
            self.assertEqual(TESTGATE.main(), 0)


if __name__ == "__main__":
    unittest.main()
