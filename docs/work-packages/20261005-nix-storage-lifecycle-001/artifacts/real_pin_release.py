"""Real Nix pin/release and generation-root retirement integration."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

REPO=Path('/workdir/openWEPP')
OUT=REPO/'docs/work-packages/20261005-nix-storage-lifecycle-001/artifacts/real-pin-release-final.json'
root=Path(tempfile.mkdtemp(prefix='openwepp-real-pin-final-',dir='/tmp'))
env=os.environ|{'OPENWEPP_NIX_CACHE_ROOT':str(root/'cache'),'OPENWEPP_NIX_STATE_ROOT':str(root/'state'),'OPENWEPP_CACHE_ROOT':str(root/'build'),'OPENWEPP_TASK_ID':'nix-storage-real-pin-final'}
records=[]


def run(argv, expected=0):
    r=subprocess.run(argv,cwd=REPO,env=env,capture_output=True,text=True,timeout=60)
    records.append({'argv':argv,'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr})
    if (expected==0 and r.returncode!=0) or (expected!=0 and r.returncode==0):
        raise RuntimeError(f'unexpected exit {r.returncode}: {argv}')
    return r


try:
    if (REPO/'result').exists() or (REPO/'result').is_symlink():
        raise RuntimeError('checkout result exists before test; preserve it')
    run(['tools/dev/develop','true'])
    stage=next(p for p in (root/'cache/sources').iterdir() if p.is_dir())
    identity=stage.name
    profile=root/'state/profiles'/identity
    env_target=profile.resolve(strict=True)
    run(['tools/dev/nix-lifecycle','pin',identity,'qa-retention'])
    if (REPO/'result').exists() or (REPO/'result').is_symlink():
        raise RuntimeError('pin created an unwanted checkout result')
    pin=root/'state/pins/qa-retention'
    target=pin.resolve(strict=True)
    roots=run(['nix-store','--query','--roots',str(target)]).stdout
    if str(root/'state/pins/') not in roots:
        raise RuntimeError('pin not actually rooted')
    extra=root/'state/pins/qa-retention-1-extra-link';extra.symlink_to(target)
    tampered=root/'state/pins/qa-retention-999-link';tampered.symlink_to(env_target)
    original_generation=root/'state/pins/qa-retention-1-link'
    run(['tools/dev/nix-lifecycle','release','qa-retention'],expected=1)
    if not all(p.is_symlink() for p in [pin,original_generation,extra,tampered]) or not (root/'state/pins/qa-retention.identity').exists():
        raise RuntimeError('tampered generation caused partial release')
    tampered.unlink()
    run(['tools/dev/nix-lifecycle','prune','--keep','0','--older-than-days','0'])
    if not stage.exists() or not profile.is_symlink():
        raise RuntimeError('package pin did not retain normal environment')
    run(['tools/dev/nix-lifecycle','release','qa-retention'])
    if pin.is_symlink() or original_generation.is_symlink() or not extra.is_symlink():
        raise RuntimeError('release failed numeric-only generation semantics')
    roots=run(['nix-store','--query','--roots',str(target)]).stdout
    if str(root/'state/pins/') in roots:
        raise RuntimeError('package pin GC roots remain after release')
    profile_bad=Path(str(profile)+'-999-link');profile_bad.symlink_to(target)
    run(['tools/dev/nix-lifecycle','prune','--keep','0','--older-than-days','0'])
    if not profile.is_symlink() or not stage.exists() or not Path(str(profile)+'-1-link').is_symlink():
        raise RuntimeError('tampered profile generation caused partial prune')
    profile_bad.unlink()
    run(['tools/dev/nix-lifecycle','prune','--keep','0','--older-than-days','0'])
    if profile.is_symlink() or stage.exists() or Path(str(profile)+'-1-link').is_symlink():
        raise RuntimeError('prune did not retire generated profile root')
    roots=run(['nix-store','--query','--roots',str(env_target)]).stdout
    if str(root/'state/profiles/') in roots:
        raise RuntimeError('retired normal environment GC roots remain')
    records.append({'result':'PASS','identity':identity,'pin_root_created_then_released':True,'pinned_environment_retained':True,'numeric_generations_removed':True,'nonnumeric_sibling_preserved':True,'tampered_pin_and_profile_candidates_preserved':True,'pruned_profile_has_no_gc_roots':True,'checkout_result_not_created':True})
finally:
    OUT.write_text(json.dumps({'isolated_root':str(root),'records':records},indent=2)+'\n')
    shutil.rmtree(root)
print('PASS: real pin/release and prune remove owned generation roots; pins, ambiguous candidates, and nonnumeric siblings preserved appropriately.')
