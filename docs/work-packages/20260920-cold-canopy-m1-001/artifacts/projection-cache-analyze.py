"""Summarize the frozen six paired batches without dropping or selecting samples."""
import argparse
import hashlib
import json
from pathlib import Path
import statistics


def main():
    p = argparse.ArgumentParser()
    p.add_argument('cohort', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    rows = json.loads(a.cohort.read_text())
    expected = [('warmup-' + arm, arm, 16) for arm in ('baseline', 'treatment')]
    for i in range(1, 7):
        order = ('baseline', 'treatment') if i % 2 else ('treatment', 'baseline')
        expected.extend((f'pair-{i}-{arm}', arm, 32) for arm in order)
    if [(r['label'], r['arm'], r['count']) for r in rows] != expected:
        raise RuntimeError('Cohort differs from frozen schedule')
    reference = rows[0]['result']['outcomes'][0]
    for r in rows:
        if r['exit_code'] != 0 or 'error' in r:
            raise RuntimeError('Failed batch ' + r['label'])
        if len(r['result']['outcomes']) != r['count']:
            raise RuntimeError('Incomplete outcomes ' + r['label'])
        if any(x != reference for x in r['result']['outcomes']):
            raise RuntimeError('Numerical mismatch ' + r['label'])
        if (r['result']['projection_count_mode'] != 'disabled_optional_capture'
                or r['result']['projection_counts'] != [None] * r['count']):
            raise RuntimeError('Optional counter capture during timing ' + r['label'])
        if any(r[stage][clock + '_ns'] <= 0
               for stage in ('initialization', 'proposal') for clock in ('cpu', 'wall')):
            raise RuntimeError('Nonpositive timing interval ' + r['label'])
    paired = []
    for i in range(1, 7):
        arms = {r['arm']: r for r in rows if r['label'].startswith(f'pair-{i}-')}
        item = {'pair': i, 'order': 'B/T' if i % 2 else 'T/B'}
        for stage in ('initialization', 'proposal', 'combined'):
            item[stage] = {}
            for clock in ('cpu', 'wall'):
                costs = {}
                for arm, row in arms.items():
                    total = (row['initialization'][clock + '_ns'] + row['proposal'][clock + '_ns']
                             if stage == 'combined' else row[stage][clock + '_ns'])
                    costs[arm + '_batch_ns'] = total
                    costs[arm + '_us_per_proposal'] = total / row['count'] / 1000
                costs['paired_speedup'] = costs['baseline_batch_ns'] / costs['treatment_batch_ns']
                item[stage][clock] = costs
        paired.append(item)
    medians = {}
    for stage in ('initialization', 'proposal', 'combined'):
        medians[stage] = {}
        for clock in ('cpu', 'wall'):
            values = [r[stage][clock] for r in paired]
            ratios = [v['paired_speedup'] for v in values]
            medians[stage][clock] = {
                'baseline_us_per_proposal': statistics.median(v['baseline_us_per_proposal'] for v in values),
                'treatment_us_per_proposal': statistics.median(v['treatment_us_per_proposal'] for v in values),
                'median_paired_speedup': statistics.median(ratios),
                'min_paired_speedup': min(ratios), 'max_paired_speedup': max(ratios)}
    result = dict(evidence_class='Ran: exact finite-cohort arithmetic reduction, no additional physical execution',
                  cohort_sha256=hashlib.sha256(a.cohort.read_bytes()).hexdigest(),
                  numerical_outcomes_equal=True, proposals_per_arm=208,
                  measured_proposals_per_arm=192, pairs=paired, summary=medians,
                  combined_definition='Sum of separately measured initialization and proposal intervals; excludes between-interval waits.',
                  limits='Proposal costs only. No completed solve/OFE-day/100-year evidence. No historical-observer cost credited to cache.')
    with a.output.open('x') as f:
        json.dump(result, f, indent=2)
        f.write('\n')
    print(json.dumps(medians, indent=2))


if __name__ == '__main__':
    main()
