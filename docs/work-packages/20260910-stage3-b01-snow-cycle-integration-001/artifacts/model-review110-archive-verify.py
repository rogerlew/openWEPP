from pathlib import Path
import json,hashlib,tarfile,shutil,subprocess
b=Path('/tmp/openwepp-b01-cycle-20260910');a=Path('/workdir/openWEPP/docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts');root=b/'model-review110-archive-replay';root.mkdir(exist_ok=False)
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
result={'evidence_class':'Ran: indexed member verification and explicit-parent patch replay only; no scientific workflow','archives':[],'edges':[],'live_log_comparison':{}}
source='source-steps104-109-soil005-009'
for stem in [source,'raw-correction102-105-and-selected-soil-drafts']:
 idx=json.loads((a/(stem+'-index.json')).read_text());p=a/idx['archive'];errors=[];actualsha=sha(p)
 if actualsha!=idx['sha256']:errors.append('archive SHA mismatch')
 expected={v['path']:v for v in idx['files']}
 if len(expected)!=len(idx['files']):errors.append('duplicate index names')
 seen=set();memberbytes=0
 with tarfile.open(p,'r|gz') as t:
  for m in t:
   if not m.isfile() or m.name not in expected or m.name in seen:
    errors.append('invalid/extra/duplicate member '+m.name);continue
   mexp=expected[m.name];seen.add(m.name);memberbytes+=m.size
   observed=hashlib.file_digest(t.extractfile(m),'sha256').hexdigest()
   if m.size!=mexp['bytes']:errors.append('member byte mismatch '+m.name)
   if observed!=mexp['sha256']:errors.append('member hash mismatch '+m.name)
   if m.name=='soil009-check.log':
    current=b/m.name
    result['live_log_comparison']={'member':m.name,'indexed_bytes':mexp['bytes'],'archive_member_bytes':m.size,'current_bytes':current.stat().st_size,'indexed_sha256':mexp['sha256'],'archive_member_sha256':observed,'current_sha256':sha(current),'current_tail':current.read_text()[-1200:]}
 if seen!=set(expected):errors.append('missing names '+str(sorted(set(expected)-seen)))
 item={'archive':idx['archive'],'bytes':p.stat().st_size,'indexed_sha256':idx['sha256'],'actual_sha256':actualsha,'members':len(seen),'uncompressed_member_bytes':memberbytes,'errors':errors,'status':'FAIL' if errors else 'PASS'};result['archives'].append(item);print('ARCHIVE',json.dumps(item),flush=True)
records={};patches={}
with tarfile.open(a/(source+'.tar.gz')) as t:
 for m in t:
  if m.name.endswith('-source.json'):
   r=json.load(t.extractfile(m));records[r['cut']]=r;patches[r['cut']]=t.extractfile(m.name.replace('-source.json','.patch')).read()
prior={}
with tarfile.open(a/'source-restart001-009-soil001-004-steps101-103.tar.gz') as t:
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
 expectedcount=901 if cut.startswith('soil') else 900;assert len(actual)==expectedcount,(cut,len(actual))
 item={'cut':cut,'parent':parent,'parent_files':len(base['files']),'ending_files':len(actual),'patch_sha256':r['patch_sha256'],'status':'PASS'};result['edges'].append(item);print('EDGE',json.dumps(item),flush=True)
log=result['live_log_comparison'];log['archive_matches_index']=log['archive_member_sha256']==log['indexed_sha256'] and log['archive_member_bytes']==log['indexed_bytes'];log['current_matches_archive']=log['archive_member_sha256']==log['current_sha256'] and log['archive_member_bytes']==log['current_bytes'];log['current_sha256_at_end']=sha(b/log['member']);log['stable_during_verification']=log['current_sha256_at_end']==log['current_sha256']
result['status']='PASS_ARCHIVE_AND_REPLAY_WITH_LIVE_LOG_DIFFERENCE' if all(x['status']=='PASS' for x in result['archives']) and not log['current_matches_archive'] else ('PASS' if all(x['status']=='PASS' for x in result['archives']) else 'FAIL')
(b/'model-review110-archive-results.json').write_text(json.dumps(result,indent=2)+'\n');print('LOG',json.dumps(log),flush=True);print('STATUS',result['status'],flush=True)
