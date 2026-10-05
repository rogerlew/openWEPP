#!/usr/bin/env python3
"""One-time source custody migration. Archives/restores/verifies; NEVER deletes store paths."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

HERE = Path(__file__).resolve().parent
DEST = Path('/workdir/openwepp-recovery/nix-snapshots-20261005')


def dump_hash(path):
    process = subprocess.Popen(['nix-store', '--dump', str(path)], stdout=subprocess.PIPE)
    digest = hashlib.sha256()
    for data in iter(lambda: process.stdout.read(1024 * 1024), b''):
        digest.update(data)
    if process.wait() != 0:
        raise RuntimeError(f'dump failed: {path}')
    return digest.hexdigest()


def main():
    DEST.mkdir(parents=True, exist_ok=True)
    if any(p.is_symlink() for p in [DEST, *DEST.parents]):
        raise RuntimeError('recovery destination may not be a symlink')
    if shutil.disk_usage(DEST).free < 50 * 2**30:
        raise RuntimeError('need at least 50 GiB recovery headroom')
    rows = json.loads((HERE / 'legacy-snapshot-inventory.json').read_text())
    for i, row in enumerate(rows, 1):
        source = Path(row['path'])
        if source.parent != Path('/nix/store') or not source.name.endswith('-source') or source.is_symlink():
            raise RuntimeError(f'invalid source: {source}')
        archive = DEST / (source.name + '.nar.zst')
        receipt = DEST / (source.name + '.json')
        if receipt.exists():
            saved = json.loads(receipt.read_text())
            if saved['source'] != str(source) or hashlib.file_digest(archive.open('rb'), 'sha256').hexdigest() != saved['archive_sha256']:
                raise RuntimeError('existing archive identity mismatch')
            print(f'{i}/{len(rows)} previously verified {source.name}', flush=True)
            continue
        if not source.is_dir() or not (source / 'crates/openwepp-kernel-contract').is_dir():
            raise RuntimeError('source disappeared or is not openWEPP')
        started = time.monotonic()
        expected = subprocess.check_output(['nix-store', '--query', '--hash', str(source)], text=True).strip()
        expected_hex = subprocess.check_output(['nix', 'hash', 'to-base16', '--type', 'sha256', expected], text=True).strip()
        partial = archive.with_suffix('.partial')
        with partial.open('xb') as output:
            dump = subprocess.Popen(['nix-store', '--dump', str(source)], stdout=subprocess.PIPE)
            compressor = subprocess.Popen(['zstd', '-q', '-T2', '-3', '-c'], stdin=dump.stdout, stdout=output)
            dump.stdout.close()
            compression_code = compressor.wait()
            dump_code = dump.wait()
            if compression_code or dump_code:
                raise RuntimeError('archive pipeline failed')
            output.flush()
            os.fsync(output.fileno())
        partial.rename(archive)
        verify = DEST / ('.restore-' + source.name)
        if verify.exists():
            raise RuntimeError('unexpected existing restore directory')
        decompressor = subprocess.Popen(['zstd', '-q', '-d', '-c', str(archive)], stdout=subprocess.PIPE)
        restore = subprocess.run(['nix-store', '--restore', str(verify)], stdin=decompressor.stdout)
        decompressor.stdout.close()
        if decompressor.wait() or restore.returncode:
            raise RuntimeError('restore failed')
        actual_hex = dump_hash(verify)
        if actual_hex != expected_hex:
            raise RuntimeError('restored source NAR hash does not match registered original')
        with archive.open('rb') as handle:
            archive_hash = hashlib.file_digest(handle, 'sha256').hexdigest()
        record = dict(source=str(source), original_nar_hash=expected, restored_nar_sha256=actual_hex,
                      archive=str(archive), archive_sha256=archive_hash, archive_bytes=archive.stat().st_size,
                      allocated_source_bytes=row['allocated_bytes'], restored_and_verified=True,
                      elapsed_seconds=round(time.monotonic()-started, 3))
        receipt.write_text(json.dumps(record, indent=2)+'\n')
        with receipt.open('rb') as handle:
            os.fsync(handle.fileno())
        shutil.rmtree(verify)
        print(f'{i}/{len(rows)} verified {source.name} archive={archive.stat().st_size/2**20:.1f}MiB seconds={record["elapsed_seconds"]}', flush=True)
    records = [json.loads(p.read_text()) for p in sorted(DEST.glob('*.json'))]
    (HERE / 'legacy-snapshot-recovery.json').write_text(json.dumps(records, indent=2)+'\n')
    print(f'COMPLETE {len(records)} source snapshots; archive bytes={sum(r["archive_bytes"] for r in records)}', flush=True)


if __name__ == '__main__':
    main()
