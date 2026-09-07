"""One-time, source-bound LSE relocation evidence; not a production document engine."""
from pathlib import Path
import hashlib
import json
import re
import subprocess

ROOT=Path(__file__).resolve().parents[4]
PKG=Path(__file__).resolve().parent
SOURCE='docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md'
BASE='b932db101cce07d0860b45b5ecaa8ddb7f455b58'
raw=subprocess.check_output(['git','show',f'{BASE}:{SOURCE}'],cwd=ROOT)
assert hashlib.sha256(raw).hexdigest()=='7a002bcac2ad640716b52f4bd59326f241d5fd3084d26e56f6a703eb496ed61d'
lines=raw.decode().splitlines()
entry=ROOT/SOURCE
outdir=entry.with_suffix('')
NAMES=['interface','surface-energy','soil-coupling','water-vapor','nonlinear-solve','terminal-support','litter-phase','soil-custody','surface-custody','map-custody','dependency-replay','qualification','binding-index','history']
# Derive all top-level boundaries from source, avoiding accidental hand-split paragraphs.
heading_dest={
'Purpose':'interface','Scientific Scope':'interface','Authority Anchors':'interface','Variables and Units':'interface','Algorithm State':'interface','Algorithm Specification':'interface','Branch and Guard':'interface','Invariants and Invariant':'interface','Producer Obligations':'interface','Symbol Alias':'interface','Constants and Parameters':'interface','Unit-Governance':'interface','Tolerance and Numeric':'interface','Calibration and Identifiability':'interface','Test-Vector':'interface','Binding Exposure':'binding-index','Gap Register':'interface',
'`OPENWEPP_SNOW_FREE_LSE_V1`':'surface-energy','Terminal Receiver':'terminal-support','Child 2C':'terminal-support','Version 8 persistent':'soil-coupling','Version 9 exact liquid':'water-vapor','Version 11 inactive':'nonlinear-solve','Version 12 exact':'nonlinear-solve','Version 13 first':'nonlinear-solve','Version 14 snow-free':'litter-phase','Version 15 Receiver':'soil-custody','Version 16 LSE':'surface-custody','Change Log':'history','`OPENWEPP_SNOW_FREE_LSE_V2`':'nonlinear-solve','Version 9 positive':'terminal-support','Canonical Stage-3':'map-custody','Exact-Surface Parent':'surface-custody','Represented-Snow':'terminal-support','Candidate-Only':'soil-custody','Exact V3 Litter':'litter-phase','Exact Heterogeneous':'litter-phase','Topology-Ranked':'surface-custody','Validated In-Memory':'map-custody','Snow-Free Final':'map-custody','Covered Nonfinal':'map-custody','Stage-3 Identity':'nonlinear-solve','Covered Leaf':'nonlinear-solve','Carrier Parent':'map-custody','Component-Temperature':'dependency-replay','Prospective Stencil':'qualification'}
assignment={}
current='interface'
for i,line in enumerate(lines,1):
    if line.startswith('## '):
        matches=[v for k,v in heading_dest.items() if line[3:].startswith(k)]
        assert len(matches)==1,(i,line,matches)
        current=matches[0]
    assignment[i]=current
# Split V1 at its own mechanism boundaries, retaining exact heading/paragraph bytes.
for lo,hi,name in [(775,879,'soil-coupling'),(880,973,'water-vapor'),(974,1058,'nonlinear-solve'),(1059,1117,'water-vapor')]:
    for i in range(lo,hi+1): assignment[i]=name
# Original support amendment interrupts the V10 narrative; restore that narrative's owner.
start=next(i for i,l in enumerate(lines,1) if l=='It does not invoke hydraulic attenuation, conductance or vulnerability floors,')
end=next(i for i,l in enumerate(lines,1) if l=='## Canonical Stage-3 Accepted-Map Boundary Amendment')
for i in range(start,end):assignment[i]='nonlinear-solve'
# Historical fixed release protocol stays binding qualification, not nonnormative history.
start=next(i for i,l in enumerate(lines,1) if l=='Production retention uses this exact command for baseline and candidate:')
end=next(i for i,l in enumerate(lines,1) if l=='## Prospective Stencil-Aware Dependency-Replay Experiment')
for i in range(start,end): assignment[i]='qualification'

def slug(s): return re.sub(r'[^\w\- ]','',s.lower()).replace(' ','-')
def safe_cells(line):
    # Literal code-span pipes are data, never Markdown delimiters.
    return re.sub(r'`[^`]*`',lambda m:m[0].replace('|','&#124;'),line)
def table(headers,rows):
    return ['| '+' | '.join(headers)+' |','|'+'---|'*len(headers)]+['| '+' | '.join(r)+' |' for r in rows]
def url(a,b): return f'https://github.com/rogerlew/openWEPP/blob/{BASE}/{SOURCE}#L{a}-L{b}'
def owner(ident):
    n=int(ident.rsplit('-',1)[1])
    if ident.startswith('OBL-'):
        if '-P-' in ident:return 'soil-custody' if n==5 else 'surface-custody' if n==6 else 'water-vapor'
        return {1:'water-vapor',2:'water-vapor',3:'soil-coupling',4:'interface',5:'soil-custody',6:'surface-custody',7:'map-custody',8:'surface-custody',9:'terminal-support',10:'soil-custody',11:'litter-phase',12:'litter-phase',13:'surface-custody',14:'map-custody',15:'map-custody',16:'map-custody',17:'nonlinear-solve',18:'nonlinear-solve',19:'map-custody',20:'dependency-replay'}[n]
    if n in [100,103,106,124,125,126]:return 'soil-coupling'
    if n in [101,102]:return 'surface-energy'
    if n in [104,105,107,130]:return 'water-vapor'
    if n in [108,109,110,111,112,113,131,138,139,162,163]:return 'nonlinear-solve'
    if 114<=n<=123 or n==154:return 'terminal-support'
    if 140<=n<=149 or n in [156,157]:return 'litter-phase'
    if n in [150,155]:return 'soil-custody'
    if n in [151,153,158]:return 'surface-custody'
    if n in [152,159,160,161]:return 'map-custody'
    if n==164:return 'dependency-replay'
    return 'interface'

def obligation_scope(ident):
    if ident.startswith('INV-'): return 'See invariant Statement'
    role,n=ident.rsplit('-',2)[1:]; n=int(n)
    if role=='P':
        return {5:'Producers delivering accepted soil thermal credits',6:'Producers delivering accepted surface enthalpy operands'}[n]
    return {
        1:'ET consumers of actual evaporation and latent energy',
        2:'Infiltration/runoff consumers of the LSE water offer',
        3:'Soil/frost consumers of the surface ground-heat transfer',
        4:'All scheduler/direct-path consumers claiming runtime closure',
        5:'Consumers of exact soil thermal enthalpy credits',
        6:'Consumers of exact LSE surface enthalpy operands',
        7:'Canonical Stage-3 accepted-map consumers',
        8:'Exact-surface parent-local chronology consumers',
        9:'Represented-snow consumers with inactive litter',
        10:'Candidate-only exact-soil non-owner beginning consumers',
        11:'V3 litter post-phase capacity-spill consumers',
        12:'Heterogeneous finalized native/ordinary resource join consumers',
        13:'Topology-ranked exact-surface owner consumers',
        14:'Validated in-memory Stage-3 handoff consumers',
        15:'Snow-free final receipt reseal consumers',
        16:'Covered nonfinal physical-map companion consumers',
        17:'Represented-snow ground/soil identity-anchor probe consumers',
        18:'Covered leaf dependency-reuse consumers',
        19:'Carrier parent-static/same-map validation reuse consumers',
        20:'Component-temperature dependency-replay consumers',
    }[n]+'; retain the Statement and linked source model/regime, failure and qualification limits'

def obligation_tests(ident, name):
    if ident.startswith('INV-'): return f'{name}.md#{name}'
    role,n=ident.rsplit('-',2)[1:];n=int(n)
    if n<=4: return 'interface.md#test-vector-obligations'
    titles={
        5:'Version 15 Receiver-Owned Exact Soil-Enthalpy-Carry Amendment',
        6:'Version 16 LSE Surface-Enthalpy Exact-Carry Amendment',
        7:'Canonical Stage-3 Accepted-Map Boundary Amendment',
        8:'Exact-Surface Parent-Local Chronology Amendment',
        9:'Represented-Snow',
        10:'Candidate-Only V2 Soil-Beginning Amendment',
        11:'Exact V3 Litter-Phase Capacity-Spill Amendment',
        12:'Exact Heterogeneous V3 Surface-Resource Join Amendment',
        13:'Topology-Ranked V16 Exact-Surface Owner Amendment',
        14:'Validated In-Memory LSE Custody Handoff Amendment',
        15:'Snow-Free Final-Receipt Reseal Amendment',
        16:'Covered Nonfinal Physical-Only Map Amendment',
        17:'Stage-3 Identity-Anchor Jacobian Amendment',
        18:'Covered Leaf Maximum-Demand Exact-Reuse Amendment',
        19:'Carrier Parent-Static and Same-Map Validation-Once Amendment',
        20:'Component-Temperature Jacobian Dependency-Replay Amendment',
    }
    # Source-bound exact heading, never a guessed empty old obligation bin.
    actual=next(l[3:] for l in lines if l.startswith('## ') and l[3:].startswith(titles[n].split(' Amendment')[0]))
    return f'{name}.md#{slug(actual)}'

ids={}; removed=set(); canonical={name:[] for name in NAMES}; source_map=[]
ID=r'(?:INV-LANDSURFACEENERGY-\d+|OBL-LANDSURFACEENERGY-[PC]-\d+)'
# First substantive table definition, excluding guard maps, BEI and references.
header=[]
for i,line in enumerate(lines,1):
    if line.startswith('|'):
        cells=[c.strip() for c in safe_cells(line).strip('|').split('|')]
        if line.startswith('| Invariant ID') or line.startswith('| ID |') or line.startswith('| New guard |'):
            header=cells
        m=re.fullmatch('`('+ID+')`',cells[0])
        if m and m[1] not in ids and ('Statement' in header or header==['ID','Binding V1 rule'] or header==['ID','Binding rule'] or 'Binding rule' in header or 'Binding V3 rule' in header or header==['New guard','Required result']):
            ident=m[1];name=owner(ident)
            if len(cells)==6 and 'Statement' in header:
                row=cells[:]
            else:
                row=[cells[0],cells[1],f'[Original authority]({url(i,i)})',f'[INFERENCE][Static]; [original rule/context]({url(i,i)})',cells[2] if len(cells)>2 else f'Existing coupled domain/closure guards in [ordered solve](nonlinear-solve.md#ordered-numerical-algorithm-active-branches-and-error-precedence) and [typed errors](water-vapor.md#independent-closure-and-errors); [source]({url(i,i)})',cells[2] if len(cells)>2 else 'Existing typed domain/convergence/closure failure: [error map](water-vapor.md#independent-closure-and-errors); no partial state or promotion']
            row[0]=f'<a id="{ident}"></a> `{ident}`'
            ids[ident]=(name,i,i);canonical[name].append(('INV',row,i,i));removed.add(i)
    elif line.startswith('#'):header=[]
# Obligations and the two prose-only invariants retain their complete original paragraph.
i=1
while i<=len(lines):
    m=re.match(r'(?:- )?`('+ID+r')`(?:[: —]| requires)',lines[i-1])
    if m and m[1] not in ids:
        ident=m[1];j=i
        while j<len(lines) and lines[j].strip() and not lines[j].startswith(('- `','## ')):j+=1
        body=' '.join(l.strip() for l in lines[i-1:j])
        statement=re.sub(r'^(?:- )?`'+re.escape(ident)+r'`\s*[:—]?\s*','',body)
        name=owner(ident)
        row=[f'<a id="{ident}"></a> `{ident}`',safe_cells(statement),('All LSE producer paths; retain named model/regime and reviewed terminal-cutover limits' if ident in [f'OBL-LANDSURFACEENERGY-P-{n:03}' for n in range(1,5)] else obligation_scope(ident)),f'[Original clause]({url(i,j)})',f'Existing local mechanism guards and [coupled error map](water-vapor.md#independent-closure-and-errors); [original obligation]({url(i,j)})',f'[Shared test-vector obligations](interface.md#test-vector-obligations) and [owning detailed requirements]({obligation_tests(ident,name)}), including their named fixture/test and real-consumer requirements; [original source]({url(i,j)})']
        if ident.startswith('INV-'):row=[row[0],row[1],row[3],f'[INFERENCE][Static]; [original rule/context]({url(i,j)})',row[4],row[4]]
        ids[ident]=(name,i,j);canonical[name].append(('INV' if ident.startswith('INV-') else 'OBL',row,i,j));removed.update(range(i,j+1));i=j
    i+=1
all_ids=set(re.findall(ID,raw.decode()))
assert all_ids==set(ids),(all_ids-set(ids),set(ids)-all_ids)
chapters={name:[] for name in NAMES}; anchors={}; used={name:set() for name in NAMES}
fence=None
for i,line in enumerate(lines,1):
    if i<=25:continue # Entry metadata/preamble are coherently revised.
    name=assignment[i]
    if i in removed:continue
    if line.startswith(('```','~~~')):fence=None if fence else line[:3]
    if not fence and re.match(r'^#{2,6} ',line):
        title=re.sub(r'^#+ ','',line);anchor=slug(title)
        if anchor in used[name]:anchor=f'{anchor}-source-{i}'
        used[name].add(anchor);anchors.setdefault(slug(title),[]).append((name,anchor,i))
        chapters[name].append((None,f'<a id="{anchor}"></a>'))
    if line.startswith('| Invariant ID | Statement'):
        line=line.replace('Invariant ID','Binding reference',1)
    if line.startswith('|'):
        line=safe_cells(line)
    chapters[name].append((i,line))

intros={
'interface':'Shared authority applies to every task. The v1/v2 baseline below retains its exact-one, failure and adjacent-owner requirements. Its missing-authority/future/M_l-mutation wording is superseded only for the named V1 supported domain by the current constitutive ownership and equations in surface-energy, soil-coupling and water-vapor. Later model/regime admissions remain limited to their named branches. No contract approval activates production or changes retained qualification HOLDs.',
'surface-energy':'Current V1 snow-free radiation and neutral turbulent physics. V2 imports this physics; V3 imports it for its admitted litter successor. The represented-snow lower boundary has its separate terminal/soil interface authority. No current-temperature, recipient or domain qualifier below is optional.',
'soil-coupling':'Current surface humidity/thermal-state and soil transfer rules, plus the separate persistent represented-snow bottom-volume/first-OFE-node boundary. Select the actual regime; tile and OFE/lane bases never alias. Exact storage representation is owned by soil-custody and surface-custody.',
'water-vapor':'Current signed water/vapor enthalpy and immutable-beginning/current-ingress chronology. The exact one-ULP liquid publication rule applies only at its named boundaries. Litter phase-specific vapor specializes these rules only as specified in litter-phase.',
'nonlinear-solve':'Current complete ordered covered solver and V10-specific specialization. INV-138 retains the exact admitted closed-bound qualifier; valid-current/one-inadmissible-probe summaries do not broaden that scope. INV-139 accepts only the unchanged current iterate. V10 diagonal scaling remains potential/nonpositive-assimilation-only. Identity-anchor and leaf reuse are distinct limited optimizations; component replay additionally requires dependency-replay and its applicable qualification.',
 'terminal-support':'Current receiver/regime and positive-support admission rules. The released covered-forest policy is exactly 60000000000 ns; wire chronology at one nanosecond is not physical admission. Earlier floor-dependent evidence is superseded only as stated. V3 remains snow-free; represented snow uses one separate standard native covered map and retains inactive litter bytes.',
'litter-phase':'Current V3 litter specialization, including conservative post-phase capacity spill and heterogeneous resource joins. The later spill rule extends the initial exact-surface operand enumeration with one named negative spill operand. It does not replace phase, vapor, retained-ingress or WB14 custody.',
'soil-custody':'Current exact soil enthalpy representation and candidate-only non-owner beginning. Exact accepted physical operands, one rounding, restart and real-consumer obligations remain binding. Candidate read custody cannot publish an owner.',
'surface-custody':'Current exact surface high/carry and frozen high-mirror custody. Apply parent-local posture and authenticated topology ordering together with the base receipt rules. The accepted litter spill is an additional negative operand under litter-phase; no original accepted operand is removed.',
'map-custody':'Current validated handoff, final reseal and pending-map custody. V27 consumes the converged pending map’s own physical prefix; the v26 separately charged final physical-map description in historical Change Log is not current authority. V30 extends INV-159 only at the original validation positions and retains full trust-boundary validation.',
'dependency-replay':'Current INV-164/C-020 scientific implementation obligations. Historical revision-31 retention and prospective EXP-R/PC1/SG1 qualifications remain separate in qualification. No implementation, experiment, production retention or selector is activated by this migration.',
'qualification':'Binding qualification and capture requirements, not evidence of successful adoption or current execution authorization. Historical revision-31 thresholds/results remain unchanged. EXP-R changes only its stated gates; PC1 and SG1 retain their exact experimental scope. The frozen prospective kickoff remains binding and owner-paused; format migration does not resume it.',
'binding-index':'Structural schema/definition and original-span preservation locators. Scientific adequacy and equivalence require independent review; ID mentions alone are not definitions.'}
# Mechanism edges are explicit and conditional; whole chapters are bounded concerns.
deps={
'interface':[],
'surface-energy':[('interface','all surface physics','scope, signs, owners and claim limits'),('soil-coupling','surface state or soil boundary evaluation','surface enthalpy, humidity and G'),('water-vapor','signed vapor or accepted energy ledger','enthalpy and immutable water'),('nonlinear-solve','physical implementation or solver correctness','ordered complete solve')],
'soil-coupling':[('interface','all coupling work','shared signs/units/owners'),('water-vapor','liquid/energy closure or infiltration','accepted enthalpy and ingress'),('soil-custody','persistent soil energy or reconstruction','exact high/carry owner'),('surface-custody','persistent surface energy or reconstruction','exact surface owner'),('terminal-support','represented snow or receiver transitions','regime, support and receipts')],
'water-vapor':[('interface','all water/vapor work','units and owner limits'),('soil-coupling','ground/soil transfer or closure','opposite G and thermal state'),('surface-energy','component physical evaluation or closure','radiative and turbulent operands'),('litter-phase','frozen-litter vapor/phase or closure','phase-specific vapor and spill')],
'nonlinear-solve':[('interface','all solver work','authority, owner and qualification boundaries'),('surface-energy','changed evaluator or equivalence/correctness review','all affected radiative/turbulent physics'),('soil-coupling','changed lower boundary or soil evaluator','regime-specific equations'),('water-vapor','changed evaluator/error-order or full solver review','water/enthalpy ordering'),('terminal-support','represented-snow or terminal solves','admission and regime'),('dependency-replay','component-probe reuse review','graph and error/custody proof')],
 'terminal-support':[('interface','all receiver/support work','shared obligations'),('soil-coupling','snow--soil receipt/closure','OFE/lane heat interface'),('map-custody','native map identity or owner join','charged map custody')],
'litter-phase':[('interface','all litter phase work','shared obligations'),('water-vapor','vapor/ingress or closure','accepted signed enthalpy and chronology'),('soil-coupling','physical soil boundary or closure','ground transfer'),('surface-custody','phase/spill energy custody or closure','exact owner and mirrors')],
'soil-custody':[('interface','all exact soil custody','shared obligations'),('soil-coupling','physical operand or closure reconstruction','accepted conduction operands'),('water-vapor','infiltration credit reconstruction','accepted parcel enthalpy')],
'surface-custody':[('interface','all exact surface custody','shared obligations'),('soil-custody','exact dyadic representation','canonical wire/arithmetic definition'),('litter-phase','phase/fusion/spill or closure','accepted physical operands and spill'),('water-vapor','retained-ingress or closure','physical basis conversion and parcel receipts')],
'map-custody':[('interface','all map custody','shared obligations'),('surface-custody','exact owner/receipt/restart work','high/carry and parent chronology'),('terminal-support','represented-snow map or transition','inactive litter/native regime'),('soil-custody','unpublished soil continuation','non-owner versus final promotion')],
'dependency-replay':[('interface','all replay review','authority versus activation'),('nonlinear-solve','all replay correctness','canonical stencils, leaf reuse and errors'),('surface-energy','all replay correctness','longwave/turbulent evaluator dependencies'),('water-vapor','routing/error-order review','liquid chronology'),('qualification','qualification/admission or experiment review','historical versus EXP-R/PC1/SG1 limits')],
'qualification':[('interface','all identity/qualification checks','scope and claim limits'),('dependency-replay','scientific replay/coverage/result claims','graph/custody/errors/forced-complete proof'),('nonlinear-solve','solver scientific result claims','ordered canonical solver')],
'binding-index':[]}
# Cross-owner dependencies use existing unique section fragments, never copied owner authority.
external={
'surface-energy':[('../SC-VEGETATION-001.md#openwepp_c3_woody_v8-coupled-ground-energy-amendment','covered-canopy physics or closure','V8 canopy owner'),('../SC-VEGETATION-001.md#purpose','cross-contract scope','vegetation entry/scope constraints')],
'soil-coupling':[('../SC-SNOWENERGY-001.md#child-2c-shared-snow--canopy-turbulent-carrier-amendment','represented-snow boundary','snow carrier/soil receipt owner'),('../SC-SNOWENERGY-001.md#purpose','crossing represented-snow authority','snow scope and qualification')],
'water-vapor':[('../SC-WATBAL-001.md#wb14-infiltration-and-hyetograph-coupling-addendum','ingress partition or closure','sole WB14 owner'),('../SC-WATBAL-001.md#purpose','cross-contract water scope','water scope'),('../SC-SURFACELIQUID-001.md#algorithm-specification','surface-water custody or closure','accepted water protocol'),('../SC-SURFACELIQUID-001.md#purpose-and-scientific-scope','cross-contract surface scope','surface owner scope')],
'nonlinear-solve':[('../SC-VEGETATION-001.md#openwepp_c3_woody_v10-nonpositive-assimilation-amendment','V10 gas/evaluator equivalence or full solver correctness','V10 gas branch owner'),('../SC-VEGETATION-001.md#purpose','cross-contract vegetation scope','vegetation scope')],
'litter-phase':[('../SC-SURFACELIQUID-001.md#frozen-forest-litter-surface-owner-v2-amendment','litter state or closure','exclusive liquid/ice owner'),('../SC-SURFACELIQUID-001.md#exact-v3-litter-phase-capacity-spill-custody-amendment','phase spill custody or closure','once-only spill'),('../SC-SURFACELIQUID-001.md#exact-heterogeneous-v3-finalized-use-join-amendment','heterogeneous finalized resources','exact native/ordinary join'),('../SC-SURFACELIQUID-001.md#purpose-and-scientific-scope','cross-contract surface scope','surface scope')],
'soil-custody':[('../SC-SURFACELIQUID-001.md#version-15-exact-soil-thermal-enthalpy-carry-amendment','accepted soil credit or closure','receiver/producer exact credit join'),('../SC-SURFACELIQUID-001.md#purpose-and-scientific-scope','cross-contract surface scope','surface scope')],
'surface-custody':[('../SC-SURFACELIQUID-001.md#version-16-exact-lse-surface-enthalpy-carry-amendment','exact surface owner or closure','frozen mirror and operand custody'),('../SC-SURFACELIQUID-001.md#exact-surface-parent-local-chronology-amendment','parent-local receipts/restart','partial/final owner chronology'),('../SC-SURFACELIQUID-001.md#topology-ranked-exact-surface-owner-amendment','topology/receipt reconstruction','opaque topology ordering'),('../SC-SURFACELIQUID-001.md#purpose-and-scientific-scope','cross-contract surface scope','surface scope')],
'terminal-support':[('../SC-SNOWENERGY-001.md#default-off-terminal-receiver-transaction-amendment','terminal transition/receiver','snow event and terminal parcel'),('../SC-COUPLEDTIME-001.md#algorithm-specification','support/event chronology','time admission and transaction owner'),('../SC-COUPLEDTIME-001.md#purpose-and-scientific-scope','cross-contract chronology scope','time scope')],
'map-custody':[('../SC-SNOWENERGY-001.md#adr-0044-nonfinal-physical-only-covered-map-companion','native pending map or publication','same-map custody and physical-prefix owner'),('../SC-COUPLEDTIME-001.md#algorithm-specification','parent commit/restart or map custody','transaction owner')],
'qualification':[('../../../../work-packages/20260906-stage3-prospective-mechanism-experiments-001/prompts/active/kickoff.md#4-experimental-design-one-baseline-two-independent-treatments','EXP-R executable identity/capture checks','frozen execution requirements; not scientific authority'),('../../../../work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/worker-handoff.md#stage-3-current-continuation','EXP-R execution authorization/status','owner pause and frozen execution posture')],
}

outdir.mkdir(exist_ok=True)
for name in NAMES:
    content=[f'[Parent contract](../{entry.name})','']
    if name=='history':
        content+=['<a id="history"></a>','## history Original change log','- status: historical',f'- source_package: {BASE}:{SOURCE}', '- effective_date: 2026-09-07','- verdict: historical','- canonical_binding_ids: none',f'- provenance_anchors: {url(next(i for i,l in enumerate(lines,1) if l=='## Change Log'),next(i for i,l in enumerate(lines,1) if l.startswith('## `OPENWEPP_SNOW_FREE_LSE_V2`'))-1)}','','Original revision history is retained below. It does not override current effective rules.','']
        # One provenance entry; original Change Log heading becomes a subordinate heading.
    else:
        content+=['## Dependencies']
        rows=[[f'{other}.md#{other}',when,why,'whole mechanism chapter'] for other,when,why in deps[name]]
        rows += [[ref,when,why,'named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements'] for ref,when,why in external.get(name,[])]
        content+=table(['Target','Required when','Boundary/obligation','Reading extent'],rows) if rows else ['none beyond entry']
        content+=['',f'<a id="{name}"></a>',f'# {name.replace("-"," ").title()}',intros[name],'']
    for original,line in chapters[name]:
        if name=='history' and line=='## Change Log':line='### Change Log'
        if original:
            source_map.append({'source_start':original,'source_end':original,'target':f'{name}.md','target_line':len(content)+1,'kind':'historical' if name=='history' else 'normative','transform':'verbatim or Markdown table pipe/header encoding'})
        content.append(line)
    if name!='history':
        for kind,headers in [('INV',['Invariant ID','Statement','Authority','Evidence','Guard','Failure posture']),('OBL',['Obligation ID','Statement','Applicability','Authority','Enforcement/failure','Test bindings'])]:
            rows=[item for item in canonical[name] if item[0]==kind]
            if rows:
                content+=['',f'<a id="canonical-{"invariants" if kind=="INV" else "obligations"}"></a>',f'## Canonical {"invariants" if kind=="INV" else "obligations"}']
                for row in rows:
                    _,cells,a,b=row
                    source_map.append({'source_start':a,'source_end':b,'target':f'{name}.md','target_anchor':re.search(r'id="([^"]+)"',cells[0])[1],'kind':'normative','transform':'marked canonical definition; original statement retained, source/guard context made explicit'})
                content+=table(headers,[r[1] for r in rows])
    (outdir/f'{name}.md').write_text('\n'.join(content)+'\n')
# Entries/registries added after all canonical IDs exist.
binding=outdir/'binding-index.md';s=binding.read_text()
# Original BEI source descriptors become exact original Git spans.
bl=s.splitlines();in_bei=False
for i,l in enumerate(bl):
    if l=='## Binding Exposure Index':in_bei=True
    elif in_bei and l.startswith('## '):in_bei=False
    if in_bei and l.startswith('| `'):
        cells=[c.strip() for c in l.strip('|').split('|')]
        original=next(n for n,line in enumerate(lines,1) if line.startswith('| '+cells[0]+' |'))
        cells[1]=f'[Original {cells[0].strip("`")}]({url(original,original)})'
        bl[i]='| '+' | '.join(cells)+' |'
# History source row added to sole BEI; full span mapping retained in package.
idx=next(i for i,l in enumerate(bl) if l.startswith('| `EXP-STAGE3-20260906-R`'))+1
bl.insert(idx,'| `LSE-DIRECTORY-HISTORY` | [Original change log](history.md#history) | `historical` | `historical-or-superseded` | `none` | `none` | Original history retained; no authority demotion. |')
s='\n'.join(bl)+'\n\n## Binding definitions\n'+'\n'.join(table(['ID','Definition'],[[f'`{ident}`',f'{name}.md#{ident}'] for ident,(name,_,_) in sorted(ids.items())]))+'\n\n## Schema coverage\n'
artifact=['purpose','scientific-scope-and-explicit-out-of-scope-boundaries','authority-anchors-with-top-down-citations','variables-and-units-using-canonical-symbols-first','algorithm-state-surfaces','algorithm-specification-with-step-sequence','branch-and-guard-table','invariants-and-invariant-guard-map','producer-obligations-and-consumer-obligations','symbol-alias-map','constants-and-parameters-with-provenance-anchors','unit-governance-map','tolerance-and-numeric-notes','calibration-and-identifiability','test-vector-obligations','binding-exposure-index','gap-register-and-promotability-labels','change-log']
kernel=[artifact[i-1] for i in [1,3,4,5,6,7,8,10,11,12,13,14,15,17]]
rows=[]
for prefix,items in [('artifact',artifact),('kernel',kernel)]:
 for n,a in enumerate(items,1):
    name,target,_=anchors[a][0]
    if a=='change-log':
        # Normative change record belongs to the entry; original historical log is linked.
        ref='../'+entry.name+'#change-log'
    else:ref=f'{name}.md#{target}'
    # Distributed content maps every owning mechanism, not emptied old headings.
    key = artifact.index(a)+1
    owners = {
      1: ['surface-energy','soil-coupling','nonlinear-solve','terminal-support','litter-phase','soil-custody','surface-custody','map-custody','dependency-replay','qualification'],
      2: ['surface-energy','soil-coupling','nonlinear-solve','terminal-support','litter-phase','soil-custody','surface-custody','map-custody','dependency-replay','qualification'],
      3: ['surface-energy','soil-coupling','litter-phase','soil-custody','surface-custody','dependency-replay','qualification'],
      4: ['surface-energy','soil-coupling','water-vapor','litter-phase','soil-custody','surface-custody'],
      5: ['surface-energy','soil-coupling','water-vapor','litter-phase','soil-custody','surface-custody','map-custody','dependency-replay'],
      6: ['surface-energy','soil-coupling','water-vapor','nonlinear-solve','terminal-support','litter-phase','soil-custody','surface-custody','map-custody','dependency-replay'],
      7: ['soil-coupling','water-vapor','nonlinear-solve','terminal-support','litter-phase','soil-custody','surface-custody','map-custody','dependency-replay'],
      8: [name for name in NAMES if any(r[0]=='INV' for r in canonical[name])],
      9: [name for name in NAMES if any(r[0]=='OBL' for r in canonical[name])],
      10: ['surface-energy','soil-coupling','water-vapor','litter-phase','soil-custody','surface-custody','map-custody'],
      11: ['surface-energy','soil-coupling','water-vapor','nonlinear-solve','terminal-support','litter-phase'],
      12: ['surface-energy','soil-coupling','water-vapor','litter-phase','soil-custody','surface-custody'],
      13: ['nonlinear-solve','terminal-support','soil-coupling','soil-custody','surface-custody','qualification'],
      14: ['map-custody','qualification'],
      15: [name for name in NAMES if name not in ['interface','history','binding-index']],
      17: ['qualification'],
    }.get(key, [])
    refs=[] if key==9 else [f'[original schema surface]({ref})']
    for name in owners:
        target = 'canonical-invariants' if key==8 else 'canonical-obligations' if key==9 else name
        refs.append(f'[{name}]({name}.md#{target})')
    ref=' '.join(refs) if refs else ref
    rows.append([f'{prefix}.section.{n:02}',ref,'Applicable; complete mechanism requirements and subordinate rules remain binding'])
s+='\n'.join(table(['Requirement','Canonical target(s)','Applicability'],rows))+'\n';binding.write_text(s)
# Small retained entry, no scientific clause duplicates or test-string padding.
front='\n'.join(lines[:18]).replace('contract_version: 31','contract_version: 32').replace('last_reviewed: 2026-09-04','last_reviewed: 2026-09-07')
assert front.endswith('---')
front=front[:-3]+'contract_format: directory-v1\nbinding_index: SC-LANDSURFACEENERGY-001/binding-index.md#binding-exposure-index\n---'
content=[front,'','# SC-LANDSURFACEENERGY-001 Land-Surface Energy-Balance Process Contract','','Candidate adoption: NOT YET QUALIFIED. Whole-contract reading remains required until this package passes all adoption gates.','Scientific implementation authority, production activation and empirical/release qualification are distinct. This format-only revision changes none of them. Retained FAIL/HOLD and frozen kickoff requirements remain binding.','','<a id="contract-scope"></a>','## Scope and cross-cutting requirements','Read [shared scope, signs, units, owners and qualification limits](SC-LANDSURFACEENERGY-001/interface.md#interface) for every task. The complete declared normative set remains binding. Unknown tasks or uncertain applicability require expansion and reconciliation, never omission. Other single-file contracts retain their whole-contract rules.','','## Document inventory']
content+=table(['Path','Kind','Purpose','Applicability'],[[f'SC-LANDSURFACEENERGY-001/{name}.md','historical' if name=='history' else 'normative',name.replace('-',' '),'audit' if name=='history' else 'all' if name=='interface' else name.replace('-',' ')+' work'] for name in NAMES])
content+=['','## Reading routes']
rows=[]
for task,role,names,trigger in [('Surface-energy and soil coupling','implementation/review',['surface-energy','soil-coupling','water-vapor'],'Changed evaluator adds nonlinear-solve; litter phase adds litter-phase; follow every applicable dependency.'),('Solver correctness and evaluator reuse','correctness reviewer',['nonlinear-solve','dependency-replay','qualification'],'Expand every affected evaluator, error-order and owner dependency; role never excludes physics.'),('Executable identity capture','identity verifier',['qualification'],'Frozen capture protocol remains mandatory; scientific result/closure claims add physical and operand chapters.'),('Physical closure reconstruction','scientific verifier',['water-vapor','soil-coupling','litter-phase','soil-custody','surface-custody'],'Expand radiative/turbulent operands and every affected receiver/lineage boundary.'),('Restart and owner custody','implementation/review',['map-custody','soil-custody','surface-custody'],'Transaction/rollback/publication changes add every affected owner.'),('Unknown or uncertain task','all',['binding-index'],'Read the entire normative set and reconcile applicability before action.')]:
 rows.append([task,role,'entry + [interface](SC-LANDSURFACEENERGY-001/interface.md#interface); '+', '.join(f'[{n}](SC-LANDSURFACEENERGY-001/{n}.md#{n})' for n in names),trigger])
content+=table(['Task','Role/check','Initial material','Expansion trigger'],rows)
content+=['','[Binding index](SC-LANDSURFACEENERGY-001/binding-index.md#binding-exposure-index); [gaps/status](SC-LANDSURFACEENERGY-001/interface.md#gap-register-and-promotability-labels); [qualification](SC-LANDSURFACEENERGY-001/qualification.md#qualification); [original history](SC-LANDSURFACEENERGY-001/history.md#history).','','<a id="change-log"></a>','## Change Log','2026-09-07, revision 32: directory-format relocation with preserved scientific meaning, logical IDs, qualification and activation limits; candidate adoption remains gated.','','## Compatibility anchors']
aliases=[]
for ident,(name,_,_) in sorted(ids.items()):aliases.append((ident,f'SC-LANDSURFACEENERGY-001/{name}.md#{ident}'))
for old,found in anchors.items():
 if len(found)==1 and old not in {'change-log','binding-exposure-index'}:
    name,target,_=found[0]
    if name!='history':aliases.append((old,f'SC-LANDSURFACEENERGY-001/{name}.md#{target}'))
content+=table(['Alias','Target'],[[a,t] for a,t in aliases])
content+=['']+[f'<a id="{a}"></a>' for a,_ in aliases]
entry.write_text('\n'.join(content)+'\n')
# Reconcile final target positions after BEI/coverage insertion and source-link changes.
for item in source_map:
    if 'target_line' not in item:
        continue
    original = lines[item['source_start']-1]
    target_lines = (outdir/item['target']).read_text().splitlines()
    expected = safe_cells(original) if original.startswith('|') else original
    if expected=='## Change Log' and item['target']=='history.md':expected='### Change Log'
    if expected.startswith('| Invariant ID | Statement'):expected=expected.replace('Invariant ID','Binding reference',1)
    if not original.strip():
        item.pop('target_line')
        item['transform']='formatting-only blank line; no scientific clause'
        continue
    candidates=[n for n,l in enumerate(target_lines,1) if l==expected]
    if not candidates and item['target']=='binding-index.md' and original.startswith('| `'):
        key=original.split('|')[1].strip()
        candidates=[n for n,l in enumerate(target_lines,1) if l.startswith('| '+key+' |')]
        item['transform']='BEI Source replaced by pinned original-row provenance; all other cells retained'
    if candidates:
        item['target_line']=min(candidates,key=lambda n:abs(n-item['target_line']))
    else:
        raise AssertionError(('unmapped original clause',item,original))
(PKG/'clause-map.json').write_text(json.dumps({'source_commit':BASE,'source_path':SOURCE,'source_sha256':hashlib.sha256(raw).hexdigest(),'entry_metadata_lines':[1,25],'clauses':source_map,'binding_definitions':ids},indent=2)+'\n')
print(f'Generated {len(NAMES)} chapters, {len(ids)} canonical definitions; candidate, not adopted.')
