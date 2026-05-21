# Watershed Impoundment Input File Specification (`.imp`)

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001`
- `surface_id`: `infile-watershed-impoundment-imp`
- `status`: `draft`
- `owner`: `openWEPP core`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-20T00:00:00Z`
- `evidence_mode`: `Static`

## 2. Surface Scope and Applicability
- The `.imp` file defines watershed impoundment geometry and outlet structures for WEPP watershed runs with impoundments enabled. [DIRECT]
- This surface applies to watershed executions (not standalone hillslope mode). [DIRECT]
- The file is consumed across three legacy read stages:
1. `infile.for` reads file version (`datver`) and performs compatibility checks.
2. `wshini.for` reads declared impoundment count (`jpond`) and checks against structure-derived count (`npond`).
3. `impint.for` reads per-impoundment content and derives outflow/geometry functions. [DIRECT]

### Applicability notes
- The users guide states watershed model versions 2 and 3 cover channel/impoundment workflows. [DIRECT]
- If no impoundments are modeled, this file is not required. [DIRECT]

## 3. Version / `datver` Applicability Matrix

| Mode | Input expectation | Legacy behavior | openWEPP draft contract |
|---|---|---|---|
| Explicit version header | First non-comment token is `datver` and `datver > 10.0` | `verchk` enforces `datver >= impchk` (`94.301`) | Accept when `datver >= 94.301`; otherwise typed version error |
| Legacy/older unguarded form | First non-comment token `<= 10.0` | `verchk` path is skipped | Keep supported only under explicit legacy-compat mode; mark warning |

- `impchk` is initialized to `94.301` in legacy startup data. [DIRECT]

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Parser staging grammar

```text
imp_file ::= preamble impoundment_block{jpond}

preamble ::= datver_line jpond_line

datver_line ::= REAL
jpond_line ::= INT

impoundment_block ::= description3
                     drop_spillway_section
                     culvert_section_1
                     culvert_section_2
                     rockfill_section
                     emergency_spillway_section
                     filter_fence_section
                     perforated_riser_section
                     misc_section
                     geometry_section

# Comment/blank lines may appear where legacy code calls eatcom().
```

### 4.2 Preamble (file-level)
- Line 1: `datver` (real version number). [DIRECT]
- Line 2: `jpond` (number of impoundments declared in the `.imp` file). [DIRECT]

### 4.3 Per-impoundment block (repeated `jpond` times)
- Lines 3-5: `impdes(1..3)` comment/description lines. [DIRECT]

#### Drop spillway section
- Line 6: `ids` (drop spillway index). [DIRECT]
- `ids = 0`: no drop spillway; skip drop structure lines. [DIRECT]
- `ids = 1`:
1. comment line `strdes`
2. `diars hrs coefw coefo`
3. `diabl hrh lbl sbl hblot`
4. `ke kb kc`
- `ids = 2`:
1. comment line `strdes`
2. `lenrs widrs hrs coefw coefo`
3. `diabl hrh lbl sbl hblot`
4. `ke kb kc`
- `ids = 3`:
1. comment line `strdes`
2. `lenrs widrs hrs coefw coefo`
3. `hitbl wdbl hrh lbl sbl hblot`
4. `ke kb kc`

#### Culvert sections
- Culvert #1 header line: `icv ncv`. [DIRECT]
- If `icv >= 1`:
1. comment line `strdes`
2. `arcv hitcv hcv lcv scv hcvot`
3. `ke kb kc kus mus cs ys`
- Culvert #2 header line: `icv ncv` (read unconditionally after culvert #1 branch). [DIRECT]
- If Culvert #2 `icv >= 1`, same 3 lines as Culvert #1.

#### Rock-fill check dam section
- Line 15 conceptually: `irf`. [DIRECT]
- If `irf != 0`:
1. comment line `strdes`
2. `lnrf hrf hotrf wdrf diarf`

#### Emergency spillway section
- Line 18 conceptually: `ies`. [DIRECT]
- If `ies = 1` (open channel outlet):
1. comment line `strdes`
2. `bwes sses nes hes hmxes`
3. `ses1 les1 ses2 les2 ses3`
- If `ies = 2` (user stage-discharge):
1. comment line `strdes`
2. `npts`
3. `hes`
4. `hest(1..npts)` list
5. `qes(1..npts)` list

#### Filter fence / straw bale / trash barrier section
- Line 24 conceptually: `iff`. [DIRECT]
- If `iff != 0`:
1. comment line `strdes`
2. `vsl wdff hff hotff`

#### Perforated riser section
- Line 31 conceptually: `ipr`. [DIRECT]
- If `ipr != 0`:
1. comment line `strdes`
2. `hr hb hs hd diar as diab`
3. `hrh lbl sbl diabl`
4. `cb coefw coefo cs`
5. `ke kb kc`

#### Miscellaneous and stage-area-length section
- Misc line: `hottmp hfltmp htmp dltimp qnftmp` (`hot`, `hfull`, `h`, `deltat`, `qinf`). [DIRECT]
- Size line: `isztmp ndvtmp` (`isize`, `ndiv`). [DIRECT]
- Geometry count line: `nalpts`. [DIRECT]
- Minimum geometry line: `hmntmp a0tmp l0tmp` (`hmin`, `a0`, `l0`). [DIRECT]
- Stage list line(s): `hal(1..nalpts)` (after optional comment/blank lines skipped by `eatcom`). [DIRECT]
- Area list line(s): `area(1..nalpts)` (after `eatcom`). [DIRECT]
- Length list line(s): `length(1..nalpts)` (after `eatcom`). [DIRECT]

### 4.4 Units at file boundary
- Input file units are SI per users guide (`m`, `m2`, `m3/s`, `m/d`). [DIRECT]
- Legacy runtime converts many geometric/hydraulic inputs to feet/imperial internal variables (`* 3.281`, `* 3.281**2`, `* 3.281**3`). [DIRECT]

## 5. Field Dictionary and Alias Mapping

### 5.1 Canonical field dictionary (legacy symbols)

| Symbol | Meaning | Units (file) | Type | Cardinality | Requiredness |
|---|---|---|---|---|---|
| `datver` | impoundment file version | unitless | real | 1/file | required |
| `jpond` | declared number of impoundments in `.imp` | count | int | 1/file | required |
| `impdes(i)` | impoundment description lines | text | char | 3/impoundment | required |
| `ids` | drop spillway index | enum int | int | 1/impoundment | required |
| `diars,lenrs,widrs,hrs,coefw,coefo,diabl,hitbl,wdbl,hrh,lbl,sbl,hblot,ke,kb,kc` | drop spillway parameters by `ids` branch | mixed SI | real | branch-specific | conditional |
| `icv,ncv` | culvert enabled flag and multiplicity | enum/count | int | 2 pairs/impoundment | required |
| `arcv,hitcv,hcv,lcv,scv,hcvot,ke,kb,kc,kus,mus,cs,ys` | culvert parameters per active culvert | mixed SI | real | per active culvert | conditional |
| `irf` | rock-fill dam index | enum int | int | 1/impoundment | required |
| `lnrf,hrf,hotrf,wdrf,diarf` | rock-fill parameters | mixed SI | real | when `irf!=0` | conditional |
| `ies` | emergency spillway index | enum int | int | 1/impoundment | required |
| `bwes,sses,nes,hes,hmxes,ses1,les1,ses2,les2,ses3` | open-channel emergency spillway params | mixed SI | real | when `ies=1` | conditional |
| `npts,hest(i),qes(i)` | user stage-discharge relation | count + SI | int + real[] | when `ies=2` | conditional |
| `iff` | filter fence/straw barrier index | enum int | int | 1/impoundment | required |
| `vsl,wdff,hff,hotff` | filter fence/straw barrier params | mixed SI | real | when `iff!=0` | conditional |
| `ipr` | perforated riser index | enum int | int | 1/impoundment | required |
| `hr,hb,hs,hd,diar,as,diab,hrh,lbl,sbl,diabl,cb,coefw,coefo,cs,ke,kb,kc` | perforated riser parameters | mixed SI | real | when `ipr!=0` | conditional |
| `hot,hfull,h,deltat,qinf` | overtop/full/start stage, timestep, infiltration | m, hr, m/d | real | 1 set/impoundment | required |
| `isize,ndiv` | structure size flag and sediment subclass divisions | enum/count | int | 1 set/impoundment | required |
| `nalpts` | stage-area-length point count | count | int | 1/impoundment | required |
| `hmin,a0,l0` | baseline geometry point | m,m2,m | real | 1 set/impoundment | required |
| `hal(i),area(i),length(i)` | stage-area-length arrays | m,m2,m | real[] | `nalpts` each | required |

### 5.2 openWEPP alias mapping (draft)

| Canonical symbol | openWEPP boundary alias (proposed) |
|---|---|
| `datver` | `file_version` |
| `jpond` | `impoundment_count_declared` |
| `npond` (from `.str`) | `impoundment_count_structural` |
| `ids` | `drop_spillway.kind` |
| `icv` / `ncv` | `culvert.enabled` / `culvert.count` |
| `irf` | `rockfill.enabled` |
| `ies` | `emergency_spillway.kind` |
| `iff` | `filter_barrier.kind` |
| `ipr` | `perforated_riser.enabled` |
| `hot` | `overtop_stage_m` |
| `hfull` | `full_sediment_stage_m` |
| `h` | `initial_stage_m` |
| `deltat` | `initial_timestep_hr` |
| `qinf` | `infiltration_rate_m_per_d` |
| `isize` | `structure_size_class` |
| `ndiv` | `particle_subclass_divisions` |
| `nalpts` | `stage_area_length_point_count` |
| `hal/area/length` | `stage_area_length_curve.{stage_m,area_m2,length_m}` |

## 6. Conditional Branches and Optional Sections
- Drop spillway branch is controlled by `ids` (`0/1/2/3`). [DIRECT]
- Two culvert branches are always header-read (`icv ncv` twice), with parameter lines conditional on each `icv >= 1`. [DIRECT]
- Rock-fill section is conditional on `irf != 0`. [DIRECT]
- Emergency spillway section is conditional on `ies` with mutually exclusive shape: open-channel (`ies=1`) vs user stage-discharge (`ies=2`). [DIRECT]
- Filter barrier section is conditional on `iff != 0`. [DIRECT]
- Perforated riser section is conditional on `ipr != 0`. [DIRECT]
- Geometry arrays (`hal/area/length`) are required regardless of branch choices. [DIRECT]

## 7. Cross-File Consistency Constraints and Coupling Dependencies
- `.str` defines structural impoundment count (`npond`); `.imp` declares `jpond`; legacy stop condition triggers when `npond > jpond`. [DIRECT]
- Legacy enforces upper bound by internal `mximp` (`25`) on both `npond` and `jpond`. [DIRECT]
- Users guide states a 10-impoundment limit for simulations; this conflicts with the internal `mximp=25` default in current legacy source. [DIRECT]
- Watershed structure rules from users guide constrain whether impoundments can be fed by channels vs hillslopes and prohibit mixed channel+hillslope feeding into a single impoundment. [DIRECT]
- `.imp` is semantically coupled to `.str` element ordering and IDs; the per-impoundment block order is assumed to align with impoundment indexing used by watershed routines. [INFERENCE]

## 8. Defaulting, Missing-File Behavior, and Typed Error Expectations

### 8.1 Legacy behavior summary
- Missing/invalid compatibility path can terminate with `stop` after console message. [DIRECT]
- Count mismatch (`npond > jpond`) terminates with `stop`. [DIRECT]
- Exceeding `mximp` terminates with `stop`. [DIRECT]
- Branch/array parse failures propagate as Fortran read/runtime failures (non-recoverable). [INFERENCE]

### 8.2 openWEPP typed error expectations (contract draft)
- `InputFileMissing { surface: infile-watershed-impoundment-imp }`
- `InputVersionIncompatible { datver, minimum_supported: 94.301 }`
- `ImpoundmentCountMismatch { structural_npond, declared_jpond }`
- `ImpoundmentCountExceeded { value, max_supported }`
- `ImpoundmentBranchInvalid { field, value, allowed }`
- `ImpoundmentUnexpectedEof { section, impoundment_index }`
- `ImpoundmentArrayCardinalityError { field, expected: nalpts, got }`
- `ImpoundmentPhysicalInvariantError { field, relation }`

### 8.3 Minimum invariants for parser guardrails
- `jpond >= 0`; `nalpts >= 1`; `ndiv >= 1`. [INFERENCE]
- `hot >= h`, `hfull >= hmin`, `hmxes >= hes` where those fields exist. [INFERENCE]
- All geometry arrays must have equal cardinality `nalpts`. [DIRECT]

## 9. Example Snippets

### 9.1 Minimal valid example (one impoundment, no structures)
```text
95.700
1
# impoundment 1
# no structures
# baseline geometry only
0
0 0
0 0
0
0
0
0
0.80 0.75 0.75 1.00 0.00
1 5
3
0.70 100.0 20.0
0.80 0.90 1.10
110.0 125.0 150.0
22.0 25.0 30.0
```

### 9.2 Representative branched example (drop spillway + culvert + emergency channel)
```text
95.700
1
# impoundment 1
# drop spillway + culvert + emergency channel
# representative only
1
# drop spillway
0.80 1.20 3.10 0.60
0.60 0.40 12.0 0.02 0.10
0.78 0.20 0.03
1 2
# culvert 1
0.45 0.60 0.90 10.0 0.01 0.05
0.50 0.20 0.03 0.90 1.00 0.40 -0.10
0 0
0
1
# emergency spillway
2.0 1.5 0.04 1.1 2.5
0.02 8.0 0.01 8.0 0.005
0
0
1.50 1.40 1.00 1.00 0.00
2 5
4
0.80 90.0 18.0
1.00 1.20 1.40 1.60
100.0 120.0 150.0 190.0
20.0 24.0 29.0 35.0
```

### 9.3 Invalid examples
- `npond` in `.str` greater than `jpond` in `.imp` -> reject (`ImpoundmentCountMismatch`). [DIRECT]
- `ies=2` but missing either `hest` or `qes` vectors -> reject (`ImpoundmentUnexpectedEof`). [DIRECT]
- `nalpts=4` but only 3 `area` values -> reject (`ImpoundmentArrayCardinalityError`). [INFERENCE]

## 10. Gap / Conflict Register

| ID | Severity | Claim | Evidence tag | Status | HOLD condition |
|---|---|---|---|---|---|
| G-IMP-001 | medium | users guide says up to 10 impoundments; legacy source enforces `mximp=25` | [DIRECT] | open | HOLD if openWEPP max-count policy is not explicitly chosen |
| G-IMP-002 | medium | usersum Table 28 line-number narrative around filter-fence/perforated-riser sublines is partially ambiguous in text extraction | [DIRECT] | open | HOLD if unresolved ambiguity affects parser grammar decisions |
| G-IMP-003 | low | usersum culvert brief table omits some coefficients (`kus/mus/cs/ys`) read by legacy parser | [DIRECT] | open | HOLD only if coefficients cannot be physically/algorithmically traced |
| G-IMP-004 | low | wepppy canonical spec set currently lacks `.imp` spec and wepppyo3 scoped source search did not find explicit `.imp` parser logic | [DIRECT] | open | no hold; provenance-only note |

## 11. Parser-Contract Handoff Map

| Spec section | Target parser contract | Handoff intent |
|---|---|---|
| Sections 3, 4 | `SC-INFILE-WATERSHED-IMPOUNDMENT-001` | Define accepted grammar, branch semantics, and version gating |
| Sections 5, 6 | `SC-INFILE-WATERSHED-IMPOUNDMENT-001` | Define typed data model + legacy symbol continuity + alias mapping |
| Sections 7, 8 | `SC-INFILE-WATERSHED-IMPOUNDMENT-001` | Define cross-file constraints and typed error surface |
| Section 9 | `SC-INFILE-WATERSHED-IMPOUNDMENT-001` | Seed parser conformance fixtures |
| Section 10 | `SC-INFILE-WATERSHED-IMPOUNDMENT-001` | Track unresolved policy decisions and hold criteria |

## Evidence Anchors
- `EA-IMP-001` [DIRECT]: `references/vendorable/usersum2024.pdf`, Table 28 (Impoundment input file description), Table 23 (structure rules), watershed input sections.
- `EA-IMP-002` [DIRECT]: `references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7622-7843` (Table 28 extraction).
- `EA-IMP-003` [DIRECT]: `references/copyrighted/source_pdfs/WEPP_usersum2024.txt:6875-6953,7093-7137` (structure + restrictions).
- `EA-IMP-004` [DIRECT]: `references/copyrighted/source_pdfs/WEPP_usersum2024.txt:512-516,6790-6799` (impoundment scope + watershed versions).
- `EA-IMP-005` [DIRECT]: `/workdir/wepp-forest/src/infile.for:438-458` (open/read `datver` + `verchk`).
- `EA-IMP-006` [DIRECT]: `/workdir/wepp-forest/src/verchk.for:19-31` (version compare stop condition).
- `EA-IMP-007` [DIRECT]: `/workdir/wepp-forest/src/inidat.for:1160-1163` and `/workdir/wepp-forest/src/cdat.inc:23-25` (`impchk` provenance).
- `EA-IMP-008` [DIRECT]: `/workdir/wepp-forest/src/wshini.for:337-361` (`npond/jpond` + `mximp` checks).
- `EA-IMP-009` [DIRECT]: `/workdir/wepp-forest/src/pmximp.inc:6-7` (`mximp=25`).
- `EA-IMP-010` [DIRECT]: `/workdir/wepp-forest/src/impint.for:553-557,570,597-600,763,797-800,873,905-908,978,993-995,1080,1090-1093,1282-1293,1346,1354-1356,1422,1441-1445,1639-1641,1768-1781` (line-wise reads and branch structure).
- `EA-IMP-011` [DIRECT]: `/workdir/wepp-forest/src/impint.for:607-613,806-810,1099-1103,1453-1463,1654-1659,1794-1801` (SI to imperial internal conversions).
- `EA-IMP-012` [DIRECT]: `/workdir/wepp-forest/src/wshinp.for:246-275` (structure element ingestion + contribution existence check).
- `EA-IMP-013` [DIRECT]: `/workdir/wepp-forest/src/eatcom.for:29-38,53` (comment/blank line skipping behavior).
- `EA-IMP-014` [DIRECT]: `/workdir/wepppy/wepppy/weppcloud/routes/usersum/input-file-specifications/` contains only `cligenparms.md`, `climate-file.spec.md`, `plant-file.spec.md`, `soil-file.spec.md`.
- `EA-IMP-015` [DIRECT]: `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:30-47` (option parsing surface; no explicit `.imp` grammar implementation).
- `EA-IMP-016` [DIRECT]: `rg` static search over `/workdir/wepppyo3/cli_revision/src`, `/workdir/wepppyo3/watershed_abstraction/src`, `/workdir/wepppyo3/wepp_interchange/src` for `impound|impoundment|.imp|npound|npond` returned no matches.

## Provenance Labels
- `usersum2024`: `EA-IMP-001` to `EA-IMP-004`
- `legacy-code`: `EA-IMP-005` to `EA-IMP-013`
- `wepppy`: `EA-IMP-014`, `EA-IMP-015`
- `wepppyo3`: `EA-IMP-016`
