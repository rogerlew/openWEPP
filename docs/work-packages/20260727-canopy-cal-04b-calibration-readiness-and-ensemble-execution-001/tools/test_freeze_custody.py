from __future__ import annotations

import csv
import hashlib
import importlib.util
import tempfile
import unittest
import sys
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("cal04b_custody", TOOLS / "custody.py")
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
sys.modules["custody"] = MODULE
HOLDOUT_SPEC = importlib.util.spec_from_file_location(
    "cal04b_holdout", TOOLS / "holdout.py"
)
assert HOLDOUT_SPEC and HOLDOUT_SPEC.loader
HOLDOUT = importlib.util.module_from_spec(HOLDOUT_SPEC)
HOLDOUT_SPEC.loader.exec_module(HOLDOUT)


class FreezeCustodyTest(unittest.TestCase):
    def write_receipt(
        self, path: Path, verifier: str, digest: str, script: Path, command: str
    ) -> None:
        row = {
            "verifier_id": verifier,
            "invocation_id": ("1" if verifier == "verifier_a" else "2") * 32,
            "freeze_digest": digest,
            "verifier_script_sha256": MODULE.sha256_file(script),
            "command": command,
            "command_sha256": hashlib.sha256(command.encode()).hexdigest(),
            "timestamp": "2026-07-27T12:00:00+00:00",
            "state": "PASS",
        }
        with path.open("w", newline="", encoding="utf-8") as stream:
            writer = csv.DictWriter(stream, fieldnames=MODULE.RECEIPT_FIELDS)
            writer.writeheader()
            writer.writerow(row)

    def test_two_direct_verifier_records_form_barrier(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            script = root / "verify.py"
            script.write_text("pass\n")
            digest = "a" * 64
            commands = {"verifier_a": "verify a", "verifier_b": "verify b"}
            paths = []
            for verifier, command in commands.items():
                path = root / f"{verifier}.csv"
                self.write_receipt(path, verifier, digest, script, command)
                paths.append(path)
            rows = MODULE.validate_receipt_barrier(
                paths, digest, script, commands
            )
            self.assertEqual([row["verifier_id"] for row in rows], list(commands))

    def test_duplicate_or_drifted_verifiers_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            script = root / "verify.py"
            script.write_text("pass\n")
            path = root / "a.csv"
            self.write_receipt(path, "verifier_a", "a" * 64, script, "verify a")
            with self.assertRaises(ValueError):
                MODULE.validate_receipt_barrier(
                    [path, path],
                    "a" * 64,
                    script,
                    {"verifier_a": "verify a", "verifier_b": "verify b"},
                )

    def test_custody_has_no_capability_or_attestation_protocol(self) -> None:
        source = (TOOLS / "custody.py").read_text(encoding="utf-8")
        self.assertNotIn("capability", source)
        self.assertNotIn("attestation", source)

    def test_holdout_requires_bubblewrap_and_read_only_root(self) -> None:
        source = (TOOLS / "holdout.py").read_text(encoding="utf-8")
        self.assertIn('shutil.which("bwrap")', source)
        self.assertIn('"--ro-bind"', source)
        self.assertIn('"--unshare-all"', source)
        self.assertIn("--holdout-output-root", source)

    def test_writable_roots_cannot_overlap_protected_paths_or_each_other(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            attempt = root / "attempt"
            execution = attempt / "objects"
            execution.mkdir(parents=True)
            custody = root / "custody"
            output = root / "output"
            custody.mkdir()
            output.mkdir()
            HOLDOUT.require_disjoint_writable_roots(execution, custody, output)
            for bad_custody, bad_output in (
                (attempt, output),
                (custody, execution),
                (custody, custody),
                (HOLDOUT.ROOT, output),
                (custody, HOLDOUT.ROOT),
            ):
                with self.assertRaises(ValueError):
                    HOLDOUT.require_disjoint_writable_roots(
                        execution, bad_custody, bad_output
                    )

    def test_opening_token_binds_resolved_command(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            token = root / "custody/open.lock"
            token.parent.mkdir()
            command = "direct holdout --exact argv"
            HOLDOUT.create_token(token, "a" * 64, command)
            fields = dict(
                line.split("=", 1)
                for line in token.read_text().splitlines()
                if "=" in line
            )
            self.assertEqual(fields["command"], command)


if __name__ == "__main__":
    unittest.main()
