import sys,re,json,collections,hashlib
from pathlib import Path
p=Path('/workdir/openWEPP/docs/work-packages/20260920-cold-canopy-m1-001/artifacts')
b,c=sys.argv[1:]
def norm(s):
 s=re.sub(r'(?m)^.*= note: `-D clippy::[^`]+` implied by `-D warnings`\n', '', s)
 s=re.sub(r'(?m)^.*= help: to override `-D warnings` add `#\[allow\(clippy::[^)]+\)\]`\n', '', s)
 s=re.sub(r':\d+:\d+',':LINE:COL',s)
 s=re.sub(r'(?m)^\s*\d+(?=\s*[|~+\-])', 'LINE', s)
 s=re.sub(r'(?m)^\s*(-->|\||\.\.\.)',r'\1',s)
 return s.strip()
def blocks(label):
 return collections.Counter(norm(s) for s in re.split(r'(?m)(?=^error(?:\[|:))',(p/(label+'.stderr')).read_text()) if s.startswith('error') and not s.startswith(('error: could not compile','error: command')))
x,y=blocks(b),blocks(c)
d={'evidence_class':'Static: complete source-bearing comparison of Ran strict lint logs; not a strict lint PASS','baseline':b,'candidate':c,'argv_identical':json.loads((p/(b+'.json')).read_text())['argv']==json.loads((p/(c+'.json')).read_text())['argv'],'normalization':'Line/column and numeric diagnostic/suggestion gutters and first-occurrence-only lint-enablement notes; diagnostic headers, locations and source-bearing content remain unchanged','baseline_count':x.total(),'candidate_count':y.total(),'matched_count':(x&y).total(),'introduced':list((y-x).elements()),'removed':list((x-y).elements()),'baseline_stderr_sha256':hashlib.sha256((p/(b+'.stderr')).read_bytes()).hexdigest(),'candidate_stderr_sha256':hashlib.sha256((p/(c+'.stderr')).read_bytes()).hexdigest()}
(p/(c+'-comparison02.json')).write_text(json.dumps(d,indent=2)+'\n');print(json.dumps(d,indent=2))
