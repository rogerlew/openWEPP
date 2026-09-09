"""Bounded provenance and no-F-exception tests for the reused collector adapter."""
import copy
import json
from pathlib import Path
import tempfile
import unittest

import admit_identity_v34 as adapter


class IdentityTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="v34-identity-")
        self.addCleanup(self.temp.cleanup)
        self.binary = Path(self.temp.name) / "test-binary"
        self.binary.write_bytes(b"identity-only test fixture, not executable")
        self.source = {"checkout": "a" * 40, "source_identity": "b" * 64}
        self.sidecar = Path(str(self.binary) + ".json")
        self.sidecar.write_text(json.dumps({
            "sha256": adapter.COMMON.digest(self.binary),
            "source_commit": self.source["checkout"],
        }))
        run = "/tmp/explicit-v34-fixture"
        manifest = {
            "run_dir": run, "run_file": run + "/case.run", "invoked_utc": "fixture",
            "input_checksums": {run + "/case.run": "input"},
            "output_checksums": {run + "/output": "output"},
            "stage3_evidence_archive": {"output_path": run + "/stage3"},
            "wat5_output": {"output_path": run + "/wat5"},
            "resolved_sidecars": {name: run + "/" + name for name in
                                  ("frost", "pmetpara", "snow", "snow_stage3_v11_owner_seed", "wepp_ui")},
            "binary_path": str(self.binary),
            "binary_sha256": adapter.COMMON.digest(self.binary),
            "binary_sidecar_path": str(self.sidecar),
            "binary_sidecar_sha256": adapter.COMMON.digest(self.sidecar),
            "source_commit": self.source["checkout"],
            "direct_runtime_counters": {"day_frame_constructions": 1205},
        }
        self.record = {field: 1 for field in adapter.COMMON.IDENTITY_FIELDS}
        self.record.update(output_manifest=manifest, carrier_counts={}, lse_counts={})

    def test_exact_identity_preserves_raw_frame_counter(self):
        first = adapter.identity(self.record, self.binary, self.source)
        self.assertNotIn(adapter.COMMON.FRAME_POINTER, first["manifest_rules"])
        changed = copy.deepcopy(self.record)
        changed["output_manifest"]["direct_runtime_counters"]["day_frame_constructions"] = 805
        second = adapter.identity(changed, self.binary, self.source)
        self.assertNotEqual(first["common_identity"], second["common_identity"])

    def test_stale_binary_and_foreign_checkout_fail(self):
        self.binary.write_bytes(b"changed fixture")
        with self.assertRaisesRegex(ValueError, "sidecar"):
            adapter.identity(self.record, self.binary, self.source)
        self.binary.write_bytes(b"identity-only test fixture, not executable")
        self.source["checkout"] = "c" * 40
        with self.assertRaisesRegex(ValueError, "sidecar"):
            adapter.identity(self.record, self.binary, self.source)

    def test_manifest_substitution_fails(self):
        self.record["output_manifest"]["binary_sha256"] = "d" * 64
        with self.assertRaisesRegex(ValueError, "manifest provenance"):
            adapter.identity(self.record, self.binary, self.source)


if __name__ == "__main__":
    unittest.main()
