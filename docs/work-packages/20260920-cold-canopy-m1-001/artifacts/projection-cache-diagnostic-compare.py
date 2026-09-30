"""Compare the single authorized diagnostic in each arm without physical replay."""
from pathlib import Path
import json
A=Path(__file__).resolve().parent
records={}
for arm in ('baseline','treatment'):
    records[arm]={}
    for line in (A/f'projection-cache-{arm}-g4-diagnostic.stderr').read_text().splitlines():
        if line.startswith(('PC_DIAGNOSTIC ','SHIFTED_FACE_ATTRIBUTION_CAPTURE ','G4_PROPOSAL_COST ')):
            key,value=line.split(' ',1)
            if key in records[arm]:
                raise RuntimeError('Duplicate diagnostic '+key)
            records[arm][key]=json.loads(value)
b,t=records['baseline'],records['treatment']
if b['PC_DIAGNOSTIC']['outcome']!=t['PC_DIAGNOSTIC']['outcome']:
    raise RuntimeError('Numerical outcome differs')
if b['SHIFTED_FACE_ATTRIBUTION_CAPTURE']!=t['SHIFTED_FACE_ATTRIBUTION_CAPTURE']:
    raise RuntimeError('Complete face/controller/decision capture differs')
for key in ('result','work','work_categories_overlap'):
    if b['G4_PROPOSAL_COST'][key]!=t['G4_PROPOSAL_COST'][key]:
        raise RuntimeError('G4 algorithmic record differs: '+key)
counts={arm:r['PC_DIAGNOSTIC']['projection_count'] for arm,r in records.items()}
if counts!={'baseline':336,'treatment':4}:
    raise RuntimeError('Unexpected reconstruction counts')
result=dict(evidence_class='Ran: retained diagnostic record comparison; no new physical execution',
            complete_outcome_equal=True,complete_face_capture_equal=True,g4_result_work_equal=True,
            projection_reconstruction_counts=counts,
            intentionally_uncompared=['G4_PROPOSAL_COST.initialization_wall_ns','G4_PROPOSAL_COST.timing'],
            typed_error=b['SHIFTED_FACE_ATTRIBUTION_CAPTURE']['typed_error_debug'])
(A/'projection-cache-diagnostic-comparison.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
