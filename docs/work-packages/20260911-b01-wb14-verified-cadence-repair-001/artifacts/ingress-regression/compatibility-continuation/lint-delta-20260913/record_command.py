"""Record a directly selected command; no policy selection or automatic retries."""
import datetime
import hashlib
import json
import os
import pathlib
import subprocess
import sys

OUT = pathlib.Path(__file__).resolve().parent
REPO = pathlib.Path('/workdir/openWEPP')
name, source_arg, target, *command = sys.argv[1:]
source = pathlib.Path(source_arg)
identity = json.loads((OUT / 'source-reference.json').read_text())
paths = list(identity['candidate_initial_hashes'])
paths += ['flake.nix', 'flake.lock']
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

argv = ['nix', 'develop', str(REPO), '--command', 'env',
        'CARGO_TARGET_DIR=' + target, 'RUST_MIN_STACK=67108864'] + command
record = dict(name=name, argv=argv, cwd=str(source), effective_target=target,
              source_hashes={p: digest(source / p) for p in paths if (source / p).exists()},
              missing_identity_paths=[p for p in paths if not (source / p).exists()],
              nix_hashes={p: digest(REPO / p) for p in ['flake.nix', 'flake.lock']},
              compiler_environment={k: os.environ.get(k) for k in
                  ['RUSTFLAGS', 'RUSTDOCFLAGS', 'CARGO_ENCODED_RUSTFLAGS',
                   'RUSTC', 'RUSTC_WRAPPER', 'RUSTUP_TOOLCHAIN']},
              start_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
              stdout_path=str(OUT / (name + '.stdout.log')),
              stderr_path=str(OUT / (name + '.stderr.log')))
record_path = OUT / (name + '.json')
with open(record['stdout_path'], 'xb') as stdout, open(record['stderr_path'], 'xb') as stderr:
    with record_path.open('x') as stream:
        json.dump(record, stream, indent=2)
        stream.write('\n')
    result = subprocess.run(argv, cwd=source, stdout=stdout, stderr=stderr)
record.update(exit=result.returncode,
              end_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
              source_unchanged=all(digest(source / p) == value
                                   for p, value in record['source_hashes'].items()),
              log_sha256={key: digest(pathlib.Path(record[key]))
                          for key in ['stdout_path', 'stderr_path']})
record_path.write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps({'name': name, 'exit': result.returncode,
                  'source_unchanged': record['source_unchanged'], 'record': str(record_path)}))
sys.exit(result.returncode)
