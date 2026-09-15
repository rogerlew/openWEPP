"""Bounded diagnostic comparison aid; raw logs remain authority for manual review."""
import collections
import difflib
import hashlib
import json
from pathlib import Path
import sys

HERE=Path(__file__).resolve().parent
REFERENCE=Path('/workdir/openwepp-experiments/b01-wb14-cadence/current-context-capture-20260913')
CANDIDATE=Path('/workdir/openwepp-experiments/b01-wb14-cadence/snowfree-recorder-candidate-20260915')


def records(path):
    result=[];banners=[];finished=[]
    for line in path.read_text().splitlines():
        try:value=json.loads(line)
        except json.JSONDecodeError:
            banners.append(line);continue
        if value.get('reason')=='compiler-message':result.append(value)
        if value.get('reason')=='build-finished':finished.append(value)
    return result,banners,finished


def source_file(name,root,package):
    p=Path(name)
    choices=[p] if p.is_absolute() else [root/p,root/'crates'/package/p]
    for p in choices:
        if p.is_file():
            try:return p.relative_to(root).as_posix()
            except ValueError:return p.as_posix()
    return name


def comparison(package, candidate_label=None):
    ref_log=HERE/f'baseline-{package}-clippy.stdout'
    cand_log=HERE/((candidate_label or f'candidate-{package}-clippy')+'.stdout')
    ref,banner_ref,finish_ref=records(ref_log)
    cand,banner_cand,finish_cand=records(cand_log)
    crate='openwepp-hillslope-orchestrator' if package=='orchestrator' else 'openwepp-runner'
    mappings={};unmapped=[]
    def line_mapping(name):
        if name in mappings:return mappings[name]
        a=REFERENCE/name;b=CANDIDATE/name
        if not a.is_file() or not b.is_file():mappings[name]=({},[],[]);return mappings[name]
        old=a.read_bytes().splitlines(keepends=True);new=b.read_bytes().splitlines(keepends=True)
        lookup={}
        for block in difflib.SequenceMatcher(a=old,b=new,autojunk=False).get_matching_blocks():
            for k in range(block.size):lookup[block.b+k+1]=block.a+k+1
        starts_a=[0];starts_b=[0]
        for line in old:starts_a.append(starts_a[-1]+len(line))
        for line in new:starts_b.append(starts_b[-1]+len(line))
        mappings[name]=(lookup,starts_a,starts_b);return mappings[name]
    def normalize(value,root,is_candidate=False):
        if isinstance(value,str):return value.replace(str(root),'<SOURCE>')
        if isinstance(value,list):return [normalize(v,root,is_candidate) for v in value]
        if not isinstance(value,dict):return value
        # `rendered` is ANSI/source-position presentation of the structured
        # message. Structured text, children, spans, suggestions and expansions
        # are all retained; raw rendered bytes stay in the original logs.
        out={k:normalize(v,root,is_candidate) for k,v in value.items() if k!='rendered'}
        if 'file_name' in value and 'line_start' in value:
            name=source_file(value['file_name'],root,crate);out['file_name']=name
            if is_candidate and not Path(name).is_absolute():
                lookup,old_starts,new_starts=line_mapping(name)
                first=value['line_start'];last=value['line_end']
                mapped=[lookup.get(n) for n in range(first,last+1)]
                if mapped and all(n is not None for n in mapped) and mapped==list(range(mapped[0],mapped[0]+len(mapped))):
                    out['line_start']=mapped[0];out['line_end']=mapped[-1]
                    out['byte_start']=value['byte_start']+old_starts[mapped[0]-1]-new_starts[first-1]
                    out['byte_end']=value['byte_end']+old_starts[mapped[-1]-1]-new_starts[last-1]
                else:
                    out['unmapped_candidate_span']=True
                    unmapped.append({'path':name,'line_start':first,'line_end':last})
        return out
    old=collections.Counter(json.dumps(normalize(v,REFERENCE),sort_keys=True) for v in ref)
    new=collections.Counter(json.dumps(normalize(v,CANDIDATE,True),sort_keys=True) for v in cand)
    added=new-old;removed=old-new
    return {'evidence_class':'Ran: exact structured diagnostic comparison with exact-equal source-line relocation; manual review assigns acceptance',
            'package':package,'reference_messages':len(ref),'candidate_messages':len(cand),
            'matched_messages':sum((old&new).values()),
            'candidate_unmatched':[{'multiplicity':n,'message':json.loads(v)} for v,n in added.items()],
            'reference_unmatched':[{'multiplicity':n,'message':json.loads(v)} for v,n in removed.items()],
            'candidate_unmapped_spans':unmapped,
            'reference_build_finished':finish_ref,'candidate_build_finished':finish_cand,
            'reference_banners':banner_ref,'candidate_banners':banner_cand,
            'reference_log_sha256':hashlib.sha256(ref_log.read_bytes()).hexdigest(),
            'candidate_log_sha256':hashlib.sha256(cand_log.read_bytes()).hexdigest(),
            'limits':'Does not waive new/changed diagnostics, complete-target obligations, compiler errors or missing tests. ANSI rendered presentation excluded; all structured diagnostics retained.'}


if __name__=='__main__':
    result=comparison(sys.argv[1], sys.argv[2] if len(sys.argv)>2 else None);(HERE/f'{sys.argv[1]}-lint-comparison.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:result[k] for k in ['package','reference_messages','candidate_messages','matched_messages']}))
    print('unmatched candidate/reference',len(result['candidate_unmatched']),len(result['reference_unmatched']))
