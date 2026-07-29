from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("cal04b_publish_results", TOOLS / "publish-results.py")
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class PublishResultsTest(unittest.TestCase):
    def fixture(self, root: Path) -> tuple[Path, Path, Path]:
        repository = root / "repo"
        package = repository / "docs/work-packages/cal04b"
        destination = package / "artifacts"
        destination.mkdir(parents=True)
        attempt = root / "attempt"
        execution = attempt / "objects"
        execution.mkdir(parents=True)
        source = attempt / "publication" / package.relative_to(repository) / "artifacts"
        source.mkdir(parents=True)
        return repository, package, execution

    def test_plan_is_non_mutating_and_apply_is_bounded(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            repository, package, execution = self.fixture(root)
            source = MODULE.source_artifacts(execution)
            with patch.object(MODULE, "ROOT", repository), patch.object(
                MODULE, "PACKAGE", package
            ):
                source = MODULE.source_artifacts(execution)
                (source / "candidate-ledger.csv").write_text("id\n1\n")
                actions = MODULE.publish(execution, apply=False, replace=False)
                self.assertEqual(actions, ["CREATE candidate-ledger.csv"])
                self.assertFalse((package / "artifacts/candidate-ledger.csv").exists())
                MODULE.publish(execution, apply=True, replace=False)
                self.assertEqual(
                    (package / "artifacts/candidate-ledger.csv").read_text(), "id\n1\n"
                )

    def test_unrecognized_result_and_differing_replace_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            repository, package, execution = self.fixture(root)
            with patch.object(MODULE, "ROOT", repository), patch.object(
                MODULE, "PACKAGE", package
            ):
                source = MODULE.source_artifacts(execution)
                (source / "unexpected.txt").write_text("no")
                with self.assertRaises(ValueError):
                    MODULE.publish(execution, apply=False, replace=False)
                (source / "unexpected.txt").unlink()
                (source / "candidate-ledger.csv").write_text("new")
                (package / "artifacts/candidate-ledger.csv").write_text("old")
                with self.assertRaises(ValueError):
                    MODULE.publish(execution, apply=True, replace=False)

    def test_holdout_publication_is_bounded(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            repository, package, execution = self.fixture(root)
            holdout = root / "holdout"
            (holdout / "artifacts").mkdir(parents=True)
            with patch.object(MODULE, "ROOT", repository), patch.object(
                MODULE, "PACKAGE", package
            ):
                source = MODULE.source_artifacts(execution)
                (source / "candidate-ledger.csv").write_text("id\n1\n")
                (holdout / "artifacts/harvard-holdout-results.csv").write_text(
                    "candidate_id\nGSI-0001\n"
                )
                actions = MODULE.publish(
                    execution,
                    holdout_output_root=holdout,
                    apply=False,
                    replace=False,
                )
                self.assertEqual(
                    actions,
                    [
                        "CREATE candidate-ledger.csv",
                        "CREATE harvard-holdout-results.csv",
                    ],
                )

    def test_primary_failure_directory_is_outside_publication_scope(self) -> None:
        source = (TOOLS / "publish-results.py").read_text(encoding="utf-8")
        self.assertNotIn("direct-evidence", source)
        self.assertNotIn("rmtree", source)
        self.assertNotIn("openwepp-gate", source)


if __name__ == "__main__":
    unittest.main()
