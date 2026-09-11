import json, hashlib, collections, argparse
from pathlib import Path
from decimal import Decimal
import ijson

parser=argparse.ArgumentParser(description="Committed snow timeline and separately labeled provisional diagnostics; no whole-system closure claim")
parser.add_argument('run',type=Path)
parser.add_argument('output',type=Path)
args=parser.parse_args()
run=args.run
if args.output.exists(): raise ValueError('output exists')
def norm(x):
    if isinstance(x,Decimal): return float(x)
    if isinstance(x,dict): return {k:norm(v) for k,v in x.items()}
    if isinstance(x,list): return [norm(v) for v in x]
    return x
def inventories(snow):
    out=[]
    for lane,s in snow['lanes']:
        layers=s['layers']
        out.append(dict(lane=lane, ice_kg_m2=sum(v['mass_swe_m']*1000 for v in layers),
            liquid_kg_m2=sum(v['liquid_water_m']*1000 for v in layers)+s.get('detached_retained_liquid_kg_m2',0),
            cold_content_j_m2=sum(v['cold_content_j_m2'] for v in layers),
            layer_count=len(layers), temperatures_c=[v['temperature_c'] for v in layers],
            experimental_accuracy=s.get('experimental_accuracy'),
            cumulative={k:v for k,v in s.items() if k.startswith('cumulative_')}))
    return out
fields=['beginning_ice_kg_m2','beginning_liquid_kg_m2','ending_ice_kg_m2','ending_liquid_kg_m2','ending_cold_content_j_m2','melt_kg_m2','refreeze_kg_m2','terminal_liquid_kg_m2','terminal_liquid_sensible_enthalpy_j_m2','solid_precipitation_kg_m2','liquid_precipitation_kg_m2','sublimation_kg_m2','deposition_kg_m2','sensible_j_m2','latent_j_m2']
records={};days=[];admitted=set(); kinds=collections.Counter(); last_inner=None; inner_timeline=[]; latest={}; regime_counts=collections.Counter(); minimum_inner_ice=float('inf')
with (run/'observations.json').open('rb') as f:
    for raw in ijson.items(f,'physical.item'):
        k=raw['kind'];kinds[k]+=1
        if k=='candidate_subslab_primitives':
            r=norm(raw); key=r['receipt_sha256']
            slim={q:r[q] for q in ['receipt_sha256','accepted_slab_sha256','owner_join','support','day_index','physical_outcome_ledger_set_sha256']}
            slim['lanes']={lane:{q:l[q] for q in fields} for lane,l in r['physical_outcome_ledgers'].items()}
            slim['disposition']=r['disposition']
            if key in records and records[key]!=slim:raise ValueError('conflicting candidate')
            records[key]=slim; latest[r['day_index']]=slim
        elif k=='inner_accepted_state':
            r=norm(raw); snow=json.loads(r['owners_canonical_json']['snow'])
            last_inner={'time_ns':r['time_ns'],'parent_transaction_id':r['parent_transaction_id'],'commit_posture':r['commit_posture'],'clock_owner_set_sha256':r['clock_owner_set_sha256'],'snow':inventories(snow)}
            for inventory in last_inner['snow']:
                regime_counts[str(inventory['experimental_accuracy'])]+=1
                minimum_inner_ice=min(minimum_inner_ice,inventory['ice_kg_m2'])
            if not inner_timeline or int(r['time_ns'])//3600000000000!=int(inner_timeline[-1]['time_ns'])//3600000000000:inner_timeline.append(last_inner)
        elif k=='accepted_day':
            r=norm(raw); selected=[]
            for identity in r['accepted_subslab_identities']:
                key=identity['receipt_sha256']; c=records[key]
                if key in admitted:raise ValueError('duplicate admission')
                for q in identity:
                    if q in c and c[q]!=identity[q]:raise ValueError('identity mismatch '+q)
                if c['day_index']!=r['day_index']:raise ValueError('day mismatch')
                admitted.add(key);selected.append(c)
            selected.sort(key=lambda c:int(c['support']['start_ns']))
            covered_support_discontinuities=[{'previous':a['support'],'next':b['support']} for a,b in zip(selected,selected[1:]) if a['support']['end_ns']!=b['support']['start_ns']]
            totals={};events=[]
            for c in selected:
                for lane,l in c['lanes'].items():
                    t=totals.setdefault(lane,{'supports':0,'duration_s':0,'sums':{},'peak_ice_kg_m2':0,'minimum_ending_ice_kg_m2':float('inf')})
                    t['supports']+=1;t['duration_s']+=(int(c['support']['end_ns'])-int(c['support']['start_ns']))/1e9
                    for q in fields[5:]:t['sums'][q]=t['sums'].get(q,0)+l[q]
                    t['peak_ice_kg_m2']=max(t['peak_ice_kg_m2'],l['beginning_ice_kg_m2'],l['ending_ice_kg_m2'])
                    t['minimum_ending_ice_kg_m2']=min(t['minimum_ending_ice_kg_m2'],l['ending_ice_kg_m2'])
                    if l['terminal_liquid_kg_m2']>0:events.append({'lane':lane,'support':c['support'],'receipt_sha256':c['receipt_sha256'],**l})
            wire_totals={}; positive_wire=[]
            for entry in r['transfers']:
                wire=entry['accepted_wire']
                for v in wire['accepted_snow_liquid_outputs']:
                    t=wire_totals.setdefault(str(v['lane_id']),{'mass_kg_m2':0,'sensible_enthalpy_j_m2':0,'positive_supports':0})
                    t['mass_kg_m2']+=v['mass_kg_m2_ofe_ground'];t['sensible_enthalpy_j_m2']+=v['sensible_enthalpy_j_m2_ofe_ground']
                    if v['mass_kg_m2_ofe_ground']>0:t['positive_supports']+=1;positive_wire.append(v)
            days.append({'day_index':r['day_index'],'covered_support_discontinuities':covered_support_discontinuities,'snow':inventories(r['snow']),'totals':totals,'accepted_wire_snow_outputs':wire_totals,'first_positive_wire':positive_wire[:1],'last_positive_wire':positive_wire[-1:],'first_positive_ledger':events[:1],'last_positive_ledger':events[-1:],'native_support_custody_count':len(r['native_support_custody']),'native_support_custody_example_keys':list(r['native_support_custody'][0]) if r['native_support_custody'] else [],'terminal_receipts_count':len(r['terminal_receipts']),'selected_receipts':[c['receipt_sha256'] for c in selected]})
            print('committed day',r['day_index'],totals,flush=True)
receipt=json.loads((run/'receipt.json').read_text())
result={'schema':'committed_snow_trajectory_v1','qualification':'COMMITTED_PREFIX_AND_PROVISIONAL_TRAJECTORY_ONLY_NOT_CLOSURE','source':str(run),'source_observations_sha256':hashlib.file_digest((run/'observations.json').open('rb'),'sha256').hexdigest(),'run_execution_valid':receipt['execution_valid'],'row_kinds':dict(kinds),'all_inner_regime_counts':dict(regime_counts),'minimum_provisional_inner_ice_kg_m2':minimum_inner_ice,'committed_days':days,'last_inner_provisional':last_inner,'hourly_inner_provisional':inner_timeline,'latest_candidate_by_day':latest,'unselected_candidate_count':len(records)-len(admitted),'limitations':['Final uncommitted day is excluded from committed sums.','Inner records are provisional and hourly samples may contain superseded trials.','Snow donor ledger/wire sums are not independent recipient or whole-system closure.','Regime observations must not infer committed bulk execution solely from snow mass.']}
assert result['source_observations_sha256']==receipt['artifact_sha256']['observations.json']
result['last_covered_candidate_matches_last_inner'] = bool(latest and last_inner and latest[max(latest)]['owner_join']['ending_complete_owner_set_sha256']==last_inner['clock_owner_set_sha256'])
args.output.write_text(json.dumps(result,indent=2,allow_nan=False)+'\n')
print('saved trajectory',[(d['day_index'],d['snow']) for d in days], 'last inner',last_inner,flush=True)
