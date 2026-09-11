from pathlib import Path
import tarfile,json,hashlib
root=Path(__file__).parent
idx=json.loads((root/'raw-stop-point145-index.json').read_text());archive=root/idx['archive']
assert hashlib.file_digest(archive.open('rb'),'sha256').hexdigest()==idx['sha256']
expected={v['path']:v for v in idx['files']};seen=set()
with tarfile.open(archive,'r|gz') as t:
 for member in t:
  assert member.isfile() and member.name in expected and member.name not in seen
  value=expected[member.name];assert member.size==value['bytes']
  assert hashlib.file_digest(t.extractfile(member),'sha256').hexdigest()==value['sha256'];seen.add(member.name)
assert seen==expected.keys()
(root/'stop-point-verification.json').write_text(json.dumps({'archive':idx['archive'],'sha256':idx['sha256'],'bytes':archive.stat().st_size,'members_verified':len(seen),'status':'PASS','scope':'executor byte preservation; not independent scientific verification'},indent=2)+'\n')
print('PASS',len(seen),archive.stat().st_size)
