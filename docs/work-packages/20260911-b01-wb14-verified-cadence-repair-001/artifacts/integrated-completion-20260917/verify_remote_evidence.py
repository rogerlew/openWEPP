"""Fetch explicitly committed evidence bytes from the public exact-commit endpoint."""
import concurrent.futures,datetime,hashlib,json,subprocess,sys,urllib.request
from pathlib import Path
commit,parent,destination=sys.argv[1:]
root=Path('/workdir/openWEPP')
paths=subprocess.check_output(['git','diff-tree','--no-commit-id','--name-only','-r',parent,commit],cwd=root,text=True).splitlines()
assert paths and all(p.startswith('docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/') for p in paths)
def verify(path):
    expected=subprocess.check_output(['git','show',commit+':'+path],cwd=root)
    url='https://raw.githubusercontent.com/rogerlew/openWEPP/'+commit+'/'+path
    request=urllib.request.Request(url,headers={'User-Agent':'openWEPP-evidence-verification'})
    with urllib.request.urlopen(request,timeout=30) as response:
        actual=response.read(len(expected)+1)
    if actual != expected:raise ValueError('remote byte mismatch: '+path)
    return dict(path=path,bytes=len(actual),sha256=hashlib.sha256(actual).hexdigest())
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:results=list(pool.map(verify,paths))
receipt=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commit=commit,parent=parent,method='independent exact-commit HTTPS raw fetch; byte-for-byte comparison to git show',files=results,all_match=True,scope='Published package/evidence/patch files only; external source trees, frozen binaries and large corpus remain local custody.')
Path(destination).write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(dict(commit=commit,files=len(results),bytes=sum(x['bytes'] for x in results),all_match=True)))
