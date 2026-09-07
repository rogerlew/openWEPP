#!/usr/bin/env python3
"""Prespecified paired summaries; no sample exclusion, fitting or rerun selection."""
import argparse
from collections import defaultdict
import json
from pathlib import Path
import random
from statistics import median

def percentile(values,p):
    values=sorted(values)
    index=(len(values)-1)*p
    lo=int(index)
    return values[lo]+(values[min(lo+1,len(values)-1)]-values[lo])*(index-lo)

def bootstrap(rows):
    rng=random.Random(20260906)
    gain,saved=[],[]
    for _ in range(10000):
        selected=[rows[rng.randrange(len(rows))] for _ in rows]
        gain.append(median(row['wall_improvement'] for row in selected))
        saved.append(median(row['seconds_saved'] for row in selected))
    return {'resamples':10000,'seed':20260906,
            'median_improvement_95pct':[percentile(gain,.025),percentile(gain,.975)],
            'median_seconds_saved_95pct':[percentile(saved,.025),percentile(saved,.975)]}

def summarize(records):
    if not records or any(not row['valid'] for row in records):
        raise ValueError('empty/invalid series; retain failures and do not report speedup')
    groups=defaultdict(dict)
    for row in records:
        if row['label']=='measured':
            key=(row['series'],row['pair'])
            if row['arm'] in groups[key]:
                raise ValueError('duplicate pair arm')
            groups[key][row['arm']]=row
    pairs=[]
    memory=defaultdict(lambda:defaultdict(list))
    for key,arms in sorted(groups.items()):
        if set(arms)!={'A','B'}:
            raise ValueError('incomplete pair '+str(key))
        a,b=arms['A'],arms['B']
        wall=lambda row:sum(r['run_wall_us'] for r in row['records'])/1e6
        cpu=lambda row:row['process_user_s']+row['process_system_s']
        runner_cpu=lambda row:sum(r['cpu_ticks']/r['clock_tick_hz'] for r in row['records'])
        pairs.append({'series':key[0],'pair':key[1], 'A_wall_s':wall(a),'B_wall_s':wall(b),
                      'seconds_saved':wall(a)-wall(b),'wall_ratio':wall(b)/wall(a),
                      'wall_improvement':1-wall(b)/wall(a),
                      'process_cpu_ratio':cpu(b)/cpu(a),
                      'runner_cpu_ratio':runner_cpu(b)/runner_cpu(a)})
        for arm,row in arms.items():
            memory[arm]['lifetime_peak_rss_kib'].append(row['lifetime_peak_rss_kib'])
            if row['observer_requested']:
                phases={(r['iteration'],r['phase']):r for r in row['phases']}
                for i in range(row['repeats']):
                    for phase in ('pre_fixture','pre_run','end_run','post_validation','post_drop'):
                        memory[arm][phase+'_rss_kib'].append(phases[i,phase]['rss_kib'])
                    memory[arm]['post_drop_minus_pre_fixture_kib'].append(
                        phases[i,'post_drop']['rss_kib']-phases[i,'pre_fixture']['rss_kib'])
                drops=[phases[i,'post_drop']['rss_kib'] for i in range(row['repeats'])]
                memory[arm]['post_drop_iteration_vectors_kib'].append(drops)
                memory[arm]['last_minus_first_drop_kib'].append(drops[-1]-drops[0])
                memory[arm]['sample_gaps_ms'].extend((right['monotonic_ns']-left['monotonic_ns'])/1e6
                    for left,right in zip(row['samples'],row['samples'][1:]))
                memory[arm]['active_sampled_max_rss_kib'].extend(
                    r['maximum_rss_kib'] for r in row['active_sampled_maxima'] if r['maximum_rss_kib'] is not None)
    if not pairs:
        raise ValueError('no measured pairs')
    interval=bootstrap(pairs)
    gain=median(row['wall_improvement'] for row in pairs)
    cpu_ratio=median(row['process_cpu_ratio'] for row in pairs)
    return {'pairs':pairs,'pair_count':len(pairs),'warmup_processes':sum(r['label']=='warmup' for r in records),
            'median_A_wall_s':median(r['A_wall_s'] for r in pairs),
            'median_B_wall_s':median(r['B_wall_s'] for r in pairs),
            'median_seconds_saved':median(r['seconds_saved'] for r in pairs),
            'median_wall_improvement':gain,'median_process_cpu_ratio':cpu_ratio,
            'median_runner_cpu_ratio':median(r['runner_cpu_ratio'] for r in pairs),
            'bootstrap':interval,'timing_standalone_predicate':gain>=.05 and interval['median_improvement_95pct'][0]>0 and cpu_ratio<=1.02,
            'memory_raw_summaries':memory,
            'limitations':'predicate is timing only, not scientific/memory admission; repeated-run wall totals are ten separate complete runs, not continuity'}

def main():
    p=argparse.ArgumentParser()
    p.add_argument('results',type=Path,nargs='+')
    a=p.parse_args()
    rows=[json.loads(line) for path in a.results for line in path.read_text().splitlines()]
    print(json.dumps(summarize(rows),indent=2))

if __name__=='__main__':
    main()
