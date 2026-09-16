"""Select retained fields in one streaming read; does not run or validate physics."""
import hashlib
import json
from pathlib import Path

import ijson
from ijson.common import ObjectBuilder

EXPORT = Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1')
HERE = Path(__file__).resolve().parent
native_path = EXPORT / 'rows/123091.19.json'
wanted = {
    'accepted_interval_count', 'next_day_index', 'provider_gsi_receipt_sha256',
    'gsi_owner_state', 'provider_cursor_canonical_json', 'wb14_parent',
    'frozen_litter_residents.physical_wb14_parent',
    'frozen_litter_residents.surface_owner_canonical_bytes',
    'frozen_litter_residents.exact_surface_owner_canonical_bytes',
}
selected = {}
builder = None
active = None
depth = 0
with native_path.open('rb') as stream:
    for prefix, event, value in ijson.parse(stream):
        if active is None and prefix in wanted and event != 'map_key':
            active = prefix
            builder = ObjectBuilder()
            depth = 0
        if active is not None:
            builder.event(event, value)
            depth += int(event in ('start_map', 'start_array'))
            depth -= int(event in ('end_map', 'end_array'))
            if depth == 0:
                selected[active] = builder.value
                active = None
assert selected.keys() == wanted
provider_path = EXPORT / 'rows/108033.11.json'
provider = json.loads(provider_path.read_text())
receipt = provider['gsi_receipt']
cursor = json.loads(bytes(selected['provider_cursor_canonical_json']))
surface = json.loads(bytes(selected['frozen_litter_residents.surface_owner_canonical_bytes']))
exact_surface = json.loads(bytes(selected['frozen_litter_residents.exact_surface_owner_canonical_bytes']))
identities = {}
for path in (native_path, provider_path):
    with path.open('rb') as stream:
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    ordinal = path.name.split('.')[0]
    records = [json.loads(line) for line in (EXPORT / 'rows' / f'{ordinal}.events.jsonl').read_text().splitlines()]
    binding = [row for row in records if row['event'] == 'external_json_end'
               and row.get('file') == 'rows/' + path.name]
    assert len(binding) == 1 and binding[0]['sha256'] == digest
    assert binding[0]['bytes'] == path.stat().st_size
    identities[path.name] = {'sha256': digest, 'bytes': path.stat().st_size}

# Keep the original surface object shape; this is inspection, not normalization.
result = {
    'evidence_class': 'Ran: read-only streaming JSON selection; no Rust/physics/canonical admission',
    'members': identities,
    'accepted_interval_count': selected['accepted_interval_count'],
    'next_day_index': selected['next_day_index'],
    'native_wb14_parent': selected['wb14_parent'],
    'physical_wb14_parent': selected['frozen_litter_residents.physical_wb14_parent'],
    'surface_owner': surface,
    'exact_surface_owner': exact_surface,
    'provider_cursor': cursor,
    'captured_gsi_receipt': selected['provider_gsi_receipt_sha256'],
    'captured_gsi_state': selected['gsi_owner_state'],
    'prepared_receipt': {key: receipt[key] for key in
        ('schema_version', 'day_index', 'receipt_sha256', 'beginning_state', 'ending_state')},
    'captured_gsi_equals_prepared_beginning': selected['gsi_owner_state'] == receipt['beginning_state'],
    'captured_cursor_equals_prepared_beginning': selected['provider_cursor_canonical_json'] == provider['beginning_cursor_canonical_json'],
    'limitation': 'No independently retained day3 receipt preimage is reconstructed here. Its association with the captured receipt is source-lifecycle inference, not authenticated receipt replay.',
}
assert result['captured_gsi_equals_prepared_beginning']
assert result['captured_cursor_equals_prepared_beginning']
def encode(value):
    # Selected GSI canonical states and surface objects use hex/string numerics.
    # Unexpected Decimal values must remain explicit, never rounded to float.
    from decimal import Decimal
    if isinstance(value, Decimal):
        return {'exact_decimal': str(value)}
    raise TypeError(type(value).__name__)
(HERE / 'position-gsi-operands.json').write_text(json.dumps(result, indent=2, default=encode) + '\n')
print(json.dumps({key: result[key] for key in (
    'members', 'accepted_interval_count', 'next_day_index', 'captured_gsi_receipt',
    'captured_gsi_equals_prepared_beginning', 'captured_cursor_equals_prepared_beginning')}, indent=2))
