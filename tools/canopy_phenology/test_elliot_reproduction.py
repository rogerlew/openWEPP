import tempfile
import unittest
from unittest import mock
from pathlib import Path

import elliot_reproduction as harness


class ElliotReproductionTests(unittest.TestCase):
    def test_prepare_092_changes_only_dropfc_and_output_switch(self):
        arm = next(item for item in harness.ARMS if item.arm_id == "hubbard_hardwood_092")
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            arm_root = harness.prepare_arm(
                arm, root, "C:/Users/roger/AppData/Local/Temp/openwepp-cal02-test"
            )
            management = (arm_root / "run/p1.man").read_text(encoding="utf-8")
            self.assertIn("0.85000 0.92000 0.65000 0.99000", management)
            self.assertNotIn("0.85000 0.95000 0.65000 0.99000", management)
            source_lines = (
                harness.SOURCES / arm.source_dir / "inputs/p1.run"
            ).read_text(encoding="utf-8").splitlines()
            derived_lines = (arm_root / "run/p1.run").read_text(encoding="ascii").splitlines()
            self.assertEqual(derived_lines[:11], source_lines[:11])
            self.assertEqual(derived_lines[11:13], ["Yes", "../output/p1.crop.dat"])
            self.assertEqual(derived_lines[13:], source_lines[12:])

    def test_prepare_uses_mukey_bound_2006_soil(self):
        arm = harness.ARMS[0]
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            soils = root / "soils"
            soils.mkdir()
            (soils / f"{arm.mukey}.sol").write_text(
                "2006.2\n# derived test soil\nAny comments:\n1 0\n"
                "'soil' 'FSL' 1 0.1 0.75 1 0.1 1 1\n"
                "200 50 10 1 5 1\n1 10000 0.01\n",
                encoding="ascii",
            )
            arm_root = harness.prepare_arm(
                arm,
                root / "runs",
                "C:/Users/roger/AppData/Local/Temp/openwepp-cal02-test",
                soils,
            )
            self.assertEqual(
                (arm_root / "run/p1.sol").read_text(encoding="ascii").splitlines()[0],
                "2006.2",
            )
            manifest = __import__("json").loads(
                (arm_root / "input-manifest.json").read_text(encoding="utf-8")
            )
            self.assertEqual(manifest["soil_derivation"]["mukey"], 665220)

    def test_remote_confinement_rejects_escape(self):
        self.assertIsNone(harness.SAFE_REMOTE.fullmatch("C:/Temp/cal02"))
        self.assertEqual(harness.ADMITTED_HOST, "BLARHG")

    def test_fixture_symlink_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source = root / "source"
            target = root / "target"
            source.mkdir()
            target.mkdir()
            (source / "regular").write_text("input", encoding="ascii")
            (source / "link").symlink_to("/etc/hosts")
            with self.assertRaisesRegex(ValueError, "not a regular file"):
                harness.copy_regular_inputs(source, target)

    def test_crop_parser_reconstructs_disjoint_pools(self):
        arm = harness.ARMS[0]
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "crop.dat"
            path.write_text(
                " 1 270 100 5.00 .900 6.00 .999 .999 1 19.000 0.1000"
                " 1 1.0000 1 2.0000 1 3.0000 0.0 0.0 0.0"
                " 1 0.1 1 0.2 1 0.3 12.0\n",
                encoding="ascii",
            )
            row = harness.parse_crop(path, arm)[0]
            self.assertAlmostEqual(row["total_flat_residue_kg_m2"], 6.0)
            self.assertAlmostEqual(row["dead_root_kg_m2"], 0.6)
            self.assertFalse(row["canopy_height_overflow"])
            self.assertFalse(row["lai_overflow"])

    def test_crop_parser_preserves_fixed_width_overflow_as_null(self):
        arm = harness.ARMS[0]
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "crop.dat"
            path.write_text(
                " 1 270 100 **** .900 **** .999 .999 1 19.000 0.1000"
                " 1 1.0000 1 2.0000 1 3.0000 0.0 0.0 0.0"
                " 1 0.1 1 0.2 1 0.3 12.0\n",
                encoding="ascii",
            )
            row = harness.parse_crop(path, arm)[0]
            self.assertIsNone(row["canopy_height_m"])
            self.assertTrue(row["canopy_height_overflow"])
            self.assertIsNone(row["lai_m2_m2"])
            self.assertTrue(row["lai_overflow"])

    def test_event_parser_uses_fourteen_field_sediment_column(self):
        arm = harness.ARMS[0]
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "events.dat"
            path.write_text(
                "26 3 5 0.3 52.6 0.000 0.00 0.00 0.0 0.00 0.00 0.0 4.2 1.00\n",
                encoding="ascii",
            )
            row = harness.parse_events(path, arm)[0]
            self.assertEqual(row["sediment_delivery_kg_m"], 4.2)

    def test_element_parser_uses_peak_runoff_column(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "element.dat"
            path.write_text(
                "1 13 9 1 43.3 .243 102.699 .131 1.858 1.795 33.028"
                " 210.353 6 51 99 99 99 2.362 1.170 .180 .213 2.568 .150 .102\n",
                encoding="ascii",
            )
            self.assertEqual(harness.parse_peak_runoff(path), [0.131])

    def test_perennial_litter_transfer_uses_peak_live_and_dropfc(self):
        arm = next(item for item in harness.ARMS if item.arm_id == "hubbard_hardwood_095")
        rows = [
            {"live_biomass_kg_m2": 18.5},
            {"live_biomass_kg_m2": 20.0},
            {"live_biomass_kg_m2": 19.0},
        ]
        self.assertAlmostEqual(harness.reconstruct_annual_litter_transfer(rows, arm), 1.0)
        self.assertEqual(
            harness.reconstruct_annual_litter_transfer(rows, harness.ARMS[0]),
            0.0,
        )

    @mock.patch("elliot_reproduction.subprocess.run")
    def test_remote_executable_hash_is_parsed_and_enforced(self, run):
        run.return_value.stdout = (
            "SHA256 hash of file:\n"
            + harness.EXECUTABLE_SHA256
            + "\nCertUtil: -hashfile command completed successfully.\n"
        )
        self.assertEqual(
            harness.remote_executable_sha256("BLARHG"),
            harness.EXECUTABLE_SHA256,
        )
        with self.assertRaisesRegex(ValueError, "only the admitted host"):
            harness.remote_executable_sha256("example.invalid")


if __name__ == "__main__":
    unittest.main()
