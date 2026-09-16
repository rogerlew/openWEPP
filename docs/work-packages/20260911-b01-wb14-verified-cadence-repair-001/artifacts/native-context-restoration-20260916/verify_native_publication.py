from pathlib import Path
import subprocess,hashlib,json,datetime,urllib.request,urllib.parse,concurrent.futures
R=Path('/workdir/openWEPP');A=R/'docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/native-context-restoration-20260916'
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()
paths=subprocess.check_output(['git','diff-tree','--no-commit-id','--name-only','-r',head],cwd=R,text=True).splitlines()
def verify(p):
 url='https://raw.githubusercontent.com/rogerlew/openWEPP/'+head+'/'+urllib.parse.quote(p,safe='/')
 with urllib.request.urlopen(url,timeout=25) as f:data=f.read()
 local=(R/p).read_bytes();h=hashlib.sha256(data).hexdigest()
 return {'path':p,'bytes':len(data),'sha256':h,'match':data==local}
start=datetime.datetime.now(datetime.timezone.utc)
with concurrent.futures.ThreadPoolExecutor(max_workers=12) as pool:rows=list(pool.map(verify,paths))
r={'evidence_commit':head,'start_utc':start.isoformat(),'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'method':'Independent HTTPS GET of every changed file at exact GitHub commit; compare response bytes to retained local bytes','all_match':all(x['match'] for x in rows),'entries':rows}
(A/'remote-byte-verification.json').write_text(json.dumps(r,indent=2)+'\n');print(head,len(rows),r['all_match'])
assert r['all_match']
