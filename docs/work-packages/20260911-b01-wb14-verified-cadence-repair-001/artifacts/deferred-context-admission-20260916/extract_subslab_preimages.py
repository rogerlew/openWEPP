"""Recover exact subslab records at previously indexed raw receipt occurrences."""
from pathlib import Path
import hashlib
import json
import re

HERE = Path(__file__).resolve().parent
index = json.loads((HERE / 'retained-soil-trial-primitives.json').read_text())
source = Path(index['source'])
assert source.stat().st_size == index['source_bytes']
decoder = json.JSONDecoder()
records = {}
with source.open('rb') as stream:
    for occurrence in index['occurrences_outside_extracted_primitives']:
        offset = occurrence['offset']
        start = max(0, offset - 262144)
        stream.seek(start)
        window = stream.read(524288)
        for match in re.finditer(rb'\{"accepted_slab_sha256"\s*:', window):
            if start + match.start() > offset:
                break
            try:
                text = window[match.start():].decode()
                record, end = decoder.raw_decode(text)
            except (ValueError, UnicodeDecodeError):
                continue
            if record.get('kind') != 'candidate_subslab_primitives':
                continue
            raw = text[:end].encode()
            stop = start + match.start() + len(raw)
            if start + match.start() <= offset < stop:
                records[start + match.start()] = dict(
                    source_start=start + match.start(), source_end=stop,
                    source_record_sha256=hashlib.sha256(raw).hexdigest(),
                    record=record, source_canonical_json=raw.decode())
result = dict(
    evidence_class='Ran: exact raw source slices, no physical computation or admission',
    source=str(source), source_sha256_previously_verified=index['source_sha256'],
    records=list(records.values()),
    limitation='Whole source SHA inherited from preceding complete scan; this operation verifies exact indexed slices only. Candidate records require independent accepted-clock membership.')
assert len(records) == 8
(HERE / 'retained-subslab-primitives.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'records': len(records), 'source_offsets': list(records)}))
