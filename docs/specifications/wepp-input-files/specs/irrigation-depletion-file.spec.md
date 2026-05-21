# WEPP Irrigation Depletion-Level Input File Specification

## 1. Header Metadata
- `spec_id`: `SPEC-INFILE-IRRIGATION-DEPLETION-001`
- `surface_id`: `infile-irrigation-depletion`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-20T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence Anchors
- `[DIRECT][E-US-01]` `/home/workdir/openWEPP/references/vendorable/usersum2024.pdf` (Table 18, pages 61-63: depletion-level irrigation file format).
- `[DIRECT][E-US-02]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:6610-6622` (depletion-level schedule context and line-structure overview).
- `[DIRECT][E-US-03]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:6625-6687` (Table 18 field definitions, sprinkler vs furrow variants).
- `[DIRECT][E-US-04]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:6689-6701` (line-4 repetition/order and zero-value no-irrigation semantics).
- `[DIRECT][E-US-05]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:736-737` (irrigation input is required per OFE when simulated).
- `[DIRECT][E-US-06]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:8825-8829` (channel irrigation file uses hillslope irrigation format with channel-element IDs).
- `[DIRECT][E-US-07]` `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9178-9185` (run-file irrigation option codes 0..6).
- `[DIRECT][E-WF-01]` `/workdir/wepp-forest/src/infile.for:2057-2130` (depletion-file open on unit 15; datver probe; `itemp/jtemp/ktemp` validation against `jstruc/irsyst/1`).
- `[DIRECT][E-WF-02]` `/workdir/wepp-forest/src/irinpt.for:75-103` (sprinkler depletion initialization read: `irdmin irdmax`; per-OFE line-4 fields; `idsver<94.21` nozzle omission branch).
- `[DIRECT][E-WF-03]` `/workdir/wepp-forest/src/irinpt.for:145-164` (furrow depletion initialization read: `irdmin`; per-OFE line-4 fields; `depsrg` normalization; per-OFE schedule disabling).
- `[DIRECT][E-WF-04]` `/workdir/wepp-forest/src/irinpt.for:95-101` and `/workdir/wepp-forest/src/irinpt.for:156-162` (`irbeg==0` transition rules for `irschd` values).
- `[DIRECT][E-WF-05]` `/workdir/wepp-forest/src/irrig.for:545-590` (depletion-period continuation records are consumed during runtime when prior period ends).
- `[DIRECT][E-WF-06]` `/workdir/wepp-forest/src/input.for:366-369` (parse-time coupling: `nplane=jstruc`; `irinpt` called only when `irsyst!=0`).
- `[DIRECT][E-WF-07]` `/workdir/wepp-forest/src/inidat.for:1165-1168` and `/workdir/wepp-forest/src/cdat.inc:26-33` (legacy irrigation compatibility/version constants).
- `[DIRECT][E-WF-08]` `/workdir/wepp-forest/src/irinpt.for:127-140` (furrow irrigation disabled for contour and non-cropland; simulation continues with irrigation disabled).
- `[DIRECT][E-WP-01]` `/workdir/wepppy/wepp_runner/templates/hillslope.template:25-30` and `/workdir/wepppy/wepp_runner/templates/watershed.template:29-34` (current wepppy run templates default to `irrigation option = 0`, no depletion sidecar path in baseline template).
- `[DIRECT][E-WP3-01]` `/workdir/wepppyo3/README.md:66-73` (module surface emphasizes output/interchange and helper utilities; no dedicated irrigation input-file parser module listed).
- `[DIRECT][E-OW-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/input-surface-registry.md:24` (surface `infile-irrigation-depletion` and target contract ID are canonicalized).

## 2. Surface Scope and Applicability
- This specification covers the legacy unit-15 depletion-level irrigation sidecar consumed when irrigation option requires depletion scheduling. `[DIRECT][E-OW-01]`, `[DIRECT][E-US-07]`, `[DIRECT][E-WF-01]`
- Applies to both stationary sprinkler and furrow systems; line-3/line-4 structures branch by `jtemp` (`1` sprinkler, `2` furrow). `[DIRECT][E-US-03]`
- Applies in hillslope mode and channel irrigation mode where channels replace OFE IDs in increasing order. `[DIRECT][E-US-06]`
- This file is not consumed when irrigation option is `0` (no irrigation). `[DIRECT][E-US-07]`, `[DIRECT][E-WP-01]`

## 3. Version / datver Applicability Matrix

| Input form | First record interpretation | Legacy behavior | openWEPP draft stance | Evidence |
| --- | --- | --- | --- | --- |
| Canonical current | explicit `datver` line (Table 18 shows `95.7`) | accepted; `idsver/idfver` tracked | `MUST` accept explicit datver line | `[DIRECT][E-US-03]`, `[DIRECT][E-WF-01]` |
| Pre-93 legacy compatibility | first numeric token treated as line-2 header when `datver <= 2.0` probe | legacy backspaces and treats first line as `itemp jtemp ktemp` | `MAY` support in explicit compatibility mode | `[DIRECT][E-WF-01]` |
| Sprinkler depletion old variant | `idsver < 94.21` | nozzle not present; legacy sets `nozzle=1.0` | `MUST` decide compatibility behavior explicitly (see HOLD) | `[DIRECT][E-WF-02]`, `[DIRECT][E-WF-07]` |
| Furrow depletion compatibility floor | `irdfch = 91.5` constant | constants exist; direct `verchk` call currently commented in this path | `MUST` codify acceptance policy in parser contract | `[DIRECT][E-WF-01]`, `[DIRECT][E-WF-07]` |

## 4. Record Grammar and Line-by-Line Format Definition

### 4.1 Canonical stream grammar (depletion file)

```ebnf
irrig_depletion_file = [datver_line] header_line static_line first_period_block continuation_block* ;

datver_line          = real ;
header_line          = itemp jtemp ktemp ;
static_line          = sprinkler_static | furrow_static ;
first_period_block   = period_line{itemp} ;
continuation_block   = period_line ;

sprinkler_static     = irdmin irdmax ;
furrow_static        = irdmin ;

period_line          = sprinkler_period | furrow_period ;
sprinkler_period     = ofeflg irrate aprati deplev nozzle irbeg yrbeg irend yrend ;
furrow_period        = ofeflg endpln florat timest depsrg filrat deplev irbeg yrbeg irend yrend ;
```

- Line-2 tokens are: `itemp` (element count), `jtemp` (system type), `ktemp` (scheduling type, depletion=`1`). `[DIRECT][E-US-03]`
- Sprinkler depletion line-3 is `irdmin irdmax`; furrow depletion line-3 is `irdmin` only. `[DIRECT][E-US-03]`, `[DIRECT][E-WF-02]`, `[DIRECT][E-WF-03]`
- The first `itemp` line-4 records initialize period state for each element. `[DIRECT][E-US-04]`, `[DIRECT][E-WF-02]`, `[DIRECT][E-WF-03]`
- Additional line-4 records are a continuation stream, consumed in chronological/ordering sequence during simulation when each element reaches `yrend/irend`. `[DIRECT][E-US-04]`, `[DIRECT][E-WF-05]`

### 4.2 Line-4 ordering semantics
- First `n` records must be in increasing element ID (`ofeflg`) where `n=itemp`. `[DIRECT][E-US-04]`
- Continuation records are ordered by previous period end dates; ties are broken by increasing element ID. `[DIRECT][E-US-04]`
- "No additional periods" for an element is encoded by zero-valued period fields with nonzero `ofeflg`. `[DIRECT][E-US-04]`

## 5. Field Dictionary With Canonical Symbols and openWEPP Alias Mapping

| Canonical symbol | Meaning | Units | Type | Cardinality | Required | Core constraints | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | file version marker | none | real | 0..1 per file | yes in canonical mode | canonical in usersum is `95.7`; compatibility mode may omit line | `depletion_file.datver` |
| `itemp` | number of elements represented | count | integer | 1 per file | yes | must equal `jstruc`/`nplane` in legacy integration path | `depletion_file.element_count` |
| `jtemp` | irrigation system flag | code | integer | 1 per file | yes | `1` sprinkler, `2` furrow | `depletion_file.system_type` |
| `ktemp` | schedule-type flag | code | integer | 1 per file | yes | depletion file requires `1` | `depletion_file.schedule_type` |
| `irdmin` | minimum irrigation depth | m | real | 1 per file | yes | legacy mutates `<0.001` to `0.025` | `depletion_file.min_depth_m` |
| `irdmax` | maximum irrigation depth (sprinkler only) | m | real | 0..1 per file | required for sprinkler | omitted in furrow format | `depletion_file.max_depth_m` |
| `ofeflg` | element selector for period row | id | integer | repeated | yes | first `itemp` rows must be `1..itemp` in order | `period.element_id` |
| `irrate` | sprinkler application rate | m/s | real | repeated (sprinkler) | yes for sprinkler | finite, positive for active periods | `period.sprinkler_rate_m_per_s` |
| `aprati` | depth ratio to fill-to-field-capacity amount | ratio | real | repeated (sprinkler) | yes for sprinkler | finite numeric | `period.sprinkler_depth_ratio` |
| `deplev` | depletion ratio trigger threshold | ratio | real | repeated | yes | finite, physically meaningful ratio | `period.depletion_trigger_ratio` |
| `nozzle` | sprinkler nozzle energy factor | none | real | repeated (sprinkler) | yes for modern sprinkler | legacy pre-94.21 assigns `1.0` implicitly | `period.nozzle_factor` |
| `irbeg` | begin Julian day | day-of-year | integer | repeated | yes | zero sentinel disables further depletion for element | `period.start_doy` |
| `yrbeg` | begin year | year | integer | repeated | yes | paired with `irbeg` | `period.start_year` |
| `irend` | end Julian day | day-of-year | integer | repeated | yes | paired with `yrend`; ordering driver | `period.end_doy` |
| `yrend` | end year | year | integer | repeated | yes | ordering/continuation driver | `period.end_year` |
| `endpln` | terminal OFE reached by furrow advance | id | integer | repeated (furrow) | yes for furrow | valid OFE/element ID | `period.furrow_end_element_id` |
| `florat` | furrow supply rate | m3/s | real | repeated (furrow) | yes for furrow | finite, nonnegative for active periods | `period.furrow_supply_rate_m3_per_s` |
| `timest` | furrow supply duration estimate | s | real | repeated (furrow) | yes for furrow | finite, nonnegative | `period.furrow_supply_duration_s` |
| `depsrg` | number of furrow supply-rate/duration combinations | count/code | integer | repeated (furrow) | yes for furrow | usersum allows `1`, `2`, `4..6`; legacy remaps `3->4`, clamps `>6` to `6` | `period.furrow_surge_code` |
| `filrat` | furrow lower-end depth ratio target | m/m | real | repeated (furrow) | yes for furrow | finite numeric | `period.furrow_fill_ratio` |

### 5.1 Alias mapping notes
- Canonical symbols follow WEPP/legacy variable names (`itemp`, `jtemp`, `ktemp`, `ofeflg`, etc.) to preserve provenance continuity. `[DIRECT][E-US-03]`, `[DIRECT][E-WF-02]`
- openWEPP aliases are boundary-facing names only; canonical symbol names remain the normative contract surface. `[INFERENCE][E-OW-01]`

## 6. Conditional Branches and Optional Sections
1. System-type branch (`jtemp`).
- `jtemp=1` selects sprinkler line-3/line-4 layout.
- `jtemp=2` selects furrow line-3/line-4 layout.
- `[DIRECT][E-US-03]`, `[DIRECT][E-WF-02]`, `[DIRECT][E-WF-03]`

2. Scheduling-type branch (`ktemp`).
- Depletion file requires `ktemp=1`; other values are rejected in legacy open validation.
- `[DIRECT][E-US-03]`, `[DIRECT][E-WF-01]`

3. Datver/no-datver branch.
- Legacy accepts pre-93 style without explicit datver by probing first token and backspacing.
- `[DIRECT][E-WF-01]`

4. Optional continuation records.
- After first `itemp` period rows, extra rows are optional and used only if subsequent depletion periods are defined.
- `[DIRECT][E-US-04]`, `[DIRECT][E-WF-05]`

5. Zero-sentinel period termination.
- `irbeg=0` transitions schedule flags to fixed-date-only (`3->2`) or none (`1->0`) for that element.
- `[DIRECT][E-WF-04]`

## 7. Cross-File Consistency Constraints and Coupling Dependencies
1. Element cardinality closure.
- `itemp` must match management-structured element count (`jstruc`/`nplane`) for the run.
- `[DIRECT][E-WF-01]`, `[DIRECT][E-WF-06]`, `[DIRECT][E-US-05]`

2. Run-option coupling.
- Depletion file is required only when irrigation option includes depletion scheduling (codes `2`, `3`, `5`, `6`).
- `[DIRECT][E-US-07]`, `[DIRECT][E-WF-01]`

3. System coupling.
- `jtemp` must equal run-level `irsyst` (`1` sprinkler or `2` furrow).
- `[DIRECT][E-WF-01]`

4. Period-stream coupling.
- Record order is part of runtime behavior: continuation lines are consumed by day/year progression and per-element period completion.
- `[DIRECT][E-US-04]`, `[DIRECT][E-WF-05]`

5. Channel-mode compatibility.
- Same format may be used for channel elements, replacing OFE IDs with channel IDs.
- `[DIRECT][E-US-06]`

## 8. Defaulting and Missing-File Behavior (Typed Error Expectations)

openWEPP parser-contract targets should expose typed outcomes and avoid silent mutation in normative mode.

| Condition | Legacy behavior | openWEPP draft expectation |
| --- | --- | --- |
| Missing required depletion file | interactive reopen loop/stop path | `InputFileMissing(surface_id=infile-irrigation-depletion)` |
| `ktemp != 1` for depletion file | legacy rejects and re-prompts | `ScheduleTypeMismatch(expected=1, observed=ktemp)` |
| `jtemp != irsyst` | legacy rejects and re-prompts | `IrrigationSystemMismatch` |
| `itemp != jstruc` | legacy rejects and re-prompts | `ElementCountMismatch` |
| `ofeflg` not in expected order | legacy warning; run may continue with misordered periods | `ElementOrderingError` in strict mode |
| `irdmin < 0.001` | legacy mutates to `0.025` | `FieldRangeError(field=irdmin)` in strict mode; compatibility mode optional |
| `depsrg == 3` or `depsrg > 6` (furrow) | legacy remaps/clamps (`3->4`, `>6->6`) | `FieldNormalizationRequired` or strict `FieldRangeError` policy pending disposition |
| Furrow with contour/non-cropland | legacy disables irrigation and continues | `UnsupportedIrrigationConfiguration` or compatibility warning-mode policy pending disposition |

- Error names above are draft targets for `SC-INFILE-IRRIGATION-DEPLETION-001`. `[INFERENCE][E-WF-01]`, `[INFERENCE][E-WF-03]`

## 9. Example Snippets

### 9.1 Minimal valid sprinkler depletion (single period per OFE)

```text
95.7
2 1 1
0.010 0.030
1 2.50e-6 1.00 0.50 1.00 120 2001 273 2001
2 2.20e-6 0.95 0.52 1.00 120 2001 273 2001
```

- `ktemp=1` indicates depletion scheduling. `[DIRECT][E-US-03]`

### 9.2 Minimal valid furrow depletion (single period per OFE)

```text
95.7
2 2 1
0.010
1 2 3.50e-4 7200 2 0.90 0.55 120 2001 273 2001
2 2 3.20e-4 7200 2 0.88 0.55 120 2001 273 2001
```

- Furrow line-3 includes only `irdmin`; line-4 includes `endpln/florat/timest/depsrg/filrat/...`. `[DIRECT][E-US-03]`

### 9.3 Representative continuation-period example (sprinkler)

```text
95.7
2 1 1
0.010 0.030
1 2.50e-6 1.00 0.50 1.00 120 2001 180 2001
2 2.20e-6 0.95 0.52 1.00 120 2001 190 2001
1 2.30e-6 0.90 0.50 1.00 181 2001 240 2001
2 2.10e-6 0.90 0.52 1.00 191 2001 250 2001
```

- First two line-4 rows initialize OFEs 1..2; additional rows are continuation periods consumed later in end-date order. `[DIRECT][E-US-04]`, `[DIRECT][E-WF-05]`

### 9.4 Invalid examples

1. Invalid: wrong schedule type (`ktemp=2`)
```text
95.7
2 1 2
0.010 0.030
1 2.50e-6 1.00 0.50 1.00 120 2001 273 2001
2 2.20e-6 0.95 0.52 1.00 120 2001 273 2001
```
Reason: depletion file requires `ktemp=1`. `[DIRECT][E-US-03]`, `[DIRECT][E-WF-01]`

2. Invalid: furrow missing required tokens on line-4
```text
95.7
1 2 1
0.010
1 2 3.50e-4 7200 2 0.90 0.55 120 2001 273
```
Reason: furrow line-4 requires 11 fields including `yrend`. `[DIRECT][E-US-03]`, `[DIRECT][E-WF-03]`

3. Invalid: `itemp` inconsistent with hillslope/OFE structure
```text
95.7
3 1 1
0.010 0.030
1 2.50e-6 1.00 0.50 1.00 120 2001 273 2001
2 2.20e-6 0.95 0.52 1.00 120 2001 273 2001
```
Reason: row count and run-level OFE structure mismatch (example run has 2 OFEs). `[DIRECT][E-WF-01]`, `[DIRECT][E-WF-06]`

## 10. Gap/Conflict Register and HOLD Conditions

| ID | Issue | Evidence | Draft disposition |
| --- | --- | --- | --- |
| `IRDEP-GAP-001` | usersum canonical datver is `95.7`, but legacy accepts no-datver/pre-93 branch and mixed compatibility constants (`94.21` / `91.5`) | `[DIRECT][E-US-03]`, `[DIRECT][E-WF-01]`, `[DIRECT][E-WF-07]` | `HOLD` until explicit openWEPP compatibility matrix is ratified |
| `IRDEP-GAP-002` | legacy silently mutates `irdmin` and `depsrg` values; openWEPP correctness policy prefers explicit typed handling | `[DIRECT][E-WF-03]`, `[DIRECT][E-WF-08]` | `HOLD` until strict-vs-compat policy is dispositioned |
| `IRDEP-GAP-003` | usersum defines a line-4 repetition stream, while initialization parser consumes first `itemp` rows and runtime consumes continuation rows; parser contract must define streaming model explicitly | `[DIRECT][E-US-04]`, `[DIRECT][E-WF-05]` | `HOLD` until data-model contract for continuation ingestion is finalized |
| `IRDEP-GAP-004` | wepppy templates default to no irrigation, and wepppyo3 module map does not declare an irrigation input parser surface | `[DIRECT][E-WP-01]`, `[DIRECT][E-WP3-01]` | `HOLD` as provenance gap (no modern reference implementation to triangulate parser behavior) |

`status` remains `draft-HOLD` until all high-impact gaps are dispositioned.

## 11. Parser-Contract Handoff Map
- Target parser contract ID: `SC-INFILE-IRRIGATION-DEPLETION-001`. `[DIRECT][E-OW-01]`

| Contract area | Source spec requirement | Parser-contract expectation |
| --- | --- | --- |
| Header validation | Section 4 header grammar | enforce `itemp/jtemp/ktemp` arity and depletion `ktemp=1` |
| System branching | Section 6 | parse sprinkler/furrow variants by `jtemp` with explicit schema |
| Streaming continuation | Sections 4 and 7 | support first-period initialization + continuation record ingestion model |
| Cross-file closure | Section 7 | validate against run-level `irsyst`, `jstruc/nplane`, irrigation option codes |
| Error semantics | Section 8 | typed errors in strict mode; explicit compatibility toggles only |
| Symbol continuity | Section 5 | preserve canonical WEPP symbols with explicit alias table |
| Correctness gate | Section 10 | unresolved gaps remain `HOLD` until disposition and verification are complete |

Handoff package linkage:
- `docs/work-packages/20260520-infile08-author-sc-infile-irrigation-depletion-001/`
