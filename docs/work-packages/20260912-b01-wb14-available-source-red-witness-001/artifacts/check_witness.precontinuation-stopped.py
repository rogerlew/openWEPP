#!/usr/bin/env python3
"""Streaming, fail-closed checker for the sole B01 cadence-refusal witness."""
import argparse, hashlib, json
from pathlib import Path
import ijson
TEST='hillslope::tests::stage3_snow_accuracy_case'
ERROR='SURFACELIQUID-E-008 IngressCandidate: WB14 day or interval continuation mismatch'
def sha(p):
 h=hashlib.sha256()
 with open(p,'rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()
def n(x,k):
 if type(x) is int and 0<=x<2**128:return x
 if type(x) is str and x.isdecimal() and int(x)<2**128:return int(x)
 raise ValueError(k)
def need(x,k):
 if not isinstance(x,dict) or k not in x:raise ValueError('missing '+k)
 return x[k]
def raw(x,k):
 x=need(x,k)
 if set(x)!={'Ok'} or not isinstance(x['Ok'],list) or any(type(b)!=int or not 0<=b<256 for b in x['Ok']):raise ValueError(k)
 b=bytes(x['Ok']);return b,json.loads(b)
def a32(x,k):
 x=need(x,k)
 if not isinstance(x,list) or len(x)!=32 or any(type(b)!=int or not 0<=b<256 for b in x):raise ValueError(k)
def state(x,k):
 cs=need(x,'continuations')
 if len(cs)!=1 or n(need(cs[0],'day_index'),k+'.day')!=0 or n(need(cs[0],'next_interval_index'),k+'.interval')!=0:raise ValueError(k)
 for q in ('cumulative_supply_m','cumulative_infiltration_m'):need(cs[0],q)
def check(obs,receipt,run,binary,listing):
 meta={};rows=[]
 with open(obs,'rb') as f:
  for p,e,v in ijson.parse(f):
   if p in ('schema','physical_enabled','overflow','counts_poisoned') and e in ('string','boolean'):meta[p]=v
   if p=='counts' and e=='start_map':meta['counts']=True
 with open(obs,'rb') as f:
  for i,r in enumerate(ijson.items(f,'physical.item')):
   if r.get('kind') in ('surface_liquid_wb14_cadence_failure','surface_liquid_wb14_cadence_caller_failure'):rows.append((i,r))
 if meta!={'schema':'snow_accuracy_observation_v2','physical_enabled':True,'overflow':False,'counts_poisoned':False,'counts':True} or len(rows)!=2:raise ValueError('observation')
 gi,g=rows[0];ci,c=rows[1]
 if (g.get('kind'),c.get('kind'),ci-gi)!=( 'surface_liquid_wb14_cadence_failure','surface_liquid_wb14_cadence_caller_failure',1):raise ValueError('pair')
 if any(g['capture'][k]!=c['capture'][k] for k in ('process','session','thread')) or c['capture']['ordinal']!=g['capture']['ordinal']+1:raise ValueError('capture')
 gb,i=raw(g,'input_typed_bytes');cb,j=raw(c,'input_typed_bytes')
 if gb!=cb or [n(need(i,k),k) for k in ('day_index','interval_index','transaction_id')]!=[4,22,255] or need(i,'interval_s')!=60.0:raise ValueError('input')
 if c.get('error')!=ERROR or [g.get(k) for k in ('initial','expected_day','expected_interval','accepted_parent_local_projection')]!=[True,0,0,False]:raise ValueError('guard')
 if [g.get(k) for k in ('parent_child_mode','finalize_parent_interval')]!=[True,False] or [c.get(k) for k in ('parent_child_mode','finalize_parent_interval')]!=[True,False]:raise ValueError('posture')
 bb,beg=raw(g,'beginning_typed_bytes');bb2,beg2=raw(c,'beginning_typed_bytes')
 if bb!=bb2:raise ValueError('beginning')
 state(beg,'beginning');state(beg2,'caller beginning')
 _,parent=raw(c,'parent_working_typed_bytes');_,working=raw(c,'working_typed_bytes')
 if n(need(parent,'accepted_until_ns'),'accepted')!=385920000000000:raise ValueError('accepted')
 state(need(parent,'persistent_beginning_state'),'persistent');state(need(parent,'candidate_state'),'candidate');state(working,'working')
 auth=next(iter(need(parent,'per_ofe_authorities').values()));w=need(auth,'working')
 if n(need(w,'next_child_ordinal'),'ordinal')!=0 or need(auth,'receipts')!=[]:raise ValueError('ordinal')
 prefix=need(auth,'inactive_prefix')
 for k in ('coupled_parent_transaction_sha256','parent_beginning_owner_sha256','prefix_ending_owner_sha256','coupled_receipt_sha256','proof_sha256'):a32(prefix,k)
 b=need(c,'coupled_binding')
 for k in ('coupled_parent_transaction_sha256','accepted_slab_sha256','parent_beginning_complete_owner_set_sha256'):a32(b,k)
 if [n(need(b,k),k) for k in ('parent_support_start_ns','parent_support_end_ns','child_support_start_ns','child_support_end_ns')]!=[385200000000000,387000000000000,385920000000000,385980000000000]:raise ValueError('support')
 if n(need(prefix,'parent_support_start_ns'),'prefix start')!=n(need(b,'parent_support_start_ns'),'binding start') or n(need(prefix,'prefix_end_ns'),'prefix end')!=n(need(b,'child_support_start_ns'),'child start') or n(need(prefix,'parent_support_end_ns'),'prefix end')!=n(need(b,'parent_support_end_ns'),'binding end') or prefix['coupled_parent_transaction_sha256']!=b['coupled_parent_transaction_sha256'] or prefix['parent_beginning_owner_sha256']!=b['parent_beginning_complete_owner_set_sha256']:raise ValueError('prefix join')
 R=json.load(open(receipt));Run=json.load(open(run))
 if R.get('timeout') or R.get('infrastructure_error') or R.get('exit_code',0)==0 or R.get('execution_valid') is not False or R.get('runner_execution')!='FAIL' or R.get('binary_unchanged') is not True or R.get('runner_receipt_sha256')!=sha(run) or Run.get('execution')!='FAIL' or R.get('binary_sha256')!=sha(binary):raise ValueError('receipt')
 av=R.get('argv',[])
 if len(av)!=9 or av[0:2]!=['taskset','-c'] or not str(av[2]).isdecimal() or av[3]!=str(binary) or av[4:]!=[TEST,'--ignored','--exact','--nocapture','--test-threads=1'] or listing.read_text().splitlines()!=[TEST+': test','','1 test, 0 benchmarks']:raise ValueError('selection')
 if R.get('artifact_sha256',{}).get('observations.json')!=sha(obs):raise ValueError('observation receipt hash')
 return {'witness_verdict':'PASS','runner_verdict':'FAIL','observation_sha256':sha(obs),'failure_row':gi,'caller_row':ci,'guard_record':g,'caller_record':c}
if __name__=='__main__':
 p=argparse.ArgumentParser();[p.add_argument(x,type=Path) for x in ('observation','receipt','run','binary','listing')];p.add_argument('--output',type=Path);x=p.parse_args();r=check(x.observation,x.receipt,x.run,x.binary,x.listing);x.output.write_text(json.dumps(r,indent=2)+'\n') if x.output else print(json.dumps(r))
