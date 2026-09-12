#!/usr/bin/env python3
"""Read-only input reconciliation and concrete launch declaration; never launches."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
from datetime import datetime, timezone

REPO = Path('/workdir/openWEPP')
PACKAGE = Path(__file__).resolve().parent.parent
EVIDENCE = Path('/workdir/openwepp-experiments/b01-wb14-available-source-red-witness')
SRC = Path('/workdir/openwepp-experiments/b01-wb14-source-reconciliation/reconstructed-available145')
BINARY = EVIDENCE / 'artifacts/openwepp_runner-identity-stamped.frozen'
OUT = EVIDENCE / 'authentic-continuation-20260912-1'
SCRATCH = EVIDENCE / 'fixture-scratch-continuation-20260912-1'
BASE = '484b03b21fb059c4d2b99c66c4bc7cf7c1c1fe2c'
PIN = 'a13d9a5137658fd5f931dcd7fcd2a03301bd2d30'
SOURCE = 'c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e'
BUILD = '69de3ea11154652532983f91b3e87e6ed1355aa462a9f9fee66c0463f8a8c994'
BIN_HASH = '958e61764d373a2c183764e7f7ff41607a32ef74a2af5cd7b3f299bf1123906f'
TEST = 'hillslope::tests::stage3_snow_accuracy_case'


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def identity(path):
    return {'path': str(path), 'bytes': path.stat().st_size, 'sha256': digest(path)}


def main():
    checks = {}
    result = {'verified_at_utc': datetime.now(timezone.utc).isoformat(),
              'evidence_class': 'Current read-only verification; historical build environment was not captured',
              'build_revision': BASE, 'governing_revision': PIN, 'checks': checks}
    retained = REPO / 'docs/work-packages/20260911-b01-wb14-source-reconciliation-001/artifacts'
    inventory = json.loads((retained / 'actual-byte-inventory.json').read_bytes())
    expected = inventory['actual_manifest_path_sha256']
    actual = {name: digest(SRC / name) for name in expected}
    map_hash = hashlib.sha256(json.dumps(actual, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
    result['source_map'] = {'actual_sha256': map_hash, 'entries': len(actual),
                            'mismatches': [name for name in expected if expected[name] != actual[name]]}
    checks['source_map'] = map_hash == SOURCE and actual == expected and len(actual) == 927
    whole = json.loads((retained / 'fresh-recipe-whole-source-comparison.json').read_bytes())['fresh_regular_file_sha256']
    template_root = SRC / 'tests/fixtures/erosion_multi_ofe_p102'
    fixture_paths = sorted(p for p in template_root.rglob('*') if p.is_file())
    fixture_paths.append(SRC / 'tests/fixtures/cancov_forest/marcell_conifer_mn/p8.man.yaml')
    result['runtime_template_inputs'] = [identity(p) for p in fixture_paths]
    checks['runtime_templates'] = bool(fixture_paths) and all(
        whole.get(str(p.relative_to(SRC))) == digest(p) for p in fixture_paths)
    result['binary'] = identity(BINARY)
    checks['binary'] = result['binary']['sha256'] == BIN_HASH and result['binary']['bytes'] == 48794192
    row_path = EVIDENCE / 'artifacts/selected-cargo-artifact-corrected.jsonl'
    row = json.loads(row_path.read_bytes())
    log_path = EVIDENCE / 'logs/build-corrected.jsonl'
    rows = []
    for line in log_path.read_text().splitlines():
        if line.startswith('{'):
            item = json.loads(line)
            if item.get('reason') == 'compiler-artifact' and item.get('target', {}).get('name') == 'openwepp_runner' and item.get('profile', {}).get('test'):
                rows.append(item)
    checks['selected_cargo_row'] = rows == [row]
    listing_path = EVIDENCE / 'artifacts/compiled-listing-identity-stamped.stdout'
    checks['retained_exact_listing'] = listing_path.read_text().splitlines() == [TEST + ': test', '', '1 test, 0 benchmarks']
    build_path = EVIDENCE / 'artifacts/current-build-inputs.json'
    build_inputs = json.loads(build_path.read_bytes())
    encoded = json.dumps(build_inputs, sort_keys=True, separators=(',', ':')).encode('utf-8')
    result['build_input_digest'] = {'sha256': hashlib.sha256(encoded).hexdigest(),
        'algorithm': "SHA256(json.dumps(inputs, sort_keys=True, separators=(',', ':')).encode('utf-8')); no newline",
        'retained_inputs': identity(build_path)}
    checks['build_input_digest'] = result['build_input_digest']['sha256'] == BUILD and build_inputs['source_sha256'] == SOURCE
    binary_bytes = BINARY.read_bytes()
    checks['embedded_stamps'] = SOURCE.encode() in binary_bytes and BUILD.encode() in binary_bytes
    result['retained_build_evidence'] = [identity(p) for p in (row_path, log_path, EVIDENCE / 'logs/build-corrected.stderr', listing_path)]
    nix_rows = []
    for name in ('flake.nix', 'flake.lock', 'tools/dev/openwepp-env'):
        current = (REPO / name).read_bytes()
        historic = subprocess.check_output(['git', 'show', BASE + ':' + name], cwd=REPO)
        pinned = subprocess.check_output(['git', 'show', PIN + ':' + name], cwd=REPO)
        nix_rows.append(dict(identity(REPO / name), matches_build_revision=current == historic,
                            matches_governing_revision=current == pinned))
    result['actual_repo_nix_inputs'] = nix_rows
    checks['repo_nix_revision_join'] = all(x['matches_build_revision'] and x['matches_governing_revision'] for x in nix_rows)
    integration = REPO / 'docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001'
    collector = integration / 'collect.py'
    predecessor = REPO / 'docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/reproduction/run_series.py'
    cases = integration / 'artifacts/gradual-warm-tail7-cases.json'
    result['collectors_and_cases'] = [identity(p) for p in (collector, predecessor, cases)]
    checks['collectors_pinned'] = all(p.read_bytes() == subprocess.check_output(['git', 'show', PIN + ':' + str(p.relative_to(REPO))], cwd=REPO) for p in (collector, predecessor))
    checks['cases'] = digest(cases) == 'c48cc560dff31102085ebcf852225b2063a1629ad96cfa9f7283364df44fbf3f' and len(json.loads(cases.read_bytes())['cases']['gradual_warm_tail7']['forcing']) == 7
    affinity = sorted(os.sched_getaffinity(0))
    archive_index = json.loads((integration / 'artifacts/raw-stop-point145-index.json').read_bytes())
    expanded = sum(x['bytes'] for x in archive_index['files'])
    result['resources'] = {'available_affinity': affinity, 'selected_cpu': affinity[0],
        'free_bytes': shutil.disk_usage(EVIDENCE).free, 'historical_archive_expanded_bytes': expanded,
        'historical_archive_index': identity(integration / 'artifacts/raw-stop-point145-index.json'),
        'space_margin_requirement': '3 times retained expanded inventory, allowing observation, spool and preservation'}
    checks['durable_space'] = result['resources']['free_bytes'] > 3 * expanded
    checks['new_output'] = not OUT.exists()
    checks['scratch_directory'] = SCRATCH.is_dir()
    argv = [str(REPO / '.venv/bin/python'), str(collector), str(BINARY), str(OUT),
        '--policy', 'B01', '--case', 'gradual_warm_tail7', '--ofes', '1', '--cases', str(cases), '--cpu', str(affinity[0]), '--timeout', '3600']
    result['launch'] = {'argv': argv, 'cwd': str(REPO), 'out': str(OUT),
        'parent_environment_overrides': {'TMPDIR': str(SCRATCH), 'PYTHONDONTWRITEBYTECODE': '1'},
        'relevant_inherited_environment': {k: os.environ[k] for k in ('PATH', 'LD_LIBRARY_PATH', 'RUST_BACKTRACE', 'RUST_LOG', 'TZ') if k in os.environ},
        'collector_environment_policy': 'Unchanged collector removes OPENWEPP_* then installs its explicit treatment; no ambient secret dump',
        'expected_child_argv': ['taskset', '-c', str(affinity[0]), str(BINARY), TEST, '--ignored', '--exact', '--nocapture', '--test-threads=1']}
    result['all_checks_pass'] = all(checks.values())
    destination = PACKAGE / 'artifacts/continuation-preflight.json'
    destination.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({'all_checks_pass': result['all_checks_pass'], 'failed': [k for k, v in checks.items() if not v], 'receipt': str(destination)}))
    raise SystemExit(0 if result['all_checks_pass'] else 1)


if __name__ == '__main__':
    main()
