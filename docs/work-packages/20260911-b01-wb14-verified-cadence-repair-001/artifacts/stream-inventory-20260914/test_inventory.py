"""Focused offline controls; synthetic byte arrays are written incrementally."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from inventory import scan


class InventoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)

    def tearDown(self):
        self.temp.cleanup()

    def run_bytes(self, data, rows, **overrides):
        source = self.root / 'input.json'
        source.write_bytes(data)
        args = {'expected_sha': hashlib.sha256(data).hexdigest(),
                'expected_bytes': len(data), 'expected_rows': rows}
        args.update(overrides)
        result = scan(source, self.root / 'index.jsonl', **args)
        records = [json.loads(line) for line in (self.root / 'index.jsonl').read_text().splitlines()]
        return result, records

    def test_large_late_kind_bounded_process(self):
        source = self.root / 'large.json'
        digest = hashlib.sha256()
        size = 0
        with source.open('wb') as output:
            def write(data):
                nonlocal size
                output.write(data)
                digest.update(data)
                size += len(data)
            write(b'{"physical":[{"payload":[')
            chunk = b'123,' * 16384
            for _ in range(512):
                write(chunk)
            write(b'0],"kind":"b01_wb14_archive_test","day_index":3}],"trailing":{"ok":true}}')
        destination = self.root / 'result'
        command = ['prlimit', '--as=1073741824:1073741824', sys.executable,
                   str(Path(__file__).with_name('inventory.py')), str(source), str(destination),
                   '--sha256', digest.hexdigest(), '--bytes', str(size), '--rows', '1']
        run = subprocess.run(command, capture_output=True, text=True, check=False)
        self.assertEqual(run.returncode, 0, run.stderr + run.stdout)
        result = json.loads((destination / 'summary.json').read_text())
        row = json.loads((destination / 'records.jsonl').read_text())
        self.assertEqual(result['status'], 'COMPLETE')
        self.assertEqual(row['physical_ordinal'], 0)
        self.assertEqual(row['kinds'], ['b01_wb14_archive_test'])
        self.assertEqual(row['fields'][0]['element_count'], 8388609)
        self.assertLess(len(row['nested_fields']), 20)
        self.assertLess(result['peak_rss_kib'], 100000)
        print('large_control:', json.dumps({'source_bytes': size,
              'elements': row['fields'][0]['element_count'],
              'peak_rss_kib': result['peak_rss_kib'],
              'metadata_bytes': (destination / 'records.jsonl').stat().st_size,
              'rlimit_as_bytes': result['rlimit_as_bytes']}))

    def test_error_instead_of_success_and_ordinals(self):
        result, rows = self.run_bytes(b'{"physical":[{"kind":"ordinary"},{"kind":"b01_wb14_prepared_day_current_context_capture_error_v1","day_index":4,"detail":"failed clone"}],"tail":[1,2]}', 2)
        self.assertEqual(result['status'], 'COMPLETE')
        self.assertEqual([row['physical_ordinal'] for row in rows], [1])
        self.assertNotIn('b01_wb14_prepared_day_current_context_v1', result['kind_counts'])
        self.assertEqual(rows[0]['fields'][1]['value'], 4)

    def test_missing_duplicate_kind_and_shapes(self):
        result, rows = self.run_bytes(b'{"physical":[{"kind":"one","kind":"two"},{},17,[],{"kind":null}]}', 5)
        self.assertEqual(result['status'], 'COMPLETE')
        self.assertEqual(result['anomaly_counts'], {'duplicate_kind': 1, 'missing_kind': 3,
                         'unexpected_row_shape': 2, 'non_small_string_kind': 1})
        self.assertEqual(result['kind_counts'], {'one': 1, 'two': 1})
        self.assertNotEqual(rows[0]['fields'][0]['path'], rows[0]['fields'][1]['path'])

    def test_integer_decimal_and_ambiguous_names(self):
        result, rows = self.run_bytes(b'{"physical":[{"kind":"cadence","transaction":18446744073709551617,"a.b":{"item":0.12345678901234567890123456789},"a":{"b":false}}]}', 1)
        self.assertEqual(result['status'], 'COMPLETE')
        self.assertEqual(rows[0]['fields'][1]['value'], 18446744073709551617)
        decimal = rows[0]['nested_fields'][0]
        self.assertEqual(decimal['value']['decimal'], '0.12345678901234567890123456789')
        self.assertEqual(decimal['path'][0], {'member': 'a.b', 'position': 2})

    def test_truncation_retains_completed_entries(self):
        data = b'{"physical":[{"kind":"cadence"},{"kind":"b01_wb14_x","payload":['
        result, rows = self.run_bytes(data, 2)
        self.assertEqual(result['status'], 'INCOMPLETE')
        self.assertFalse(result['syntax_complete'])
        self.assertEqual(result['completed_rows'], 1)
        self.assertEqual(result['interrupted_physical_ordinal'], 1)
        self.assertEqual(len(rows), 1)
        self.assertTrue(rows[0]['provisional'])

    def test_malformed_trailing_document(self):
        result, rows = self.run_bytes(b'{"physical":[{"kind":"cadence"}],"tail":[1,]}', 1)
        self.assertEqual(result['status'], 'INCOMPLETE')
        self.assertFalse(result['syntax_complete'])
        self.assertEqual(len(rows), 1)

    def test_hash_mismatch_is_incomplete(self):
        result, _ = self.run_bytes(b'{"physical":[]}', 0, expected_sha='0' * 64)
        self.assertEqual(result['status'], 'INCOMPLETE')
        self.assertTrue(result['syntax_complete'])
        self.assertIn('hash/byte mismatch', result['error']['detail'])

    def test_count_mismatch_is_incomplete(self):
        result, _ = self.run_bytes(b'{"physical":[]}', 1)
        self.assertEqual(result['status'], 'INCOMPLETE')
        self.assertIn('row count mismatch', result['error']['detail'])

    def test_caps_and_string_preview(self):
        data = ('{"physical":[{"kind":"capture","nested":{' +
                ','.join('"k%d":%d' % (i, i) for i in range(1100)) +
                '},"detail":"' + 'x' * 400 + '","day_index":4}]}').encode()
        result, rows = self.run_bytes(data, 1)
        self.assertEqual(result['status'], 'COMPLETE')
        self.assertEqual(len(rows[0]['nested_fields']), 1024)
        self.assertEqual(rows[0]['nested_fields_omitted'], 76)
        self.assertTrue(rows[0]['fields'][2]['value']['truncated'])
        self.assertEqual(rows[0]['fields'][3]['value'], 4)

    def test_duplicate_physical_member_rejected(self):
        result, _ = self.run_bytes(b'{"physical":[],"physical":[]}', 0)
        self.assertEqual(result['status'], 'INCOMPLETE')
        self.assertIn('duplicate root physical', result['error']['detail'])


if __name__ == '__main__':
    unittest.main(verbosity=2)
