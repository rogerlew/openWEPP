#!/usr/bin/env python3
"""Focused provenance/resume tests for the Topanga census harness."""

from __future__ import annotations

import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path

import numpy as np


MODULE_PATH = Path(__file__).with_name("topanga_openwepp_census.py")
SPEC = importlib.util.spec_from_file_location("topanga_openwepp_census", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
CENSUS = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = CENSUS
SPEC.loader.exec_module(CENSUS)


class RecordProvenanceTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        for suffix in ["sol", "man", "slp"]:
            (self.root / f"p1.{suffix}").write_text(suffix, encoding="utf-8")
        (self.root / "p1.cli").write_text(
            "header\n  1  1 2001  0.0\n  2  1 2001  1.0\n", encoding="utf-8"
        )
        self.record_path = self.root / "record.npz"
        self.case = CENSUS.Case(
            case_id="case-1",
            scenario="base",
            hillslope_id=1,
            source_dir=self.root,
            record_path=self.record_path,
            trial=None,
        )
        self.provenance = CENSUS.Provenance(
            plan_sha256="plan-a", binary_sha256="binary-a"
        )

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def write_record(self) -> None:
        year, julian = CENSUS.expected_calendar(self.case)
        np.savez_compressed(
            self.record_path,
            record_schema=np.asarray(CENSUS.RECORD_SCHEMA),
            case_id=np.asarray(self.case.case_id),
            plan_sha256=np.asarray(self.provenance.plan_sha256),
            binary_sha256=np.asarray(self.provenance.binary_sha256),
            input_hashes_json=np.asarray(CENSUS.case_input_hashes(self.case)),
            expected_row_count=np.asarray(len(year)),
            calendar_sha256=np.asarray(CENSUS.calendar_sha256(year, julian)),
            year=year,
            julian=julian,
            runvol_m3=np.asarray([1.0, 0.0]),
            peakro_m3_s=np.asarray([1.0 / 3_600.0, 0.0]),
        )

    def test_valid_record_reuses(self) -> None:
        self.write_record()
        self.assertTrue(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )

    def test_corrupt_record_and_provenance_changes_invalidate(self) -> None:
        self.record_path.write_bytes(b"not-npz")
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )
        self.write_record()
        changed_binary = CENSUS.Provenance(
            plan_sha256="plan-a", binary_sha256="binary-b"
        )
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, changed_binary)
        )
        (self.root / "p1.sol").write_text("changed", encoding="utf-8")
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )
        self.write_record()
        (self.root / "snow.txt").write_text("active sidecar", encoding="utf-8")
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )

    def test_nonfinite_or_negative_record_values_invalidate(self) -> None:
        self.write_record()
        with np.load(self.record_path) as source:
            record = {name: source[name].copy() for name in source.files}
        record["peakro_m3_s"] = np.asarray([np.nan])
        np.savez_compressed(self.record_path, **record)
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )
        record["peakro_m3_s"] = np.asarray([1.0 / 3_600.0])
        record["runvol_m3"] = np.asarray([-1.0, 0.0])
        np.savez_compressed(self.record_path, **record)
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )

    def test_empty_truncated_and_wrong_calendar_records_invalidate(self) -> None:
        self.write_record()
        with np.load(self.record_path) as source:
            valid = {name: source[name].copy() for name in source.files}
        for name in ["year", "julian", "runvol_m3", "peakro_m3_s"]:
            valid[name] = valid[name][:0]
        np.savez_compressed(self.record_path, **valid)
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )

        self.write_record()
        with np.load(self.record_path) as source:
            truncated = {name: source[name].copy() for name in source.files}
        for name in ["year", "julian", "runvol_m3", "peakro_m3_s"]:
            truncated[name] = truncated[name][:-1]
        np.savez_compressed(self.record_path, **truncated)
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )

        self.write_record()
        with np.load(self.record_path) as source:
            wrong_calendar = {name: source[name].copy() for name in source.files}
        wrong_calendar["julian"][1] = 3
        np.savez_compressed(self.record_path, **wrong_calendar)
        self.assertFalse(
            CENSUS.record_matches(self.record_path, self.case, self.provenance)
        )

    def test_empty_paired_event_evidence_rejected(self) -> None:
        with self.assertRaisesRegex(RuntimeError, "no paired runoff events"):
            CENSUS.validate_event_rows([])

    def test_paired_rows_normalize_structured_mutation_values_for_parquet(self) -> None:
        self.write_record()
        mutation_path = self.root / "mutation.npz"
        with np.load(self.record_path) as source:
            record = {name: source[name].copy() for name in source.files}
        np.savez_compressed(mutation_path, **record)
        trial = {
            "trial_id": "cover-pair",
            "scenario": "base",
            "hillslope_id": 1,
            "family": "cover",
            "direction": "plus",
            "source_value": {"inrcov": 0.7, "rilcov": 0.8},
            "expected_value": {"inrcov": 0.71, "rilcov": 0.81},
        }
        rows = CENSUS.paired_event_rows(
            trial, self.record_path, mutation_path
        )
        self.assertEqual(rows[0]["source_value_json"], '{"inrcov": 0.7, "rilcov": 0.8}')
        self.assertEqual(
            rows[0]["expected_value_json"],
            '{"inrcov": 0.71, "rilcov": 0.81}',
        )
        CENSUS.pa.Table.from_pylist(rows)


if __name__ == "__main__":
    unittest.main()
