"""Source-shaped checker controls from streamed historical records; no Rust run."""
import copy,hashlib,json,tempfile
from pathlib import Path
from check_witness import check,TEST
BASE=Path(__file__).with_name('historical-correction145-warm-tail7-pair.json')
def h(p):return hashlib.sha256(p.read_bytes()).hexdigest()
with tempfile.TemporaryDirectory() as q:
 d=Path(q);b=d/'binary';b.write_bytes(b'frozen');l=d/'listing';l.write_text(TEST+': test\n\n1 test, 0 benchmarks\n')
 x=json.loads(BASE.read_text())['records'];g=copy.deepcopy(x[0]['row']);c=copy.deepcopy(x[1]['row']);g['capture']['ordinal']=0;c['capture']['ordinal']=1
 o=d/'observations.json';r=d/'run.json';r.write_text('{"execution":"FAIL"}')
 def receipt():return {'timeout':False,'exit_code':1,'execution_valid':False,'runner_execution':'FAIL','binary_unchanged':True,'runner_receipt_sha256':h(r),'binary_sha256':h(b),'argv':['taskset','-c','0',str(b),TEST,'--ignored','--exact','--nocapture','--test-threads=1'],'artifact_sha256':{'observations.json':h(o)}}
 def run(rows,mut=None,mut_receipt=None):
  v={'schema':'snow_accuracy_observation_v2','physical_enabled':True,'overflow':False,'counts_poisoned':False,'counts':{},'physical':rows};o.write_text(json.dumps(v));z=receipt();
  if mut:mut(z)
  if mut_receipt:mut_receipt(z)
  rp=d/'receipt.json';rp.write_text(json.dumps(z));return check(o,rp,r,b,l)
 assert run([g,c])['witness_verdict']=='PASS'
 cases=[('missing',lambda a:[a[0]]),('observer',lambda a:(a.__setitem__(0,dict(a[0],capture=dict(a[0]['capture'],process=9))),a)[1]),('boundary',lambda a:(a[0]['input_typed_bytes']['Ok'].__setitem__(0,124),a)[1]),('panic',lambda a:(a[1].__setitem__('error','panic'),a)[1]),('raw-pair',lambda a:(a[1]['input_typed_bytes']['Ok'].__setitem__(1,123),a)[1]),('ordinal',lambda a:(a[1]['capture'].__setitem__('ordinal',9),a)[1])]
 for name,f in cases:
  a=copy.deepcopy([g,c])
  try:run(f(a));raise AssertionError(name)
  except (ValueError,json.JSONDecodeError):pass
 for name,change in [('wrong-binary',lambda z:z.update(binary_sha256='0'*64)),('timeout',lambda z:z.update(timeout=True)),('zero-exit',lambda z:z.update(exit_code=0)),('infrastructure',lambda z:z.update(infrastructure_error='x')),('wrong-argv',lambda z:z.update(argv=[])),('hash-join',lambda z:z.update(runner_receipt_sha256='0'*64))]:
  try:run(copy.deepcopy([g,c]),mut_receipt=change);raise AssertionError(name)
  except ValueError:pass
 print('PASS source-shaped positive and twelve targeted negatives')
