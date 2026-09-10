#!/usr/bin/env python3
"""Reconstruct every frozen matrix row and retain the unqualified cost frontier."""
import argparse
import json
import math
from pathlib import Path
from analyze import project as physical
from project_outputs import project as outputs
from compare import compare
from collect import digest


def read(path):
    return json.loads(path.read_text())


def write(path, value):
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + '\n')


def optional_object(path, errors):
    try:
        value = read(path)
        if not isinstance(value, dict):
            raise ValueError('expected JSON object')
        return value
    except (ValueError, OSError) as error:
        errors[path.name] = repr(error)
        return {}


def main():
    p = argparse.ArgumentParser()
    p.add_argument('output', type=Path)
    p.add_argument('matrices', nargs='+', type=Path)
    a = p.parse_args()
    rows = []
    for matrix in a.matrices:
        protocol, progress = read(matrix / 'protocol.json'), read(matrix / 'progress.json')
        expected = [tuple(job) for job in protocol['jobs']]
        actual = [(r['policy'], r['case'], r['ofes']) for r in progress]
        if actual != expected:
            raise ValueError('incomplete or reordered matrix')
        projection_errors = {}
        for item in progress:
            directory = matrix / item['name']
            if digest(directory / 'receipt.json') != item['receipt_sha256']:
                raise ValueError('matrix receipt drift')
            receipt = read(directory / 'receipt.json')
            if any(item[key] != receipt[key] for key in ['policy', 'case', 'ofes', 'execution_valid']):
                raise ValueError('matrix row/receipt identity mismatch')
            if item['name'] != f"{receipt['policy']}-{receipt['case']}-{receipt['ofes']}ofe":
                raise ValueError('matrix row name mismatch')
            if item['execution_valid']:
                try:
                    write(directory / 'physical.json', physical(directory))
                    write(directory / 'output-physical.json', outputs(directory))
                except (ValueError, KeyError, OSError, TypeError, OverflowError) as error:
                    projection_errors[item['name']] = repr(error)
        for item in progress:
            directory = matrix / item['name']
            reference = matrix / f"R0-{item['case']}-{item['ofes']}ofe"
            try:
                if item['name'] in projection_errors or reference.name in projection_errors:
                    raise ValueError(str(projection_errors))
                result = compare(reference, directory)
            except (ValueError, KeyError, OSError, TypeError, OverflowError) as error:
                result = dict(policy=item['policy'], case=item['case'], ofes=item['ofes'],
                              candidate_execution_valid=item['execution_valid'],
                              qualified=False, disposition='ANALYSIS_FAILED',
                              analysis_error=repr(error), metrics=[], speedup=None)
            artifact_errors = {}
            run = optional_object(directory / 'run.json', artifact_errors)
            receipt = read(directory / 'receipt.json')
            observation = optional_object(directory / 'observations.json', artifact_errors)
            result['optional_artifact_errors'] = artifact_errors
            result['run_directory'] = str(directory.resolve())
            result['receipt_sha256'] = digest(directory / 'receipt.json')
            result['binary_sha256'] = receipt['binary_sha256']
            result['matrix_protocol_sha256'] = digest(matrix / 'protocol.json')
            result['requested_days'] = run.get('days_requested')
            result['completed_days'] = receipt.get('completed_days')
            result['observed_work'] = dict(carrier=run.get('carrier'), lse=run.get('lse'),
                                           method_counts=observation.get('counts'))
            if receipt['execution_valid'] and result['disposition'] != 'ANALYSIS_FAILED':
                parent = math.fsum(r['parent_seconds'] for r in run['parents'])
                result['descriptive_adaptive_parent_s'] = parent
                result['outside_adaptive_parent_s'] = run['runner_wall_s'] - parent
                result['parent_timing_scope'] = 'sequential adaptive parent spans include induced coupled/snow-free successor work; not exclusive snow arithmetic'
                audits = read(directory / 'physical.json')['audits']
                result['partial_snow_audit'] = {
                    'accepted_ledger_count': len(audits),
                    'maximum_absolute_mass_kg_m2': max((abs(x['mass_kg_m2']) for x in audits), default=None),
                    'maximum_absolute_energy_j_m2': max((abs(x['energy_j_m2']) for x in audits), default=None),
                    'cumulative_absolute_mass_kg_m2': math.fsum(abs(x['mass_kg_m2']) for x in audits),
                    'cumulative_absolute_energy_j_m2': math.fsum(abs(x['energy_j_m2']) for x in audits),
                    'all_pass': bool(audits) and all(x['pass'] for x in audits),
                    'scope': 'accepted snow supports only; cumulative sums are unweighted sums of local OFE-ground-basis residual magnitudes, not hillslope-area balances; whole-system and native receiver not independently closed'}
            write(directory / 'comparison.json', result)
            rows.append(result)
    write(a.output, dict(schema='snow_accuracy_frontier_v1', rows=rows,
        qualified_frontier=[], qualification='HOLD',
        interpretation='Available value comparisons and measured costs remain visible. No complete comparative band or production qualification is established. Failed rows have no speedup.',
        provenance=dict(analyzer_sha256=digest(Path(__file__)),
                        matrices=[str(m.resolve()) for m in a.matrices])))


if __name__ == '__main__':
    main()
