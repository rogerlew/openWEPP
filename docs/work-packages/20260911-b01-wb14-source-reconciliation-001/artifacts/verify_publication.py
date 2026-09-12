from pathlib import Path
import concurrent.futures, hashlib, json, subprocess, sys, urllib.parse, urllib.request
repo=Path('/workdir/openWEPP'); commit=sys.argv[1]
paths=subprocess.check_output(['git','diff-tree','--no-commit-id','--name-only','-r',commit],cwd=repo,text=True).splitlines()
def verify(path):
    url='https://raw.githubusercontent.com/rogerlew/openWEPP/'+commit+'/'+urllib.parse.quote(path,safe='/')
    local=subprocess.check_output(['git','show',commit+':'+path],cwd=repo)
    row={'path':path,'url':url,'local_sha256':hashlib.sha256(local).hexdigest(),'local_bytes':len(local)}
    try:
        with urllib.request.urlopen(url,timeout=60) as response:
            remote=response.read()
        row.update({'remote_sha256':hashlib.sha256(remote).hexdigest(),'remote_bytes':len(remote),'equal':remote==local,'lfs_pointer':remote.startswith(b'version https://git-lfs.github.com/spec/v1')})
    except Exception as error:
        row.update({'equal':False,'error':str(error)})
    return row
with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
    rows=list(pool.map(verify,paths))
result={'cwd':str(repo),'argv':[str(repo/'.venv/bin/python'),str(Path(__file__).resolve()),commit],'commit':commit,'method':'Actual GitHub raw-content retrieval compared with git show COMMIT:path bytes for every file in commit diff','files':rows,'all_equal':all(x['equal'] and not x.get('lfs_pointer',False) for x in rows),'scope':'New scoped commit bytes only; not full historical source/LFS retrieval'}
output=Path('/workdir/openwepp-experiments/b01-wb14-source-reconciliation')/('remote-evidence-'+commit+'.json');output.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'files':len(rows),'all_equal':result['all_equal'],'failed':[x['path'] for x in rows if not x['equal']],'output':str(output)}))
raise SystemExit(0 if result['all_equal'] else 1)
