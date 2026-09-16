"""Collect the already-running export and run approved offline inspections."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = Path('/workdir/openWEPP')
DEADLINE = datetime.datetime.fromisoformat('2026-09-16T05:39:08.596115+00:00')
EXPECTED = {
    'export_selected.py': '2e6e0d8c6c318124ee5d30fb06b97baebcd3e6233a0ce6a998e4a4648aea586e',
    'inspect_exports.py': '57b5dcafd8677d1122b71027eba90ea90f4f40e03477d38ba487d3857c37f93a',
    'inspect_context.py': '586408287ff1ad3cc866db868d17b0bb98cf34dc1649f480592a5016dcfc4edb',
}


def main():
    path = HERE / 'inspection-pipeline.json'
    assert not path.exists()
    record = {'pid': os.getpid(), 'state': 'WAITING_FOR_EXISTING_EXPORT',
              'start_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'frozen_tools': EXPECTED, 'model_invocations': 0}
    def save():
        path.write_text(json.dumps(record, indent=2) + '\n')
    save()
    try:
        while True:
            command = json.loads((HERE / 'export-command.json').read_text())
            if 'exit' in command:
                assert command['exit'] == 0 and command['utility_unchanged'], 'export did not complete unchanged'
                break
            assert (DEADLINE - datetime.datetime.now(datetime.timezone.utc)).total_seconds() > 2100, 'preservation reserve reached'
            time.sleep(5)
        for script, expected in EXPECTED.items():
            assert hashlib.sha256((HERE / script).read_bytes()).hexdigest() == expected, script
        for action in ['inspect', 'context']:
            record['state'] = action.upper()
            save()
            process = subprocess.run([str(ROOT / '.venv/bin/python'), str(HERE / 'run_export.py'), action], cwd=ROOT)
            assert process.returncode == 0, f'{action} wrapper failed'
            command = json.loads((HERE / f'{action}-command.json').read_text())
            assert command['exit'] == 0 and command['utility_unchanged'], f'{action} incomplete'
        record['state'] = 'COMPLETE_OFFLINE_INSPECTIONS'
    except Exception as error:
        record.update(state='INCOMPLETE_PRESERVED', error=f'{type(error).__name__}: {error}')
        raise
    finally:
        record['end_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
        save()
    print(json.dumps(record), flush=True)


if __name__ == '__main__':
    main()
