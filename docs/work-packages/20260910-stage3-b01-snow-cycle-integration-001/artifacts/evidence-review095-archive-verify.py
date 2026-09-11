from pathlib import Path
import hashlib,json,tarfile,shutil,subprocess
r=Path('/tmp/openwepp-b01-cycle-20260910');a=Path('/workdir/openWEPP/docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts');out=r/'evidence-review095-archive-replay-complete';out.mkdir(exist_ok=False)
def digest(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
results={'evidence_class':'Ran: independent archive/index membership and SHA256; source patches replayed in new scratch only. Custody verification, not model validation.','archives':[],'replay':[]}
def verify(name):
 idx=json.loads((a/name).read_text());p=a/idx['archive'];assert digest(p)==idx['sha256'],p
 expected={v['path']:v for v in idx['files']};assert len(expected)==len(idx['files']);seen=set()
 with tarfile.open(p,'r|gz') as t:
  for m in t:
   assert m.isfile() and m.name in expected and m.name not in seen,m.name
   seen.add(m.name);v=expected[m.name];assert m.size==v['bytes'],m.name
   f=t.extractfile(m);assert hashlib.file_digest(f,'sha256').hexdigest()==v['sha256'],m.name
 assert seen==set(expected)
 results['archives'].append({'index':name,'archive_sha256':idx['sha256'],'members':len(seen),'status':'PASS'});print('ARCHIVE',name,len(seen),flush=True);return idx
verify('raw-correction087-through094-index.json')
with tarfile.open(a/'source-steps075-087.tar.gz') as t:base=json.load(t.extractfile('step087-source.json'))
for name,sha in base['files'].items():
 p=r/'working087'/name;assert digest(p)==sha,name;q=out/name;q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,q)
with tarfile.open(a/'source-steps088-088.tar.gz') as t: expanded=json.load(t.extractfile('step088-source.json'))
extra={}
for name in expanded['files']:
 if name not in base['files']:
  p=r/'working087'/name
  if p.exists():
   extra[name]=digest(p);q=out/name;q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,q)
results['supplemental_retained_parent_base']={'reason':'step088 manifest expands scope from689 to899 files; prior step087 archive lacks tests/**/*.rs','working_cut':'working087','files':extra,'status':'separately hashed retained-parent source, not previously archived step087 manifest'}
base['files'].update(extra)
for group,ns in [('088-088',[88]),('089-090',[89,90]),('091-091',[91]),('092-092',[92]),('093-095',[93,94,95])]:
 idx=verify('source-steps'+group+'-index.json')
 with tarfile.open(a/idx['archive']) as t:
  for n in ns:
   patch=t.extractfile(f'step{n:03}.patch').read();rec=json.load(t.extractfile(f'step{n:03}-source.json'));assert rec['parent']==f'working{n-1:03}';assert hashlib.sha256(patch).hexdigest()==rec['patch_sha256']
   # prior replay must match retained immutable parent before applying next patch
   for name in base['files']:assert digest(out/name)==digest(r/f'working{n-1:03}'/name),(n,'parent',name)
   q=subprocess.run(['git','apply','--whitespace=nowarn','-'],cwd=out,input=patch,capture_output=True);assert q.returncode==0,q.stderr.decode()
   for name,sha in rec['files'].items():assert digest(out/name)==sha==digest(r/f'working{n:03}'/name),(n,name)
   results['replay'].append({'cut':n,'parent':n-1,'files_verified':len(rec['files']),'patch_sha256':rec['patch_sha256'],'status':'PASS'});base=rec;print('REPLAY',n,len(rec['files']),flush=True)
results['status']='PASS';(r/'evidence-review095-archive-results.json').write_text(json.dumps(results,indent=2)+'\n')
