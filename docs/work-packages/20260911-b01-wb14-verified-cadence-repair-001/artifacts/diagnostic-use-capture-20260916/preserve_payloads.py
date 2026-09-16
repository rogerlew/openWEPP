"""Preserve compact export/decoded payloads for publication; retain large originals."""
import datetime
import gzip
import hashlib
import json
from pathlib import Path
import shutil

HERE = Path(__file__).resolve().parent
DURABLE = Path('/workdir/openwepp-experiments/b01-wb14-cadence')
EXPORT = DURABLE / 'corrected-recorder-export-20260916-1'
CONTEXT = DURABLE / 'corrected-recorder-context-20260916-1'
LIMIT = 8 * 1024**2


def digest(path):
    with path.open('rb') as raw:
        return hashlib.file_digest(raw, 'sha256').hexdigest()


def main():
    summary = json.loads((EXPORT / 'summary.json').read_text())
    context = json.loads((CONTEXT / 'summary.json').read_text())
    assert summary['status'] == context['status'] == 'COMPLETE'
    destination = HERE / 'payloads'
    destination.mkdir(exist_ok=False)
    record = {'created_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'source_observation_sha256': summary['source_sha256'],
              'maximum_individual_raw_bytes_for_publication': LIMIT,
              'export_summary_sha256': digest(EXPORT / 'summary.json'),
              'context_summary_sha256': digest(CONTEXT / 'summary.json'), 'files': []}

    def preserve(source, relative, expected_bytes, expected_sha, ordinal, member_path):
        assert source.stat().st_size == expected_bytes
        item = {'source': str(source), 'bytes': expected_bytes, 'sha256': expected_sha,
                'physical_ordinal': ordinal, 'member_path': member_path}
        if expected_bytes > LIMIT:
            item.update(custody='durable local only', limitation='large payload excluded from compact remote publication; complete original retained')
        else:
            assert digest(source) == expected_sha
            target = destination / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            if expected_bytes > 100_000:
                target = target.with_name(target.name + '.gz')
                with source.open('rb') as raw, target.open('xb') as output:
                    with gzip.GzipFile(filename='', mode='wb', fileobj=output, mtime=0) as zipped:
                        shutil.copyfileobj(raw, zipped, length=65536)
                with gzip.open(target, 'rb') as raw:
                    restored = hashlib.file_digest(raw, 'sha256').hexdigest()
                assert restored == expected_sha
                item['encoding'] = 'gzip; decompressed bytes verified'
            else:
                with source.open('rb') as raw, target.open('xb') as output:
                    shutil.copyfileobj(raw, output, length=65536)
                assert digest(target) == expected_sha
                item['encoding'] = 'exact bytes'
            item.update(custody='prepared for compact publication', published_path=str(target.relative_to(HERE)),
                        published_bytes=target.stat().st_size, published_sha256=digest(target))
        record['files'].append(item)

    for row in summary['rows']:
        preserve(EXPORT / row['event_file'], Path('export') / row['event_file'], row['event_file_bytes'],
                 row['event_file_sha256'], row['ordinal'], [])
        for member in row.get('external_byte_arrays', []) + row.get('external_json', []):
            preserve(EXPORT / member['file'], Path('export') / member['file'], member['bytes'], member['sha256'], row['ordinal'], member['path'])
    for row in context['rows']:
        for member in row['members']:
            if 'decoded_file' in member:
                relative = Path(str(row['physical_ordinal'])) / member['decoded_file']
                preserve(CONTEXT / relative, Path('decoded') / relative, member['bytes'], member['sha256'],
                         row['physical_ordinal'], [{'member': member['member'], 'position': member['position']}])
    record['published_files'] = sum('published_path' in item for item in record['files'])
    record['local_only_files'] = sum('published_path' not in item for item in record['files'])
    record['published_bytes'] = sum(item.get('published_bytes', 0) for item in record['files'])
    (HERE / 'payload-publication-manifest.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps({key: record[key] for key in ['published_files', 'local_only_files', 'published_bytes']}))


if __name__ == '__main__':
    main()
