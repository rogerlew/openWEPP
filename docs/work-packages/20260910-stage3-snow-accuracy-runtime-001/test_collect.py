"""Negative evidence-receipt tests; no simulation or timing claims."""
import json
from pathlib import Path
import tempfile
import unittest

import collect


class FailedAttemptReceipts(unittest.TestCase):
    def attempt(self, script):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            binary = root / "fake-runner"
            binary.write_text(script)
            binary.chmod(0o755)
            output = root / "evidence"
            receipt = collect.collect(binary, output, "R0", "continuity", timeout=2)
            self.assertEqual(receipt, json.loads((output / "receipt.json").read_text()))
            self.assertFalse(receipt["execution_valid"])
            self.assertIn("stdout.log", receipt["artifact_sha256"])
            return receipt

    def test_malformed_runner_json_finalizes_receipt(self):
        receipt = self.attempt("#!/bin/sh\nprintf '{invalid' > run.json\n")
        self.assertIn("JSONDecodeError", receipt["infrastructure_error"])

    def test_missing_artifacts_never_pass(self):
        receipt = self.attempt("#!/bin/sh\nexit 0\n")
        self.assertEqual(receipt["exit_code"], 0)
        self.assertFalse(receipt["runner_receipt_present"])

    def test_timeout_finalizes_receipt(self):
        receipt = self.attempt("#!/bin/sh\nsleep 10\n")
        self.assertTrue(receipt["timeout"])
        self.assertLess(receipt["exit_code"], 0)


if __name__ == "__main__":
    unittest.main()
