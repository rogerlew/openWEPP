#!/usr/bin/env python3
"""Map every removed carrier execution to its authentic unchanged invocation/result.

Transition fingerprints are attribution, not the separate complete-field carrier oracle.
"""
import argparse
from collections import Counter, defaultdict
import json
from pathlib import Path

KINDS=('Outer','Evaluator','Provider','Carrier','BatchProvider')

def validate(data):
    assert data['dropped_records']==0
    groups={kind:{} for kind in KINDS}
    for row in data['records']:
        assert row['kind'] in groups
        target=groups[row['kind']]
        assert row['ordinal'] not in target
        target[row['ordinal']]=row
    for kind,counts in zip(KINDS,data['counts']):
        rows=groups[kind]
        assert set(rows)==set(range(1,counts['started']+1))
        assert counts['completed']==sum(row['success'] for row in rows.values())
        assert counts['errors']==sum(not row['success'] for row in rows.values())
    for row in groups['Evaluator'].values():
        assert row['outer_ordinal'] in groups['Outer']
        assert row['evaluator_ordinal']==row['ordinal']
    for row in groups['Provider'].values():
        assert row['evaluator_ordinal'] in groups['Evaluator']
        assert row['outer_ordinal']==groups['Evaluator'][row['evaluator_ordinal']]['outer_ordinal']
        assert row['provider_ordinal']==row['ordinal']
    carriers=defaultdict(list)
    for row in groups['Carrier'].values():
        provider=groups['Provider'][row['provider_ordinal']]
        assert (row['outer_ordinal'],row['evaluator_ordinal'],row['output'],row['success']) == (
            provider['outer_ordinal'],provider['evaluator_ordinal'],provider['output'],provider['success'])
        assert provider['input'].endswith('request_sha256='+row['input'])
        carriers[row['provider_ordinal']].append(row)
    assert set(carriers)==set(groups['Provider'])
    assert all(len(rows)==1 for rows in carriers.values())
    return groups,carriers

def audit(a,f):
    ag,ac=validate(a)
    fg,fc=validate(f)
    for kind in ('Outer','Evaluator','BatchProvider'):
        assert ag[kind]==fg[kind], 'independent invocation/result altered: '+kind
    removed=[]
    multiplicity=Counter()
    for evaluator in ag['Evaluator']:
        before=[row for row in ag['Provider'].values() if row['evaluator_ordinal']==evaluator]
        after=[row for row in fg['Provider'].values() if row['evaluator_ordinal']==evaluator]
        multiplicity[(len(before),len(after))]+=1
        assert len(before) in (1,2) and len(after)==1
        assert before[0]['input']==after[0]['input'], 'retained physical request differs'
        for row in before:
            assert (row['success'],row['output'])==(after[0]['success'],after[0]['output'])
        for row in before[1:]:
            assert ';coupling=1;' in row['input'] and ';coupling=0;' in before[0]['input']
            removed.append({'outer':row['outer_ordinal'],'evaluator':evaluator,
                            'baseline_removed_provider':row['ordinal'],
                            'baseline_retained_provider':before[0]['ordinal'],
                            'candidate_provider':after[0]['ordinal'],
                            'retained_request':after[0]['input'],
                            'unchanged_result_fingerprint':row['output']})
    assert len(ag['Carrier'])-len(fg['Carrier'])==len(removed)
    return {'status':'PASS','baseline_counts':a['counts'],'candidate_counts':f['counts'],
            'multiplicity':{'%d_to_%d'%key:count for key,count in multiplicity.items()},
            'removed_executions':removed,
            'limitation':'fingerprint attribution only; complete-field forced-two-call oracle required separately'}

def main():
    p=argparse.ArgumentParser()
    p.add_argument('baseline',type=Path)
    p.add_argument('candidate',type=Path)
    a=p.parse_args()
    print(json.dumps(audit(json.loads(a.baseline.read_text()),json.loads(a.candidate.read_text())),indent=2))

if __name__=='__main__':
    main()
