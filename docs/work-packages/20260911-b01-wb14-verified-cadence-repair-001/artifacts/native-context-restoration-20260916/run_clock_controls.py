"""Isolated file substitutions against the frozen provisional component."""
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = Path('/workdir/openwepp-experiments/b01-wb14-cadence')
ORIGINAL = ROOT / 'corrected-recorder-export-20260916-1'
BINARY = ROOT / 'frozen-provisional-006.frozen'
SELECTOR = ('snow_stage3_v11_current_context_capture::member_backed_restore_tests::'
            'snow_free_member_export_reconstructs_provisional_on_clone_from_pinned_seed')


def encoded(value):
    return json.dumps(value, separators=(',', ':'), ensure_ascii=False).encode()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def limits():
    resource.setrlimit(resource.RLIMIT_AS, (16 * 1024**3, 16 * 1024**3))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def main():
    receipt = json.loads((HERE / 'frozen-provisional-006.json').read_text())
    with BINARY.open('rb') as stream:
        assert hashlib.file_digest(stream, 'sha256').hexdigest() == receipt['binary_sha256']
    plans = [
        ('clock_omit_slab', 'ERR-CT-015 invalid restart state'),
        ('clock_reorder_slabs', 'ERR-CT-015 invalid restart state'),
        ('clock_wrong_parent_support', 'ERR-CT-015 invalid restart state'),
        ('provisional_wrong_owner', 'assertion `left == right` failed'),
    ]
    planned = dict(binary=str(BINARY), binary_sha256=receipt['binary_sha256'],
                   source_tree_sha256=receipt['source_tree_sha256'],
                   cases=plans, rule='exit101 + declared semantic predicate; JSON/hash failures do not pass')
    (HERE / 'clock-controls-plan.json').write_text(json.dumps(planned, indent=2) + '\n')
    results = []
    for name, predicate in plans:
        derived = ROOT / ('native-context-control-' + name)
        derived.mkdir()
        (derived / 'rows').mkdir()
        for file in (ORIGINAL / 'rows').iterdir():
            (derived / 'rows' / file.name).symlink_to(file)
        summary = json.loads((ORIGINAL / 'summary.json').read_text())
        target = next(row for row in summary['rows'] if row['ordinal'] == 123091)
        member = 'rows/123091.29.json' if name == 'provisional_wrong_owner' else 'rows/123091.12.json'
        value = json.loads((ORIGINAL / member).read_text())
        if name == 'provisional_wrong_owner':
            value['constraint_source_owner_id'] = 'v11-snow-covered-real-consumer'
        else:
            canonical = bytes(value)
            clock = json.loads(canonical)
            assert encoded(clock) == canonical, 'positive canonical round trip prerequisite'
            if name == 'clock_omit_slab':
                clock['accepted_slab_receipts'].pop()
            elif name == 'clock_reorder_slabs':
                clock['accepted_slab_receipts'].reverse()
            else:
                clock['parent_support']['end_ns'] = str(int(clock['parent_support']['end_ns']) + 1)
            value = list(encoded(clock))
        changed = encoded(value)
        path = derived / member
        path.unlink()  # unlink the derived symlink, never its original target
        path.write_bytes(changed)
        events = []
        for line in (ORIGINAL / target['event_file']).read_text().splitlines():
            event = json.loads(line)
            if event.get('event') == 'external_json_end' and event.get('file') == member:
                event.update(bytes=len(changed), sha256=sha(changed))
            events.append(event)
        event_bytes = b''.join(encoded(event) + b'\n' for event in events)
        event_path = derived / target['event_file']
        event_path.unlink()
        event_path.write_bytes(event_bytes)
        target.update(event_file_bytes=len(event_bytes), event_file_sha256=sha(event_bytes))
        for item in target.get('external_json', []):
            if item['file'] == member:
                item.update(bytes=len(changed), sha256=sha(changed))
        (derived / 'summary.json').write_bytes(encoded(summary))
        env = {k: v for k, v in os.environ.items() if not k.startswith('OPENWEPP_')}
        env.update(OPENWEPP_B01_MEMBER_EXPORT=str(derived),
                   OPENWEPP_B01_PINNED_SEED=str(HERE / 'pinned-original-seed.json'),
                   RUST_MIN_STACK='67108864')
        argv = ['/usr/bin/timeout', '--kill-after=5s', '60s', str(BINARY),
                SELECTOR, '--exact', '--nocapture', '--test-threads=1']
        started = time.monotonic()
        result = subprocess.run(argv, env=env, capture_output=True, preexec_fn=limits)
        (HERE / (name + '.stdout')).write_bytes(result.stdout)
        (HERE / (name + '.stderr')).write_bytes(result.stderr)
        actual = result.stderr.decode()
        passed = result.returncode == 101 and predicate in actual and not any(
            text in actual for text in ('expected a sequence', 'expected a map', 'event identity', 'external member identity'))
        results.append(dict(case=name, argv=argv, export=str(derived), exit_code=result.returncode,
                            elapsed_seconds=time.monotonic() - started,
                            expected_predicate=predicate, semantic_control_pass=passed,
                            changed_member=member, changed_member_sha256=sha(changed),
                            qualification='diagnostic equality refusal' if name.startswith('provisional') else 'typed canonical restart refusal'))
        # Preserve the small changed bytes; the original corpus remains local-only.
        (HERE / (name + '-member.json')).write_bytes(changed)
    (HERE / 'clock-controls-results.json').write_text(json.dumps(results, indent=2) + '\n')
    print(json.dumps(results, indent=2))
    raise SystemExit(0 if all(row['semantic_control_pass'] for row in results) else 1)


if __name__ == '__main__':
    main()
