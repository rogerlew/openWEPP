"""Offline authentication and support arithmetic; never evaluates physical equations."""
from pathlib import Path
import hashlib
import json
import struct

ROOT = Path(__file__).resolve().parent
MANIFEST_HASH = 'b3ad4fa07c7748c56df0ad9c2fb681be019e86e9bd9befb97d705eb4d10e2ce8'

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def bits(value):
    return struct.pack('>d', value).hex()

def check():
    path = ROOT / 'architecture-decision-successor-cases.json'
    assert digest(path) == MANIFEST_HASH, 'manifest bytes'
    data = json.loads(path.read_text())
    assert digest(ROOT / data['base_input']) == data['base_sha256'], 'base bytes'
    assert digest(ROOT / data['forcing_source']) == data['forcing_source_sha256'], 'forcing bytes'
    forcing = json.loads((ROOT / data['forcing_source']).read_text())
    cases = []
    for case in data['cases']:
        for state in case['initial_phase_records']:
            assert bits(state['mass_kg_m2_tile_ground']) == state['mass_binary64_hex']
            assert bits(state['enthalpy_j_m2_tile_ground']) == state['enthalpy_binary64_hex']
        records = []
        end = 0
        for segment in case['segments']:
            assert segment['start_s'] == end
            end = segment['end_s']
            source = forcing
            for part in segment['forcing_value_source_json_pointer'].split('/')[1:]:
                part = part.replace('~1', '/').replace('~0', '~')
                source = source[int(part)] if isinstance(source, list) else source[part]
            assert source == segment['boundary_overrides']
            for key, value in source.items():
                assert bits(value) == segment['binary64_hex'][key]
            for start in range(segment['start_s'], end, 60):
                records.append((start, start + 60, segment['binary64_hex']))
        assert end == 86400 and len(records) == 1440
        schedules = {}
        for cadence in (60, 300, 900, 1800):
            supports = []
            for start, finish, fields in records:
                if supports and supports[-1]['end_s'] == start and supports[-1]['forcing_bits'] == fields and supports[-1]['start_s'] // 1800 == start // 1800 and finish - supports[-1]['start_s'] <= cadence:
                    supports[-1]['end_s'] = finish
                    supports[-1]['consumed_record_count'] += 1
                else:
                    supports.append({'start_s': start, 'end_s': finish, 'forcing_bits': fields, 'consumed_record_count': 1})
            assert sum(s['consumed_record_count'] for s in supports) == 1440
            assert sum(s['end_s']-s['start_s'] for s in supports) == 86400
            assert all(s['start_s']//1800 == (s['end_s']-1)//1800 for s in supports)
            schedules[str(cadence)] = {'interval_count': len(supports), 'support_sha256': hashlib.sha256(json.dumps(supports, sort_keys=True).encode()).hexdigest(), 'duration_counts': {str(d): sum(s['end_s']-s['start_s'] == d for s in supports) for d in sorted({s['end_s']-s['start_s'] for s in supports})}}
        cases.append({'id': case['id'], 'authenticated_original_records': len(records), 'schedules': schedules})
    return {'evidence_class': 'Ran: offline byte/bit authentication and schedule arithmetic only; no physical execution', 'manifest_sha256': MANIFEST_HASH, 'base_sha256': data['base_sha256'], 'forcing_sha256': data['forcing_source_sha256'], 'support_policy': 'greedy maximal identical-forcing runs bounded by selected cadence and 1800-second parents; all original records consumed once', 'cases': cases}

if __name__ == '__main__':
    print(json.dumps(check(), indent=2))
