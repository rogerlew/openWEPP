"""Validate assessment artifacts without executing Rust or proposed commands."""
from pathlib import Path
import ast, json, re, subprocess, sys
ROOT = Path('/workdir/openWEPP')
PKG = ROOT/'docs/work-packages/20260911-b01-wb14-source-reconciliation-001'
checks = []
for path in sorted((PKG/'artifacts').rglob('*.json')):
    json.loads(path.read_text())
    checks.append({'check': 'JSON parse', 'path': str(path.relative_to(ROOT)), 'status': 'PASS'})
for path in sorted((PKG/'artifacts').glob('*.py')):
    ast.parse(path.read_text())
    checks.append({'check': 'Python syntax', 'path': str(path.relative_to(ROOT)), 'status': 'PASS'})
for path in sorted((PKG/'artifacts').glob('*.sh')):
    subprocess.run(['bash', '-n', str(path)], check=True, cwd=ROOT)
    checks.append({'check': 'shell syntax', 'path': str(path.relative_to(ROOT)), 'status': 'PASS'})
record = (PKG/'package.md').read_text()
for index, block in enumerate(re.findall(r'```(?:sh|text)\n(.*?)```', record, re.S)):
    subprocess.run(['bash', '-n'], input=block, text=True, check=True, cwd=ROOT)
    checks.append({'check': 'prospective command syntax only', 'block': index, 'status': 'PASS'})
for path in [PKG/'package.md', ROOT/'docs/work-packages/active.md', ROOT/'docs/work-packages/README.md']:
    missing = []
    for link in re.findall(r'\]\(([^)]+)\)', path.read_text()):
        if '://' in link or link.startswith('#'):
            continue
        target = link.split('#')[0]
        if target and not (path.parent/target).exists():
            missing.append(link)
    # Catalogs can contain unrelated historic links; only this package is owned.
    if path.name != 'package.md':
        missing = [x for x in missing if '20260911-b01-wb14-source-reconciliation' in x]
    if missing:
        raise SystemExit(f'Missing owned links: {missing}')
    checks.append({'check': 'owned local links', 'path': str(path.relative_to(ROOT)), 'status': 'PASS'})
command = ['git', 'diff', 'HEAD', '--check']
subprocess.run(command, check=True, cwd=ROOT)
checks.append({'check': 'terminal staged plus unstaged whitespace', 'argv': command, 'status': 'PASS'})
output = {'cwd': str(ROOT), 'argv': [str(ROOT/'.venv/bin/python'), str(Path(__file__).resolve())], 'exit': 0, 'results': checks, 'scope': 'Assessment artifact validation only; no Rust or prospective shell workflows executed'}
(PKG/'artifacts/executor-checks.json').write_text(json.dumps(output, indent=2)+'\n')
print(json.dumps({'checks': len(checks), 'status': 'PASS'}))
