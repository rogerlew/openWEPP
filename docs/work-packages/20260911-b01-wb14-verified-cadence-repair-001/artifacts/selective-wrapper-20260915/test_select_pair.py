import decimal
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
import ijson
from select_pair import select, decode_selected, typed_bytes
from decode_pair import decode_payload, export


def wrap(rows):
    return b'{"before":{"physical":[{"kind":"decoy"}],"arrays":[[1,2],[]]},"physical":'+rows+b',"after":[{"physical":[3]},false,null]}'


class SelectTests(unittest.TestCase):
    def run_case(self, data, targets, rows, refusal=None, digest=None, size=None):
        with tempfile.TemporaryDirectory() as d:
            p=Path(d); src=p/'raw'; src.write_bytes(data); out=p/'out'
            args=(src,out,targets,digest or hashlib.sha256(data).hexdigest(),len(data) if size is None else size,rows)
            if refusal:
                exception, message = refusal
                with self.assertRaisesRegex(exception,message):
                    select(*args)
                self.assertEqual(json.loads((out/'summary.json').read_text())['status'],'INCOMPLETE')
                with self.assertRaisesRegex(ValueError,'incomplete extraction'):
                    decode_selected(out)
            else:
                summary=select(*args)
                self.assertTrue(summary['eof']); self.assertTrue(summary['syntax_complete'])
                self.assertEqual(summary['physical_arrays'],1)
                return decode_selected(out)

    def test_target_exact_numbers_and_member_order(self):
        r=self.run_case(wrap(b'[{"skip":[1,{"kind":"wanted"}]},{"n":340282366920938463463374607431768211455,"d":1.234567890123456789,"kind":"wanted","typed":{"Ok":[123,125]},"empty":{},"array":[null,true,{}]}]'),{1:'wanted'},2)[1]
        self.assertEqual(r['n'],2**128-1)
        self.assertEqual(r['d'],decimal.Decimal('1.234567890123456789'))
        self.assertEqual(list(r),['n','d','kind','typed','empty','array'])
        self.assertEqual(r['array'],[None,True,{}])
        self.assertEqual(decode_payload(typed_bytes(r['typed'])),{})

    def test_large_unselected_late_kind(self):
        with tempfile.TemporaryDirectory() as d:
            p=Path(d);src=p/'raw';h=hashlib.sha256();n=0
            with src.open('wb') as f:
                block=b'{"before":[{"physical":[]}],"physical":[{"bytes":['
                f.write(block);h.update(block);n+=len(block)
                block=b'255,'*16384
                for _ in range(512):f.write(block);h.update(block);n+=len(block)
                block=b'0],"kind":"late"},{"kind":"wanted","typed":{"Ok":[123,125]}}],"after":[[]]}'
                f.write(block);h.update(block);n+=len(block)
            summary=select(src,p/'out',{1:'wanted'},h.hexdigest(),n,2)
            self.assertLess(summary['selected_event_bytes'],1000)
            row=decode_selected(p/'out')[1]
            self.assertEqual(decode_payload(typed_bytes(row['typed'])),{})

    def test_malformed_truncated_trailing(self):
        for data in (b'{"physical":[{"kind":"wanted"}]}x',
                     b'{"physical":[{"kind":}]}',
                     b'{"physical":[{"kind":"wanted"}]',
                     b'{"physical":[{"kind":"wanted"}],"after":[1,}',
                     b'{"physical":[{"kind":"wanted"}]} {}'):
            with self.subTest(data=data):
                self.run_case(data,{0:'wanted'},1,(ijson.JSONError,'.+'))

    def test_wrapper_refusals(self):
        for data, message in (
            (b'[]','expected one top-level object'),
            (b'{"nested":{"physical":[]}}','missing top-level physical array'),
            (b'{"physical":[],"physical":[]}','duplicate top-level physical member'),
            (b'{"physical":null}','top-level physical must be array'),
            (b'{"physical":{}}','top-level physical must be array'),
            (b'{"physical":[[]]}','physical row must be object')):
            with self.subTest(data=data):self.run_case(data,{},0,(ValueError,message))

    def test_missing_duplicate_wrong_kind(self):
        for data in (b'[{}]',b'[{"kind":"wanted","kind":"wanted"}]',
                     b'[{"kind":"other"}]',b'[{"kind":1}]',b'[{"kind":[]}]'):
            with self.subTest(data=data):
                self.run_case(wrap(data),{0:'wanted'},1,(ValueError,'target kind missing, duplicate or mismatch'))

    def test_identity_count_missing_target(self):
        data=wrap(b'[{"kind":"wanted"}]')
        refusal=(ValueError,'source hash/byte/count mismatch')
        self.run_case(data,{0:'wanted'},2,refusal)
        self.run_case(data,{0:'wanted'},1,refusal,'0'*64)
        self.run_case(data,{0:'wanted'},1,refusal,size=len(data)+1)
        self.run_case(data,{1:'wanted'},1,(ValueError,'target missing'))

    def test_duplicate_selected_member(self):
        for row in (b'{"kind":"wanted","x":1,"x":2}',
                    b'{"kind":"wanted","x":{"y":1,"y":2}}'):
            with self.subTest(row=row):
                with self.assertRaisesRegex(ValueError,'duplicate selected member'):
                    self.run_case(wrap(b'['+row+b']'),{0:'wanted'},1)

    def test_byte_validation_through_wrapper(self):
        for value in (True,1.0,-1,256,'1'):
            with self.subTest(value=value):
                row={'kind':'wanted','typed':{'Ok':[value]}}
                r=self.run_case(wrap(json.dumps([row]).encode()),{0:'wanted'},1)[0]
                with self.assertRaisesRegex(ValueError,'byte must be integer in 0..255'):
                    typed_bytes(r['typed'])
        for value in (None,{'Err':'error'},{'Ok':1},{'Ok':[],'extra':1}):
            row={'kind':'wanted','typed':value}
            r=self.run_case(wrap(json.dumps([row]).encode()),{0:'wanted'},1)[0]
            with self.assertRaisesRegex(ValueError,'required typed payload is not Ok array'):
                typed_bytes(r['typed'])
        self.assertEqual(typed_bytes({'Ok':[0,255]}),b'\0\xff')

    def test_decoded_duplicate_and_exact_precision(self):
        for raw, message in ((b'{"a":1,"a":2}','duplicate decoded member'),
                             (b'{"x":NaN}','non-JSON numeric constant')):
            row={'kind':'wanted','typed':{'Ok':list(raw)}}
            r=self.run_case(wrap(json.dumps([row]).encode()),{0:'wanted'},1)[0]
            with self.assertRaisesRegex(ValueError,message):decode_payload(typed_bytes(r['typed']))
        raw=b'{"n":340282366920938463463374607431768211455,"d":1.234567890123456789}'
        row={'kind':'wanted','typed':{'Ok':list(raw)}}
        r=self.run_case(wrap(json.dumps([row]).encode()),{0:'wanted'},1)[0]
        decoded=decode_payload(typed_bytes(r['typed']))
        self.assertEqual(decoded,{'n':2**128-1,'d':decimal.Decimal('1.234567890123456789')})

    def test_export_complete_pair_and_missing_member(self):
        # Tiny rows keep the real bound ordinals while testing the production export path.
        for missing in (False,True):
            with self.subTest(missing=missing), tempfile.TemporaryDirectory() as d:
                p=Path(d); raw=b'{"d":1.234567890123456789}'
                guard={'kind':'surface_liquid_wb14_cadence_failure','beginning_typed_bytes':{'Ok':list(raw)},'input_typed_bytes':{'Ok':list(raw)}}
                caller=dict(guard,kind='surface_liquid_wb14_cadence_caller_failure',parent_working_typed_bytes={'Ok':list(raw)},working_typed_bytes={'Ok':list(raw)})
                if missing:del caller['working_typed_bytes']
                data=wrap(b'['+b'{},'*123091+json.dumps(guard).encode()+b','+json.dumps(caller).encode()+b']')
                src=p/'raw';src.write_bytes(data);out=p/'out'
                select(src,out,{123091:guard['kind'],123092:caller['kind']},hashlib.sha256(data).hexdigest(),len(data),123093)
                if missing:
                    with self.assertRaisesRegex(ValueError,'missing required selected member: working_typed_bytes'):export(out)
                    self.assertEqual(json.loads((out/'decoded/summary.json').read_text())['status'],'INCOMPLETE')
                else:
                    result=export(out);self.assertEqual(result['status'],'COMPLETE');self.assertEqual(len(result['payloads']),6)
                    for payload in result['payloads']:
                        self.assertEqual((out/'decoded'/payload['raw_file']).read_bytes(),raw)
                        self.assertEqual(payload['sha256'],hashlib.sha256(raw).hexdigest())


if __name__=='__main__':unittest.main()
