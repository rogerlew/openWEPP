import json
from pathlib import Path
r=Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1/rows')
n=json.loads((r/'123091.19.json').read_bytes())
p=json.loads((r/'108033.11.json').read_bytes())
print(json.dumps({'native':{k:n[k] for k in ['accepted_interval_count','next_day_index','provider_gsi_receipt_sha256']},'native_cursor':json.loads(bytes(n['provider_cursor_canonical_json'])),'provider_gsi_receipt':p['gsi_receipt'],'provider_beginning_cursor':json.loads(bytes(p['beginning_cursor_canonical_json']))},indent=2))
