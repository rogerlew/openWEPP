#!/usr/bin/env python3
"""Frozen fresh-process comparisons; no builds, source mutations or statistical retries."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time

TEST = 'hillslope::tests::stage3_controlled_mechanism_experiment'
ORDER = ['AB', 'BA', 'BA', 'AB', 'AB', 'BA', 'BA', 'AB', 'AB', 'BA', 'BA', 'AB']
IDENTITY_FIELDS = ('ofe_count', 'input_sha256', 'input_files', 'output_files',
                   'closure_operands', 'committed_day_count', 'total_parent_support_count',
                   'covered_parent_support_count', 'snow_free_parent_support_count',
                   'accepted_publication_support_count', 'fixed_point_evaluation_count',
                   'direct_trial_count', 'split_child_trial_count', 'accepted_microstep_count',
                   'qualification_scopes_balanced', 'qualification_counters_complete',
                   'laned_days_seen', 'laned_days_routed', 'laned_source_m3',
                   'laned_outlet_m3', 'laned_end_window_storage_m3', 'laned_clamp_m3')

def normalized_manifest(manifest, rules):
    value = json.loads(json.dumps(manifest))
    run_dir = value['run_dir']
    for pointer, operation in rules.items():
        parts = [part.replace('~1','/').replace('~0','~') for part in pointer.lstrip('/').split('/')]
        parent = value
        for part in parts[:-1]:
            parent = parent[int(part)] if isinstance(parent, list) else parent[part]
        key = int(parts[-1]) if isinstance(parent, list) else parts[-1]
        item = parent[key]
        if operation == 'invoked_utc':
            if pointer != '/invoked_utc' or not isinstance(item, str):
                raise ValueError('invalid timestamp exception')
            parent[key] = '$INVOKED_UTC'
        elif operation == 'run_path':
            if not isinstance(item,str) or not (item == run_dir or item.startswith(run_dir + '/')):
                raise ValueError('invalid exact path exception ' + pointer)
            parent[key] = '$RUN_DIR' + item[len(run_dir):]
        elif operation == 'run_path_keys':
            if not isinstance(item,dict):
                raise ValueError('invalid path-key map ' + pointer)
            replacement = {}
            for path, content in item.items():
                if not (path == run_dir or path.startswith(run_dir + '/')):
                    raise ValueError('non-run path in exception map ' + pointer)
                replacement['$RUN_DIR' + path[len(run_dir):]] = content
            parent[key] = replacement
        else:
            raise ValueError('unknown volatile exception ' + operation)
    return value

def record_identity(record, rules):
    identity = {field:record[field] for field in IDENTITY_FIELDS}
    identity['manifest'] = normalized_manifest(record['output_manifest'], rules)
    return identity

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def exact_json(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False)

def status(pid):
    text = Path(f'/proc/{pid}/status').read_text()
    values = {}
    for line in text.splitlines():
        key, _, value = line.partition(':')
        if key in ('VmRSS', 'VmHWM', 'RssAnon', 'RssFile', 'RssShmem'):
            values[key + '_kib'] = int(value.split()[0])
        elif key == 'Cpus_allowed_list':
            values[key] = value.strip()
    return values

def run(args, arm, pair, label, binary, environment, frozen):
    binary = Path(binary).resolve()
    binary_hash = digest(binary)
    if binary_hash != frozen['binary_hashes'][arm]:
        raise RuntimeError('executable changed since series freeze; start a new series')
    name = f'{args.series}-{label}-{pair:02d}-{arm}'
    log = args.out / (name + '.log')
    env = dict(environment)
    env.update(RUST_MIN_STACK='67108864', CARGO_PROFILE_RELEASE_LTO='false',
               OPENWEPP_EXPERIMENT_OFES=str(args.ofes),
               OPENWEPP_EXPERIMENT_REPEATS=str(args.repeats),
               OPENWEPP_EXPERIMENT_MEMORY='1' if args.memory else '0')
    env.pop('OPENWEPP_EXPERIMENT_TRACE_DIR', None)
    command = ['taskset', '-c', str(args.cpu), str(binary), TEST,
               '--ignored', '--exact', '--nocapture', '--test-threads=1']
    samples, ready = [], False
    started = time.monotonic_ns()
    timeout = 1800 if args.repeats == 10 else 600
    timed_out = False
    with log.open('xb') as output:
        process = subprocess.Popen(command, cwd=args.cwd, env=env, stdout=output,
                                   stderr=subprocess.STDOUT, start_new_session=True)
        next_sample = started
        while True:
            pid, wait_status, usage = os.wait4(process.pid, os.WNOHANG)
            if pid:
                process.returncode = os.waitstatus_to_exitcode(wait_status)
                break
            now = time.monotonic_ns()
            if now - started > timeout * 1_000_000_000:
                os.killpg(process.pid, signal.SIGKILL)
                timed_out = True
                pid, wait_status, usage = os.wait4(process.pid, 0)
                process.returncode = os.waitstatus_to_exitcode(wait_status)
                break
            if not ready:
                try:
                    ready = Path(f'/proc/{process.pid}/exe').resolve() == binary
                except (FileNotFoundError, PermissionError):
                    pass
            if args.memory and now >= next_sample:
                try:
                    values = status(process.pid)
                    samples.append({'monotonic_ns': now, 'read_end_monotonic_ns':time.monotonic_ns(),
                                    'pid': process.pid, **values})
                except FileNotFoundError:
                    pass
                next_sample = now + 100_000_000
            time.sleep(0.02 if args.memory else 0.1)
    ended = time.monotonic_ns()
    records, phases, parse_errors = [], [], []
    for line in log.read_text().splitlines():
        for marker, destination in [('STAGE3_CONTROLLED_MECHANISM ', records),
                                    ('STAGE3_CONTROLLED_MEMORY ', phases)]:
            if marker in line:
                try:
                    destination.append(json.loads(line.split(marker, 1)[1]))
                except json.JSONDecodeError as error:
                    parse_errors.append(str(error))
    result = dict(series=args.series, arm=arm, pair=pair, label=label,
                  executable=str(binary), executable_sha256=binary_hash,
                  command=command, cwd=str(args.cwd), pid=process.pid,
                  start_monotonic_ns=started, end_monotonic_ns=ended,
                  process_wall_s=(ended-started)/1e9, process_user_s=usage.ru_utime,
                  process_system_s=usage.ru_stime, lifetime_peak_rss_kib=usage.ru_maxrss,
                  exit_code=process.returncode, timeout=timed_out, ready=ready,
                  log=str(log), log_sha256=digest(log), records=records, phases=phases,
                  samples=samples, observer_period_ms=100 if args.memory else None,
                  observer_requested=args.memory, ofes=args.ofes, repeats=args.repeats,
                  parse_errors=parse_errors)
    result['frozen_series'] = frozen
    parity_errors = []
    admitted = frozen['admitted'][arm]
    for record in records:
        try:
            if exact_json(record_identity(record, admitted['manifest_rules'])) != exact_json(admitted['common_identity']):
                parity_errors.append('scientific/input/output/control identity mismatch')
            for field in ('carrier_counts', 'lse_counts'):
                if exact_json(record[field]) != exact_json(admitted[field]):
                    parity_errors.append(field + ' differs from admitted arm')
        except (KeyError, TypeError, ValueError) as error:
            parity_errors.append(str(error))
    result['parity_errors'] = parity_errors
    result['active_sampled_maxima'] = []
    for record in records:
        try:
            run_start, run_end = record['run_start_monotonic_ns'], record['run_end_monotonic_ns']
            if not isinstance(run_start,int) or not isinstance(run_end,int) or run_end < run_start:
                raise ValueError('invalid monotonic runner interval')
            active = [sample for sample in samples if
                      run_start <= sample['monotonic_ns'] and sample['read_end_monotonic_ns'] <= run_end]
            result['active_sampled_maxima'].append({
                'iteration':record['iteration'], 'samples':len(active),
                'maximum_rss_kib':max((sample['VmRSS_kib'] for sample in active), default=None),
                'clock':'CLOCK_MONOTONIC; entire status-read bracket inside run interval'})
        except (KeyError, TypeError, ValueError) as error:
            parity_errors.append('interval schema: ' + str(error))
    result['valid'] = (process.returncode == 0 and not timed_out and ready and not parse_errors and not parity_errors
                       and len(records) == args.repeats and digest(binary) == binary_hash
                       and (not args.memory or (len(phases) == 5 * args.repeats and bool(samples))))
    with (args.out / 'results.jsonl').open('a') as out:
        out.write(json.dumps(result, separators=(',', ':')) + '\n')
    print(json.dumps({k: result[k] for k in ('series','arm','pair','label','exit_code','valid')}), flush=True)
    if not result['valid']:
        raise RuntimeError(f'invalid process retained: {log}; series stops')

def main():
    parser = argparse.ArgumentParser()
    for field in ('baseline', 'candidate', 'environment', 'series', 'baseline-manifest', 'candidate-manifest', 'baseline-identity', 'candidate-identity'):
        parser.add_argument('--' + field, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--cwd', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--ofes', type=int, choices=(1,10,19), default=1)
    parser.add_argument('--repeats', type=int, choices=(1,10), default=1)
    parser.add_argument('--pairs', type=int, choices=(3,6,12), default=12)
    parser.add_argument('--pair-offset', type=int, default=0)
    parser.add_argument('--warmups', type=int, choices=(0,2), default=2)
    parser.add_argument('--memory', action='store_true')
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    environment = json.loads(Path(args.environment).read_text())
    binaries = {'A':args.baseline, 'B':args.candidate}
    frozen = {'binary_hashes':{arm:digest(path) for arm,path in binaries.items()},
              'environment_sha256':digest(args.environment),
              'source_manifests':{arm:json.loads(Path(path).read_text()) for arm,path in
                                  [('A',args.baseline_manifest),('B',args.candidate_manifest)]},
              'admitted':{arm:json.loads(Path(path).read_text()) for arm,path in
                          [('A',args.baseline_identity),('B',args.candidate_identity)]}}
    if exact_json(frozen['admitted']['A']['common_identity']) != exact_json(frozen['admitted']['B']['common_identity']):
        raise ValueError('arms not admitted with exact scientific/input/output/control parity')
    with (args.out / (args.series + '-series-freeze.json')).open('x') as out:
        json.dump(frozen, out, indent=2)
    for number in range(args.warmups):
        for arm in 'AB':
            run(args, arm, number, 'warmup', binaries[arm], environment, frozen)
    for number in range(args.pairs):
        for arm in ORDER[number]:
            run(args, arm, number+args.pair_offset, 'measured', binaries[arm], environment, frozen)

if __name__ == '__main__':
    main()
