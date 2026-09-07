#!/usr/bin/env python3
"""Execute one frozen local gate; do not retry or overwrite output."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time
from run_series import digest

def main():
    p=argparse.ArgumentParser()
    p.add_argument('--cwd',type=Path,required=True)
    p.add_argument('--log',type=Path,required=True)
    p.add_argument('command',nargs=argparse.REMAINDER)
    a=p.parse_args()
    command=a.command[1:] if a.command and a.command[0]=='--' else a.command
    if not command:
        p.error('command required')
    env=dict(os.environ,RUST_MIN_STACK='67108864',CARGO_NET_OFFLINE='true')
    start=time.monotonic_ns()
    checkout=subprocess.check_output(['git','-C',str(a.cwd),'rev-parse','HEAD'],text=True).strip()
    with a.log.open('xb') as output:
        process=subprocess.Popen(command,cwd=a.cwd,env=env,stdout=output,stderr=subprocess.STDOUT)
        print(json.dumps({'pid':process.pid,'command':command,'log':str(a.log),'stack':env['RUST_MIN_STACK']}),flush=True)
        code=process.wait()
    result={'pid':process.pid,'command':command,'cwd':str(a.cwd),'checkout':checkout,
            'exit_code':code,'duration_ns':time.monotonic_ns()-start,'log_sha256':digest(a.log),
            'environment_overrides':{'RUST_MIN_STACK':env['RUST_MIN_STACK'],'CARGO_NET_OFFLINE':'true'}}
    with Path(str(a.log)+'.json').open('x') as output:
        json.dump(result,output,indent=2)
    print(json.dumps(result),flush=True)
    raise SystemExit(code)

if __name__=='__main__':
    main()
