#!/usr/bin/env python3
"""Snapshot isolated experimental Rust changes against the retained R0 composition."""
import argparse
import difflib
import hashlib
import json
from pathlib import Path

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
    p=argparse.ArgumentParser();p.add_argument('reference',type=Path);p.add_argument('candidate',type=Path);p.add_argument('binary',type=Path);p.add_argument('prefix',type=Path);a=p.parse_args()
    rows=[];patch=[]
    paths=set(p.relative_to(a.reference) for p in (a.reference/'crates').rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
    paths|=set(p.relative_to(a.candidate) for p in (a.candidate/'crates').rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name=='Cargo.toml'))
    for rel in sorted(paths):
        r,c=a.reference/rel,a.candidate/rel
        before=r.read_text() if r.exists() else '';after=c.read_text() if c.exists() else ''
        if before==after:continue
        rows.append(dict(path=str(rel),sha256=sha(c) if c.exists() else None))
        patch.extend(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/'+str(rel) if r.exists() else '/dev/null',tofile='b/'+str(rel) if c.exists() else '/dev/null'))
    patch_path=a.prefix.with_suffix('.patch');patch_path.write_text(''.join(patch))
    a.prefix.with_name(a.prefix.name+'-source.json').write_text(json.dumps(dict(base='R0 retained composition; see R0-source.json and package.md',changed_files=rows,patch_sha256=sha(patch_path),runner_sha256=sha(a.binary),runner=str(a.binary.resolve()),build='cargo test --workspace --release --offline --no-run --message-format=json-render-diagnostics; CARGO_PROFILE_RELEASE_LTO=false; RUST_MIN_STACK=67108864; repository Nix shell'),indent=2)+'\n')
if __name__=='__main__':main()
