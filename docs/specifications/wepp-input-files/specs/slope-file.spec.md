# WEPP Slope Input File Specification

## Header Metadata
- `spec_id`: `SPEC-INFILE-SLOPE-SLP-001`
- `surface_id`: `infile-slope-slp`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.2.0`
- `last_updated_utc`: `2026-05-21T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- `[DIRECT][E-US-01]` `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf` (Table 2, pages 11-12: slope file format and field definitions).
- `[DIRECT][E-US-02]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:721-810` (Slope Input File narrative + Table 2 line-by-line description).
- `[DIRECT][E-US-03]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:732-747` (OFE semantics, per-OFE coupling statement, adjoining OFE border-slope rule).
- `[DIRECT][E-US-04]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:800-809` (distance-input modes; minimum two points; endpoint requirements; legacy no-version-line note).
- `[DIRECT][E-WF-01]` `/workdir/wepp-forest/src/infile.for:1652-1678` (slope file open/read; datver detection branch; fallback where line 1 is treated as `nwsofe`; `readin` OFE-count read).
- `[DIRECT][E-WF-02]` `/workdir/wepp-forest/src/infile.for:1333-1337` and `/workdir/wepp-forest/src/infile.for:1678-1692` (management `nwsofe` -> `jstruc` and required equality with slope-file OFE count).
- `[DIRECT][E-WF-03]` `/workdir/wepp-forest/src/input.for:380-398` (per-OFE reads for `azm`, `fwidth`, `nslpts`, `slplen`, `xinput`, `slpinp`).
- `[DIRECT][E-WF-04]` `/workdir/wepp-forest/src/profil.for:37-55` (profile normalization and derived `avgslp`; use of terminal `xinput` as OFE length proxy for interpolation logic).
- `[DIRECT][E-WF-05]` `/workdir/wepp-forest/src/profil.for:51` (legacy guard clamps non-positive `avgslp` to `1.0e-6`).
- `[DIRECT][E-WF-06]` `/workdir/wepp-forest/src/eatcom.for:24-39` (comment/blank-line behavior: first non-space `#` treated as comment line).
- `[DIRECT][E-WF-07]` `/workdir/wepp-forest/src/readin.for:17-22` and `/workdir/wepp-forest/src/pntype.inc:6-8` (legacy read gate for `nwsofe` range `1..ntype`; `ntype=20` in current forest build).
- `[DIRECT][E-WF-08]` `/workdir/wepp-forest/src/pmxslp.inc:6-12` (`mxslp=100` compile-time slope-point array bound in current forest build).
- `[DIRECT][E-WF-09]` `/workdir/wepp-forest/src/inidat.for:1158` and `/workdir/wepp-forest/src/verchk.for:19-31` (legacy slope compatibility threshold `slpchk=91.5`; incompatible versions stop).
- `[DIRECT][E-WP-01]` `/workdir/wepppy/wepppy/topo/watershed_abstraction/slope_file.py:73-95` and `:82-85` (wepppy slope helper reads non-comment lines, expects one OFE, and supports a `2023*` 3-value metadata variant).
- `[DIRECT][E-WP3-01]` `/workdir/wepppyo3/wepp_interchange/src/mofe.rs:206-299` (wepppyo3 MOFE utility parser constraints: single OFE, min 5 lines, `nSegments>=2`, exact pair count).
- `[DIRECT][E-WP3-02]` `/workdir/wepppyo3/wepp_interchange/src/mofe.rs:142-186` (wepppyo3 MOFE writer emits `97.5`, OFE count, and normalized pair rows).
- `[DIRECT][E-PERIDOT-01]` `/workdir/peridot/src/watershed_abstraction/flowpath.rs:217-246` (Peridot writer emits `datver=2023.3`, OFE metadata `azm fwidth elevation`, and comma-delimited slope pairs).

## 1. Surface Scope and Applicability
This specification defines the canonical `.slp` hillslope slope-input surface used to describe OFE geometry for WEPP hillslope runs.

- Canonical filename class: `*.slp`. `[INFERENCE][E-US-01]`
- Applicability: hillslope profile geometry (orientation, representative width, OFE lengths, and slope-point pairs). `[DIRECT][E-US-02]`
- OFE semantics: each OFE is a homogeneous strip for soils/cropping/management; slope inputs are per OFE. `[DIRECT][E-US-03]`
- Watershed channel-slope inputs are out of scope here and belong to separate channel/watershed specifications. `[INFERENCE][E-US-01]`
- Peridot channel bundle formats (`datver=2025.8`) and `.slps` bundle parsing are explicitly out of scope for this surface. `[DIRECT][E-PERIDOT-01]`

## 2. Version/datver Applicability Matrix

| Input form | First non-comment line | Legacy interpretation | openWEPP draft interpretation | Evidence |
|---|---|---|---|---|
| Canonical current | `97.5` | Datver branch; compatibility check | `MUST` accept `97.5` | `[DIRECT][E-US-01]`, `[DIRECT][E-WF-01]` |
| Peridot hillslope | `2023.3` | Non-usersum extension from first-party producer | `MUST` accept in strict and compatibility modes for hillslope `.slp`; parse OFE metadata as `azm fwidth elevation` | `[DIRECT][E-PERIDOT-01]`, `[DIRECT][E-WP-01]` |
| Legacy no-version-line | integer OFE count | Fallback branch: line 1 treated as `nwsofe` | `SHOULD` support via compatibility mode; strict mode `MUST` reject with typed error | `[DIRECT][E-US-04]`, `[DIRECT][E-WF-01]` |
| Older explicit datver | e.g., `91.5+` | Allowed when `datver >= slpchk` | `MAY` support in compatibility mode pending disposition | `[DIRECT][E-WF-09]` |
| Unsupported datver | `< slpchk` when parsed as datver | Hard stop in legacy `verchk` | `MUST` raise typed error | `[DIRECT][E-WF-09]` |

## 3. Record Grammar and Line-by-Line Format Definition

### 3.1 Canonical grammar (normative draft)

```ebnf
slope_file          = [datver_line] nelem_line ofe_block{nelem} ;
datver_line         = real ;   (* strict accepts 97.5 or 2023.3 *)
nelem_line          = integer ;
ofe_block           = line3 line4 slope_pairs ;
line3               = azm fwidth [elevation] ; (* elevation required when datver=2023.3 *)
line4               = nslpts slplen ;
slope_pairs         = (xinput slpinp){nslpts} ;  (* whitespace and/or comma delimiters; may span physical lines *)
```

- `slope_pairs` may be continued across one or more physical lines. `[DIRECT][E-US-02]`
- A minimum of two pairs per OFE is required. `[DIRECT][E-US-04]`

### 3.2 Line definitions

- Line 1 (optional in legacy mode): `datver` (real), strict accepts `97.5` or `2023.3`. `[DIRECT][E-US-02]`, `[DIRECT][E-PERIDOT-01]`
- Next line: `nelem` (integer OFE count). `[DIRECT][E-US-02]`
- Per OFE Line 3: `azm` and `fwidth`; when `datver=2023.3`, append `elevation`. `[DIRECT][E-US-02]`, `[DIRECT][E-PERIDOT-01]`
- Per OFE Line 4: `nslpts` and `slplen`. `[DIRECT][E-US-02]`
- Per OFE Line 5+: repeated `(xinput, slpinp)` pairs for `nslpts` (comma delimiters tolerated). `[DIRECT][E-US-02]`, `[DIRECT][E-PERIDOT-01]`

## 4. Field Dictionary With Canonical Symbols and openWEPP Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Constraints | openWEPP alias |
|---|---|---|---|---|---|---|---|
| `datver` | slope-file format/version marker | none | real | 0..1 per file | yes in canonical mode; optional in legacy mode | strict accepted values: `97.5`, `2023.3`; compatibility-mode branch for omitted line | `slope_file.datver` |
| `nelem` (`nwsofe` in legacy reader path) | number of OFEs | count | integer | 1 per file | yes | usersum narrative: up to 10; legacy build gate currently `1..ntype` with `ntype=20` | `slope_file.ofe_count` |
| `azm` | profile aspect from north | degrees | real | 1 per OFE | yes | finite numeric | `ofe[i].aspect_deg` |
| `fwidth` | representative profile width | m | real | 1 per OFE | yes | `> 0` | `ofe[i].width_m` |
| `elevation` (`z0` in wepppy helper) | Peridot OFE reference elevation metadata | m | real | 0..1 per OFE | required when `datver=2023.3`; absent otherwise | finite numeric | `ofe[i].elevation_m` |
| `nslpts` | number of slope points on OFE | count | integer | 1 per OFE | yes | `>= 2`; integer | `ofe[i].point_count` |
| `slplen` | OFE length | m | real | 1 per OFE | yes | `> 0` | `ofe[i].length_m` |
| `xinput` | distance from top of OFE to point | m or nondim (m/m) | real | `nslpts` per OFE | yes | all points within one OFE use one distance mode; first point `0`; last point `slplen` or `1.0` by mode | `ofe[i].points[j].x` |
| `slpinp` | slope steepness at point | m/m | real | `nslpts` per OFE | yes | finite numeric; border continuity required across adjoining OFEs | `ofe[i].points[j].slope_m_per_m` |

### 4.1 Alias mapping notes
- Canonical symbol names in this spec default to legacy WEPP/wepp-forest nomenclature (`datver`, `nelem`, `azm`, `fwidth`, `nslpts`, `slplen`, `xinput`, `slpinp`). `[DIRECT][E-US-02]`
- openWEPP boundary names above are aliases only; canonical symbols remain authoritative for provenance and contract traceability. `[INFERENCE][E-US-02]`

## 5. Conditional Branches and Optional Sections

1. Datver-present vs legacy-no-datver branch.
- If first non-comment numeric token is `> 10.0`, legacy reader treats it as datver and runs compatibility check.
- Otherwise, legacy reader rewinds and treats line 1 as OFE count.
- `[DIRECT][E-WF-01]`

2. `datver=2023.3` metadata branch.
- For Peridot `2023.3`, each OFE metadata row is `azm fwidth elevation` (three values on one row).
- Comma-delimited pair tokens are accepted for slope-point rows.
- `[DIRECT][E-PERIDOT-01]`, `[DIRECT][E-WP-01]`

3. Distance mode for `xinput`.
- Users may encode distances as absolute meters or nondimensional (`x/slplen`), but the two methods must not be mixed within an OFE profile.
- `[DIRECT][E-US-04]`

4. Physical-line wrapping of slope pairs.
- Pair list may span multiple lines; parser consumes `2*nslpts` numeric values.
- `[DIRECT][E-US-02]`, `[INFERENCE][E-WP3-01]`

5. Comment/blank lines.
- Legacy read path skips lines whose first non-space character is `#`, and skips blank lines.
- `[DIRECT][E-WF-06]`

## 6. Cross-File Consistency Constraints and Coupling Dependencies

1. OFE count coupling.
- Slope-file OFE count must match management-derived hillslope OFE count (`jstruc`); legacy runtime rejects mismatch.
- `[DIRECT][E-WF-02]`

2. Per-OFE completeness coupling.
- Soil/management/irrigation surfaces must also provide per-OFE values for the same OFE partition.
- `[DIRECT][E-US-03]`

3. Adjoining OFE border-slope continuity.
- Adjoining OFEs must have matching border point slopes.
- `[DIRECT][E-US-03]`

4. Endpoint closure for each OFE.
- Minimum point set includes top (`x=0`) and OFE end (`x=slplen` or `x=1.0` in nondimensional mode).
- `[DIRECT][E-US-04]`

## 7. Defaulting and Missing-File Behavior (Typed Error Expectations)

openWEPP parser-contract targets should enforce typed outcomes instead of silent mutation.

| Condition | Expected behavior |
|---|---|
| Missing `.slp` file | return `InputFileMissing(surface_id=infile-slope-slp)` |
| Empty/insufficient records | return `RecordCountError` |
| Missing `datver` line while compatibility mode is disabled | return `MissingDatverHeaderError` |
| Unsupported datver | return `UnsupportedDatver` |
| `datver=2023.3` metadata row not exactly `azm fwidth elevation` | return `RecordCountError` |
| `nelem < 1` or above accepted cap | return `FieldRangeError(field=nelem)` |
| `nslpts < 2` | return `FieldRangeError(field=nslpts)` |
| Non-finite/parse-failed numeric token | return `TokenParseError` |
| `slplen <= 0` | return `FieldRangeError(field=slplen)` |
| Mixed `xinput` mode within OFE | return `DistanceModeMixError` |
| Missing required endpoint point | return `EndpointConstraintError` |
| Adjoining OFE border slope mismatch | return `CrossOfeBoundaryError` |

- The above error names are draft contract targets for `SC-INFILE-SLOPE-001`. `[INFERENCE][E-US-02]`, `[INFERENCE][E-WF-01]`
- Legacy runtime includes internal numeric guards (e.g., clamping non-positive derived average slope), but this spec does not permit parser-side silent correction for malformed inputs. `[DIRECT][E-WF-05]`, `[INFERENCE][E-WF-05]`

## 8. Example Snippets

### 8.1 Minimal valid canonical example (single OFE)

```text
97.5
1
180.0 25.0
2 100.0
0.0 0.0500 1.0 0.0500
```

- Uses nondimensional `xinput` mode (`0.0`, `1.0`). `[DIRECT][E-US-04]`

### 8.2 Representative multi-OFE example

```text
97.5
2
180.0 30.0
3 60.0
0.0 0.0200 0.6 0.0800 1.0 0.0600
180.0 30.0
3 40.0
0.0 0.0600 0.5 0.0400 1.0 0.0300
```

- OFE border slope continuity shown by downstream slope of OFE1 (`0.0600`) matching upstream slope of OFE2 (`0.0600`). `[INFERENCE][E-US-03]`

### 8.3 Minimal valid Peridot `2023.3` example

```text
2023.3
1
180.0 35.0 1450.0
4 120.0
0.0000, 0.0200 0.3500, 0.0400 0.7000, 0.0300 1.0000, 0.0250
```

- Uses first-party Peridot branch with `azm fwidth elevation` OFE metadata and comma-delimited point pairs. `[DIRECT][E-PERIDOT-01]`

### 8.4 Invalid examples

1. Invalid: mixed `xinput` mode in one OFE
```text
97.5
1
180.0 25.0
3 100.0
0.0 0.0500 0.5 0.0400 100.0 0.0500
```
Reason: mixes nondimensional and meter encoding in one OFE (`0.5` normalized interior point with `100.0` meter endpoint). `[DIRECT][E-US-04]`

2. Invalid: missing terminal point
```text
97.5
1
180.0 25.0
2 100.0
0.0 0.0500 0.7 0.0400
```
Reason: terminal endpoint must be OFE end (`1.0` or `slplen`). `[DIRECT][E-US-04]`

3. Invalid: insufficient slope points
```text
97.5
1
180.0 25.0
1 100.0
0.0 0.0500
```
Reason: minimum two points required. `[DIRECT][E-US-04]`

## 9. Gap/Conflict Register and HOLD Conditions

| ID | Issue | Evidence | Provenance tags | Draft disposition |
|---|---|---|---|---|
| `SLOPE-GAP-001` | usersum says max 10 OFEs; current legacy build gate allows up to `ntype=20` | `[DIRECT][E-US-03]`, `[DIRECT][E-WF-07]` | `usersum2024`, `legacy-code` | `HOLD` until canonical cap policy is set for openWEPP |
| `SLOPE-GAP-002` | usersum table notes up to 20 slope pairs/OFE; current legacy arrays define `mxslp=100` | `[DIRECT][E-US-02]`, `[DIRECT][E-WF-08]` | `usersum2024`, `legacy-code` | `HOLD` until canonical cap and compatibility behavior are fixed |
| `SLOPE-GAP-003` | wepppy helper supports non-usersum `2023*` metadata variant (`azm fwidth z0`) | `[DIRECT][E-WP-01]` vs `[DIRECT][E-US-02]` | `usersum2024`, `wepppy` | `RESOLVED` by ratifying strict support for exact hillslope `datver=2023.3` in openWEPP (`INIMPL08`) |
| `SLOPE-NOTE-001` | wepppyo3 MOFE parser is single-OFE utility, not full canonical multi-OFE parser | `[DIRECT][E-WP3-01]` | `wepppyo3` | `NOTE` provenance-only, non-blocking |

`status` remains `draft-HOLD` due unresolved cap-policy gaps `SLOPE-GAP-001` and `SLOPE-GAP-002`; `SLOPE-GAP-003` is dispositioned.

## 10. Parser-Contract Handoff Map
- Target parser contract ID: `SC-INFILE-SLOPE-001`.
- Canonical contract path:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- This specification is the governing source for:
  - tokenization and comment-skipping semantics,
  - datver branch behavior,
  - OFE/block cardinalities,
  - endpoint and cross-OFE continuity constraints,
  - typed error surface.
- Parser-contract Sections 9-11 additionally govern boundary export mapping, explicit compat threshold handling (`datver >= 91.5` when enabled), and tolerance-bound closure checks.
- Contract authoring package linkage: `docs/work-packages/20260520-infile04-author-sc-infile-slope-001/`.
