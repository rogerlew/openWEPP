"""Compare actual compiler diagnostics by code, message, file and primary source text.
Line numbers are retained in evidence but excluded from identity because edits shift them.
This is observational comparison, never a qualification substitute.
"""
from collections import Counter
import json
from pathlib import Path
import sys


def read(path):
    items = []
    for line in path.read_text().splitlines():
        try:
            record = json.loads(line)
        except ValueError:
            continue
        if record.get('reason') != 'compiler-message':
            continue
        message = record['message']
        spans = []
        for span in message.get('spans', []):
            if not span.get('is_primary'):
                continue
            name = span['file_name']
            if '/crates/' in name:
                name = 'crates/' + name.split('/crates/', 1)[1]
            spans.append({'file': name, 'line': span['line_start'], 'text': [x['text'] for x in span.get('text', [])]})
        items.append({'code': (message.get('code') or {}).get('code'), 'level': message['level'], 'message': message['message'], 'spans': spans})
    return items


def identity(item):
    return json.dumps({**item, 'spans': [{k:v for k,v in span.items() if k != 'line'} for span in item['spans']]}, sort_keys=True)

baseline, candidate, output = map(Path, sys.argv[1:])
old, new = read(baseline), read(candidate)
old_counts, new_counts = Counter(map(identity, old)), Counter(map(identity, new))
introduced, removed = new_counts-old_counts, old_counts-new_counts
result = {'evidence_class': 'Ran: matched diagnostic content comparison, not strict-lint acceptance', 'baseline': str(baseline), 'candidate': str(candidate), 'baseline_count': len(old), 'candidate_count': len(new), 'introduced_or_changed': [{**json.loads(key), 'count': count} for key,count in sorted(introduced.items())], 'removed_or_changed': [{**json.loads(key), 'count': count} for key,count in sorted(removed.items())], 'candidate_locations': [item for item in new if identity(item) in introduced]}
output.write_text(json.dumps(result, indent=2)+'\n')
print(json.dumps({'baseline': len(old), 'candidate': len(new), 'introduced_or_changed': sum(introduced.values()), 'removed_or_changed': sum(removed.values())}))
