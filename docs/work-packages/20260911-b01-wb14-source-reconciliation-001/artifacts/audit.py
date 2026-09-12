#!/usr/bin/env python3
"""Deterministic, read-only reconciliation inventory; no source mutation."""
from __future__ import annotations
import hashlib, json, os, re, sys
from pathlib import Path

SRC = Path('/workdir/openwepp-experiments/b01-wb14-cadence/recovery-inputs')
REPO = Path('/workdir/openWEPP')
OUT = Path('/workdir/openwepp-experiments/b01-wb14-source-reconciliation')
IDENT = REPO / 'docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/artifacts/build145-execution-identity.json'
INPUTS = [
 REPO/'docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/artifacts/R0-composition.patch',
 REPO/'docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/artifacts/R0-untracked-source.tar.gz',
 REPO/'docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/artifacts/R0-source.json',
 REPO/'docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/artifacts/PBC-final-build016.patch',
 REPO/'docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/artifacts/PBC-final-build016-source.json',
 REPO/'docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts/correction145.patch',
 REPO/'docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts/correction145-source.json',
 REPO/'docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts/raw-stop-point145.tar.gz',
 REPO/'docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts/wb14-parent-cadence-repair-candidate.patch',
]
def sha(p: Path) -> str: return hashlib.sha256(p.read_bytes()).hexdigest()
def canonical_digest(mapping: dict[str, str | None]) -> str:
 return hashlib.sha256(json.dumps(mapping, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
def relevant_extra(path: str) -> bool:
 name=Path(path).name
 return (path in {'Cargo.toml','Cargo.lock','.config/nextest.toml','rust-toolchain.toml','rust-toolchain','clippy.toml','.clippy.toml','rustfmt.toml','.rustfmt.toml','flake.nix','flake.lock','shell.nix','default.nix'} or
  path.startswith('.cargo/') or name == 'build.rs' or
  path == 'tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs' or
  path.startswith('tests/integration/land_surface_energy_real_hydrology_shadow_contract/'))
def direct_source_references() -> list[dict[str, str | None]]:
 roots=[SRC/'tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs',
        SRC/'tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs',
        SRC/'tests/integration/land_surface_energy_real_hydrology_shadow_contract/raw_hash_tests.rs']
 found=[]
 pat=re.compile(r'(?:#\[path\s*=\s*|include!\s*\()\s*"([^"]+)"')
 for p in roots:
  for literal in pat.findall(p.read_text()):
   candidate=(p.parent/literal).resolve()
   found.append({'from':str(p.relative_to(SRC)),'literal':literal,
                 'resolved':str(candidate),'inside_source_root':str(candidate).startswith(str(SRC)),
                 'exists':candidate.exists(),'sha256':sha(candidate) if candidate.is_file() else None})
 return found
def main() -> None:
 ident=json.loads(IDENT.read_text()); expected=ident['files']; actual={}; missing=[]
 for name in sorted(expected):
  p=SRC/name
  if p.is_file(): actual[name]=sha(p)
  else: missing.append(name); actual[name]=None
 mismatches=[{'path': n, 'expected_sha256': expected[n], 'actual_sha256': actual[n]} for n in expected if actual[n] != expected[n]]
 expected_digest=canonical_digest(expected); actual_digest=canonical_digest(actual)
 files=[]; file_symlinks=[]; directory_symlinks=[]
 for base, dirs, names in os.walk(SRC, followlinks=False):
  for n in dirs:
   p=Path(base)/n
   if p.is_symlink(): directory_symlinks.append({'path':str(p.relative_to(SRC)),'target':os.readlink(p)})
  for n in names:
   p=Path(base)/n; rel=str(p.relative_to(SRC))
   if p.is_symlink(): file_symlinks.append({'path':rel,'target':os.readlink(p)})
   elif p.is_file(): files.append(rel)
 extras=sorted(set(files)-set(expected)); map_nonregular=[]
 for n in expected:
  p=SRC/n
  if p.exists() and not p.is_file(): map_nonregular.append(n)
 result={'predicate_protocol_sha256':sha(OUT/'protocol.md'),'source_root':str(SRC),'source_root_is_git':(SRC/'.git').exists(),
  'expected_map_sha256':ident['source_sha256'],'computed_expected_map_sha256':expected_digest,
  'actual_map_sha256':actual_digest,'actual_manifest_path_sha256':actual,'manifest_entries':len(expected),
  'matches':len(expected)-len(mismatches),'mismatches':mismatches,'missing_manifest_paths':missing,'manifest_nonregular_paths':map_nonregular,
  'all_regular_files':len(files),'outside_map_regular_files':len(extras),'outside_map_paths':extras,
  'file_symlinks':file_symlinks,'directory_symlinks':directory_symlinks,
  'relevant_outside_map_inputs':[{'path':n,'sha256':sha(SRC/n)} for n in extras if relevant_extra(n)],
  'direct_test_source_references':direct_source_references(),
  'reconstruction_inputs':[{'path':str(p),'exists':p.is_file(),'sha256':sha(p) if p.is_file() else None} for p in INPUTS],
  'historical_stages_reused_not_reexecuted':{'R0_files':701,'final016_changed_files':41},
  'prohibited_work_not_run':['Rust build','Rust test','authentic run','cadence candidate patch application','source edit']}
 (OUT/'actual-byte-inventory.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
 print(json.dumps({'entries':len(expected),'matches':result['matches'],'mismatches':len(mismatches),'extras':len(extras),'file_symlinks':len(file_symlinks),'directory_symlinks':len(directory_symlinks),'expected_map_digest':expected_digest,'actual_map_digest':actual_digest},sort_keys=True))
if __name__=='__main__': main()
