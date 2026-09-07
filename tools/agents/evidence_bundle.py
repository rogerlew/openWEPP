#!/usr/bin/env python3
"""Explicit local byte custody. No semantic acceptance or experiment execution."""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import subprocess
import sys

GROUPS = ('source', 'executable', 'fixture', 'protocol', 'result')
ENV_ALLOWLIST = {'CC', 'CXX', 'CFLAGS', 'CXXFLAGS', 'RUSTFLAGS',
                 'CARGO_PROFILE_RELEASE_LTO', 'RUST_MIN_STACK', 'LANG', 'LC_ALL'}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':')).encode()


def fingerprint(info):
    # Reads may update atime; it is not a source mutation.
    return (info.st_dev, info.st_ino, info.st_mode, info.st_size,
            info.st_mtime_ns, info.st_ctime_ns, info.st_nlink)


def relative(value):
    if not isinstance(value, str) or not value or '\\' in value or '\x00' in value:
        raise ValueError('invalid path')
    p = PurePosixPath(value)
    if p.is_absolute() or str(p) != value or any(x in ('..', '.', '.git') for x in p.parts):
        raise ValueError('unsafe archive path: ' + value)
    return p


def no_symlink_parents(path):
    for parent in [path, *path.parents]:
        if parent.is_symlink():
            raise ValueError('symlink destination/parent: ' + str(parent))


def destination(path, excluded):
    if '..' in path.parts:
        raise ValueError('noncanonical destination path')
    path = path.absolute()
    no_symlink_parents(path)
    if path.exists() or not path.parent.is_dir():
        raise ValueError('destination must be absent with existing parent')
    for root in excluded:
        if path.is_relative_to(root.resolve()) or root.resolve().is_relative_to(path):
            raise ValueError('destination overlaps source/bundle')
    return path


def git(root, *args, data=None):
    return subprocess.run(['git', '-C', str(root), *args], input=data,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True).stdout


def link_safe(path, data):
    target = data.decode('utf-8')
    if not target or target.startswith('/') or '\\' in target or '\x00' in target:
        raise ValueError('unsafe symlink')
    # Lexical confinement fails if an earlier link redirects parent traversal.
    if '..' in target.split('/'):
        raise ValueError('parent-traversing symlink')
    parts = list(PurePosixPath(path).parent.parts)
    for component in target.split('/'):
        if component == '..':
            if not parts:
                raise ValueError('external symlink')
            parts.pop()
        elif component not in ('', '.'):
            if component == '.git':
                raise ValueError('git symlink')
            parts.append(component)


def entry(path, logical):
    no_symlink_parents(path.parent)
    before = path.lstat()
    mode = stat.S_IMODE(before.st_mode)
    if stat.S_ISLNK(before.st_mode):
        data = os.readlink(path).encode()
        link_safe(logical, data)
        kind = 'symlink'
    elif stat.S_ISREG(before.st_mode):
        if before.st_nlink != 1 or mode & 0o7000:
            raise ValueError('unsupported hardlink or special mode')
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(fd, 'rb') as stream:
            opened = os.fstat(stream.fileno())
            if (opened.st_dev, opened.st_ino) != (before.st_dev, before.st_ino):
                raise ValueError('input changed during open')
            data = stream.read()
        kind = 'file'
    else:
        raise ValueError('unsupported entry')
    after = path.lstat()
    if fingerprint(before) != fingerprint(after):
        raise ValueError('input changed during capture')
    return dict(kind=kind, mode=mode, sha256=digest(data), size=len(data)), data


def git_state(root):
    return dict(head=git(root,'rev-parse','HEAD').decode().strip(),
                branch=git(root,'symbolic-ref','--quiet','--short','HEAD').decode().strip()
                if git(root,'rev-parse','--abbrev-ref','HEAD').strip() != b'HEAD' else 'DETACHED',
                status=git(root,'status','--porcelain=v1','-z','--untracked-files=all').hex(),
                index=git(root,'ls-files','--stage','-z').hex())


def capture(root, spec, dest):
    root = root.resolve()
    dest = destination(dest, [root])
    if set(spec['files']) != set(GROUPS):
        raise ValueError('all five input groups required')
    for field in ('build_command', 'toolchain', 'commands', 'external_dependencies', 'limitations', 'retention'):
        if not spec.get(field):
            raise ValueError('missing metadata: '+field)
    if set(spec.get('environment', [])) - ENV_ALLOWLIST:
        raise ValueError('environment variable not allowlisted')
    for group in GROUPS:
        if not spec['files'][group]:
            raise ValueError('empty declared input group')
    selected = {group: [str(relative(p)) for p in spec['files'][group]] for group in GROUPS}
    if any(len(v) != len(set(v)) for v in selected.values()):
        raise ValueError('duplicate selection')
    for paths in selected.values():
        for p in paths:
            if any(x == '.env' or x.startswith('.env.') or re.search(r'(credential|secret|token)', x, re.I) for x in PurePosixPath(p).parts):
                raise ValueError('sensitive input name')
    initial = git_state(root)
    def input_state():
        states = {}
        for paths in selected.values():
            for name in paths:
                path = root/name
                no_symlink_parents(path.parent)
                states[name] = fingerprint(path.lstat()) if path.exists() or path.is_symlink() else None
        return states
    initial_inputs = input_state()
    index = {}
    for row in bytes.fromhex(initial['index']).split(b'\0'):
        if not row:
            continue
        info, path = row.split(b'\t',1)
        mode, oid, stage = info.decode().split()
        name = path.decode('utf-8')
        if name in selected['source']:
            if stage != '0' or mode not in ('100644','100755','120000'):
                raise ValueError('unsupported index stage/type')
            index[name] = (mode,oid)
    blobs = {}

    def snapshot():
        entries = []
        for group, paths in selected.items():
            for name in sorted(paths):
                path = root / name
                if not path.exists() and not path.is_symlink():
                    # Only tracked deletions can be declared absent.
                    if group != 'source' or not git(root,'ls-tree','HEAD','--',name):
                        raise ValueError('missing declared input: '+name)
                    record = dict(kind='deleted', mode=0, sha256=None, size=0)
                else:
                    record, data = entry(path, name)
                    blobs[record['sha256']] = data
                entries.append(dict(group=group,path=name,**record))
        return entries

    entries = snapshot()
    staged = []
    for name,(mode,oid) in sorted(index.items()):
        data = git(root,'cat-file','blob',oid)
        if mode == '120000':
            link_safe(name,data)
        key = digest(data)
        blobs[key] = data
        staged.append(dict(path=name,mode=mode,sha256=key,size=len(data),git_oid=oid))
    if entries != snapshot() or initial != git_state(root) or initial_inputs != input_state():
        raise ValueError('source/index changed during capture')
    manifest = dict(version=1, git=initial, spec=spec, entries=entries, staged=staged,
                    environment={k:os.environ.get(k) for k in spec.get('environment',[])},
                    recovery=dict(source='declared bytes only', index='selected paths with distinct blobs',
                                  executable='captured bytes', rebuildability='NOT PROVEN',
                                  bit_reproducible='NOT PROVEN'))
    dest.mkdir()
    (dest/'blobs').mkdir()
    for key,data in sorted(blobs.items()):
        (dest/'blobs'/key).write_bytes(data)
    payload = canonical(manifest)
    (dest/'manifest.json').write_bytes(payload)
    (dest/'manifest.sha256').write_text(digest(payload)+'\n')
    verify(dest)
    return manifest


def verify(bundle):
    no_symlink_parents(bundle)
    if (bundle/'manifest.json').is_symlink() or (bundle/'manifest.sha256').is_symlink():
        raise ValueError('symlink manifest')
    data = (bundle/'manifest.json').read_bytes()
    if digest(data) != (bundle/'manifest.sha256').read_text().strip():
        raise ValueError('manifest hash mismatch')
    m = json.loads(data)
    if m['version'] != 1:
        raise ValueError('unsupported version')
    names = set()
    expected = set()
    for row in m['entries']:
        group, path = row['group'], str(relative(row['path']))
        if group not in GROUPS or (group,path) in names:
            raise ValueError('invalid/duplicate member')
        names.add((group,path))
        if row['kind'] not in ('file','symlink','deleted') or not isinstance(row['mode'],int) or row['mode'] & ~0o777:
            raise ValueError('unsupported member type/mode')
        if row['kind'] == 'deleted':
            if group != 'source' or row['sha256'] is not None or row['size'] != 0 or row['mode'] != 0:
                raise ValueError('invalid deletion')
            continue
        expected.add(row['sha256'])
    for row in m['staged']:
        path = str(relative(row['path']))
        if row['mode'] not in ('100644','100755','120000') or ('source',path) not in names:
            raise ValueError('invalid staged member')
        expected.add(row['sha256'])
    for group,path in names:
        if any((group,str(parent)) in names for parent in PurePosixPath(path).parents):
            raise ValueError('member path collision')
    no_symlink_parents(bundle/'blobs')
    if {p.name for p in (bundle/'blobs').iterdir()} != expected:
        raise ValueError('blob inventory mismatch')
    for row in [*m['entries'],*m['staged']]:
        key = row['sha256']
        if key is None:
            continue
        if not isinstance(key,str) or not re.fullmatch('[0-9a-f]{64}',key):
            raise ValueError('unsafe blob key')
        p = bundle/'blobs'/key
        if not p.is_file() or p.is_symlink():
            raise ValueError('unsupported blob')
        b = p.read_bytes()
        if digest(b) != key or len(b) != row['size']:
            raise ValueError('blob hash/size mismatch')
        if row.get('kind') == 'symlink' or row['mode'] == '120000':
            link_safe(row['path'],b)
    declared={(g,p) for g,ps in m['spec']['files'].items() for p in ps}
    if declared != names:
        raise ValueError('declared inventory mismatch')
    return m


def restore(bundle, dest):
    m = verify(bundle)
    dest = destination(dest,[bundle])
    if any((parent/'.git').exists() for parent in dest.parents):
        raise ValueError('restore destination is inside a checkout')
    dest.mkdir()
    for group in GROUPS:
        (dest/group).mkdir()
    # Files before symlinks; collisions rejected before creating destination.
    for row in sorted(m['entries'],key=lambda r:r['kind']=='symlink'):
        if row['kind']=='deleted':
            continue
        path=dest/row['group']/row['path']
        path.parent.mkdir(parents=True,exist_ok=True)
        data=(bundle/'blobs'/row['sha256']).read_bytes()
        if row['kind']=='symlink':
            path.symlink_to(data.decode())
        else:
            with path.open('xb') as stream:
                stream.write(data)
            path.chmod(row['mode'])
    source=dest/'source'
    git(source,'init','--quiet')
    for row in m['staged']:
        data=(bundle/'blobs'/row['sha256']).read_bytes()
        oid=git(source,'hash-object','-w','--stdin',data=data).decode().strip()
        git(source,'update-index','--add','--cacheinfo',row['mode'],oid,row['path'])
    (dest/'custody.json').write_bytes(canonical(m))
    return m


def identity(root, memberships):
    if set(memberships) != {'experiment','evidence_claim','publication'}:
        raise ValueError('three explicit identity memberships required')
    result={}
    for group,paths in memberships.items():
        if not paths or len(paths)!=len(set(paths)):
            raise ValueError('empty/duplicate identity membership')
        rows=[]
        for name in sorted(paths):
            relative(name)
            row,_=entry(root/name,name)
            rows.append(dict(path=name,**row))
        result[group]=dict(sha256=digest(canonical(rows)),members=rows)
    return result


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('action',choices=['capture','verify','restore','identity','compare'])
    p.add_argument('input',type=Path)
    p.add_argument('--root',type=Path,default=Path.cwd())
    p.add_argument('--destination',type=Path)
    a=p.parse_args()
    if a.action=='capture':
        result=capture(a.root,json.loads(a.input.read_text()),a.destination)
    elif a.action=='restore':
        result=restore(a.input,a.destination)
    elif a.action=='verify':
        result=verify(a.input)
    elif a.action=='identity':
        result=identity(a.root,json.loads(a.input.read_text()))
    else:
        before=json.loads(a.input.read_text()); after=json.loads(a.destination.read_text())
        result={k:before[k]!=after[k] for k in ('experiment','evidence_claim','publication')}
    print(json.dumps(result,indent=2))


if __name__=='__main__':
    try:
        main()
    except (ValueError,OSError,KeyError,TypeError,subprocess.CalledProcessError) as error:
        print('evidence_bundle: '+str(error),file=sys.stderr)
        sys.exit(2)
