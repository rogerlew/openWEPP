"""Check formatting only where the explicit final source differs from its frozen base."""
from pathlib import Path
from difflib import SequenceMatcher
import json, subprocess, tempfile, hashlib
HERE=Path(__file__).resolve().parent
SOURCE=Path('/home/roger/openwepp-experiments/b01-wb14-feature-compatibility-20260917')
BASE=Path('/home/roger/openwepp-experiments/b01-wb14-integrated-promotion-20260917')
receipt=json.loads((HERE/'completion-final-targeted-format-check-01-source.json').read_text())
results=[]
for rel, digest in receipt['entries'].items():
    if not rel.endswith('.rs'): continue
    candidate=SOURCE/rel; baseline=BASE/rel
    raw=candidate.read_bytes()
    assert hashlib.sha256(raw).hexdigest()==digest, rel
    if baseline.exists() and baseline.read_bytes()==raw: continue
    old=baseline.read_text().splitlines(True) if baseline.exists() else []
    current=raw.decode().splitlines(True)
    touched=[(j1,j2) for tag,i1,i2,j1,j2 in SequenceMatcher(None,old,current,autojunk=False).get_opcodes() if tag!='equal']
    with tempfile.TemporaryDirectory() as tmp:
        path=Path(tmp)/candidate.name;path.write_bytes(raw)
        subprocess.run(['rustfmt','--edition','2024','--config','skip_children=true',str(path)],check=True,capture_output=True)
        formatted=path.read_text().splitlines(True)
    pending=[]; inherited=0
    for tag,i1,i2,j1,j2 in SequenceMatcher(None,current,formatted,autojunk=False).get_opcodes():
        if tag=='equal': continue
        intersects=any(i1 < max(b,a+1) and max(i2,i1+1) > a for a,b in touched)
        if intersects:pending.append(dict(line_start=i1+1,line_end=i2,before=''.join(current[i1:i2]),after=''.join(formatted[j1:j2])))
        else:inherited+=1
    results.append(dict(path=rel,introduced_unformatted=pending,unrelated_formatter_hunks=inherited))
report=dict(source_sha256=receipt['tree_sha256'],policy='Every changed Rust file; formatter hunks intersecting actual baseline-to-candidate changed lines must be clean. Unchanged lines are not reformatted or claimed clean.',files=results)
(HERE/'completion-changed-format-result.json').write_text(json.dumps(report,indent=2)+'\n')
fail=[(r['path'],len(r['introduced_unformatted'])) for r in results if r['introduced_unformatted']]
print(json.dumps(dict(changed_rust_files=len(results),introduced_unformatted=fail)))
raise SystemExit(bool(fail))
