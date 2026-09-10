#!/usr/bin/env python3
"""Frozen balanced fresh-process timing; includes every failure, never retries."""
import argparse
import datetime
import hashlib
import json
import math
from pathlib import Path
import statistics
from collect import collect, digest, PACKAGE


def main():
    p=argparse.ArgumentParser();p.add_argument('binary',type=Path);p.add_argument('directory',type=Path);p.add_argument('--candidate',action='append',required=True);p.add_argument('--case',default='continuity');p.add_argument('--ofes',type=int,choices=(1,10,19),default=1);p.add_argument('--source',type=Path,required=True);a=p.parse_args()
    if not 1<=len(set(a.candidate))==len(a.candidate)<=2:raise ValueError('one or two distinct candidates')
    a.directory.mkdir(parents=True,exist_ok=False)
    policies=['R0']+a.candidate
    jobs=[dict(name=f'warmup-{policy}-{i}',policy=policy,kind='warmup') for i in range(2) for policy in policies]
    for candidate in a.candidate:
        for pair in range(6):
            for policy in (['R0',candidate] if pair%2==0 else [candidate,'R0']):
                jobs.append(dict(name=f'{candidate}-pair-{pair}-{policy}',policy=policy,candidate=candidate,pair=pair,kind='pair'))
    protocol=dict(schema='snow_accuracy_balanced_series_v1',created_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),binary=str(a.binary.resolve()),binary_sha256=digest(a.binary),source_sha256=digest(a.source),source=str(a.source.resolve()),cases_sha256=digest(PACKAGE/'artifacts/cases.json'),metrics_sha256=digest(PACKAGE/'artifacts/metrics.json'),collector_sha256=digest(PACKAGE/'collect.py'),series_sha256=digest(Path(__file__)),case=a.case,ofes=a.ofes,physical=False,timeout_s=180,order=jobs,policy_selection='development diagnostic extension only; no complete-band finalist is qualified',failure='retain all failures; no retry; incomplete pairs yield no aggregate speedup',measurement='runner_wall_s; process CPU reported separately; taskset CPU0; serial fresh processes')
    (a.directory/'protocol.json').write_text(json.dumps(protocol,indent=2)+'\n')
    frozen_files={str(path.resolve()):digest(path) for path in [a.binary,a.source,PACKAGE/'artifacts/cases.json',PACKAGE/'artifacts/metrics.json',PACKAGE/'collect.py',Path(__file__)]}
    samples=[]
    for job in jobs:
        if any(digest(Path(path))!=expected for path,expected in frozen_files.items()):raise ValueError('frozen source/input/collector changed within series')
        r=collect(a.binary,a.directory/job['name'],job['policy'],a.case,ofes=a.ofes,physical=False)
        obs_path=a.directory/job['name']/'observations.json'
        try: obs=json.loads(obs_path.read_text()) if obs_path.exists() else {}
        except (ValueError,OSError): obs={}
        if not isinstance(obs,dict): obs={}
        if obs.get('physical') or obs.get('physical_ledgers'):raise ValueError('minimal observer retained physical data')
        wall=r.get('runner_wall_s')
        if not isinstance(wall,(int,float)) or not math.isfinite(wall) or wall<=0:
            r['execution_valid']=False
            r['series_timing_invalid']=True
        samples.append(dict(job=job,receipt=r))
        print(json.dumps(dict(name=job['name'],valid=r['execution_valid'],runner_wall_s=r.get('runner_wall_s'))),flush=True)
    summary=dict(protocol_sha256=digest(a.directory/'protocol.json'),sample_count=len(samples),failure_count=sum(not x['receipt']['execution_valid'] for x in samples),warmup_failure_count=sum(not x['receipt']['execution_valid'] for x in samples if x['job']['kind']=='warmup'),candidates={})
    for candidate in a.candidate:
        pairs=[]
        for pair in range(6):
            matched={x['job']['policy']:x['receipt'] for x in samples if x['job'].get('candidate')==candidate and x['job'].get('pair')==pair}
            r,c=matched['R0'],matched[candidate];valid=r['execution_valid'] and c['execution_valid']
            pairs.append(dict(pair=pair,valid=valid,reference_s=r.get('runner_wall_s'),candidate_s=c.get('runner_wall_s'),difference_s=c['runner_wall_s']-r['runner_wall_s'] if valid else None,ratio_reference_over_candidate=r['runner_wall_s']/c['runner_wall_s'] if valid else None))
        item=dict(pairs=pairs,all_pairs_valid=all(v['valid'] for v in pairs))
        if item['all_pairs_valid']:
            ratios=[v['ratio_reference_over_candidate'] for v in pairs];logs=list(map(math.log,ratios));delta=[v['difference_s'] for v in pairs]
            m=statistics.mean(logs);half=2.570582*statistics.stdev(logs)/math.sqrt(6)
            item.update(geometric_mean_ratio=math.exp(m),descriptive_paired_log_t95_interval=[math.exp(m-half),math.exp(m+half)],ratio_range=[min(ratios),max(ratios)],mean_difference_s=statistics.mean(delta),difference_range_s=[min(delta),max(delta)],uncertainty='descriptive paired t interval, n=6; not a distribution-free guarantee')
        summary['candidates'][candidate]=item
    (a.directory/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')

if __name__=='__main__':main()
