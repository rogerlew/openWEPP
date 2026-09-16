from pathlib import Path
import json,re
A=Path(__file__).resolve().parent
S=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/crates/openwepp-hillslope-orchestrator/src')
j=json.loads((A/'constructor-type-map.json').read_text())
prefix=Path('/tmp/openwepp_inverse_prefix.rs').read_text()
(A/'constructor_inverse_prefix.rs.txt').write_text(prefix)
out=[prefix]
for name,e in sorted(j.items()):
 ns='direct_runtime' if e['path'].startswith('direct_runtime/') else ('winter_column' if e['path']=='winter_column.rs' else 'hydrology')
 ty=f'crate::{ns}::{name}'
 out.append(f"// Source fields: {e['path']}:{e['line']}\n")
 if e['kind']=='struct':
  out.append(f'captured_struct!({ty} {{'+', '.join(f for f,_ in e['fields'])+'});\n')
 elif name.endswith('ActiveContext'):
  out.append(f'active_context!({ty});\n')
 else:
  variants=re.findall(r'^\s*(\w+),\s*$',e['body'],re.M)
  assert variants,name
  out.append(f'captured_enum!({ty} {{'+', '.join(variants)+'});\n')
(S/'snow_stage3_v11_constructor_inverse.rs').write_text(''.join(out))
