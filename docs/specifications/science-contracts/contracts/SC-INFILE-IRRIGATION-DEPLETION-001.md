---
contract_id: SC-INFILE-IRRIGATION-DEPLETION-001
title: Irrigation Depletion Input Parser Contract (legacy unit 15)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.1
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-IRRIGATION-DEPLETION-001 Irrigation Depletion Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-IRD-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md` (canonical format, branching, stream-order rules, and HOLD gaps).
- `[DIRECT][E-SURVEY-IRD-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and parser ownership context).
- `[DIRECT][E-WF-IRD-01]` `/workdir/wepp-forest/src/infile.for`, `/workdir/wepp-forest/src/irinpt.for`, `/workdir/wepp-forest/src/irrig.for`, `/workdir/wepp-forest/src/inidat.for`, `/workdir/wepp-forest/src/cdat.inc` (legacy open/probe behavior, initialization reads, continuation consumption, and compatibility constants cited by spec).
- `[INFERENCE][E-PHYS-IRD-01]` Physical/common-sense invariants: period dates are valid and ordered, flow/depth controls are non-negative when active, and continuation stream must remain deterministic.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for sidecar surface `infile-irrigation-depletion` (legacy unit 15) and parse-to-runtime handoff of depletion-schedule period streams for sprinkler/furrow irrigation modes.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | explicit `datver=95.7` | Accept in strict and compatibility modes. | Canonical modern path. | `[DIRECT][E-SPEC-IRD-01]` |
| B | explicit legacy datver by system type | Strict reject. Compat accepts only bounded legacy sets: sprinkler (`jtemp=1`): `94.21 <= datver < 95.7`; furrow (`jtemp=2`): `91.5 <= datver < 95.7`. | Emits typed compatibility warning on acceptance. | `[DIRECT][E-SPEC-IRD-01]`, `[DIRECT][E-WF-IRD-01]` |
| C | legacy no-datver probe branch | Strict reject. Compat-only optional path. | Accept only with explicit compatibility mode and warning. | `[DIRECT][E-SPEC-IRD-01]`, `[DIRECT][E-WF-IRD-01]` |
| D | explicit datver outside accepted sets | Reject. | Emit typed `UnsupportedDatver`. | `[DIRECT][E-SPEC-IRD-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
ird_file = [datver_line] header_line static_line first_period_rows continuation_rows* ;

header_line = itemp jtemp ktemp ;
static_line = sprinkler_static | furrow_static ;
first_period_rows = period_row{itemp} ;
continuation_rows = period_row* ;

sprinkler_static = irdmin irdmax ;
furrow_static = irdmin ;

period_row = sprinkler_row | furrow_row ;
sprinkler_row = ofeflg irrate aprati deplev nozzle irbeg yrbeg irend yrend ;
furrow_row = ofeflg endpln florat timest depsrg filrat deplev irbeg yrbeg irend yrend ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves header/static records and an ordered stream of period rows as encountered.
- Simulation model normalizes into:
  - file-level schedule metadata (`system_type`, `schedule_type`, min/max depth settings),
  - per-element first-period records,
  - deterministic continuation stream records consumed by runtime period-transition logic.
- Parser produces typed period records but does not execute runtime period advancement.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `irrigation.depletion.version.datver` | none | real | 0..1 | conditional | see Section 1 | none | `depletion_file.datver` |
| `itemp` | `header.itemp` | `irrigation.depletion.element_count` | count | int | 1 | yes | all | none | `depletion_file.element_count` |
| `jtemp` | `header.jtemp` | `irrigation.depletion.system_type` | enum | int | 1 | yes | all | none | `depletion_file.system_type` |
| `ktemp` | `header.ktemp` | `irrigation.depletion.schedule_type` | enum | int | 1 | yes | all | none | `depletion_file.schedule_type` |
| `irdmin` | `static.irdmin` | `irrigation.depletion.min_depth_m` | m | real | 1 | yes | all | none | `depletion_file.min_depth_m` |
| `irdmax` | `static.irdmax` | `irrigation.depletion.max_depth_m` | m | real | subset(1, `jtemp=1`) | conditional | sprinkler | none | `depletion_file.max_depth_m` |
| `ofeflg` | `period[r].ofeflg` | `irrigation.depletion.periods[r].element_id` | id | int | n_period_rows | yes | all | none | `period.element_id` |
| `irrate` | `period[r].sprinkler.irrate` | `...periods[r].sprinkler.rate_m_per_s` | m/s | real | subset(rows, `jtemp=1`) | conditional | sprinkler | none | `period.sprinkler_rate_m_per_s` |
| `aprati` | `period[r].sprinkler.aprati` | `...periods[r].sprinkler.depth_ratio` | ratio | real | subset(rows, `jtemp=1`) | conditional | sprinkler | none | `period.sprinkler_depth_ratio` |
| `deplev` | `period[r].deplev` | `...periods[r].depletion_trigger_ratio` | ratio | real | n_period_rows | yes | all | none | `period.depletion_trigger_ratio` |
| `nozzle` | `period[r].sprinkler.nozzle` | `...periods[r].sprinkler.nozzle_factor` | none | real | subset(rows, `jtemp=1`) | conditional | sprinkler | compat-only fallback `1.0` for legacy row shape | `period.nozzle_factor` |
| `irbeg` | `period[r].irbeg` | `...periods[r].start_doy` | day-of-year | int | n_period_rows | yes | all | none | `period.start_doy` |
| `yrbeg` | `period[r].yrbeg` | `...periods[r].start_year` | year | int | n_period_rows | yes | all | none | `period.start_year` |
| `irend` | `period[r].irend` | `...periods[r].end_doy` | day-of-year | int | n_period_rows | yes | all | none | `period.end_doy` |
| `yrend` | `period[r].yrend` | `...periods[r].end_year` | year | int | n_period_rows | yes | all | none | `period.end_year` |
| `endpln` | `period[r].furrow.endpln` | `...periods[r].furrow.end_element_id` | id | int | subset(rows, `jtemp=2`) | conditional | furrow | none | `period.furrow_end_element_id` |
| `florat` | `period[r].furrow.florat` | `...periods[r].furrow.supply_rate_m3_per_s` | m^3/s | real | subset(rows, `jtemp=2`) | conditional | furrow | none | `period.furrow_supply_rate_m3_per_s` |
| `timest` | `period[r].furrow.timest` | `...periods[r].furrow.supply_duration_s` | s | real | subset(rows, `jtemp=2`) | conditional | furrow | none | `period.furrow_supply_duration_s` |
| `depsrg` | `period[r].furrow.depsrg` | `...periods[r].furrow.surge_code` | code | int | subset(rows, `jtemp=2`) | conditional | furrow | strict no normalization; compat may map legacy values with warning | `period.furrow_surge_code` |
| `filrat` | `period[r].furrow.filrat` | `...periods[r].furrow.fill_ratio` | m/m | real | subset(rows, `jtemp=2`) | conditional | furrow | none | `period.furrow_fill_ratio` |
| derived `initial_rows_complete` | first `itemp` rows | `irrigation.depletion.initialization_complete` | flag | bool | 1 | yes | all | derived row-count closure | `initialization_complete` |
| derived `continuation_rows` | rows after first `itemp` | `irrigation.depletion.continuation_stream` | list | record[] | 0..N | yes | all | preserve file order | `continuation_stream` |
| derived `continuation_order_key` | `(yrend,irend,ofeflg)` | `irrigation.depletion.periods[r].continuation_order_key` | tuple | tuple(int,int,int) | n_period_rows | yes | all | derived ordering-key metadata for continuation validation | `continuation_order_key` |
| derived `irbeg_zero_transition` | `irbeg==0` branch | `irrigation.depletion.periods[r].zero_start_transition` | enum | string | n_period_rows | yes | all | derives schedule-state transition marker (`3->2`, `1->0`, or `none`) | `zero_start_transition` |
| derived `furrow_disabled_by_landuse` | furrow with contour/non-cropland | `irrigation.depletion.periods[r].furrow_disabled_by_landuse` | flag | bool | subset(rows, `jtemp=2`) | conditional | furrow | derived from management/topology coupling | `furrow_disabled_by_landuse` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `irrigation.depletion.version` | `input::sidecar::irrigation_depletion` | init | immutable | compatibility policy gate | `G-IRD-001` |
| `itemp` | `header.itemp` | `irrigation.depletion.element_count` | `input::sidecar::irrigation_depletion` | init | immutable | run-level element-count closure | `G-IRD-002`, `G-IRD-009` |
| `jtemp` | `header.jtemp` | `irrigation.depletion.system_type` | `input::sidecar::irrigation_depletion` | init | immutable | system-type closure and datver compatibility branch | `G-IRD-002`, `G-IRD-003`, `G-IRD-010` |
| `ktemp` | `header.ktemp` | `irrigation.depletion.schedule_type` | `input::sidecar::irrigation_depletion` | init | immutable | depletion-schedule gate (`ktemp=1`) | `G-IRD-002` |
| `irdmin` | `static.irdmin` | `irrigation.depletion.depth_policy.min_depth_m` | `input::sidecar::irrigation_depletion` | init,daily | immutable | irrigation depth-control routines | `G-IRD-004` |
| `irdmax` | `static.irdmax` | `irrigation.depletion.depth_policy.max_depth_m` | `input::sidecar::irrigation_depletion` | init,daily | immutable | sprinkler depth upper bound | `G-IRD-004` |
| `ofeflg` | `periods[*].ofeflg` | `irrigation.depletion.periods[*].element_id` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | scheduler per-element addressing | `G-IRD-005`, `G-IRD-006` |
| `irrate` | `periods[*].sprinkler.irrate` | `irrigation.depletion.periods[*].sprinkler.rate_m_per_s` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | sprinkler scheduling logic | `G-IRD-005`, `G-IRD-006` |
| `aprati` | `periods[*].sprinkler.aprati` | `irrigation.depletion.periods[*].sprinkler.depth_ratio` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | sprinkler scheduling logic | `G-IRD-005`, `G-IRD-006` |
| `deplev` | `periods[*].deplev` | `irrigation.depletion.periods[*].depletion_trigger_ratio` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | depletion trigger logic | `G-IRD-005`, `G-IRD-006` |
| `nozzle` | `periods[*].sprinkler.nozzle` | `irrigation.depletion.periods[*].sprinkler.nozzle_factor` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | sprinkler energy/depth adjustment | `G-IRD-005`, `G-IRD-006`, `G-IRD-010` |
| `irbeg` | `periods[*].irbeg` | `irrigation.depletion.periods[*].start_doy` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | period activation logic | `G-IRD-005`, `G-IRD-006`, `G-IRD-011` |
| `yrbeg` | `periods[*].yrbeg` | `irrigation.depletion.periods[*].start_year` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | period activation logic | `G-IRD-005`, `G-IRD-006`, `G-IRD-011` |
| `irend` | `periods[*].irend` | `irrigation.depletion.periods[*].end_doy` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | period transition logic | `G-IRD-005`, `G-IRD-006`, `G-IRD-008` |
| `yrend` | `periods[*].yrend` | `irrigation.depletion.periods[*].end_year` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | period transition logic | `G-IRD-005`, `G-IRD-006`, `G-IRD-008` |
| `endpln` | `periods[*].furrow.endpln` | `irrigation.depletion.periods[*].furrow.end_element_id` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | furrow routing target logic | `G-IRD-005`, `G-IRD-006` |
| `florat` | `periods[*].furrow.florat` | `irrigation.depletion.periods[*].furrow.supply_rate_m3_per_s` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | furrow flow scheduling | `G-IRD-005`, `G-IRD-006` |
| `timest` | `periods[*].furrow.timest` | `irrigation.depletion.periods[*].furrow.supply_duration_s` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | furrow duration scheduling | `G-IRD-005`, `G-IRD-006` |
| `depsrg` | `periods[*].furrow.depsrg` | `irrigation.depletion.periods[*].furrow.surge_code` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | furrow surge branch logic | `G-IRD-005`, `G-IRD-006`, `G-IRD-010` |
| `filrat` | `periods[*].furrow.filrat` | `irrigation.depletion.periods[*].furrow.fill_ratio` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | furrow fill-ratio logic | `G-IRD-005`, `G-IRD-006` |
| derived `initialization_complete` | `derived.initialization_complete` | `irrigation.depletion.initialization_complete` | `input::sidecar::irrigation_depletion` | init | immutable | initialization closure diagnostics | `G-IRD-007` |
| derived `continuation_rows` | `derived.continuation_rows` | `irrigation.depletion.continuation_stream` | `input::sidecar::irrigation_depletion` | init,event | immutable | runtime continuation ingestion | `G-IRD-008` |
| derived `continuation_order_key` | `derived.order_key[*]` | `irrigation.depletion.periods[*].continuation_order_key` | `input::sidecar::irrigation_depletion` | init,event | immutable | deterministic continuation validation and observability | `G-IRD-008` |
| derived `irbeg_zero_transition` | `derived.zero_start_transition[*]` | `irrigation.depletion.periods[*].zero_start_transition` | `input::sidecar::irrigation_depletion` | init,event | immutable | schedule-state transition observability | `G-IRD-011` |
| derived `furrow_disabled_by_landuse` | `derived.furrow_disabled[*]` | `irrigation.depletion.periods[*].furrow_disabled_by_landuse` | `input::sidecar::irrigation_depletion` | init,daily,event | immutable | strict/compat furrow-disable policy enforcement | `G-IRD-012` |

## 5. State Ownership and Mutability

- `input::sidecar::irrigation_depletion` owns parsed source rows and normalized period-stream state.
- Parsed rows are immutable after parse success.
- Runtime irrigation subsystem owns mutable active-period pointers/state per element.
- Forbidden mutation path: runtime modules rewriting canonical period rows or header metadata (`itemp/jtemp/ktemp`) in place.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-IRD-001` | Split period stream into initialization rows (`first itemp`) and continuation rows (`remaining`). | parse finalize | `C-IRD-001` |
| `D-IRD-002` | Derive per-element initialization map from first `itemp` ordered rows. | parse finalize | `C-IRD-002` |
| `D-IRD-003` | Derive continuation ordering key (`prior end date, then element id`) observability metadata for runtime scheduler checks. | parse finalize | `C-IRD-003` |
| `D-IRD-004` | Derive `irbeg==0` schedule-state transition marker (`3->2`, `1->0`, else `none`). | parse finalize | `C-IRD-004` |
| `D-IRD-005` | Derive furrow disable marker for contour/non-cropland contexts from management coupling. | cross-file finalize | `C-IRD-005` |

Closure hooks:
- `C-IRD-001`: first `itemp` rows must be present.
- `C-IRD-002`: first `itemp` rows must cover expected element IDs in deterministic order.
- `C-IRD-003`: continuation stream rows must satisfy deterministic ordering policy or raise strict error / compat warning.
- `C-IRD-004`: `irbeg==0` branch must emit explicit transition marker and typed compatibility/strict outcome.
- `C-IRD-005`: furrow disable policy branch must be explicit and mode-gated (strict error or compatibility disable-warning behavior).

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `IRD-E-000` | io | missing/unopenable depletion file |
| `IRD-E-001` | syntax | token parse failure in required numeric fields |
| `IRD-E-002` | syntax | record arity mismatch for selected sprinkler/furrow row shape |
| `IRD-E-003` | semantic | unsupported datver policy result |
| `IRD-E-004` | semantic | invalid header domain (`itemp<=0`, unsupported `jtemp`, `ktemp!=1`) |
| `IRD-E-005` | semantic | invalid field ranges/domains (negative rates/durations where disallowed, invalid day/year domains) |
| `IRD-E-006` | cross-file | run-level closure mismatch (`itemp` vs topology count, `jtemp` vs run irrigation system, irrigation option mismatch) |
| `IRD-E-007` | cross-file | invalid channel/hillslope ID domain for current run mode |
| `IRD-E-008` | runtime-guard | continuation stream closure/ordering failure |
| `IRD-E-009` | cross-file | furrow irrigation disallowed under contour/non-cropland in strict mode |
| `IRD-W-001` | compat-warning | legacy no-datver branch accepted |
| `IRD-W-002` | compat-warning | legacy sprinkler nozzle default (`nozzle=1.0`) applied |
| `IRD-W-003` | compat-warning | legacy `depsrg` normalization (`3->4`, `>6->6`) applied |
| `IRD-W-004` | compat-warning | `irbeg==0` schedule-state transition applied |
| `IRD-W-005` | compat-warning | furrow irrigation disabled for contour/non-cropland compatibility path |
| `IRD-W-006` | compat-warning | legacy explicit datver acceptance (`94.21..95.7` sprinkler or `91.5..95.7` furrow) |

No silent fallback/default masking is allowed in strict mode.

## 8. Cross-File Consistency Constraints

1. `itemp` must match run-level element count authority (`jstruc`/`nplane` depending run context). `[DIRECT][E-SPEC-IRD-01]`
2. `jtemp` must match run irrigation-system mode (`1` sprinkler, `2` furrow). `[DIRECT][E-SPEC-IRD-01]`
3. Depletion file is valid only when run-level irrigation option uses depletion scheduling. `[DIRECT][E-SPEC-IRD-01]`
4. For channel irrigation mode, `ofeflg`/`endpln` IDs refer to channel element IDs, not hillslope OFE IDs. `[DIRECT][E-SPEC-IRD-01]`
5. Continuation rows must be deterministic for runtime consumption (ordered by prior end date, ties by element ID). `[DIRECT][E-SPEC-IRD-01]`, `[INFERENCE][E-PHYS-IRD-01]`
6. Furrow mode (`jtemp=2`) coupled with contour/non-cropland management context follows explicit mode policy: strict mode emits `IRD-E-009`; compatibility mode disables irrigation for affected period(s) and emits `IRD-W-005`. `[DIRECT][E-SPEC-IRD-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `datver,itemp,jtemp,ktemp` | `irrigation.depletion.version`, `meta` | parser output manifest | canonical symbols preserved + aliases (`depletion_file.*`) | strict/compat mode annotations exported |
| static line symbols (`irdmin,irdmax`) | `irrigation.depletion.depth_policy` | irrigation scheduler config boundary | canonical names with typed units (`m`) | `irdmax` present only for sprinkler form |
| period symbols (`ofeflg,irrate,aprati,deplev,nozzle,irbeg,yrbeg,irend,yrend,endpln,florat,timest,depsrg,filrat`) | `irrigation.depletion.period_stream` | runtime period-scheduler boundary | canonical field continuity with branch-specific typed records | no parser-side time advancement |
| derived stream partitions and flags (`initialization_complete`,`continuation_rows`,`continuation_order_key`,`irbeg_zero_transition`,`furrow_disabled_by_landuse`) | `initial_period_by_element`, `continuation_stream`, `periods[*].derived` | runtime transition boundary | explicit derived fields preserving deterministic ordering and transition semantics | supports kernel stimulation without full end-to-end run |

## 10. Compatibility Policy

- Strict mode:
  - requires explicit datver/header forms with `datver=95.7`;
  - rejects legacy no-datver format;
  - rejects sprinkler rows missing `nozzle` and furrow rows with malformed arity;
  - rejects legacy value rewrites (`irdmin` mutation, `depsrg` remap/clamp);
  - rejects furrow-on-contour/non-cropland branch as `IRD-E-009`.
- Compatibility mode:
  - may accept legacy no-datver branch with `IRD-W-001`;
  - may accept legacy explicit datver sets only: sprinkler `94.21 <= datver < 95.7`, furrow `91.5 <= datver < 95.7`, with `IRD-W-006`;
  - may accept sprinkler missing-`nozzle` rows by injecting `nozzle=1.0` with `IRD-W-002`;
  - may support legacy `depsrg` remap/clamp only with `IRD-W-003`;
  - emits `IRD-W-004` for `irbeg==0` transitions;
  - disables furrow irrigation for contour/non-cropland contexts with `IRD-W-005`.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-IRD-001` | datver policy gate | preamble parse | `IRD-E-003` |
| `G-IRD-002` | header domain (`itemp>0`, `jtemp in {1,2}`, `ktemp==1`) | header parse | `IRD-E-004` |
| `G-IRD-003` | run-mode coupling (`jtemp`, irrigation option) | cross-surface validator | `IRD-E-006` |
| `G-IRD-004` | static-line arity/branch closure (`irdmin`, optional `irdmax`) | static parse | `IRD-E-002`/`IRD-E-005` |
| `G-IRD-005` | period row arity by system type | row parse | `IRD-E-002` |
| `G-IRD-006` | period field domain checks (rates, dates, ratios, IDs) | row parse | `IRD-E-005`/`IRD-E-007` |
| `G-IRD-007` | first `itemp` initialization row closure | parse finalize | `IRD-E-008` |
| `G-IRD-008` | continuation stream deterministic ordering policy | parse finalize/runtime guard | `IRD-E-008` |
| `G-IRD-009` | cross-file element-count closure | cross-surface validator | `IRD-E-006` |
| `G-IRD-010` | concrete strict/compat datver acceptance sets and legacy normalization warnings | preamble policy gate | `IRD-E-003`/`IRD-W-001`/`IRD-W-002`/`IRD-W-003`/`IRD-W-006` |
| `G-IRD-011` | `irbeg==0` transition observability and branch closure | parse finalize | `IRD-W-004`/`IRD-E-008` |
| `G-IRD-012` | furrow disable policy for contour/non-cropland contexts | cross-file policy gate | `IRD-E-009`/`IRD-W-005` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`datver`, `itemp`, `jtemp`, `ktemp`, `irdmin`, `irdmax`, `ofeflg`, `irrate`,
`aprati`, `deplev`, `nozzle`, `irbeg`, `yrbeg`, `irend`, `yrend`, `endpln`,
`florat`, `timest`, `depsrg`, `filrat`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `IRD-GAP-001` | Canonical datver compatibility matrix (`95.7` vs legacy no-datver + mixed constants) requires fixture-backed policy ratification. | `[DIRECT][E-SPEC-IRD-01]` | `HOLD` |
| `IRD-GAP-002` | Strict-vs-compat policy for legacy silent value rewrites (`irdmin`, `depsrg`) requires final governance decision. | `[DIRECT][E-SPEC-IRD-01]` | `HOLD` |
| `IRD-GAP-003` | Runtime continuation-stream ingestion contract boundaries (parse-layer vs scheduler-layer responsibilities) need explicit architecture linkage docs. | `[DIRECT][E-SPEC-IRD-01]`, `[INFERENCE][E-WF-IRD-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.1` | Added concrete datver acceptance sets, symbol-level propagation rows, `irbeg==0` transition modeling, and explicit strict/compat furrow-disable policy guards. |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE08. |
