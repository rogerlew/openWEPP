from __future__ import annotations

import importlib.util
import json
import secrets
import tempfile
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("cal04b_custody", TOOLS / "custody.py")
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class FreezeCustodyTest(unittest.TestCase):
    def test_capability_is_retained_for_rust_and_attestation_is_identity_bound(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            capability = root / "capability"
            capability.write_bytes(secrets.token_bytes(32))
            capability_hash = MODULE.capability_identity(capability)
            self.assertTrue(capability.is_file())
            receipt = root / "receipt.csv"
            receipt.write_text("state\nPASS\n", encoding="utf-8")
            script = root / "verifier.py"
            script.write_text("pass\n", encoding="utf-8")
            attestation = root / "attestation.json"
            MODULE.write_attestation(
                attestation,
                capability_hash=capability_hash,
                parent_dispatch_id="dispatch-a",
                agent_task_id="task-a",
                principal="principal-a",
                workflow="workflow-a",
                job="job-a",
                runner="runner-a",
                attempt=1,
                script=script,
                argv=["verifier.py"],
                receipt=receipt,
                freeze_digest="a" * 64,
            )
            value = json.loads(attestation.read_text(encoding="utf-8"))
            self.assertEqual(
                value["attestation_id"], MODULE.derived_id(value, "attestation_id")
            )

    def test_capability_identity_rejects_links_and_short_preimages(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            capability = root / "capability"
            capability.write_bytes(secrets.token_bytes(32))
            linked = root / "linked"
            linked.symlink_to(capability)
            with self.assertRaises(ValueError):
                MODULE.capability_identity(linked)
            capability.write_bytes(b"short")
            with self.assertRaises(ValueError):
                MODULE.capability_identity(capability)


if __name__ == "__main__":
    unittest.main()
