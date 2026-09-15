"""Record an explicitly supplied command and source delta; assigns no acceptance."""
import datetime
import difflib
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
BASE = Path('/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913')


def snapshot(source):
    paths = sorted({str(p.relative_to(root)) for root in (source, BASE)
                    for p in (root/'crates').rglob('*') if p.is_file()} |
                   {'Cargo.toml','Cargo.lock','flake.nix','flake.lock','rust-toolchain.toml','clippy.toml'})
    identities={};changes=[];patch=[]
    for rel in paths:
        p=source/rel;b=BASE/rel
        data=p.read_bytes() if p.exists() else b''
        old=b.read_bytes() if b.exists() else b''
        identities[rel]=hashlib.sha256(data).hexdigest() if p.exists() else None
        if data!=old:
            changes.append({'path':rel,'base_sha256':hashlib.sha256(old).hexdigest() if b.exists() else None,'source_sha256':identities[rel]})
            patch.extend(difflib.unified_diff(old.decode().splitlines(True),data.decode().splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))
    return identities,changes,''.join(patch)


if __name__=='__main__':
    name,root,*argv=sys.argv[1:];source=Path(root)
    identities,changes,patch=snapshot(source)
    patch_hash=hashlib.sha256(patch.encode()).hexdigest()
    patch_name='source-'+patch_hash+'.patch'
    if not (HERE/patch_name).exists():(HERE/patch_name).write_text(patch)
    receipt={'argv':argv,'cwd':str(source),'source':str(source),'base':str(BASE),
             'source_tree_sha256':hashlib.sha256(json.dumps(identities,sort_keys=True).encode()).hexdigest(),
             'changed_files':changes,'patch':patch_name,'patch_sha256':patch_hash,
             'start_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
    receipt['execution_input_custody']='../reference-input-recovery-20260915/input-custody.json'
    receipt['execution_input_custody_sha256']=hashlib.sha256((HERE/'../reference-input-recovery-20260915/input-custody.json').read_bytes()).hexdigest()
    record=HERE/(name+'.json');record.write_text(json.dumps(receipt,indent=2)+'\n')
    deadline=datetime.datetime.fromisoformat('2026-09-15T15:33:16.994589+00:00')
    remaining=(deadline-datetime.datetime.now(datetime.timezone.utc)).total_seconds()-90
    if remaining<=0:raise SystemExit('execution budget reserved for preservation')
    receipt['timeout_seconds']=remaining
    receipt['executed_argv']=['/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(remaining)+'s',*argv]
    record.write_text(json.dumps(receipt,indent=2)+'\n')
    with (HERE/(name+'.stdout')).open('wb') as out,(HERE/(name+'.stderr')).open('wb') as err:
        result=subprocess.run(receipt['executed_argv'],cwd=source,stdout=out,stderr=err)
    receipt.update(end_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exit=result.returncode,
                   source_unchanged_during_command=snapshot(source)[0]==identities)
    record.write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps({k:receipt[k] for k in ['start_utc','end_utc','exit','source_unchanged_during_command','changed_files']}))
