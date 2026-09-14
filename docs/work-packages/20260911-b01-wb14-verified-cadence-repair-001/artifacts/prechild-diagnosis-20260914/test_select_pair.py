import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from select_pair import select,decode_selected,typed_bytes

class SelectTests(unittest.TestCase):
    def run_case(self,data,targets,rows,fail=False,digest=None):
        with tempfile.TemporaryDirectory() as d:
            p=Path(d);src=p/'raw';src.write_bytes(data);out=p/'out'
            args=(src,out,targets,digest or hashlib.sha256(data).hexdigest(),len(data),rows)
            if fail:
                with self.assertRaises(Exception):select(*args)
                self.assertEqual(json.loads((out/'summary.json').read_text())['status'],'INCOMPLETE')
                with self.assertRaises(ValueError):decode_selected(out)
            else:
                select(*args)
                return decode_selected(out)
    def test_target_exact_numbers_and_member_order(self):
        r=self.run_case(b'[{"skip":0},{"n":340282366920938463463374607431768211455,"d":1.234567890123456789,"kind":"wanted","typed":{"Ok":[123,125]}}]',{1:'wanted'},2)[1]
        self.assertEqual(r['n'],2**128-1)
        self.assertEqual(str(r['d']),'1.234567890123456789')
        self.assertEqual(list(r),['n','d','kind','typed'])
        self.assertEqual(typed_bytes(r['typed']),b'{}')
    def test_large_unselected_late_kind(self):
        # Incremental fixture creation avoids an unrelated large test allocation.
        with tempfile.TemporaryDirectory() as d:
            p=Path(d);src=p/'raw';h=hashlib.sha256();n=0
            with src.open('wb') as f:
                for block in [b'[{"bytes":[']:
                    f.write(block);h.update(block);n+=len(block)
                block=b'255,'*16384
                for _ in range(512):f.write(block);h.update(block);n+=len(block)
                block=b'0],"kind":"late"},{"kind":"wanted"}]';f.write(block);h.update(block);n+=len(block)
            summary=select(src,p/'out',{1:'wanted'},h.hexdigest(),n,2)
            self.assertLess(summary['selected_event_bytes'],200)
            self.assertEqual(decode_selected(p/'out')[1],{'kind':'wanted'})
    def test_malformed_truncated_trailing(self):
        for data in (b'[{"kind":"wanted"}',b'[{"kind":}]',b'[{"kind":"wanted"}]x'):
            with self.subTest(data=data):self.run_case(data,{0:'wanted'},1,True)
    def test_missing_duplicate_wrong_kind(self):
        for data in (b'[{}]',b'[{"kind":"wanted","kind":"wanted"}]',b'[{"kind":"other"}]'):
            with self.subTest(data=data):self.run_case(data,{0:'wanted'},1,True)
    def test_identity_count_missing_target(self):
        self.run_case(b'[{"kind":"wanted"}]',{0:'wanted'},2,True)
        self.run_case(b'[{"kind":"wanted"}]',{0:'wanted'},1,True,'0'*64)
        self.run_case(b'[{"kind":"wanted"}]',{1:'wanted'},1,True)
    def test_byte_validation(self):
        for value in (True,1.0,-1,256,'1'):
            with self.subTest(value=value):
                with self.assertRaises(ValueError):typed_bytes({'Ok':[value]})
        self.assertEqual(typed_bytes({'Ok':[0,255]}),b'\0\xff')

if __name__=='__main__':unittest.main()
