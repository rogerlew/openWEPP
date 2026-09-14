"""Single authorized launch metadata; unchanged collector owns collection."""
from pathlib import Path
import datetime,json,os,subprocess
p=Path(__file__).resolve().parent
r=json.loads((p/'raw-launch.json').read_text())
assert r['state']=='READY'
r['start_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
r['wrapper_pid']=os.getpid()
r['state']='LAUNCHING'
(p/'raw-launch.json').write_text(json.dumps(r,indent=2)+'\n')
env=os.environ.copy();env.update(r['environment_overrides'])
with (p/'collector.stdout').open('xb') as out,(p/'collector.stderr').open('xb') as err:
    child=subprocess.Popen(r['argv'],cwd=r['cwd'],env=env,stdout=out,stderr=err,start_new_session=True)
    r['collector_pid']=child.pid;r['state']='RUNNING';r['capture_runs']=1
    (p/'raw-launch.json').write_text(json.dumps(r,indent=2)+'\n')
    r['collector_exit']=child.wait()
r['state']='RETURNED';r['end_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
(p/'raw-launch.json').write_text(json.dumps(r,indent=2)+'\n')
