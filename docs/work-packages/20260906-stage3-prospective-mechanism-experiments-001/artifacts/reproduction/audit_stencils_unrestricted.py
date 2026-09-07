#!/usr/bin/env python3
"""Independent binary64 coordinate-domain oracle, separate from measured execution."""
import argparse
from collections import Counter
import json
import math
from pathlib import Path
import struct

def value(bits):
    return struct.unpack('<d', struct.pack('<Q', bits))[0]

def bits(value):
    return struct.unpack('<Q', struct.pack('<d', value))[0]

def domain(index, n, liquid_ground):
    if index < 10*n:
        local = index % 10
        if local < 4:
            return -math.inf, math.inf, 1000.0
        if local < 6:
            return 0.0, 1.0, 1.0
        return (273.15 if local < 9 else 200.0), 350.0, 1.0
    local = index - 10*n
    if local == 1:
        return 0.0, 0.1, .001
    return (273.15 if local == 2 and liquid_ground else 200.0), 350.0, 1.0

def enumerate_sweep(base, replay):
    n, soil = base['occupancies'], base['soil_nodes']
    coordinates = [value(raw) for raw in base['base_bits']]
    assert len(coordinates) == 10*n + 3 + soil
    probes, stencils = [], []
    for index, current in enumerate(coordinates):
        lower, upper, unit = domain(index,n,base['liquid_ground'])
        assert math.isfinite(current) and lower <= current <= upper
        step = math.sqrt(2.0**-52) * max(abs(current),unit)
        minus, plus = current-step, current+step
        legal = [math.isfinite(v) and lower <= v <= upper for v in (minus,plus)]
        stencils.append((index,bits(minus),bits(plus),*legal))
        if base['represented_snow'] and index >= 10*n+2:
            kind = 'IdentityAnchor'
        elif replay and base['represented_snow'] and index < 10*n and index%10 >= 6:
            kind = 'ComponentReplay'
        else:
            kind = 'Complete'
        for signed, admitted in zip((minus,plus),legal):
            if admitted:
                probes.append((index,bits(signed),kind))
    return probes, stencils

def audit(data, replay):
    assert not data['overflow'] and data['dropped_events'] == 0
    starts, ends, bases, stencils, probes, iterations = {}, {}, {}, {}, {}, {}
    active=[]
    pending_join=None
    solve_iterations=Counter()
    children={}
    last_iteration={}
    aborted=set()
    map_joins = []
    for event in data['events']:
        assert len(event)==1
        kind,row = next(iter(event.items()))
        if active and active[-1] in aborted:
            assert kind=='End' and row['id']==active[-1] and not row['success'], 'work after propagated failure'
        if kind == 'Start':
            assert row['id'] not in starts and row['id'] not in iterations
            parent=row['parent']
            if pending_join is not None:
                assert not active and row['kind']=='Solve' and parent==pending_join
                pending_join=None
            else:
                assert parent==(active[-1] if active else None), 'inactive/foreign parent'
            allowed={'Map':{None},'Solve':{'Map',None},'Sweep':{'Solve'},
                     'Probe':{'Sweep'},'Evaluation':{'Solve','Probe',None}}
            assert (starts[parent]['kind'] if parent is not None else None) in allowed[row['kind']],row
            starts[row['id']] = row
            children.setdefault(parent,[]).append(row['id'])
            active.append(row['id'])
        elif kind == 'End':
            assert row['id'] in starts and row['id'] not in ends
            assert starts[row['id']]['kind'] == row['kind']
            assert active and active.pop()==row['id'], 'scope end out of order'
            ends[row['id']] = row
            if not row['success'] and row['kind'] in ('Probe','Sweep','Solve') and active:
                aborted.add(active[-1])
        elif kind == 'Iteration':
            assert row['id'] not in starts and row['id'] not in iterations
            assert starts[row['solve']]['kind']=='Solve'
            assert active and active[-1]==row['solve']
            assert row['ordinal']==solve_iterations[row['solve']]
            solve_iterations[row['solve']]+=1
            last_iteration[row['solve']]=row['id']
            iterations[row['id']] = row
        elif kind == 'MapJoin':
            assert starts[row['map']]['kind']=='Map'
            assert not active and pending_join is None
            assert row['map'] in ends and ends[row['map']]['success']
            assert row['map'] not in map_joins
            pending_join=row['map']
            map_joins.append(row['map'])
        elif kind == 'SweepBase':
            assert row['sweep'] not in bases and starts[row['sweep']]['kind']=='Sweep'
            assert active and active[-1]==row['sweep']
            assert row['iteration'] in iterations
            assert iterations[row['iteration']]['solve'] == starts[row['sweep']]['parent']
            assert row['iteration']==last_iteration[starts[row['sweep']]['parent']]
            bases[row['sweep']]=row
        elif kind == 'Stencil':
            assert row['sweep'] in bases
            assert active and active[-1]==row['sweep']
            stencils.setdefault(row['sweep'],[]).append(row)
        elif kind == 'Probe':
            assert starts[row['id']]['kind']=='Probe' and row['id'] not in probes
            assert active and active[-1]==row['id']
            sweep=starts[row['id']]['parent']
            assert stencils[sweep][-1]['coordinate']==row['coordinate']
            probes[row['id']]=row
        else:
            raise AssertionError('unknown event '+kind)
    assert starts.keys()==ends.keys(), 'unclosed/lost lifecycle'
    assert not active and pending_join is None
    for identity,row in starts.items():
        if row['kind']=='Map':
            solves=[key for key in children.get(identity,[]) if starts[key]['kind']=='Solve']
            assert len(solves)==(2 if identity in map_joins else 1)
            if ends[identity]['success']:
                assert identity in map_joins, 'successful real map missing final join'
        if row['kind']=='Probe':
            evaluations=children.get(identity,[])
            if probes[identity]['class'] in ('Complete','ComponentReplay'):
                assert len(evaluations)==1 and starts[evaluations[0]]['kind']=='Evaluation'
                assert ends[evaluations[0]]['success']==ends[identity]['success']
            else:
                assert not evaluations, 'identity anchor performed evaluation'
    field_for_kind = {'Map':'maps','Solve':'solves','Sweep':'sweeps','Probe':'probes','Evaluation':'evaluations'}
    for kind,field in field_for_kind.items():
        selected = [key for key,row in starts.items() if row['kind']==kind]
        expected = {'starts':len(selected), 'completions':sum(ends[key]['success'] for key in selected),
                    'errors':sum(not ends[key]['success'] for key in selected)}
        assert data[field] == expected, (field,data[field],expected)
    assert data['iterations']==len(iterations)
    assert len(bases)==data['sweeps']['starts']
    assert len(probes)==data['probes']['starts']
    histogram, totals = Counter(), Counter()
    probe_ids_by_sweep={}
    anchors_by_solve={}
    for identity in probes:
        probe_ids_by_sweep.setdefault(starts[identity]['parent'],[]).append(identity)
    for sweep,base in bases.items():
        expected, expected_stencils = enumerate_sweep(base,replay)
        actual_ids=probe_ids_by_sweep.get(sweep,[])
        actual = [probes[identity] for identity in actual_ids]
        actual_tuples = [(row['coordinate'],row['value_bits'],row['class']) for row in actual]
        success = ends[sweep]['success']
        failed=[index for index,identity in enumerate(actual_ids) if not ends[identity]['success']]
        if failed:
            assert failed==[len(actual_ids)-1] and not success, 'work continued after failed probe'
        if success:
            assert actual_tuples==expected, ('completed sweep',sweep)
        else:
            assert actual_tuples==expected[:len(actual_tuples)], ('failed prefix',sweep)
        observed_stencils = [(row['coordinate'],row['minus_bits'],row['plus_bits'],row['minus_valid'],row['plus_valid'])
                             for row in stencils.get(sweep,[])]
        assert observed_stencils == expected_stencils[:len(observed_stencils)]
        for row in stencils.get(sweep,[]):
            anchored=base['represented_snow'] and row['coordinate']>=10*base['occupancies']+2
            anchor=row['identity_anchor_bits']
            if anchored:
                assert anchor is not None and math.isfinite(value(anchor)) and 200 <= value(anchor) <= 350
                key=(starts[sweep]['parent'],row['coordinate'])
                assert anchors_by_solve.setdefault(key,anchor)==anchor
            else:
                assert anchor is None
        if actual:
            assert len(observed_stencils)==actual[-1]['coordinate']+1, 'stencil/probe prefix mismatch'
        if success:
            assert len(observed_stencils)==len(expected_stencils)
        class_counts = Counter(row['class'] for row in actual)
        histogram[(len(actual),class_counts['IdentityAnchor'],class_counts['ComponentReplay'],class_counts['Complete'])]+=1
        totals.update(class_counts)
    for kind,field in [('Complete','complete'),('IdentityAnchor','identity_anchor'),('ComponentReplay','component_replay')]:
        keys = [key for key,row in probes.items() if row['class']==kind]
        expected = {'starts':len(keys),'completions':sum(ends[key]['success'] for key in keys),
                    'errors':sum(not ends[key]['success'] for key in keys)}
        assert data[field]==expected
    if replay:
        assert data['component_replay']['completions']>0, 'real replay must execute'
    return {'status':'PASS', 'oracle':'independent coordinate-domain binary64 enumeration',
            'replay_expected':replay,'maps':data['maps'],'solves':data['solves'],
            'iterations':len(iterations),'sweeps':len(bases),'probe_classes':dict(totals),
            'map_final_joins':len(map_joins),'histogram':{'/'.join(map(str,key)):count for key,count in sorted(histogram.items())},
            'dropped_events':data['dropped_events'],
            'limitations':'graph eligibility assumes reviewed complete descriptor for represented-snow component coordinates; direct-edge oracle is independently required'}

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('trace',type=Path)
    parser.add_argument('--replay',action='store_true')
    args=parser.parse_args()
    print(json.dumps(audit(json.loads(args.trace.read_text()),args.replay),indent=2))

if __name__=='__main__':
    main()
