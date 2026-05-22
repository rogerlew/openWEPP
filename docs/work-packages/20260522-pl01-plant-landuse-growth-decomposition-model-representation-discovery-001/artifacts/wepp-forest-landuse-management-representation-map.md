# WEPP-Forest Landuse/Management Representation Map (PL01)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- `.man` representation is a two-stage model: section/scenario registries first, then yearly/OFE schedule projection into runtime state surfaces.
- `landuse` gates both parse branches and runtime dispatch (`lanuse=1` cropland, `lanuse=2` rangeland).

Ran:
- Audited baseline intake and scheduling flow in `infile.for`, `tilage.for`, `contin.for`, `watbal.for` and include surfaces `cinpman1.inc`, `cinpman2.inc`, `cinpop.inc`, `cinpsur.inc`, `ccover.inc`, `ccrpprm.inc`, `cperen.inc`, `cupdate.inc`.

## Canonical Representation Layers

| layer | canonical symbols | storage surface | semantics |
|---|---|---|---|
| Scenario registries | `ityp1`, `imngm1`, `tilse1`, `conse1`, `drseq1`, `jdpl1`, `jdhar1`, `resmg1`, `mgtop1`, `ncu1`, `ncycl1`, `gda1`, `gen1`, `jfdat1`, `ihdat1`, `grazi1` | `/inpman1/`, `/inpman2/`, `/inpop/`, `/inpsur/` | Section-local catalog read from `.man` scenario blocks. |
| OFE initial projection | `lanuse`, `cancov`, `inrcov`, `rilcov`, `iresd`, `imngmt(nowres+3,iplane)`, `tillay`, `width`, `rspace`, residue/root initial pools | runtime common blocks (`/cover/`, `/crpprm/`, `/crpvr1/`, etc.) | Converts `inindx` references into active OFE state and applies normalization guards. |
| Yearly schedule expansion | `nycrop`, per-slot `manndx`, runtime `itype/imngmt/tilseq/conseq/drseq`, date/event options | runtime common blocks (`/crpprm/`, `/peren/`, `/update/`) | Realizes yearly management graph into day-dispatchable arrays. |

## Coupling Semantics

1. Intake phase (`infile`) builds scenario registries and yearly references.
- Section loops parse plant/operation/initial/surface/contour/drain/yearly records.
- Cropland/rangeland branch behavior is explicit; forest/road branches stop in multiple sections.

2. OFE conversion phase maps initial-condition references to runtime state.
- For cropland initial scenarios, OFE surfaces are populated and normalized (tillage-depth ordering/defaults, rill width/spacing guards, unit conversions).
- For rangeland initial scenarios, distinct residue/cover/grazing-related surfaces are populated.

3. Runtime schedule projection (`tilage`) binds yearly scenario pointers to active arrays.
- Per OFE-year slot, `manndx` is dereferenced into `itype`, `imngmt`, `tilseq`, `conseq`, `drseq`, plus option-specific date/fraction controls.
- `imngmt` branch sets annual/fallow (`1/3`) vs perennial (`2`) event surfaces (`resmgt`, `mgtopt`, cut/graze cycles).

4. Daily dispatch (`contin` + `watbal`) uses projected schedule state.
- Start-of-run: `tilage(nowcrp)` initializes active crop slot pointers.
- Daily loop executes `decomp` (cropland) before `soil` for same-day management impact, then `watbal` dispatches plant growth path by `lanuse/imngmt`.
- Crop transitions are date-triggered through `switch`, `nowup`, and `newtil` to activate the next slot.

## Invariants Observed

- `imngmt` domain is `1..3` in yearly cropland parse/projection path.
- Primary/secondary tillage depths are reordered and bounded to prevent invalid layer geometry.
- Rill geometry enforces non-negative widths and positive spacing defaults.
- `landuse` branch controls ownership of downstream state families and kernels.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/infile.for:499`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:648`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1095`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1121`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1329`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1342`
- `/workdir/wepp-forest_260430_baseline/src/infile.for:1463`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:228`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:268`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:376`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:442`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:1288`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
- `/workdir/wepp-forest_260430_baseline/src/cinpman1.inc:6`
- `/workdir/wepp-forest_260430_baseline/src/cinpman2.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/ccrpprm.inc:7`
- `/workdir/wepp-forest_260430_baseline/src/cperen.inc:7`
