#!/usr/bin/env python3
"""Count declared reading exposure; never infer semantic obligations or tokens."""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path


def report(root, spec, revision=None):
    def read(path, binding=None):
        if Path(path).is_absolute() or '..' in Path(path).parts:
            raise ValueError('unsafe reading path')
        selected_revision = binding or revision
        if selected_revision:
            return subprocess.check_output(['git', 'show', f'{selected_revision}:{path}'], cwd=root)
        return (root / path).read_bytes()

    results = {}
    for role, assignment in spec['roles'].items():
        seen = {}
        stages = {}
        for stage in ('bootstrap', 'expansion'):
            exposure = 0
            rows = []
            stage_seen = {}

            def visit(key, stack=()):
                nonlocal exposure
                if key in stack:
                    raise ValueError('cyclic mandatory reading binding')
                item = spec['readings'][key]
                path = item['path']
                data = read(path, item.get('revision'))
                lines = data.splitlines(keepends=True)
                first, last = item.get('lines', [1, len(lines)])
                if not 1 <= first <= last <= len(lines):
                    raise ValueError('invalid reading range')
                start = sum(map(len, lines[:first-1]))
                end = sum(map(len, lines[:last]))
                count = item.get('exposures', 1)
                if not isinstance(count, int) or count < 1:
                    raise ValueError('invalid exposure count')
                exposure += (end-start)*count
                positions = set(range(start, end))
                stage_seen.setdefault(path, set()).update(positions)
                seen.setdefault(path, set()).update(positions)
                rows.append(dict(path=path, lines=[first,last], bytes=end-start,
                                 exposures=count, sha256=hashlib.sha256(data).hexdigest(),
                                 reason=item['reason']))
                for child in item.get('requires', []):
                    visit(child, (*stack,key))

            for key in assignment.get(stage, []):
                visit(key)
            stages[stage] = dict(unique_bytes=sum(map(len,stage_seen.values())),
                                 exposure_bytes=exposure, reads=rows)
        results[role] = dict(**stages, combined_unique_bytes=sum(map(len,seen.values())))
    return dict(measurement='structural declared file/range bytes, not session telemetry',
                revision=revision or 'working-tree', roles=results,
                workflow_total='UNOBSERVED', runtime_automatic_context='UNOBSERVED beyond declared repository/role instructions')


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('selection', type=Path)
    p.add_argument('--root', type=Path, default=Path.cwd())
    p.add_argument('--revision')
    p.add_argument('--phase', choices=['before','after'])
    a = p.parse_args()
    selection = json.loads(a.selection.read_text())
    revision = a.revision
    if a.phase:
        if a.phase == 'before' and revision is None:
            revision = selection['baseline_revision']
        selection = selection[a.phase]
    print(json.dumps(report(a.root, selection, revision), indent=2))
