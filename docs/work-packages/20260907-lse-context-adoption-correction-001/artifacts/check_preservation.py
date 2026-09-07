"""Compact bidirectional relocation index against the immutable reviewed original map."""
from pathlib import Path
import hashlib
import json
import re
import subprocess
ROOT=Path(__file__).resolve().parents[4]
HERE=Path(__file__).resolve().parent
OLD=ROOT/'docs/work-packages/20260907-directory-contract-checker-lse-adoption-001/artifacts'
source=json.loads((OLD/'clause-map.json').read_text())
raw=subprocess.check_output(['git','show',source['source_commit']+':'+source['source_path']],cwd=ROOT)
assert hashlib.sha256(raw).hexdigest()==source['source_sha256']
lines=raw.decode().splitlines()
folder=ROOT/Path(source['source_path']).with_suffix('')
docs={f.name:f.read_text().splitlines() for f in folder.glob('*.md')}
def normal(s):return ' '.join(s.replace('&#124;','|').split())
rows=[]
for item in source['clauses']:
    a,b=item['source_start'],item['source_end']
    text='\n'.join(lines[a-1:b])
    if not text.strip():continue
    name=item['target']
    if 'target_anchor' in item:
        ident=item['target_anchor']
        locations=[(n,i+1) for n,ls in docs.items() for i,l in enumerate(ls) if l.startswith(f'| <a id="{ident}"></a>')]
    else:
        if name=='interface.md':name='common-details.md'
        expected=text.replace('| Invariant ID | Statement','| Binding reference | Statement')
        if item['kind']=='historical':expected=expected.replace('## Change Log','### Change Log')
        if name=='binding-index.md' and text.startswith('| `'):
            key=text.split('|')[1].strip()
            locations=[(name,i+1) for i,l in enumerate(docs[name]) if l.startswith('| '+key+' |')]
        else:locations=[(name,i+1) for i,l in enumerate(docs[name]) if normal(l)==normal(expected)]
    assert locations,(a,b,text)
    name,line=locations[0]
    row=[a,b,name,line,line,item['kind']]
    if rows and a==rows[-1][1]+1 and name==rows[-1][2] and line==rows[-1][4]+1 and item['kind']==rows[-1][5] and a==b:
        rows[-1][1]=b;rows[-1][4]=line
    else:rows.append(row)
result={'source_commit':source['source_commit'],'source_path':source['source_path'],'source_sha256':source['source_sha256'],'columns':['source_start','source_end','target','target_start','target_end','kind'],'note':'Formatting blanks omitted; many-to-one canonical paragraph rows retained. All original scientific cells independently checked in pytest. New routing/effective-scope presentation is independently reviewed, not mapped as original wording.','spans':rows}
(HERE/'source-map.json').write_text(json.dumps(result,indent=2)+'\n')
print(len(rows),'bidirectional scientific relocation spans')
