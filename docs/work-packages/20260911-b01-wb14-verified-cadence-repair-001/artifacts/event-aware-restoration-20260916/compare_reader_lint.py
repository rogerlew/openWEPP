"""Compare explicitly named capped streams; observational only, no gate selection."""
import importlib.util,json,sys
from pathlib import Path
A=Path(__file__).resolve().parent
C=A.parent/'complete-lint-diagnostics-20260916'
spec=importlib.util.spec_from_file_location('comparison',C/'compare-complete.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
Q=Path('/workdir/openwepp-experiments/b01-wb14-cadence/snowfree-recorder-candidate-20260915')
R=Q.parent/'native-context-restoration-reader-20260916'
package,label=sys.argv[1:]
baseline=m.stream(C/f'candidate-{package}.stdout.gz');candidate=m.stream(A/f'{label}.stdout')
result=m.compare(package,baseline,candidate,Q,R)
result['target_coverage']=m.target_coverage(package,json.loads((A.parent/'execution-discretion-20260915/final-lint-target-coverage.json').read_text()),candidate)
result['build_finished']=candidate['finished']
result['scope']='Capped diagnostic observation; never a strict Clippy PASS. Source/content mapping is provisional until independent review.'
result['command_receipt']=label+'.json'
(A/f'{label}-comparison.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:result[k] for k in ['package','baseline_messages','candidate_messages','matched_messages','build_finished']}))
print('coverage complete',all(x['complete'] for x in result['target_coverage']))
for item in result['candidate_unmatched']:
    v=item['message']['message'];span=next((s for s in v['spans'] if s['is_primary']),{})
    print(item['multiplicity'],(v.get('code') or {}).get('code'),span.get('file_name'),span.get('line_start'),v['message'])
