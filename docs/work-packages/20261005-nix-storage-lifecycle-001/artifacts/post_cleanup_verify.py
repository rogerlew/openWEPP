"""Verify exact source restoration and protected tools after scoped cleanup."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
from archive_snapshots import dump_hash

HERE=Path(__file__).resolve().parent
rows=sorted(json.loads((HERE/'legacy-snapshot-recovery.json').read_text()),key=lambda r:r['archive_bytes'])
if not all(not Path(r['source']).exists() for r in rows):
    raise RuntimeError('historical removal is not complete')
results=[]
for index in [0,len(rows)//2,len(rows)-1]:
    row=rows[index]
    temp=Path(tempfile.mkdtemp(prefix='post-cleanup-',dir='/workdir/openwepp-recovery/nix-snapshots-20261005'))
    dest=temp/'source'
    decoder=subprocess.Popen(['zstd','-q','-d','-c',row['archive']],stdout=subprocess.PIPE)
    restore=subprocess.run(['nix-store','--restore',str(dest)],stdin=decoder.stdout)
    decoder.stdout.close()
    if decoder.wait() or restore.returncode:
        raise RuntimeError('post-cleanup restoration failed')
    actual=dump_hash(dest)
    if actual!=row['restored_nar_sha256']:
        raise RuntimeError('post-cleanup NAR identity mismatch')
    results.append({'original_source':row['source'],'archive':row['archive'],'restored_nar_sha256':actual,'result':'PASS'})
    shutil.rmtree(temp)
pins=json.loads((HERE/'legacy-toolchain-pins.json').read_text())
tools=json.loads(Path(pins['source_receipt']).read_text())
for pin in pins['pins']:
    if str(Path(pin['root']).resolve(strict=True))!=pin['store_path']:
        raise RuntimeError('retained tool root changed')
    roots=subprocess.check_output(['nix-store','--query','--roots',pin['store_path']],text=True)
    if [pin['root'],pin['store_path']] not in [line.split(' -> ',1) for line in roots.splitlines()]:
        raise RuntimeError('retained tool root unregistered')
    item=tools[pin['tool']]
    actual=hashlib.sha256(Path(item['resolved']).read_bytes()).hexdigest()
    if actual!=item['sha256']:
        raise RuntimeError('retained tool executable changed')
    results.append({'tool':pin['tool'],'root':pin['root'],'executable_sha256':actual,'result':'PASS'})
(HERE/'post-cleanup-verification.json').write_text(json.dumps({'removed_source_count':len(rows),'restore_samples_and_protected_tools':results},indent=2)+'\n')
print('PASS: all183 original snapshot paths absent; three archive-size cohorts restore identically; five frozen tool roots and executable hashes unchanged.')
