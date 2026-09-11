from pathlib import Path
import tarfile,json,hashlib,difflib
r=Path('/tmp/openwepp-b01-cycle-20260910');out=Path('/workdir/openWEPP/docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts')
p=out/'source145-to146-test-only.patch'
a=r/'working145';b=r/'working146';diff=[]
for f in sorted(b.glob('crates/**/*.rs')):
 rel=f.relative_to(b);old=(a/rel).read_text();new=f.read_text()
 if old!=new:diff.extend(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+str(rel),tofile='b/'+str(rel)))
p.write_text(''.join(diff))
names=['correction145-warm-checkpoint','correction145-cold-checkpoint','correction145-warm-tail7','checkpoint145-cold.json']
for pattern in ['build145*','prepare14*.py','test14*.log','check143.log','model-review14*','evidence-review14*','collect-before-archive-ack.py','update-collector-ack*.py','retired-spool136.json','warm145-*.json','amend-mode-authority.py']:
 names.extend(p.name for p in r.glob(pattern) if p.is_file())
paths=[r/n for n in sorted(set(names)) if (r/n).exists()]
for p in list(paths):
 if p.is_dir() and (p/'fixture-path.txt').exists():paths.append(Path((p/'fixture-path.txt').read_text().strip()))
archive=out/'raw-stop-point145.tar.gz';assert not archive.exists();idx=[]
with tarfile.open(archive,'w:gz',compresslevel=3) as t:
 for p in paths:
  for f in ([p] if p.is_file() else sorted(p.rglob('*'))):
   if not f.is_file() or f.name in ('physical-rows.jsonpart','operands.sqlite'):continue
   arc=str(f.relative_to(r)) if f.is_relative_to(r) else 'fixtures/'+str(f.relative_to('/tmp'))
   t.add(f,arcname=arc,recursive=False);idx.append({'path':arc,'bytes':f.stat().st_size,'sha256':hashlib.file_digest(f.open('rb'),'sha256').hexdigest()})
(out/'raw-stop-point145-index.json').write_text(json.dumps({'archive':archive.name,'bytes':archive.stat().st_size,'sha256':hashlib.file_digest(archive.open('rb'),'sha256').hexdigest(),'files':idx},indent=2)+'\n')
print(archive.stat().st_size,len(idx),flush=True)
