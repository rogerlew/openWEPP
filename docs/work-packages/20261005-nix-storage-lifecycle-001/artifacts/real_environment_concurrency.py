"""Run real Nix shells against isolated lifecycle state; preserve all host tools."""
from pathlib import Path
import json
import os
import shutil
import subprocess
import tempfile
import time

REPO = Path('/workdir/openWEPP')
OUT = REPO / 'docs/work-packages/20261005-nix-storage-lifecycle-001/artifacts/real-environment-concurrency.json'
root = Path(tempfile.mkdtemp(prefix='openwepp-nix-concurrency-', dir='/tmp'))
env = os.environ.copy()
env.update(OPENWEPP_NIX_CACHE_ROOT=str(root/'cache'), OPENWEPP_NIX_STATE_ROOT=str(root/'state'), OPENWEPP_CACHE_ROOT=str(root/'build'))
children = []
records = []


def prune(label):
    argv = [str(REPO/'tools/dev/nix-lifecycle'), 'prune', '--keep', '0', '--older-than-days', '0']
    run = subprocess.run(argv, env=env, capture_output=True, text=True, timeout=30)
    records.append(dict(label=label, argv=argv, exit=run.returncode, stdout=run.stdout, stderr=run.stderr))
    if run.returncode != 0:
        raise RuntimeError(f'prune failed: {label}')


try:
    for name in ['a','b']:
        child_env = env | {'OPENWEPP_TASK_ID': 'nix-storage-real-'+name}
        argv = [str(REPO/'tools/dev/develop'), 'bash', '-c', 'touch "$1"; while [ ! -e "$2" ]; do sleep 0.1; done', 'bash', str(root/(name+'.ready')), str(root/(name+'.release'))]
        log = (root/(name+'.log')).open('w')
        child = subprocess.Popen(argv, cwd=REPO, env=child_env, stdout=log, stderr=subprocess.STDOUT)
        children.append((name, child, log, argv))
    deadline = time.monotonic()+60
    while not all((root/(name+'.ready')).exists() for name in ['a','b']):
        if any(child.poll() is not None for _,child,_,_ in children) or time.monotonic()>deadline:
            raise RuntimeError('both real shells did not coexist')
        time.sleep(0.1)
    sources=[p for p in (root/'cache/sources').iterdir() if p.is_dir() and not p.name.startswith('.')]
    if len(sources)!=1:
        raise RuntimeError('two identical manifests did not share one staged identity')
    stage=sources[0]
    staged_bytes=sum(p.stat().st_size for p in stage.iterdir())
    if staged_bytes>=1024*1024:
        raise RuntimeError('staged input exceeds budget')
    prune('both active')
    if not stage.exists(): raise RuntimeError('prune removed active stage')
    (root/'a.release').touch()
    if children[0][1].wait(timeout=15)!=0: raise RuntimeError('first shell failed')
    prune('second still active')
    if not stage.exists(): raise RuntimeError('prune removed second active shell stage')
    (root/'b.release').touch()
    if children[1][1].wait(timeout=15)!=0: raise RuntimeError('second shell failed')
    profile=root/'state/profiles'/stage.name
    target=profile.resolve(strict=True)
    prune('both retired')
    if stage.exists() or profile.is_symlink(): raise RuntimeError('retired local source/profile not removed')
    if not target.exists(): raise RuntimeError('shared host environment unexpectedly removed')
    records.append({'result':'PASS','same_identity':stage.name,'staged_source_bytes':staged_bytes,'remaining_shared_store_output':str(target)})
finally:
    for name,child,log,argv in children:
        (root/(name+'.release')).touch()
        try: child.wait(timeout=15)
        except subprocess.TimeoutExpired:
            child.terminate(); child.wait(timeout=15)
        log.close()
        records.append({'child':name,'argv':argv,'exit':child.returncode,'output':(root/(name+'.log')).read_text()})
    OUT.write_text(json.dumps(records,indent=2)+'\n')
    shutil.rmtree(root)
print('PASS: two real same-toolchain shells coexist; prune waits for both; scoped retirement respects shared Nix roots.')
