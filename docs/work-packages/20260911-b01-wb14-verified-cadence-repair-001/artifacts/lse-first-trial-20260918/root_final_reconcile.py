"""Offline final source/trace/custody reconciliation; no physical execution."""
import collections
import datetime as dt
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = Path('/home/roger/openwepp-experiments/b01-wb14-lse-first-trial-source-20260918')
BASE = Path('/home/roger/openwepp-experiments/b01-wb14-observer-source-cut02-20260918')
PRIOR = Path('/workdir/openWEPP/docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/file-reader-20260918')
spec = importlib.util.spec_from_file_location('recorder', HERE/'root_run_recorded.py')
recorder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(recorder)
sha = recorder.sha
read = lambda name: json.loads((HERE/name).read_text())
freeze = read('root-freeze.json')
identities = recorder.SNAPSHOT.snapshot(SOURCE)[0]
assert hashlib.sha256(json.dumps(identities, sort_keys=True).encode()).hexdigest() == freeze['source_sha256']
baseline = recorder.SNAPSHOT.snapshot(BASE)[0]
assert hashlib.sha256(json.dumps(baseline, sort_keys=True).encode()).hexdigest() == freeze['baseline_tree_sha256']
assert sha('/home/roger/openwepp-experiments/b01-wb14-publication-observer-evidence-20260918/observer-cut02.frozen') == '9cf75b7337097df3454e2b301e2734565a36a3bdedbb5039b5df50d37abf0a49'
changed = [p for p in sorted(set(identities)|set(baseline)) if identities.get(p) != baseline.get(p)]
assert changed == ['crates/openwepp-land-surface-energy/src/lib.rs', 'crates/openwepp-land-surface-energy/src/lse_first_trial_diagnosis.rs', 'crates/openwepp-land-surface-energy/src/solver_covered_solve.rs']
assert all(sha(k) == v for k,v in freeze['external_build_inputs'].items())
assert all(str((SOURCE/k).resolve()) == v for k,v in freeze['support_links'].items())
fixture = 'crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_tests.rs'
assert (SOURCE/fixture).read_bytes() == (BASE/fixture).read_bytes()
first = read('instrumented-first-trial-trace.json')['records']
second = read('directional-followup-trace.json')['records']
final = read('final-first-failure-trace.json')['records']
policy = final[0]['value'].pop('snow_accuracy_policy')
assert policy == 'R0'
assert final == second
assert [r for r in second if r['kind'] != 'diagnostic_directional_probe'] == first

def payload(path):
    return next(line.strip() for line in path.read_text().splitlines() if line.lstrip().startswith('local staged native day 0'))
assert all(payload(HERE/name) == payload(PRIOR/'reader-exact-02.stderr') for name in ('instrumented-first-trial.stderr','directional-followup.stderr','final-first-failure.stderr'))
receipts = {name:read(name+'.json') for name in ('final-build','final-inventory','final-first-failure')}
for name,r in receipts.items():
    assert r['exit_code'] == (101 if name == 'final-first-failure' else 0)
    assert r['automatic_retries'] == 0 and not r['timed_out'] and not r['disk_stop']
    assert all(r[key] for key in ('source_unchanged','pinned_files_unchanged','external_build_inputs_unchanged','support_links_unchanged'))
    assert r['source_tree_sha256'] == freeze['source_sha256']
    if name != 'final-build':
        assert r['binary_unchanged'] and r['binary_sha256'] == read('final-binary.json')['sha256']
assert '--manifest-path' in receipts['final-build']['argv']
assert '1 test, 0 benchmarks' in (HERE/'final-inventory.stdout').read_text()
assert '0 passed; 1 failed; 0 ignored; 0 measured; 1519 filtered out' in (HERE/'final-first-failure.stdout').read_text()
counts = dict(collections.Counter(r['kind'] for r in final))
attempts = [r['value'] for r in final if r['kind'] == 'line_search_attempt']
terminal = [r for r in attempts if r['iteration'] == 6]
assert [r['exponent'] for r in terminal] == list(range(21))
assert all(not r['domain_valid'] and not r['accepted'] for r in terminal)
accepted = [r for r in attempts if r['accepted']]
assert [r['exponent'] for r in accepted] == [1,2,5,5,12,20]
bases = [r['value'] for r in final if r['kind'] == 'accepted_base']
for trial,base in zip(accepted,bases[1:]):
    assert trial['coordinates'] == base['coordinates']
    assert trial['normalized_residuals'] == base['normalized_residuals']
probes = [r['value'] for r in final if r['kind'] == 'diagnostic_directional_probe']
assert sum(p['frozen_canonical_evaluator_calls'] for p in probes) == 3
assert sum(p['unfrozen_canonical_evaluator_calls'] for p in probes) == 1
assert all(p['domain_valid'] for p in probes)
assert probes[0]['frozen_evaluation'] == probes[0]['unfrozen_evaluation']
roots = [{'directory':str(p), 'files':[str(q.relative_to(p)) for q in sorted(p.rglob('*')) if q.is_file()]} for p in (HERE/'tmp').glob('openwepp_native_owned_reader_*')]
assert roots and all(not r['files'] for r in roots)
result = dict(evidence_class='Ran: offline source/trace/hash reconstruction only; physical execution is the linked final-first-failure receipt',utc=dt.datetime.now(dt.timezone.utc).isoformat(),baseline_immutable=True,source_sha256=freeze['source_sha256'],source_entries=len(identities),external_build_inputs=len(freeze['external_build_inputs']),changed_source_paths=changed,binary_sha256=read('final-binary.json')['sha256'],fixture_file_sha256=sha(SOURCE/fixture),fixture_file_byte_equal_baseline=True,prior_test_body_sha256='ee1830a7d073b8faae671c1b7df41a43b58175766d12d6c7f1d16054073954a1',prior_climate_rust_literal_sha256='0164d7ed616995b81037ae687e3456568493dd8f67e7333396ae4893a79d15ef',test_hash_basis='Prior published extraction hashes carried through whole-fixture-file byte equality, plus unchanged remaining747 baseline entries except two observed LSE files and one added module.',effective_policy=policy,final_trace_sha256=sha(HERE/'final-first-failure-trace.json'),final_trace_equals_second_except_policy_header=True,ordinary_records_identical_all_runs=True,ordinary_record_count=len(first),record_counts=counts,full_terminal_diagnostic_byte_equal_baseline=True,accepted_exponents=[r['exponent'] for r in accepted],final_search_exponents=list(range(21)),final_trial_evaluator_calls=0,diagnostic_calls_by_run={'instrumented-first-trial':0,'directional-followup':4,'final-first-failure':4},total_diagnostic_calls={'frozen':6,'unfrozen':2,'total':8},diagnostic_points_installed=0,physical_first_failure_executions=3,completed_days=0,reader='NOT REACHED',observation_file_written=False,fixture_directories=roots,owner_rollback_evidence='Original reported before/after hashes byte-identical; not independent global rollback or conservation.',automatic_retries=0,limits='No converged root, later iteration, day1-4, reader, original E008 or promotion claim; first two command receipts reconstructed, final receipt contemporaneous.')
(HERE/'final-reconciliation.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:result[k] for k in ('source_sha256','binary_sha256','final_trace_sha256','record_counts','total_diagnostic_calls','reader')},indent=2))
