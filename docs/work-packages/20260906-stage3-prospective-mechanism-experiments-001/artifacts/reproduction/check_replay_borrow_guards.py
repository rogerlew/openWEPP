#!/usr/bin/env python3
"""Parent-serial compiler gate: fresh verified R source, positive then negative.

Example: .venv/bin/python <this-script> --repo . --kit <R-source-kit>
  --output <new-evidence-directory> -- cargo check --tests
  -p openwepp-land-surface-energy --message-format=json
An environment wrapper (e.g. nix develop --command) may precede cargo.
This script runs builds ONLY when explicitly invoked; authoring does not run it.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import subprocess

from verify_source_kit import verify


TARGET = Path('crates/openwepp-land-surface-energy/src/solver_component_dependency_replay_tests.rs')
EXPECTED = {
    'replay_negative_second_consumption': 'E0382',
    'replay_negative_mutate_borrow': 'E0502',
    'replay_negative_drop_borrow': 'E0505',
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def record(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def unchanged_manifest(source, kit, injected=None):
    metadata = json.loads((kit / 'source-manifest.json').read_text())
    rows = json.loads((kit / metadata['identity_inputs']).read_text())
    for row in rows:
        path = source / row['path']
        if row['state'] == 'deleted':
            if path.exists() or path.is_symlink():
                raise ValueError('compiler recreated deleted source: ' + row['path'])
            continue
        data = path.read_bytes()
        actual = {'bytes': len(data), 'sha256': sha(data),
                  'mode': oct(stat.S_IMODE(path.stat().st_mode)),
                  'symlink': os.readlink(path) if path.is_symlink() else None}
        expected = dict(row)
        if injected is not None and row['path'] == str(TARGET):
            expected.update(bytes=len(injected), sha256=sha(injected))
        if any(actual[field] != expected[field] for field in actual):
            raise ValueError('compiler changed manifest source: ' + row['path'])


def compile_gate(command, source, output, label):
    record(output / (label + '-command.json'), {'argv': command, 'cwd': str(source)})
    with (output / (label + '.stdout.jsonl')).open('wb') as stdout:
        with (output / (label + '.stderr.log')).open('wb') as stderr:
            run = subprocess.run(command, cwd=source, stdout=stdout, stderr=stderr, check=False)
    record(output / (label + '-status.json'), {'returncode': run.returncode})
    return run.returncode


def classify(log, source, spans):
    found = {name: [] for name in EXPECTED}
    rejected = []
    finished = []
    for line in log.read_text().splitlines():
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            # Tool-wrapper progress is not a Rust diagnostic. Compiler errors
            # are accepted only through structured cargo compiler-message data.
            continue
        if item.get('reason') == 'build-finished':
            finished.append(item.get('success'))
        if item.get('reason') != 'compiler-message':
            continue
        message = item['message']
        if message.get('level') != 'error':
            continue
        code = (message.get('code') or {}).get('code')
        matched = []
        for span in message.get('spans', []):
            filename = Path(span['file_name'])
            path = filename if filename.is_absolute() else source / filename
            if not span.get('is_primary') or path.resolve() != (source / TARGET).resolve():
                continue
            for name, (first, last) in spans.items():
                if first <= span['line_start'] <= span['line_end'] <= last and code == EXPECTED[name]:
                    matched.append(name)
        if len(set(matched)) != 1:
            rejected.append(message)
        else:
            found[matched[0]].append(message)
    return found, rejected, finished


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', required=True, type=Path)
    parser.add_argument('--kit', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command or '--message-format=json' not in command or '--tests' not in command:
        parser.error('explicit crate test compile command with --tests --message-format=json required')
    if 'openwepp-land-surface-energy' not in command:
        parser.error('command must explicitly select openwepp-land-surface-energy')
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    previous = os.umask(0o002)
    try:
        verified = verify(args.repo.resolve(), args.kit.resolve(), True)
    finally:
        os.umask(previous)
    record(output / 'source-verification.json', verified)
    if verified['status'] != 'PASS':
        raise SystemExit('source verification failed; no compiler invocation')
    source = Path(verified['scratch_source']).resolve()
    # verify() always allocates fresh scratch. There is deliberately no arbitrary
    # --source option that could inject into a live or unverified working tree.
    target = source / TARGET
    original = target.read_bytes()
    (output / 'original-test-source.rs').write_bytes(original)
    positive = compile_gate(command, source, output, 'positive')
    _, errors, completed = classify(output / 'positive.stdout.jsonl', source, {})
    if positive != 0 or errors or completed != [True]:
        raise SystemExit('positive exact-source compile not proven; injection refused')
    unchanged_manifest(source, args.kit.resolve())
    snippet = Path(__file__).with_name('replay_borrow_negative.rs').read_bytes()
    prefix = original + b'\n'
    injected = prefix + snippet
    spans = {}
    offset = prefix.count(b'\n')
    for index, line in enumerate(snippet.decode().splitlines(), 1):
        for name in EXPECTED:
            if line.startswith('fn ' + name + '('):
                spans[name] = [offset + index, None]
        if line == '}':
            pending = [name for name, bounds in spans.items() if bounds[1] is None]
            if len(pending) != 1:
                raise ValueError('snippet function bounds ambiguous')
            spans[pending[0]][1] = offset + index
    if set(spans) != set(EXPECTED) or any(last is None for _, last in spans.values()):
        raise ValueError('snippet missing exact expected function definitions')
    context = original.decode().splitlines()[-3:]
    patch = ('*** Begin Patch\n*** Update File: ' + str(target) + '\n@@\n'
             + ''.join(' ' + line + '\n' for line in context)
             + '+\n' + ''.join('+' + line + '\n' for line in snippet.decode().splitlines())
             + '*** End of File\n*** End Patch\n')
    with (output / 'injection.log').open('wb') as log:
        subprocess.run(['apply_patch', patch], cwd=source, stdout=log,
                       stderr=subprocess.STDOUT, check=True)
    if target.read_bytes() != injected:
        raise ValueError('scratch injection bytes differ from preserved expected source')
    (output / 'injected-test-source.rs').write_bytes(injected)
    (output / 'replay_borrow_negative.rs').write_bytes(snippet)
    negative = compile_gate(command, source, output, 'negative')
    unchanged_manifest(source, args.kit.resolve(), injected)
    found, rejected, completed = classify(output / 'negative.stdout.jsonl', source, spans)
    passed = negative != 0 and completed == [False] and not rejected and all(found.values())
    result = {'status': 'PASS' if passed else 'FAIL', 'source_identity': verified['source_identity'],
              'scratch_source': str(source), 'positive_returncode': positive,
              'negative_returncode': negative, 'original_sha256': sha(original),
              'injected_sha256': sha(injected), 'snippet_sha256': sha(snippet),
              'function_lines': spans, 'expected': EXPECTED, 'matched': found,
              'rejected_errors': rejected, 'build_finished': completed,
              'scope': 'actual private Rust types; source and both logs preserved; no runtime claim'}
    record(output / 'result.json', result)
    print(json.dumps({'status': result['status'], 'evidence': str(output), 'scratch_source': str(source)}))
    raise SystemExit(0 if passed else 1)


if __name__ == '__main__':
    main()
