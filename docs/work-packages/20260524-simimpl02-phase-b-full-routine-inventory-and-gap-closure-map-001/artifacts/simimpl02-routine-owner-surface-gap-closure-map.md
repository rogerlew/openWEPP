# simimpl02 routine owner surface gap closure map

Status: phase-c-and-phase-d-complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Owner surface taxonomy enforced per package objective:
  - `runner`
  - `orchestrator`
  - `kernel`
  - `output`
  - `contract`
  - `unowned-gap`
- Closure status taxonomy:
  - `mapped`
  - `partial`
  - `gap`
  - `deferred`
- High-confidence mapped/partial assignments are limited to routines with direct
  openWEPP evidence anchors from SIMIMPL01/SIMIMPL02 probes; all other rows are
  conservative (`gap` or `deferred`).

## Ran
- Per-routine classification generated from full inventory at
  `/tmp/simimpl02/routine_owner_map.tsv`.
- Summary counts:
  - owner counts: `contract=25`, `kernel=27`, `orchestrator=4`, `output=2`,
    `runner=2`, `unowned-gap=142`
  - status counts: `mapped=5`, `partial=28`, `gap=102`, `deferred=67`

## Rationale code legend
- `R-001`: Runner/output ownership gap for watbal runtime/output closure
  (projection-first publication still active).
- `R-002`: Orchestrator partial mapping (scheduler exists; production runner
  wiring remains incomplete).
- `R-003`: Soil/winter coupling partially mapped through current orchestrator and
  parser surfaces; full legacy closure deferred.
- `R-004`: Kernel lane mapped via existing typed hydrology/decomposition
  execution surfaces.
- `R-005`: Contract-mapped climate seam (`stmget` lineage) through climate
  runtime adapter.
- `R-006`: Unresolved legacy callee symbol in baseline source set.
- `R-007`: Deferred watershed/channel/impoundment/erosion scope outside current
  SIMIMPL hillslope closure wave.
- `R-008`: No direct owner evidence yet; classify as unowned hillslope gap.
- `R-009`: Parser/input contract surfaces exist, but runtime closure remains
  partial.
- `R-010`: Likely kernel-owned hillslope routine family pending consolidated
  kernel intake and lane closure packages.

## Priority closure rows (SIMIMPL03+ driving set)
| Routine | Owner surface | Status | Rationale code | Next queue lane |
|---|---|---|---|---|
| `watbal` | `runner` | `gap` | `R-001` | `simimpl05/simimpl07` |
| `watbal_hourly` | `runner` | `gap` | `R-001` | `simimpl05/simimpl07` |
| `hydout` | `output` | `gap` | `R-001` | `simimpl06` |
| `contin` | `orchestrator` | `partial` | `R-002` | `simimpl05` |
| `soil` | `orchestrator` | `partial` | `R-003` | `simimpl10` |
| `frsoil` | `orchestrator` | `partial` | `R-003` | `simimpl10` |
| `winter` | `orchestrator` | `partial` | `R-003` | `simimpl10` |
| `decomp` | `kernel` | `mapped` | `R-004` | `closed-in-simimpl02` |
| `drain` | `kernel` | `mapped` | `R-004` | `closed-in-simimpl02` |
| `evap` | `kernel` | `mapped` | `R-004` | `closed-in-simimpl02` |
| `evappm` | `kernel` | `mapped` | `R-004` | `closed-in-simimpl02` |
| `stmget` | `contract` | `mapped` | `R-005` | `closed-in-simimpl02` |

## Full owner map (routine, definition_anchor, owner_surface, status, rationale_code, next_queue_lane)
```tsv
annchn	/workdir/wepp-forest_260430_baseline/src/annchn.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
annout	/workdir/wepp-forest_260430_baseline/src/annout.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
appmth	/workdir/wepp-forest_260430_baseline/src/appmth.for:1	unowned-gap	gap	R-008	simimpl10
aspect	/workdir/wepp-forest_260430_baseline/src/aspect.for:1	unowned-gap	gap	R-008	simimpl10
bgnrnd	/workdir/wepp-forest_260430_baseline/src/bgnrnd.for:1	unowned-gap	gap	R-008	simimpl10
bighdr	/workdir/wepp-forest_260430_baseline/src/bighdr.for:1	unowned-gap	gap	R-008	simimpl10
bigout	/workdir/wepp-forest_260430_baseline/src/bigout.for:1	unowned-gap	gap	R-008	simimpl10
brkpt	/workdir/wepp-forest_260430_baseline/src/brkpt.for:1	unowned-gap	gap	R-008	simimpl10
case12	/workdir/wepp-forest_260430_baseline/src/case12.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
case34	/workdir/wepp-forest_260430_baseline/src/case34.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
chncon	/workdir/wepp-forest_260430_baseline/src/chncon.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
chnero	/workdir/wepp-forest_260430_baseline/src/chnero.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
chnrt	/workdir/wepp-forest_260430_baseline/src/chnrt.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
chrqin	/workdir/wepp-forest_260430_baseline/src/chrqin.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
close	/workdir/wepp-forest_260430_baseline/src/close.for:1	contract	partial	R-009	simimpl03/simimpl04
conrun	/workdir/wepp-forest_260430_baseline/src/conrun.for:1	unowned-gap	gap	R-008	simimpl10
const	/workdir/wepp-forest_260430_baseline/src/const.for:1	unowned-gap	gap	R-008	simimpl10
contin	/workdir/wepp-forest_260430_baseline/src/contin.for:1	orchestrator	partial	R-002	simimpl05
convrt	/workdir/wepp-forest_260430_baseline/src/convrt.for:1	unowned-gap	gap	R-008	simimpl10
covcal	/workdir/wepp-forest_260430_baseline/src/covcal.for:1	unowned-gap	gap	R-008	simimpl10
dblex	/workdir/wepp-forest_260430_baseline/src/dblex.for:1	unowned-gap	gap	R-008	simimpl10
dcap	/workdir/wepp-forest_260430_baseline/src/dcap.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
decomp	/workdir/wepp-forest_260430_baseline/src/decomp.for:1	kernel	mapped	R-004	closed-in-simimpl02
depeqs	/workdir/wepp-forest_260430_baseline/src/depeqs.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
depirr	/workdir/wepp-forest_260430_baseline/src/depirr.for:1	unowned-gap	gap	R-008	simimpl10
deplet	/workdir/wepp-forest_260430_baseline/src/deplet.for:1	unowned-gap	gap	R-008	simimpl10
depos	/workdir/wepp-forest_260430_baseline/src/depos.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
depsto	/workdir/wepp-forest_260430_baseline/src/depsto.for:1	unowned-gap	gap	R-008	simimpl10
detach	/workdir/wepp-forest_260430_baseline/src/detach.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
disag	/workdir/wepp-forest_260430_baseline/src/disag.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
drain	/workdir/wepp-forest_260430_baseline/src/drain.for:1	kernel	mapped	R-004	closed-in-simimpl02
eatcom	/workdir/wepp-forest_260430_baseline/src/eatcom.for:11	unowned-gap	gap	R-008	simimpl10
endchn	/workdir/wepp-forest_260430_baseline/src/endchn.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
enddet	/workdir/wepp-forest_260430_baseline/src/enddet.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
endout	/workdir/wepp-forest_260430_baseline/src/endout.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
enrcmp	/workdir/wepp-forest_260430_baseline/src/enrcmp.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
enrich	/workdir/wepp-forest_260430_baseline/src/enrich.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
enrprt	/workdir/wepp-forest_260430_baseline/src/enrprt.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
eqroot	/workdir/wepp-forest_260430_baseline/src/eqroot.for:1	unowned-gap	gap	R-008	simimpl10
erod	/workdir/wepp-forest_260430_baseline/src/erod.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
evap	/workdir/wepp-forest_260430_baseline/src/evap.for:1	kernel	mapped	R-004	closed-in-simimpl02
evappm	/workdir/wepp-forest_260430_baseline/src/evappm.for:1	kernel	mapped	R-004	closed-in-simimpl02
frcfac	/workdir/wepp-forest_260430_baseline/src/frcfac.for:1	unowned-gap	gap	R-008	simimpl10
frichn	/workdir/wepp-forest_260430_baseline/src/frichn.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
frostn	/workdir/wepp-forest_260430_baseline/src/frostn.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
frsoil	/workdir/wepp-forest_260430_baseline/src/frsoil.for:1	orchestrator	partial	R-003	simimpl10
frwatc	/workdir/wepp-forest_260430_baseline/src/frwatc.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
frzng	/workdir/wepp-forest_260430_baseline/src/frzng.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
frznw	/workdir/wepp-forest_260430_baseline/src/frznw.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
fslpar	/workdir/wepp-forest_260430_baseline/src/fslpar.for:1	unowned-gap	gap	R-008	simimpl10
furadv	/workdir/wepp-forest_260430_baseline/src/furadv.for:1	unowned-gap	gap	R-008	simimpl10
furgps	/workdir/wepp-forest_260430_baseline/src/furgps.for:1	unowned-gap	gap	R-008	simimpl10
furlea	/workdir/wepp-forest_260430_baseline/src/furlea.for:1	unowned-gap	gap	R-008	simimpl10
furrec	/workdir/wepp-forest_260430_baseline/src/furrec.for:1	unowned-gap	gap	R-008	simimpl10
furrow	/workdir/wepp-forest_260430_baseline/src/furrow.for:1	unowned-gap	gap	R-008	simimpl10
furrun	/workdir/wepp-forest_260430_baseline/src/furrun.for:1	unowned-gap	gap	R-008	simimpl10
gcurve	/workdir/wepp-forest_260430_baseline/src/gcurve.for:1	unowned-gap	gap	R-008	simimpl10
gdmax	/workdir/wepp-forest_260430_baseline/src/gdmax.for:1	unowned-gap	gap	R-008	simimpl10
getdat	/workdir/wepp-forest_260430_baseline/src/getdat.for:3	contract	partial	R-009	simimpl03/simimpl04
grna	/workdir/wepp-forest_260430_baseline/src/grna.for:1	unowned-gap	gap	R-008	simimpl10
grow	/workdir/wepp-forest_260430_baseline/src/grow.for:1	unowned-gap	gap	R-008	simimpl10
growop	/workdir/wepp-forest_260430_baseline/src/growop.for:1	unowned-gap	gap	R-008	simimpl10
hdreng	/workdir/wepp-forest_260430_baseline/src/hdreng.for:1	unowned-gap	gap	R-008	simimpl10
hdrive	/workdir/wepp-forest_260430_baseline/src/hdrive.for:1	unowned-gap	gap	R-008	simimpl10
hr_tmp	/workdir/wepp-forest_260430_baseline/src/hr_tmp.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
hrtmp	/workdir/wepp-forest_260430_baseline/src/hrtmp.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
hydchn	/workdir/wepp-forest_260430_baseline/src/hydchn.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
hydout	/workdir/wepp-forest_260430_baseline/src/hydout.for:1	output	gap	R-001	simimpl06
idat	/workdir/wepp-forest_260430_baseline/src/idat.for:1	unowned-gap	gap	R-008	simimpl10
impday	/workdir/wepp-forest_260430_baseline/src/impday.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
impeo	/workdir/wepp-forest_260430_baseline/src/impeo.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
impeos	/workdir/wepp-forest_260430_baseline/src/impeos.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
impflo	/workdir/wepp-forest_260430_baseline/src/impflo.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
imphnw	/workdir/wepp-forest_260430_baseline/src/imphnw.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
impint	/workdir/wepp-forest_260430_baseline/src/impint.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
impmai	/workdir/wepp-forest_260430_baseline/src/impmai.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
impmon	/workdir/wepp-forest_260430_baseline/src/impmon.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
imppol	UNRESOLVED	unowned-gap	deferred	R-006	outside-simimpl02-hillslope
imppow	UNRESOLVED	unowned-gap	deferred	R-006	outside-simimpl02-hillslope
imppro	/workdir/wepp-forest_260430_baseline/src/imppro.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
impreg	/workdir/wepp-forest_260430_baseline/src/impreg.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
impris	UNRESOLVED	unowned-gap	deferred	R-006	outside-simimpl02-hillslope
impyr	/workdir/wepp-forest_260430_baseline/src/impyr.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
infile	/workdir/wepp-forest_260430_baseline/src/infile.for:1	contract	partial	R-009	simimpl03/simimpl04
infpar	/workdir/wepp-forest_260430_baseline/src/infpar.for:1	contract	partial	R-009	simimpl03/simimpl04
inidat	/workdir/wepp-forest_260430_baseline/src/inidat.for:1	unowned-gap	gap	R-008	simimpl10
init1	/workdir/wepp-forest_260430_baseline/src/init1.for:1	contract	partial	R-009	simimpl03/simimpl04
initd	/workdir/wepp-forest_260430_baseline/src/initd.for:1	contract	partial	R-009	simimpl03/simimpl04
initgr	/workdir/wepp-forest_260430_baseline/src/initgr.for:1	contract	partial	R-009	simimpl03/simimpl04
input	/workdir/wepp-forest_260430_baseline/src/input.for:1	contract	partial	R-009	simimpl03/simimpl04
irinpt	/workdir/wepp-forest_260430_baseline/src/irinpt.for:1	unowned-gap	gap	R-008	simimpl10
irprnt	/workdir/wepp-forest_260430_baseline/src/irprnt.for:1	unowned-gap	gap	R-008	simimpl10
irrig	/workdir/wepp-forest_260430_baseline/src/irrig.for:1	unowned-gap	gap	R-008	simimpl10
irs	/workdir/wepp-forest_260430_baseline/src/irs.for:1	unowned-gap	gap	R-008	simimpl10
kostia	/workdir/wepp-forest_260430_baseline/src/kostia.for:1	unowned-gap	gap	R-008	simimpl10
locate	/workdir/wepp-forest_260430_baseline/src/locate.for:1	unowned-gap	gap	R-008	simimpl10
mann	/workdir/wepp-forest_260430_baseline/src/mann.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
melt	/workdir/wepp-forest_260430_baseline/src/melt.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
mltbtm	/workdir/wepp-forest_260430_baseline/src/mltbtm.for:1	unowned-gap	gap	R-008	simimpl10
mlttp	/workdir/wepp-forest_260430_baseline/src/mlttp.for:1	unowned-gap	gap	R-008	simimpl10
monchn	/workdir/wepp-forest_260430_baseline/src/monchn.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
monout	/workdir/wepp-forest_260430_baseline/src/monout.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
mxint	/workdir/wepp-forest_260430_baseline/src/mxint.for:1	unowned-gap	gap	R-008	simimpl10
mxreal	/workdir/wepp-forest_260430_baseline/src/mxreal.for:1	unowned-gap	gap	R-008	simimpl10
newrap	/workdir/wepp-forest_260430_baseline/src/newrap.for:1	unowned-gap	gap	R-008	simimpl10
newtil	/workdir/wepp-forest_260430_baseline/src/newtil.for:2	contract	partial	R-009	simimpl03/simimpl04
newton	/workdir/wepp-forest_260430_baseline/src/newton.for:1	unowned-gap	gap	R-008	simimpl10
nowup	/workdir/wepp-forest_260430_baseline/src/nowup.for:1	contract	partial	R-009	simimpl03/simimpl04
open	/workdir/wepp-forest_260430_baseline/src/open.for:1	contract	partial	R-009	simimpl03/simimpl04
outeng	/workdir/wepp-forest_260430_baseline/src/outeng.for:1	unowned-gap	gap	R-008	simimpl10
outfil	/workdir/wepp-forest_260430_baseline/src/outfil.for:1	contract	partial	R-009	simimpl03/simimpl04
param	/workdir/wepp-forest_260430_baseline/src/param.for:1	contract	partial	R-009	simimpl03/simimpl04
patrib	/workdir/wepp-forest_260430_baseline/src/patrib.for:1	unowned-gap	gap	R-008	simimpl10
peak	/workdir/wepp-forest_260430_baseline/src/peak.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
perc	/workdir/wepp-forest_260430_baseline/src/perc.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
pmetcoef	/workdir/wepp-forest_260430_baseline/src/pmetcoef.for:1	contract	partial	R-009	simimpl03/simimpl04
print	/workdir/wepp-forest_260430_baseline/src/print.for:1	contract	partial	R-009	simimpl03/simimpl04
profil	/workdir/wepp-forest_260430_baseline/src/profil.for:1	contract	partial	R-009	simimpl03/simimpl04
prtcmp	/workdir/wepp-forest_260430_baseline/src/prtcmp.for:2	contract	partial	R-009	simimpl03/simimpl04
psis	/workdir/wepp-forest_260430_baseline/src/psis.for:1	unowned-gap	gap	R-008	simimpl10
ptgra	/workdir/wepp-forest_260430_baseline/src/ptgra.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
ptgrp	/workdir/wepp-forest_260430_baseline/src/ptgrp.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
purk	/workdir/wepp-forest_260430_baseline/src/purk.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
qinf	/workdir/wepp-forest_260430_baseline/src/qinf.for:1	unowned-gap	gap	R-008	simimpl10
radcur	/workdir/wepp-forest_260430_baseline/src/radcur.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
rand	/workdir/wepp-forest_260430_baseline/src/rand.for:1	unowned-gap	gap	R-008	simimpl10
range	/workdir/wepp-forest_260430_baseline/src/range.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
rdat	/workdir/wepp-forest_260430_baseline/src/rdat.for:1	unowned-gap	gap	R-008	simimpl10
readin	/workdir/wepp-forest_260430_baseline/src/readin.for:2	contract	partial	R-009	simimpl03/simimpl04
reid	/workdir/wepp-forest_260430_baseline/src/reid.for:1	unowned-gap	gap	R-008	simimpl10
res_dp	/workdir/wepp-forest_260430_baseline/src/res_dp.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
resup	/workdir/wepp-forest_260430_baseline/src/resup.for:1	unowned-gap	gap	R-008	simimpl10
rgraze	/workdir/wepp-forest_260430_baseline/src/rgraze.for:1	unowned-gap	gap	R-008	simimpl10
rgrcur	/workdir/wepp-forest_260430_baseline/src/rgrcur.for:1	unowned-gap	gap	R-008	simimpl10
rherb	/workdir/wepp-forest_260430_baseline/src/rherb.for:1	unowned-gap	gap	R-008	simimpl10
rngint	/workdir/wepp-forest_260430_baseline/src/rngint.for:1	unowned-gap	gap	R-008	simimpl10
rochek	/workdir/wepp-forest_260430_baseline/src/rochek.for:1	unowned-gap	gap	R-008	simimpl10
root	/workdir/wepp-forest_260430_baseline/src/root.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
route	/workdir/wepp-forest_260430_baseline/src/route.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
runge	/workdir/wepp-forest_260430_baseline/src/runge.for:1	unowned-gap	gap	R-008	simimpl10
runout	/workdir/wepp-forest_260430_baseline/src/runout.for:1	unowned-gap	gap	R-008	simimpl10
saxfun	/workdir/wepp-forest_260430_baseline/src/saxfun.for:1	unowned-gap	gap	R-008	simimpl10
saxpar	/workdir/wepp-forest_260430_baseline/src/saxpar.for:1	unowned-gap	gap	R-008	simimpl10
scenhd	/workdir/wepp-forest_260430_baseline/src/scenhd.for:2	contract	partial	R-009	simimpl03/simimpl04
scon	/workdir/wepp-forest_260430_baseline/src/scon.for:1	contract	partial	R-009	simimpl03/simimpl04
scurv	/workdir/wepp-forest_260430_baseline/src/scurv.for:1	unowned-gap	gap	R-008	simimpl10
sedist	/workdir/wepp-forest_260430_baseline/src/sedist.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
sedmax	/workdir/wepp-forest_260430_baseline/src/sedmax.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
sedout	/workdir/wepp-forest_260430_baseline/src/sedout.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
sedseg	/workdir/wepp-forest_260430_baseline/src/sedseg.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
sedsta	/workdir/wepp-forest_260430_baseline/src/sedsta.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
shears	/workdir/wepp-forest_260430_baseline/src/shears.for:1	unowned-gap	gap	R-008	simimpl10
sheart	/workdir/wepp-forest_260430_baseline/src/sheart.for:1	unowned-gap	gap	R-008	simimpl10
sloss	/workdir/wepp-forest_260430_baseline/src/sloss.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
snowd	/workdir/wepp-forest_260430_baseline/src/snowd.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
soil	/workdir/wepp-forest_260430_baseline/src/soil.for:1	orchestrator	partial	R-003	simimpl10
stmget	/workdir/wepp-forest_260430_baseline/src/stmget.for:1	contract	mapped	R-005	closed-in-simimpl02
stmtim	/workdir/wepp-forest_260430_baseline/src/stmtim.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
strip	/workdir/wepp-forest_260430_baseline/src/strip.for:1	unowned-gap	gap	R-008	simimpl10
strout	/workdir/wepp-forest_260430_baseline/src/strout.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
sumfrc	/workdir/wepp-forest_260430_baseline/src/sumfrc.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
sumrnf	/workdir/wepp-forest_260430_baseline/src/sumrnf.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
sumrun	/workdir/wepp-forest_260430_baseline/src/sumrun.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
sunmap	/workdir/wepp-forest_260430_baseline/src/sunmap.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
swu	/workdir/wepp-forest_260430_baseline/src/swu.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
table	/workdir/wepp-forest_260430_baseline/src/table.for:1	unowned-gap	gap	R-008	simimpl10
tfail	/workdir/wepp-forest_260430_baseline/src/tfail.for:1	unowned-gap	gap	R-008	simimpl10
tilage	/workdir/wepp-forest_260430_baseline/src/tilage.for:1	contract	partial	R-009	simimpl03/simimpl04
tmpadj	/workdir/wepp-forest_260430_baseline/src/tmpadj.for:1	unowned-gap	gap	R-008	simimpl10
tmpcft	/workdir/wepp-forest_260430_baseline/src/tmpcft.for:1	unowned-gap	gap	R-008	simimpl10
trncap	/workdir/wepp-forest_260430_baseline/src/trncap.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
trnlos	/workdir/wepp-forest_260430_baseline/src/trnlos.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
undflo	/workdir/wepp-forest_260430_baseline/src/undflo.for:1	unowned-gap	gap	R-008	simimpl10
verchk	/workdir/wepp-forest_260430_baseline/src/verchk.for:1	contract	partial	R-009	simimpl03/simimpl04
watbal	/workdir/wepp-forest_260430_baseline/src/watbal.for:1	runner	gap	R-001	simimpl05/simimpl07
watbal_hourly	/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1	runner	gap	R-001	simimpl05/simimpl07
watbalprint	/workdir/wepp-forest_260430_baseline/src/watbalprint.for:1	output	gap	R-001	simimpl06
watdst	/workdir/wepp-forest_260430_baseline/src/watdst.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
wepp_observe	/workdir/wepp-forest_260430_baseline/src/wepp_observe.for:1	unowned-gap	gap	R-008	simimpl10
winit	/workdir/wepp-forest_260430_baseline/src/winit.for:1	contract	partial	R-009	simimpl03/simimpl04
winter	/workdir/wepp-forest_260430_baseline/src/winter.for:1	orchestrator	partial	R-003	simimpl10
winthd	/workdir/wepp-forest_260430_baseline/src/winthd.for:1	contract	partial	R-009	simimpl03/simimpl04
writeyearlylossbypoint	/workdir/wepp-forest_260430_baseline/src/writeyearlylossbypoint.for:1	unowned-gap	gap	R-008	simimpl10
wshchr	/workdir/wepp-forest_260430_baseline/src/wshchr.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshcqi	/workdir/wepp-forest_260430_baseline/src/wshcqi.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshdrv	/workdir/wepp-forest_260430_baseline/src/wshdrv.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshimp	/workdir/wepp-forest_260430_baseline/src/wshimp.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshini	/workdir/wepp-forest_260430_baseline/src/wshini.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshinp	/workdir/wepp-forest_260430_baseline/src/wshinp.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshiqi	/workdir/wepp-forest_260430_baseline/src/wshiqi.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshirs	/workdir/wepp-forest_260430_baseline/src/wshirs.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshout	/workdir/wepp-forest_260430_baseline/src/wshout.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshpas	/workdir/wepp-forest_260430_baseline/src/wshpas.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshpek	/workdir/wepp-forest_260430_baseline/src/wshpek.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshred	/workdir/wepp-forest_260430_baseline/src/wshred.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshrun	/workdir/wepp-forest_260430_baseline/src/wshrun.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshscs	/workdir/wepp-forest_260430_baseline/src/wshscs.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
wshtc	/workdir/wepp-forest_260430_baseline/src/wshtc.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
xcrit	/workdir/wepp-forest_260430_baseline/src/xcrit.for:1	unowned-gap	deferred	R-007	outside-simimpl02-hillslope
xinflo	/workdir/wepp-forest_260430_baseline/src/xinflo.for:1	kernel	gap	R-010	simimpl08/simimpl09/simimpl10
yalin	/workdir/wepp-forest_260430_baseline/src/yalin.for:1	unowned-gap	gap	R-008	simimpl10
yldopt	/workdir/wepp-forest_260430_baseline/src/yldopt.for:1	unowned-gap	gap	R-008	simimpl10
```
