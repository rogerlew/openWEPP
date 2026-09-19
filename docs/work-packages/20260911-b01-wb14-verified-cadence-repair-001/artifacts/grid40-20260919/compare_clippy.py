"""Compare retained Cargo JSON diagnostics without executing Cargo or a solver."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path


FROZEN = Path('/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-source-20260918')
CANDIDATE = Path('/home/roger/openwepp-experiments/b01-wb14-grid40-source-20260919')


def diagnostics(path, source_root):
    result = Counter()
    messages = 0
    for line in Path(path).read_text().splitlines():
        if not line.startswith('{'):
            continue
        row = json.loads(line)
        if row.get('reason') != 'compiler-message':
            continue
        manifest = Path(row['manifest_path'])
        if not manifest.is_relative_to(source_root):
            raise ValueError(f'wrong source manifest: {manifest}; expected {source_root}')
        message = row['message']
        if message['level'] not in ('error', 'warning') or not message.get('code'):
            continue
        spans = message['spans']
        primary = [span for span in spans if span['is_primary']]
        if not primary:
            raise ValueError('coded diagnostic lacks primary source span')
        locations = []
        for span in primary:
            filename = span['file_name']
            if '/crates/' in filename:
                filename = 'crates/' + filename.split('/crates/', 1)[1]
            locations.append({
                'file': filename,
                'highlighted_source': [
                    text['text'][text['highlight_start'] - 1:
                                 text['highlight_end'] - 1].strip()
                    for text in span['text']],
            })
        key = json.dumps({'level': message['level'],
                          'code': message['code']['code'],
                          'message': message['message'],
                          'primary_source': locations}, sort_keys=True)
        result[key] += 1
        messages += 1
    if not messages:
        raise ValueError('no coded source diagnostics; cannot establish comparison')
    return result


def expanded(counter):
    return [dict(json.loads(key), occurrences=count)
            for key, count in sorted(counter.items())]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('baseline', type=Path)
    parser.add_argument('candidate', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    baseline = diagnostics(args.baseline, FROZEN)
    candidate = diagnostics(args.candidate, CANDIDATE)
    introduced = candidate - baseline
    removed = baseline - candidate
    receipt = {
        'evidence_class': 'Ran: offline comparison of retained Cargo JSON only',
        'normalization': 'Ignore source-root prefixes, line/column positions, indentation and unhighlighted line context; retain code, message, exact primary highlighted source and multiplicity',
        'limitation': 'Source identity, exact flags and build provenance require independent review; strict Clippy remains failed if inherited diagnostics exist',
        'baseline': str(args.baseline),
        'candidate': str(args.candidate),
        'verified_manifest_roots': [str(FROZEN), str(CANDIDATE)],
        'input_sha256': {str(path): hashlib.sha256(path.read_bytes()).hexdigest()
                         for path in (args.baseline, args.candidate)},
        'baseline_diagnostic_occurrences': sum(baseline.values()),
        'candidate_diagnostic_occurrences': sum(candidate.values()),
        'candidate_only': expanded(introduced),
        'baseline_only': expanded(removed),
        'no_new_or_changed_diagnostics': not introduced,
    }
    args.output.write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps({key: receipt[key] for key in
                      ('baseline_diagnostic_occurrences',
                       'candidate_diagnostic_occurrences',
                       'no_new_or_changed_diagnostics')}))
    return 0 if not introduced else 1


if __name__ == '__main__':
    raise SystemExit(main())
