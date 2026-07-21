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

    def test_attempt_history_uses_utf8_canonical_bytes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "attempts.jsonl"
            digest = TESTGATE._append_history(
                ledger,
                {
                    "record_type": "ATTEMPT",
                    "error": "résumé",
                    "\ue000": "bmp-private",
                    "😀": "astral",
                },
            )
            record = json.loads(ledger.read_text(encoding="utf-8"))
            claimed = record.pop("entry_sha256")
            canonical = TESTGATE._canonical_json(record).encode("utf-8")
            self.assertEqual(claimed, digest)
            self.assertEqual(claimed, TESTGATE.hashlib.sha256(canonical).hexdigest())
            self.assertLess(canonical.index("😀".encode()), canonical.index("\ue000".encode()))

    def test_attempt_history_rejects_noncanonical_numbers(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "attempts.jsonl"
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._append_history(ledger, {"wall_time": 1.5})

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

    def test_accepted_receipt_is_finalized_beside_current_recovery_tree(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / "evidence"
            recovery = root / "history/recovery/run-1"
            artifact.mkdir()
            recovery.mkdir(parents=True)
            TESTGATE._append_history(
                root / "history/attempts.jsonl",
                {"record_type": "ATTEMPT", "status": "CLOSED"},
            )
            (artifact / "receipt.json").write_text('{"result":"PASS"}\n')
            (artifact / "terminal-plan.json").write_text('{"plan_id":"p"}\n')
            (recovery / ".checkpoints").mkdir()
            (recovery / ".checkpoints/node.json").write_text("{}\n")
            with mock.patch.dict(
                TESTGATE.os.environ,
                {"OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT": str(recovery)},
            ):
                self.assertEqual(TESTGATE._finalize_recovery(artifact), str(recovery))
                TESTGATE._snapshot_history(root / "history/attempts.jsonl", artifact)
            self.assertEqual(
                (recovery / "receipt.json").read_bytes(),
                (artifact / "receipt.json").read_bytes(),
            )
            self.assertEqual(
                (recovery / "plan.json").read_bytes(),
                (artifact / "terminal-plan.json").read_bytes(),
            )
            self.assertTrue(
                (artifact / "recovery/run-1/.checkpoints/node.json").is_file()
            )

    def test_snapshot_retains_every_ledger_referenced_recovery_root(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            history = root / "history"
            evidence = root / "evidence"
            evidence.mkdir()
            for name in ("run-1", "run-2"):
                recovery = history / "recovery" / name
                recovery.mkdir(parents=True)
                (recovery / "checkpoint.json").write_text(name, encoding="utf-8")
            ledger = history / "attempts.jsonl"
            for name in ("run-1", "run-2"):
                TESTGATE._append_history(
                    ledger,
                    {
                        "record_type": "ATTEMPT",
                        "status": "CLOSED",
                        "recovery_root": str(history / "recovery" / name),
                    },
                )
            prior_provenance = history / "provenance/run-1"
            prior_provenance.mkdir(parents=True)
            prior_index = {
                "schema_version": "openwepp-testgate-attempt-index-v1",
                "provenance": {
                    "repository": "owner/repo",
                    "workflow": "testgate",
                    "run_id": "1",
                    "run_attempt": "1",
                    "head_sha": "a" * 40,
                },
                "files": [{
                    "path": "recovery/run-1/checkpoint.json",
                    "sha256": TESTGATE.hashlib.sha256(b"run-1").hexdigest(),
                }],
            }
            TESTGATE._atomic_json(prior_provenance / "attempt-index.json", prior_index)
            TESTGATE._atomic_json(
                prior_provenance / "recovery-predicate.json",
                {
                    "schema_version": "openwepp-testgate-recovery-provenance-v1",
                    "index_sha256": TESTGATE.hashlib.sha256(
                        (prior_provenance / "attempt-index.json").read_bytes()
                    ).hexdigest(),
                    **prior_index["provenance"],
                    "source_ref": "refs/heads/main",
                },
            )
            (prior_provenance / "recovery-attestation.jsonl").write_text("{}\n")
            environment = {
                "OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT": str(
                    history / "recovery/run-2"
                ),
                "GITHUB_REPOSITORY": "owner/repo",
                "GITHUB_WORKFLOW": "testgate",
                "GITHUB_REF": "refs/heads/main",
            }
            verified = subprocess.CompletedProcess([], 0, "[{}]", "")
            with mock.patch.dict(TESTGATE.os.environ, environment), mock.patch.object(
                TESTGATE.subprocess, "run", return_value=verified
            ):
                TESTGATE._snapshot_history(ledger, evidence)
            TESTGATE._write_attempt_index(
                evidence,
                {
                    "repository": "owner/repo",
                    "workflow": "testgate",
                    "run_id": "2",
                    "run_attempt": "1",
                    "head_sha": "a" * 40,
                },
            )
            restored = root / "restored"
            restored.mkdir()
            TESTGATE._verify_attempt_archive(
                evidence,
                repository="owner/repo",
                workflow="testgate",
                run_id="2",
                run_attempt="1",
                head_sha="a" * 40,
            )
            TESTGATE._restore_attempt_archive(evidence, restored)
            auth = root / "auth"
            auth.mkdir()
            (auth / "recovery-predicate.json").write_text("{}\n")
            (auth / "recovery-attestation.jsonl").write_text("{}\n")
            TESTGATE._install_recovery_provenance(evidence, auth, restored)
            self.assertEqual(
                (restored / "recovery/run-1/checkpoint.json").read_text(), "run-1"
            )
            self.assertEqual(
                (restored / "recovery/run-2/checkpoint.json").read_text(), "run-2"
            )
            self.assertTrue((restored / "provenance/run-1/attempt-index.json").is_file())
            self.assertTrue((restored / "provenance/run-2/attempt-index.json").is_file())
            (auth / "recovery-predicate.json").write_text('{"generation":2}\n')
            (auth / "recovery-attestation.jsonl").write_text('{"bundle":2}\n')
            TESTGATE._install_recovery_provenance(evidence, auth, restored)
            for name in ("run-1", "run-2"):
                self.assertEqual(
                    (restored / f"provenance/{name}/recovery-predicate.json").read_text(),
                    '{"generation":2}\n',
                )

    def test_finalize_recovery_rejects_final_and_ancestor_symlinks(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            artifact = root / "evidence"
            artifact.mkdir()
            (artifact / "receipt.json").write_text("{}\n")
            (artifact / "terminal-plan.json").write_text("{}\n")
            outside = root / "outside"
            outside.mkdir()
            ancestor = root / "history"
            ancestor.symlink_to(outside, target_is_directory=True)
            with mock.patch.dict(
                TESTGATE.os.environ,
                {"OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT": str(ancestor / "run-1")},
            ):
                with self.assertRaises(TESTGATE.TestgateError):
                    TESTGATE._finalize_recovery(artifact)
            ancestor.unlink()
            ancestor.mkdir()
            final = ancestor / "run-1"
            final.symlink_to(outside, target_is_directory=True)
            with mock.patch.dict(
                TESTGATE.os.environ,
                {"OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT": str(final)},
            ):
                with self.assertRaises(TESTGATE.TestgateError):
                    TESTGATE._finalize_recovery(artifact)

    def test_provenance_install_rejects_ancestor_symlink_without_writing_outside(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            evidence = root / "evidence"
            auth = root / "auth"
            history = root / "history"
            outside = root / "outside"
            for path in (evidence, auth, history, outside):
                path.mkdir()
            (evidence / "attempt-index.json").write_text(
                json.dumps({"files": [{"path": "recovery/run-1/checkpoint.json"}]})
            )
            (auth / "recovery-predicate.json").write_text("{}\n")
            (auth / "recovery-attestation.jsonl").write_text("{}\n")
            (history / "provenance").symlink_to(outside, target_is_directory=True)
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._install_recovery_provenance(evidence, auth, history)
            self.assertFalse((outside / "run-1").exists())

    def test_snapshot_rejects_self_asserted_retained_provenance(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            history = root / "history"
            recovery = history / "recovery/run-1"
            provenance = history / "provenance/run-1"
            evidence = root / "evidence"
            recovery.mkdir(parents=True)
            provenance.mkdir(parents=True)
            evidence.mkdir()
            (recovery / "checkpoint.json").write_text("forged\n")
            ledger = history / "attempts.jsonl"
            TESTGATE._append_history(
                ledger,
                {"record_type": "ATTEMPT", "recovery_root": str(recovery)},
            )
            for filename in (
                "attempt-index.json",
                "recovery-predicate.json",
                "recovery-attestation.jsonl",
            ):
                (provenance / filename).write_text("{}\n")
            with mock.patch.dict(
                TESTGATE.os.environ,
                {
                    "OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT": str(
                        history / "recovery/run-2"
                    ),
                    "GITHUB_REPOSITORY": "owner/repo",
                    "GITHUB_WORKFLOW": "testgate",
                    "GITHUB_REF": "refs/heads/main",
                },
            ):
                with self.assertRaises(TESTGATE.TestgateError):
                    TESTGATE._snapshot_history(ledger, evidence)
            self.assertFalse((evidence / "recovery/run-1").exists())

    def test_archive_restore_is_exact_provenance_bound_and_chain_verified(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            evidence = root / "evidence"
            history = root / "history"
            evidence.mkdir()
            history.mkdir()
            TESTGATE._append_history(
                evidence / "attempts.jsonl",
                {"record_type": "ATTEMPT", "status": "CLOSED"},
            )
            (evidence / "recovery/run-1").mkdir(parents=True)
            (evidence / "recovery/run-1/checkpoint.json").write_text("{}\n")
            provenance = {
                "repository": "owner/repo",
                "workflow": "testgate",
                "run_id": "42",
                "run_attempt": "1",
                "head_sha": "a" * 40,
            }
            TESTGATE._write_attempt_index(evidence, provenance)
            TESTGATE._verify_attempt_archive(
                evidence,
                repository="owner/repo",
                workflow="testgate",
                run_id="42",
                run_attempt="1",
                head_sha="a" * 40,
            )
            TESTGATE._restore_attempt_archive(evidence, history)
            self.assertEqual(
                (history / "recovery/run-1/checkpoint.json").read_text(), "{}\n"
            )

    def test_archive_rejects_unindexed_file_and_symlink(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            TESTGATE._append_history(
                root / "attempts.jsonl",
                {"record_type": "ATTEMPT", "status": "CLOSED"},
            )
            provenance = {
                "repository": "owner/repo",
                "workflow": "testgate",
                "run_id": "42",
                "head_sha": "a" * 40,
            }
            TESTGATE._write_attempt_index(root, provenance)
            (root / "unindexed").write_text("unsafe\n")
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._verify_attempt_archive(
                    root,
                    repository="owner/repo",
                    workflow="testgate",
                    run_id="42",
                    run_attempt="1",
                    head_sha="a" * 40,
                )
            (root / "unindexed").unlink()
            (root / "link").symlink_to(root / "missing")
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._write_attempt_index(root, provenance)

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
