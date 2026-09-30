"""Record an explicitly selected command for the isolated cost attribution experiment."""
import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time

HERE = Path(__file__).resolve().parent
CUTOFF = dt.datetime.fromisoformat('2026-09-30T05:52:00+00:00')


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def snapshot(root):
    result = {}
    for base, dirs, files in os.walk(root, followlinks=False):
        dirs[:] = sorted(d for d in dirs if d not in ('target', '.git'))
        for name in sorted(files + [d for d in dirs if (Path(base) / d).is_symlink()]):
            p = Path(base) / name
            result[str(p.relative_to(root))] = (
                {'link': os.readlink(p)} if p.is_symlink() else digest(p)
            )
    return result


def main():
    p = argparse.ArgumentParser()
    p.add_argument('label')
    p.add_argument('--source', required=True, type=Path)
    p.add_argument('--binary', type=Path)
    p.add_argument('--timeout', type=int, default=180)
    p.add_argument('--pins', type=Path, required=True)
    args, command = p.parse_known_args()
    if command[:1] == ['--']:
        command = command[1:]
    if not command or not 0 < args.timeout <= 180:
        p.error('explicit argv and 1..180 second bound required')
    if (CUTOFF - dt.datetime.now(dt.timezone.utc)).total_seconds() < args.timeout:
        raise SystemExit('Command cannot fit before work cutoff')
    paths = {s: HERE / (args.label + s) for s in ('.json', '.stdout', '.stderr')}
    if any(v.exists() for v in paths.values()):
        raise SystemExit('Attempt already exists; no overwrite or automatic retry')
    pins = json.loads(args.pins.read_text())
    for path, expected in pins.items():
        if digest(path) != expected:
            raise SystemExit('Changed pinned input: ' + path)
    before = snapshot(args.source)
    receipt = dict(argv=command, cwd=str(args.source), source_entries=before,
                   source_sha256=hashlib.sha256(json.dumps(before, sort_keys=True).encode()).hexdigest(),
                   external_pins=pins, recorder_sha256=digest(__file__),
                   start_utc=dt.datetime.now(dt.timezone.utc).isoformat(), timeout=args.timeout,
                   binary=str(args.binary) if args.binary else None,
                   binary_sha256=digest(args.binary) if args.binary else None,
                   environment={k: os.environ.get(k) for k in (
                       'PATH', 'RUSTFLAGS', 'RUSTC', 'RUSTUP_TOOLCHAIN', 'CARGO_TARGET_DIR',
                       'CARGO_BUILD_JOBS', 'LD_LIBRARY_PATH', 'NIX_CONFIG', 'LANG')})
    paths['.json'].write_text(json.dumps(receipt, indent=2) + '\n')
    start = time.monotonic()
    with paths['.stdout'].open('xb') as out, paths['.stderr'].open('xb') as err:
        child = subprocess.Popen(command, cwd=args.source, stdout=out, stderr=err, start_new_session=True)
        try:
            code = child.wait(timeout=args.timeout)
        except subprocess.TimeoutExpired:
            os.killpg(child.pid, signal.SIGTERM)
            try:
                child.wait(timeout=3)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait()
            code = 124
    unchanged = snapshot(args.source) == before
    pins_unchanged = all(digest(path) == expected for path, expected in pins.items())
    binary_unchanged = not args.binary or digest(args.binary) == receipt['binary_sha256']
    receipt.update(exit_code=code, elapsed_seconds=time.monotonic() - start,
                   end_utc=dt.datetime.now(dt.timezone.utc).isoformat(),
                   source_unchanged=unchanged, pins_unchanged=pins_unchanged,
                   binary_unchanged=binary_unchanged,
                   stdout_sha256=digest(paths['.stdout']), stderr_sha256=digest(paths['.stderr']))
    paths['.json'].write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps({k: receipt[k] for k in ('exit_code', 'elapsed_seconds', 'source_unchanged', 'pins_unchanged', 'binary_unchanged')}))
    if not (unchanged and pins_unchanged and binary_unchanged):
        raise SystemExit('INVALID: source/input mutation; stop experiment')
    raise SystemExit(code)


if __name__ == '__main__':
    main()
