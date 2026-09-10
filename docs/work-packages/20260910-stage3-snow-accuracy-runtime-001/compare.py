#!/usr/bin/env python3
"""Prospective physical-unit comparisons; missing metrics never pass admission."""
import argparse
import hashlib
import json
import math
from pathlib import Path
from analyze import number

BANDS=json.loads((Path(__file__).parent/'artifacts/metrics.json').read_text())['bands_tight_coarse_abs_relative']


def band_key(metric):
    if metric.startswith('soil_temperature_K_at_'):return 'soil_temperature_K'
    return {'cumulative_generated_melt_mm':'cumulative_melt_mm',
      'cumulative_sublimation_mm':'cumulative_sublimation_mm','cumulative_deposition_mm':'cumulative_deposition_mm',
      'daily_generated_runoff_mm':'daily_runoff_mm','daily_routed_outlet_runoff_mm':'daily_runoff_mm',
      'cumulative_generated_runoff_mm':'cumulative_runoff_mm','cumulative_routed_outlet_runoff_mm':'cumulative_runoff_mm',
      'final_snapshot_frost_depth_m':'frost_depth_m','final_snapshot_thaw_depth_m':'frost_depth_m',
      'published_swe_mm':'swe_mm'}.get(metric,metric if metric in BANDS else None)


def compare_value(metric,reference,candidate):
    reference,candidate=number(reference),number(candidate)
    delta=candidate-reference;key=band_key(metric)
    answer={'metric':metric,'reference':reference,'candidate':candidate,'signed_difference':delta,
      'absolute_difference':abs(delta),'relative_difference':None if reference==0 else delta/abs(reference),
      'band_metric':key,'band_status':'UNBANDED_DIAGNOSTIC'}
    if key:
        answer['allowances']={name:a+r*abs(reference) for name,(a,r) in zip(['tight','coarse'],BANDS[key],strict=True)}
        answer['passes_available_value']={name:abs(delta)<=allowance for name,allowance in answer['allowances'].items()}
        answer['band_status']='VALUE_COMPARISON_ONLY'
    return answer


def compare_observed_generation(reference, candidate):
    """Conservative 300s-bin projection over observed common support only."""
    coverage=lambda p:{(r['day'],r['lane']):r for r in p['generation_coverage']}
    rc,cc=coverage(reference),coverage(candidate)
    if len(rc)!=len(reference['generation_coverage']) or len(cc)!=len(candidate['generation_coverage']) or set(rc)!=set(cc):
        raise ValueError('generation coverage identity')
    rows=[]
    for identity in sorted(rc):
        day,lane=identity
        end=min(number(rc[identity]['observed_end_s']),number(cc[identity]['observed_end_s']))
        if rc[identity]['observed_start_s']!=0 or cc[identity]['observed_start_s']!=0:
            raise ValueError('generation comparison origin')
        bins=[]
        selected=[[r for r in p['generation_series'] if (r['day'],r['lane'])==identity] for p in [reference,candidate]]
        for start in range(0,int(end)//300*300,300):
            absolute=day*86400+start
            depths=[]
            for series in selected:
                overlaps=[(r,max(0.0,min(absolute+300,r['start_s']+r['duration_s'])-max(absolute,r['start_s']))) for r in series]
                if abs(math.fsum(overlap for _,overlap in overlaps)-300)>1e-9:
                    raise ValueError('unobserved or duplicated common generation bin')
                depths.append(math.fsum(number(r['generated_runoff_mm'])*overlap/number(r['duration_s']) for r,overlap in overlaps if overlap>0))
            bins.append(dict(start_s=absolute,duration_s=300,reference_depth_mm=depths[0],candidate_depth_mm=depths[1],
                signed_depth_difference_mm=depths[1]-depths[0],signed_rate_difference_mm_h=(depths[1]-depths[0])*12))
        rows.append(dict(day=day,lane=lane,common_observed_end_s=len(bins)*300,
            reference_observed_end_s=rc[identity]['observed_end_s'],candidate_observed_end_s=cc[identity]['observed_end_s'],
            full_common_day=len(bins)==288,bins=bins,
            cumulative_signed_difference_mm=math.fsum(r['signed_depth_difference_mm'] for r in bins),
            maximum_absolute_rate_difference_mm_h=max((abs(r['signed_rate_difference_mm_h']) for r in bins),default=None),
            reference_observed_grid_maximum_mm_h=max((r['reference_depth_mm']*12 for r in bins),default=None),
            candidate_observed_grid_maximum_mm_h=max((r['candidate_depth_mm']*12 for r in bins),default=None)))
    return dict(basis='local OFE generation averaged conservatively onto frozen 300s grid; no time warp or missing-tail zero fill',
        status='OBSERVED_WINDOW_DIAGNOSTIC_NOT_FULL_DAY_OR_ROUTED_EVENT_PEAK_BAND',rows=rows)


def work_comparison(reference, candidate):
    result={}
    for name,extract in [('scalar_carrier_requests',lambda run:run['carrier']['counts'][3]['completed']),
                         ('batch_physical_requests',lambda run:run['carrier']['counts'][4]['completed']),
                         ('shared_physical_maps',lambda run:run['lse']['maps']['completions'])]:
        r,c=extract(reference),extract(candidate)
        result[name]={'reference':r,'candidate':c,'reduction_fraction':None if r==0 else 1-c/r}
    result['basis']='matched topology; scalar entry requests, joint batch requests and shared-engine maps are separate counters, never summed; zero scalar counts do not imply absent physical work'
    return result


def compare(reference,candidate):
    reference,candidate=Path(reference),Path(candidate)
    read=lambda d,n:json.loads((d/n).read_text())
    rr,cr=read(reference,'receipt.json'),read(candidate,'receipt.json')
    if (rr['case'],rr['ofes'])!=(cr['case'],cr['ofes']):raise ValueError('different case/topology')
    row={'policy':cr['policy'],'case':cr['case'],'ofes':cr['ofes'],
      'reference_execution_valid':rr['execution_valid'],'candidate_execution_valid':cr['execution_valid'],
      'runner_wall_s':cr.get('runner_wall_s'),'process_user_s':cr.get('process_user_s'),
      'process_system_s':cr.get('process_system_s'),'peak_rss_kib':cr.get('lifetime_peak_rss_kib'),
      'qualified':False,'disposition':'EXPERIMENT_IN_PROGRESS','metrics':[],
      'unresolved':['melt-derived delivery','routed outlet event timing','complete-system independent energy/receiver proof',
        'common-time full physical trajectories','balanced runtime uncertainty','restart/integration/full correctness gates']}
    if not rr['execution_valid'] or not cr['execution_valid']:
        row['speedup']=None;row['disposition']='FAILED_OR_UNAVAILABLE_COMPARISON'
        row['candidate_failure']=read(candidate,'run.json').get('error') if (candidate/'run.json').exists() else 'runner receipt absent'
        row['reference_failure']=read(reference,'run.json').get('error') if (reference/'run.json').exists() else 'runner receipt absent'
        return row
    if read(reference,'inputs.json')['files']!=read(candidate,'inputs.json')['files']:
        raise ValueError('different admitted physical inputs')
    row['diagnostic_wall_ratio']=rr['runner_wall_s']/cr['runner_wall_s']
    row['timing_scope']='one diagnostic execution, not paired speedup estimate'
    for file,collection,key in [('physical.json','days',lambda r:(r['day'],r['lane'])),
                               ('output-physical.json','daily',lambda r:(r['day'],r['lane']))]:
        r=read(reference,file);c=read(candidate,file)
        analyzer=Path(__file__).parent/('analyze.py' if file=='physical.json' else 'project_outputs.py')
        for directory,projection in [(reference,r),(candidate,c)]:
            provenance=projection['provenance']
            if provenance['process_receipt_sha256']!=hashlib.sha256((directory/'receipt.json').read_bytes()).hexdigest() or provenance['analyzer_sha256']!=hashlib.sha256(analyzer.read_bytes()).hexdigest():
                raise ValueError('unbound or obsolete physical projection')
            if file=='output-physical.json' and provenance['number_parser_sha256']!=hashlib.sha256((Path(__file__).parent/'analyze.py').read_bytes()).hexdigest():
                raise ValueError('obsolete projection numeric parser')

        rrows={key(v):v['values'] for v in r[collection]};crows={key(v):v['values'] for v in c[collection]}
        if len(rrows)!=len(r[collection]) or len(crows)!=len(c[collection]):raise ValueError('duplicate physical output identity')
        if set(rrows)!=set(crows):raise ValueError('different complete output coverage')
        for identity,rv in rrows.items():
            cv=crows[identity]
            if set(rv)!=set(cv):raise ValueError('different physical metric coverage')
            for name,value in rv.items():
                metric=compare_value(name,value,cv[name]);metric['identity']=identity;row['metrics'].append(metric)
    row['observed_generation_comparison']=compare_observed_generation(
        read(reference,'output-physical.json'),read(candidate,'output-physical.json'))
    rrun,crun=read(reference,'run.json'),read(candidate,'run.json')
    row['physical_work_comparison']=work_comparison(rrun,crun)
    row['policy_counts']=read(candidate,'observations.json')['counts']
    row['partial_snow_balance']=read(candidate,'physical.json')['disposition']
    row['available_value_bands']={band:all(m['passes_available_value'][band] for m in row['metrics'] if 'passes_available_value' in m) for band in ['tight','coarse']}
    row['disposition']='PARTIAL_COMPARISON_NO_ADMISSION'
    return row


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('reference',type=Path);p.add_argument('candidate',type=Path);p.add_argument('output',type=Path);a=p.parse_args()
    a.output.write_text(json.dumps(compare(a.reference,a.candidate),indent=2)+'\n')
