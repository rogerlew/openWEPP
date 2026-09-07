"""Run the compiled actual Rust bridge on isolated positive/negative fixtures."""
from pathlib import Path
import json
import subprocess
import sys
import tempfile
ROOT=Path(__file__).resolve().parents[4]
sys.path.insert(0,str(ROOT/'tests/python'))
from test_sc_contract_directory import valid, mutate
BINARY='/tmp/openwepp-lse-correction-bridge'
results=[]
def run(entry,success,expected=None):
    p=subprocess.run([BINARY,str(entry)],capture_output=True)
    assert (p.returncode==0)==success,(entry,p.stderr)
    if expected is not None:assert p.stdout==expected
    results.append({'case':str(entry),'exit':p.returncode,'expected_success':success})
entry=ROOT/'docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md'
sys.path.insert(0,str(ROOT/'tools'))
from sc_contract_directory import ContractSet
run(entry,True,ContractSet(entry).normative_text().encode())
for p in entry.parent.glob('SC-*.md'):
    if p!=entry:run(p,True,p.read_bytes())
for case in ['valid','missing-definition','unknown-format','malformed-format']:
    with tempfile.TemporaryDirectory(prefix='lse-bridge-') as directory:
        fixture=valid.__wrapped__(Path(directory))
        if case=='missing-definition':mutate(fixture,'surface','<a id="INV-X-001"></a> `INV-X-001`','`INV-X-001`')
        if case=='unknown-format':mutate(fixture,None,'directory-v1','directory-v2')
        if case=='malformed-format':mutate(fixture,None,'contract_format: directory-v1','contract_format directory-v1')
        run(fixture,case=='valid')
(Path(__file__).parent/'logs/bridge-results.json').write_text(json.dumps(results,indent=2)+'\n')
print(f'{len(results)} actual bridge cases passed; {len(results)-5} unchanged legacy entries byte-identical')
