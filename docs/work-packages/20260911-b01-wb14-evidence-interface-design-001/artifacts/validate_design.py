"""Static design-document checks; never executes prospective Rust commands."""
import ast
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
PKG = Path(__file__).resolve().parent.parent
PIN = 'a0af6afac5caa6e84e2a782948c658f9a7b8dff3'
checks = []

def check(name, condition, **details):
    checks.append(dict(check=name, status='PASS' if condition else 'FAIL', **details))
    if not condition:
        raise ValueError(f'{name}: {details}')

retired_path = PKG / 'artifacts/retired-design-draft.md.gz'
if retired_path.exists():
    retired_identity = json.loads((PKG / 'artifacts/retired-design-draft-identity.json').read_text())
    compressed = retired_path.read_bytes()
    raw = gzip.decompress(compressed)
    check('Lossless retired draft custody',
          len(raw) == retired_identity['raw_bytes']
          and hashlib.sha256(raw).hexdigest() == retired_identity['raw_sha256']
          and hashlib.sha256(compressed).hexdigest() == retired_identity['gzip_sha256'])

for path in sorted((PKG / 'artifacts').glob('*.json')):
    json.loads(path.read_text())
    check('JSON syntax', True, path=str(path.relative_to(ROOT)))
for path in sorted((PKG / 'artifacts').glob('*.py')):
    ast.parse(path.read_text())
    check('Python syntax', True, path=str(path.relative_to(ROOT)))
for path in (PKG / 'package.md', ROOT / 'docs/work-packages/active.md', ROOT / 'docs/work-packages/README.md'):
    missing = []
    for link in re.findall(r'\]\(([^)]+)\)', path.read_text()):
        if '://' in link or link.startswith('#'):
            continue
        if path.parent != PKG and PKG.name not in link:
            continue
        target = link.split('#')[0]
        if target and not (path.parent / target).exists():
            missing.append(link)
    check('Owned local link targets', not missing, path=str(path.relative_to(ROOT)), missing=missing)
record = (PKG / 'package.md').read_text()
for number, block in enumerate(re.findall(r'```(?:bash|sh)\n(.*?)```', record, re.S)):
    result = subprocess.run(['bash', '-n'], input=block, text=True, capture_output=True, cwd=ROOT)
    check('Prospective shell syntax ONLY', result.returncode == 0, block=number, stderr=result.stderr)
authorities = [
    'AGENTS.md', 'docs/work-packages/AGENTS.md', 'docs/work-packages/role-orchestration.md',
    'docs/work-packages/role-review.md', 'docs/standards/bounded-agent-execution.md',
    'docs/standards/testing-and-gate-strategy.md', 'docs/standards/kernel-work-package-preparation.md',
    'docs/specifications/science-contracts/AGENTS.md',
    'docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md',
    *['docs/specifications/science-contracts/contracts/SC-' + name + '-001.md'
      for name in ('SURFACELIQUID', 'COUPLEDTIME', 'SNOWENERGY')],
    'docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts/gradual-warm-tail7-cases.json',
]
for relative in authorities:
    actual = (ROOT / relative).read_bytes()
    pinned = subprocess.check_output(['git', 'show', PIN + ':' + relative], cwd=ROOT)
    check('Pinned authority/input bytes', actual == pinned, path=relative, sha256=hashlib.sha256(actual).hexdigest())
identity_path = PKG / 'artifacts/selected-source-identity.json'
if identity_path.exists():
    identity = json.loads(identity_path.read_text())
    source = Path(identity['source_root'])
    prior_path = ROOT / 'docs/work-packages/20260911-b01-wb14-source-reconciliation-001/artifacts/fresh-recipe-whole-source-comparison.json'
    prior = json.loads(prior_path.read_text())['fresh_regular_file_sha256']
    for relative, recorded in identity['selected_regular_files'].items():
        path = source / relative
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        check('Selected runtime source matches prior inventory',
              actual == recorded == prior.get(relative), path=relative,
              actual_sha256=actual, recorded_sha256=recorded, prior_sha256=prior.get(relative))
    for relative, recorded in identity.get('additional_relied_on_inputs_and_helpers', {}).items():
        if relative.startswith('crates/'):
            actual = hashlib.sha256((source / relative).read_bytes()).hexdigest()
            expected = prior.get(relative)
        else:
            actual = hashlib.sha256((ROOT / relative).read_bytes()).hexdigest()
            expected = hashlib.sha256(subprocess.check_output(['git', 'show', PIN + ':' + relative], cwd=ROOT)).hexdigest()
        check('Additional recorded source/input identity', actual == recorded == expected,
              path=relative, actual_sha256=actual, recorded_sha256=recorded, expected_sha256=expected)
custody_path = PKG / 'artifacts/final-inspected-source-identities.json'
if custody_path.exists():
    custody = json.loads(custody_path.read_text())
    prior = json.loads((ROOT / custody['prior_inventory']).read_text())['fresh_regular_file_sha256']
    for row in custody['files']:
        actual = hashlib.sha256((Path(custody['source_root']) / row['path']).read_bytes()).hexdigest()
        check('Final inspected source custody', actual == row['sha256'] == row['prior_sha256'] == prior.get(row['path']),
              path=row['path'], actual_sha256=actual)
for argv in (['git', 'diff', 'HEAD', '--check'],):
    result = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
    check('Tracked staged/unstaged whitespace', result.returncode == 0, argv=argv, stdout=result.stdout, stderr=result.stderr)
# Untracked new documents are absent from git diff until staged.
for path in sorted(PKG.rglob('*')):
    if path.is_file() and path.suffix in ('.md', '.py', '.json', '.txt'):
        rows = path.read_text().splitlines()
        bad = [i for i, row in enumerate(rows, 1) if row.rstrip(' \t') != row]
        check('New evidence trailing whitespace', not bad, path=str(path.relative_to(ROOT)), lines=bad)
result = dict(cwd=str(ROOT), argv=[str(ROOT / '.venv/bin/python'), str(Path(__file__).resolve())],
              governing_commit=PIN, results=checks, exit=0,
              scope='Static document/input validation only. No compiled listing, Rust build/test, simulation or prospective workflow executed.')
(PKG / 'artifacts/executor-checks.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(dict(checks=len(checks), status='PASS')))
