"""Independent exact-rational reconstruction of retained credit operands; no solver."""
from fractions import Fraction as F
from pathlib import Path
import hashlib,json,struct
HERE=Path(__file__).resolve().parent
ROWS=Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1/rows')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def carry(x):
 c=F(int(x['coefficient_hex'],16)*x['sign']);e=x['exponent2'];return c*2**e if e>=0 else c/F(2**-e)
def bits(x):return struct.pack('>d',float(x)).hex()
phase=json.loads((ROWS/'123091.26.json').read_text());cont=phase['deferred_native_v2_soil_custody']['continuation'];groups=cont['ordered_layer_credit_chain'];raw=json.loads((HERE/'retained-soil-trial-primitives.json').read_text())
byreceipt={}
for x in raw['records']:
 r=x['record'];semantic={k:v for k,v in r.items() if k!='capture'};key=r['receipt']['receipt_sha256']
 if key in byreceipt:assert semantic==byreceipt[key]
 byreceipt[key]=semantic
results=[];previous={};last_end=None
for gi,g in enumerate(groups):
 top=[]
 for layer in g:
  key=(layer['ofe_id'],layer['layer_id']);begin=F(layer['beginning_enthalpy_hi_j_m2_ofe_ground'])+carry(layer['beginning_enthalpy_carry']);end=F(layer['ending_enthalpy_hi_j_m2_ofe_ground'])+carry(layer['ending_enthalpy_carry']);delta=sum((F(o['energy_j_m2_ofe_ground']) for o in layer['accepted_operands']),F())
  assert end==begin+delta,(gi,key,'energy')
  assert bits(float(end))==bits(layer['ending_enthalpy_hi_j_m2_ofe_ground']),(gi,key,'nearest-even high')
  if key in previous:assert previous[key]==(bits(layer['beginning_enthalpy_hi_j_m2_ofe_ground']),layer['beginning_enthalpy_carry'],bits(layer['beginning_temperature_k']))
  previous[key]=(bits(layer['ending_enthalpy_hi_j_m2_ofe_ground']),layer['ending_enthalpy_carry'],bits(layer['ending_temperature_k']))
  for o in layer['accepted_operands']:
   if o['source_kind']=='top_boundary':
    r=byreceipt[o['debit_credit_identity_sha256']];receipt=r['receipt'];assert o['ofe_id']==receipt['ofe_id'];assert bits(o['energy_j_m2_ofe_ground'])==bits(receipt['soil_heat_j_m2']);assert F(receipt['soil_heat_j_m2'])==-F(receipt['snow_heat_j_m2']);top.append(r)
 assert len(top)==1
 r=top[0];a=int(r['receipt']['support']['start_ns']);b=int(r['receipt']['support']['end_ns']);assert b>a
 if last_end is not None:assert last_end==a
 last_end=b
 results.append({'group':gi,'support':[a,b],'layers':len(g),'operand_count':sum(len(x['accepted_operands']) for x in g),'primitive_transaction':r['transaction_id'],'receipt':r['receipt']['receipt_sha256'],'exact_energy_and_carry':'PASS'})
event=phase['ordered_event_groups'][0]['candidates'][0];a=int(event['support']['start_ns']);b=int(event['support']['end_ns']);selected=[x for x in results if a<=x['support'][0] and x['support'][1]<=b];assert selected[0]['support'][0]==a and selected[-1]['support'][1]==b
ordered=sum((byreceipt[x['receipt']]['receipt']['snow_heat_j_m2'] for x in selected),0.0);eventheat=event['event']['snow_soil_heat_energy_j_m2'];assert abs(ordered-eventheat)<=1e-9
result={'evidence_class':'Ran: independent Python Fraction reconstruction from retained operands; no physical evolution','inputs':{str(p):sha(p)for p in [ROWS/'123091.26.json',HERE/'retained-soil-trial-primitives.json']},'groups':results,'terminal_child_groups':[x['group']for x in selected],'terminal_ordered_snow_heat':ordered,'terminal_event_snow_heat':eventheat,'regrouping_residual':ordered-eventheat,'limitations':'Energy/state continuity and support coverage only. Not canonical receipt/trial-seal validation, source/event capability, historical conservation, or admission.'}
(HERE/'independent-credit-energy.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'groups':len(results),'layers':sum(x['layers']for x in results),'terminal_groups':result['terminal_child_groups'],'regrouping_residual':ordered-eventheat}))
