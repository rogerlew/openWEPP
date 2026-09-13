"""Bounded checkpoint command recorder; no validation policy decisions."""
import datetime, hashlib, json, os, pathlib, subprocess, sys
out = pathlib.Path(__file__).resolve().parent
source = pathlib.Path('/workdir/openwepp-experiments/b01-wb14-cadence/baseline-red')
name, expected, *command = sys.argv[1:]
paths = ['crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_context_tests.rs', 'tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs', 'tests/integration/land_surface_energy_real_hydrology_shadow_contract/raw_hash_tests.rs', 'crates/openwepp-hillslope-orchestrator/src/direct_runtime/fixtures/b01_wb14_current_run_boundary_packet.json', 'crates/openwepp-hillslope-orchestrator/src/direct_runtime/fixtures/b01_wb14_owner_seed.json', 'Cargo.toml', 'Cargo.lock', 'crates/openwepp-hillslope-orchestrator/Cargo.toml']
argv = ['nix', 'develop', '/workdir/openWEPP', '--command', 'env', 'CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-cadence-targets/baseline-red', 'RUST_MIN_STACK=67108864'] + command
record = dict(name=name, intended_outcome=expected, argv=argv, cwd=str(source), effective_target='/tmp/openwepp-b01-wb14-cadence-targets/baseline-red', target_evidence='cargo metadata observed 2026-09-13', source_base='reconciled-available145-r1', source_hashes={p:hashlib.sha256((source/p).read_bytes()).hexdigest() for p in paths}, start_utc=datetime.datetime.now(datetime.timezone.utc).isoformat())
rpath=out/(name+'.json')
assert not rpath.exists(), 'Refuse to overwrite an existing command record'
rpath.write_text(json.dumps(record, indent=2)+'\n')
with (out/(name+'.stdout.log')).open('wb') as stdout, (out/(name+'.stderr.log')).open('wb') as stderr:
    result=subprocess.run(argv, cwd=source, stdout=stdout, stderr=stderr)
record.update(exit=result.returncode, end_utc=datetime.datetime.now(datetime.timezone.utc).isoformat())
rpath.write_text(json.dumps(record, indent=2)+'\n')
print(json.dumps(dict(name=name, exit=result.returncode, record=str(rpath))))
sys.exit(result.returncode)
