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

FRAME_POINTER = '/direct_runtime_counters/day_frame_constructions'

def frame_work_comparison(rule):
    if (set(rule) != {'mechanism','ofe_count','provider_count','total_constructions'} or
            rule['mechanism'] != 'F-two-full-lane-seeds'):
        raise ValueError('unknown mechanism work qualification')
    n,p,d = (rule[key] for key in ('ofe_count','provider_count','total_constructions'))
    if (any(type(value) is not int or not 0 <= value <= 2**64-1 for value in (n,p,d)) or
            n not in (1,10,19) or p == 0 or 2*n*p > d):
        raise ValueError('invalid authenticated frame work operands')
    if n == 1 and (d,p) not in ((1205,400),(805,200)):
        raise ValueError('primary frame work differs from reviewed actual populations')
    return {'qualification':rule['mechanism'], 'noncarrier_constructions':d-2*n*p}

def frame_work_rule(record):
    audit = record['carrier_counts']
    provider,carrier = audit['counts'][2:4]
    p = provider['completed']
    if (audit['dropped_records'] != 0 or len(audit['counts']) != 5 or
            type(audit['dropped_records']) is not int or
            any(type(value) is not int or value < 0 for row in (provider,carrier) for value in row.values()) or
            any(row != {'started':p,'completed':p,'errors':0} for row in (provider,carrier))):
        raise ValueError('frame work requires balanced error-free provider/carrier executions')
    rule = {'mechanism':'F-two-full-lane-seeds', 'ofe_count':record['ofe_count'],
            'provider_count':p,
            'total_constructions':record['output_manifest']['direct_runtime_counters']['day_frame_constructions']}
    frame_work_comparison(rule)
    return rule

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
        if isinstance(operation, dict) and 'mechanism' in operation:
            if pointer != FRAME_POINTER or item != operation['total_constructions']:
                raise ValueError('mechanism qualification does not bind the exact raw counter')
            parent[key] = frame_work_comparison(operation)
        elif isinstance(operation, dict):
            if (pointer not in ('/binary_path', '/binary_sha256', '/binary_sidecar_path',
                                '/binary_sidecar_sha256', '/source_commit') or
                    set(operation) != {'expected_arm_identity'} or
                    item != operation['expected_arm_identity']):
                raise ValueError('unverified executable provenance exception ' + pointer)
            parent[key] = '$ARM_IDENTITY:' + pointer
        elif operation == 'invoked_utc':
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
    if FRAME_POINTER in rules and rules[FRAME_POINTER] != frame_work_rule(record):
        raise ValueError('frame qualification does not bind actual lane/provider/count operands')
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

def validate_memory(records, phases, samples, maxima, pid, repeats, cpu):
    expected_phases=('pre_fixture','pre_run','end_run','post_validation','post_drop')
    expected=[(iteration,phase) for iteration in range(repeats) for phase in expected_phases]
    if [(r['iteration'],r['phase']) for r in phases] != expected:
        raise ValueError('missing/duplicate/reordered memory phase')
    if [r['iteration'] for r in records] != list(range(repeats)):
        raise ValueError('missing/duplicate/reordered runner iteration')
    previous=-1
    for phase in phases:
        if phase['pid'] != pid or phase['monotonic_ns'] <= previous:
            raise ValueError('foreign PID or nonmonotonic memory phase')
        previous=phase['monotonic_ns']
        for key in ('rss_kib','hwm_kib'):
            if not isinstance(phase[key],int) or phase[key] <= 0:
                raise ValueError('invalid memory observation '+key)
        for key in ('Rss:','Anonymous:','Private_Clean:','Private_Dirty:','Pss_Anon:','Pss_File:'):
            value=phase['mapping_kib'][key]
            if not isinstance(value,int) or value < 0:
                raise ValueError('invalid mapping observation '+key)
    for record in records:
        group=phases[5*record['iteration']:5*(record['iteration']+1)]
        if not (group[1]['monotonic_ns'] <= record['run_start_monotonic_ns'] <
                record['run_end_monotonic_ns'] <= group[2]['monotonic_ns']):
            raise ValueError('memory phases do not bracket runner interval')
    if len(maxima)!=repeats or any(row['samples']<=0 or row['maximum_rss_kib'] is None for row in maxima):
        raise ValueError('no actual active-run memory sample')
    if not samples:
        raise ValueError('no process status samples')
    for sample in samples:
        if sample['pid']!=pid or sample['read_end_monotonic_ns'] < sample['monotonic_ns']:
            raise ValueError('foreign/malformed process sample')
        # Ignore pre-exec taskset readiness samples; after affinity is applied it must be exact.
        if sample['monotonic_ns'] >= phases[0]['monotonic_ns'] and sample['Cpus_allowed_list'] != str(cpu):
            raise ValueError('observed affinity differs from frozen CPU')

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
    arm_cwd = args.baseline_cwd if arm == 'A' else args.candidate_cwd
    with log.open('xb') as output:
        process = subprocess.Popen(command, cwd=arm_cwd, env=env, stdout=output,
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
                  command=command, cwd=str(arm_cwd), pid=process.pid,
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
        except (KeyError, IndexError, TypeError, ValueError) as error:
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
    if args.memory:
        try:
            validate_memory(records,phases,samples,result['active_sampled_maxima'],
                            process.pid,args.repeats,args.cpu)
        except (KeyError,IndexError,TypeError,ValueError) as error:
            parity_errors.append('memory schema: '+str(error))
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
    parser.add_argument('--baseline-cwd', type=Path, required=True)
    parser.add_argument('--candidate-cwd', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--ofes', type=int, choices=(1,10,19), default=1)
    parser.add_argument('--repeats', type=int, choices=(1,10), default=1)
    parser.add_argument('--pairs', type=int, choices=(3,6,12), default=12)
    parser.add_argument('--pair-offset', type=int, default=0)
    parser.add_argument('--warmups', type=int, choices=(0,2), default=2)
    parser.add_argument('--memory', action='store_true')
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    environment_record = json.loads(Path(args.environment).read_text())
    environment = environment_record.get('runtime_env', environment_record)
    if not all(isinstance(key,str) and isinstance(value,str) for key,value in environment.items()):
        raise ValueError('runtime environment must contain only string pairs')
    binaries = {'A':args.baseline, 'B':args.candidate}
    frozen = {'binary_hashes':{arm:digest(path) for arm,path in binaries.items()},
              'collector_sha256':digest(__file__),
              'environment_sha256':digest(args.environment),
              'source_manifests':{arm:json.loads(Path(path).read_text()) for arm,path in
                                  [('A',args.baseline_manifest),('B',args.candidate_manifest)]},
              'admitted':{arm:json.loads(Path(path).read_text()) for arm,path in
                          [('A',args.baseline_identity),('B',args.candidate_identity)]}}
    for arm, binary in binaries.items():
        rules = frozen['admitted'][arm]['manifest_rules']
        source=frozen['source_manifests'][arm]
        if (frozen['admitted'][arm]['source_identity'] != source['source_identity'] or
                rules['/source_commit'] != {'expected_arm_identity':source['checkout']}):
            raise ValueError('admission source identity differs from source manifest: '+arm)
        for pointer, expected in (('/binary_path',str(Path(binary).resolve())),
                                  ('/binary_sha256',digest(binary)),
                                  ('/binary_sidecar_path',str(Path(binary).resolve()) + '.json'),
                                  ('/binary_sidecar_sha256',digest(str(Path(binary).resolve()) + '.json'))):
            if rules[pointer] != {'expected_arm_identity':expected}:
                raise ValueError('admission provenance does not match executable: ' + arm + pointer)
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
