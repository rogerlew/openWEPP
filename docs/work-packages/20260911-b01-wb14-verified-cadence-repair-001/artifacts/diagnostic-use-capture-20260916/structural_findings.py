"""Bounded comparisons of recorded operands, without native authentication."""
from decimal import Decimal
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
BASE = Path('/workdir/openwepp-experiments/b01-wb14-cadence')
EXPORT = BASE / 'corrected-recorder-export-20260916-1'
INSPECT = BASE / 'corrected-recorder-inspection-20260916-1'
CONTEXT = BASE / 'corrected-recorder-context-20260916-1'
spec = importlib.util.spec_from_file_location('member_decoder', HERE / 'inspect_exports.py')
decoder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(decoder)


def read(path):
    assert path.stat().st_size <= 8 * 1024**2
    return json.loads(path.read_text(), parse_float=Decimal)


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def main():
    export, inspection, context = [read(path / 'summary.json') for path in [EXPORT, INSPECT, CONTEXT]]
    assert export['status'] == inspection['status'] == context['status'] == 'COMPLETE'
    inputs = []

    def by_kind(kind):
        rows = [row for row in export['rows'] if row['kinds'] == [kind]]
        assert len(rows) == 1
        return rows[0]

    def member(row, name):
        external = [item for item in row.get('external_json', []) if item['path'][0]['member'] == name]
        if external:
            assert len(external) == 1
            item = external[0]; path = EXPORT / item['file']
            assert sha(path) == item['sha256']
            value = read(path)
        else:
            selected = next(item for item in inspection['rows'] if item['physical_ordinal'] == row['ordinal'])
            entries = [item for item in selected['top_level_members'] if item['member'] == name]
            assert len(entries) == 1
            path = INSPECT / str(row['ordinal']) / entries[0]['file']
            value = decoder.decode_small_member(path)
        inputs.append({'path': str(path), 'sha256': sha(path), 'physical_ordinal': row['ordinal'], 'member': name})
        return value

    def decoded(row, name):
        selected = next(item for item in context['rows'] if item['physical_ordinal'] == row['ordinal'])
        item = next(item for item in selected['members'] if item['member'] == name)
        path = CONTEXT / str(row['ordinal']) / item['decoded_file']
        assert sha(path) == item['sha256']
        inputs.append({'path': str(path), 'sha256': item['sha256'], 'physical_ordinal': row['ordinal'], 'member': name})
        return read(path)

    archives = []
    for row in export['rows']:
        if row['kinds'] == ['b01_wb14_committed_day_archive_record_v1']:
            entry = member(row, 'archive_entry')
            raw = row['external_byte_arrays'][0]
            archives.append({'physical_ordinal': row['ordinal'], 'actual_day_index': member(row, 'day_index'),
                             'archive_entry': entry, 'binary_file': raw['file'],
                             'binary_sha256': raw['sha256'], 'binary_bytes': raw['bytes'],
                             'entry_day_matches_row': entry['day_index'] == member(row, 'day_index'),
                             'entry_length_hash_match_binary': entry['canonical_uncompressed_len'] == raw['bytes'] and entry['content_sha256'] == raw['sha256']})
    provider = by_kind('b01_wb14_prepared_day_current_context_v1')
    prefix = member(provider, 'archived_receipt_prefix')
    supports = member(provider, 'ordered_support_constructor_operands')
    target = by_kind('b01_wb14_snow_free_pre_child_current_context_v1')
    caller = by_kind('surface_liquid_wb14_cadence_caller_failure')
    day, interval = member(target, 'day_index'), member(target, 'interval_index')
    support, parent_support = member(target, 'support'), member(target, 'parent_support')
    prepared, phase = member(target, 'prepared_support_constructor_operands'), member(target, 'phase_diagnostics')
    clock = decoded(target, 'coupled_clock_restart_canonical_json')
    parent = decoded(target, 'parent_checkpoint_canonical_json')
    provisional = decoded(target, 'provisional_receipt_canonical_json')
    wb14 = decoded(caller, 'parent_working_typed_bytes')
    actual_supports = [item['support'] for item in phase['ordered_owner_joins']]
    selected_provider = supports[interval]['support']
    last = archives[-1]['archive_entry']
    result = {
        'evidence_class': 'Ran: bounded recorded-operand comparisons; no native or scientific authentication',
        'source_observation_sha256': export['source_sha256'],
        'report_erratum': 'inspect-summary archive_integrity.day_index contains a field-name label, not a day value. actual_day_index below is decoded from retained row events and cross-checked against archive_entry.',
        'archives': archives,
        'archive_structural_joins': {
            'actual_days': [item['actual_day_index'] for item in archives],
            'content_root_links': all(a['archive_entry']['resulting_archive_content_root_sha256'] == b['archive_entry']['previous_archive_content_root_sha256'] for a, b in zip(archives, archives[1:])),
            'ordered_day_links': all(a['archive_entry']['resulting_ordered_day_chain_sha256'] == b['archive_entry']['previous_ordered_day_chain_sha256'] for a, b in zip(archives, archives[1:])),
            'owner_set_links': all(a['archive_entry']['ending_owner_set_sha256'] == b['archive_entry']['beginning_owner_set_sha256'] for a, b in zip(archives, archives[1:])),
            'provider_content_root_matches_last': prefix['archive_content_root_sha256'] == last['resulting_archive_content_root_sha256'],
            'provider_day_chain_matches_last': prefix['ordered_day_chain_sha256'] == last['resulting_ordered_day_chain_sha256'],
            'provider_last_record_matches_last': prefix['last_day_record_sha256'] == last['record_sha256'],
            'provider_owner_set_matches_last': prefix['ending_owner_set_sha256'] == last['ending_owner_set_sha256'],
        },
        'provider': {'physical_ordinal': provider['ordinal'], 'support_count': len(supports),
                     'archived_prefix': prefix, 'selected_interval_support': selected_provider},
        'target': {'physical_ordinal': target['ordinal'], 'day_index': day, 'interval_index': interval,
                   'support': support, 'parent_support': parent_support, 'capture_phase': member(target, 'capture_phase'),
                   'provisional_constructor_inputs': member(target, 'provisional_constructor_inputs'),
                   'covered_projection': prepared['covered_projection'],
                   'prepared_support_matches_target': prepared['support'] == support,
                   'provisional_support_matches_target': provisional['support'] == support,
                   'provider_parent_support_matches_clock': selected_provider == clock['parent_support'] == parent_support,
                   'accepted_until_matches_child_start': str(parent['accepted_until_ns']) == str(clock['accepted_until_ns']) == support['start_ns'],
                   'accepted_parent_segment_count': len(parent['accepted_segments']),
                   'accepted_clock_slab_count': len(clock['accepted_slab_receipts'])},
        'history': {'owner_join_count': len(actual_supports), 'event_group_count': len(phase['ordered_event_groups']),
                    'ordered_owner_join_supports': actual_supports,
                    'join_supports_contiguous': all(a['end_ns'] == b['start_ns'] for a, b in zip(actual_supports, actual_supports[1:])),
                    'starts_at_parent_start': actual_supports[0]['start_ns'] == parent_support['start_ns'],
                    'ends_at_child_start': actual_supports[-1]['end_ns'] == support['start_ns'],
                    'prefix_validation_parent_matches': phase['prefix_validation_parent_support'] == parent_support,
                    'validated_native_inactive_wb14_prefix_recorded': phase['validated_native_inactive_wb14_prefix'],
                    'deferred_soil_present': phase['deferred_native_v2_soil_custody'] is not None,
                    'pending_terminal_parcels_present': phase['pending_terminal_parcels'] is not None,
                    'history_posture': phase['history_posture']},
        'unresolved_authority': {'outer_day_interval': [wb14['parent_day_index'], wb14['parent_interval_index']],
                                'inner_ofe_day_intervals': {key: [item['authority']['parent_day_index'], item['authority']['parent_interval_index']] for key, item in wb14['per_ofe_authorities'].items()}},
        'capture_metadata': [{'physical_ordinal': row['ordinal'], 'capture': member(row, 'capture')} for row in export['rows']],
        'input_bindings': inputs,
        'limits': 'Recorded structure and equality only. No native import, prefix proof recomputation, conservation, restart, model correctness, or cadence acceptance.'}
    (HERE / 'structural-findings.json').write_text(json.dumps(result, indent=2, default=str) + '\n')
    print(json.dumps({'archive_days': result['archive_structural_joins']['actual_days'],
                      'provider_supports': len(supports), 'history_owner_joins': len(actual_supports),
                      'target': [day, interval], 'unresolved_authority': result['unresolved_authority']}))


if __name__ == '__main__':
    main()
