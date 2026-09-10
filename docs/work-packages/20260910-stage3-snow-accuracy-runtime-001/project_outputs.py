#!/usr/bin/env python3
"""Read admitted physical output files, preserving actual spatial/time bases."""
import argparse
import hashlib
import json
import math
from pathlib import Path
import pyarrow.parquet as pq
from analyze import number


def daily_projection(wat, passed, days, ofes, area):
    if not isinstance(days,int) or days<=0 or not isinstance(ofes,int) or ofes<=0:
        raise ValueError('invalid daily topology')
    area=number(area)
    if area<=0:raise ValueError('nonpositive contributing area')
    expected={(day,lane) for day in range(1,days+1) for lane in range(1,ofes+1)}
    keyed={}
    areas={}
    for row in wat:
        key=(row['sim_day_index'],row['ofe_id'])
        if key in keyed or key not in expected or row['OFE']!=key[1]:
            raise ValueError('duplicate, foreign or inconsistent WAT identity')
        keyed[key]=row
        local=number(row['Area'])
        if local<=0 or areas.get(key[1],local)!=local:
            raise ValueError('invalid or changing OFE area')
        areas[key[1]]=local
    if set(keyed)!=expected:raise ValueError('incomplete daily OFE coverage')
    if abs(math.fsum(areas.values())-area)>8*math.ulp(area):
        raise ValueError('water contributing-area mismatch')
    outlet_rows={}
    for row in passed:
        day=row['sim_day_index']
        if day in outlet_rows:raise ValueError('duplicate routed output day')
        outlet_rows[day]=row
    if set(outlet_rows)!=set(range(1,days+1)):raise ValueError('daily routed output coverage')
    daily=[]
    cumulative_outlet=0.0
    for day in range(1,days+1):
        p=outlet_rows[day];outlet=number(p['runvol'])*1000/area
        cumulative_outlet+=outlet
        daily.append({'day':day-1,'lane':'hillslope','area_m2':area,'values':{
            'daily_routed_outlet_runoff_mm':outlet,
            'cumulative_routed_outlet_runoff_mm':cumulative_outlet,
            'daily_routed_outlet_hourly_maximum_mm_h':number(p['peakro'])*3_600_000/area}})
        for lane in range(1,ofes+1):
            w=keyed[day,lane]
            daily.append({'day':day-1,'lane':lane,'area_m2':areas[lane],'values':{
                'published_cumulative_length_normalized_Q_mm':number(w['Q']),
                'frost_depth_m':number(w['frdp'])/1000,
                'published_swe_mm':number(w['Snow-Water']),
                'profile_soil_water_mm':number(w['SoilWaterTotal'])}})
    return daily


def validate_generation_rows(rows, days, ofes):
    expected={(day,lane) for day in range(1,days+1) for lane in range(1,ofes+1)}
    for row in rows:
        if (row['sim_day_index'],row['ofe_id']) not in expected:
            raise ValueError('foreign WAT5 day/OFE')
        start=number(row['interval_start_s']);duration=number(row['interval_duration_s'])
        if start<0 or duration<=0 or start+duration>86400:
            raise ValueError('WAT5 interval outside day')


def add_final_front_depths(daily, snapshot, days, ofes):
    if snapshot['committed_day_count']!=days:raise ValueError('final front-depth chronology')
    lanes={}
    for row in snapshot['lanes']:
        lane=row['lane_id']
        if lane in lanes or lane not in range(1,ofes+1):raise ValueError('final front-depth lane identity')
        lanes[lane]=row
    if len(lanes)!=ofes:raise ValueError('final front-depth lane coverage')
    for row in daily:
        if row['day']==days-1 and row['lane']!='hillslope':
            for name in ['frost_depth_m','thaw_depth_m']:
                value=number(lanes[row['lane']][name])
                if value<0:raise ValueError('negative final front depth')
                row['values']['final_snapshot_'+name]=value


def project(directory):
    directory=Path(directory)
    receipt=json.loads((directory/'receipt.json').read_text())
    if not receipt['execution_valid']:
        return {'disposition':'EXECUTION_INVALID','case':receipt['case'],'policy':receipt['policy']}
    for name,expected in receipt['artifact_sha256'].items():
        if hashlib.sha256((directory/name).read_bytes()).hexdigest()!=expected:
            raise ValueError('changed admitted artifact '+name)
    outputs=json.loads((directory/'outputs.json').read_text())
    fixture=Path((directory/'fixture-path.txt').read_text().strip())
    for name,expected in outputs['files'].items():
        with (fixture/'output'/name).open('rb') as stream:
            if hashlib.file_digest(stream,'sha256').hexdigest()!=expected:
                raise ValueError('changed physical output '+name)
    def rows(suffix):
        candidates=[name for name in outputs['files'] if name.endswith(suffix)]
        if len(candidates)!=1:raise ValueError('ambiguous physical output '+suffix)
        return pq.read_table(fixture/'output'/candidates[0]).to_pylist()
    wat,wat5,passed=rows('.wat.parquet'),rows('.wat-subhourly.parquet'),rows('.pass.parquet')
    days=receipt['completed_days']
    daily=daily_projection(wat,passed,days,receipt['ofes'],
        outputs['manifest']['wb13_publication']['publication_area_m2'])
    run=json.loads((directory/'run.json').read_text())
    add_final_front_depths(daily,run['snapshot'],days,receipt['ofes'])
    validate_generation_rows(wat5,days,receipt['ofes'])
    series=[]
    coverage=[]
    selected_count=0
    for day in range(1,days+1):
        for lane in range(1,receipt['ofes']+1):
            selected=sorted((r for r in wat5 if r['sim_day_index']==day and r['ofe_id']==lane),key=lambda r:r['interval_start_s'])
            selected_count+=len(selected)
            cursor=0.0
            for r in selected:
                start=number(r['interval_start_s']);duration=number(r['interval_duration_s'])
                if start!=cursor or duration<=0:raise ValueError('WAT5 gap/overlap')
                cursor+=duration
                depth=number(r['closing_surface_generation_depth_mm'])
                rate=number(r['closing_surface_generation_intensity_mm_h'])
                if depth<0 or rate<0:raise ValueError('negative WAT5 generation')
                if abs(depth-rate*duration/3600)>1e-9:raise ValueError('WAT5 rate/depth identity')
                series.append({'day':day-1,'lane':lane,'start_s':(day-1)*86400+start,'duration_s':duration,
                    'generated_runoff_mm':depth,'generated_runoff_mm_h':rate})
            coverage.append({'day':day-1,'lane':lane,'observed_start_s':0.0,'observed_end_s':cursor,'full_day':cursor==86400})
    if selected_count!=len(wat5):raise ValueError('unprojected WAT5 rows')
    return {'schema':'snow_accuracy_outputs_v2','case':receipt['case'],'policy':receipt['policy'],
      'basis':'WAT is per OFE; Q is cumulative-length-normalized diagnostic; PASS is routed outlet over actual total area; WAT5 is local OFE generation',
      'provenance':{'process_receipt_sha256':hashlib.sha256((directory/'receipt.json').read_bytes()).hexdigest(),
        'analyzer_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        'number_parser_sha256':hashlib.sha256((Path(__file__).parent/'analyze.py').read_bytes()).hexdigest()},
      'daily':daily,'generation_series':series,'generation_coverage':coverage,'disposition':'PHYSICAL_OUTPUT_PROJECTION',
      'unresolved':['routed outlet peak timestamps/event hydrograph','thaw depth projected only at final committed snapshot; intermediate thaw trajectory unavailable',
        'generated-runoff timing is not routed-outlet timing','WAT5 omitted intervals are unobserved, not assumed zero']}


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('directory',type=Path);p.add_argument('output',type=Path);a=p.parse_args()
    a.output.write_text(json.dumps(project(a.directory),indent=2)+'\n')
