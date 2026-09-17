"""One explicit unchanged-v6 Clippy visibility command; no acceptance policy."""
import datetime as dt
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import subprocess
import time

raw=Path(__file__).parent
source=Path('/home/roger/openwepp-experiments/b01-wb14-integrated-evidence-20260917/terminal-source-reconstruction')
spec=importlib.util.spec_from_file_location('recorder',raw/'run_recorded.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
before=module.SNAPSHOT.snapshot(source)[0]
digest=hashlib.sha256(json.dumps(before,sort_keys=True).encode()).hexdigest()
assert digest=='50672ab77e97334e731c99ccbc869ecdcc213da7257b86b141c5b1879104b6c5'
label='strict-clippy-baseline-v6-nodeps'
argv=['nix','develop','/workdir/openWEPP','--command','env','CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-six-owner-target','CARGO_BUILD_JOBS=2','cargo','clippy','--manifest-path',str(source/'Cargo.toml'),'-p','openwepp-hillslope-orchestrator','-p','openwepp-persisted-restart-v1','--all-targets','--no-deps','--features','persisted-restart-v1,restart-authority-evidence,fixtures','--locked','--message-format=json','--','-D','warnings']
r=dict(label=label,argv=argv,cwd=str(source),source_tree_sha256=digest,source_entries=before,start_utc=dt.datetime.now(dt.timezone.utc).isoformat(),timeout_seconds=240,address_space_bytes=16*1024**3,output_file_limit_bytes=1024**3,automatic_retries=0,purpose='Baseline strict no-deps diagnostic visibility; no workflow acceptance')
path=raw/(label+'.json');assert not path.exists();path.write_text(json.dumps(r,indent=2)+'\n')
start=time.monotonic()
with (raw/(label+'.stdout')).open('xb') as out,(raw/(label+'.stderr')).open('xb') as err:
 proc=subprocess.Popen(['/usr/bin/time','-v','-o',str(raw/(label+'.resources')),*argv],cwd=source,stdout=out,stderr=err,preexec_fn=module.limits,env={k:v for k,v in os.environ.items() if not k.startswith('OPENWEPP_')},start_new_session=True)
 try: code=proc.wait(timeout=240);timed_out=False
 except subprocess.TimeoutExpired:
  import signal
  os.killpg(proc.pid,signal.SIGTERM)
  try: proc.wait(timeout=5)
  except subprocess.TimeoutExpired: os.killpg(proc.pid,signal.SIGKILL);proc.wait()
  code=124;timed_out=True
r.update(exit_code=code,timed_out=timed_out,elapsed_seconds=time.monotonic()-start,end_utc=dt.datetime.now(dt.timezone.utc).isoformat(),source_unchanged=module.SNAPSHOT.snapshot(source)[0]==before,stdout_sha256=module.sha(raw/(label+'.stdout')),stderr_sha256=module.sha(raw/(label+'.stderr')))
path.write_text(json.dumps(r,indent=2)+'\n');print(json.dumps({k:r[k] for k in ('label','exit_code','elapsed_seconds','source_unchanged')}))
