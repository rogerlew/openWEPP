from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

TOOLS = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "cal04b_publish_results", TOOLS / "publish-results.py"
)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class PublishResultsTest(unittest.TestCase):
    def test_publication_build_target_stays_external(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            attempt = Path(temporary) / "attempt"
            objects = attempt / "objects"
            objects.mkdir(parents=True)
            with patch.object(MODULE.subprocess, "run") as run:
                binary = MODULE.planner_binary(objects)
            target = attempt.with_name(f"{attempt.name}.publication-target")
        self.assertEqual(run.call_args.kwargs["env"]["CARGO_TARGET_DIR"], str(target))
        self.assertEqual(binary, target / "debug/openwepp-gate-plan")


if __name__ == "__main__":
    unittest.main()
