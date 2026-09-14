#!/usr/bin/env python3
"""Offline event census. JSONL entries are provisional until summary is COMPLETE.

Paths use member name AND zero-based member position (duplicates are preserved),
plus array indices. They address the original JSON, never decoded byte payloads.
Only the first eight elements of each array and capped nested fields are indexed.
"""
import argparse
from collections import Counter
from decimal import Decimal
import hashlib
import json
import os
from pathlib import Path
import resource
import sys
import time

import ijson

VERSION = 1
LIMIT = 1073741824
PREVIEW = 8
FIELD_CAP = 1024
TEXT_CAP = 256
SCALARS = frozenset(('string', 'number', 'boolean', 'null'))


class InventoryError(Exception):
    pass


class HashingReader:
    def __init__(self, raw):
        self.raw = raw
        self.digest = hashlib.sha256()
        self.bytes = 0
        self.eof = False

    def read(self, size=-1):
        if size < 0:
            raise InventoryError('unbounded parser read rejected')
        data = self.raw.read(size)
        self.digest.update(data)
        self.bytes += len(data)
        if size and not data:
            self.eof = True
        return data


def stat_identity(stat):
    return {key: getattr(stat, key) for key in
            ('st_dev', 'st_ino', 'st_size', 'st_mtime_ns', 'st_ctime_ns')}


def scalar(value):
    if isinstance(value, Decimal):
        return {'decimal': str(value), 'encoding': 'exact decimal, not binary64'}
    if isinstance(value, str) and len(value) > TEXT_CAP:
        return {'preview': value[:TEXT_CAP], 'characters': len(value), 'truncated': True}
    return value


def selected(kind):
    return any(word in kind.lower() for word in
               ('b01_wb14_', 'cadence', 'provider', 'archive', 'capture',
                'serializ', 'error', 'prepared_day'))


def scan(source, index, expected_sha, expected_bytes, expected_rows):
    summary = {'version': VERSION, 'status': 'INCOMPLETE', 'source': str(source),
               'expected_sha256': expected_sha, 'expected_bytes': expected_bytes,
               'expected_physical_rows': expected_rows, 'physical_rows': 0,
               'completed_rows': 0, 'selected_rows': 0, 'kind_counts': {},
               'anomaly_counts': {}, 'syntax_complete': False, 'eof': False,
               'metadata': {'nested_field_cap': FIELD_CAP, 'array_preview_elements': PREVIEW,
                            'string_preview_characters': TEXT_CAP,
                            'locators': 'source SHA + physical ordinal + member-position path',
                            'payload_authentication': 'NOT EVALUATED'},
               'ijson': {'version': ijson.__version__, 'backend': ijson.backend},
               'rlimit_as_bytes': list(resource.getrlimit(resource.RLIMIT_AS))}
    stack = []
    record = None
    kinds = Counter()
    anomalies = Counter()
    physical_arrays = 0
    reader = None
    started = time.monotonic()
    try:
        with source.open('rb') as raw, index.open('x', buffering=1) as output:
            before = stat_identity(os.fstat(raw.fileno()))
            summary['source_stat_before'] = before
            if before['st_size'] != expected_bytes:
                raise InventoryError('source byte size mismatch before parse')
            reader = HashingReader(raw)
            for event, value in ijson.basic_parse(reader, use_float=False, buf_size=65536):
                parent = stack[-1] if stack else None
                # Dominant giant-byte-array path: no per-element object/path allocation.
                if (event in SCALARS and parent and parent['type'] == 'array'
                        and not parent['physical'] and parent['count'] >= PREVIEW):
                    parent['count'] += 1
                    continue
                if event == 'map_key':
                    if len(value) > 4096:
                        raise InventoryError('member name exceeds bounded locator limit')
                    parent['key'] = value
                    continue
                if event in ('end_map', 'end_array'):
                    frame = stack.pop()
                    if frame['meta'] is not None:
                        frame['meta']['member_count' if frame['type'] == 'map'
                                      else 'element_count'] = frame['count']
                        if frame['type'] == 'array':
                            frame['meta']['elements_not_indexed'] = max(0, frame['count'] - PREVIEW)
                    if frame['row']:
                        finish_record(record, output, summary, kinds, anomalies)
                        record = None
                    continue

                row_start = bool(parent and parent['physical'])
                path = None
                key = None
                if parent:
                    position = parent['count']
                    parent['count'] += 1
                    if parent['type'] == 'map':
                        key = parent['key']
                        if parent['path'] is not None:
                            path = parent['path'] + [{'member': key, 'position': position}]
                    elif parent['path'] is not None and position < PREVIEW:
                        path = parent['path'] + [position]
                else:
                    path = []
                is_physical = bool(len(stack) == 1 and key == 'physical')
                if is_physical:
                    if event != 'start_array':
                        raise InventoryError('root physical member is not an array')
                    physical_arrays += 1
                    if physical_arrays != 1:
                        raise InventoryError('duplicate root physical array')
                if row_start:
                    ordinal = summary['physical_rows']
                    summary['physical_rows'] += 1
                    path = []
                    record = {'physical_ordinal': ordinal, 'source_sha256': expected_sha,
                              'provisional': True, 'shape': event, 'kinds': [],
                              'kind_occurrences': 0, 'fields': [], 'nested_fields': [],
                              'nested_fields_omitted': 0, 'anomalies': []}
                    if event != 'start_map':
                        record['anomalies'].append('unexpected_row_shape')
                top_field = bool(record is not None and parent and parent['row'])
                if top_field and key == 'kind':
                    record['kind_occurrences'] += 1
                    if event != 'string' or len(value) > TEXT_CAP:
                        record['anomalies'].append('non_small_string_kind')
                    elif len(record['kinds']) < 256:
                        record['kinds'].append(value)
                    else:
                        raise InventoryError('kind occurrence cap exceeded')
                meta = None
                if record is not None and not row_start and path is not None:
                    target = record['fields'] if top_field else record['nested_fields']
                    if len(target) < FIELD_CAP:
                        meta = {'path': path, 'type': event.removeprefix('start_')}
                        if event in SCALARS:
                            meta['value'] = scalar(value)
                        target.append(meta)
                    elif top_field:
                        raise InventoryError('top-level field cap exceeded')
                    else:
                        record['nested_fields_omitted'] += 1
                if event in ('start_map', 'start_array'):
                    if len(stack) >= 128:
                        raise InventoryError('nesting exceeds bounded stack limit')
                    # No metadata outside rows; retain root path to identify physical structurally.
                    if record is None and stack:
                        path = None
                    stack.append({'type': event[6:], 'path': path, 'key': None,
                                  'count': 0, 'meta': meta, 'row': row_start,
                                  'physical': is_physical})
                elif row_start:
                    finish_record(record, output, summary, kinds, anomalies)
                    record = None
            summary['syntax_complete'] = True
            summary['eof'] = reader.eof
            summary['source_stat_after'] = stat_identity(os.fstat(raw.fileno()))
            summary['source_path_stat_after'] = stat_identity(source.stat())
            if stack or physical_arrays != 1 or not reader.eof:
                raise InventoryError('missing physical array or incomplete EOF traversal')
            if before != summary['source_stat_after'] or before != summary['source_path_stat_after']:
                raise InventoryError('source identity mutated during traversal')
            if reader.bytes != expected_bytes or reader.digest.hexdigest() != expected_sha:
                raise InventoryError('source hash/byte mismatch')
            if summary['physical_rows'] != expected_rows:
                raise InventoryError('physical row count mismatch')
            summary['status'] = 'COMPLETE'
    except (InventoryError, ijson.JSONError, OSError, MemoryError, OverflowError) as exc:
        summary['error'] = {'type': type(exc).__name__, 'detail': str(exc)[:2048]}
        if record is not None:
            summary['interrupted_physical_ordinal'] = record['physical_ordinal']
    finally:
        summary['kind_counts'] = dict(sorted(kinds.items()))
        summary['anomaly_counts'] = dict(sorted(anomalies.items()))
        summary['bytes_supplied'] = reader.bytes if reader else 0
        summary['sha256_supplied'] = reader.digest.hexdigest() if reader else None
        summary['elapsed_seconds'] = time.monotonic() - started
        summary['peak_rss_kib'] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    return summary


def finish_record(record, output, summary, kinds, anomalies):
    occurrences = record['kind_occurrences']
    if not occurrences:
        record['anomalies'].append('missing_kind')
    elif occurrences > 1:
        record['anomalies'].append('duplicate_kind')
    for kind in record['kinds']:
        if kind not in kinds and len(kinds) >= 256:
            raise InventoryError('distinct kind cap exceeded')
        kinds[kind] += 1
    anomalies.update(record['anomalies'])
    summary['completed_rows'] += 1
    if record['anomalies'] or any(selected(kind) for kind in record['kinds']):
        output.write(json.dumps(record, ensure_ascii=True, separators=(',', ':')) + '\n')
        summary['selected_rows'] += 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    parser.add_argument('output_directory', type=Path)
    parser.add_argument('--sha256', required=True)
    parser.add_argument('--bytes', required=True, type=int)
    parser.add_argument('--rows', required=True, type=int)
    args = parser.parse_args()
    limits = resource.getrlimit(resource.RLIMIT_AS)
    if limits[0] < 0 or limits[1] < 0 or max(limits) > LIMIT:
        parser.error('invoke under prlimit --as=1073741824:1073741824')
    args.output_directory.mkdir(parents=True, exist_ok=False)
    receipt = args.output_directory / 'summary.json'
    receipt.write_text(json.dumps({'status': 'INCOMPLETE', 'reason': 'process started',
                                   'argv': sys.argv, 'rlimit_as_bytes': limits}) + '\n')
    result = scan(args.source, args.output_directory / 'records.jsonl', args.sha256,
                  args.bytes, args.rows)
    result['argv'] = sys.argv
    result['utility_sha256'] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    temporary = receipt.with_suffix('.tmp')
    temporary.write_text(json.dumps(result, indent=2) + '\n')
    temporary.replace(receipt)
    print(json.dumps({key: result[key] for key in ('status', 'physical_rows', 'selected_rows',
                                                   'bytes_supplied', 'peak_rss_kib')}))
    return 0 if result['status'] == 'COMPLETE' else 1


if __name__ == '__main__':
    sys.exit(main())
