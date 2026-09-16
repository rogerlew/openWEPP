"""Wait for the existing capture's terminal record, preserve it, inventory once.

This script cannot launch a model. It reads the separately recorded existing job.
"""
import datetime
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = Path('/workdir/openWEPP')
DEADLINE = datetime.datetime.fromisoformat('2026-09-16T05:39:08.596115+00:00')


def now():
    return datetime.datetime.now(datetime.timezone.utc)


def write(name, value):
    (HERE / name).write_text(json.dumps(value, indent=2) + '\n')


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def main():
    while not (HERE / 'raw-launch-terminal.json').exists():
        if (DEADLINE - now()).total_seconds() < 900:
            write('terminal-collection.json', {'status': 'BLOCKED', 'reason': 'preservation reserve reached while waiting; existing job identities remain in raw-launch-start.json'})
            return
        time.sleep(5)
    terminal = json.loads((HERE / 'raw-launch-terminal.json').read_text())
    source = Path(terminal['output'])
    record = {'status': 'INCOMPLETE', 'started_utc': now().isoformat(), 'terminal': terminal,
              'files': [], 'large_raw_remote_publication': False}
    small = HERE / 'terminal'
    small.mkdir(exist_ok=False)
    for path in sorted(source.iterdir()):
        if not path.is_file():
            continue
        entry = {'path': str(path), 'bytes': path.stat().st_size, 'sha256': sha(path)}
        # Full large observation/spool/comparison data remain durable locally.
        if path.stat().st_size <= 1_000_000:
            destination = small / path.name
            shutil.copy2(path, destination)
            assert sha(destination) == entry['sha256']
            entry['published_copy'] = str(destination.relative_to(HERE))
        record['files'].append(entry)
    record['end_utc'] = now().isoformat()
    record['status'] = 'COMPLETE_LOCAL_PRESERVATION'
    write('terminal-collection.json', record)
    receipt_path = source / 'receipt.json'
    if not receipt_path.exists():
        print('Terminal preserved; collector receipt absent; inventory BLOCKED', flush=True)
        return
    receipt = json.loads(receipt_path.read_text())
    observation = source / 'observations.json'
    if not observation.exists() or not isinstance(receipt.get('physical_rows'), int):
        print('Terminal preserved; observation/row count absent; inventory BLOCKED', flush=True)
        return
    actual = next(item for item in record['files'] if item['path'] == str(observation))
    assert actual['sha256'] == receipt['artifact_sha256']['observations.json']
    available = (DEADLINE - now()).total_seconds()
    assert available >= 2100, 'insufficient inventory plus preservation allowance'
    inventory = HERE.parent / 'stream-inventory-20260914/inventory.py'
    argv = ['/usr/bin/time', '-v', '-o', str(HERE / 'inventory.memory.log'),
            '/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '1200s',
            '/usr/bin/prlimit', '--as=1073741824:1073741824', str(ROOT / '.venv/bin/python'),
            str(inventory), str(observation), str(HERE / 'inventory'),
            '--sha256', actual['sha256'], '--bytes', str(actual['bytes']), '--rows', str(receipt['physical_rows'])]
    command = {'argv': argv, 'cwd': str(ROOT), 'start_utc': now().isoformat(),
               'utility_sha256': sha(inventory), 'source_sha256': actual['sha256'],
               'source_bytes': actual['bytes'], 'physical_rows': receipt['physical_rows'],
               'remaining_seconds_at_start': available, 'model_invocations': 0}
    write('inventory-command.json', command)
    with (HERE / 'inventory.stdout').open('xb') as out, (HERE / 'inventory.stderr').open('xb') as err:
        process = subprocess.Popen(argv, cwd=ROOT, stdout=out, stderr=err)
        command['pid'] = process.pid
        write('inventory-command.json', command)
        command['exit'] = process.wait()
    command['end_utc'] = now().isoformat()
    write('inventory-command.json', command)
    print(json.dumps({'collection': record['status'], 'inventory_exit': command['exit'],
                      'runner_exit': receipt.get('exit_code'), 'collector_exit': terminal['collector_exit']}), flush=True)


if __name__ == '__main__':
    main()
