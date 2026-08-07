#!/usr/bin/env python3
"""Non-model tests for predecessor endpoint execution custody."""

from __future__ import annotations

import importlib.util
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path

TOOL = Path(__file__).with_name("run_predecessor_bridge_matrix.py")
SPEC = importlib.util.spec_from_file_location("predecessor_bridge_runner", TOOL)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"cannot load {TOOL}")
runner = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = runner
SPEC.loader.exec_module(runner)


class RunnerCustodyTests(unittest.TestCase):
    def test_sanitized_environment_removes_authority_and_compiler_overrides(self) -> None:
        old = dict(os.environ)
        try:
            os.environ["OPENWEPP_BAD_SELECTOR"] = "bad"
            os.environ["RUSTFLAGS"] = "-C target-cpu=native"
            os.environ["CARGO_ENCODED_RUSTFLAGS"] = "bad"
            os.environ["CARGO_HOME"] = "/ambient"
            os.environ["RUSTC_WRAPPER"] = "/untrusted/wrapper"
            os.environ["CC"] = "/untrusted/cc"
            os.environ["CARGO_BUILD_TARGET"] = "untrusted-target"
            environment, removed = runner.sanitized_environment(
                cargo_home=Path("/tmp/local-cargo"),
                cargo_target=Path("/tmp/local-target"),
            )
        finally:
            os.environ.clear()
            os.environ.update(old)
        self.assertNotIn("OPENWEPP_BAD_SELECTOR", environment)
        self.assertNotIn("RUSTFLAGS", environment)
        self.assertNotIn("CARGO_ENCODED_RUSTFLAGS", environment)
        self.assertNotIn("RUSTC_WRAPPER", environment)
        self.assertNotIn("CC", environment)
        self.assertNotIn("CARGO_BUILD_TARGET", environment)
        self.assertEqual(environment["CARGO_NET_OFFLINE"], "true")
        self.assertIn("OPENWEPP_BAD_SELECTOR", removed)

    def test_rendered_runfile_contains_exactly_seven_absolute_bindings(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            fixture = root / "fixture"
            output = root / "output"
            fixture.mkdir()
            output.mkdir()
            rendered = runner.render_runfile(fixture, output)
            self.assertEqual(rendered.count(' = "/'), 7)
            for name in ("soil", "management", "slope", "climate", "pass", "loss", "wat"):
                self.assertIn(f"{name} = ", rendered)
            self.assertNotIn("../", rendered)

    def test_frozen_checkpoint_digests_match_source_tree(self) -> None:
        frozen = json.loads(runner.FREEZE_PATH.read_text(encoding="utf-8"))
        checkpoints = dict(frozen["checkpoint_grouping"]["checkpoints"])
        for source_name in ("old", "current"):
            source_sha = frozen["sources"][source_name]
            self.assertEqual(runner.build_input_digest(source_sha), checkpoints[source_sha])

    def test_cargo_seed_excludes_credentials_and_config(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "source"
            (source / "registry/cache/example").mkdir(parents=True)
            (source / "registry/cache/example/crate.bin").write_bytes(b"crate")
            (source / "credentials.toml").write_text("secret", encoding="utf-8")
            (source / "config.toml").write_text("config", encoding="utf-8")
            target = root / "target"
            old = os.environ.get("CARGO_HOME")
            try:
                os.environ["CARGO_HOME"] = str(source)
                manifest = runner.seed_cargo_home(target)
            finally:
                if old is None:
                    os.environ.pop("CARGO_HOME", None)
                else:
                    os.environ["CARGO_HOME"] = old
            self.assertEqual(manifest["file_count"], 1)
            self.assertTrue((target / "registry/cache/example/crate.bin").is_file())
            self.assertFalse((target / "credentials.toml").exists())
            self.assertFalse((target / "config.toml").exists())

    def test_endpoint_matrix_has_only_four_typed_cells(self) -> None:
        frozen = json.loads(runner.FREEZE_PATH.read_text(encoding="utf-8"))
        cells = {
            key: value
            for key, value in frozen["endpoint_matrix"].items()
            if key.startswith("E")
        }
        self.assertEqual(
            cells,
            {
                "E00": ["old", "canonical"],
                "E01": ["old", "development"],
                "E10": ["current", "canonical"],
                "E11": ["current", "development"],
            },
        )

    def test_normalized_semantic_manifest_binds_forcing_and_windows(self) -> None:
        frozen = json.loads(runner.FREEZE_PATH.read_text(encoding="utf-8"))
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            fixture = root / "fixture"
            fixture.mkdir()
            (fixture / "p8.cli").write_bytes(b"climate")
            runfile = root / "case.run"
            runfile.write_bytes(b"run")
            old_freeze = runner.FREEZE_PATH
            try:
                runner.FREEZE_PATH = root / "freeze.json"
                runner.FREEZE_PATH.write_text(json.dumps(frozen), encoding="utf-8")
                result = runner.normalized_semantic_input_manifest(
                    source_sha=frozen["sources"]["current"],
                    forcing="development",
                    fixture=fixture,
                    runfile=runfile,
                    effective={"OPENWEPP_TEST": "enabled"},
                    frozen=frozen,
                )
            finally:
                runner.FREEZE_PATH = old_freeze
        self.assertEqual(result["forcing_sha256"], frozen["forcings"]["development"]["sha256"])
        self.assertEqual(result["date_count"], 14245)
        self.assertEqual(result["scheduler"], "daily")
        self.assertEqual(result["snow_evaluation_forcing_cadence"], "hourly")
        self.assertEqual(result["science_selectors"], {"OPENWEPP_TEST": "enabled"})

    def test_path_checksum_requires_exact_resolved_binding(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "input.txt"
            path.write_bytes(b"input")
            runner.require_path_checksum(
                {str(path.resolve()): runner.sha256(path)},
                path,
                runner.sha256(path),
                "test",
            )
            with self.assertRaises(runner.CustodyError):
                runner.require_path_checksum(
                    {str(path.resolve()): "0" * 64},
                    path,
                    runner.sha256(path),
                    "test",
                )

    def test_malformed_endpoint_matrix_is_rejected(self) -> None:
        frozen = json.loads(runner.FREEZE_PATH.read_text(encoding="utf-8"))
        frozen["endpoint_matrix"]["E11"] = ["old", "development"]
        with self.assertRaises(runner.CustodyError):
            runner.endpoint_cells(frozen)

    def test_checkpoint_trigger_requires_exact_derived_lane_list(self) -> None:
        result = {
            "source_gates": {
                "canonical": {"checkpoint_trigger": False},
                "development": {"checkpoint_trigger": True},
            },
            "checkpoint_lanes_triggered": ["development"],
        }
        self.assertEqual(runner.checkpoint_trigger_lanes(result), ["development"])
        result["checkpoint_lanes_triggered"] = []
        with self.assertRaises(runner.CustodyError):
            runner.checkpoint_trigger_lanes(result)

    def test_legacy_and_explicit_selectors_normalize_to_same_operator(self) -> None:
        base = {
            "source_sha": "0" * 40,
            "runfile_semantics": {"raw_sha256": "1" * 64},
            "science_selectors": {"OPENWEPP_R7H_SNOW_TRACE_PATH": "<trace>"},
        }
        legacy = json.loads(json.dumps(base))
        explicit = json.loads(json.dumps(base))
        legacy["science_selectors"]["OPENWEPP_SNOW_STAGE3_COMPLETE_CARRIER_SHADOW"] = "enabled"
        explicit["science_selectors"]["OPENWEPP_SNOW_STAGE3_EVALUATION_OPERATOR"] = "sequential_resolved_shadow_v1"
        self.assertEqual(
            runner.normalized_operator_semantics(legacy),
            runner.normalized_operator_semantics(explicit),
        )

    def test_all_fourteen_frozen_checkpoints_have_exact_build_digests(self) -> None:
        frozen = json.loads(runner.FREEZE_PATH.read_text(encoding="utf-8"))
        checkpoints = runner.frozen_checkpoints(frozen)
        self.assertEqual(len(checkpoints), 14)
        self.assertEqual(checkpoints[0][0], frozen["sources"]["old"])
        self.assertEqual(checkpoints[-1][0], frozen["sources"]["current"])


if __name__ == "__main__":
    unittest.main()
