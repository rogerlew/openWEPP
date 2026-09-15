"""Preserve final candidate delta and verify reconstruction; no acceptance inference."""
import datetime
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile

HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('recorded',HERE/'run-recorded.py')
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
CANDIDATE=Path(sys.argv[1]) if len(sys.argv)>1 else Path('/workdir/openwepp-experiments/b01-wb14-cadence/snowfree-recorder-candidate-20260915')
BASE=module.BASE
identities,changes,patch=module.snapshot(CANDIDATE)
patch_bytes=patch.encode();patch_sha=hashlib.sha256(patch_bytes).hexdigest()
patch_file=HERE/('source-'+patch_sha+'.patch');patch_file.write_bytes(patch_bytes)
base_identity=module.snapshot(BASE)[0]
base_sha=hashlib.sha256(json.dumps(base_identity,sort_keys=True).encode()).hexdigest()
copy=json.loads((HERE.parent/'snowfree-recorder-20260915'/'candidate-copy.json').read_text())
with tempfile.TemporaryDirectory(prefix='snowfree-patch-verification-') as temp:
    temp=Path(temp)
    for entry in changes:
        target=temp/entry['path'];target.parent.mkdir(parents=True,exist_ok=True)
        target.write_bytes((BASE/entry['path']).read_bytes())
    result=subprocess.run(['patch','--batch','--fuzz=0','-p1','-i',str(patch_file)],cwd=temp,capture_output=True)
    reconstructed={entry['path']:(temp/entry['path']).read_bytes()==(CANDIDATE/entry['path']).read_bytes() for entry in changes}
frozen=Path('/workdir/openwepp-experiments/b01-wb14-cadence/authentic-recorder-20260914.frozen')
frozen_sha=hashlib.file_digest(frozen.open('rb'),'sha256').hexdigest()
receipt={'evidence_class':'Ran: final source/delta preservation and exact patch reconstruction only',
         'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'candidate':str(CANDIDATE),'base':str(BASE),
         'source_membership':copy['source_membership'],'source_entries':len(identities),
         'source_tree_sha256':hashlib.sha256(json.dumps(identities,sort_keys=True).encode()).hexdigest(),
         'source_identities':identities,'changed_files':changes,'patch':patch_file.name,'patch_sha256':patch_sha,
         'base_tree_sha256':base_sha,'base_unchanged_from_copy':base_sha==copy['source_tree_sha256'],
         'patch_reconstruction_exit':result.returncode,'patch_reconstruction_stdout':result.stdout.decode(),
         'patch_reconstruction_stderr':result.stderr.decode(),'exact_reconstructed_files':reconstructed,
         'frozen_binary':str(frozen),'frozen_binary_sha256':frozen_sha,
         'frozen_binary_unchanged':frozen_sha=='0f286b43ef8a30412b41f27414732e0c6ea4cbec7fb5e7e90913197c79687c55'}
(HERE/(sys.argv[2] if len(sys.argv)>2 else 'final-source.json')).write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({k:receipt[k] for k in ['patch','source_tree_sha256','base_unchanged_from_copy','patch_reconstruction_exit','exact_reconstructed_files','frozen_binary_unchanged']}))
