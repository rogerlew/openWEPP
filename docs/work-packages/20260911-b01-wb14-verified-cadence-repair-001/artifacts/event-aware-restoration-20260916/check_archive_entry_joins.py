"""Independent scalar join check on retained archive-entry member events only.

This does not restore archives or validate their receipt/root digests. It retains
expected operands for comparison with the separately executed canonical reader.
"""
import datetime
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPORT = Path('/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1')


def archive_entry(path):
    events = [json.loads(line) for line in path.read_text().splitlines()]
    start = next(i for i, event in enumerate(events)
                 if event['event'] == 'map_key' and event['value'] == 'archive_entry')
    assert events[start + 1]['event'] == 'start_map'
    entry = {}
    position = start + 2
    while events[position]['event'] != 'end_map':
        key, value = events[position], events[position + 1]
        assert key['event'] == 'map_key' and key['value'] not in entry
        assert value['event'] in ('number', 'string')
        entry[key['value']] = int(value['value']['number_lexeme']) if value['event'] == 'number' else value['value']
        position += 2
    return entry


def main():
    summary_path = EXPORT / 'summary.json'
    summary = json.loads(summary_path.read_text())
    assert summary['source_sha256'] == '1555efe1b592081a3dd67639d71a18e4e67f518e24b93a95ddfe18de2023d87c'
    selected = [row for row in summary['rows'] if row['kinds'] == ['b01_wb14_committed_day_archive_record_v1']]
    rows = []
    for row in selected:
        path = EXPORT / row['event_file']
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        assert digest == row['event_file_sha256']
        entry = archive_entry(path)
        rows.append(dict(ordinal=row['ordinal'], event_file=row['event_file'], sha256=digest, entry=entry))
    assert len(rows) == 4
    checks = []
    for index, row in enumerate(rows):
        entry = row['entry']
        assert entry['day_index'] == index, 'recorded order; never sorted'
        if index:
            previous = rows[index - 1]['entry']
            joins = {
                'beginning_owner': entry['beginning_owner_set_sha256'] == previous['ending_owner_set_sha256'],
                'content_root': entry['previous_archive_content_root_sha256'] == previous['resulting_archive_content_root_sha256'],
                'ordered_root': entry['previous_ordered_day_chain_sha256'] == previous['resulting_ordered_day_chain_sha256'],
                'clock_monotone': entry['ending_accepted_until_ns'] > previous['ending_accepted_until_ns'],
                'sequence_monotone': entry['ending_next_parent_sequence'] > previous['ending_next_parent_sequence'],
            }
            assert all(joins.values()), joins
            checks.append(dict(day_index=index, joins=joins))
    result = dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  evidence_class='Ran: independent Python read of small member events in recorded manifest order',
                  scope='Expected scalar joins only; no canonical reconstruction, archive admission, physics or native restoration',
                  summary_sha256=hashlib.sha256(summary_path.read_bytes()).hexdigest(),
                  rows=rows, joins=checks, status='PASS scalar adjacency only')
    (HERE / 'independent-archive-entry-joins.json').write_text(json.dumps(result, indent=2) + '\n')
    print(result['status'])


if __name__ == '__main__':
    main()
