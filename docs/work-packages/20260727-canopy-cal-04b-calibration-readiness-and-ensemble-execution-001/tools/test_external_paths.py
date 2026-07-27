from __future__ import annotations

import hashlib
import csv
import io
import json
import subprocess
import tempfile
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
PACKAGE = TOOLS.parent
ROOT = PACKAGE.parents[2]
EXECUTION_SCRIPTS = (
    "observe.py",
    "prepare.py",
    "native-proof.py",
    "synthetic-gsi.py",
    "retain.py",
    "summarize.py",
    "freeze.py",
    "freeze-verify.py",
    "holdout.py",
    "validate.py",
)
FORBIDDEN = (
    "/home/workdir/cal04b-objects",
    "tools/executor/target",
    "/target/debug/openwepp-cli-hill",
)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


class ExternalPathTest(unittest.TestCase):
    def test_scripts_require_execution_root_and_reject_hard_coded_outputs(self) -> None:
        for name in EXECUTION_SCRIPTS:
            source = (TOOLS / name).read_text(encoding="utf-8")
            self.assertIn("--execution-root", source, name)
            for forbidden in FORBIDDEN:
                self.assertNotIn(forbidden, source, f"{name}: {forbidden}")

    def test_prepare_writes_only_below_execution_root_from_foreign_cwd(self) -> None:
        protected = PACKAGE / "artifacts/candidate-configurations.csv"
        before = sha256(protected)
        with tempfile.TemporaryDirectory() as temporary:
            scratch = Path(temporary)
            attempt_root = scratch / "attempt"
            execution_root = attempt_root / "objects"
            execution_root.mkdir(parents=True)
            foreign_cwd = scratch / "cwd"
            foreign_cwd.mkdir()
            subprocess.run(
                [
                    str(ROOT / ".venv/bin/python"),
                    str(TOOLS / "prepare.py"),
                    "--execution-root",
                    str(execution_root),
                ],
                cwd=foreign_cwd,
                check=True,
                capture_output=True,
                text=True,
            )
            publication = (
                attempt_root
                / "publication"
                / PACKAGE.relative_to(ROOT)
                / "artifacts/candidate-configurations.csv"
            )
            self.assertTrue(publication.is_file())
            self.assertFalse((foreign_cwd / "candidate-configurations.csv").exists())
        self.assertEqual(sha256(protected), before)

    def test_rust_outputs_are_injected_and_verification_derives_primary_path(self) -> None:
        executor = TOOLS / "executor/src/bin"
        reconstruct = (executor / "reconstruct.rs").read_text(encoding="utf-8")
        verify = (executor / "verify_reconstruct.rs").read_text(encoding="utf-8")
        readiness = (executor / "readiness.rs").read_text(encoding="utf-8")
        self.assertIn('arg_value(&args, "--component-out")', reconstruct)
        self.assertIn('arg_value(&args, "--object-root")', readiness)
        self.assertIn(
            'primary_components\n                .join("candidate-observation-components.csv")',
            verify,
        )
        combined = reconstruct + verify + readiness
        self.assertNotIn("/home/workdir/cal04b-objects", combined)

    def test_dual_path_fixture_changes_only_normalized_evidence_path(self) -> None:
        header = (
            "candidate_id,configuration_id,state,objective,boundary_flags,"
            "saturation_flags,evidence\n"
        )
        scientific = "GSI-0001,CFG-0001,FINITE,1.25,NONE,NONE,"
        legacy = header + scientific + "/legacy/primary/candidate-observation-components.csv\n"
        external = header + scientific + "/attempt/objects/primary/candidate-observation-components.csv\n"

        def normalized(value: str) -> tuple[list[tuple[str, ...]], list[str]]:
            reader = csv.DictReader(io.StringIO(value))
            scientific_rows = []
            evidence = []
            for row in reader:
                evidence.append(Path(row.pop("evidence")).name)
                scientific_rows.append(tuple(row[field] for field in reader.fieldnames[:-1]))
            return scientific_rows, evidence

        legacy_rows, legacy_paths = normalized(legacy)
        external_rows, external_paths = normalized(external)
        self.assertEqual(len(legacy_rows), len(external_rows))
        self.assertEqual(legacy_rows, external_rows)
        self.assertEqual(legacy_paths, external_paths)

    def test_external_plan_maps_all_frozen_rows_and_path_injections(self) -> None:
        artifact_root = PACKAGE / "artifacts"
        plan = json.loads(
            (artifact_root / "external-dag-transaction-plan.json").read_text(
                encoding="utf-8"
            )
        )
        identity = plan["plan_id"]
        payload = dict(plan)
        payload.pop("plan_id")
        derived = hashlib.sha256(
            json.dumps(payload, sort_keys=True, separators=(",", ":")).encode()
        ).hexdigest()
        self.assertEqual(identity, derived)
        nodes = [
            *plan["custody_commands"],
            *[
                node
                for transaction in plan["transactions"]
                for node in (*transaction["light"], *transaction["heavy"])
            ],
        ]
        with (artifact_root / "executor-command-plan.csv").open(
            newline="", encoding="utf-8"
        ) as stream:
            frozen = list(csv.DictReader(stream))
        self.assertEqual(len(nodes), 18)
        self.assertEqual(
            {node["command_id"] for node in nodes},
            {row["command_id"] for row in frozen},
        )
        by_id = {node["command_id"]: node for node in nodes}
        self.assertEqual(
            by_id["hubbard_primary_reconstruct"]["argv"][-2:],
            ["--component-out", "${OBJECTS_ROOT}/primary"],
        )
        self.assertEqual(
            by_id["readiness"]["argv"][-2:],
            ["--object-root", "${OBJECTS_ROOT}/readiness"],
        )


if __name__ == "__main__":
    unittest.main()
