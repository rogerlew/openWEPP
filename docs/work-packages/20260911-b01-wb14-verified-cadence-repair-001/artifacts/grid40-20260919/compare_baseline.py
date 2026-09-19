"""Offline exact comparison to the frozen first-solve trace; never runs a solver."""
import argparse
import hashlib
import json
from pathlib import Path
import struct


def require(condition, detail):
    if not condition:
        raise ValueError(detail)


def compare(expected, actual, path):
    if isinstance(expected, dict):
        require(isinstance(actual, dict), (path, 'not object'))
        require(set(expected) == set(actual), (path, 'object key mismatch'))
        for key, value in expected.items():
            compare(value, actual[key], path + '/' + key)
    elif isinstance(expected, list):
        require(isinstance(actual, list) and len(expected) == len(actual), (path, 'list shape'))
        for index, (left, right) in enumerate(zip(expected, actual)):
            compare(left, right, path + '/' + str(index))
    elif isinstance(expected, float):
        require(isinstance(actual, float), (path, 'binary64 representation type'))
        require(struct.pack('>d', expected) == struct.pack('>d', actual), (path, 'binary64 bits', expected, actual))
    else:
        require(type(actual) is type(expected) and actual == expected, (path, expected, actual))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('actual', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    reference = Path(__file__).resolve().parent.parent / 'lse-first-trial-20260918/final-first-failure-trace.json'
    frozen_bytes = reference.read_bytes()
    require(hashlib.sha256(frozen_bytes).hexdigest() == '527208c3e5ad07f1744f1bd92c9e4ac17cf7efd9469968a97d7964c5db482dfd', 'frozen reference hash mismatch')
    reference_document = json.loads(frozen_bytes)
    expected = [r for r in reference_document['records'] if r['kind'] != 'diagnostic_directional_probe']
    actual_bytes = args.actual.read_bytes()
    actual_document = json.loads(actual_bytes)
    require(set(actual_document) == set(reference_document), 'top-level keys mismatch')
    compare(reference_document['schema'], actual_document['schema'], '/schema')
    records = actual_document['records']
    require(not any(r['kind'] == 'diagnostic_directional_probe' for r in records), 'forbidden extra probe')
    compare(expected, records, '/ordinary_records')
    result = {
        'evidence_class': 'Ran: offline full ordinary-trace binary64/typed-value comparison; no evaluator calls',
        'reference_sha256': hashlib.sha256(frozen_bytes).hexdigest(),
        'actual_trace_sha256': hashlib.sha256(actual_bytes).hexdigest(),
        'ordinary_record_count': len(expected),
        'ordinary_record_values_and_order_match': True,
        'signed_zero_bits_preserved': True,
        'top_level_schema_match': True,
        'uncompared_fields_or_records': 0,
        'diagnostic_probe_records': 0,
    }
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
