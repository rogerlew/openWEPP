#!/usr/bin/env python3
"""Serial frozen development/extension execution; failed cases remain rows."""
import argparse
import datetime
import json
from pathlib import Path
from collect import PACKAGE, collect, digest


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('binary', type=Path)
    parser.add_argument('source', type=Path)
    parser.add_argument('directory', type=Path)
    parser.add_argument('--extension', action='store_true')
    args = parser.parse_args()
    cases = json.loads((PACKAGE / 'artifacts/cases.json').read_text())['cases']
    if args.extension:
        jobs = [(p, c, n) for c, n in [('long_transition', 1),
                ('heldout_intermittent', 1), ('continuity', 10), ('continuity', 19)]
                for p in ['R0', 'C600', 'B01']]
    else:
        jobs = [(p, c, 1) for c, spec in cases.items()
                if spec['split'] == 'development'
                for p in ['R0', 'P1', 'P2', 'B01', 'B1', 'B5', 'C600', 'C1800']]
        jobs.append(('P1_STEP', 'continuity', 1))
    args.directory.mkdir(parents=True, exist_ok=False)
    paths = [args.binary, args.source, PACKAGE / 'artifacts/cases.json',
             PACKAGE / 'artifacts/metrics.json', PACKAGE / 'collect.py', Path(__file__)]
    frozen = {str(p.resolve()): digest(p) for p in paths}
    protocol = dict(created_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                    frozen_files=frozen, jobs=jobs, physical=True, timeout_s=180,
                    selection='C600 and B01 fixed exploratory representatives; no qualified finalist',
                    failure='all failures retained; no retry or successful-prefix speedup')
    (args.directory / 'protocol.json').write_text(json.dumps(protocol, indent=2) + '\n')
    rows = []
    for policy, case, ofes in jobs:
        if any(digest(Path(p)) != h for p, h in frozen.items()):
            raise ValueError('frozen matrix identity changed')
        name = f'{policy}-{case}-{ofes}ofe'
        receipt = collect(args.binary, args.directory / name, policy, case, ofes)
        row = dict(name=name, policy=policy, case=case, ofes=ofes,
                   execution_valid=receipt['execution_valid'],
                   runner_wall_s=receipt.get('runner_wall_s'),
                   timeout=receipt['timeout'], receipt_sha256=digest(args.directory / name / 'receipt.json'))
        rows.append(row)
        (args.directory / 'progress.json').write_text(json.dumps(rows, indent=2) + '\n')
        print(json.dumps(row), flush=True)


if __name__ == '__main__':
    main()
