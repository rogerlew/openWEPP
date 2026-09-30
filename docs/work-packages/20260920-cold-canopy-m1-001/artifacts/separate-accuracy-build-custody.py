"""Freeze or check actual detached build inputs, depfiles, protocol, and binary."""
import argparse
import hashlib
import json
from pathlib import Path
import shlex


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['freeze', 'check'])
    parser.add_argument('receipt', type=Path)
    parser.add_argument('--source', type=Path)
    parser.add_argument('--deps', type=Path)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--input', action='append', type=Path, default=[])
    args = parser.parse_args()
    if args.action == 'freeze':
        if args.receipt.exists():
            raise RuntimeError('Use a new receipt; do not overwrite a frozen cut')
        if any(v is None for v in [args.source, args.deps, args.binary]):
            parser.error('freeze requires source, deps and binary')
        root = args.source.resolve()
        files = set()
        resolutions = {}
        depfiles = sorted(args.deps.glob('*.d'))
        if not depfiles:
            raise RuntimeError('No dependency files')
        for depfile in depfiles:
            files.add(depfile.resolve())
            for line in depfile.read_text().replace('\\\n', '').splitlines():
                if not line or line.startswith('#') or ': ' not in line:
                    continue
                _, dependencies = line.split(': ', 1)
                for token in shlex.split(dependencies):
                    path = Path(token)
                    path = path if path.is_absolute() else root / path
                    if not path.is_file():
                        raise RuntimeError(f'Unresolved dependency from {depfile}: {path}')
                    resolutions[str(path.absolute())] = str(path.resolve())
                    files.add(path.resolve())
        for path in [args.binary, *args.input]:
            if not path.is_file():
                raise RuntimeError(f'Missing explicit build/protocol input: {path}')
            resolutions[str(path.absolute())] = str(path.resolve())
            files.add(path.resolve())
        pins = {str(path): sha(path) for path in sorted(files)}
        result = {'schema': 1, 'source_root': str(root),
                  'binary_path': str(args.binary.resolve()),
                  'binary_sha256': sha(args.binary),
                  'depfile_count': len(depfiles), 'pins': pins, 'resolutions': resolutions,
                  'scope': 'Source tree separately authenticated by source-custody receipt; this pins all observed build depfiles and their resolved source dependencies plus explicit protocol/toolchain inputs. No claim of continuous host isolation.'}
        args.receipt.write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    else:
        result = json.loads(args.receipt.read_text())
    mismatches = [name for name, resolved in result['resolutions'].items()
                  if str(Path(name).resolve()) != resolved]
    for name, expected in result['pins'].items():
        path = Path(name)
        if not path.is_file() or sha(path) != expected:
            mismatches.append(name)
    if mismatches:
        raise RuntimeError('Frozen build/input drift: ' + json.dumps(mismatches))
    print(json.dumps({'verified': True, 'pins': len(result['pins']),
                      'depfile_count': result['depfile_count'],
                      'binary_sha256': result['binary_sha256']}))


if __name__ == '__main__':
    main()
