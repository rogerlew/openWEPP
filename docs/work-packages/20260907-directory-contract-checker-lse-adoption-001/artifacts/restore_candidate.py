"""Reconstruct frozen candidate in a NEW external structural/unit fixture tree.

This does not adopt LSE, modify the checkout, or reconstruct a Cargo workspace.
For full Rust reproduction use the complete named baseline plus archived overlay.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    artifacts = Path(__file__).resolve().parent
    repository = artifacts.parents[3]
    output = args.output.absolute()
    if output.exists() or output.is_symlink() or output.resolve().is_relative_to(repository):
        parser.error('output must be a new path outside the repository')
    manifest = json.loads((artifacts / 'candidate-recovery.json').read_text())
    prepared = []
    for group in ('baseline_dependencies', 'overlay'):
        for item in manifest[group]:
            relative = Path(item['path'])
            if relative.is_absolute() or '..' in relative.parts:
                parser.error('unsafe manifest path')
            if group == 'overlay':
                data = (artifacts / 'candidate-tree' / relative).read_bytes()
            else:
                data = subprocess.check_output(
                    ['git', 'show', f"{item['revision']}:{item['path']}"], cwd=repository,
                )
            if len(data) != item['bytes'] or hashlib.sha256(data).hexdigest() != item['sha256']:
                parser.error(f"frozen input identity mismatch: {relative}")
            prepared.append((relative, data))
    output.mkdir(parents=True)
    # Confinement marker for the bounded parser; not a Git checkout.
    (output / '.git').mkdir()
    for relative, data in prepared:
        target = output / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
    (output / 'CANDIDATE-NOT-ADOPTED.txt').write_text(manifest['status'] + '\n')
    print(f'Restored {len(prepared)} verified files to {output}; NOT ADOPTED')


if __name__ == '__main__':
    main()
