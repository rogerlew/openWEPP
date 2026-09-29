"""Preserve and verify the explicitly frozen completion source; no validation policy."""
import datetime as dt,difflib,hashlib,importlib.util,json,pathlib,subprocess,tarfile
p=pathlib.Path('/workdir/openWEPP/docs/work-packages/20260920-cold-canopy-m1-001/artifacts')
spec=importlib.util.spec_from_file_location('rec',p/'run_recorded.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
entries=r.SNAPSHOT.snapshot(r.SOURCE)[0];base=r.SNAPSHOT.snapshot(r.BASE)[0]
assert hashlib.sha256(json.dumps(base,sort_keys=True).encode()).hexdigest()=='85b8314efcd53ccb8111aa97af1f752ed49a99f46b805e754db5a1b2c7059963'
reconstructed=pathlib.Path('/home/roger/openwepp-experiments/cold-canopy-m1-bvls05-reconstruction09-20260929');reconstructed.mkdir(exist_ok=False)
for name in base:
 target=reconstructed/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes((r.BASE/name).read_bytes())
patch=[]
for name in sorted(set(entries)|set(base)):
 old=(r.BASE/name).read_text() if name in base else '';new=(r.SOURCE/name).read_text() if name in entries else ''
 if old!=new:patch.extend(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+name,tofile='b/'+name))
patch_path=p/'bvls05-terminal09-from-observer.patch'
with patch_path.open('x') as f:f.write(''.join(patch))
run=subprocess.run(['patch','-p1','--batch','--input',str(patch_path)],cwd=reconstructed,capture_output=True,text=True,timeout=60)
(p/'bvls05-reconstruction09.stdout').write_text(run.stdout);(p/'bvls05-reconstruction09.stderr').write_text(run.stderr)
assert run.returncode==0
actual=r.SNAPSHOT.snapshot(reconstructed)[0];assert actual==entries
archive=p/'bvls05-terminal09-source.tar.gz'
with archive.open('xb') as f:
 with tarfile.open(fileobj=f,mode='w:gz') as t:
  for name in sorted(entries):t.add(r.SOURCE/name,arcname=name,recursive=False)
with tarfile.open(archive,'r:gz') as t:
 assert sorted(t.getnames())==sorted(entries)
 for name,digest in entries.items():assert hashlib.sha256(t.extractfile(name).read()).hexdigest()==digest
assert r.SNAPSHOT.snapshot(r.SOURCE)[0]==entries
old=json.loads((p/'refinement-terminal-recovery.json').read_text())['unrelated_dirty_sha256']
assert all(r.sha(pathlib.Path('/workdir/openWEPP')/n)==v for n,v in old.items())
record=dict(evidence_class='Ran: fresh observer-entry copy plus exact BVLS05 confined source patch, all entry hashes and archive bytes verified; no numerical execution',recorded_utc=dt.datetime.now(dt.timezone.utc).isoformat(),source=str(r.SOURCE),source_tree_sha256=hashlib.sha256(json.dumps(entries,sort_keys=True).encode()).hexdigest(),entries=entries,entry_count=len(entries),observer_tree_sha256=hashlib.sha256(json.dumps(base,sort_keys=True).encode()).hexdigest(),terminal_patch=patch_path.name,terminal_patch_sha256=r.sha(patch_path),reconstruction=str(reconstructed),patch_exit_code=run.returncode,all_entry_hashes_equal=True,runtime_source_unchanged=True,archive=archive.name,archive_sha256=r.sha(archive),archive_all_bytes_equal=True,archive_scope='Exact detached source/config snapshot; command support manifests bind external build/input supports. Not a self-contained toolchain distribution.',unrelated_dirty_sha256=old)
with (p/'bvls05-terminal09-recovery.json').open('x') as f:json.dump(record,f,indent=2);f.write('\n')
print(json.dumps({k:v for k,v in record.items() if k!='entries'},indent=2))
