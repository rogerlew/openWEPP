"""Execute the adopted R/D/E attribution cohort; no extra runs or retries."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import selectors
import subprocess
import sys
import time


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def run_batch(configuration, count, label, spec, output):
    command = spec['argv']
    if Path(command[0]).resolve() != Path(spec['binary']).resolve():
        raise RuntimeError('Launch the measured binary directly, without a CPU-clock wrapper')
    if sha(spec['binary']) != spec['binary_sha256']:
        raise RuntimeError('binary changed before ' + label)
    raw = output / (label + '.stdout')
    error = output / (label + '.stderr')
    receipt = dict(configuration=configuration, count=count, label=label, argv=command,
                   binary=spec['binary'], binary_sha256=spec['binary_sha256'])
    with raw.open('xb') as log, error.open('xb') as err:
        child = subprocess.Popen(command, cwd=spec['cwd'], env={**os.environ, **spec.get('env', {})},
                                 stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=err)
        selector = selectors.DefaultSelector()
        selector.register(child.stdout, selectors.EVENT_READ)
        pending = b''
        expires = time.monotonic() + 30

        def line_with(marker):
            nonlocal pending
            while True:
                while b'\n' in pending:
                    line, pending = pending.split(b'\n', 1)
                    prefix = marker.encode()
                    if line == prefix or line.startswith(prefix + b' '):
                        return line[len(prefix):].decode().strip()
                wait = expires - time.monotonic()
                if wait <= 0 or not selector.select(wait):
                    raise RuntimeError('protocol timeout: ' + marker)
                data = os.read(child.stdout.fileno(), 65536)
                if not data:
                    raise RuntimeError('premature EOF: ' + marker)
                log.write(data)
                log.flush()
                pending += data

        def send(command):
            child.stdin.write((command + '\n').encode())
            child.stdin.flush()

        try:
            reported_pid = json.loads(line_with('PC_READY'))['pid']
            if reported_pid != child.pid:
                raise RuntimeError('READY PID is not the sampled child')
            # Linux MAKE_PROCESS_CPUCLOCK(pid, CPUCLOCK_SCHED); no tick estimate.
            clock_id = ((~child.pid) << 3) | 2
            receipt.update(pid=child.pid, cpu_clock_id=clock_id,
                           cpu_clock_resolution_seconds=time.clock_getres(clock_id),
                           wall_clock_resolution_seconds=time.get_clock_info('perf_counter').resolution)

            def interval(command, marker):
                cpu_start = time.clock_gettime_ns(clock_id)
                wall_start = time.perf_counter_ns()
                send(command)
                line_with(marker)
                wall_end = time.perf_counter_ns()
                cpu_end = time.clock_gettime_ns(clock_id)
                if cpu_end < cpu_start or wall_end < wall_start:
                    raise RuntimeError('nonmonotonic clock')
                return dict(cpu_start_ns=cpu_start, cpu_end_ns=cpu_end,
                            wall_start_ns=wall_start, wall_end_ns=wall_end,
                            cpu_ns=cpu_end-cpu_start, wall_ns=wall_end-wall_start)

            receipt['initialization'] = interval('RUN_INIT ' + str(count), 'PC_INIT_END')
            receipt['proposal'] = interval('RUN_PROPOSAL', 'PC_PROP_END')
            send('DUMP')
            result = json.loads(line_with('PC_RESULT'))
            receipt['result'] = result
            attribution = result.get('cost_attribution')
            if configuration == 'E':
                if not isinstance(attribution, dict) or attribution.get('enabled') is not True:
                    raise RuntimeError('enabled attribution payload missing: ' + label)
                if attribution.get('scope') != 'complete-hierarchy':
                    raise RuntimeError('incomplete attribution hierarchy: ' + label)
            elif attribution not in (None, {'enabled': False}):
                raise RuntimeError('attribution active outside E: ' + label)
            if result.get('projection_count_mode') != 'disabled_optional_capture':
                raise RuntimeError('optional projection capture enabled during timing')
            if len(result.get('outcomes', [])) != count:
                raise RuntimeError('incomplete numerical outcomes: ' + label)
            if len(result.get('projection_counts', [])) != count:
                raise RuntimeError('incomplete projection counts: ' + label)
            if any(value != spec['expected_projection_count'] for value in result['projection_counts']):
                raise RuntimeError('unexpected actual projection count: ' + label)
            expected = spec['expected_outcome']
            if any(outcome != expected for outcome in result['outcomes']):
                raise RuntimeError('numerical mismatch against diagnostic: ' + label)
            send('EXIT')
            child.stdin.close()
            # Drain libtest's final status; JSON consumption already lies outside timing.
            while True:
                data = child.stdout.read(65536)
                if not data:
                    break
                log.write(data)
            receipt['exit_code'] = child.wait(timeout=max(0.1, expires-time.monotonic()))
            if receipt['exit_code'] != 0:
                raise RuntimeError('child failed: ' + label)
            if error.stat().st_size:
                raise RuntimeError('unexpected stderr: optional logging must be disabled in timing')
            if sha(spec['binary']) != spec['binary_sha256']:
                raise RuntimeError('binary changed during ' + label)
        except BaseException as exc:
            receipt['error'] = repr(exc)
            if child.poll() is None:
                child.terminate()
                try:
                    child.wait(timeout=1)
                except subprocess.TimeoutExpired:
                    child.kill()
                    child.wait()
            raise
        finally:
            selector.close()
            log.flush()
            receipt['stdout_sha256'] = sha(raw)
            receipt['stderr_sha256'] = sha(error)
            (output / (label + '.json')).write_text(json.dumps(receipt, indent=2) + '\n')
    return receipt


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('config', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    config = json.loads(args.config.read_text())
    configurations = ('R', 'D', 'E')
    if set(config) != set(configurations):
        raise RuntimeError('configurations must be exactly R, D, E')
    expected = config['R']['expected_outcome']
    if any(config[name]['expected_outcome'] != expected for name in configurations):
        raise RuntimeError('diagnostic R/D/E mismatch')
    args.output.mkdir(exist_ok=False)
    environment = dict(python=sys.version, platform=platform.platform(), kernel=platform.release(),
                       affinity=sorted(os.sched_getaffinity(0)), config_sha256=sha(args.config),
                       driver_sha256=sha(__file__), cpuinfo=Path('/proc/cpuinfo').read_text(),
                       wall_clock=vars(time.get_clock_info('perf_counter')),
                       overhead='Intervals include child command reading/end-marker writes and parent-wall pipe/scheduling latency; no overhead subtraction.')
    (args.output / 'environment.json').write_text(json.dumps(environment, indent=2) + '\n')
    rows = []
    for configuration in configurations:
        rows.append(run_batch(configuration, 16, 'warmup-' + configuration, config[configuration], args.output))
    orders = (('R', 'D', 'E'), ('D', 'E', 'R'), ('E', 'R', 'D'),
              ('E', 'D', 'R'), ('D', 'R', 'E'), ('R', 'E', 'D'))
    for triplet, order in enumerate(orders, start=1):
        for configuration in order:
            rows.append(run_batch(configuration, 32, 'triplet-' + str(triplet) + '-' + configuration,
                                  config[configuration], args.output))
    # Full bit outcomes, including individual initialization/input identities, are
    # compared by the separately reviewed result analysis; never timed here.
    (args.output / 'cohort.json').write_text(json.dumps(rows, indent=2) + '\n')
    print(json.dumps({'batches': len(rows), 'measurement_triplets': 6,
                      'proposals_per_configuration': 208, 'orders': orders}))


if __name__ == '__main__':
    main()
