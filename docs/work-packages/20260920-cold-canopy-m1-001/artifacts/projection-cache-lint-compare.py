"""Compare actual source-bearing Clippy diagnostics, including macro call sites."""
import argparse
from collections import defaultdict
import json
from pathlib import Path
import re


def diagnostics(log, source):
    result = defaultdict(list)
    for line in log.read_text().splitlines():
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            continue
        if item.get('reason') != 'compiler-message':
            continue
        m = item['message']
        if m['level'] not in ('warning', 'error'):
            continue
        spans = [s for s in m['spans'] if s['is_primary']]
        if not spans:
            continue
        locations = []
        for span in spans:
            while span.get('expansion'):
                span = span['expansion']['span']
            name = span['file_name']
            path = source / name
            function = ''
            if path.is_file():
                lines = path.read_text().splitlines()
                for text in reversed(lines[:span['line_start']]):
                    match = re.search(r'\bfn\s+(\w+)', text)
                    if match:
                        function = match.group(1)
                        break
                content = '\n'.join(lines[span['line_start']-1:span['line_end']])
            else:
                content = '\n'.join(v['text'] for v in span.get('text', []))
            locations.append((name, function, re.sub(r'\s+', ' ', content).strip()))
        key = json.dumps([(m.get('code') or {}).get('code'), m['message'], locations], sort_keys=True)
        result[key].append({'locations': locations, 'rendered': m.get('rendered')})
    return result


def main():
    p = argparse.ArgumentParser()
    p.add_argument('reference_log', type=Path)
    p.add_argument('reference_source', type=Path)
    p.add_argument('candidate_log', type=Path)
    p.add_argument('candidate_source', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    reference = diagnostics(a.reference_log, a.reference_source)
    candidate = diagnostics(a.candidate_log, a.candidate_source)
    additions, removals = [], []
    for key in sorted(reference.keys() | candidate.keys()):
        old, new = reference.get(key, []), candidate.get(key, [])
        additions.extend(new[len(old):])
        removals.extend(old[len(new):])
    result = dict(reference_count=sum(map(len, reference.values())),
                  candidate_count=sum(map(len, candidate.values())),
                  additions=additions, removals=removals,
                  method='lint + message + source path/function/content; macro expansion traced to call site; multiset comparison, not count-only',
                  reference_log=str(a.reference_log), reference_source=str(a.reference_source),
                  candidate_log=str(a.candidate_log), candidate_source=str(a.candidate_source))
    a.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({k: result[k] if k.endswith('count') else len(result[k])
                      for k in ('reference_count', 'candidate_count', 'additions', 'removals')}))


if __name__ == '__main__':
    main()
