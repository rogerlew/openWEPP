from __future__ import annotations

import importlib.util
import io
import json
import subprocess
import tempfile
import unittest
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path
from types import SimpleNamespace
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
    def test_final_observation_binds_retained_package_authorization(self) -> None:
        authorization = {"package_authority_chain_id": "c" * 64}
        for result in ("PASS", "FAIL"):
            with self.subTest(result=result):
                observation = TESTGATE._final_observation(
                    {"execution_result": {"result": result}}, authorization
                )
                self.assertIs(observation["intent_authorization"], authorization)
                self.assertIs(observation["package_audit"], authorization)
                self.assertEqual(observation["execution_result"]["result"], result)

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

    def test_ledger_bootstrap_creates_durable_regular_file_once(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "new/deep/attempts.jsonl"
            with mock.patch.object(
                TESTGATE.os, "fsync", wraps=TESTGATE.os.fsync
            ) as fsync:
                guard = TESTGATE._open_ledger_guard(ledger, create=True)
                try:
                    self.assertEqual(guard.path, ledger)
                    self.assertEqual(ledger.read_bytes(), b"")
                    self.assertTrue(TESTGATE.stat.S_ISREG(ledger.lstat().st_mode))
                    self.assertEqual(ledger.stat().st_mode & 0o777, 0o600)
                    self.assertGreaterEqual(fsync.call_count, 2)
                    self.assertEqual(
                        [call.args[0] for call in fsync.call_args_list[-2:]],
                        [
                            guard._file_descriptor,
                            guard._directories[-1][0],
                        ],
                    )
                finally:
                    guard.close()

    def test_ledger_bootstrap_fails_when_required_primitives_are_unavailable(self) -> None:
        cases = (
            mock.patch.object(TESTGATE.os, "O_NOFOLLOW", None),
            mock.patch.object(TESTGATE.os, "supports_dir_fd", set()),
            mock.patch.object(TESTGATE.os, "supports_follow_symlinks", set()),
        )
        for unavailable in cases:
            with self.subTest(unavailable=unavailable):
                with tempfile.TemporaryDirectory() as directory, unavailable:
                    with self.assertRaises(TESTGATE.TestgateError):
                        TESTGATE._open_ledger_guard(
                            Path(directory) / "attempts.jsonl", create=True
                        )

    def test_ledger_bootstrap_preserves_valid_existing_bytes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "attempts.jsonl"
            TESTGATE._append_history(
                ledger, {"record_type": "ATTEMPT", "status": "CLOSED"}
            )
            before = ledger.read_bytes()
            guard = TESTGATE._open_ledger_guard(ledger, create=True)
            try:
                self.assertEqual(guard.read_bytes(), before)
            finally:
                guard.close()
            self.assertEqual(ledger.read_bytes(), before)

    def test_ledger_bootstrap_rejects_final_and_ancestor_symlinks(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            outside = root / "outside"
            outside.mkdir()
            outside_ledger = outside / "attempts.jsonl"
            outside_ledger.write_text("outside\n", encoding="utf-8")
            final = root / "final.jsonl"
            final.symlink_to(outside_ledger)
            with self.assertRaises((OSError, TESTGATE.TestgateError)):
                TESTGATE._open_ledger_guard(final, create=True)
            ancestor = root / "history"
            ancestor.symlink_to(outside, target_is_directory=True)
            with self.assertRaises((OSError, TESTGATE.TestgateError)):
                TESTGATE._open_ledger_guard(
                    ancestor / "attempts.jsonl", create=True
                )
            self.assertEqual(outside_ledger.read_text(encoding="utf-8"), "outside\n")

    def test_ledger_bootstrap_rejects_directory_fifo_and_unsafe_components(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            directory_ledger = root / "directory"
            directory_ledger.mkdir()
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._open_ledger_guard(directory_ledger, create=True)
            if hasattr(TESTGATE.os, "mkfifo"):
                fifo = root / "fifo"
                TESTGATE.os.mkfifo(fifo)
                with self.assertRaises(TESTGATE.TestgateError):
                    TESTGATE._open_ledger_guard(fifo, create=True)
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._open_ledger_guard(
                    Path(directory) / "safe/../escape.jsonl", create=True
                )

    def test_ledger_bootstrap_rejects_malformed_chains_without_mutation(self) -> None:
        malformed = (
            b"{not-json}\n",
            b'{"previous_entry_sha256":null,"entry_sha256":"bad"}\n',
            (
                b'{"entry_sha256":'
                + json.dumps("0" * 64).encode()
                + b',"previous_entry_sha256":"unexpected"}\n'
            ),
        )
        for content in malformed:
            with self.subTest(content=content):
                with tempfile.TemporaryDirectory() as directory:
                    ledger = Path(directory) / "attempts.jsonl"
                    ledger.write_bytes(content)
                    with self.assertRaises(
                        (json.JSONDecodeError, TESTGATE.TestgateError)
                    ):
                        TESTGATE._open_ledger_guard(ledger, create=True)
                    self.assertEqual(ledger.read_bytes(), content)

    def test_ledger_exclusive_create_collision_preserves_competing_file(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            ledger = Path(directory) / "attempts.jsonl"
            real_open = TESTGATE.os.open

            def collide(path: object, flags: int, *args: object, **kwargs: object) -> int:
                if flags & TESTGATE.os.O_EXCL:
                    ledger.write_bytes(b"competitor\n")
                    raise FileExistsError
                return real_open(path, flags, *args, **kwargs)

            with (
                mock.patch.object(TESTGATE.os, "open", side_effect=collide),
                mock.patch.object(
                    TESTGATE.os,
                    "supports_dir_fd",
                    {*TESTGATE.os.supports_dir_fd, TESTGATE.os.open},
                ),
            ):
                with self.assertRaisesRegex(
                    TESTGATE.TestgateError, "exclusive creation collided"
                ):
                    TESTGATE._open_ledger_guard(ledger, create=True)
            self.assertEqual(ledger.read_bytes(), b"competitor\n")

    def test_bootstrap_failure_never_invokes_transition(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            outside = root / "outside"
            outside.mkdir()
            ledger = root / "history/attempts.jsonl"
            ledger.parent.symlink_to(outside, target_is_directory=True)
            args = mock.Mock()
            args.execute = True
            args.history_ledger = ledger
            args.artifact_root = root / "artifacts"
            with (
                mock.patch.object(TESTGATE, "_parse_args", return_value=args),
                mock.patch.object(TESTGATE, "_invoke") as invoke,
                redirect_stderr(io.StringIO()),
            ):
                self.assertEqual(TESTGATE.main(), 2)
            invoke.assert_not_called()
            self.assertFalse((outside / "attempts.jsonl").exists())

    def test_ledger_file_swap_is_rejected_before_append_and_finalization(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            ledger = root / "history/attempts.jsonl"
            guard = TESTGATE._open_ledger_guard(ledger, create=True)
            try:
                retained = ledger.with_name("retained.jsonl")
                ledger.rename(retained)
                ledger.write_bytes(b"replacement\n")
                with self.assertRaises(TESTGATE.TestgateError):
                    TESTGATE._append_history(
                        ledger, {"record_type": "ATTEMPT"}, guard
                    )
                artifacts = root / "artifacts"
                artifacts.mkdir()
                with self.assertRaises(TESTGATE.AttemptFinalizationError):
                    TESTGATE._finalize_attempt_archive(ledger, artifacts, guard)
                self.assertEqual(ledger.read_bytes(), b"replacement\n")
                self.assertEqual(retained.read_bytes(), b"")
            finally:
                guard.close()

    def test_ledger_ancestor_swap_is_rejected_without_outside_write(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            history = root / "history"
            ledger = history / "attempts.jsonl"
            guard = TESTGATE._open_ledger_guard(ledger, create=True)
            outside = root / "outside"
            outside.mkdir()
            moved = root / "retained-history"
            history.rename(moved)
            history.symlink_to(outside, target_is_directory=True)
            try:
                with self.assertRaises(TESTGATE.TestgateError):
                    guard.validate()
                with self.assertRaises(TESTGATE.TestgateError):
                    TESTGATE._append_history(
                        ledger, {"record_type": "ATTEMPT"}, guard
                    )
                self.assertFalse((outside / "attempts.jsonl").exists())
                self.assertEqual((moved / "attempts.jsonl").read_bytes(), b"")
            finally:
                guard.close()

    def test_ledger_ancestor_swap_is_rejected_before_finalization(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            history = root / "history"
            ledger = history / "attempts.jsonl"
            guard = TESTGATE._open_ledger_guard(ledger, create=True)
            outside = root / "outside"
            outside.mkdir()
            moved = root / "retained-history"
            history.rename(moved)
            history.symlink_to(outside, target_is_directory=True)
            artifacts = root / "artifacts"
            artifacts.mkdir()
            try:
                with self.assertRaises(TESTGATE.AttemptFinalizationError):
                    TESTGATE._finalize_attempt_archive(ledger, artifacts, guard)
                self.assertFalse((outside / "attempts.jsonl").exists())
                self.assertEqual((moved / "attempts.jsonl").read_bytes(), b"")
            finally:
                guard.close()

    def test_observe_closes_only_guard_it_acquires_on_success_and_error(self) -> None:
        for failure in (False, True):
            with self.subTest(failure=failure):
                with tempfile.TemporaryDirectory() as directory:
                    args = SimpleNamespace()
                    guard = TESTGATE._open_ledger_guard(
                        Path(directory) / "attempts.jsonl", create=True
                    )

                    def implementation(_args: object) -> dict:
                        _args._history_ledger_guard = guard
                        if failure:
                            raise TESTGATE.TestgateError("injected")
                        return {"result": "PASS"}

                    with mock.patch.object(
                        TESTGATE, "_observe", side_effect=implementation
                    ):
                        if failure:
                            with self.assertRaises(TESTGATE.TestgateError):
                                TESTGATE.observe(args)
                        else:
                            self.assertEqual(
                                TESTGATE.observe(args), {"result": "PASS"}
                            )
                    with self.assertRaisesRegex(
                        TESTGATE.TestgateError, "authority is closed"
                    ):
                        guard.validate()

    def test_observe_does_not_close_caller_owned_guard(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            guard = TESTGATE._open_ledger_guard(
                Path(directory) / "attempts.jsonl", create=True
            )
            args = SimpleNamespace(_history_ledger_guard=guard)
            try:
                with mock.patch.object(
                    TESTGATE, "_observe", return_value={"result": "PASS"}
                ):
                    self.assertEqual(TESTGATE.observe(args), {"result": "PASS"})
                guard.validate()
            finally:
                guard.close()

    def test_ledger_swap_is_rejected_before_transition_invoke(self) -> None:
        with tempfile.TemporaryDirectory(dir="/home/workdir") as directory:
            root = Path(directory)
            repo = root / "repo"
            repo.mkdir()
            ledger = root / "history/attempts.jsonl"
            guard = TESTGATE._open_ledger_guard(ledger, create=True)
            args = mock.Mock(
                repo=repo,
                artifact_root=root / "artifacts",
                history_ledger=ledger,
                binary=repo / "gate",
                base="base",
                head="head",
                dirty=False,
                intent_package="docs/work-packages/example/package.md",
                execute=True,
                boundary="INCREMENT",
                campaign="CAMPAIGN",
                principal="developer",
                repository="owner/repo",
                source_event="local",
                source_ref="refs/heads/main",
                workflow="testgate",
                job="job",
                runner="runner",
                attempt=1,
                _history_ledger_guard=guard,
            )
            calls = 0

            def invoke(arguments: list[str], _repo: Path, **_kwargs: object) -> dict:
                nonlocal calls
                calls += 1
                output = Path(arguments[arguments.index("--output") + 1])
                if calls == 1:
                    output.write_text('{"plan_id":"intent"}\n', encoding="utf-8")
                    return {"plan_id": "intent"}
                if calls == 2:
                    output.write_text(
                        json.dumps(
                            {
                                "plan_id": "terminal",
                                "nodes": [{"execution_cost_class": "HEAVY"}],
                                "risk": {"class": "CRITICAL", "reason_codes": []},
                            }
                        ),
                        encoding="utf-8",
                    )
                    retained = ledger.with_name("retained.jsonl")
                    ledger.rename(retained)
                    ledger.write_bytes(b"replacement\n")
                    return {"plan_id": "terminal"}
                self.fail("transition was invoked after ledger substitution")

            def authorize(
                _repo: Path,
                _binary: Path,
                _base: str,
                _head: str,
                _paths: list[str],
                _package: str,
                output: Path,
            ) -> dict:
                value = {"status": "READY"}
                output.write_text(json.dumps(value), encoding="utf-8")
                return value

            try:
                with (
                    mock.patch.object(
                        TESTGATE, "_resolve_commit", side_effect=("a" * 40, "b" * 40)
                    ),
                    mock.patch.object(
                        TESTGATE, "_changed_paths", return_value=["changed"]
                    ),
                    mock.patch.object(
                        TESTGATE, "_intent_authorization", side_effect=authorize
                    ),
                    mock.patch.object(TESTGATE, "_invoke", side_effect=invoke),
                    mock.patch.object(TESTGATE, "_initialize_recovery_plan"),
                ):
                    with self.assertRaises(TESTGATE.TestgateError):
                        TESTGATE.observe(args)
                self.assertEqual(calls, 2)
                self.assertEqual(ledger.read_bytes(), b"replacement\n")
            finally:
                guard.close()

    def test_attempt_index_covers_pre_receipt_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "pre-receipt-failure.json").write_text("{}\n", encoding="utf-8")
            TESTGATE._write_attempt_index(root)
            index = json.loads((root / "attempt-index.json").read_text())
        self.assertEqual(index["files"][0]["path"], "pre-receipt-failure.json")

    def test_attempt_index_prunes_caches_but_retains_execution_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cache = root / "execution/.work/cargo-target/debug"
            report = root / "execution/.work/nextest/full/junit.xml"
            attempt = root / "execution/.attempts/node-1.log"
            for path in (cache / "binary", report, attempt):
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("retained or disposable\n", encoding="utf-8")
            TESTGATE._prune_disposable_execution_state(root)
            TESTGATE._write_attempt_index(root)
            index = json.loads((root / "attempt-index.json").read_text())
            paths = {item["path"] for item in index["files"]}
        self.assertFalse(cache.exists())
        self.assertEqual(
            paths,
            {
                "execution/.attempts/node-1.log",
                "execution/.work/nextest/full/junit.xml",
            },
        )

    def test_attempt_cache_pruning_rejects_symlink_escape(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            work = root / "execution/.work"
            outside = root / "outside"
            work.mkdir(parents=True)
            outside.mkdir()
            (work / "cargo-target").symlink_to(outside, target_is_directory=True)
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._prune_disposable_execution_state(root)
            self.assertTrue(outside.exists())

    def test_main_does_not_repeat_a_failed_attempt_finalizer(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            args = mock.Mock()
            args.artifact_root = Path(directory)
            args.history_ledger = args.artifact_root / "history.jsonl"
            failure = TESTGATE.AttemptFinalizationError("injected finalizer failure")
            with (
                mock.patch.object(TESTGATE, "_parse_args", return_value=args),
                mock.patch.object(TESTGATE, "observe", side_effect=failure),
                mock.patch.object(TESTGATE, "_finalize_attempt_archive") as finalize,
                redirect_stderr(io.StringIO()) as stderr,
            ):
                self.assertEqual(TESTGATE.main(), 2)
            finalize.assert_not_called()
            self.assertTrue(
                (args.artifact_root / "pre-receipt-failure.json").is_file()
            )
            self.assertIn("injected finalizer failure", stderr.getvalue())

    def test_main_reports_failure_when_error_archive_finalization_also_fails(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            args = mock.Mock()
            args.artifact_root = Path(directory)
            args.history_ledger = args.artifact_root / "history.jsonl"
            with (
                mock.patch.object(TESTGATE, "_parse_args", return_value=args),
                mock.patch.object(
                    TESTGATE,
                    "observe",
                    side_effect=TESTGATE.TestgateError("injected observation failure"),
                ),
                mock.patch.object(
                    TESTGATE,
                    "_finalize_attempt_archive",
                    side_effect=TESTGATE.AttemptFinalizationError(
                        "injected secondary failure"
                    ),
                ) as finalize,
                redirect_stderr(io.StringIO()) as stderr,
            ):
                self.assertEqual(TESTGATE.main(), 2)
            finalize.assert_called_once()
            self.assertIn("injected observation failure", stderr.getvalue())
            self.assertIn("injected secondary failure", stderr.getvalue())

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

    def test_archive_restore_replaces_only_safe_empty_ledger_placeholder(self) -> None:
        for source_is_empty in (True, False):
            with (
                self.subTest(source_is_empty=source_is_empty),
                tempfile.TemporaryDirectory() as directory,
            ):
                root = Path(directory)
                evidence = root / "evidence"
                history = root / "history"
                evidence.mkdir()
                history.mkdir()
                if source_is_empty:
                    (evidence / "attempts.jsonl").touch()
                else:
                    TESTGATE._append_history(
                        evidence / "attempts.jsonl",
                        {"record_type": "ATTEMPT", "status": "CLOSED"},
                    )
                (history / "attempts.jsonl").touch()

                TESTGATE._restore_attempt_archive(evidence, history)

                self.assertEqual(
                    (history / "attempts.jsonl").read_bytes(),
                    (evidence / "attempts.jsonl").read_bytes(),
                )
                self.assertEqual(
                    (history / "attempts.jsonl").stat().st_mode & 0o777, 0o600
                )

    def test_archive_restore_rejects_symlinked_history_chain(self) -> None:
        cases = ("history-root", "history-ancestor")
        for case in cases:
            with self.subTest(case=case), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                evidence = root / "evidence"
                outside = root / "outside"
                evidence.mkdir()
                outside.mkdir()
                (evidence / "attempts.jsonl").touch()
                if case == "history-root":
                    history = root / "history"
                    history.symlink_to(outside, target_is_directory=True)
                else:
                    ancestor = root / "ancestor"
                    ancestor.symlink_to(outside, target_is_directory=True)
                    history = ancestor / "history"
                    (outside / "history").mkdir()

                with self.assertRaisesRegex(
                    TESTGATE.TestgateError,
                    "recovery directory is not a real directory",
                ):
                    TESTGATE._restore_attempt_archive(evidence, history)
                self.assertFalse((outside / "attempts.jsonl").exists())
                self.assertFalse((outside / "history/attempts.jsonl").exists())

    def test_archive_restore_rejects_occupied_or_unsafe_destination(self) -> None:
        cases = ("nonempty-ledger", "extra-entry", "ledger-directory", "ledger-symlink")
        for case in cases:
            with self.subTest(case=case), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                evidence = root / "evidence"
                history = root / "history"
                evidence.mkdir()
                history.mkdir()
                (evidence / "attempts.jsonl").write_text("")
                placeholder = history / "attempts.jsonl"
                if case == "nonempty-ledger":
                    placeholder.write_text("{}\n")
                elif case == "extra-entry":
                    placeholder.touch()
                    (history / "other").touch()
                elif case == "ledger-directory":
                    placeholder.mkdir()
                else:
                    placeholder.symlink_to(evidence / "attempts.jsonl")

                with self.assertRaisesRegex(
                    TESTGATE.TestgateError,
                    "history restore destination is not empty",
                ):
                    TESTGATE._restore_attempt_archive(evidence, history)

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

    def test_intent_authorization_consumes_exact_rust_chain(self) -> None:
        changed = ["docs/work-packages/example/package.md", "src/lib.rs"]
        chain_id = "c" * 64
        chain = {
            "status": "READY",
            "base_commit": "a" * 40,
            "head_commit": "b" * 40,
            "intent_package_path": "docs/work-packages/example/package.md",
            "changed_paths": changed,
            "package_authority_chain_id": chain_id,
        }
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "chain.json"
            output.write_text(json.dumps(chain), encoding="utf-8")
            with mock.patch.object(
                TESTGATE,
                "_invoke",
                return_value={
                    "result": "READY",
                    "package_authority_chain_id": chain_id,
                },
            ):
                authorization = TESTGATE._intent_authorization(
                    ROOT,
                    ROOT / "target/release/openwepp-gate-plan",
                    "a" * 40,
                    "b" * 40,
                    changed,
                    "docs/work-packages/example/package.md",
                    output,
                )
            self.assertEqual(authorization["changed_paths"], changed)

            chain["changed_paths"] = [*changed, "undeclared"]
            output.write_text(json.dumps(chain), encoding="utf-8")
            with mock.patch.object(
                TESTGATE,
                "_invoke",
                return_value={
                    "result": "READY",
                    "package_authority_chain_id": chain_id,
                },
            ):
                with self.assertRaises(TESTGATE.TestgateError):
                    TESTGATE._intent_authorization(
                        ROOT,
                        ROOT / "target/release/openwepp-gate-plan",
                        "a" * 40,
                        "b" * 40,
                        changed,
                        "docs/work-packages/example/package.md",
                        output,
                    )

    def test_intent_package_is_explicit_and_paths_are_nonempty(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "chain.json"
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._intent_authorization(
                    ROOT,
                    ROOT / "target/release/openwepp-gate-plan",
                    "a" * 40,
                    "b" * 40,
                    [],
                    "docs/work-packages/example/package.md",
                    output,
                )
            with self.assertRaises(TESTGATE.TestgateError):
                TESTGATE._intent_authorization(
                    ROOT,
                    ROOT / "target/release/openwepp-gate-plan",
                    "a" * 40,
                    "b" * 40,
                    ["src/lib.rs"],
                    "invalid/package.md",
                    output,
                )

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

    def test_invoke_transports_only_explicit_inherited_descriptors(self) -> None:
        completed = subprocess.CompletedProcess(
            args=["gate"], returncode=0, stdout='{"result":"PASS"}', stderr=""
        )
        with mock.patch.object(
            TESTGATE.subprocess, "run", return_value=completed
        ) as run:
            value = TESTGATE._invoke(["gate"], ROOT, pass_fds=(17,))
        self.assertEqual(value["result"], "PASS")
        self.assertEqual(run.call_args.kwargs["pass_fds"], (17,))

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
