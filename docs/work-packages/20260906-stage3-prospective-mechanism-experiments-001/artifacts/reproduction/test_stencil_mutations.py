#!/usr/bin/env python3
"""Reviewer A's read-only sensitivity checks using an authentic trace prefix.

These mutate copied event records, not solver inputs or source. Counter totals
are recomputed so a rejection must detect the targeted structural inconsistency.
The complete map prefix includes the potential solve and joined fixed-final
solve. The full trace can additionally be checked with --full-trace.
"""

import argparse
import copy
import hashlib
import json
from pathlib import Path
import runpy


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def recompute_counts(data):
    starts = {e['Start']['id']: e['Start'] for e in data['events'] if 'Start' in e}
    ends = {e['End']['id']: e['End'] for e in data['events'] if 'End' in e}

    def counts(keys):
        return {
            'starts': len(keys),
            'completions': sum(ends[key]['success'] for key in keys),
            'errors': sum(not ends[key]['success'] for key in keys),
        }

    for kind, field in {
        'Map': 'maps', 'Solve': 'solves', 'Sweep': 'sweeps',
        'Probe': 'probes', 'Evaluation': 'evaluations',
    }.items():
        data[field] = counts([key for key, row in starts.items() if row['kind'] == kind])
    for kind, field in [
        ('Complete', 'complete'), ('IdentityAnchor', 'identity_anchor'),
        ('ComponentReplay', 'component_replay'),
    ]:
        data[field] = counts([
            e['Probe']['id'] for e in data['events']
            if e.get('Probe', {}).get('class') == kind
        ])
    data['iterations'] = sum('Iteration' in e for e in data['events'])


def first_complete_map(data):
    map_starts = [
        index for index, event in enumerate(data['events'])
        if event.get('Start', {}).get('kind') == 'Map'
    ]
    assert len(map_starts) >= 2, 'fixture needs two consecutive real maps'
    result = {key: value for key, value in data.items() if key != 'events'}
    result['events'] = copy.deepcopy(data['events'][map_starts[0]:map_starts[1]])
    recompute_counts(result)
    assert result['maps']['starts'] == 1 and result['maps']['completions'] == 1
    assert result['solves']['starts'] == 2 and result['solves']['completions'] == 2
    return result


def close_solve_early(data):
    events = data['events']
    index = next(i for i, e in enumerate(events) if e.get('End', {}).get('kind') == 'Solve')
    end = events.pop(index)
    start = next(i for i, e in enumerate(events) if e.get('Start', {}).get('id') == end['End']['id'])
    events.insert(start + 1, end)


def wrong_iteration_ordinals(data):
    for event in data['events']:
        if 'Iteration' in event:
            event['Iteration']['ordinal'] = 800


def remove_map_join(data):
    assert any('MapJoin' in e for e in data['events'])
    data['events'] = [e for e in data['events'] if 'MapJoin' not in e]


def failed_probe_continues(data):
    # Preserve Evaluation -> Probe outcome parity so the targeted rejection is
    # first-error continuation, not the earlier child-result consistency guard.
    probe = next(e['Start']['id'] for e in data['events'] if e.get('Start', {}).get('kind') == 'Probe')
    child = next(e['Start']['id'] for e in data['events'] if e.get('Start', {}).get('parent') == probe)
    for event in data['events']:
        if event.get('End', {}).get('id') in (probe, child):
            event['End']['success'] = False


def full_failed_sweep_without_stencils(data):
    sweep = next(e['Start']['id'] for e in data['events'] if e.get('Start', {}).get('kind') == 'Sweep')
    data['events'] = [e for e in data['events'] if e.get('Stencil', {}).get('sweep') != sweep]
    for event in data['events']:
        if event.get('End', {}).get('id') == sweep:
            event['End']['success'] = False


def failed_sweep_continues_solve(data):
    # First probe error correctly ends its sweep, but the real canonical `?`
    # must also stop this solve. Preserve later iterations/sweeps to test that
    # an otherwise well-formed local prefix cannot conceal parent continuation.
    events = data['events']
    sweep = next(e['Start']['id'] for e in events if e.get('Start', {}).get('kind') == 'Sweep')
    probe = next(e['Probe']['id'] for e in events if 'Probe' in e)
    probe_end = next(i for i, e in enumerate(events) if e.get('End', {}).get('id') == probe)
    sweep_end = next(i for i, e in enumerate(events) if e.get('End', {}).get('id') == sweep)
    failed_probe_continues(data)
    events[sweep_end]['End']['success'] = False
    data['events'] = events[:probe_end + 1] + events[sweep_end:]


def remove_complete_evaluation(data):
    probe = next(e['Probe']['id'] for e in data['events'] if e.get('Probe', {}).get('class') == 'Complete')
    child = next(e['Start']['id'] for e in data['events'] if e.get('Start', {}).get('parent') == probe)
    data['events'] = [
        e for e in data['events']
        if e.get('Start', {}).get('id') != child and e.get('End', {}).get('id') != child
    ]


def wrong_anchor(data):
    # A finite first anchor is changed by one binary64 ULP; later sweeps in
    # this same solve still carry the immutable physical anchor. This tests
    # stable per-solve identity, not equality with the changing trial value.
    row = next(e['Stencil'] for e in data['events'] if e.get('Stencil', {}).get('identity_anchor_bits') is not None)
    row['identity_anchor_bits'] ^= 1


def duplicate_join(data):
    index = next(i for i, e in enumerate(data['events']) if 'MapJoin' in e)
    data['events'].insert(index + 1, copy.deepcopy(data['events'][index]))


def missing_probe_error_match(data):
    for event in data['events']:
        if event.get('End', {}).get('kind') == 'Probe':
            event['End']['success'] = False
            return
    raise AssertionError('fixture needs a probe')


def dropped_record(data):
    data['dropped_events'] = 1


def run_checks(audit, original, replay):
    fixture = first_complete_map(original)
    baseline = audit(fixture, replay)
    assert baseline['status'] == 'PASS'
    mutations = [
        ('solve_closes_before_iteration', close_solve_early),
        ('iteration_ordinals_800', wrong_iteration_ordinals),
        ('missing_final_map_join', remove_map_join),
        ('failed_probe_continues', failed_probe_continues),
        ('failed_full_sweep_without_stencils', full_failed_sweep_without_stencils),
        ('failed_sweep_continues_solve', failed_sweep_continues_solve),
        ('missing_complete_evaluation', remove_complete_evaluation),
        ('changed_anchor_within_solve', wrong_anchor),
        ('duplicate_map_join', duplicate_join),
        ('probe_evaluation_outcome_mismatch', missing_probe_error_match),
        ('dropped_record', dropped_record),
    ]
    results = {}
    for name, mutate in mutations:
        candidate = copy.deepcopy(fixture)
        mutate(candidate)
        recompute_counts(candidate)
        try:
            audit(candidate, replay)
        except (AssertionError, KeyError, IndexError) as error:
            results[name] = {'status': 'REJECTED', 'reason': str(error)}
        else:
            results[name] = {'status': 'ACCEPTED_INVALID'}
    return {
        'status': 'PASS' if all(row['status'] == 'REJECTED' for row in results.values()) else 'FAIL',
        'evidence': 'Ran: artifact-only, in-memory mutations of authentic trace; no physics execution',
        'positive_prefix': baseline,
        'prefix_event_count': len(fixture['events']),
        'mutations': results,
    }


def main():
    if not __debug__:
        raise SystemExit('assert-enabled Python required for the audit oracle')
    parser = argparse.ArgumentParser()
    parser.add_argument('trace', type=Path)
    parser.add_argument('--oracle', type=Path, default=Path(__file__).with_name('audit_stencils.py'))
    parser.add_argument('--replay', action='store_true')
    parser.add_argument('--full-trace', action='store_true')
    args = parser.parse_args()
    original = json.loads(args.trace.read_text())
    audit = runpy.run_path(str(args.oracle))['audit']
    result = run_checks(audit, original, args.replay)
    result.update(trace_sha256=digest(args.trace), oracle_sha256=digest(args.oracle),
                  mutation_script_sha256=digest(Path(__file__)))
    if args.full_trace:
        result['full_trace'] = audit(original, args.replay)
    print(json.dumps(result, indent=2))
    if result['status'] != 'PASS':
        raise SystemExit(1)


if __name__ == '__main__':
    main()
