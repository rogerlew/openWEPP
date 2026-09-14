"""Preserve the stopped diagnostic Rust cut against its retained baseline."""
import datetime
import difflib
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys

BASE = Path('/workdir/openwepp-experiments/b01-wb14-cadence/baseline-red')
SOURCE = Path('/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913')
OUT = Path(__file__).resolve().parent


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    name = sys.argv[1]
    if not name.replace('-', '').isalnum():
        raise ValueError('simple unique evidence name required')
    patch_path = OUT / (name + '-source.patch')
    record_path = OUT / (name + '-source-recovery.json')
    if patch_path.exists() or record_path.exists():
        raise FileExistsError(name)
    differences = []
    patch = []
    for directory in ('crates', 'open_wepp_runner', 'src', 'tests', 'tools'):
        names = {p.relative_to(BASE) for p in (BASE / directory).rglob('*') if p.is_file()}
        names |= {p.relative_to(SOURCE) for p in (SOURCE / directory).rglob('*') if p.is_file()}
        for path in sorted(names):
            bp, sp = BASE / path, SOURCE / path
            before = bp.read_bytes() if bp.exists() else None
            after = sp.read_bytes() if sp.exists() else None
            if before == after:
                continue
            differences.append(dict(path=str(path), baseline_sha256=sha(before) if before is not None else None,
                                    capture_sha256=sha(after) if after is not None else None))
            patch.extend(difflib.unified_diff((before or b'').decode().splitlines(True),
                         (after or b'').decode().splitlines(True),
                         fromfile='a/' + str(path) if before is not None else '/dev/null',
                         tofile='b/' + str(path) if after is not None else '/dev/null'))
    patch_path.write_text(''.join(patch))
    recovery = Path('/workdir/openwepp-experiments/b01-wb14-cadence/logs') / ('file-importer-' + name + '-reconstruction')
    recovery.mkdir(exist_ok=False)
    for entry in differences:
        if entry['baseline_sha256'] is not None:
            destination = recovery / entry['path']
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(BASE / entry['path'], destination)
    applied = subprocess.run(['patch', '-p1', '-i', str(patch_path)], cwd=recovery, capture_output=True, text=True)
    matches = {}
    for entry in differences:
        p = recovery / entry['path']
        matches[entry['path']] = (sha(p.read_bytes()) if p.exists() else None) == entry['capture_sha256']
    record = dict(evidence_class='Ran: exact baseline-plus-patch subset reconstruction, not behavioral qualification',
                  utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), base=str(BASE), source=str(SOURCE),
                  reconstructed_subset=str(recovery), files=differences, patch_sha256=sha(patch_path.read_bytes()),
                  patch_exit=applied.returncode, patch_stdout=applied.stdout, patch_stderr=applied.stderr,
                  reconstructed_matches=matches)
    record_path.write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(dict(files=len(differences), patch_exit=applied.returncode, all_match=all(matches.values()), record=str(record_path))))
    if applied.returncode or not all(matches.values()):
        raise RuntimeError('reconstruction failed; retained evidence is not qualified')


if __name__ == '__main__':
    main()
