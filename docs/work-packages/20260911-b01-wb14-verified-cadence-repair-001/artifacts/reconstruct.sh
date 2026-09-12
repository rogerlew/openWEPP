#!/usr/bin/env bash
set -euo pipefail

root=/workdir/openwepp-experiments/b01-wb14-cadence
repo=/workdir/openWEPP
prior="$repo/docs/work-packages/20260910-stage3-snow-accuracy-runtime-001/artifacts"
b01="$repo/docs/work-packages/20260910-stage3-b01-snow-cycle-integration-001/artifacts"
log="$root/logs/reconstruct.log"

if [ -e "$root/working145" ] || [ -e "$root/baseline-red" ] || [ -e "$root/candidate" ]; then
  echo 'REFUSE: a requested reconstruction directory already exists' >&2
  exit 2
fi
mkdir -p "$root/logs" "$root/recovery-inputs"
exec > >(tee "$log") 2>&1

printf 'started_utc=%s\n' "$(date -u +%FT%TZ)"
printf 'repo_head=%s\n' "$(git -C "$repo" rev-parse HEAD)"
sha256sum "$prior/R0-composition.patch" "$prior/R0-untracked-source.tar.gz" \
  "$prior/R0-source.json" "$prior/PBC-final-build016.patch" \
  "$prior/PBC-final-build016-source.json" "$b01/correction145.patch" \
  "$b01/correction145-source.json" "$b01/raw-stop-point145.tar.gz" \
  "$b01/wb14-parent-cadence-repair-candidate.patch"
git -C "$repo" archive e89befa4678eadec039b3e7f7fe0a176af8e9dc5 | tar -x -C "$root/recovery-inputs"
patch -d "$root/recovery-inputs" -p1 < "$prior/R0-composition.patch"
tar -xzf "$prior/R0-untracked-source.tar.gz" -C "$root/recovery-inputs"
python3 - "$root/recovery-inputs" "$prior/R0-source.json" <<'PY'
import hashlib, json, sys
from pathlib import Path
root, manifest = Path(sys.argv[1]), json.loads(Path(sys.argv[2]).read_text())
files = manifest['files']
mismatches=[]
for name, expected in files.items():
    p=root/name
    actual=hashlib.sha256(p.read_bytes()).hexdigest() if p.is_file() else None
    if actual != expected: mismatches.append((name, expected, actual))
print(f'R0_files={len(files)} R0_mismatches={len(mismatches)}')
if mismatches: print(mismatches[:20]); raise SystemExit(1)
PY
patch -d "$root/recovery-inputs" -p1 < "$prior/PBC-final-build016.patch"
python3 - "$root/recovery-inputs" "$prior/PBC-final-build016-source.json" <<'PY'
import hashlib, json, sys
from pathlib import Path
root, manifest = Path(sys.argv[1]), json.loads(Path(sys.argv[2]).read_text())
files = {item['path']: item['sha256'] for item in manifest['changed_files']}
mismatches=[]
for name, expected in files.items():
    p=root/name
    actual=hashlib.sha256(p.read_bytes()).hexdigest() if p.is_file() else None
    if actual != expected: mismatches.append((name, expected, actual))
print(f'final016_changed_files={len(files)} final016_mismatches={len(mismatches)}')
if mismatches: print(mismatches[:20]); raise SystemExit(1)
PY
patch -d "$root/recovery-inputs" -p1 < "$b01/correction145.patch"
tar -xOzf "$b01/raw-stop-point145.tar.gz" build145-execution-identity.json > "$root/build145-execution-identity.json"
tar -xOzf "$b01/raw-stop-point145.tar.gz" build145-prepare.py > "$root/build145-prepare.py"
python3 - "$root/recovery-inputs" "$root/build145-execution-identity.json" <<'PY'
import hashlib, json, sys
from pathlib import Path
root, ident = Path(sys.argv[1]), json.loads(Path(sys.argv[2]).read_text())
files=ident['files']; mismatches=[]
for name, expected in files.items():
    p=root/name
    actual=hashlib.sha256(p.read_bytes()).hexdigest() if p.is_file() else None
    if actual != expected: mismatches.append((name, expected, actual))
canonical=json.dumps(files,sort_keys=True,separators=(',',':')).encode()
digest=hashlib.sha256(canonical).hexdigest()
print(f'working145_files={len(files)} working145_mismatches={len(mismatches)} working145_map_sha256={digest}')
if mismatches: print(mismatches[:20]); raise SystemExit(1)
if digest != ident['source_sha256']: raise SystemExit('working145 map digest mismatch')
PY
mv "$root/recovery-inputs" "$root/working145"
cp -a "$root/working145" "$root/baseline-red"
cp -a "$root/working145" "$root/candidate"
patch -d "$root/candidate" -p1 < "$b01/wb14-parent-cadence-repair-candidate.patch"
sha256sum "$root/candidate/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs" \
  "$root/candidate/crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_preflight.rs" \
  "$root/candidate/Cargo.toml"
printf 'completed_utc=%s\n' "$(date -u +%FT%TZ)"
