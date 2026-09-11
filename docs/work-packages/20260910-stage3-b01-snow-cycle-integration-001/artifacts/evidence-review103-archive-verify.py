from pathlib import Path
import json,hashlib,tarfile,shutil,subprocess
b=Path('/tmp/openwepp-b01-cycle-20260910');a=Path('/workdir/openWEPP/docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts');root=b/'evidence-review103-archive-replay';root.mkdir(exist_ok=False)
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
result={'evidence_class':'Ran: archive index/hash verification and explicit-parent source replay, not modeling evidence','archives':[],'edges':[]}
source='source-restart001-009-soil001-004-steps101-103'
for stem in [source,'raw-correction100-101-and-restart-soil-drafts']:
 idx=json.loads((a/(stem+'-index.json')).read_text());p=a/idx['archive'];assert sha(p)==idx['sha256'];expected={v['path']:v for v in idx['files']};assert len(expected)==len(idx['files']);seen=set()
 with tarfile.open(p,'r|gz') as t:
  for m in t:
   assert m.isfile() and m.name in expected and m.name not in seen;mexp=expected[m.name];seen.add(m.name);assert m.size==mexp['bytes'];assert hashlib.file_digest(t.extractfile(m),'sha256').hexdigest()==mexp['sha256'],m.name
 assert seen==set(expected);result['archives'].append({'archive':idx['archive'],'bytes':p.stat().st_size,'sha256':idx['sha256'],'members':len(seen),'status':'PASS'});print('archive',stem,len(seen),flush=True)
records={};patches={}
with tarfile.open(a/(source+'.tar.gz')) as t:
 for m in t:
  if m.name.endswith('-source.json'):
   r=json.load(t.extractfile(m));records[r['cut']]=r;patches[r['cut']]=t.extractfile(m.name.replace('-source.json','.patch')).read()
prior={}
with tarfile.open(a/'source-steps096-100.tar.gz') as t:
 for m in t:
  if m.name.endswith('-source.json'):
   r=json.load(t.extractfile(m));prior[r['cut']]=r
for cut,r in records.items():
 parent=r['parent'];base=records.get(parent,prior.get(parent));assert base is not None,(cut,parent);out=root/cut;out.mkdir()
 for name,h in base['files'].items():
  p=b/parent/name;assert sha(p)==h,(cut,'parent',name);q=out/name;q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,q)
 patch=patches[cut];assert hashlib.sha256(patch).hexdigest()==r['patch_sha256'];proc=subprocess.run(['git','apply','--whitespace=nowarn','-'],cwd=out,input=patch,capture_output=True);assert proc.returncode==0,(cut,proc.stderr.decode())
 for name,h in r['files'].items():assert sha(out/name)==h==sha(b/cut/name),(cut,'ending',name)
 actual={str(p.relative_to(out)) for p in out.rglob('*') if p.is_file()};assert actual==set(r['files']),(cut,'unmanifested',actual-set(r['files']))
 result['edges'].append({'cut':cut,'parent':parent,'parent_files':len(base['files']),'ending_files':len(r['files']),'patch_sha256':r['patch_sha256'],'status':'PASS'});print('edge',parent,'->',cut,len(r['files']),flush=True)
result['status']='PASS';(b/'evidence-review103-archive-results.json').write_text(json.dumps(result,indent=2))
