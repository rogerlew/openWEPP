#!/usr/bin/env python3
"""Compare complete natural-limit records without any normalization/field removal."""
import argparse
import hashlib
import json
from pathlib import Path

MARKER = 'STAGE3_NATURAL_LIMIT_DIAGNOSTIC '


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read_record(path):
    raw = path.read_bytes()
    receipt = json.loads(Path(str(path) + '.json').read_text())
    if receipt['exit_code'] != 0 or receipt['log_sha256'] != digest(raw):
        raise ValueError('unsuccessful or unbound execution: ' + str(path))
    records = [json.loads(line.split(MARKER, 1)[1])
               for line in raw.decode().splitlines() if MARKER in line]
    if len(records) != 1:
        raise ValueError('expected exactly one complete record: ' + str(path))
    record = records[0]
    if record['schema'] != 'stage3-natural-limit-diagnostic-parity-v1':
        raise ValueError('unknown observation schema')
    audit = record['audit']
    if audit['overflow'] or audit['dropped_events'] != 0 or not audit['events']:
        raise ValueError('incomplete detailed observation')
    encoded = json.dumps(record, sort_keys=True, separators=(',', ':'),
                         allow_nan=False).encode()
    return encoded, {
        'log': str(path), 'log_sha256': digest(raw),
        'record_sha256': digest(encoded), 'record_bytes': len(encoded),
        'audit_events': len(audit['events']),
        'newton_sweep_bases': len(record['newton_sweep_base_bits']),
        'failure_iterations': record['failure']['iterations'],
        'failure_kind': record['failure']['kind'],
        'component_replay_starts': audit['component_replay']['starts'],
        'identity_anchor_starts': audit['identity_anchor']['starts'],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for arm in ('A', 'F', 'R'):
        parser.add_argument(arm, type=Path)
    args = parser.parse_args()
    rows = {arm: read_record(getattr(args, arm)) for arm in ('A', 'F', 'R')}
    if any(rows[arm][0] != rows['A'][0] for arm in ('F', 'R')):
        raise ValueError('complete diagnostic/input/trajectory record differs')
    print(json.dumps({
        'status': 'PASS', 'comparison': 'all record fields, no exceptions',
        'arms': {arm: row[1] for arm, row in rows.items()},
        'limits': 'Preserved HistoricalV8 failure route only; unchanged golden '
                  'FAIL remains separate. Newton bases are not a full LU or '
                  'per-line-search-trial trace. DTO owner hashes are conversion '
                  'lineage, not an executed external-owner workflow.',
    }, indent=2))


if __name__ == '__main__':
    main()
