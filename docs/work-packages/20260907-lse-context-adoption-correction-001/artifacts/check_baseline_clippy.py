"""Isolated baseline root-test lint using unchanged current production dependencies."""
from pathlib import Path
import json
import re
import shutil
import subprocess
import tempfile
ROOT=Path(__file__).resolve().parents[4]
HERE=Path(__file__).resolve().parent
BASE='fb32d27f2'
def original(path):return subprocess.check_output(['git','show',BASE+':'+str(path)],cwd=ROOT)
files=[p for p in json.loads((HERE/'candidate-manifest.json').read_text()) if p.startswith('tests/integration/') and '/support/' not in p]
names={Path(p).stem for p in files}
folder=Path(tempfile.mkdtemp(prefix='lse-baseline-clippy-'))
manifest=original('Cargo.toml').decode()
manifest=re.sub(r'\[workspace\]\n.*?(?=\[workspace.package\])','[workspace]\nresolver = "2"\nmembers = []\n\n',manifest,flags=re.S)
parts=manifest.split('[[test]]')
manifest=parts[0]
for block in parts[1:]:
    m=re.search(r'^name = "([^"]+)"',block,re.M)
    if m and m[1] in names:manifest+='[[test]]'+block
manifest=re.sub(r'path = "(crates/[^"]+)"',lambda m:'path = "'+str(ROOT/m[1])+'"',manifest)
(folder/'Cargo.toml').write_text(manifest)
for p in files:
    dest=folder/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(original(p))
for p in ['Cargo.lock','clippy.toml']:
    if (ROOT/p).exists(): (folder/p).write_bytes(original(p))
# Workspace dependencies/lints are identical; only member enumeration and target
# membership/path roots differ. No production source, assertion or lint flag edited.
argv=['nix','develop','-c','cargo','clippy','--manifest-path',str(folder/'Cargo.toml'),'-p','openwepp','--no-deps']
for name in sorted(names):argv+=['--test',name]
argv+=['--','-D','warnings']
with (HERE/'logs/clippy-baseline-root.log').open('w') as log:
    result=subprocess.run(argv,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT)
(HERE/'logs/clippy-baseline-command.json').write_text(json.dumps({'baseline':BASE,'fixture':str(folder),'command':argv,'cwd':str(ROOT),'exit':result.returncode,'source_files':files,'scope':'original test source and lint settings; unchanged current production dependencies; isolated manifest target enumeration'},indent=2)+'\n')
print('baseline root Clippy exit',result.returncode,'fixture',folder)
# Retain actual Cargo-mutated fixture identities: a smaller workspace lock is not
# byte-identical to the source lock, even when common versions/checksums are fixed.
import hashlib
import tomllib
record=HERE/'logs/clippy-baseline-command.json'
data=json.loads(record.read_text())
old_lock=original('Cargo.lock');new_lock=(folder/'Cargo.lock').read_bytes()
def packages(raw):
    return {(r['name'],r['version'],r.get('source','local')):r for r in tomllib.loads(raw.decode())['package']}
a,b=packages(old_lock),packages(new_lock)
assert not b.keys()-a.keys()
assert all(a[k].get('checksum')==b[k].get('checksum') for k in a.keys()&b.keys())
data.update(actual_fixture_manifest_sha256=hashlib.sha256((folder/'Cargo.toml').read_bytes()).hexdigest(),actual_fixture_lock_sha256=hashlib.sha256(new_lock).hexdigest(),original_lock_sha256=hashlib.sha256(old_lock).hexdigest(),lock_disposition={'removed_unused_packages':sorted(a.keys()-b.keys()),'added_packages':[],'common_version_source_checksum_identity':True,'dependency_lists':'Cargo pruned dev/workspace-only dependencies; lock bytes NOT identical','claim':'bounded root-source lint only'})
record.write_text(json.dumps(data,indent=2)+'\n')
