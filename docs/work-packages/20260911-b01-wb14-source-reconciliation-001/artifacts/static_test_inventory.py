#!/usr/bin/env python3
"""Read-only source inventory for the two available test modules and nearby harnesses."""
from __future__ import annotations
import hashlib, json, re, subprocess
from pathlib import Path

REPO=Path('/workdir/openWEPP')
SRC=Path('/workdir/openwepp-experiments/b01-wb14-cadence/recovery-inputs')
OUT=Path('/workdir/openwepp-experiments/b01-wb14-source-reconciliation')
TESTDIR=SRC/'tests/integration/land_surface_energy_real_hydrology_shadow_contract'
def sha(b: bytes)->str: return hashlib.sha256(b).hexdigest()
def line_at(s:str, pos:int)->int: return s.count('\n',0,pos)+1
def fn_entries(p:Path):
 s=p.read_text(); out=[]
 for m in re.finditer(r'(?m)^fn ([A-Za-z0-9_]+)\([^\n]*\)\s*(?:->[^\{]+)?\{',s):
  depth=0; end=m.end()-1
  for i in range(m.end()-1,len(s)):
   depth += (s[i]=='{')-(s[i]=='}')
   if depth==0: end=i+1; break
  body=s[m.start():end]
  out.append({'name':m.group(1),'line':line_at(s,m.start()),
   'test': '#[test]' in s[max(0,m.start()-100):m.start()],
   'ignored': '#[ignore' in s[max(0,m.start()-180):m.start()],
   'error_codes':sorted(set(re.findall(r'DirectSurfaceLiquidErrorCode::(E\d+)',body))),
   'production_calls':sorted(set(re.findall(r'\b(execute_unified_real_hydrology_shadow|execute_surface_liquid_ingress|configure_surface_liquid_shadow|unified_beginning_hydrology_snapshot_sha256)\b',body))),
   'assertion_kinds':sorted(set(re.findall(r'\b(assert_eq!|assert_ne!|assert!|expect_err)(?=\s*\()',body)))})
 return out
def git_versions(path:str):
 commits=subprocess.check_output(['git','-C',str(REPO),'log','--all','--format=%H','--',path],text=True).splitlines()
 rows=[]
 for c in commits:
  b=subprocess.check_output(['git','-C',str(REPO),'show',f'{c}:{path}'])
  rows.append({'commit':c,'sha256':sha(b)})
 return rows
def citation(path:str, lines:list[int]): return [{'path':path,'line':n} for n in lines]
def main():
 files=['precedence_tests.rs','raw_hash_tests.rs']
 tests={f:fn_entries(TESTDIR/f) for f in files}
 test_only={f:[x for x in tests[f] if x['test']] for f in files}
 evidence_paths = ['Cargo.toml', 'crates/openwepp-hillslope-orchestrator/Cargo.toml', 'crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs', 'tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs', 'crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs', 'crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs'] + ['crates/openwepp-hillslope-orchestrator/src/direct_runtime/'+n for n in ['surface_liquid_wb14.rs','surface_liquid_wb14_native_prefix_tests.rs','surface_liquid_ingress.rs','surface_liquid_ingress_context_tests.rs','stage3_committed_publication.rs','stage3_committed_publication_tests.rs']]
 result={'manual_static_evidence': {'classification':'Registration, fixtures, production call paths and nearby harness descriptions are manual static findings; hashes bind their inspected source, not compiler discovery.', 'source_sha256':{n:sha((SRC/n).read_bytes()) for n in evidence_paths}}, 'source_root':str(SRC),'available_files':{f:{'sha256':sha((TESTDIR/f).read_bytes()),'functions':tests[f],'test_functions':test_only[f]} for f in files},
 'registration':{
  'module_parent':citation('crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs',[77,78]),
  'test_cfg':'#[cfg(test)] gates raw_boundary_contract_tests; no Cargo [[test]] path names this root source file',
  'parent_module_source':'tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs',
  'submodule_lines':citation('tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs',[36,37,39,40]),
  'crate_package':'openwepp-hillslope-orchestrator (crates/openwepp-hillslope-orchestrator/Cargo.toml:2)',
  'features':'no feature cfg adjacent to registration; crate feature restart-authority-evidence exists but does not gate it'},
 'fixture_lineage':citation('tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs',[55,77,203,240,313,1385,1741]),
 'production_call_paths':{
  'unified_entry':'crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/real_hydrology_execution.rs:5 execute_unified_real_hydrology_shadow',
  'attachment':'crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs:677 configure_surface_liquid_shadow',
  'snapshot':'crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:866 unified_beginning_hydrology_snapshot_sha256'},
 'git_available_versions':{f:git_versions('tests/integration/land_surface_energy_real_hydrology_shadow_contract/'+f) for f in files},
 'retained_patch_trace':{'searched_patches':['R0-composition.patch','PBC-final-build016.patch','correction145.patch'],'direct_path_occurrences':0,'archive_member_occurrences':0,'meaning':'retained-patch/archive inspection does not identify a source-producing patch/member for either available file'},
 'nearby_harnesses':[
  {'kind':'native-prefix unit','registration':'surface_liquid_wb14.rs:2973 include!(surface_liquid_wb14_native_prefix_tests.rs)','test_file':'direct_runtime/surface_liquid_wb14_native_prefix_tests.rs','tests':'native_inactive_prefix_* lines 2,53,116,154,197; ordinary controls 261,284','entrypoint':'DirectWb14ParentIntervalV1 / validate_native_inactive_wb14_prefix_v1','limit':'not ignored; does not bind day4/interval22/transaction255 real ingress'},
  {'kind':'ingress-context unit','registration':'surface_liquid_ingress.rs:2996 #[path = surface_liquid_ingress_context_tests.rs]','test_file':'direct_runtime/surface_liquid_ingress_context_tests.rs','tests':'cadence_failure_is_e008_with_exact_transaction_and_attempt_hash line 284','entrypoint':'execute_surface_liquid_ingress','limit':'uses day_index=3, interval=0, transaction=411; no fresh-process/parent prefix lifecycle'},
  {'kind':'ignored B01 publication','registration':'stage3_committed_publication.rs:2711 #[path = stage3_committed_publication_tests.rs]','test_file':'direct_runtime/stage3_committed_publication_tests.rs','tests':'actual_day_four_thin_snow_publication_preserves_owner line 748; #[ignore] line 747','entrypoint':'B01.install then publication projection','limit':'no surface-liquid ingress/WB14 or target boundary'}],
 'proposed_owner_only_amendment':{'new_module':'tests/integration/land_surface_energy_real_hydrology_shadow_contract/wb14_prefix_cadence_fresh_process_tests.rs','registration':'add #[path = "land_surface_energy_real_hydrology_shadow_contract/wb14_prefix_cadence_fresh_process_tests.rs"] mod wb14_prefix_cadence_fresh_process_tests; to raw_boundary_contract_tests parent','test_name':'wb14_day4_interval22_transaction255_fresh_process_prefix_cadence_replay','ignore':'#[ignore = "fresh process WB14 prefix/cadence authority"]','proposed_command':'cargo test -p openwepp-hillslope-orchestrator --features restart-authority-evidence land_surface_energy_shadow::raw_boundary_contract_tests::wb14_prefix_cadence_fresh_process_tests::wb14_day4_interval22_transaction255_fresh_process_prefix_cadence_replay -- --ignored --exact --nocapture','cargo_target_note':'The parent is a #[cfg(test)] crate module; the similarly named Cargo [[test]] target points at strict_v8, so no --test selector is valid unless a later owner separately changes target registration.','required_shared_helpers':'real day/parent construction, direct ingress invocation, receipt/owner-byte capture, split/replay serialization, independent operand decoder','scope':'test-only registration/module/helpers; separately adopted, not created or executed here'}}
 result.pop('proposed_owner_only_amendment', None)
 (OUT/'static-test-harness-inventory.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'available_test_count':sum(len(v) for v in test_only.values()),'precedence':len(test_only['precedence_tests.rs']),'raw_hash':len(test_only['raw_hash_tests.rs']),'git_versions':{k:len(v) for k,v in result['git_available_versions'].items()}},sort_keys=True))
if __name__=='__main__': main()
