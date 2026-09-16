"""Run only offline export or inspection against the newly collected corpus."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = Path('/workdir/openWEPP')
DURABLE = Path('/workdir/openwepp-experiments/b01-wb14-cadence')
EXPORT = DURABLE / 'corrected-recorder-export-20260916-1'
INSPECTION = DURABLE / 'corrected-recorder-inspection-20260916-1'
CONTEXT = DURABLE / 'corrected-recorder-context-20260916-1'
DEADLINE = datetime.datetime.fromisoformat('2026-09-16T05:39:08.596115+00:00')


def now():
    return datetime.datetime.now(datetime.timezone.utc)


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['export', 'inspect', 'context'])
    args = parser.parse_args()
    census = json.loads((HERE / 'inventory/summary.json').read_text())
    assert census['status'] == 'COMPLETE'
    remaining = (DEADLINE - now()).total_seconds()
    timeout = {'export': 3600, 'inspect': 600, 'context': 900}[args.action]
    assert remaining >= timeout + 1200, 'preserve independent review/publication reserve'
    script = HERE / {'export': 'export_selected.py', 'inspect': 'inspect_exports.py',
                     'context': 'inspect_context.py'}[args.action]
    destination = {'export': EXPORT, 'inspect': INSPECTION, 'context': CONTEXT}[args.action]
    assert not destination.exists(), 'preserve prior offline output'
    if args.action == 'export':
        tool_args = [str(DURABLE / 'corrected-recorder-capture-20260916-1/observations.json'),
                     str(HERE / 'inventory'), str(destination),
                     '--max-export-bytes', str(4 * 1024**3), '--reserve-bytes', str(1024**3)]
    else:
        tool_args = [str(EXPORT), str(destination)]
    argv = ['/usr/bin/time', '-v', '-o', str(HERE / f'{args.action}.memory.log'),
            '/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', f'{timeout}s',
            '/usr/bin/prlimit', '--as=1073741824:1073741824', str(ROOT / '.venv/bin/python'),
            str(script), *tool_args]
    record = {'action': args.action, 'argv': argv, 'cwd': str(ROOT),
              'utility_sha256': sha(script), 'inventory_summary_sha256': sha(HERE / 'inventory/summary.json'),
              'source_sha256': census['sha256_supplied'], 'start_utc': now().isoformat(),
              'remaining_seconds_at_start': remaining, 'model_invocations': 0}
    receipt = HERE / f'{args.action}-command.json'
    assert not receipt.exists()
    receipt.write_text(json.dumps(record, indent=2) + '\n')
    with (HERE / f'{args.action}.stdout').open('xb') as out, (HERE / f'{args.action}.stderr').open('xb') as err:
        process = subprocess.Popen(argv, cwd=ROOT, stdout=out, stderr=err)
        record['pid'] = process.pid
        receipt.write_text(json.dumps(record, indent=2) + '\n')
        record['exit'] = process.wait()
    record.update(end_utc=now().isoformat(), utility_unchanged=sha(script) == record['utility_sha256'])
    if (destination / 'summary.json').exists():
        record['summary_sha256'] = sha(destination / 'summary.json')
        (HERE / f'{args.action}-summary.json').write_bytes((destination / 'summary.json').read_bytes())
    receipt.write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(record), flush=True)


if __name__ == '__main__':
    main()
