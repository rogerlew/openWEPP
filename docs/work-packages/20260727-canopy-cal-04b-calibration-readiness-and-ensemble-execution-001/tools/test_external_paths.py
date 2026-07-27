from __future__ import annotations

import hashlib
import csv
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

    def test_actual_dual_executable_path_fixtures_are_science_equivalent(self) -> None:
        executor = TOOLS / "executor"
        subprocess.run(
            [
                "cargo",
                "build",
                "--manifest-path",
                str(executor / "Cargo.toml"),
                "--bins",
            ],
            cwd=ROOT,
            check=True,
            capture_output=True,
        )
        binaries = executor / "target/debug"
        with tempfile.TemporaryDirectory() as temporary:
            scratch = Path(temporary)
            missing = scratch / "same-missing-input"
            failure_commands = {
                "reconstruct": [
                    "--trace", str(missing), "--identity", str(missing),
                    "--configs", str(missing), "--observations", str(missing),
                ],
                "verify-reconstruct": [
                    "--trace", str(missing), "--identity", str(missing),
                    "--configs", str(missing), "--observations", str(missing),
                    "--primary-components", str(missing),
                    "--primary-ledgers", str(missing),
                ],
            }
            for binary, shared in failure_commands.items():
                results = []
                for label in ("legacy", "external"):
                    ledger = scratch / label / binary / "ledgers"
                    objects = scratch / label / binary / "objects"
                    result = subprocess.run(
                        [
                            str(binaries / binary),
                            *shared,
                            "--out", str(ledger),
                            *(
                                ["--component-out", str(objects)]
                                if binary == "reconstruct"
                                else []
                            ),
                        ],
                        capture_output=True,
                        text=True,
                    )
                    results.append((result, ledger, objects))
                self.assertEqual(results[0][0].returncode, results[1][0].returncode)
                self.assertNotEqual(results[0][0].returncode, 0)
                self.assertEqual(results[0][0].stderr, results[1][0].stderr)
                self.assertEqual(
                    [path.name for path in results[0][1].glob("*")],
                    [path.name for path in results[1][1].glob("*")],
                )
                self.assertEqual(
                    [path.name for path in results[0][2].glob("*")],
                    [path.name for path in results[1][2].glob("*")],
                )

            accepted = scratch / "accepted.csv"
            accepted.write_text("candidate_id\nGSI-0001\n", encoding="utf-8")
            readiness_runs = []
            for label in ("legacy", "external"):
                root = scratch / label / "readiness"
                out = root / "ledgers"
                objects = root / "objects"
                result = subprocess.run(
                    [
                        str(binaries / "readiness"),
                        "--design", str(PACKAGE / "artifacts/later-stage-design.csv"),
                        "--accepted", str(accepted),
                        "--out", str(out),
                        "--object-root", str(objects),
                    ],
                    capture_output=True,
                    text=True,
                )
                readiness_runs.append((result, root))
            self.assertEqual(readiness_runs[0][0].returncode, 0)
            self.assertEqual(readiness_runs[1][0].returncode, 0)

            def normalized_tree(root: Path) -> dict[str, bytes]:
                values = {}
                for path in sorted(item for item in root.rglob("*") if item.is_file()):
                    raw = path.read_bytes()
                    normalized = raw.replace(str(root).encode(), b"<FIXTURE_ROOT>")
                    values[path.relative_to(root).as_posix()] = normalized
                return values

            legacy_tree = normalized_tree(readiness_runs[0][1])
            external_tree = normalized_tree(readiness_runs[1][1])
            self.assertEqual(set(legacy_tree), set(external_tree))
            path_metadata = {
                "ledgers/later-stage-membership.csv",
                "objects/execution-receipt.csv",
            }
            for name in set(legacy_tree) - path_metadata:
                self.assertEqual(legacy_tree[name], external_tree[name], name)

            def csv_rows(root: Path, relative: str) -> list[dict[str, str]]:
                with (root / relative).open(newline="", encoding="utf-8") as stream:
                    return list(csv.DictReader(stream))

            legacy_membership = csv_rows(
                readiness_runs[0][1], "ledgers/later-stage-membership.csv"
            )
            external_membership = csv_rows(
                readiness_runs[1][1], "ledgers/later-stage-membership.csv"
            )
            for left, right in zip(
                legacy_membership, external_membership, strict=True
            ):
                for field in set(left) - {"membership_path", "parent_results_path"}:
                    self.assertEqual(left[field], right[field], field)
                self.assertEqual(
                    Path(left["membership_path"]).name,
                    Path(right["membership_path"]).name,
                )
                self.assertEqual(
                    Path(left["parent_results_path"]).name,
                    Path(right["parent_results_path"]).name,
                )

            def receipt(root: Path) -> dict[str, str]:
                rows = csv_rows(root, "objects/execution-receipt.csv")
                return {row["field"]: row["value"] for row in rows}

            legacy_receipt = receipt(readiness_runs[0][1])
            external_receipt = receipt(readiness_runs[1][1])
            receipt_path_allowlist = {"exact_command", "membership_index_sha256"}
            for field in set(legacy_receipt) - receipt_path_allowlist:
                self.assertEqual(
                    legacy_receipt[field], external_receipt[field], field
                )
            for name, raw in legacy_tree.items():
                if name.endswith(".csv"):
                    self.assertEqual(
                        raw.count(b"\n"),
                        external_tree[name].count(b"\n"),
                        name,
                    )

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
            by_id["build_production_runner"]["declared_outputs"],
            ["cargo-target/debug/openwepp-cli-hill"],
        )
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
