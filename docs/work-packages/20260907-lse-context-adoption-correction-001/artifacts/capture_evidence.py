"""Detached source/log identities and diagnostic comparisons for this package only."""
from pathlib import Path
from collections import Counter
import hashlib
import json
import re
import subprocess
ROOT=Path(__file__).resolve().parents[4]
HERE=Path(__file__).resolve().parent
BASE=subprocess.check_output(['git','rev-parse','fb32d27f2'],cwd=ROOT,text=True).strip()
def sha(raw):return hashlib.sha256(raw).hexdigest()
def original(path):return subprocess.check_output(['git','show',BASE+':'+str(path)],cwd=ROOT)
manifest=json.loads((HERE/'candidate-manifest.json').read_text())
manifest['docs/specifications/science-contracts/index.md']=''
(HERE/'candidate-manifest.json').write_text(json.dumps({p:sha((ROOT/p).read_bytes()) for p in sorted(manifest)},indent=2)+'\n')
inputs={'Cargo.toml','Cargo.lock','flake.nix','flake.lock','clippy.toml','AGENTS.md','tests/AGENTS.md','docs/work-packages/AGENTS.md','docs/specifications/science-contracts/AGENTS.md','tools/sc_contract_directory.py','tools/check_sc_binding_exposure.py','tools/release/check_sc_unit_compliance.py','tools/release/check_science_contract_admission.sh'}
for file in manifest:
    if file.endswith('.rs') and '/support/' not in file:
        text=original(file).decode()
        inputs.update(re.findall(r'"((?:crates|docs|tests|tools|\.config)/[^"\n]+)"',text))
inputs.update(str(p.relative_to(ROOT)) for p in (ROOT/'docs/specifications/science-contracts/contracts').glob('SC-*.md'))
rows=[]
for path in sorted(inputs):
    file=ROOT/path
    if not file.is_file():continue
    before=original(path)
    rows.append({'path':path,'baseline_sha256':sha(before),'current_sha256':sha(file.read_bytes()),'unchanged':before==file.read_bytes(),'disposition':'migration authority/date' if path in ['docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md','docs/specifications/science-contracts/index.md'] else 'unchanged input'})
assert all(r['unchanged'] or r['disposition']=='migration authority/date' for r in rows)
# Every tracked production/tool/compiler input outside declared authority/test changes
# is also protected at tree level, not only by literal-read extraction.
subprocess.run(['git','diff','--exit-code',BASE,'--','crates','tools','.config','Cargo.toml','Cargo.lock','flake.nix','flake.lock'],cwd=ROOT,check=True,stdout=subprocess.DEVNULL)
(HERE/'unchanged-inputs.json').write_text(json.dumps({'baseline':BASE,'scope':'literal input paths of all seven original consumer targets plus all legacy SC entries and compiler/tool/instruction inputs; production/tools/config tree diff is empty','inputs':rows},indent=2)+'\n')
old=ROOT/'docs/work-packages/20260907-directory-contract-checker-lse-adoption-001/artifacts/logs/rust-baseline-comparison.log'
new=HERE/'logs/rust-final.log'
def failures(path):
    text=path.read_text()
    names=set(re.findall(r'FAIL \[.*?\]\s*\(\s*\d+/\d+\)\s+(\S+) (\S+)',text))
    panics=[]
    for m in re.finditer(r"thread '[^']+' \([^\n]+\) panicked at [^\n]+:\n(.*?)(?:\n\s*note:|\n\n)",text,re.S):
        panics.append(re.sub(r'0x[0-9a-f]+','0xADDR',m[1]).strip())
    return sorted(names),Counter(panics)
a,b=failures(old),failures(new)
assert a==b,(a[0],b[0],a[1]-b[1],b[1]-a[1])
(HERE/'logs/rust-comparison.json').write_text(json.dumps({'baseline_log':str(old.relative_to(ROOT)),'baseline_log_sha256':sha(old.read_bytes()),'candidate_log_sha256':sha(new.read_bytes()),'same_failure_identities_and_normalized_panics':True,'failures':a[0],'panic_payloads':dict(a[1]),'claim':'diagnostic equivalence only; masked obligations independently passing in focused tests'},indent=2)+'\n')
logs={str(p.relative_to(ROOT)):sha(p.read_bytes()) for p in sorted((HERE/'logs').glob('*')) if p.is_file()}
(HERE/'evidence-manifest.json').write_text(json.dumps({'source_manifest_sha256':sha((HERE/'candidate-manifest.json').read_bytes()),'unchanged_inputs_sha256':sha((HERE/'unchanged-inputs.json').read_bytes()),'logs':logs,'bridge_binary_sha256':sha(Path('/tmp/openwepp-lse-correction-bridge').read_bytes()),'excludes':'detached manifests, handoff, review/disposition reports; no future commit prediction'},indent=2)+'\n')
print(len(rows),'bound inputs;',len(a[0]),'identical inherited failure identities;',sum(a[1].values()),'emitted panic payloads')
