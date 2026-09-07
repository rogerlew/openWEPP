#!/usr/bin/env python3
"""Reconstruct a prospective source kit in fresh scratch; never builds or edits Git."""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import stat
import subprocess
import tarfile
import tempfile


def digest(data):
    return hashlib.sha256(data).hexdigest()


def executable_input(path, compiler_docs=()):
    return (path.startswith(('crates/', 'src/', 'tests/', '.cargo/', '.config/', 'tools/'))
            or path in compiler_docs
            or path in ('Cargo.toml', 'Cargo.lock', 'flake.nix', 'flake.lock',
                        'rust-toolchain', 'rust-toolchain.toml', 'build.rs'))


def safe_path(path):
    value = PurePosixPath(path)
    if value.is_absolute() or '..' in value.parts or str(value) != path:
        raise ValueError('unsafe or noncanonical path: ' + path)
    return value


def extract(archive, root):
    def checked(member, destination):
        safe_path(member.name.rstrip('/'))
        if not (member.isfile() or member.isdir() or member.issym()):
            raise ValueError('unsupported archive entry: ' + member.name)
        filtered = tarfile.data_filter(member, destination)
        # data_filter validates link containment but removes group-write mode.
        # Preserve the supplied ordinary permission bits for exact mode verification.
        if filtered is not None and not member.issym():
            filtered = filtered.replace(mode=member.mode & 0o777)
        return filtered
    archive.extractall(root, filter=checked)


def verify(repo, kit, restore_permissions):
    metadata = json.loads((kit / 'source-manifest.json').read_text())
    compiler_docs = metadata.get('compiler_document_inputs', [])
    if compiler_docs != sorted(set(compiler_docs)):
        raise ValueError('compiler document catalogue is not sorted and unique')
    for path in compiler_docs:
        safe_path(path)
        if not path.startswith('docs/'):
            raise ValueError('compiler document outside explicit docs scope')
    included = lambda path: executable_input(path, compiler_docs)
    encoded = (kit / metadata['identity_inputs']).read_bytes().rstrip(b'\n')
    if digest(encoded) != metadata['source_identity']:
        raise ValueError('manifest source identity mismatch')
    rows = json.loads(encoded)
    paths = [row['path'] for row in rows]
    if paths != sorted(set(paths)):
        raise ValueError('manifest paths are not sorted and unique')
    for path in paths:
        safe_path(path)
        if not included(path):
            raise ValueError('manifest path outside declared input scope: ' + path)
    if not set(compiler_docs).issubset(paths):
        raise ValueError('compiler document missing from content identity')
    patch = kit / 'tracked-source.patch'
    if digest(patch.read_bytes()) != metadata['patch_sha256']:
        raise ValueError('patch identity mismatch')
    base = subprocess.check_output(
        ['git', '-C', str(repo), 'rev-parse', metadata['base'] + '^{commit}'], text=True).strip()
    if base != metadata['base']:
        raise ValueError('base must be a full exact commit identity')
    tracked = subprocess.check_output(
        ['git', '-C', str(repo), 'ls-tree', '-r', '--name-only', '-z', base]).decode().split('\0')
    base_paths = {path for path in tracked if included(path)}
    roots = sorted({path if path in compiler_docs else path.split('/')[0]
                    for path in base_paths})
    scratch = Path(tempfile.mkdtemp(prefix='openwepp-source-kit-verify-'))
    source = scratch / 'source'
    source.mkdir()
    with tempfile.TemporaryFile() as stream:
        subprocess.run(['git', '-C', str(repo), 'archive', '--format=tar', base, '--', *roots],
                       stdout=stream, check=True)
        stream.seek(0)
        with tarfile.open(fileobj=stream) as archive:
            extract(archive, source)
    subprocess.run(['git', 'apply', '--check', str(patch)], cwd=source, check=True)
    subprocess.run(['git', 'apply', str(patch)], cwd=source, check=True)
    with tarfile.open(kit / 'new-source-files.tar.gz') as archive:
        members = {item.name for item in archive if not item.isdir()}
        if members != set(metadata['untracked_paths']):
            raise ValueError('new-source archive disagrees with untracked path manifest')
        present = {row['path'] for row in rows if row['state'] == 'present'}
        if not members.issubset(present) or any(not included(path) for path in members):
            raise ValueError('new-source archive contains an unbound source input')
        extract(archive, source)
    failures = []
    permission_restorations = []
    actual_paths = set()
    for directory, _, files in os.walk(source):
        for name in files:
            path = Path(directory) / name
            relative = path.relative_to(source).as_posix()
            if included(relative):
                actual_paths.add(relative)
    present = {row['path'] for row in rows if row['state'] == 'present'}
    deleted = {row['path'] for row in rows if row['state'] == 'deleted'}
    if present | deleted != set(paths):
        raise ValueError('unknown manifest state')
    for missing in sorted(present - actual_paths):
        failures.append({'path': missing, 'field': 'missing'})
    for extra in sorted(actual_paths - present):
        failures.append({'path': extra, 'field': 'unexpected'})
    for row in rows:
        path = source / row['path']
        if row['state'] == 'deleted':
            if path.exists() or path.is_symlink():
                failures.append({'path': row['path'], 'field': 'deletion'})
            continue
        if not path.is_file():
            continue
        current_mode = stat.S_IMODE(path.stat().st_mode)
        recorded_mode = int(row['mode'], 8)
        if (restore_permissions and not path.is_symlink()
                and current_mode != recorded_mode
                and current_mode & 0o111 == recorded_mode & 0o111):
            # Git does not retain read/write mode bits. Restore those explicit
            # source-manifest inputs, but never conceal an executable-bit error.
            path.chmod(recorded_mode)
            permission_restorations.append({'path': row['path'],
                                            'from': oct(current_mode), 'to': row['mode']})
        data = path.read_bytes()
        actual = {'bytes': len(data), 'sha256': digest(data),
                  'mode': oct(stat.S_IMODE(path.stat().st_mode)),
                  'symlink': os.readlink(path) if path.is_symlink() else None}
        for field, value in actual.items():
            if value != row[field]:
                failures.append({'path': row['path'], 'field': field,
                                 'expected': row[field], 'actual': value})
    # Explicitly reconcile base paths removed by the patch, even when a kit's
    # producer omitted a deleted tracked row after staging its removal.
    removed_base_paths = sorted(base_paths - actual_paths)
    unrecorded_deletions = sorted(set(removed_base_paths) - deleted)
    for path in unrecorded_deletions:
        failures.append({'path': path, 'field': 'unrecorded_base_deletion'})
    return {'status': 'PASS' if not failures else 'FAIL', 'name': metadata['name'],
            'base': base, 'source_identity': metadata['source_identity'],
            'manifest_rows': len(rows), 'present_rows': len(present), 'deleted_rows': len(deleted),
            'removed_base_paths': removed_base_paths, 'new_archive_paths': len(members),
            'scratch_source': str(source), 'umask': '0002',
            'permission_restorations': permission_restorations,
            'failures': failures, 'builds_executed': False,
            'scope': 'all declared executable-input paths; documentary outputs excluded'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, required=True)
    parser.add_argument('--kit', type=Path, required=True)
    parser.add_argument('--restore-recorded-permissions', action='store_true',
                        help='restore manifest read/write bits Git cannot encode; retain executable-bit checks')
    args = parser.parse_args()
    # git represents executable bits, not group-write permissions; the source
    # kit captures 0664/0775 local modes. This explicit reconstruction umask is
    # checked against every row rather than chmod-ing files to expected values.
    previous = os.umask(0o002)
    try:
        result = verify(args.repo.resolve(), args.kit.resolve(), args.restore_recorded_permissions)
    finally:
        os.umask(previous)
    print(json.dumps(result, indent=2))
    raise SystemExit(0 if result['status'] == 'PASS' else 1)


if __name__ == '__main__':
    main()
