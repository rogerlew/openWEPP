from __future__ import annotations

import importlib.util
import hashlib
import json
import secrets
import subprocess
import sys
import tempfile
import unittest
from argparse import Namespace
from pathlib import Path
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "cal04b_execute_prefix", TOOLS / "execute-prefix.py"
)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ExecutePrefixTest(unittest.TestCase):
    def options(self, root: Path, transaction: str = "calibration-v1") -> Namespace:
        return Namespace(
            execution_root=root,
            transaction_id=transaction,
            principal="worker",
            repository="openwepp/openwepp",
            source_event="local",
            source_ref="refs/heads/main",
            workflow="cal04b",
            job="external-transition",
            runner="forest1",
            attempt=1,
            control_root=root.with_name(f"{root.name}.control"),
            custody_root=root.with_name(f"{root.name}.custody"),
            external_plan=MODULE.PLAN,
        )

    def test_exact_transaction_command_is_argument_array(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            command = MODULE.command(self.options(root), root / "planner")
        self.assertEqual(command[1], "run-external-transition")
        self.assertIn("calibration-v1", command)
        self.assertNotIn("--opening-token", command)

    def test_holdout_binds_custody_and_opening_token(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            options = self.options(root, "holdout-v1")
            options.custody_root.mkdir()
            command = MODULE.command(options, root / "planner")
        self.assertIn("--custody-root", command)
        self.assertIn("--opening-token", command)
        self.assertIn(str(options.custody_root / "holdout-opened-once.lock"), command)

    def test_planner_build_uses_attempt_local_cargo_target(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            with patch.object(MODULE.subprocess, "run") as run:
                binary = MODULE.planner_binary(root)
            environment = run.call_args.kwargs["env"]
        target = root.with_name(f"{root.name}.planner-target")
        self.assertEqual(environment["CARGO_TARGET_DIR"], str(target))
        self.assertEqual(binary, target / "debug/openwepp-gate-plan")

    def generation_fixture(
        self, root: Path
    ) -> tuple[Path, Path, list[Path], Path, Path, Path]:
        repository = root / "source"
        repository.mkdir()
        base_plan = repository / "generation-a.json"
        base_plan.write_bytes(MODULE.PLAN.read_bytes())
        subprocess.run(["git", "init", "-q"], cwd=repository, check=True)
        subprocess.run(
            ["git", "config", "user.email", "cal04b@example.invalid"],
            cwd=repository,
            check=True,
        )
        subprocess.run(
            ["git", "config", "user.name", "CAL-04B test"],
            cwd=repository,
            check=True,
        )
        subprocess.run(["git", "add", "generation-a.json"], cwd=repository, check=True)
        subprocess.run(
            ["git", "commit", "-qm", "Generation A fixture"],
            cwd=repository,
            check=True,
        )
        planner = root / "verify-plan"
        planner.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
        planner.chmod(0o755)
        custody = root / "custody"
        (custody / "capabilities").mkdir(parents=True)
        (custody / "freeze-receipts").mkdir()
        calibration = root / "calibration.json"
        calibration.write_text(
            json.dumps(
                {
                    "receipt_id": "a" * 64,
                    "result": "PASS",
                    "transaction_id": "calibration-v1",
                },
                sort_keys=True,
                separators=(",", ":"),
            ),
            encoding="utf-8",
        )
        freeze = custody / "freeze.receipt.json"
        freeze_value = {
            "calibration_receipt_sha256": hashlib.sha256(
                calibration.read_bytes()
            ).hexdigest(),
            "freeze_digest": "b" * 64,
            "freeze_receipt_id": "",
            "result": "PASS",
            "schema": "cal04b-freeze-receipt-v1",
        }
        freeze_value["freeze_receipt_id"] = MODULE.derived_id(
            freeze_value, "freeze_receipt_id"
        )
        freeze.write_text(
            json.dumps(freeze_value, sort_keys=True, separators=(",", ":")),
            encoding="utf-8",
        )
        attestations = []
        script = TOOLS / "freeze-verify.py"
        for suffix, principal in (("a", "alice"), ("b", "bob")):
            verifier = f"verifier_{suffix}"
            receipt = custody / "freeze-receipts" / f"{verifier}.csv"
            receipt.write_text("state\nPASS\n", encoding="utf-8")
            capability = secrets.token_bytes(32)
            capability_hash = hashlib.sha256(capability).hexdigest()
            (custody / "capabilities" / f"{capability_hash}.cap").write_bytes(
                capability
            )
            value = {
                "schema": "openwepp-external-verifier-attestation-v1",
                "attestation_id": "",
                "capability_hash": capability_hash,
                "parent_dispatch_id": "dispatch-1",
                "agent_task_id": f"task-{suffix}",
                "principal": principal,
                "workflow": "cal04b",
                "job": f"verify-{suffix}",
                "runner": f"runner-{suffix}",
                "attempt": 1,
                "script_sha256": hashlib.sha256(script.read_bytes()).hexdigest(),
                "argv": [
                    str(script),
                    "--verifier-id",
                    verifier,
                ],
                "receipt_sha256": hashlib.sha256(receipt.read_bytes()).hexdigest(),
                "freeze_digest": "b" * 64,
                "created_at": "2026-07-27T12:00:00+00:00",
            }
            value["attestation_id"] = MODULE.derived_id(value, "attestation_id")
            path = custody / f"freeze_verify_{suffix}.json"
            path.write_text(
                json.dumps(value, sort_keys=True, separators=(",", ":")),
                encoding="utf-8",
            )
            attestations.append(path)
        return custody, calibration, attestations, freeze, base_plan, planner

    def test_generation_b_binds_exact_custody_and_is_deterministic(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            custody, calibration, attestations, freeze, base_plan, planner = (
                self.generation_fixture(root)
            )
            first = MODULE.build_generation_b(
                base_plan, calibration, freeze, attestations, custody, planner
            )
            second = MODULE.build_generation_b(
                base_plan, calibration, freeze, attestations, custody, planner
            )
            output = root / "generation-b.json"
            subprocess.run(
                [
                    sys.executable,
                    str(TOOLS / "execute-prefix.py"),
                    "generate-holdout-plan",
                    "--base-plan",
                    str(base_plan),
                    "--calibration-receipt",
                    str(calibration),
                    "--freeze-receipt",
                    str(freeze),
                    *[
                        argument
                        for path in attestations
                        for argument in ("--attestation", str(path))
                    ],
                    "--custody-root",
                    str(custody),
                    "--output",
                    str(output),
                    "--planner",
                    str(planner),
                ],
                check=True,
                capture_output=True,
                text=True,
            )
            generated = json.loads(output.read_text(encoding="utf-8"))
            base_bytes = base_plan.read_bytes()
            base_value = json.loads(base_plan.read_text(encoding="utf-8"))
        self.assertEqual(first, second)
        self.assertEqual(first, generated)
        self.assertEqual(first["generation"], "B")
        self.assertEqual(first["parent_plan"]["path"], str(base_plan.resolve()))
        self.assertEqual(
            first["parent_plan"]["sha256"],
            hashlib.sha256(base_bytes).hexdigest(),
        )
        self.assertEqual(
            first["parent_plan"]["plan_id"],
            MODULE.derived_id(base_value, "plan_id"),
        )
        self.assertEqual(
            first["source_identity"]["diff_sha256"], hashlib.sha256(b"").hexdigest()
        )
        holdout = next(
            transaction
            for transaction in first["transactions"]
            if transaction["transaction_id"] == "holdout-v1"
        )
        self.assertEqual(
            holdout["custody_prerequisites"],
            ["freeze_verify_a.json", "freeze_verify_b.json"],
        )
        self.assertEqual(
            {binding["kind"] for binding in holdout["custody_receipts"]},
            {"TRANSACTION", "FREEZE"},
        )
        self.assertEqual(first["plan_id"], MODULE.derived_id(first, "plan_id"))

    def test_generation_b_rejects_attestation_replay_and_receipt_drift(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            custody, calibration, attestations, freeze, base_plan, planner = (
                self.generation_fixture(root)
            )
            with self.assertRaises(ValueError):
                MODULE.build_generation_b(
                    base_plan,
                    calibration,
                    freeze,
                    [attestations[0], attestations[0]],
                    custody,
                    planner,
                )
            receipt = custody / "freeze-receipts/verifier_a.csv"
            receipt.write_text("state\nDRIFT\n", encoding="utf-8")
            with self.assertRaises(ValueError):
                MODULE.build_generation_b(
                    base_plan, calibration, freeze, attestations, custody, planner
                )

    def test_generation_b_rejects_receipt_rejected_by_rust_verifier(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            custody, calibration, attestations, freeze, base_plan, planner = (
                self.generation_fixture(root)
            )
            planner.write_text("#!/bin/sh\nexit 23\n", encoding="utf-8")
            with self.assertRaises(subprocess.CalledProcessError):
                MODULE.build_generation_b(
                    base_plan, calibration, freeze, attestations, custody, planner
                )

    def test_coordinator_cli_rejects_existing_attempt_before_planner_build(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            result = subprocess.run(
                [
                    sys.executable,
                    str(TOOLS / "execute-prefix.py"),
                    "--execution-root",
                    str(root),
                    "--transaction-id",
                    "calibration-v1",
                    "--principal",
                    "worker",
                    "--repository",
                    "openwepp/openwepp",
                    "--source-event",
                    "local",
                    "--source-ref",
                    "refs/heads/main",
                    "--workflow",
                    "cal04b",
                    "--job",
                    "calibration",
                    "--runner",
                    "forest1",
                    "--attempt",
                    "1",
                ],
                capture_output=True,
                text=True,
            )
        self.assertEqual(result.returncode, 1)
        self.assertIn("execution root must be a fresh path", result.stderr)


if __name__ == "__main__":
    unittest.main()
