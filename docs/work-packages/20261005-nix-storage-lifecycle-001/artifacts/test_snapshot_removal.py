"""Negative controls for one-time archival deletion (never invokes deletion)."""
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('removal', HERE / 'remove_verified_snapshots.py')
removal = importlib.util.module_from_spec(spec)
spec.loader.exec_module(removal)


class RefusalControls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='openwepp-nix-refusal-', dir='/tmp')
        self.root = Path(self.temp.name)
        row = json.loads((HERE / 'legacy-snapshot-inventory.json').read_text())[0]
        self.inventory = [row]
        self.receipts = [json.loads((removal.DEST / (Path(row['path']).name + '.json')).read_text())]
        self.pins = json.loads((HERE / 'legacy-toolchain-pins.json').read_text())

    def tearDown(self):
        self.temp.cleanup()

    def run_refusal(self, message):
        (self.root / 'legacy-snapshot-inventory.json').write_text(json.dumps(self.inventory))
        (self.root / 'legacy-snapshot-recovery.json').write_text(json.dumps(self.receipts))
        (self.root / 'legacy-toolchain-pins.json').write_text(json.dumps(self.pins))
        real_run = removal.subprocess.run

        def read_only_run(argv, *args, **kwargs):
            if '--delete' in argv:
                raise AssertionError('Deletion must not run')
            return real_run(argv, *args, **kwargs)

        with patch.object(removal, 'HERE' , self.root), patch.object(sys, 'argv', ['removal']), patch.object(removal.subprocess, 'run', side_effect=read_only_run):
            with self.assertRaisesRegex(RuntimeError, message):
                removal.main()

    def test_duplicate_inventory_refused(self):
        self.inventory *= 2
        self.run_refusal('unique')

    def test_missing_receipt_refused(self):
        self.receipts = []
        self.run_refusal('receipt count')

    def test_unverified_receipt_refused(self):
        self.receipts[0]['restored_and_verified'] = False
        self.run_refusal('recovery receipt mismatch')

    def test_wrong_archive_path_refused(self):
        self.receipts[0]['archive'] = '/tmp/not-the-archive'
        self.run_refusal('recovery receipt mismatch')

    def test_wrong_archive_size_refused(self):
        self.receipts[0]['archive_bytes'] += 1
        self.run_refusal('archive size mismatch')

    def test_wrong_archive_checksum_refused(self):
        self.receipts[0]['archive_sha256'] = '0' * 64
        self.run_refusal('archive corruption')

    def test_missing_root_refused(self):
        self.pins['pins'][0]['root'] += '-not-registered'
        self.run_refusal('GC root missing')

    def test_source_outside_store_refused(self):
        self.inventory[0]['path'] = '/tmp/wrong-source'
        self.receipts[0]['source'] = '/tmp/wrong-source'
        self.run_refusal('invalid source path')


if __name__ == '__main__':
    unittest.main()
