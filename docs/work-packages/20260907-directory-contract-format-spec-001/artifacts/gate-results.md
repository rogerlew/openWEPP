# Gate results
Static: substantive specification-governance change with conditional adoption,
not editorial and not a change to effective scientific/production authority.
Working directory for commands: /workdir/openWEPP.
Source: base d8249849d6e015070818be7caf6f8caa75485098 plus declared specification diff;
stable substantive commit recorded by subsequent independent review artifacts.

## Ran
- `.venv/bin/python -m pytest -q tests/python/test_check_sc_binding_exposure.py`:
  exit 1, `No module named pytest`. The pytest runner is unavailable, not PASS.
- Equivalent direct execution below: exit 0, all 3 existing test functions PASS.
  No assertion/fixture logic changed; each tmp_path supplied a new external directory.
- `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`:
  exit 0, PASS, 16 rows. Legacy behavior only, not directory-format support.
- Markdown local-path/heading-anchor check of eight changed nonpackage documents:
  exit 0, 31 links resolve. Scanner resolves relative targets, checks files and
  normalized Markdown headings/explicit HTML anchors; no external fetch.
- `git diff --check`: exit 0.

Direct equivalent Python test invocation:

```sh
.venv/bin/python - <<'PY'
import importlib.util
import tempfile
from pathlib import Path
path = Path('tests/python/test_check_sc_binding_exposure.py')
spec = importlib.util.spec_from_file_location('binding_tests', path)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
for name in sorted(n for n in vars(module) if n.startswith('test_')):
    with tempfile.TemporaryDirectory(prefix='openwepp-format-test-') as tmp:
        getattr(module, name)(Path(tmp))
    print('PASS', name)
print('3 existing test functions executed; pytest runner unavailable')
PY
```

## Newly discovered applicable check
Authoring procedure is read by one focused Rust governance assertion.
Selected before execution:
`nix develop --offline --command cargo nextest run --offline --test adr0017_comparator_distrust_ratification_contract governance_docs_encode_comparator_flag_adjudication_gates`.
Exit 0: 1 passed, 3 intentionally filtered; run ID
8d5c1bd1-cce3-4777-9779-b992cab949c9. Existing dead-code warning for
ending_snow_hint observed; no warnings-denied build or production quality claim.
No full workspace or scientific measurements selected.

## Scope of evidence
Directory-format conformance cases are specified future implementation tests,
not executed behavior. No context reduction numbers claimed.
Independent review and verification remain current-scope requirements.

## Whole-cut documentation and scope check
Ran after substantive commit 6419ce26b: exit 0, 18 owned paths,
31 local links/anchors. Command below; rerun against the corrected cut at closure.
This is a bounded check of this diff's Markdown subset, not a universal renderer.

```sh
.venv/bin/python - <<'PY'
import re, subprocess
from pathlib import Path
base='d8249849d6e015070818be7caf6f8caa75485098'
pkg=Path('docs/work-packages/20260907-directory-contract-format-spec-001')
contract=(pkg/'package.md').read_text()
paths=subprocess.check_output(['git','diff','--name-only',base,'HEAD'],text=True).splitlines()
count=0
for name in paths:
    path=Path(name)
    relative=str(path.relative_to(pkg)) if path.is_relative_to(pkg) else name
    assert '- '+relative+'\n' in contract, ('outside write set',name)
    assert not name.startswith(('crates/','tests/','tools/','docs/specifications/science-contracts/contracts/')), name
    for target in re.findall(r'\]\(([^\s)]+)\)',path.read_text()):
        if '://' in target: continue
        dest,_,anchor=target.partition('#')
        resolved=path.parent/dest if dest else path
        assert resolved.is_file(),(name,target)
        if anchor:
            text=resolved.read_text()
            headings={re.sub(r'[^\w\- ]','',line.lstrip('#').strip().lower()).replace(' ','-') for line in text.splitlines() if line.startswith('#')}
            explicit=set(re.findall(r'<a\s+id="([^"]+)"',text))
            assert anchor in headings|explicit,(name,target)
        count+=1
print('PASS:',len(paths),'owned paths;',count,'local Markdown links/anchors')
PY
git diff d8249849d HEAD --check
```
