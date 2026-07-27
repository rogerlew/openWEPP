#!/usr/bin/env python3
"""Isolated fail-closed tests for the CAL-04B observed execution ledger."""

from __future__ import annotations

import csv
import json
import tempfile
import unittest
from pathlib import Path

import observe


class ObservedExecutionTest(unittest.TestCase):
    def test_exact_append_only_receipts_and_output_hashes(self) -> None:
        with tempfile.TemporaryDirectory() as raw:
            root = Path(raw)
            plan = root / "plan.csv"
            contract = root / "contract.csv"
            ledger = root / "ledger"
            output = root / "made"
            plan_fields = [
                "order",
                "command_id",
                "source_path",
                "argv",
                "environment",
                "working_directory",
                "inputs",
                "outputs",
                "harvard_access",
                "cost_class",
            ]
            plan_rows = [
                {
                    "order": "1",
                    "command_id": "one",
                    "source_path": "/usr/bin/true",
                    "argv": "CHECK=1 /usr/bin/true",
                    "environment": "CHECK=1",
                    "working_directory": str(Path.cwd().resolve()),
                    "inputs": "-",
                    "outputs": "-",
                    "harvard_access": "FORBIDDEN",
                    "cost_class": "QUICK",
                },
                {
                    "order": "2",
                    "command_id": "two",
                    "source_path": "/usr/bin/touch",
                    "argv": f"/usr/bin/touch {output}",
                    "environment": "none",
                    "working_directory": str(Path.cwd().resolve()),
                    "inputs": "-",
                    "outputs": str(output),
                    "harvard_access": "FORBIDDEN",
                    "cost_class": "QUICK",
                },
            ]
            with plan.open("w", newline="", encoding="utf-8") as stream:
                writer = csv.DictWriter(
                    stream, fieldnames=plan_fields, lineterminator="\n"
                )
                writer.writeheader()
                writer.writerows(plan_rows)
            with contract.open("w", newline="", encoding="utf-8") as stream:
                writer = csv.DictWriter(
                    stream,
                    fieldnames=["command_id", "prerequisites", "receipt_outputs"],
                    lineterminator="\n",
                )
                writer.writeheader()
                writer.writerows([
                    {
                        "command_id": "one",
                        "prerequisites": "-",
                        "receipt_outputs": "-",
                    },
                    {
                        "command_id": "two",
                        "prerequisites": "one",
                        "receipt_outputs": str(output),
                    },
                ])
            original = (observe.PLAN, observe.CONTRACT, observe.LEDGER)
            observe.PLAN, observe.CONTRACT, observe.LEDGER = plan, contract, ledger
            try:
                with self.assertRaises(ValueError):
                    observe.execute("one", ["/usr/bin/false"])
                self.assertEqual(observe.execute("one", ["CHECK=1", "/usr/bin/true"]), 0)
                with self.assertRaises(ValueError):
                    observe.execute("one", ["CHECK=1", "/usr/bin/true"])
                self.assertEqual(
                    observe.execute("two", ["/usr/bin/touch", str(output)]),
                    0,
                )
                receipts = observe.validate_prefix("two")
                self.assertEqual(json.loads(receipts[0]["environment_json"]), {"CHECK": "1"})
                self.assertGreaterEqual(
                    int(receipts[1]["elapsed_ns"]),
                    0,
                )
                with Path(receipts[1]["output_manifest_path"]).open(
                    newline="", encoding="utf-8"
                ) as stream:
                    output_rows = list(csv.DictReader(stream))
                self.assertEqual(output_rows[0]["state"], "OBSERVED")
                self.assertEqual(len(output_rows[0]["sha256"]), 64)
                observe.render("two", "pre-freeze")
                self.assertEqual(
                    len(observe.validate_snapshot("pre-freeze", "two")),
                    2,
                )
                output.write_text("tampered", encoding="utf-8")
                with self.assertRaisesRegex(
                    ValueError,
                    "observed output changed after receipt",
                ):
                    observe.validate_prefix("two")
            finally:
                observe.PLAN, observe.CONTRACT, observe.LEDGER = original


if __name__ == "__main__":
    unittest.main()
