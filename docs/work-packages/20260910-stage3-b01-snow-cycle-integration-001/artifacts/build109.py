from pathlib import Path
import subprocess,json,os
r=Path('/tmp/openwepp-b01-cycle-20260910');ident=json.loads((r/'build109-execution-identity.json').read_text())
cmd=['nix','develop','/workdir/openWEPP','--command','env','OPENWEPP_B01_SOURCE_SHA256='+ident['source_sha256'],'OPENWEPP_B01_BUILD_INPUTS_SHA256='+ident['build_inputs_sha256'],'CARGO_TARGET_DIR='+str(r/'target109-release'),'CARGO_PROFILE_RELEASE_LTO=false','RUST_MIN_STACK=67108864','cargo','test','--manifest-path',str(r/'working109/Cargo.toml'),'-p','openwepp-runner','--lib','--features',','.join(ident['build_inputs']['features']),'--release','--offline','--no-run','--config','profile.release.package.openwepp-hillslope-orchestrator.opt-level=1','--config','profile.release.package.openwepp-runner.opt-level=1','--message-format=json-render-diagnostics']
(r/'build109-command.json').write_text(json.dumps(cmd,indent=2)+'\n')
with (r/'build109.log').open('w') as f: result=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
raise SystemExit(result.returncode)
