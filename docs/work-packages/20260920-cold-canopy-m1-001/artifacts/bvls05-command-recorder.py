from pathlib import Path
import importlib.util,json,hashlib,os,subprocess,sys
A=Path('/workdir/openWEPP/docs/work-packages/20260920-cold-canopy-m1-001/artifacts')
spec=importlib.util.spec_from_file_location('r',A/'run_recorded.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
label=sys.argv[1];physical=sys.argv[2]=='physical';cmd=sys.argv[3:];binary=None
if cmd[:1]==['--record-binary']:
 binary=cmd[1];cmd=cmd[2:]
if not cmd:raise SystemExit('Explicit command required')
d=json.loads((A/'g4-rank-controls-release04-support.json').read_text())
d.update(source_tree_sha256=hashlib.sha256(json.dumps(r.SNAPSHOT.snapshot(r.SOURCE)[0],sort_keys=True).encode()).hexdigest(),scope=label,intended_argv=cmd,declared_timeout_seconds=180,physical=physical,material_environment={k:os.environ.get(k) for k in r.MATERIAL_ENVIRONMENT},adopted_support_changes=['Owner-adopted BVLS05 authority and fixed allowance; explicit bounded command, no automatic selection or retry.'])
extra=[A/'bvls05-authorization.md',A/'bvls05-start-ledger.json',A/'bvls05-control-plan01.json',Path('/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md'),Path('/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md')]
extra += sorted(A.glob('bvls05-*-policy*.json'))
extra += sorted(A.glob('bvls05-control-plan*.json'))
extra += sorted(A.glob('bvls05-g4-plan*.json'))
extra += sorted(A.glob('bvls05-conditional-plan*.json'))
extra += [A/n for n in ['bvls05-g4-physical01.json','bvls05-g4-capture01.json','bvls05-g4-cost01.json','bvls05-g4-results01.json'] if (A/n).exists()]
extra += sorted(A.glob('bvls05-quality-*08.json'))
extra += sorted(A.glob('bvls05-quality-*09.json'))
extra += sorted(A.glob('bvls05-quality-*-lint09-comparison02.json'))
extra += [A/n for n in ['bvls05-g2-fix09.json','bvls05-terminal09-recovery.json','bvls05-quality09-source-diff.json','bvls05-conditional-g2-01.json'] if (A/n).exists()]
extra += sorted(A.glob('bvls05-quality-*-lint08-comparison02.json'))
extra += [A/n for n in ['bvls05-terminal08-recovery.json','bvls05-g4-protected-source08.json','bvls05-quality-empty-comparison08.json'] if (A/n).exists()]
extra += sorted(A.glob('bvls05-physical-run-plan*.json'))
extra += sorted(A.glob('bvls05-physical-*-01.json'))
extra += [A/'bvls05-physical-results01.json'] if (A/'bvls05-physical-results01.json').exists() else []
extra += [A/n for n in ['bvls05-empty-old-body-baseline.json','bvls05-focused-selection01.json','bvls05-physical-control-lineage01.json'] if (A/n).exists()]
extra += sorted(A.glob('bvls05-correctness-*.json'))
extra += sorted(A.glob('bvls05-qa-*.json'))
extra += [A/n for n in ['g4-rank-capture.json','g4-rank-correctness-reference01.stdout','g4-rank-correctness-terminal01.json','g4-rank-qa-terminal01.json']]
extra += [Path(v) for v in cmd if Path(v).is_absolute() and Path(v).is_file()]
known={f['path'] for f in d['files']}
for p in extra:
 if str(p) not in known:d['files'].append({'path':str(p),'sha256':r.sha(p)})
for f in d['files']:f['sha256']=r.sha(f['path'])
out=A/(label+'-support.json');r.save(out,d)
args=['/workdir/openWEPP/.venv/bin/python',str(A/'run_recorded.py'),'--timeout','180']
if physical:args+=['--physical']
if binary:args+=['--binary',binary]
args+=['--source-root',str(r.SOURCE),'--support-record',str(out),label,'--',*cmd]
raise SystemExit(subprocess.run(args,cwd=r.SOURCE).returncode)
