# simimpl02 full hillslope routine inventory

Status: phase-b-complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Inventory authority source:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- SIMIMPL root routines used for reachability closure:
  - `contin`
  - `watbal`
  - `watbal_hourly`
  - `winter`
  - `soil`
  - `frsoil`
  - `hydout`
- Inventory method is deterministic and reproducible: parse all `subroutine`
  definitions and `call` edges across baseline `src/*.for`, then compute
  transitive closure from root routines.

## Ran
- Reachability extraction produced:
  - `reachable_routines=202`
  - `reachable_edges=326`
  - `defined_reachable_routines=199`
- Unresolved callee symbols inside reachable set:
  - `imppol`
  - `imppow`
  - `impris`
- Intermediate machine-readable artifacts produced in this execution:
  - `/tmp/simimpl02/reachable.txt`
  - `/tmp/simimpl02/reachable_edges.tsv`
  - `/tmp/simimpl02/routine_metrics.tsv`

## Inventory totals
- Reachable routine symbols from SIMIMPL roots: `202`
- Reachable call edges: `326`
- Reachable symbols with baseline `subroutine` definition anchors: `199`
- Reachable symbols unresolved in baseline `src/*.for`: `3`

## Unresolved-symbol note
- `imppol`, `imppow`, and `impris` are called from impoundment routines
  (`impint`, `impreg`) but do not appear as `subroutine` definitions in
  `/workdir/wepp-forest_260430_baseline/src/*.for`.
- This package classifies them as deferred non-hillslope closure surfaces in the
  owner-surface map.

## Full routine metrics (routine, definition_anchor, in_degree, out_degree)
```tsv
annchn	/workdir/wepp-forest_260430_baseline/src/annchn.for:1	1	0
annout	/workdir/wepp-forest_260430_baseline/src/annout.for:1	1	0
appmth	/workdir/wepp-forest_260430_baseline/src/appmth.for:1	2	0
aspect	/workdir/wepp-forest_260430_baseline/src/aspect.for:1	2	0
bgnrnd	/workdir/wepp-forest_260430_baseline/src/bgnrnd.for:1	1	0
bighdr	/workdir/wepp-forest_260430_baseline/src/bighdr.for:1	2	0
bigout	/workdir/wepp-forest_260430_baseline/src/bigout.for:1	1	2
brkpt	/workdir/wepp-forest_260430_baseline/src/brkpt.for:1	1	0
case12	/workdir/wepp-forest_260430_baseline/src/case12.for:1	1	3
case34	/workdir/wepp-forest_260430_baseline/src/case34.for:1	1	4
chncon	/workdir/wepp-forest_260430_baseline/src/chncon.for:1	1	2
chnero	/workdir/wepp-forest_260430_baseline/src/chnero.for:1	1	3
chnrt	/workdir/wepp-forest_260430_baseline/src/chnrt.for:1	1	8
chrqin	/workdir/wepp-forest_260430_baseline/src/chrqin.for:1	1	1
close	/workdir/wepp-forest_260430_baseline/src/close.for:1	2	0
conrun	/workdir/wepp-forest_260430_baseline/src/conrun.for:1	1	7
const	/workdir/wepp-forest_260430_baseline/src/const.for:1	1	0
contin	/workdir/wepp-forest_260430_baseline/src/contin.for:1	1	41
convrt	/workdir/wepp-forest_260430_baseline/src/convrt.for:1	1	0
covcal	/workdir/wepp-forest_260430_baseline/src/covcal.for:1	1	0
dblex	/workdir/wepp-forest_260430_baseline/src/dblex.for:1	1	1
dcap	/workdir/wepp-forest_260430_baseline/src/dcap.for:1	2	2
decomp	/workdir/wepp-forest_260430_baseline/src/decomp.for:1	2	3
depeqs	/workdir/wepp-forest_260430_baseline/src/depeqs.for:1	1	1
depirr	/workdir/wepp-forest_260430_baseline/src/depirr.for:1	1	0
deplet	/workdir/wepp-forest_260430_baseline/src/deplet.for:1	1	0
depos	/workdir/wepp-forest_260430_baseline/src/depos.for:1	1	1
depsto	/workdir/wepp-forest_260430_baseline/src/depsto.for:1	1	0
detach	/workdir/wepp-forest_260430_baseline/src/detach.for:1	2	2
disag	/workdir/wepp-forest_260430_baseline/src/disag.for:1	1	2
drain	/workdir/wepp-forest_260430_baseline/src/drain.for:1	2	0
eatcom	/workdir/wepp-forest_260430_baseline/src/eatcom.for:11	8	0
endchn	/workdir/wepp-forest_260430_baseline/src/endchn.for:1	1	1
enddet	/workdir/wepp-forest_260430_baseline/src/enddet.for:1	1	1
endout	/workdir/wepp-forest_260430_baseline/src/endout.for:1	1	0
enrcmp	/workdir/wepp-forest_260430_baseline/src/enrcmp.for:1	2	0
enrich	/workdir/wepp-forest_260430_baseline/src/enrich.for:1	1	1
enrprt	/workdir/wepp-forest_260430_baseline/src/enrprt.for:1	1	0
eqroot	/workdir/wepp-forest_260430_baseline/src/eqroot.for:1	2	0
erod	/workdir/wepp-forest_260430_baseline/src/erod.for:1	1	1
evap	/workdir/wepp-forest_260430_baseline/src/evap.for:1	2	1
evappm	/workdir/wepp-forest_260430_baseline/src/evappm.for:1	2	1
frcfac	/workdir/wepp-forest_260430_baseline/src/frcfac.for:1	5	0
frichn	/workdir/wepp-forest_260430_baseline/src/frichn.for:1	1	0
frostn	/workdir/wepp-forest_260430_baseline/src/frostn.for:1	1	7
frsoil	/workdir/wepp-forest_260430_baseline/src/frsoil.for:1	1	2
frwatc	/workdir/wepp-forest_260430_baseline/src/frwatc.for:1	1	0
frzng	/workdir/wepp-forest_260430_baseline/src/frzng.for:1	1	5
frznw	/workdir/wepp-forest_260430_baseline/src/frznw.for:1	1	1
fslpar	/workdir/wepp-forest_260430_baseline/src/fslpar.for:1	1	0
furadv	/workdir/wepp-forest_260430_baseline/src/furadv.for:1	1	4
furgps	/workdir/wepp-forest_260430_baseline/src/furgps.for:1	1	0
furlea	/workdir/wepp-forest_260430_baseline/src/furlea.for:1	1	0
furrec	/workdir/wepp-forest_260430_baseline/src/furrec.for:1	1	2
furrow	/workdir/wepp-forest_260430_baseline/src/furrow.for:1	1	6
furrun	/workdir/wepp-forest_260430_baseline/src/furrun.for:1	1	0
gcurve	/workdir/wepp-forest_260430_baseline/src/gcurve.for:1	1	0
gdmax	/workdir/wepp-forest_260430_baseline/src/gdmax.for:1	1	0
getdat	/workdir/wepp-forest_260430_baseline/src/getdat.for:3	1	1
grna	/workdir/wepp-forest_260430_baseline/src/grna.for:1	3	3
grow	/workdir/wepp-forest_260430_baseline/src/grow.for:1	3	1
growop	/workdir/wepp-forest_260430_baseline/src/growop.for:1	1	0
hdreng	/workdir/wepp-forest_260430_baseline/src/hdreng.for:1	1	0
hdrive	/workdir/wepp-forest_260430_baseline/src/hdrive.for:1	2	1
hr_tmp	/workdir/wepp-forest_260430_baseline/src/hr_tmp.for:1	1	1
hrtmp	/workdir/wepp-forest_260430_baseline/src/hrtmp.for:1	1	0
hydchn	/workdir/wepp-forest_260430_baseline/src/hydchn.for:1	2	1
hydout	/workdir/wepp-forest_260430_baseline/src/hydout.for:1	1	0
idat	/workdir/wepp-forest_260430_baseline/src/idat.for:1	2	1
impday	/workdir/wepp-forest_260430_baseline/src/impday.for:1	1	0
impeo	/workdir/wepp-forest_260430_baseline/src/impeo.for:1	1	0
impeos	/workdir/wepp-forest_260430_baseline/src/impeos.for:1	1	0
impflo	/workdir/wepp-forest_260430_baseline/src/impflo.for:1	1	1
imphnw	/workdir/wepp-forest_260430_baseline/src/imphnw.for:1	1	0
impint	/workdir/wepp-forest_260430_baseline/src/impint.for:1	1	5
impmai	/workdir/wepp-forest_260430_baseline/src/impmai.for:1	1	2
impmon	/workdir/wepp-forest_260430_baseline/src/impmon.for:1	1	0
imppol	UNRESOLVED	1	0
imppow	UNRESOLVED	1	0
imppro	/workdir/wepp-forest_260430_baseline/src/imppro.for:1	1	0
impreg	/workdir/wepp-forest_260430_baseline/src/impreg.for:1	1	1
impris	UNRESOLVED	2	0
impyr	/workdir/wepp-forest_260430_baseline/src/impyr.for:1	1	0
infile	/workdir/wepp-forest_260430_baseline/src/infile.for:1	2	7
infpar	/workdir/wepp-forest_260430_baseline/src/infpar.for:1	1	0
inidat	/workdir/wepp-forest_260430_baseline/src/inidat.for:1	1	0
init1	/workdir/wepp-forest_260430_baseline/src/init1.for:1	2	1
initd	/workdir/wepp-forest_260430_baseline/src/initd.for:1	2	0
initgr	/workdir/wepp-forest_260430_baseline/src/initgr.for:1	2	0
input	/workdir/wepp-forest_260430_baseline/src/input.for:1	2	3
irinpt	/workdir/wepp-forest_260430_baseline/src/irinpt.for:1	1	0
irprnt	/workdir/wepp-forest_260430_baseline/src/irprnt.for:1	2	0
irrig	/workdir/wepp-forest_260430_baseline/src/irrig.for:1	2	2
irs	/workdir/wepp-forest_260430_baseline/src/irs.for:1	1	10
kostia	/workdir/wepp-forest_260430_baseline/src/kostia.for:1	1	1
locate	/workdir/wepp-forest_260430_baseline/src/locate.for:1	4	6
mann	/workdir/wepp-forest_260430_baseline/src/mann.for:1	1	0
melt	/workdir/wepp-forest_260430_baseline/src/melt.for:1	1	1
mltbtm	/workdir/wepp-forest_260430_baseline/src/mltbtm.for:1	1	1
mlttp	/workdir/wepp-forest_260430_baseline/src/mlttp.for:1	1	2
monchn	/workdir/wepp-forest_260430_baseline/src/monchn.for:1	1	0
monout	/workdir/wepp-forest_260430_baseline/src/monout.for:1	1	0
mxint	/workdir/wepp-forest_260430_baseline/src/mxint.for:1	3	0
mxreal	/workdir/wepp-forest_260430_baseline/src/mxreal.for:1	3	0
newrap	/workdir/wepp-forest_260430_baseline/src/newrap.for:1	2	0
newtil	/workdir/wepp-forest_260430_baseline/src/newtil.for:2	2	0
newton	/workdir/wepp-forest_260430_baseline/src/newton.for:1	2	0
nowup	/workdir/wepp-forest_260430_baseline/src/nowup.for:1	2	0
open	/workdir/wepp-forest_260430_baseline/src/open.for:1	3	0
outeng	/workdir/wepp-forest_260430_baseline/src/outeng.for:1	1	2
outfil	/workdir/wepp-forest_260430_baseline/src/outfil.for:1	2	1
param	/workdir/wepp-forest_260430_baseline/src/param.for:1	1	2
patrib	/workdir/wepp-forest_260430_baseline/src/patrib.for:1	1	0
peak	/workdir/wepp-forest_260430_baseline/src/peak.for:1	2	0
perc	/workdir/wepp-forest_260430_baseline/src/perc.for:1	1	1
pmetcoef	/workdir/wepp-forest_260430_baseline/src/pmetcoef.for:1	1	0
print	/workdir/wepp-forest_260430_baseline/src/print.for:1	1	0
profil	/workdir/wepp-forest_260430_baseline/src/profil.for:1	1	0
prtcmp	/workdir/wepp-forest_260430_baseline/src/prtcmp.for:2	2	2
psis	/workdir/wepp-forest_260430_baseline/src/psis.for:1	1	0
ptgra	/workdir/wepp-forest_260430_baseline/src/ptgra.for:1	2	2
ptgrp	/workdir/wepp-forest_260430_baseline/src/ptgrp.for:1	2	4
purk	/workdir/wepp-forest_260430_baseline/src/purk.for:1	2	1
qinf	/workdir/wepp-forest_260430_baseline/src/qinf.for:1	3	0
radcur	/workdir/wepp-forest_260430_baseline/src/radcur.for:1	1	0
rand	/workdir/wepp-forest_260430_baseline/src/rand.for:1	1	0
range	/workdir/wepp-forest_260430_baseline/src/range.for:1	2	7
rdat	/workdir/wepp-forest_260430_baseline/src/rdat.for:1	3	0
readin	/workdir/wepp-forest_260430_baseline/src/readin.for:2	3	1
reid	/workdir/wepp-forest_260430_baseline/src/reid.for:1	1	0
res_dp	/workdir/wepp-forest_260430_baseline/src/res_dp.for:1	1	0
resup	/workdir/wepp-forest_260430_baseline/src/resup.for:1	4	0
rgraze	/workdir/wepp-forest_260430_baseline/src/rgraze.for:1	1	0
rgrcur	/workdir/wepp-forest_260430_baseline/src/rgrcur.for:1	1	0
rherb	/workdir/wepp-forest_260430_baseline/src/rherb.for:1	1	0
rngint	/workdir/wepp-forest_260430_baseline/src/rngint.for:1	2	0
rochek	/workdir/wepp-forest_260430_baseline/src/rochek.for:1	1	0
root	/workdir/wepp-forest_260430_baseline/src/root.for:1	1	0
route	/workdir/wepp-forest_260430_baseline/src/route.for:1	1	4
runge	/workdir/wepp-forest_260430_baseline/src/runge.for:1	1	0
runout	/workdir/wepp-forest_260430_baseline/src/runout.for:1	1	0
saxfun	/workdir/wepp-forest_260430_baseline/src/saxfun.for:1	3	1
saxpar	/workdir/wepp-forest_260430_baseline/src/saxpar.for:1	1	0
scenhd	/workdir/wepp-forest_260430_baseline/src/scenhd.for:2	1	2
scon	/workdir/wepp-forest_260430_baseline/src/scon.for:1	2	0
scurv	/workdir/wepp-forest_260430_baseline/src/scurv.for:1	2	1
sedist	/workdir/wepp-forest_260430_baseline/src/sedist.for:1	1	0
sedmax	/workdir/wepp-forest_260430_baseline/src/sedmax.for:1	1	0
sedout	/workdir/wepp-forest_260430_baseline/src/sedout.for:1	3	5
sedseg	/workdir/wepp-forest_260430_baseline/src/sedseg.for:1	2	2
sedsta	/workdir/wepp-forest_260430_baseline/src/sedsta.for:1	1	1
shears	/workdir/wepp-forest_260430_baseline/src/shears.for:1	1	0
sheart	/workdir/wepp-forest_260430_baseline/src/sheart.for:1	1	0
sloss	/workdir/wepp-forest_260430_baseline/src/sloss.for:1	1	1
snowd	/workdir/wepp-forest_260430_baseline/src/snowd.for:1	1	1
soil	/workdir/wepp-forest_260430_baseline/src/soil.for:1	2	2
stmget	/workdir/wepp-forest_260430_baseline/src/stmget.for:1	2	3
stmtim	/workdir/wepp-forest_260430_baseline/src/stmtim.for:1	1	0
strip	/workdir/wepp-forest_260430_baseline/src/strip.for:1	2	0
strout	/workdir/wepp-forest_260430_baseline/src/strout.for:1	1	1
sumfrc	/workdir/wepp-forest_260430_baseline/src/sumfrc.for:1	1	1
sumrnf	/workdir/wepp-forest_260430_baseline/src/sumrnf.for:1	1	0
sumrun	/workdir/wepp-forest_260430_baseline/src/sumrun.for:1	1	0
sunmap	/workdir/wepp-forest_260430_baseline/src/sunmap.for:1	3	0
swu	/workdir/wepp-forest_260430_baseline/src/swu.for:1	2	0
table	/workdir/wepp-forest_260430_baseline/src/table.for:1	5	0
tfail	/workdir/wepp-forest_260430_baseline/src/tfail.for:1	1	0
tilage	/workdir/wepp-forest_260430_baseline/src/tilage.for:1	2	2
tmpadj	/workdir/wepp-forest_260430_baseline/src/tmpadj.for:1	1	1
tmpcft	/workdir/wepp-forest_260430_baseline/src/tmpcft.for:1	1	1
trncap	/workdir/wepp-forest_260430_baseline/src/trncap.for:1	4	0
trnlos	/workdir/wepp-forest_260430_baseline/src/trnlos.for:1	1	0
undflo	/workdir/wepp-forest_260430_baseline/src/undflo.for:1	6	0
verchk	/workdir/wepp-forest_260430_baseline/src/verchk.for:1	1	0
watbal	/workdir/wepp-forest_260430_baseline/src/watbal.for:1	3	10
watbal_hourly	/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1	1	9
watbalprint	/workdir/wepp-forest_260430_baseline/src/watbalprint.for:1	1	0
watdst	/workdir/wepp-forest_260430_baseline/src/watdst.for:1	2	2
wepp_observe	/workdir/wepp-forest_260430_baseline/src/wepp_observe.for:1	22	0
winit	/workdir/wepp-forest_260430_baseline/src/winit.for:1	2	2
winter	/workdir/wepp-forest_260430_baseline/src/winter.for:1	2	8
winthd	/workdir/wepp-forest_260430_baseline/src/winthd.for:1	2	0
writeyearlylossbypoint	/workdir/wepp-forest_260430_baseline/src/writeyearlylossbypoint.for:1	1	0
wshchr	/workdir/wepp-forest_260430_baseline/src/wshchr.for:1	2	3
wshcqi	/workdir/wepp-forest_260430_baseline/src/wshcqi.for:1	1	0
wshdrv	/workdir/wepp-forest_260430_baseline/src/wshdrv.for:1	1	46
wshimp	/workdir/wepp-forest_260430_baseline/src/wshimp.for:1	1	2
wshini	/workdir/wepp-forest_260430_baseline/src/wshini.for:1	1	3
wshinp	/workdir/wepp-forest_260430_baseline/src/wshinp.for:1	1	1
wshiqi	/workdir/wepp-forest_260430_baseline/src/wshiqi.for:1	1	2
wshirs	/workdir/wepp-forest_260430_baseline/src/wshirs.for:1	1	6
wshout	/workdir/wepp-forest_260430_baseline/src/wshout.for:1	1	2
wshpas	/workdir/wepp-forest_260430_baseline/src/wshpas.for:1	2	1
wshpek	/workdir/wepp-forest_260430_baseline/src/wshpek.for:1	1	5
wshred	/workdir/wepp-forest_260430_baseline/src/wshred.for:1	1	0
wshrun	/workdir/wepp-forest_260430_baseline/src/wshrun.for:1	1	1
wshscs	/workdir/wepp-forest_260430_baseline/src/wshscs.for:1	2	0
wshtc	/workdir/wepp-forest_260430_baseline/src/wshtc.for:1	1	0
xcrit	/workdir/wepp-forest_260430_baseline/src/xcrit.for:1	1	1
xinflo	/workdir/wepp-forest_260430_baseline/src/xinflo.for:1	1	0
yalin	/workdir/wepp-forest_260430_baseline/src/yalin.for:1	1	0
yldopt	/workdir/wepp-forest_260430_baseline/src/yldopt.for:1	1	2
```
