"""Wait for this capture's inventory, then run the reviewed offline tools only."""
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
    'export_selected.py': '49d0ae4f3e1118d85403bdca58326cf75237c081b0f3a2e163dd693866ec43ab',
    'inspect_exports.py': '77c5a46045f20bc296a33cc385c82771d774a1f7067ea96c85a1fd16df072370',
}


def main():
    receipt = HERE / 'offline-pipeline.json'
    assert not receipt.exists()
    record = {'pid': os.getpid(), 'start_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'state': 'WAITING_FOR_EXISTING_INVENTORY', 'model_invocations': 0, 'frozen_tools': EXPECTED}
    receipt.write_text(json.dumps(record, indent=2) + '\n')
    while True:
        path = HERE / 'inventory-command.json'
        if path.exists() and 'exit' in json.loads(path.read_text()):
            assert json.loads(path.read_text())['exit'] == 0, 'inventory failed; preserve partials'
            break
        if (DEADLINE - datetime.datetime.now(datetime.timezone.utc)).total_seconds() < 4800:
            record['state'] = 'STOPPED_PRESERVATION_RESERVE'
            receipt.write_text(json.dumps(record, indent=2) + '\n')
            return
        time.sleep(5)
    for action, script in [('export', 'export_selected.py'), ('inspect', 'inspect_exports.py')]:
        assert hashlib.sha256((HERE / script).read_bytes()).hexdigest() == EXPECTED[script]
        record['state'] = action.upper()
        receipt.write_text(json.dumps(record, indent=2) + '\n')
        process = subprocess.run([str(ROOT / '.venv/bin/python'), str(HERE / 'run_export.py'), action], cwd=ROOT)
        assert process.returncode == 0, f'{action} wrapper failed'
        command = json.loads((HERE / f'{action}-command.json').read_text())
        if command['exit'] != 0:
            record['state'] = f'{action.upper()}_FAILED_PRESERVED'
            break
    else:
        record['state'] = 'COMPLETE_OFFLINE_PASSES'
    record['end_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
    receipt.write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(record), flush=True)


if __name__ == '__main__':
    main()
