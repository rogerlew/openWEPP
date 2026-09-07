"""Public CLI conformance, using external isolated fixture trees."""
from pathlib import Path
import subprocess
import sys
import pytest

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / 'tools/check_sc_binding_exposure.py'
I = 'INV-X-001'
O = 'OBL-X-P-001'


def table(headers, rows):
    return '| ' + ' | '.join(headers) + ' |\n|' + '---|'*len(headers) + '\n' + ''.join('| '+' | '.join(row)+' |\n' for row in rows)


def run(entry, strict=False):
    return subprocess.run([sys.executable, str(CHECKER), *(['--strict'] if strict else []), str(entry)], capture_output=True, text=True)


@pytest.fixture
def valid(tmp_path):
    entry = tmp_path/'SC-X-001.md'
    folder = tmp_path/'SC-X-001'; folder.mkdir()
    entry.write_text('''---
contract_id: SC-X-001
title: Test
status: approved
maturity: active
owner: test
contract_version: 1
producer_scope: test producer
consumer_scope: test consumer
evidence_level: static
last_reviewed: pending
supersedes: []
superseded_by: []
contract_format: directory-v1
binding_index: SC-X-001/bindings.md#bei
---
# Test
## Document inventory
'''+table(['Path','Kind','Purpose','Applicability'], [[f'SC-X-001/{f}.md','normative',f,'all'] for f in ['surface','soil','bindings']])+'''
## Reading routes
'''+table(['Task','Role/check','Initial material','Expansion trigger'],[['surface coupling','review','entry and surface/soil','changed owner']])+f'''
<a id="{I}"></a>
## Compatibility anchors
'''+table(['Alias','Target'],[[I,f'SC-X-001/surface.md#{I}']]))
    (folder/'surface.md').write_text('''[Parent](../SC-X-001.md)
## Dependencies
'''+table(['Target','Required when','Boundary/obligation','Reading extent'],[['soil.md#soil','surface soil coupling','soil owner','section']])+'''
<a id="surface"></a>
## Surface
The positive energy transfer must be accounted exactly once.
'''+table(['Invariant ID','Statement','Authority','Evidence','Guard','Failure posture'],[[f'<a id="{I}"></a> `{I}`','Positive transfer exactly once','physical conservation','[INFERENCE][Static]','typed check','typed failure']]))
    (folder/'soil.md').write_text('''[Parent](../SC-X-001.md)
## Dependencies
none beyond entry
<a id="soil"></a>
## Soil
'''+table(['Obligation ID','Statement','Applicability','Authority','Enforcement/failure','Test bindings'],[[f'<a id="{O}"></a> `{O}`','Receive transfer once','soil coupling','physical conservation','typed failure','independent sum']]))
    (folder/'bindings.md').write_text('''[Parent](../SC-X-001.md)
## Dependencies
none beyond entry
<a id="bei"></a>
## Binding Exposure Index
'''+table(['Entry ID','Source','Status','Binding classification','Canonical binding IDs','Review gate','Notes'],[['source','[original](surface.md#surface)','active','maps-to-existing-INV',f'{I}, {O}','none','preserved']])+'''
## Binding definitions
'''+table(['ID','Definition'],[[I,f'surface.md#{I}'],[O,f'soil.md#{O}']])+'''
## Schema coverage
'''+table(['Requirement','Canonical target(s)','Applicability'],[[f'{p}.section.{i:02}','surface.md#surface','all'] for p,n in [('artifact',18),('kernel',14)] for i in range(1,n+1)]))
    return entry


def mutate(entry, name, old, new):
    path = entry.parent/'SC-X-001'/f'{name}.md' if name else entry
    s=path.read_text(); assert old in s; path.write_text(s.replace(old,new))


def assert_fail(entry):
    r=run(entry); assert r.returncode == 1, r.stdout+r.stderr
    assert 'FAIL' in r.stdout and 'Traceback' not in r.stderr


def test_valid_two_mechanisms_qualified_ids_and_alias(valid):
    r=run(valid,True); assert r.returncode == 0, r.stdout+r.stderr
    assert 'directory-v1' in r.stdout


@pytest.mark.parametrize('mutation', ['missing_definition','duplicate_definition','fenced_definition','missing_member','duplicate_member','unlisted_member','historical_target','bad_utf8','unknown_format','duplicate_format','malformed_frontmatter','malformed_table','empty_cell','missing_coverage','duplicate_coverage','broken_dependency','bad_alias','alias_cycle','duplicate_anchor','mismatch_id','standalone_definition','missing_provenance','invalid_status','invalid_class','invalid_gate'])
def test_negative_matrix(valid,mutation):
    folder=valid.with_suffix('')
    if mutation=='missing_definition':
        mutate(valid,'surface',f'<a id="{I}"></a> `{I}`',f'`{I}`')
    elif mutation=='duplicate_definition':
        (folder/'soil.md').write_text((folder/'soil.md').read_text()+'\n'+(folder/'surface.md').read_text().split('## Surface')[1])
    elif mutation=='fenced_definition':
        mutate(valid,'surface','| Invariant ID','```md\n| Invariant ID')
    elif mutation=='missing_member': (folder/'soil.md').unlink()
    elif mutation=='duplicate_member': mutate(valid,None,'## Reading routes','| SC-X-001/soil.md | normative | soil | all |\n\n## Reading routes')
    elif mutation=='unlisted_member': (folder/'rogue.md').write_text('unexpected')
    elif mutation=='historical_target': mutate(valid,None,'SC-X-001/soil.md | normative','SC-X-001/soil.md | historical')
    elif mutation=='bad_utf8': (folder/'soil.md').write_bytes(b'\xff')
    elif mutation=='unknown_format': mutate(valid,None,'directory-v1','directory-v2')
    elif mutation=='duplicate_format': mutate(valid,None,'contract_format: directory-v1','contract_format: directory-v1\ncontract_format: directory-v1')
    elif mutation=='malformed_frontmatter': mutate(valid,None,'title: Test','title Test')
    elif mutation=='malformed_table': mutate(valid,'surface','Positive transfer exactly once','Positive | transfer exactly once')
    elif mutation=='empty_cell': mutate(valid,'surface','physical conservation','')
    elif mutation=='missing_coverage': mutate(valid,'bindings','| kernel.section.14 | surface.md#surface | all |','')
    elif mutation=='duplicate_coverage': mutate(valid,'bindings','kernel.section.14','kernel.section.13')
    elif mutation=='broken_dependency': mutate(valid,'surface','soil.md#soil','soil.md#absent')
    elif mutation=='bad_alias': mutate(valid,None,f'surface.md#{I}','surface.md#missing')
    elif mutation=='alias_cycle': mutate(valid,None,f'SC-X-001/surface.md#{I}',f'#{I}')
    elif mutation=='duplicate_anchor': (folder/'soil.md').write_text((folder/'soil.md').read_text()+'\n<a id="soil"></a>\n')
    elif mutation=='mismatch_id': mutate(valid,'surface',f'id="{I}"','id="INV-X-999"')
    elif mutation=='standalone_definition': (folder/'soil.md').write_text((folder/'soil.md').read_text()+'\n<a id="INV-X-999"></a>\n')
    elif mutation=='missing_provenance':
        mutate(valid,None,'SC-X-001/soil.md | normative','SC-X-001/soil.md | historical')
        mutate(valid,'bindings','surface.md#surface)','soil.md#soil)')
    elif mutation=='invalid_status': mutate(valid,'bindings','| active |','| retired |')
    elif mutation=='invalid_class': mutate(valid,'bindings','maps-to-existing-INV','invented')
    elif mutation=='invalid_gate': mutate(valid,'bindings','| none | preserved','| ignored | preserved')
    assert_fail(valid)


@pytest.mark.parametrize('raw',['/tmp/outside.md','SC-X-001/../outside.md','SC-X-001//soil.md','SC-X-001/./soil.md','SC-X-001\\soil.md','SC-X-001/Soil.md','SC-X-001/soil.txt'])
def test_unsafe_members(valid,raw):
    mutate(valid,None,'SC-X-001/soil.md',raw); assert_fail(valid)


def test_symlink_member_no_outside_read(valid,tmp_path):
    sentinel=tmp_path/'outside.md';sentinel.write_bytes(b'\xff')
    p=valid.with_suffix('')/'soil.md';p.unlink();p.symlink_to(sentinel)
    r=run(valid); assert r.returncode==1 and 'symlink' in r.stdout and 'UTF-8' not in r.stdout
    assert sentinel.read_bytes()==b'\xff'


def test_escaping_dependency_no_outside_read(valid):
    mutate(valid,'surface','soil.md#soil','../../sentinel.md#outside')
    r=run(valid);assert r.returncode==1 and 'escapes' in r.stdout


@pytest.mark.parametrize('fence,close',[('```md','```'),('~~~~ example','~~~~'),('`````md','`````')])
def test_fenced_examples_are_not_definitions(valid,fence,close):
    p=valid.with_suffix('')/'soil.md'
    p.write_text(p.read_text()+f'\n{fence}\n<a id="INV-X-999"></a>\n{close}\n')
    r=run(valid);assert r.returncode==0,r.stdout+r.stderr


def test_short_or_other_fence_does_not_close(valid):
    p=valid.with_suffix('')/'soil.md'
    p.write_text(p.read_text()+'\n`````md\n```\n~~~\n<a id="INV-X-999"></a>\n`````\n')
    assert run(valid).returncode==0


def test_deferred_default_strict_and_retained_residue(valid):
    mutate(valid,'bindings','maps-to-existing-INV','undecidable')
    mutate(valid,'bindings','| none | preserved','| science-review-follow-on | owner: science; next evidence gate: adjudication; retained: surface.md#surface')
    assert run(valid).returncode==0
    assert 'PASS-DEFERRED' in run(valid).stdout
    assert run(valid,True).returncode==1


def test_qualifier_loss_is_semantic_counterexample(valid):
    mutate(valid,'surface','Positive transfer exactly once','Transfer once')
    assert run(valid,True).returncode==0
    # Preservation review must reject this, even though IDs/structure still conform.
    assert 'Positive transfer exactly once' not in (valid.with_suffix('')/'surface.md').read_text()


def test_usage():
    r=subprocess.run([sys.executable,str(CHECKER)],capture_output=True,text=True)
    assert r.returncode==2

@pytest.mark.parametrize('name,old,new',[
    ('','surface coupling | review',' | review'),
    ('soil','none beyond entry',''),
    ('bindings',f'{I}, {O}',f'{I}-BOGUS, {O}'),
    ('surface','## Surface','#NotAHeading'),
    ('','contract_format: directory-v1','contract_format: "directory-v1'),
    ('surface','|---|---|---|---|---|---|','|---|---|'),
    ('surface','| Invariant ID','<!--\n| Invariant ID'),
])
def test_independent_review_counterexamples(valid,name,old,new):
    mutate(valid,name,old,new); assert_fail(valid)

@pytest.fixture
def with_history(valid):
    mutate(valid,None,'| SC-X-001/bindings.md | normative | bindings | all |','| SC-X-001/bindings.md | normative | bindings | all |\n| SC-X-001/history.md | historical | retained history | audit |')
    path=valid.with_suffix('')/'history.md'
    path.write_text('''[Parent](../SC-X-001.md)
<a id="old"></a>
## old Original rationale
- status: historical
- source_package: original
- effective_date: 2026-09-07
- verdict: historical
- canonical_binding_ids: none
- provenance_anchors: original retained source

Original nonbinding rationale.
''')
    mutate(valid,'bindings','| source |','| old | [old](history.md#old) | historical | historical-or-superseded | none | none | retained |\n| source |')
    return valid


def test_valid_history(with_history):
    r=run(with_history); assert r.returncode==0,r.stdout+r.stderr


@pytest.mark.parametrize('old,new',[
    ('- status: historical','- status: potato'),
    ('- status: historical','- status: superseded'),
    ('- canonical_binding_ids: none','- canonical_binding_ids: INV-X-999'),
    ('- provenance_anchors: original retained source',''),
    ('Original nonbinding rationale.',''),
    ('## old Original rationale','## old'),
    ('Original nonbinding rationale.','Original nonbinding rationale.\n\n## other Missing fields'),
])
def test_provenance_fields_independently(with_history,old,new):
    mutate(with_history,'history',old,new);assert_fail(with_history)


def test_raw_provenance_target_checked(with_history):
    mutate(with_history,'bindings','[old](history.md#old)','history.md#old')
    mutate(with_history,'history','- provenance_anchors: original retained source','')
    assert_fail(with_history)


def test_supersession_target(with_history):
    mutate(with_history,'bindings','| historical | historical-or-superseded','| superseded | historical-or-superseded')
    mutate(with_history,'history','- status: historical','- status: superseded\n- superseded_by: surface.md#surface')
    r=run(with_history);assert r.returncode==0,r.stdout+r.stderr
    mutate(with_history,'history','surface.md#surface','surface.md#missing');assert_fail(with_history)


def test_empty_deferral_ownership_rejected(valid):
    mutate(valid,'bindings','| none | preserved','| science-review-follow-on | owner: ; next evidence gate: ; retained: surface.md#surface')
    assert_fail(valid)


def test_multiple_alias_targets_rejected(valid):
    mutate(valid,None,f'| {I} | SC-X-001/surface.md#{I} |',f'| {I} | [a](SC-X-001/surface.md#{I}) [b](SC-X-001/surface.md#missing) |')
    assert_fail(valid)


def test_missing_entire_definition_with_mentions_retained(valid):
    path=valid.with_suffix('')/'surface.md'
    path.write_text('\n'.join(l for l in path.read_text().splitlines() if f'<a id="{I}"></a> `{I}`' not in l)+'\n')
    assert_fail(valid)


def test_unreadable_and_nonregular(valid):
    path=valid.with_suffix('')/'soil.md';path.chmod(0)
    try: assert_fail(valid)
    finally: path.chmod(0o644)
    path.unlink()
    import os
    os.mkfifo(path)
    assert_fail(valid)

@pytest.mark.parametrize('old,new',[('status: approved','status: potato'),('maturity: active','maturity: potato'),('producer_scope: test producer','producer_scope: []')])
def test_invalid_metadata_values(valid,old,new):
    mutate(valid,None,old,new);assert_fail(valid)


def test_neighbor_history_cannot_supply_authority(valid):
    root=valid.parent
    (root/'SC-Y-001').mkdir()
    (root/'SC-Y-001/history.md').write_text('<a id="old"></a>\n## History\nold')
    (root/'SC-Y-001.md').write_text('---\ncontract_format: directory-v1\n---\n## Document inventory\n'+table(['Path','Kind','Purpose','Applicability'],[['SC-Y-001/history.md','historical','history','audit']]))
    mutate(valid,'surface','soil.md#soil','../SC-Y-001/history.md#old');assert_fail(valid)


def test_legacy_deferred_modes(tmp_path):
    from test_check_sc_binding_exposure import contract_with_binding
    p=tmp_path/'legacy.md'
    p.write_text(contract_with_binding(I,I).replace('`none` | test','`science-review-follow-on` | test'))
    assert run(p).returncode==0 and 'PASS-DEFERRED' in run(p).stdout
    assert run(p,True).returncode==1


def test_nested_neighbor_history_cannot_supply_authority(valid):
    root=valid.parent
    (root/'SC-Y-001/archive').mkdir(parents=True)
    (root/'SC-Y-001/archive/history.md').write_text('<a id="old"></a>\n## History\nold')
    (root/'SC-Y-001.md').write_text('---\ncontract_format: directory-v1\n---\n## Document inventory\n'+table(['Path','Kind','Purpose','Applicability'],[['SC-Y-001/archive/history.md','historical','history','audit']]))
    mutate(valid,'surface','soil.md#soil','../SC-Y-001/archive/history.md#old');assert_fail(valid)


def test_external_history_provenance_checked(valid):
    root=valid.parent
    (root/'SC-Y-001').mkdir()
    (root/'SC-Y-001/history.md').write_text('<a id="old"></a>\n## old History\nMissing metadata')
    (root/'SC-Y-001.md').write_text('---\ncontract_format: directory-v1\n---\n## Document inventory\n'+table(['Path','Kind','Purpose','Applicability'],[['SC-Y-001/history.md','historical','history','audit']]))
    mutate(valid,'bindings','[original](surface.md#surface)','../SC-Y-001/history.md#old');assert_fail(valid)


def test_distant_explicit_heading_collision(valid):
    p=valid.with_suffix('')/'surface.md';p.write_text(p.read_text()+'\n## Surface\nOther authority\n')
    assert_fail(valid)


def test_invalid_calendar_provenance(with_history):
    mutate(with_history,'history','2026-09-07','2026-99-99');assert_fail(with_history)


def test_unit_consumer_reads_all_chapters_and_rejects_removed_mapping(valid):
    folder=valid.with_suffix('')
    for name,symbol,alias in [('surface','surface_temp','surface_kelvin'),('soil','soil_temp','soil_kelvin')]:
        p=folder/f'{name}.md'
        p.write_text(p.read_text()+'\n## Variables and Units\n'+table(['Symbol','Units'],[[f'`{symbol}`','`K`']])+'\n## Symbol Alias Map\n'+table(['Canonical symbol','Boundary/API name','Scope','Units check'],[[f'`{symbol}`',f'`{alias}`','runtime','`K`']]))
    registry=valid.parent/'registry.rs'
    registry.write_text('\n'.join(f'BoundaryUnitEntry::new("{symbol}", &["{alias}"], "K", 0, 0, 0, 0, "SC-X-001", 0, 0, 0, &[])' for symbol,alias in [('surface_temp','surface_kelvin'),('soil_temp','soil_kelvin')]))
    command=[sys.executable,str(ROOT/'tools/release/check_sc_unit_compliance.py'),'--path',str(valid),'--registry-source',str(registry)]
    r=subprocess.run(command,capture_output=True,text=True)
    assert r.returncode==0,r.stdout+r.stderr
    mutate(valid,'soil','| `soil_temp` | `soil_kelvin` | runtime | `K` |','')
    r=subprocess.run(command,capture_output=True,text=True)
    assert r.returncode==1 and 'SCUNIT-E-011' in r.stderr,r.stdout+r.stderr


def test_whole_set_consumer_cannot_pass_on_registry_or_entry_mentions(valid):
    command=[sys.executable,str(ROOT/'tools/sc_contract_directory.py'),str(valid)]
    r=subprocess.run(command,capture_output=True,text=True)
    assert r.returncode==0 and 'Receive transfer once' in r.stdout
    p=valid.with_suffix('')/'soil.md'
    p.write_text('\n'.join(l for l in p.read_text().splitlines() if f'<a id="{O}"></a> `{O}`' not in l)+'\n')
    r=subprocess.run(command,capture_output=True,text=True)
    assert r.returncode==1 and 'missing explicit anchor' in r.stderr


def test_legacy_loader_retains_existing_yaml_continuations(tmp_path):
    p=tmp_path/'SC-LEGACY-001.md'
    original='---\nproducer_scope:\n  - first line,\n    continuation\n---\nLegacy body\n'
    p.write_text(original)
    r=subprocess.run([sys.executable,str(ROOT/'tools/sc_contract_directory.py'),str(p)],capture_output=True,text=True)
    assert r.returncode==0 and r.stdout==original,r.stderr


def test_shared_loader_rejects_malformed_format_dispatch(valid):
    mutate(valid,None,'contract_format: directory-v1','contract_format : directory-v1')
    r=subprocess.run([sys.executable,str(ROOT/'tools/sc_contract_directory.py'),str(valid)],capture_output=True,text=True)
    assert r.returncode==1 and 'malformed front matter' in r.stderr


@pytest.mark.parametrize('replacement', ['"contract_format": directory-v1', "'contract_format': directory-v1", '```yaml\ncontract_format: directory-v1\n```'])
def test_unsupported_format_key_cannot_hide_dispatch(valid,replacement):
    mutate(valid,None,'contract_format: directory-v1',replacement)
    assert_fail(valid)
    r=subprocess.run([sys.executable,str(ROOT/'tools/sc_contract_directory.py'),str(valid)],capture_output=True,text=True)
    assert r.returncode==1 and 'malformed front matter' in r.stderr


def test_admission_resolves_parent_and_hashes_chapter_bytes(valid,tmp_path):
    import os
    import shutil
    root=tmp_path/'repo'; root.mkdir()
    contracts=root/'docs/specifications/science-contracts/contracts';contracts.mkdir(parents=True)
    shutil.copy2(valid,contracts/valid.name)
    shutil.copytree(valid.with_suffix(''),contracts/valid.stem)
    for relative in ['tools/sc_contract_directory.py','tools/release/check_science_contract_admission.sh']:
        target=root/relative;target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(ROOT/relative,target)
    index=contracts.parent/'index.md'
    index.write_text('## Current Registry\n'+table(['contract_id','title','status','maturity','owner','path','a','b','c','d'],[['`SC-X-001`','x','approved','active','x','docs/specifications/science-contracts/contracts/SC-X-001.md','x','x','x','x']]))
    infile=contracts/'SC-INFILE-X-001.md'
    infile.write_text('---\ncontract_id: SC-INFILE-X-001\nstatus: approved\nmaturity: active\n---\n')
    inp=root/'docs/specifications/wepp-input-files/input-surface-registry.md';inp.parent.mkdir(parents=True)
    inp.write_text('| `infile-x` | x | x | active | SC-INFILE-X-001 |\n')
    for name,data in [('impact-map.json','{"entries":[]}'),('gate-definitions.json','{"definitions":[]}')]:
        t=root/'tools/release/authority-policy'/name;t.parent.mkdir(parents=True,exist_ok=True);t.write_text(data)
    ext=root/'docs/specifications/external-authority/registry.yaml';ext.parent.mkdir(parents=True);ext.write_text('')
    def git(*args):
        return subprocess.run(['git',*args],cwd=root,capture_output=True,text=True,check=True)
    git('init');git('add','.');git('-c','user.name=Fixture','-c','user.email=fixture@example.invalid','commit','-m','fixture')
    chapter=contracts/'SC-X-001/soil.md';chapter.write_text(chapter.read_text()+'\nAdditional existing-authority explanation.\n')
    env=dict(os.environ,PATH=str(Path(sys.executable).parent)+os.pathsep+os.environ['PATH'])
    def admission():
        return subprocess.run(['bash',str(root/'tools/release/check_science_contract_admission.sh'),'--base-ref','HEAD','--worktree'],cwd=root,env=env,capture_output=True,text=True)
    first=admission();assert first.returncode==0,first.stdout+first.stderr
    chapter.write_text(chapter.read_text()+'\nAnother explanation.\n')
    second=admission();assert second.returncode==0,second.stdout+second.stderr
    assert first.stdout.split('authority_sha256=')[1]!=second.stdout.split('authority_sha256=')[1]
    helper=root/'tools/sc_contract_directory.py'
    helper.write_text(helper.read_text()+'\n# Identity-input mutation.\n')
    third=admission();assert third.returncode==0,third.stdout+third.stderr
    assert second.stdout.split('authority_sha256=')[1]!=third.stdout.split('authority_sha256=')[1]
    chapter.write_text('\n'.join(l for l in chapter.read_text().splitlines() if f'<a id="{O}"></a> `{O}`' not in l)+'\n')
    bad=admission();assert bad.returncode==1 and 'missing explicit anchor' in bad.stderr,bad.stdout+bad.stderr
