import hashlib,json,shutil,sys
from pathlib import Path
A=Path(__file__).resolve().parent
name,build=sys.argv[1:]
receipt=json.loads((A/(build+'.json')).read_text());assert receipt['exit_code']==0 and receipt['source_unchanged']
source=Path('/tmp/openwepp-b01-wb14-final-build/debug/deps/openwepp_hillslope_orchestrator-6acf9a74daf335ab')
binary=Path('/workdir/openwepp-experiments/b01-wb14-cadence/event-aware-restoration-binaries-20260916')/(name+'.frozen')
assert not binary.exists();shutil.copy2(source,binary)
with binary.open('rb') as f:digest=hashlib.file_digest(f,'sha256').hexdigest()
(A/(name+'-binary.json')).write_text(json.dumps(dict(build=build+'.json',binary=str(binary),sha256=digest,source_tree_sha256=receipt['source_tree_sha256']),indent=2)+'\n')
print(digest)
