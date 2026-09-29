from pathlib import Path
import hashlib,json,difflib,sys,importlib.util
A=Path('/workdir/openWEPP/docs/work-packages/20260920-cold-canopy-m1-001/artifacts')
spec=importlib.util.spec_from_file_location('r',A/'run_recorded.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
old=json.loads((A/'g4-rank-terminal-recovery.json').read_text());base=Path(old['reconstruction'])
assert r.SNAPSHOT.snapshot(base)[0]==old['entries']
entries=r.SNAPSHOT.snapshot(r.SOURCE)[0];changed=[n for n in sorted(set(entries)|set(old['entries'])) if entries.get(n)!=old['entries'].get(n)]
label=sys.argv[1];assert label.replace('-','').replace('_','').isalnum()
patch=[]
for n in changed:
 before=(base/n).read_text() if (base/n).exists() else ''
 after=(r.SOURCE/n).read_text() if (r.SOURCE/n).exists() else ''
 patch.extend(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/'+n,tofile='b/'+n))
p=A/(label+'-from-start.patch')
with p.open('x') as f:f.write(''.join(patch))
out={'evidence_class':'Static: exact source bytes compared to verified BVLS05 starting reconstruction','base':str(base),'source':str(r.SOURCE),'source_tree_sha256':hashlib.sha256(json.dumps(entries,sort_keys=True).encode()).hexdigest(),'entry_count':len(entries),'changed_files':[{'path':n,'before':old['entries'].get(n),'after':entries.get(n)} for n in changed],'patch':p.name,'patch_sha256':r.sha(p),'entries':entries}
with (A/(label+'-source-diff.json')).open('x') as f:json.dump(out,f,indent=2);f.write('\n')
print(json.dumps({k:v for k,v in out.items() if k!='entries'},indent=2))
