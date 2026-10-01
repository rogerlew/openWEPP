import importlib.util,json,pathlib,subprocess,time,datetime,sys
base=pathlib.Path(__file__).parent
run=base/'run-tools-execution01-c5-replay'
art=pathlib.Path('/workdir/openWEPP/docs/work-packages/20260920-cold-canopy-m1-001/artifacts')
spec=importlib.util.spec_from_file_location('runner',run/'separate-accuracy-runner.py'); mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
st=mod.state()
if st['phase']!='READY':
 print(json.dumps({'stop':True,'phase':st['phase']}));sys.exit(2)
operation=mod.next_operation(st,mod.load(mod.MEASURE))
if operation is None:
 print(json.dumps({'stop':True,'schedule_complete':True}));sys.exit(3)
kind,cell,batch=operation
receipt=mod.load(mod.RECEIPT)
if receipt.get('status') != 'APPROVED':
 raise RuntimeError('Final independent custody approval is required before dispatch')
if receipt.get('dispatcher_sha256') != mod.sha(pathlib.Path(__file__)):
 raise RuntimeError('Dispatcher differs from reviewed approval')
binary=receipt['binary_path']
cmd=[sys.executable,str(run/'separate-accuracy-runner.py'),'RUN_FRESH_DAYS','--test-binary',binary,'--kind',kind]
cmd+=['--control-id',cell['control_id']] if kind=='control' else ['--cell',json.dumps(cell),'--batch',str(batch)]
seq=len(list(art.glob('near-bound-execution01-replay-command-*.json')))+1
out=art/f'near-bound-execution01-replay-command-{seq:03d}.json'
start=datetime.datetime.now(datetime.timezone.utc).isoformat(); t=time.monotonic()
p=subprocess.run(cmd,capture_output=True,text=True,timeout=180)
out.write_text(json.dumps({'argv':cmd,'cwd':str(pathlib.Path.cwd()),'start_utc':start,'elapsed_seconds':time.monotonic()-t,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr},indent=2)+'\n')
st=mod.state();r=st['operations'].get(mod.key(kind,cell,batch),{})
print(json.dumps({'seq':seq,'kind':kind,'cell':cell,'batch':batch,'phase':st['phase'],'status':r.get('status'),'physical_seconds':st['physical_seconds'],'reason':r.get('reason'),'first_failures':r.get('first_failures'),'stop':p.returncode!=0 or st['phase']!='READY'}))
if p.returncode!=0 or st['phase']!='READY':sys.exit(2)
