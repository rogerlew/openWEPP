from pathlib import Path
import hashlib,json,shutil
r=Path('/tmp/openwepp-b01-cycle-20260910');w=r/'working145';old=json.loads((r/'build136-execution-identity.json').read_text())
names=set(old['files'])|{str(p.relative_to(w)) for p in (w/'crates').glob('**/*.rs')}
files={n:hashlib.sha256((w/n).read_bytes()).hexdigest() for n in sorted(names)}
canonical=lambda x:json.dumps(x,sort_keys=True,separators=(',',':')).encode()
source=hashlib.sha256(canonical(files)).hexdigest();bi=dict(old['build_inputs']);bi['source_sha256']=source
ident={'source_sha256':source,'build_inputs_sha256':hashlib.sha256(canonical(bi)).hexdigest(),'build_inputs':bi,'files':files}
(r/'build145-execution-identity.json').write_text(json.dumps(ident,indent=2)+'\n')
(r/'build145.py').write_text((r/'build136.py').read_text().replace('build136','build145').replace('working136','working145').replace('target136-release','target145-release'))
assert not (r/'target145-release').exists();shutil.move(r/'target136-release',r/'target145-release')
print(len(files),source)
