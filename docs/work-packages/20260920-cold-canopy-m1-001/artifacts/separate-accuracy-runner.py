#!/usr/bin/env python3
"""Execute one predeclared operation; physical execution requires reviewed custody."""
import argparse
import calendar
import hashlib
import json
import os
from pathlib import Path
import selectors
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent
INPUT = ROOT / 'separate-accuracy-input-check.json'
MEASURE = ROOT / 'separate-accuracy-measurement-draft.json'
STATE = ROOT / 'separate-accuracy-runner-state.json'
RECEIPT = ROOT / 'separate-accuracy-source-review-approved.json'
TEST = 'm1_coupled_tests::m1_separate_accuracy_diagnostic_ipc'
CONTROL_TEST = 'm1_coupled_tests::m1_separate_accuracy_six_frozen_controls'
CONTROL_FILE = ROOT / 'separate-accuracy-control-fixtures.json'
PIN_FILES = ['separate-accuracy-runner.py', 'separate-accuracy-input-check.json',
             'separate-accuracy-measurement-draft.json', 'separate-accuracy-protocol-draft.json',
             'separate-accuracy-control-fixtures.json', 'separate-accuracy-independent-reconstruction.py',
             'separate-accuracy-source-custody.py', 'separate-accuracy-build-custody.py',
             'architecture-decision-successor-cases.json', 'original-input-reference.json',
             'continuous-fixtures-draft01.json']


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False)


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(path):
    return json.loads(Path(path).read_text())


def write(path, value):
    temporary = path.with_suffix(path.suffix + '.pending')
    temporary.write_text(canonical(value) + '\n')
    temporary.replace(path)


def state():
    if STATE.exists():
        current = load(STATE)
        if current.get('in_flight'):
            raise RuntimeError('Interrupted physical operation: preserve state; no automatic retry or further launch')
        return current
    return {
        'phase': 'NEW', 'physical_seconds': 0.0, 'reference_seconds': 0.0,
        'operations': {}, 'completed': [], 'failed': [], 'results': []}


def key(kind, request, batch=0):
    return canonical({'kind': kind, 'cell': request, 'batch': batch})


def reference_requests(measure):
    cases = list(dict.fromkeys(c['case'] for c in measure['cell_sequence']))
    for case in cases:
        cell = next(c for c in measure['cell_sequence'] if c['case'] == case
                    and c['investigation'] == 'A' and c['formulation'] == 'FULL'
                    and c['profile'] == 'P0')
        yield 'reference', cell
        yield 'stability', dict(cell, stability_variant='strict_reference')


def next_operation(cur, measure):
    operations = cur['operations']
    for control in load(CONTROL_FILE)['controls']:
        request = {'control_id': control['id']}
        if key('control', request) not in operations:
            return 'control', request, 0
    for kind, request in reference_requests(measure):
        if key(kind, request) not in operations:
            return kind, request, 0
    for request in measure['cell_sequence']:
        diagnostic = operations.get(key('diagnostic', request))
        if diagnostic is None:
            return 'diagnostic', request, 0
        if diagnostic['status'] != 'complete':
            continue
        for batch in range(6):
            result = operations.get(key('timing', request, batch))
            if result is None:
                return 'timing', request, batch
            if result['status'] != 'complete':
                break
    return None


def authenticate(binary, cur, deadline=None):
    receipt = load(RECEIPT)
    if receipt.get('binary_sha256') != sha(binary):
        raise RuntimeError('receipt binary hash mismatch')
    pins = receipt.get('artifact_pins', {})
    for name in PIN_FILES:
        if pins.get(name) != sha(ROOT / name):
            raise RuntimeError('receipt artifact hash mismatch: ' + name)
    for name in ('source_archive', 'source_receipt', 'build_receipt'):
        item = receipt.get(name)
        if not isinstance(item, dict) or item.get('sha256') != sha(item['path']):
            raise RuntimeError('receipt missing or changed ' + name)
    receipt_hash = sha(RECEIPT)
    if cur.get('receipt_sha256') not in (None, receipt_hash):
        raise RuntimeError('approval receipt changed after campaign initialization')
    root = receipt['source_root']
    commands = [
        [sys.executable, str(ROOT / 'separate-accuracy-source-custody.py'), 'check',
         root, receipt['source_archive']['path'], receipt['source_receipt']['path']],
        [sys.executable, str(ROOT / 'separate-accuracy-build-custody.py'), 'check',
         receipt['build_receipt']['path']],
    ]
    for command in commands:
        remaining = 30 if deadline is None else min(30, deadline - time.monotonic())
        if remaining <= 0:
            raise TimeoutError('command deadline reached during custody')
        result = subprocess.run(command, capture_output=True, text=True, timeout=remaining)
        if result.returncode:
            raise RuntimeError('custody check failed: ' + result.stderr[-2000:])
    return receipt_hash


class Frames:
    """Read binary pipes without TextIO read-ahead/selector deadlocks."""
    def __init__(self, proc, deadline):
        self.proc, self.deadline = proc, deadline
        self.buffer = b''
        self.raw = bytearray()
        self.selector = selectors.DefaultSelector()
        self.selector.register(proc.stdout, selectors.EVENT_READ)
        os.set_blocking(proc.stdout.fileno(), False)

    def read(self, wanted=None):
        while True:
            while b'\n' in self.buffer:
                line, self.buffer = self.buffer.split(b'\n', 1)
                prefix = b'M1_SEPARATE_ACCURACY '
                if line.startswith(prefix):
                    event = json.loads(line[len(prefix):])
                    if event.get('event') != wanted:
                        raise RuntimeError('unexpected IPC event: ' + str(event.get('event')))
                    return event
            remaining = self.deadline - time.monotonic()
            if remaining <= 0 or not self.selector.select(remaining):
                raise TimeoutError('physical watchdog expired waiting for ' + str(wanted))
            chunk = os.read(self.proc.stdout.fileno(), 65536)
            if not chunk:
                if wanted is None:
                    return None
                raise RuntimeError('missing ' + wanted + ' marker; child ended')
            self.raw.extend(chunk)
            self.buffer += chunk

    def close(self):
        self.selector.close()


def send(proc, command):
    proc.stdin.write((command + '\n').encode())
    proc.stdin.flush()


def cell_used(cur, request):
    return sum(v.get('charged_seconds', 0.0) for v in cur['operations'].values()
               if v['cell'] == request and v['kind'] in ('diagnostic', 'timing', 'reference'))


def record(cur, kind, request, batch, result):
    operation_key = key(kind, request, batch)
    result.update(key=operation_key, kind=kind, cell=request, batch=batch)
    cur['operations'][operation_key] = result
    cur['results'].append(result)
    cur['completed' if result['status'] == 'complete' else 'failed'].append(operation_key)
    charge = result.get('charged_seconds', 0.0)
    cur['physical_seconds'] += charge
    if kind in ('reference', 'stability'):
        cur['reference_seconds'] += charge
    write(STATE, cur)
    print(canonical(result), flush=True)


def run(cur, request, binary, kind, batch):
    if cur['phase'] != 'READY':
        raise RuntimeError('RUN_FRESH_DAYS requires READY; integrity halt is terminal')
    measure = load(MEASURE)
    command_deadline = time.monotonic() + measure['limits']['command_seconds'] - 1.0
    if next_operation(cur, measure) != (kind, request, batch):
        raise RuntimeError('not the next frozen operation, or attempted retry')
    try:
        receipt_hash = authenticate(binary, cur, command_deadline)
    except Exception:
        cur['phase'] = 'HALTED'
        write(STATE, cur)
        raise
    cur['receipt_sha256'] = receipt_hash
    # A FULL/P0 diagnostic reuses its identical strict-reference trajectory.
    if kind == 'diagnostic' and request['formulation'] == 'FULL' and request['profile'] == 'P0':
        original = cur['operations'][key('reference', request)]
        reuse = {k: v for k, v in original.items() if k not in ('key', 'kind', 'cell', 'batch')}
        reuse.update(charged_seconds=0.0, reused_reference=original['key'])
        record(cur, kind, request, batch, reuse)
        return
    if kind == 'diagnostic' and request['investigation'] == 'B':
        match = dict(request, investigation='A', cadence_seconds=60)
        baseline = cur['operations'].get(key('diagnostic', match))
        if baseline is None or baseline['status'] != 'complete':
            record(cur, kind, request, batch, {'status': 'unavailable',
                   'reason': 'REDUCED matched-profile 60-second diagnostic did not complete'})
            return
    limits = measure['limits']
    deadline_utc = calendar.timegm(time.strptime(limits['work_cutoff_utc'], '%Y-%m-%dT%H:%M:%SZ'))
    remaining_work = deadline_utc - time.time()
    class_cap = (limits['control_seconds'] if kind == 'control' else
                 limits['reference_trajectory_seconds'] if kind == 'reference' else
                 limits['stability_trajectory_seconds'] if kind == 'stability' else
                 limits['candidate_cell_elapsed_seconds'] - cell_used(cur, request))
    # A manually adjudicated single oracle replay retains its original charge.
    # This does not recover/reset halted state or authorize any retry itself.
    namespace = cur.get('verified_defect_namespace', '')
    if namespace:
        preserved = cur.get('preserved_defect_operations', [])
        if (namespace != 'c1-oracle-replay01'
                or type(cur.get('defect_replay_consumed')) is not int
                or cur['defect_replay_consumed'] != 1
                or len(preserved) != 1
                or preserved[0]['kind'] != 'control'
                or preserved[0]['cell'] != {'control_id': 'C1_empty_admissible'}
                or cur['physical_seconds'] < preserved[0]['charged_seconds']):
            raise RuntimeError('Invalid single-control defect recovery evidence')
        if kind == 'control' and request == preserved[0]['cell']:
            class_cap -= preserved[0]['charged_seconds']
    cap = min(class_cap, limits['command_seconds'], remaining_work,
              limits['all_physical_elapsed_seconds'] - cur['physical_seconds'])
    if kind in ('reference', 'stability'):
        cap = min(cap, limits['reference_elapsed_seconds'] - cur['reference_seconds'])
    if cap <= 0:
        record(cur, kind, request, batch, {'status': 'not_run_budget', 'reason': 'frozen cap exhausted'})
        return
    # Custody work is not physical, but it cannot extend the absolute work cutoff.
    cap = min(cap, deadline_utc - time.time(), command_deadline - time.monotonic() - 2.0)
    if cap <= 0:
        raise RuntimeError('work cutoff reached during custody checks')
    test = CONTROL_TEST if kind == 'control' else TEST
    listed = subprocess.run([str(binary), '--list'], capture_output=True, text=True, timeout=min(10, cap))
    if listed.returncode or test + ': test' not in listed.stdout.splitlines():
        raise RuntimeError('binary lacks exact required test entry')
    env = os.environ.copy()
    if kind == 'control':
        env['M1_SEPARATE_ACCURACY_CONTROL_ID'] = request['control_id']
    suffix = '-' + namespace if namespace else ''
    output = ROOT / ('separate-accuracy-operation-' + hashlib.sha256(key(kind, request, batch).encode()).hexdigest()[:20] + suffix + '.json')
    if output.exists():
        raise RuntimeError('existing operation evidence; deny overwrite/retry')
    launch = time.monotonic_ns()
    # Sample boundary remains distinct from the broader launch/collection charge.
    cap = min(cap, deadline_utc - time.time(), command_deadline - time.monotonic() - 2.0)
    if cap <= 0:
        record(cur, kind, request, batch, {'status': 'not_run_budget', 'reason': 'work cutoff reached before launch'})
        return
    cur['in_flight'] = {'key': key(kind, request, batch), 'maximum_charge_seconds': cap, 'launch_utc': time.time()}
    write(STATE, cur)
    proc = None
    frames = None
    payload = {}
    result = {'status': 'integrity_failure', 'output': str(output), 'cap_seconds': cap}
    try:
        proc = subprocess.Popen([str(binary), test, '--exact', '--ignored', '--nocapture', '--test-threads=1', '--format=terse'],
                                stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                stderr=subprocess.STDOUT, env=env)
        frames = Frames(proc, launch / 1e9 + cap)
        if kind == 'control':
            proc.stdin.close()
            frames.read()
            rc = proc.wait(timeout=max(0.001, frames.deadline - time.monotonic()))
            text = frames.raw.decode(errors='replace')
            if rc or '1 passed; 0 failed' not in text:
                raise RuntimeError('fixed control failed: ' + text[-3000:])
            payload = {'control_id': request['control_id'], 'stdout': text}
            result['status'] = 'complete'
        else:
            send(proc, 'RUN_INIT ' + canonical(request))
            ready = frames.read('READY')
            expected = 86400 // request['cadence_seconds'] + int(request['case'] == 'C_freeze_then_melt_day' and request['cadence_seconds'] > 60)
            if (ready.get('pid') != proc.pid or ready.get('base_sha256') != load(INPUT)['base_sha256']
                    or ready.get('request') != request or ready.get('support_count') != expected):
                raise RuntimeError('READY identity/request/support mismatch')
            clock_id = ((~proc.pid) << 3) | 2
            clock_resolution = time.clock_getres(clock_id)
            cpu0 = time.clock_gettime_ns(clock_id)
            wall0 = time.perf_counter_ns()
            count = 4 if kind == 'timing' else 1
            send(proc, f'RUN_FRESH_DAYS {count}')
            end = frames.read('END')
            wall1 = time.perf_counter_ns()
            cpu1 = time.clock_gettime_ns(clock_id)
            if cpu1 < cpu0 or wall1 < wall0:
                raise RuntimeError('nonmonotonic sample clock')
            if end.get('support_count') != expected or not 1 <= end.get('day_count', 0) <= count:
                raise RuntimeError('END day/support mismatch')
            send(proc, 'DUMP')
            payload = frames.read('DUMP')
            proc.stdin.close()
            frames.read()
            if proc.wait(timeout=max(0.001, frames.deadline - time.monotonic())):
                raise RuntimeError('child failed after DUMP')
            days = payload.get('days')
            if not isinstance(days, list) or len(days) != end['day_count']:
                raise RuntimeError('DUMP day count mismatch')
            for day in days:
                if (not isinstance(day, dict) or not isinstance(day.get('records'), list)
                        or type(day.get('completed_supports')) is not int
                        or day['completed_supports'] != len(day['records'])
                        or not 0 <= day['completed_supports'] <= expected):
                    raise RuntimeError('DUMP partial support shape mismatch')
            failed = [d for d in days if d.get('first_failure') is not None]
            complete = not failed and len(days) == count and all(d['completed_supports'] == expected for d in days)
            if not complete and not failed:
                raise RuntimeError('incomplete day without typed numerical/domain failure')
            digests = [hashlib.sha256(canonical(d['records']).encode()).hexdigest() for d in days]
            if complete and kind == 'timing':
                diagnostic = cur['operations'][key('diagnostic', request)]
                if any(d != diagnostic['day_digests'][0] for d in digests):
                    raise RuntimeError('timed day digest differs from diagnostic')
            result.update(status='complete' if complete else 'numerical_or_domain_failure',
                          fresh_complete_days=sum(d['first_failure'] is None and d['completed_supports'] == expected for d in days),
                          day_digests=digests, first_failures=[d['first_failure'] for d in failed],
                          completed_supports=[d['completed_supports'] for d in days],
                          cpu_clock_id=clock_id, cpu_clock_resolution=clock_resolution,
                          cpu_start_ns=cpu0, cpu_end_ns=cpu1, wall_start_ns=wall0, wall_end_ns=wall1,
                          child_process_cpu_seconds=(cpu1-cpu0)/1e9, sample_wall_seconds=(wall1-wall0)/1e9)
        write(output, {'request': request, 'kind': kind, 'batch': batch, 'payload': payload,
                       'raw_stdout': frames.raw.decode(errors='replace')})
        if time.monotonic_ns() / 1e9 > frames.deadline:
            raise TimeoutError('physical cap exhausted during evidence collection')
    except TimeoutError as error:
        result.update(status='timeout', reason=str(error))
        if kind == 'control':
            cur['phase'] = 'HALTED'
    except Exception as error:
        result.update(status='integrity_failure', reason=str(error))
        cur['phase'] = 'HALTED'
    finally:
        if proc is not None and proc.poll() is None:
            proc.kill()
            proc.wait(timeout=5)
        if frames is not None:
            if not output.exists():
                write(output, {'request': request, 'payload': payload, 'raw_stdout': frames.raw.decode(errors='replace')})
            frames.close()
        result['charged_seconds'] = (time.monotonic_ns() - launch) / 1e9
    # An externally modified source/input invalidates the run, even after success.
    try:
        authenticate(binary, cur, command_deadline)
    except Exception as error:
        result.update(status='integrity_failure', reason='post-run custody: ' + str(error))
        cur['phase'] = 'HALTED'
    cur.pop('in_flight', None)
    record(cur, kind, request, batch, result)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=['READY', 'RUN_FRESH_DAYS', 'END', 'DUMP'])
    parser.add_argument('--cell')
    parser.add_argument('--test-binary', type=Path)
    parser.add_argument('--kind', choices=['diagnostic', 'timing', 'reference', 'stability', 'control'], default='diagnostic')
    parser.add_argument('--batch', type=int, default=0)
    parser.add_argument('--control-id')
    args = parser.parse_args()
    cur = state()
    if args.command == 'READY':
        if cur['phase'] not in ('NEW', 'READY'):
            raise RuntimeError('READY cannot reset an ended/halted campaign')
        cur['phase'] = 'READY'
        write(STATE, cur)
        print(canonical({'phase': cur['phase'], 'next': next_operation(cur, load(MEASURE))}))
    elif args.command == 'RUN_FRESH_DAYS':
        if args.test_binary is None:
            parser.error('--test-binary required')
        request = {'control_id': args.control_id} if args.kind == 'control' else json.loads(args.cell)
        run(cur, request, args.test_binary, args.kind, args.batch)
    elif args.command == 'END':
        cur['phase'] = 'END'
        write(STATE, cur)
        print(canonical({'phase': 'END', 'physical_seconds': cur['physical_seconds']}))
    else:
        print(canonical({'state': cur, 'next': next_operation(cur, load(MEASURE))}))


if __name__ == '__main__':
    main()
