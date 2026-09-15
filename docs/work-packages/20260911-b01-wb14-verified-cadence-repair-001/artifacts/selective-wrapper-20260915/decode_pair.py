"""Bounded decoding of the COMPLETE selected pair; exact raw payloads retained."""
import decimal
import hashlib
import json
from pathlib import Path
import sys
from select_pair import decode_selected, typed_bytes


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError('duplicate decoded member')
        result[key] = value
    return result


def reject_constant(value):
    raise ValueError('non-JSON numeric constant: '+value)


def decode_payload(raw):
    return json.loads(raw, parse_float=decimal.Decimal, object_pairs_hook=unique_object,
                      parse_constant=reject_constant)


def exact_json(value):
    if isinstance(value, decimal.Decimal):
        if not value.is_finite():
            raise ValueError('nonfinite decimal')
        return str(value)
    if isinstance(value, dict):
        return '{'+','.join(json.dumps(k)+':'+exact_json(v) for k,v in value.items())+'}'
    if isinstance(value, list):
        return '['+','.join(exact_json(v) for v in value)+']'
    return json.dumps(value, ensure_ascii=True, allow_nan=False)


def export(destination):
    rows = decode_selected(destination)
    required = {
        123091: ('beginning_typed_bytes', 'input_typed_bytes'),
        123092: ('beginning_typed_bytes', 'input_typed_bytes',
                 'parent_working_typed_bytes', 'working_typed_bytes'),
    }
    if set(rows) != set(required):
        raise ValueError('decoded ordinal mismatch')
    out = destination/'decoded'
    out.mkdir(exist_ok=False)
    receipt = {'status':'INCOMPLETE', 'payloads':[],
               'selected_event_sha256':hashlib.sha256((destination/'events.provisional.jsonl').read_bytes()).hexdigest(),
               'row_encoding':'re-encoded complete values, not original row lexical bytes'}
    try:
        for ordinal, names in required.items():
            row = rows[ordinal]
            (out/f'{ordinal}.row.json').write_text(exact_json(row)+'\n')
            for name in names:
                if name not in row:
                    raise ValueError('missing required selected member: '+name)
                raw = typed_bytes(row[name])
                stem = f'{ordinal}.{name}'
                (out/f'{stem}.raw.json').write_bytes(raw)
                entry = {'ordinal':ordinal,'member':name,'bytes':len(raw),
                         'sha256':hashlib.sha256(raw).hexdigest(), 'raw_file':f'{stem}.raw.json'}
                receipt['payloads'].append(entry)
                value = decode_payload(raw)
                (out/f'{stem}.decoded.json').write_text(exact_json(value)+'\n')
        receipt['status'] = 'COMPLETE'
    except Exception as error:
        receipt['error'] = type(error).__name__+': '+str(error)
        raise
    finally:
        (out/'summary.json').write_text(json.dumps(receipt,indent=2)+'\n')
    return receipt


if __name__ == '__main__':
    print(json.dumps(export(Path(sys.argv[1])),indent=2))
