from __future__ import annotations

import importlib.util
import io
import subprocess
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock


ROOT = Path(__file__).parents[2]
MODULE_PATH = ROOT / "tools/local_ci/testgate_shadow.py"
SPEC = importlib.util.spec_from_file_location("testgate_shadow", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
SHADOW = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SHADOW)


class TestGateShadowTest(unittest.TestCase):
    def test_invoke_retains_valid_nonpass_result(self) -> None:
        completed = subprocess.CompletedProcess(
            args=["gate"], returncode=1, stdout='{"result":"FAIL"}', stderr=""
        )
        with mock.patch.object(SHADOW.subprocess, "run", return_value=completed):
            value = SHADOW._invoke(["gate"], ROOT, allow_nonpass=True)
        self.assertEqual(value["result"], "FAIL")

        with mock.patch.object(SHADOW.subprocess, "run", return_value=completed):
            with self.assertRaises(SHADOW.ShadowError):
                SHADOW._invoke(["gate"], ROOT)

    def test_main_reports_verified_nonpass_as_failure_visible(self) -> None:
        observation = {
            "execution_requested": True,
            "execution_result": {"result": "BLOCKED"},
            "execution_error": None,
        }
        with (
            mock.patch.object(SHADOW, "_parse_args", return_value=object()),
            mock.patch.object(SHADOW, "observe", return_value=observation),
            redirect_stdout(io.StringIO()),
        ):
            self.assertEqual(SHADOW.main(), 1)

        observation["execution_result"] = {"result": "PASS"}
        with (
            mock.patch.object(SHADOW, "_parse_args", return_value=object()),
            mock.patch.object(SHADOW, "observe", return_value=observation),
            redirect_stdout(io.StringIO()),
        ):
            self.assertEqual(SHADOW.main(), 0)


if __name__ == "__main__":
    unittest.main()
