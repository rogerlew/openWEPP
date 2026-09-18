"""Copy explicitly scoped local evidence; omit binaries and source directories."""
import gzip
import hashlib
import json
from pathlib import Path

RAW = Path('/home/roger/openwepp-experiments/b01-wb14-publication-observer-evidence-20260918')
DEST = Path('/workdir/openWEPP/docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/publication-observer-20260918')
SUFFIXES = {'.json', '.patch', '.stdout', '.stderr', '.resources', '.py', '.md', '.txt'}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    DEST.mkdir(parents=True, exist_ok=True)
    entries = []
    for source in sorted(RAW.iterdir()):
        if not source.is_file() or source.suffix not in SUFFIXES:
            continue
        raw = source.read_bytes()
        compressed = len(raw) > 500_000
        payload = gzip.compress(raw, mtime=0) if compressed else raw
        target = DEST / (source.name + ('.gz' if compressed else ''))
        target.write_bytes(payload)
        assert (gzip.decompress(target.read_bytes()) if compressed else target.read_bytes()) == raw
        entries.append(dict(raw_path=str(source), raw_bytes=len(raw), raw_sha256=digest(raw),
                            published_path=target.name, published_bytes=len(payload),
                            published_sha256=digest(payload), gzip=compressed))
    (DEST / 'publication-manifest.json').write_text(json.dumps(dict(
        scope='Top-level evidence only; local binaries/full source trees intentionally not Git artifacts',
        entries=entries), indent=2) + '\n')
    print(json.dumps(dict(files=len(entries), published_bytes=sum(e['published_bytes'] for e in entries))))


if __name__ == '__main__':
    main()
