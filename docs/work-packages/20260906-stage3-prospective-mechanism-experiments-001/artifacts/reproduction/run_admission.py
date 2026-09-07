#!/usr/bin/env python3
"""One process per exclusive log, with explicit PID/exit and immutable binary."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time
from run_series import digest, TEST

def main():
    p = argparse.ArgumentParser()
    for key in ('binary','environment','log','cwd'):
        p.add_argument('--'+key,type=Path,required=True)
    p.add_argument('--ofes',type=int,choices=(1,10,19),default=1)
    p.add_argument('--memory',action='store_true')
    p.add_argument('--trace',type=Path)
    p.add_argument('--test',default=TEST)
    a = p.parse_args()
    env = json.loads(a.environment.read_text())['runtime_env']
    for key in tuple(env):
        if key.startswith('OPENWEPP_EXPERIMENT_'):
            del env[key]
    env.update(RUST_MIN_STACK='67108864', OPENWEPP_EXPERIMENT_OFES=str(a.ofes),
               OPENWEPP_EXPERIMENT_REPEATS='1',OPENWEPP_EXPERIMENT_MEMORY=str(int(a.memory)))
    if a.trace:
        a.trace.mkdir(exist_ok=False)
        env['OPENWEPP_EXPERIMENT_TRACE_DIR']=str(a.trace.resolve())
    binary_hash=digest(a.binary)
    command=['taskset','-c','0',str(a.binary.resolve()),a.test,'--ignored','--exact','--nocapture','--test-threads=1']
    start=time.monotonic_ns()
    with a.log.open('xb') as log:
        process=subprocess.Popen(command,cwd=a.cwd,env=env,stdout=log,stderr=subprocess.STDOUT)
        pid,wait_status,usage=os.wait4(process.pid,0)
        process.returncode=os.waitstatus_to_exitcode(wait_status)
    result=dict(command=command,pid=pid,exit_code=process.returncode,
                duration_ns=time.monotonic_ns()-start,binary_sha256=binary_hash,
                binary_unchanged=digest(a.binary)==binary_hash,log_sha256=digest(a.log),
                lifetime_peak_rss_kib=usage.ru_maxrss,process_user_s=usage.ru_utime,
                process_system_s=usage.ru_stime,environment_sha256=digest(a.environment),
                ofes=a.ofes,memory=a.memory,trace=str(a.trace))
    with Path(str(a.log)+'.json').open('x') as out:
        json.dump(result,out,indent=2)
    print(json.dumps(result))
    if process.returncode or not result['binary_unchanged']:
        raise SystemExit(1)

if __name__=='__main__':
    main()
