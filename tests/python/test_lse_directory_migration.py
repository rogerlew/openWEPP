"""Direct migration obligations independent of candidate generation and inherited panics."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

import pytest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools'))
from sc_contract_directory import ContractSet  # noqa: E402

ENTRY = ROOT / 'docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md'
OLD = ROOT / 'docs/work-packages/20260907-directory-contract-checker-lse-adoption-001/artifacts'
SOURCE = json.loads((OLD / 'clause-map.json').read_text())

@pytest.fixture(scope='module')
def contract():
    return ContractSet(ENTRY)


def definition(c, ident):
    path, _ = c.definitions[ident]
    doc = c.documents[path]
    return next(line for line in doc.lines if line.startswith(f'| <a id="{ident}"></a>'))


def invariant(c, number):
    return definition(c, f'INV-LANDSURFACEENERGY-{number:03}')


def owner(c, name):
    return c.documents[c.entry.with_suffix('') / (name + '.md')].text


def normalize(s):
    return ' '.join(s.replace('&#124;', '|').split())


def original_bytes():
    raw = subprocess.check_output(['git','show',f"{SOURCE['source_commit']}:{SOURCE['source_path']}"], cwd=ROOT)
    assert hashlib.sha256(raw).hexdigest() == SOURCE['source_sha256']
    return raw


def test_all_original_statements_and_scientific_lines_survive(contract):
    """Every independently mapped original span, not an ID/aggregate self-comparison."""
    lines = original_bytes().decode().splitlines()
    corpus = normalize(contract.normative_text())
    history = normalize((contract.entry.with_suffix('')/'history.md').read_text())
    for item in SOURCE['clauses']:
        a,b = item['source_start'],item['source_end']
        source = '\n'.join(lines[a-1:b])
        if not source.strip():
            continue
        if 'target_anchor' in item:
            row = definition(contract,item['target_anchor'])
            # Original statements are independent of generated guard/provenance metadata.
            if source.startswith('|'):
                # Old code-span literal pipes are not table separators.
                source = re.sub(r'`[^`]*`',lambda m:m[0].replace('|','&#124;'),source)
                cells=source.strip('|').split('|')[1:]
                if len(cells)==5:
                    current=row.strip('|').split('|')[1:]
                    assert list(map(normalize,cells)) == list(map(normalize,current)), (a,b,item['target_anchor'])
                for cell in cells:
                    assert normalize(cell) in normalize(row), (a,b,item['target_anchor'],cell)
                statement=cells[0].strip()
            else:
                statement=re.sub(r'^(?:- )?`[^`]+`\s*[:—]?\s*','',normalize(source))
            assert normalize(statement) in normalize(row), (a,b,item['target_anchor'])
        elif item['target']=='binding-index.md' and source.startswith('| `'):
            # Original BEI provenance is intentionally pinned; all other cells survive.
            cells=source.strip('|').split('|')
            assert normalize('|'.join(cells[2:])) in corpus, (a,b)
        else:
            expected=source.replace('| Invariant ID | Statement','| Binding reference | Statement')
            if item['kind']=='historical':expected=expected.replace('## Change Log','### Change Log')
            assert normalize(expected) in (history if item['kind']=='historical' else corpus), (a,b,expected)


def check_snow_soil(c):
    assert c.meta['contract_version']=='32'
    for n in [124,125,126]: assert len(invariant(c,n).split('|')[2])>80
    text=owner(c,'soil-coupling')
    for expected in ['R_ss       = dz_sb/(2*lambda_sb) + dz_1/(2*lambda_1)', 'SnowSoilHeatReceiptV1','LSEB-E-044']:
        assert expected in text


def test_m1_masked_snow_soil_predicates(contract):check_snow_soil(contract)


def test_m2_masked_pending_map_tuple(contract):
    assert contract.meta['contract_version']=='32'
    assert 'pending' in invariant(contract,161).lower()
    assert 'physical' in definition(contract,'OBL-LANDSURFACEENERGY-C-016').lower()
    assert 'LSE-V27-PENDING-ADJUDICATION' in owner(contract,'binding-index')
    assert 'converged' in owner(contract,'map-custody')


def test_m3_masked_final_reseal_tuple(contract):
    assert contract.meta['contract_version']=='32'
    assert 'reseal' in invariant(contract,160).lower()
    assert 'physical' in definition(contract,'OBL-LANDSURFACEENERGY-C-015').lower()
    assert 'replay fallback' in owner(contract,'map-custody').lower()


def test_m4_masked_hydrology_ownership(contract):
    original=original_bytes().decode()
    expected='Hydrology exclusively owns ponded, litter-held and soil-layer water mass'
    assert expected in original
    assert expected in owner(contract,'surface-energy')


@pytest.mark.parametrize('number',[150,151,158,159])
def test_m5_m6_relocated_exact_custody_definitions(contract,number):
    lines=original_bytes().decode().splitlines()
    a,b=SOURCE['binding_definitions'][f'INV-LANDSURFACEENERGY-{number}'][1:]
    source=normalize('\n'.join(lines[a-1:b]))
    actual=re.sub(r'<a id="[^"]+"></a> ', '', invariant(contract,number))
    assert source == normalize(actual)


def test_m7_reading_and_metadata_boundary(contract):
    assert contract.meta['status']=='approved'
    assert contract.meta['maturity']=='active'
    guide=(ROOT/'docs/specifications/science-contracts/AGENTS.md').read_text()
    for text in ['numerical-solver-architecture.md','Existing single-file contracts still require the full contract','Only after','directory-format adoption']:
        assert text in guide
    original=subprocess.check_output(['git','show','fb32d27f2:docs/specifications/science-contracts/index.md'],cwd=ROOT)
    current=(ROOT/'docs/specifications/science-contracts/index.md').read_bytes()
    prefix=b'| `SC-LANDSURFACEENERGY-001` |'
    old_lines=original.splitlines(keepends=True)
    new_lines=current.splitlines(keepends=True)
    assert len(old_lines)==len(new_lines)
    for before,after in zip(old_lines,new_lines):
        if before.startswith(prefix):
            assert after==before.replace(b'| `2026-09-04` |',b'| `2026-09-07` |')
        else:assert before==after
    for predicate in [b'v30 retains V9 generation-host/provider-equivalence', b'multi-lane covered Stage-3 remains unauthorized']:
        assert (predicate in original)==(predicate in current)


def test_m8_every_legacy_entry_is_byte_identical():
    for path in ENTRY.parent.glob('SC-*.md'):
        if path!=ENTRY:
            assert ContractSet(path).normative_text()==path.read_text(),path


def test_scientific_qualifier_mutation_is_not_structural_preservation(contract):
    """Canonical IDs remain valid; independent physical equation predicate rejects."""
    doc=contract.documents[ENTRY.with_suffix('')/'soil-coupling.md']
    original=doc.text
    try:
        doc.text=original.replace('dz_sb/(2*lambda_sb)', 'dz_sb/lambda_sb')
        with pytest.raises(AssertionError):check_snow_soil(contract)
    finally:doc.text=original


@pytest.mark.parametrize('poison',['equation','failure-posture'])
def test_on_disk_scientific_poison_passes_structure_but_fails_physical_obligation(tmp_path,poison):
    """Actual parser accepts the intact registry; physics predicate still rejects."""
    import shutil
    (tmp_path/'.git').mkdir()
    recovery=json.loads((OLD/'candidate-recovery.json').read_text())
    for row in recovery['baseline_dependencies']:
        source=ROOT/row['path']
        target=tmp_path/row['path']
        target.parent.mkdir(parents=True,exist_ok=True)
        shutil.copyfile(source,target)
    target=tmp_path/ENTRY.relative_to(ROOT)
    target.parent.mkdir(parents=True,exist_ok=True)
    shutil.copyfile(ENTRY,target)
    shutil.copytree(ENTRY.with_suffix(''),target.with_suffix(''))
    if poison=='equation':
        path=target.with_suffix('')/'soil-coupling.md'
        original=path.read_text()
        assert 'dz_sb/(2*lambda_sb)' in original
        path.write_text(original.replace('dz_sb/(2*lambda_sb)','dz_sb/lambda_sb'))
    else:
        path=target.with_suffix('')/'map-custody.md'
        lines=path.read_text().splitlines()
        i=next(i for i,l in enumerate(lines) if l.startswith('| <a id="INV-LANDSURFACEENERGY-161"'))
        cells=lines[i].split('|')
        assert 'rollback' in cells[-2]
        cells[-2]=' physical failures permit publication '
        lines[i]='|'.join(cells)
        path.write_text('\n'.join(lines)+'\n')
    candidate=ContractSet(target)  # Full directory structure/definitions still PASS.
    with pytest.raises(AssertionError):
        test_all_original_statements_and_scientific_lines_survive(candidate)
    if poison=='equation':
        with pytest.raises(AssertionError):check_snow_soil(candidate)


def test_every_original_fenced_body_is_byte_identical(contract):
    from collections import Counter
    def bodies(text):
        return re.findall(r'^```[^\n]*\n(.*?)^```[ \t]*$',text,re.M|re.S)
    original=bodies(original_bytes().decode())
    assert original
    assert Counter(original)==Counter(bodies(contract.normative_text()))
