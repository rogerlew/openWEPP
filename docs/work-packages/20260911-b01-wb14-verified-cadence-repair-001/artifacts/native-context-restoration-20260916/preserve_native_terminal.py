import json,importlib.util,hashlib,datetime,difflib,subprocess
from pathlib import Path
A=Path('/workdir/openWEPP/docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/native-context-restoration-20260916')
S=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916')
O=S.parent/'snowfree-recorder-candidate-20260915'
def module(name,path):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
def write(name,obj): (A/name).write_text(json.dumps(obj,indent=2)+'\n')
def sha(p):
 with p.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
m=module('compare',A.parent/'complete-lint-diagnostics-20260916/compare-complete.py')
r=m.compare('orchestrator',m.stream(A.parent/'complete-lint-diagnostics-20260916/candidate-orchestrator.stdout.gz'),m.stream(A/'scoped-clippy.stdout'),O,S)
write('scoped-clippy-comparison.json',r)
print('clippy',{k:r[k] for k in ['baseline_messages','candidate_messages','matched_messages']},'unmatched',len(r['candidate_unmatched']))
identity=module('identity',A.parent/'complete-lint-diagnostics-20260916/run-collection.py')
after=identity.identity(O)
write('acquisition-source-after.json',after)
assert after['tree_sha256']=='ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592'
assert after['patch_sha256']=='4559c7d21fbcdb9314845e6a5c19940d051d0ef861e48c7cb2a4ae7e51020c5d'
assert not after['supplemental_mismatches']
terminal=json.loads((A/'scoped-provider-018-source.json').read_text())
old=identity.prior.snapshot(O)[0];new=terminal['entries']
changed=[p for p in new if new[p]!=old[p]]
patch=''.join(''.join(difflib.unified_diff((O/p).read_text().splitlines(True),(S/p).read_text().splitlines(True),fromfile='a/'+p,tofile='b/'+p)) for p in changed)
(A/'reader-incremental.patch').write_text(patch)
# Verify patch application against acquisition source using an isolated small reconstruction.
verify=Path('/tmp/openwepp-native-restoration-scoped-patch-verification-20260916');verify.mkdir(exist_ok=True)
for p in changed:
 q=verify/p;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes((O/p).read_bytes())
rp=subprocess.run(['git','apply',str(A/'reader-incremental.patch')],cwd=verify,capture_output=True,text=True)
assert rp.returncode==0,rp.stderr
assert all(sha(verify/p)==new[p] for p in changed)
write('terminal-reconciliation.json',dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),changed_from_acquisition=changed,tree_sha256=terminal['tree_sha256'],cumulative_patch=terminal['patch'],cumulative_patch_sha256=terminal['patch_sha256'],incremental_patch_sha256=sha(A/'reader-incremental.patch'),incremental_reconstruction_pass=True,original_source_unchanged=True,canonical_validator_changes=False,public_api_schema_dependency_changes=False,model_executions=0))
E=S.parent/'corrected-recorder-export-20260916-1';custody=json.loads((A/'input-custody-before.json').read_text())
assert sha(E/'summary.json')==custody['accepted_summary_sha256']
assert all(Path(x['path']).stat().st_size==x['bytes'] and sha(Path(x['path']))==x['sha256'] for x in custody['entries'])
write('input-custody-after.json',dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),all_49_members_unchanged=True,accepted_summary_unchanged=True,summary_sha256=sha(E/'summary.json'),pinned_seed_sha256=sha(A/'pinned-original-seed.json'),raw_inventory_reused=True))
