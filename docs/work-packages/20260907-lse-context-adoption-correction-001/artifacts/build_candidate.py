"""Source-bound presentation correction; package evidence, not a runtime loader."""
from pathlib import Path
import hashlib
import argparse
import json
import re

ROOT = Path(__file__).resolve().parents[4]
PKG = Path(__file__).resolve().parent
OLD = ROOT / 'docs/work-packages/20260907-directory-contract-checker-lse-adoption-001/artifacts'
REL = Path('docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md')
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output-root', type=Path, help='isolated reproduction root; input authority stays source-bound')
parser.add_argument('--adopted', action='store_true', help='omit candidate restriction only after independently verified adoption')
args = parser.parse_args()
ENTRY = (args.output_root.resolve() if args.output_root else ROOT) / REL
DEST = ENTRY.with_suffix('')
ARCHIVE = OLD / 'candidate-tree' / REL.with_suffix('')
BASE = 'b932db101cce07d0860b45b5ecaa8ddb7f455b58'
SOURCE_URL = 'https://github.com/rogerlew/openWEPP/blob/' + BASE + '/' + str(REL)
NAMES = ['interface','common-details','audit-details','surface-energy','soil-coupling','water-vapor','solve-boundary','nonlinear-solve','terminal-support','litter-phase','soil-custody','surface-custody','map-custody','dependency-replay','qualification','binding-index','history']

def table(headers, rows):
    return '\n'.join(['| ' + ' | '.join(headers) + ' |', '|' + '---|'*len(headers)] + ['| ' + ' | '.join(r) + ' |' for r in rows])

def compact(s):
    # Exact immutable source identity is declared once in always-read interface.
    s = re.sub(r'\[[^\]]+\]\(' + re.escape(SOURCE_URL) + r'#L(\d+)-L(\d+)\)', lambda m: f'v31:L{m[1]}-L{m[2]}', s)
    replacements = {
        '; retain the Statement and linked source model/regime, failure and qualification limits': '',
        'Existing coupled domain/closure guards in [ordered solve]': '[Ordered domain/closure guards]',
        'Existing local mechanism guards and [coupled error map]': 'Local guards; [errors]',
        '[Shared test-vector obligations]': '[Shared tests]',
        '[owning detailed requirements]': '[Mechanism tests]',
        ', including their named fixture/test and real-consumer requirements;': '; named fixtures/tests and real consumers;',
        'Existing typed domain/convergence/closure failure: [error map]': 'Typed domain/convergence/closure: [errors]',
    }
    for old,new in replacements.items(): s=s.replace(old,new)
    result=[]
    for line in s.splitlines(keepends=True):
        if line.startswith('| <a id='):
            seen=set()
            def once(m):
                key=m[1]
                if key in seen:return ''
                seen.add(key)
                return m[0]
            line=re.sub(r'(?:; )?(v31:L\d+-L\d+)',once,line)
        result.append(line)
    return ''.join(result)


def slug(s):
    return re.sub(r'[^\w\- ]','',s.lower()).replace(' ','-')

# Input hashes are immutable reviewed candidate evidence.
manifest = json.loads((OLD/'candidate-recovery.json').read_text())
for row in manifest['overlay']:
    file = OLD/'candidate-tree'/row['path']
    assert hashlib.sha256(file.read_bytes()).hexdigest() == row['sha256'], file

texts = {name:((ARCHIVE/(name+'.md')).read_text() if name=='binding-index' else compact((ARCHIVE/(name+'.md')).read_text())) for name in NAMES if name not in ['common-details','audit-details','solve-boundary']}
old_interface=texts['interface']
body_start=old_interface.index('<a id="purpose">')
def_start=old_interface.index('<a id="canonical-invariants">')
common_body=old_interface[body_start:def_start]
core_definitions=old_interface[def_start:]
# Preserve scientific body/marked rows, replacing only generated routing and source spelling.
for name in list(texts):
    if name=='history':continue
    start=texts[name].index(f'<a id="{name}">')
    texts[name]=texts[name][start:]
texts['common-details']='<a id="common-details"></a>\n# Common details\n'+common_body
texts['interface']='''<a id="interface"></a>
# Shared interface

Energy/mass is positive inward; outgoing water is nonnegative. Preserve units,
interval, lineage and tile/OFE basis; physical binary64 conversion precedes exact
decode. Owners: LSE surface thermal, hydrology water, soil thermal soil state,
vegetation canopy physiology. Keep immutable-beginning/current-ingress chronology,
exactly-once opposite-sign transfers, typed failure and all-owner atomicity.

V1 is snow-free; V2 imports V1 with V10 specialization; V3 adds admitted snow-free
litter phase. Represented snow uses its separate native map and inactive litter.
Regime precedes iteration. Covered support floor: 60000000000 ns; one-nanosecond
structural chronology is not physical admission. Follow applicable support rules.

Effective rules: contract revision 3 (not model V3) supersedes only named V1
missing/future/ownership labels;
conservation/failure/owner rules survive. Litter spill adds its negative exact
operand. V27 finalizes its own pending physical prefix; V30 reuses validation
only at original positions. Detailed mechanism qualifiers remain binding.

Authority does not activate or qualify production. Retain FAIL/HOLD, expected-red
seams, frozen identities and paused EXP-R/PC1/SG1. Identity proves no scientific
result. Uncertainty expands reading; external single-file contracts require full
reading. No silent defaults, unauthorized normalization, replay fallback or duplicate owners.

Provenance shorthand `v31:Lx-Ly` means those exact original lines at commit
`b932db101cce07d0860b45b5ecaa8ddb7f455b58`, path
`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`.
Citations remain binding; logical IDs resolve via binding-index.

'''+core_definitions
# Existing interface links now resolve to their unique current owner.
for name,s in list(texts.items()):
    s=re.sub(r'interface\.md#([^\s)]+)',lambda m: ('interface' if m[1] in {'interface','canonical-invariants','canonical-obligations'} or m[1].startswith(('INV-','OBL-')) else 'common-details')+'.md#'+m[1],s)
    texts[name]=s
# Schema original core locators moved, canonical definitions still interface.
texts['binding-index']=texts['binding-index'].replace('[interface](interface.md#interface)','[shared definitions](interface.md#interface) [common detail](common-details.md#common-details)')

COMMON=['variables-and-units-using-canonical-symbols-first','algorithm-state-surfaces','algorithm-specification-with-step-sequence','branch-and-guard-table','symbol-alias-map','constants-and-parameters-with-provenance-anchors','unit-governance-map','tolerance-and-numeric-notes','calibration-and-identifiability','test-vector-obligations']
# Iteration 2: concern boundaries, preserving complete original sections and rows.
sections=re.split(r'(?=<a id="[^"]+"></a>\n## )',common_body)
audit_anchors=[]; physical=[]; audit=[]
for section in sections:
    if not section.strip():continue
    anchor=re.match(r'<a id="([^"]+)"></a>',section)[1]
    if anchor in COMMON:physical.append(section)
    else:
        audit.append(section)
        audit_anchors.extend(re.findall(r'<a id="([^"]+)"></a>',section))
texts['common-details']='<a id="common-details"></a>\n# Common physical rules\n\n'+''.join(physical)
texts['audit-details']='<a id="audit-details"></a>\n# Authority and enforcement audit details\n\nAll sections remain normative. Read the relevant complete sections for source provenance, enforcement-path, historical applicability or promotability claims. Current physical rules and unresolved authority limits remain in interface and common-details; this relocation does not resolve or demote any gap.\n\n'+''.join(audit)
relocations={('common-details',a):'audit-details' for a in audit_anchors}
for name,anchor in [('nonlinear-solve','stage-3-identity-anchor-jacobian-amendment'),('map-custody','carrier-parent-static-and-same-map-validation-once-amendment')]:
    start=texts[name].index(f'<a id="{anchor}"></a>')
    end=texts[name].index('<a id="canonical-invariants"></a>',start)
    block=texts[name][start:end]
    texts[name]=texts[name][:start]+texts[name][end:]
    insert=texts['dependency-replay'].index('<a id="canonical-invariants"></a>')
    texts['dependency-replay']=texts['dependency-replay'][:insert]+block+texts['dependency-replay'][insert:]
    relocations.update({(name,a):'dependency-replay' for a in re.findall(r'<a id="([^"]+)"></a>',block)})
for name,ids in [('nonlinear-solve',['INV-LANDSURFACEENERGY-162','INV-LANDSURFACEENERGY-163','OBL-LANDSURFACEENERGY-C-017','OBL-LANDSURFACEENERGY-C-018']),('map-custody',['OBL-LANDSURFACEENERGY-C-019'])]:
    for ident in ids:
        row=next(l for l in texts[name].splitlines(keepends=True) if l.startswith(f'| <a id="{ident}"></a>'))
        texts[name]=texts[name].replace(row,'')
        if ident.startswith('INV-'):
            index=texts['dependency-replay'].index('\n<a id="canonical-obligations"></a>')
            texts['dependency-replay']=texts['dependency-replay'][:index].rstrip()+'\n'+row+'\n'+texts['dependency-replay'][index:]
        else:texts['dependency-replay']=texts['dependency-replay'].rstrip()+'\n'+row
        relocations[(name,ident)]='dependency-replay'
texts['solve-boundary']='''<a id="solve-boundary"></a>
# Physical solve boundary

Current ordered physical solve and acceptance boundary for physical rule selection
and reconstruction from accepted primitives. Reviewing solver implementation or
V10 eligibility, partial-root, scaling, wet-coordinate or V11–13 numerical branches
also requires nonlinear-solve. Optimization correctness adds dependency-replay;
accepted primitive reconstruction does not itself claim evaluator/reuse equivalence.

'''
for name,anchor,end_anchor,destination in [
    ('nonlinear-solve','ordered-numerical-algorithm-active-branches-and-error-precedence','version-11-inactive-liquid-vapor-coordinate-domain-amendment','solve-boundary'),
    ('map-custody','snow-free-final-receipt-reseal-amendment','covered-nonfinal-physical-only-map-amendment','surface-custody'),
]:
    start=texts[name].index(f'<a id="{anchor}"></a>');end=texts[name].index(f'<a id="{end_anchor}"></a>',start)
    block=texts[name][start:end];texts[name]=texts[name][:start]+texts[name][end:]
    if destination=='solve-boundary':texts[destination]+=block+'\n<a id="canonical-invariants"></a>\n## Canonical invariants\n'+table(['Invariant ID','Statement','Authority','Evidence','Guard','Failure posture'],[])+'\n'
    else:
        i=texts[destination].index('<a id="canonical-invariants"></a>')
        texts[destination]=texts[destination][:i]+block+texts[destination][i:]
    relocations.update({(name,a):destination for a in re.findall(r'<a id="([^"]+)"></a>',block)})
for name,destination,ids in [
    ('nonlinear-solve','solve-boundary',['INV-LANDSURFACEENERGY-108','INV-LANDSURFACEENERGY-109','INV-LANDSURFACEENERGY-110']),
    ('map-custody','surface-custody',['INV-LANDSURFACEENERGY-160','OBL-LANDSURFACEENERGY-C-015']),
]:
    for ident in ids:
        row=next(l for l in texts[name].splitlines(keepends=True) if l.startswith(f'| <a id="{ident}"></a>'))
        texts[name]=texts[name].replace(row,'')
        if ident.startswith('INV-') and destination!='solve-boundary':
            i=texts[destination].index('\n<a id="canonical-obligations"></a>')
            texts[destination]=texts[destination][:i].rstrip()+'\n'+row+'\n'+texts[destination][i:]
        else:texts[destination]=texts[destination].rstrip()+'\n'+row
        relocations[(name,ident)]=destination
for name,s in list(texts.items()):
    for (old,anchor),new in relocations.items():s=s.replace(f'{old}.md#{anchor}',f'{new}.md#{anchor}')
    texts[name]=s
# Independently reviewed 21-row coverage: scientific requirements remain in the
# complete mechanism bodies and marked definitions; exact profile records stay normative.
profile_pattern=r'(?m)^\| Profile surface \| Binding \|\n(?:\|[^\n]*\n)+'
profiles=re.findall(profile_pattern,texts['dependency-replay'])
assert len(profiles)==3
for version,block in zip([28,29,30],profiles):
    texts['dependency-replay']=texts['dependency-replay'].replace(block,'',1)
    texts['binding-index']+=f'\n<a id="profile-v{version}"></a>\n## V{version} normative profile classification\n\n'+block
texts['dependency-replay']=texts['dependency-replay'].replace('# Dependency Replay\n','# Dependency Replay\n\nV28, V29 and V30 each retain dual independent review and verification. Their original profile classification records remain normative in binding-index; all scientific mechanism and test obligations below remain required.\n',1)
texts['binding-index']=texts['binding-index'].replace('# Binding Index\n','# Binding Index\n\nThe V28–30 profile classification records below are normative authority. Their relocated algorithm/guard/test duties remain in the complete scientific bodies; profile conformance requires both.\n',1)
texts['nonlinear-solve']=texts['nonlinear-solve'].replace('# Nonlinear Solve\n','# Nonlinear Solve\n\nRepresented-snow reuse review covers potential and fixed-final solves, every applicable V10/V11–13 branch below, and complete solve-boundary error/acceptance rules, including initialization and diagnostics.\n\n',1)
SUPPORT='version-9-positive-support-admission-owner-amendment'
ORDER='ordered-numerical-algorithm-active-branches-and-error-precedence'
texts['nonlinear-solve']=texts['nonlinear-solve'].replace('When every positive final water authorization', '### Final-solve initialization and diagnostic accounting\n\nWhen every positive final water authorization',1)
texts['nonlinear-solve']=texts['nonlinear-solve'].replace('Nonpositive-assimilation partial positive root authorization is typed unsupported in V2.', '### V10 domain and numerical restrictions\n\nNonpositive-assimilation partial positive root authorization is typed unsupported in V2.',1)
# Original paragraphs and complete parent extents remain intact beneath these
# natural algorithm/owner subdivisions. These titles add no scientific rule.
subsections = {
    'nonlinear-solve': [
        ('For both owner-uncapped potential and fixed-authorization final solves', '### Witness coordinates and public/persisted diagnostics'),
        ('When every positive final water authorization', '#### Full-supply initialization'),
        ('If that initial evaluation satisfies every residual tolerance', '#### Iteration-zero acceptance'),
        ('Nonpositive-assimilation partial positive root authorization', '#### Authorization exclusions'),
        ('Every covered potential and final solve uses', '#### Derivative scope'),
        ('For that same uncapped active V10', '#### V10 unit scaling'),
        ('When the uncapped V10 nonpositive-assimilation potential evaluation', '#### Potential wet-store eligibility'),
        ('V1-to-V2 migration validates', '#### Identity migration'),
    ],
    'litter-phase': [
        ('Only after the vapor and phase candidates pass', '#### Liquid-only WB14 availability'),
        ('The immutable model tag is `OPENWEPP_SNOW_FREE_LSE_V3`', '#### Identity, restart and receipts'),
        ('This is a mass-custody join, not a second LSE energy operation.', '### Mass custody without additional energy'),
    ],
    'surface-custody': [
        ('For each immutable candidate and each surface key, the receiver executes:', '### Accepted operand reconstruction and destination-group conversion'),
    ],
    'qualification': [
        ('Require nonzero completed component replay on the real primary workload', '### Comparison controls'),
    ],
}
for name, additions in subsections.items():
    for start, heading in additions:
        assert texts[name].count(start) == 1, (name, start)
        texts[name] = texts[name].replace(start, heading + '\n\n' + start, 1)
qualification_intro = 'format migration does not resume it.'
assert texts['qualification'].count(qualification_intro) == 1
texts['qualification'] = texts['qualification'].replace(qualification_intro, qualification_intro + ' Apply each protocol limit to its named performance, memory, scaling or teardown series.', 1)
assert texts['qualification'].count('# Qualification\n') == 1
texts['qualification'] = texts['qualification'].replace('# Qualification\n', '# Replay retention and experiments\n', 1)
assert texts['litter-phase'].count('# Litter Phase\n') == 1
texts['litter-phase'] = texts['litter-phase'].replace('# Litter Phase\n', '# Litter Phase\n\nAccepted-primitive balance reconstruction applies the physical rules, accepted-solve admission checks and receipts below. An independent solver-algorithm or numerical-branch eligibility review additionally requires the full nonlinear-solve chapter. Physical primitive capture follows this mechanism’s operand and real-consumer evidence requirements; replay-experiment qualification is separate.\n', 1)
# Each edge is an authority boundary, not an automatic whole-file dependency.
D={n:[] for n in NAMES}
def dep(owner,target,when,why,extent='section'):
    D[owner].append([target,when,why,extent])
def whole(owner,target,when,why):dep(owner,f'{target}.md#{target}',when,why,'whole mechanism chapter')
for n in NAMES:
    if n not in ['interface','history','binding-index']:
        whole(n,'interface','every task','universal scope, owners, failure and qualification')
dep('interface','common-details.md#common-details','physical rules, solver correctness or accepted-primitive closure','shared physical symbols, state, algorithm, guards, units, tolerances and tests','whole chapter')
dep('interface','qualification.md#qualification','executable identity capture, experiment or qualification claims','frozen protocols and separate scientific/production limits','whole qualification chapter')
# Common detailed schema is not required for identity-only tasks.
texts['common-details']=texts['common-details'].replace('# Common physical rules\n','# Common physical rules\n\nRead this complete chapter for physical rules, solver correctness and accepted-primitive reconstruction. Interface and the selected physical mechanism retain current regime/owner exclusions and effective supersession. Missing coefficients, constitutive authority or run operands remain missing evidence; no default, numerical closure or production qualification follows from this directory. Underlying source, enforcement and historical applicability claims require the corresponding normative audit sections.\n\n',1)
dep('common-details','audit-details.md#authority-anchors-with-top-down-citations','source provenance adjudication','complete source anchors and pinned-source paragraph')
dep('common-details','audit-details.md#invariants-and-invariant-guard-map','enforcement mapping or promotion-path evidence','complete guard map and parent mapping duty')
for anchor in ['purpose','scientific-scope-and-explicit-out-of-scope-boundaries','gap-register-and-promotability-labels']:
    dep('common-details','audit-details.md#'+anchor,'retained model scope, authority gaps, their supersession or promotability adjudication','complete scope and gaps, including successor GAP007/008')
whole('common-details','qualification','historical/current experiment retention, executable identity or capture limits','complete experiment requirements')
texts['common-details']=texts['common-details'].replace('Underlying source, enforcement and historical applicability claims require the corresponding normative audit sections.', 'Applying those current physical, owner and closure rules and retaining their no-promotion limits does not itself adjudicate authority gaps, source provenance or implementation enforcement. Actual source, gap/supersession or enforcement/promotion adjudication requires the corresponding complete audit sections. Historical/current experiment retention and executable-evidence capture limits, including experiment HOLD, require qualification. Physical owner/receipt identity is governed by its mechanism.')
for n in ['surface-energy','soil-coupling','water-vapor','nonlinear-solve','litter-phase','soil-custody','surface-custody']:
    dep(n,'terminal-support.md#'+SUPPORT,'physical solve or receipt interval admission, including snow-free work','physical support, zero support and pre-Newton floor')
whole('nonlinear-solve','solve-boundary','solver correctness or implementation','complete ordered physical algorithm and acceptance')
whole('solve-boundary','nonlinear-solve','solver implementation or V10/V11–13 branch/algorithm review','complete eligibility, scaling, exact stencils and termination')
whole('solve-boundary','dependency-replay','evaluator/reuse equivalence or optimization review','exact reuse predicates, custody, first errors and qualification')
whole('surface-energy','soil-coupling','surface temperature/humidity or ground transfer','thermal state and CN lower boundary')
whole('surface-energy','terminal-support','surface/soil regime-rule selection','complete support, represented-snow and post-event receiver boundary; requirements precede execution')
whole('surface-energy','litter-phase','snow-free forest-litter phase is active or unspecified','complete admitted phase, capacity and ingress rules')
whole('surface-energy','water-vapor','signed vapor, water transaction or energy closure','accepted enthalpy and immutable water')
whole('surface-energy','solve-boundary','physical rule selection or accepted-primitive closure','ordered solve and error precedence; INV108-110')
whole('surface-energy','nonlinear-solve','solver implementation or full evaluator correctness','all active numerical branches')
whole('soil-coupling','water-vapor','liquid/energy closure or infiltration','accepted ingress enthalpy')
whole('soil-coupling','soil-custody','exact soil energy storage or reconstruction','receiver-owned high/carry')
whole('soil-coupling','surface-custody','exact surface storage or reconstruction','surface high/carry owner')
whole('soil-coupling','terminal-support','represented-snow boundary or receiver transition','regime and support receipts')
whole('water-vapor','soil-coupling','ground transfer or energy closure','opposite ground transfer and state')
whole('water-vapor','surface-energy','physical evaluator or energy closure','radiative/turbulent operands')
whole('water-vapor','litter-phase','active frozen-litter vapor/phase or closure','signed phase-specific vapor and spill')
for target,when,why in [('surface-energy','changed evaluator or correctness review','affected radiation/turbulence'),('soil-coupling','changed lower boundary or soil evaluator','regime equations'),('water-vapor','evaluator/error-order correctness','signed vapor and water chronology'),('terminal-support','represented-snow or terminal solve','regime and support'),('dependency-replay','component-temperature probe reuse','graph/custody/fallibility proof')]:whole('nonlinear-solve',target,when,why)
whole('terminal-support','soil-coupling','snow-soil receipt or closure','OFE/lane heat interface')
for target in ['water-vapor','soil-coupling','surface-custody']:whole('litter-phase',target,'active litter phase, ingress or closure','physical and exact operand custody')
whole('soil-custody','soil-coupling','physical operand or closure reconstruction','accepted conduction')
whole('soil-custody','water-vapor','infiltration credit reconstruction','accepted parcel enthalpy')
whole('surface-custody','soil-custody','exact dyadic representation','canonical arithmetic/wire definition')
whole('surface-custody','litter-phase','active phase/fusion/spill or closure','accepted operands and spill')
whole('surface-custody','water-vapor','retained ingress or closure','physical basis and parcel receipts')
for n in ['terminal-support','surface-custody','soil-custody']:
    for a,when in [('validated-in-memory-lse-custody-handoff-amendment','accepted native-map identity or physical receipt origin'),('snow-free-final-receipt-reseal-amendment','snow-free final identity/receipt reseal'),('covered-nonfinal-physical-only-map-amendment','converged pending physical-map origin')]:dep(n,'map-custody.md#'+a,when,'same-map custody and original owner/receipt boundary')
whole('map-custody','surface-custody','exact owner/receipt/restart implementation or reconstruction','high/carry and parent chronology')
whole('map-custody','terminal-support','represented-snow map or transition','inactive litter/native regime')
whole('map-custody','soil-custody','unpublished soil continuation implementation or audit','non-owner versus promotion')
whole('map-custody','dependency-replay','validation-once optimization implementation or error-order audit','V30 detail and C019; complete INV159 remains here')
for target,why in [('nonlinear-solve','canonical stencils, leaf reuse and errors'),('surface-energy','radiation/turbulent dependencies'),('water-vapor','routing and error chronology'),('qualification','retention and capture rules for timing and memory, including oracle separation and matched optional audit posture')]:whole('dependency-replay',target,'component-temperature replay correctness',why)
for a in ['validated-in-memory-lse-custody-handoff-amendment','covered-nonfinal-physical-only-map-amendment','carrier-parent-static-and-same-map-validation-once-amendment']:dep('dependency-replay','map-custody.md#'+a,'replay custody/error-order review','original validation positions and pending-map identity')
whole('qualification','dependency-replay','scientific replay/coverage/result claims','graph, custody, errors and forced-complete proof')
whole('qualification','nonlinear-solve','solver scientific result claims','complete ordered solver')
for version in [28,29,30]:
    dep('dependency-replay',f'binding-index.md#profile-v{version}','V28–30 profile conformance or enforcement-classification audit','original normative profile records')
whole('binding-index','dependency-replay','V28–30 scientific profile conformance or enforcement mapping','complete mechanism and test obligations')
for file,anchor,why in [('experiment-protocol','exp-stage3-20260906-protocol-original-draft-1-plus-prospective-reviewed-amendments','per-series capture, timeout, validity and decision rules'),('authority-input-reproduction','canonical-adjunct-reproduction-evidence','exact source composition and executable-input reconstruction')]:
    dep('qualification','../../../../work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/'+file+'.md#'+anchor,'EXP-R identity/capture or experimental requirements',why,'whole document, including later reviewed amendments')
for n in ['nonlinear-solve','dependency-replay']:
    D[n]=[row for row in D[n] if row[0]!='water-vapor.md#water-vapor']
    for anchor in ['signed-vapor-and-liquid-enthalpy','immutable-beginning-water-transaction-and-current-ingress','independent-closure-and-errors']:
        dep(n,'water-vapor.md#'+anchor,'represented-snow evaluator/error-order requirements review','signed enthalpy, immutable water/ingress and canonical errors')
    whole(n,'water-vapor','active water/ingress implementation or full water-owner audit','complete owner duties')
    for anchor in ['canonical-invariants','canonical-obligations']:
        dep(n,'water-vapor.md#'+anchor,'represented-snow evaluator/error-order requirements review','complete water definitions, including all universal P001–004')
D['surface-energy']=[r for r in D['surface-energy'] if r[0]!='water-vapor.md#water-vapor']
for anchor in ['signed-vapor-and-liquid-enthalpy','immutable-beginning-water-transaction-and-current-ingress','independent-closure-and-errors']:
    dep('surface-energy','water-vapor.md#'+anchor,'represented-snow evaluator/error-order requirements review','signed enthalpy, immutable water/ingress and canonical errors')
whole('surface-energy','water-vapor','snow-free physical rules, active water/ingress, energy closure or water-owner audit','complete accepted enthalpy and immutable-water duties')
for anchor in ['canonical-invariants','canonical-obligations']:
    dep('surface-energy','water-vapor.md#'+anchor,'represented-snow evaluator/error-order requirements review','complete water definitions, including all universal P001–004')
texts['surface-energy']=texts['surface-energy'].replace('# Surface Energy\n', '# Surface Energy\nSurface/soil regime-rule selection includes the represented-snow soil boundary and post-event receiver conditions that delimit the snow-free rules, together with their external authority; selecting a snow-free target does not defer that boundary review.\n',1)
dep('water-vapor','../SC-SNOWFREEZE-001.md#invariants','rain-temperature provider-rule review (including the retained provider) or provider-derived ingress-enthalpy requirements/closure','INV-SNOWFREEZE-075 provider; accepted authoritative parcel reconstruction alone does not rederive it','whole external contract')
# Preserve exact existing external dependencies and their single-file/frozen-protocol extent.
for name in NAMES:
    if name in ['history','common-details','audit-details','solve-boundary','interface']:continue
    old=(ARCHIVE/(name+'.md')).read_text()
    for l in old.splitlines():
        if l.startswith('| ../'):
            cells=[c.strip() for c in l.strip('|').split('|')]
            if len(cells)==4:D[name].append(cells)

for row in D['terminal-support']:
    if row[0].startswith('../SC-SNOWENERGY-001.md#'):
        row[1]='surface/soil regime-rule selection, represented-snow boundary, or terminal transition/receiver'

for name,rows in D.items():
    for row in rows:
        for (old,anchor),new in relocations.items():row[0]=row[0].replace(f'{old}.md#{anchor}',f'{new}.md#{anchor}')
        row[3]=row[3].replace('whole mechanism chapter','whole chapter').replace('named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements','whole external contract; frozen protocol scope')
    rows=[r for r in rows if not r[0].startswith(name+'.md#')]
    grouped={}
    for target,when,why,extent in rows:
        if target=='interface.md#interface':when,why='always','shared authority'
        key=(when,extent)
        group=grouped.setdefault(key,([],[]))
        group[0].append(target)
        if why not in group[1]:group[1].append(why)
    labels={'scientific-scope-and-explicit-out-of-scope-boundaries':'scope','gap-register-and-promotability-labels':'gaps','signed-vapor-and-liquid-enthalpy':'vapor','immutable-beginning-water-transaction-and-current-ingress':'water','independent-closure-and-errors':'errors','canonical-invariants':'invariants','canonical-obligations':'obligations','experiment-protocol':'protocol','authority-input-reproduction':'inputs'}
    labels.update({'exp-stage3-20260906-protocol-original-draft-1-plus-prospective-reviewed-amendments':'protocol','canonical-adjunct-reproduction-evidence':'inputs'})
    D[name]=[]
    for (when,extent),(refs,reasons) in grouped.items():
        def label(ref):
            key=ref.split('#')[-1] if '#' in ref else Path(ref).stem
            return labels.get(key,key)
        target=refs[0] if len(refs)==1 else ', '.join(f'[{label(r)}]({r})' for r in refs)
        D[name].append([target,when,'; '.join(reasons),extent])

DEST.mkdir(parents=True, exist_ok=True)
for name in NAMES:
    if name=='history':s=texts[name]
    else:
        deps=table(['Target','Required when','Boundary/obligation','Reading extent'],D[name]) if D[name] else 'none beyond entry'
        s=f'[Parent contract](../{ENTRY.name})\n\n## Dependencies\n{deps}\n\n'+texts[name]
        s=s.replace('Shared authority applies to every task.','Shared current definitions apply to every task.')
    (DEST/(name+'.md')).write_text(s)
# Direct short anchors share the exact old section heading/body; old anchors survive.
short_sections={
    'solve-boundary': {ORDER:'solve'},
    'soil-custody': {'version-15-receiver-owned-exact-soil-enthalpy-carry-amendment':'exact-soil','candidate-only-v2-soil-beginning-amendment':'soil-beginning'},
    'surface-custody': {'version-16-lse-surface-enthalpy-exact-carry-amendment':'exact-surface','exact-surface-parent-local-chronology-amendment':'parent-chronology','topology-ranked-v16-exact-surface-owner-amendment':'topology','snow-free-final-receipt-reseal-amendment':'reseal'},
    'litter-phase': {'exact-v3-litter-phase-capacity-spill-amendment':'spill','exact-heterogeneous-v3-surface-resource-join-amendment':'resource-join'},
    'dependency-replay': {'component-temperature-jacobian-dependency-replay-amendment':'replay','stage-3-identity-anchor-jacobian-amendment':'identity-anchor','covered-leaf-maximum-demand-exact-reuse-amendment':'leaf-reuse','carrier-parent-static-and-same-map-validation-once-amendment':'validation'},
    'water-vapor': {'independent-closure-and-errors':'errors','signed-vapor-and-liquid-enthalpy':'vapor','immutable-beginning-water-transaction-and-current-ingress':'water'},
    'audit-details': {'scientific-scope-and-explicit-out-of-scope-boundaries':'scope','authority-anchors-with-top-down-citations':'sources','invariants-and-invariant-guard-map':'enforcement','gap-register-and-promotability-labels':'gaps'},
    'common-details': dict(zip(COMMON,['variables','state','algorithm','guards','aliases','constants','units','tolerances','calibration','tests'])),
    'terminal-support': {SUPPORT:'support'},
    'map-custody': {'validated-in-memory-lse-custody-handoff-amendment':'handoff','covered-nonfinal-physical-only-map-amendment':'pending','canonical-stage-3-accepted-map-boundary-amendment':'accepted-map'},
}
for file in DEST.glob('*.md'):
    text=file.read_text()
    for name,anchors in short_sections.items():
        for old,new in anchors.items():
            text=text.replace(f'{name}.md#{old}',f'{name}.md#{new}')
            if file.stem==name:
                needle=f'<a id="{old}"></a>'
                assert needle in text,(file,old)
                text=text.replace(needle,f'<a id="{new}"></a>\n'+needle)
    # Generated marked-row metadata only: retain every original scientific cell.
    result=[]
    for line in text.splitlines(keepends=True):
        if line.startswith('| <a id='):
            line=line.replace('Local guards; [errors]', '[Local guards/errors]')
            line=line.replace('[Shared tests]', '[Tests]').replace(' and [Mechanism tests]', '; [Detail]')
            line=line.replace(f']({file.name}#','](#')
        result.append(line)
    rendered=''.join(result)
    if file.stem=='common-details':rendered=rendered.rstrip('\n')+'\n'
    file.write_text(rendered)
# Compact entry retains metadata and scientific status; unused aliases are removed.
old=(OLD/'candidate-tree'/REL).read_text()
front=old[:old.index('---',4)+3]
prefix='SC-LANDSURFACEENERGY-001/'
rows=[[prefix+n+'.md','historical' if n=='history' else 'normative','replay retention and experiments' if n=='qualification' else n.replace('-',' '),'audit' if n in ['history','binding-index'] else 'selected'] for n in NAMES]
routes=[
['Surface/soil rules','science review','[surface](%ssurface-energy.md#surface-energy), [soil](%ssoil-coupling.md#soil-coupling), [water](%swater-vapor.md#water-vapor)'%(prefix,prefix,prefix),'Follow physical shared set and applicable mechanism dependencies.'],
['Snow replay','correctness review','[solver](%snonlinear-solve.md#nonlinear-solve), [replay](%sdependency-replay.md#dependency-replay), [qualification](%squalification.md#qualification)'%(prefix,prefix,prefix),'All affected physics, errors/custody, support and frozen protocol.'],
['Executable identity','identity verification','[qualification](%squalification.md#qualification)'%prefix,'Frozen capture protocol; scientific claims expand physics/closure.'],
['Frozen-litter closure','closure review',', '.join('[%s](%s%s.md#%s)'%(n,prefix,n,n) for n in ['water-vapor','soil-coupling','litter-phase','soil-custody','surface-custody']),'Physical operands, errors, support and receipt origin; implementation expands full evaluator/solver.'],
['Other/uncertain','all','[binding index](%sbinding-index.md#binding-exposure-index)'%prefix,'Expand full normative set and reconcile applicability.']]
for row, root in zip(routes[:4], ['surface-energy','dependency-replay','qualification','litter-phase']):
    row[2] = f'[{root}]({prefix}{root}.md#{root})'
s=front+'\n\n# LSE contract\n\nCandidate v32; adoption gates pending. Selective reading is authorized only for this\npackage\'s bounded candidate exercises until independent adoption closure.\nRead the complete [shared interface]('+prefix+'interface.md#interface) for every\ntask, then the applicable route and every conditional dependency. The entire normative\nset remains binding. Sections include subsections to the next same/higher heading,\napplicable marked definitions and their guard/test links; inspect each selected\nchapter\'s Dependencies and introduction. Uncertainty expands reading.\n\n## Document inventory\n'+table(['Path','Kind','Purpose','Applicability'],rows)+'\n\n## Reading routes\n'+table(['Task','Role/check','Initial material','Expansion trigger'],routes)+'\n\n<a id="change-log"></a>\n## Change Log\n2026-09-07 v32: coherent directory presentation; scientific authority, production\nHOLD and frozen identities unchanged. History: history.md.\n'
candidate_notice = "Candidate v32; adoption gates pending. Selective reading is authorized only for this\npackage's bounded candidate exercises until independent adoption closure.\n"
if args.adopted:
    assert s.count(candidate_notice) == 1
    s = s.replace(candidate_notice, '', 1)
ENTRY.write_text(s)

# Independently reviewed iteration8 presentation. Original scientific inventories
# stay normative; compact operative interfaces do not authorize new physics.
presentation=json.loads((PKG/'iteration08-presentation.json').read_text())
for old,new in [('nonlinear-solve','numerical-methods'),('qualification','replay-evidence')]:
    (DEST/(new+'.md')).write_bytes((DEST/(old+'.md')).read_bytes())
source = DEST/'common-details.md'
text = source.read_text()
replacements = presentation['common']
original = text
moved = []
for anchor, body in replacements.items():
    match = re.search(r'(<a id="[^"]+"></a>\n)?<a id="'+re.escape(anchor)+r'"></a>\n## [^\n]+\n', text)
    assert match, anchor
    start = match.start()
    tail = re.search(r'\n(?=(?:<a id="[^"]+"></a>\n)+## )', text[match.end():])
    end = match.end()+tail.start()+1 if tail else len(text)
    old = text[start:end]
    new = match[0]+'\n'+body+'\n\n'
    moved.append(old)
    text=text[:start]+new+text[end:]
start=text.index('<a id="step-local-preconditions-intermediates-and-postconditions">')
end=text.index('<a id="guards">',start)
old=text[start:end]
moved.append(old)
new='''<a id="step-local-preconditions-intermediates-and-postconditions"></a>
### Step-Local Preconditions, Intermediates, and Postconditions

The ordered steps above and guard table below retain all step-local admission and
atomicity rules. Each delta, integrated sum and residual is independently finite in
its declared energy/mass units. delta_E/delta_M are the storage changes on the left
of steps4/5; sum_E/sum_M are their respective right-hand sides.
epsilon_E=delta_E-sum_E; epsilon_M=delta_M-sum_M.
Lineage counts are integers exactly one; invalid validation leaves byte-identical
state. Latent Q_LE=LE*dt and Q_evap=-L_v(T_s)*m_evap are finite J m^-2 with identical
lineage; require the admitted latent tolerance or LSEB-E-013 (v1 constitutive/tolerance
authority remains missing). Every advection pair has finite J m^-2 energy and retains its same interval, reference state, temperature/
enthalpy and lineage. Acquisition requires sealed component/water records with
matching state AND interval IDs. The surface G/consumer -G pair is finite W m^-2
and shares the same interval AND lineage. Commit adds no numerical intermediate. The original detailed
table remains normative for tabular-conformance/interpretation in audit-details.

The missing tolerances and constitutive values in steps 4-8 are precisely
`GAP-LANDSURFACEENERGY-001..004`; therefore the table makes the ledger
mechanics reproducible but does not make the runtime algorithm promotable.

'''
text=text[:start]+new+text[end:]
dependency='| audit-details.md#current-schema-reference | API/schema/registry or original tabular-conformance audit | complete retained original inventories and step table; current operative rules remain here | section |\n'
pos=text.index('\n<a id="common-details">')
text=text[:pos].rstrip()+'\n'+dependency+'\n'+text[pos:]
source.write_text(text)
audit=DEST/'audit-details.md'
audit.write_text(audit.read_text().rstrip()+'\n\n<a id="current-schema-reference"></a>\n# Current schema reference\n\nOriginal normative presentation; no obligation is historical or demoted.\n\n'+''.join(moved))
interface=DEST/'interface.md'
current=interface.read_text()
interface.write_text(presentation['interface_prefix']+current[current.index('<a id="canonical-invariants">'):])
labels=[('[Ordered domain/closure guards]','[Solve guards]'),
        ('Typed domain/convergence/closure: [errors]','[Typed failures]'),
        ('; named fixtures/tests and real consumers','; named fixtures/tests/real consumers'),
        ('[Local guards/errors]','[Guards/errors]')]
for f in [ENTRY,*DEST.glob('*.md')]:
    text=f.read_text()
    for old,new in [('nonlinear-solve','numerical-methods'),('qualification','replay-evidence')]:
        text=text.replace(old+'.md#',new+'.md#')
    lines=[]
    for line in text.splitlines(keepends=True):
        if line.startswith('| interface.md#interface | always |'):continue
        if line.startswith('| <a id='):
            for old,new in labels:line=line.replace(old,new)
        lines.append(line)
    rendered=''.join(lines)
    rendered=re.sub(r'(## Dependencies\n)\| Target[^\n]+\n\|[-|]+\n(?=\n)',r'\1none beyond entry\n',rendered)
    f.write_text(rendered)
(DEST/'nonlinear-solve.md').write_text(presentation['admission'])
(DEST/'qualification.md').write_text(presentation['capture'])
plain_changes={
    'common-details': [('including experiment HOLD, require qualification.', 'including experiment HOLD, require replay-evidence.')],
    'dependency-replay': [('qualifications remain separate in qualification.', 'qualifications remain separate in replay-evidence.')],
    'solve-boundary': [
        ('solver implementation or V10/V11–13 branch/algorithm review | complete eligibility, scaling, exact stencils and termination', 'solver implementation, numerical-branch algorithm or evaluator-equivalence review | complete V10 scaling/partial-root and V11–13 stencil/witness algorithms and termination'),
        ('and reconstruction from accepted primitives. Reviewing solver implementation or\nV10 eligibility, partial-root, scaling, wet-coordinate or V11–13 numerical branches\nalso requires nonlinear-solve.', 'and reconstruction from accepted primitives. Complete numerical-methods satisfies accepted-output admission duties.\nThe standalone accepted-primitive route requires the complete nonlinear-solve\ninterface plus its unique definitions, guards and tests. Independent implementation, numerical-branch algorithm (including V10\nscaling/partial-root and V11–13 stencil/witness computation), or evaluator-equivalence\nreview requires whole numerical-methods.'),
    ],
    'litter-phase': [('An independent solver-algorithm or numerical-branch eligibility review additionally requires the full nonlinear-solve chapter.', 'Accepted-primitive admission uses the complete nonlinear-solve interface and its required definitions. Independent solver-algorithm, numerical-branch implementation or evaluator-equivalence review additionally requires full numerical-methods.')],
    'interface': [('executable identity capture, experiment or qualification claims', 'executable identity capture, experiment or replay-retention qualification claims'), ('whole qualification chapter', 'whole chapter')],
}
for name,changes in plain_changes.items():
    f=DEST/(name+'.md');text=f.read_text()
    for old,new in changes:
        assert old in text,(name,old)
        text=text.replace(old,new)
    f.write_text(text)
f=DEST/'replay-evidence.md'
f.write_text(f.read_text().replace('### Comparison controls','### Detailed oracle/audit exclusion from both timing and memory'))
f=DEST/'binding-index.md';text=f.read_text()
for a in ['variables','state','aliases','constants','units']:
    text=text.replace('common-details.md#'+a+')','audit-details.md#'+a+')')
f.write_text(text)
text=ENTRY.read_text()
text=text.replace('| nonlinear solve | selected |','| admission | selected |')
text=text.replace('| replay retention and experiments | selected |','| physical capture | selected |')
prefix='SC-LANDSURFACEENERGY-001/'
newrows='\n'.join('| '+prefix+n+'.md | normative | '+purpose+' | selected |' for n,purpose in [('numerical-methods','methods'),('replay-evidence','replay')])
needle='\n\n## Reading routes'
assert needle in text;text=text.replace(needle,'\n'+newrows+needle)
# Physical closure starts at the complete capture/admission interface; all existing
# core mechanisms and applicable external/definition extents remain mandatory.
old='[litter-phase]('+prefix+'litter-phase.md#litter-phase)'
new='[physical capture]('+prefix+'qualification.md#qualification)'
text=text.replace(old,new)
text='\n'.join(line.replace(' |','|').replace('| ','|') if line.startswith('|') else line for line in text.splitlines())+'\n'
ENTRY.write_text(text)
print('entry',ENTRY.stat().st_size,'interface',(DEST/'interface.md').stat().st_size,'total',ENTRY.stat().st_size+sum(f.stat().st_size for f in DEST.glob('*.md')))
