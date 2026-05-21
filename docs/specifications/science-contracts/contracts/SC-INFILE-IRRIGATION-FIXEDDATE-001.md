---
contract_id: SC-INFILE-IRRIGATION-FIXEDDATE-001
title: Fixed-Date Irrigation Input Parser Contract (legacy unit 14)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-21T00:00:00Z
---

# SC-INFILE-IRRIGATION-FIXEDDATE-001 Fixed-Date Irrigation Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-FDIR-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md` (canonical fixed-date sidecar grammar, symbols, and open gaps).
- `[DIRECT][E-SURVEY-FDIR-01]` `/home/workdir/openWEPP/docs/planning/wepp-input-file-parser-survey.md` (surface provenance and parser ownership notes).
- `[DIRECT][E-WF-FDIR-01]` `/workdir/wepp-forest/src/infile.for`, `/workdir/wepp-forest/src/irinpt.for`, `/workdir/wepp-forest/src/irrig.for`, `/workdir/wepp-forest/src/inidat.for`, `/workdir/wepp-forest/src/cdat.inc`, `/workdir/wepp-forest/src/cirfixd.inc` (legacy parse/event-consumption branches and compatibility constants).
- `[INFERENCE][E-PHYS-FDIR-01]` Process/common-sense invariants: event dates must be valid and deterministic, OFE references must remain in-domain, and surge windows must be non-negative and ordered.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for sidecar surface `infile-irrigation-fixeddate` (legacy unit `14`) and parse-to-runtime handoff of fixed-date irrigation event streams for sprinkler/furrow systems.

### 1.2 Version/Datver Applicability Matrix

| Case | Input form | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | explicit `datver=95.7` | Accept in strict and compatibility modes. | Canonical modern path. | `[DIRECT][E-SPEC-FDIR-01]` |
| B | explicit legacy datver (`jtemp=1`: `94.21 <= datver < 95.7`; `jtemp=2`: `91.5 <= datver < 95.7`) | Provisional policy: strict reject, compat candidate. Authority conflict remains open because fixed-date `verchk` enforcement is commented in legacy path. | Accept only with typed compatibility warning when compatibility mode explicitly enabled. | `[DIRECT][E-WF-FDIR-01]`, `[INFERENCE][E-SPEC-FDIR-01]` |
| C | legacy no-datver probe branch (`first token <= 2`) | Strict reject. Compat-only optional branch. | Accept only with explicit compatibility mode and warning; emit branch provenance marker. | `[DIRECT][E-WF-FDIR-01]` |
| D | explicit datver outside accepted sets | Reject. | Emit typed `UnsupportedDatver`. | `[DIRECT][E-SPEC-FDIR-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
fixeddate_file        = [datver_line] header_line initial_dates event_stream ;

datver_line           = real ;
header_line           = itemp jtemp ktemp ;
initial_dates         = line3{itemp} ;
line3                 = ofeflg irday iryr ;

event_stream          = sprinkler_stream | furrow_stream ;
sprinkler_stream      = { sprinkler_line4 line3 } ;
sprinkler_line4       = irint irdept [nozzle] ;

furrow_stream         = { furrow_line4 furrow_line5{surges} line3 } ;
furrow_line4          = surges ;
furrow_line5          = furrow_line5_canonical | furrow_line5_legacy_compat ;
furrow_line5_canonical = qspply tstart tend tdepl ;
furrow_line5_legacy_compat = qspply tstart tend ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves ordered initialization `line3` records and subsequent event-payload/next-date record stream exactly as parsed.
- Simulation model normalizes into typed:
  - header metadata (`itemp`, `jtemp`, `ktemp`, optional `datver`),
  - initialization event-date map by OFE,
  - event stream records with branch-specific sprinkler/furrow payload,
  - derived compatibility/provenance annotations.
- Parser does not execute event advancement; runtime scheduler consumes normalized events.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `irrigation.fixeddate.version.datver` | none | real | 0..1 | conditional | see Section 1 | none | `fixeddate_irrigation.datver` |
| `itemp` | `header.itemp` | `irrigation.fixeddate.ofe_count` | count | int | 1 | yes | all | none | `fixeddate_irrigation.ofe_count` |
| `jtemp` | `header.jtemp` | `irrigation.fixeddate.system_flag` | enum | int | 1 | yes | all | none | `fixeddate_irrigation.system_flag` |
| `ktemp` | `header.ktemp` | `irrigation.fixeddate.schedule_flag` | enum | int | 1 | yes | all | none | `fixeddate_irrigation.schedule_flag` |
| `ofeflg` | `line3.ofeflg` | `irrigation.fixeddate.records[*].ofe_id` | id | int | n_records | yes | all | none | `event.ofe_id` |
| `irday` | `line3.irday` | `irrigation.fixeddate.records[*].event_doy` | day-of-year | int | n_records | yes | all | none | `event.day_of_year` |
| `iryr` | `line3.iryr` | `irrigation.fixeddate.records[*].event_year` | year | int | n_records | yes | all | none | `event.year` |
| `irint` | `sprinkler_line4.irint` | `irrigation.fixeddate.events[*].sprinkler.rate_m_per_s` | m/s | real | subset(records, `jtemp=1`) | conditional | sprinkler | none | `sprinkler_event.application_rate_mps` |
| `irdept` | `sprinkler_line4.irdept` | `irrigation.fixeddate.events[*].sprinkler.depth_m` | m | real | subset(records, `jtemp=1`) | conditional | sprinkler | none | `sprinkler_event.depth_m` |
| `nozzle` | `sprinkler_line4.nozzle` | `irrigation.fixeddate.events[*].sprinkler.nozzle_factor` | none | real | subset(records, `jtemp=1`) | conditional | sprinkler | compat fallback `1.0` when legacy two-field row accepted | `sprinkler_event.nozzle_factor` |
| `surges` | `furrow_line4.surges` | `irrigation.fixeddate.events[*].furrow.surge_count` | count | int | subset(records, `jtemp=2`) | conditional | furrow | none | `furrow_event.surge_count` |
| `qspply` | `furrow_line5.qspply` | `irrigation.fixeddate.events[*].furrow.surges[*].supply_rate_m3_per_s` | m^3/s | real | subset(surges, `jtemp=2`) | conditional | furrow | none | `furrow_event.surges[i].supply_rate_m3ps` |
| `tstart` | `furrow_line5.tstart` | `irrigation.fixeddate.events[*].furrow.surges[*].start_s` | s | real | subset(surges, `jtemp=2`) | conditional | furrow | none | `furrow_event.surges[i].start_s` |
| `tend` | `furrow_line5.tend` | `irrigation.fixeddate.events[*].furrow.surges[*].end_s` | s | real | subset(surges, `jtemp=2`) | conditional | furrow | none | `furrow_event.surges[i].end_s` |
| `tdepl` | `furrow_line5.tdepl` | `irrigation.fixeddate.events[*].furrow.surges[*].depletion_duration_s` | s | real | subset(surges, `jtemp=2`) | conditional | furrow | compat branch may preserve as omitted/null when three-field legacy row accepted | `furrow_event.surges[i].depletion_duration_s` |
| derived `initial_dates_complete` | first `itemp` line3 records | `irrigation.fixeddate.initial_dates_complete` | flag | bool | 1 | yes | all | derived from initialization record closure | `initial_dates_complete` |
| derived `event_stream_complete` | event payload + successor line3 stream | `irrigation.fixeddate.event_stream_complete` | flag | bool | 1 | yes | all | derived by branch-consistent record closure | `event_stream_complete` |
| derived `legacy_nozzle_default_applied` | missing sprinkler `nozzle` compat branch | `irrigation.fixeddate.events[*].sprinkler.legacy_nozzle_default_applied` | flag | bool | subset(records, `jtemp=1`) | conditional | sprinkler | derived in compatibility mode | `legacy_nozzle_default_applied` |
| derived `legacy_furrow_line5_arity` | furrow line5 token count | `irrigation.fixeddate.events[*].furrow.legacy_line5_arity` | count | int | subset(records, `jtemp=2`) | conditional | furrow | `3` or `4` in compatibility policy | `legacy_line5_arity` |
| derived `legacy_ordering_warning_emitted` | legacy warning-only ordering branch | `irrigation.fixeddate.records[*].legacy_ordering_warning_emitted` | flag | bool | subset(records) | conditional | all | compatibility-only branch marker | `legacy_ordering_warning_emitted` |
| derived `iryr_interpretation_mode` | unresolved `iryr` semantics authority conflict | `irrigation.fixeddate.meta.iryr_interpretation_mode` | enum | string | 1 | yes | all | values: `calendar_year`, `simulation_relative_year`, `unresolved_requires_runtime_policy` | `iryr_interpretation_mode` |
| derived `schedule_termination_flag` | `irday==0` | `irrigation.fixeddate.records[*].schedule_termination_flag` | flag | bool | n_records | yes | all | derived sentinel marker | `schedule_termination_flag` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `datver` | `header.datver` | `irrigation.fixeddate.version` | `input::sidecar::irrigation_fixeddate` | init | immutable | compatibility policy gates | `G-FDIR-001`, `G-FDIR-012` |
| `itemp` | `header.itemp` | `irrigation.fixeddate.ofe_count` | `input::sidecar::irrigation_fixeddate` | init | immutable | run-level OFE closure checks | `G-FDIR-002`, `G-FDIR-009` |
| `jtemp` | `header.jtemp` | `irrigation.fixeddate.system_flag` | `input::sidecar::irrigation_fixeddate` | init | immutable | sprinkler/furrow branch selection | `G-FDIR-002`, `G-FDIR-010`, `G-FDIR-012` |
| `ktemp` | `header.ktemp` | `irrigation.fixeddate.schedule_flag` | `input::sidecar::irrigation_fixeddate` | init | immutable | fixed-date schedule mode checks | `G-FDIR-002`, `G-FDIR-010` |
| `ofeflg` | `records[*].ofeflg` | `irrigation.fixeddate.records[*].ofe_id` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | OFE-specific event dispatch | `G-FDIR-003`, `G-FDIR-004` |
| `irday` | `records[*].irday` | `irrigation.fixeddate.records[*].event_doy` | `input::sidecar::irrigation_fixeddate` | init,event,daily | immutable | event activation timing | `G-FDIR-003`, `G-FDIR-005` |
| `iryr` | `records[*].iryr` | `irrigation.fixeddate.records[*].event_year` | `input::sidecar::irrigation_fixeddate` | init,event,daily | immutable | event activation timing | `G-FDIR-003`, `G-FDIR-005` |
| `irint` | `events[*].sprinkler.irint` | `irrigation.fixeddate.events[*].sprinkler.rate_m_per_s` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | sprinkler irrigation application | `G-FDIR-006` |
| `irdept` | `events[*].sprinkler.irdept` | `irrigation.fixeddate.events[*].sprinkler.depth_m` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | sprinkler irrigation application | `G-FDIR-006` |
| `nozzle` | `events[*].sprinkler.nozzle` | `irrigation.fixeddate.events[*].sprinkler.nozzle_factor` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | sprinkler nozzle adjustment | `G-FDIR-006`, `G-FDIR-012` |
| `surges` | `events[*].furrow.surges` | `irrigation.fixeddate.events[*].furrow.surge_count` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | furrow surge-branch arity | `G-FDIR-007` |
| `qspply` | `events[*].furrow.rows[*].qspply` | `irrigation.fixeddate.events[*].furrow.surges[*].supply_rate_m3_per_s` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | furrow supply schedule | `G-FDIR-008` |
| `tstart` | `events[*].furrow.rows[*].tstart` | `irrigation.fixeddate.events[*].furrow.surges[*].start_s` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | furrow surge timing | `G-FDIR-008` |
| `tend` | `events[*].furrow.rows[*].tend` | `irrigation.fixeddate.events[*].furrow.surges[*].end_s` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | furrow surge timing | `G-FDIR-008` |
| `tdepl` | `events[*].furrow.rows[*].tdepl` | `irrigation.fixeddate.events[*].furrow.surges[*].depletion_duration_s` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | furrow depletion phase | `G-FDIR-008`, `G-FDIR-012` |
| derived `initial_dates_complete` | `derived.initial_dates_complete` | `irrigation.fixeddate.initial_dates_complete` | `input::sidecar::irrigation_fixeddate` | init | immutable | initialization closure diagnostics | `G-FDIR-003` |
| derived `event_stream_complete` | `derived.event_stream_complete` | `irrigation.fixeddate.event_stream_complete` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | event stream closure diagnostics | `G-FDIR-004`, `G-FDIR-008` |
| derived `legacy_nozzle_default_applied` | `derived.legacy_nozzle_default[*]` | `irrigation.fixeddate.events[*].sprinkler.legacy_nozzle_default_applied` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | compatibility observability | `G-FDIR-012` |
| derived `legacy_furrow_line5_arity` | `derived.furrow_line5_arity[*]` | `irrigation.fixeddate.events[*].furrow.legacy_line5_arity` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | compatibility observability | `G-FDIR-012` |
| derived `legacy_ordering_warning_emitted` | `derived.legacy_ordering_warning[*]` | `irrigation.fixeddate.records[*].legacy_ordering_warning_emitted` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | compatibility observability | `G-FDIR-003`, `G-FDIR-012` |
| derived `iryr_interpretation_mode` | `derived.iryr_mode` | `irrigation.fixeddate.meta.iryr_interpretation_mode` | `input::sidecar::irrigation_fixeddate` | init | immutable | scheduler year-resolution branch | `G-FDIR-014` |
| derived `schedule_termination_flag` | `derived.schedule_termination[*]` | `irrigation.fixeddate.records[*].schedule_termination_flag` | `input::sidecar::irrigation_fixeddate` | init,event | immutable | fixed-date schedule transition logic | `G-FDIR-011` |

## 5. State Ownership and Mutability

- `input::sidecar::irrigation_fixeddate` owns parsed source rows and normalized fixed-date event stream state.
- Parsed header and event payload rows are immutable after parse success.
- Runtime irrigation scheduler modules own mutable active-event pointers/state.
- Forbidden mutation path: runtime modules mutating canonical parsed event rows (`line3`, `line4`, `line5`) in place.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-FDIR-001` | Derive initialization-date completeness from first `itemp` `line3` records. | parse finalize | `C-FDIR-001` |
| `D-FDIR-002` | Derive schedule termination marker from `irday==0`. | per record parse | `C-FDIR-002` |
| `D-FDIR-003` | Derive nozzle default provenance marker when compat branch injects `nozzle=1.0`. | per sprinkler event parse | `C-FDIR-003` |
| `D-FDIR-004` | Derive furrow line5 arity marker (`3` vs `4`) for compat provenance. | per furrow surge parse | `C-FDIR-004` |
| `D-FDIR-005` | Derive ordering-warning provenance marker when compatibility mode accepts legacy warning-only OFE ordering anomalies. | per initialization/event record parse | `C-FDIR-005` |
| `D-FDIR-006` | Derive `iryr` interpretation mode marker from unresolved authority policy state. | parse finalize | `C-FDIR-006` |

Closure hooks:
- `C-FDIR-001`: first `itemp` line3 rows must exist and be OFE-ordered.
- `C-FDIR-002`: `irday==0` records must be represented as explicit schedule-termination transitions.
- `C-FDIR-003`: compatibility `nozzle` injection must be observable in typed output.
- `C-FDIR-004`: compatibility furrow three-field arity acceptance must be observable in typed output.
- `C-FDIR-005`: ordering-anomaly compatibility acceptance must emit explicit warning provenance (`legacy_ordering_warning_emitted`).
- `C-FDIR-006`: unresolved `iryr` authority conflict must remain explicit via runtime interpretation marker; no implicit assumption.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `FDIR-E-000` | io | missing/unopenable fixed-date sidecar when required by run option |
| `FDIR-E-001` | syntax | token parse failure in required numeric fields |
| `FDIR-E-002` | syntax | line arity mismatch for selected branch |
| `FDIR-E-003` | semantic | unsupported datver policy result |
| `FDIR-E-004` | semantic | invalid header domain (`itemp<=0`, `jtemp` not in `{1,2}`, `ktemp!=2`) |
| `FDIR-E-005` | semantic | invalid field ranges/domains (`surges`, time/rate/date domains) |
| `FDIR-E-006` | cross-file | run-level coupling mismatch (`itemp`, `jtemp`, `ktemp` vs run context) |
| `FDIR-E-007` | cross-file | OFE identifier/order mismatch in initialization/event stream |
| `FDIR-E-008` | runtime-guard | event-stream closure failure post-parse |
| `FDIR-E-009` | cross-file | furrow fixed-date disallowed for contour/non-cropland in strict mode |
| `FDIR-E-010` | cross-file | strict ordering violation in initialization/event OFE sequence |
| `FDIR-W-001` | compat-warning | legacy no-datver branch accepted |
| `FDIR-W-002` | compat-warning | legacy explicit datver accepted (`94.21..95.7` sprinkler, `91.5..95.7` furrow) |
| `FDIR-W-003` | compat-warning | legacy missing-`nozzle` branch accepted with `nozzle=1.0` |
| `FDIR-W-004` | compat-warning | legacy furrow line5 three-field arity accepted (`tdepl` omitted) |
| `FDIR-W-005` | compat-warning | furrow fixed-date disabled for contour/non-cropland compatibility branch |
| `FDIR-W-006` | compat-warning | legacy warning-only OFE ordering anomaly accepted in compatibility mode |

No silent fallback/default masking is permitted in strict mode.

## 8. Cross-File Consistency Constraints

1. `itemp` must match run-level hillslope OFE count authority (`jstruc`/`nplane`). `[DIRECT][E-WF-FDIR-01]`
2. `jtemp` must match run-selected irrigation system flag (`irsyst`). `[DIRECT][E-WF-FDIR-01]`
3. `ktemp` must match fixed-date schedule mode (`2`) for this surface. `[DIRECT][E-WF-FDIR-01]`
4. Initialization/event OFE ordering follows mode-complete policy: strict rejects ordering anomalies (`FDIR-E-010`), compatibility may accept only with explicit warning/provenance (`FDIR-W-006`). `[DIRECT][E-SPEC-FDIR-01]`, `[DIRECT][E-WF-FDIR-01]`
5. Furrow mode under contour/non-cropland context follows explicit mode policy: strict emits `FDIR-E-009`; compatibility disables fixed-date furrow events with `FDIR-W-005`. `[DIRECT][E-WF-FDIR-01]`, `[INFERENCE][E-SPEC-FDIR-01]`
6. `iryr` interpretation remains unresolved and must be exported as explicit runtime policy marker (`iryr_interpretation_mode`) until authority conflict is dispositioned. `[DIRECT][E-SPEC-FDIR-01]`, `[DIRECT][E-WF-FDIR-01]`

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `datver,itemp,jtemp,ktemp` | `irrigation.fixeddate.version/meta` | `openwepp.boundary.parser.irrigation_fixeddate.v1.header` | `{datver,itemp,jtemp,ktemp}` + aliases `fixeddate_irrigation.*` | includes strict/compat mode annotations |
| `line3` symbols (`ofeflg,irday,iryr`) | `irrigation.fixeddate.records[*]` | `openwepp.boundary.scheduler.fixeddate_irrigation.v1.records` | `{ofeflg,irday,iryr,schedule_termination_flag}` + aliases `event.*` | includes `iryr_interpretation_mode` in scheduler metadata |
| sprinkler symbols (`irint,irdept,nozzle`) | `irrigation.fixeddate.events[*].sprinkler` | `openwepp.boundary.irrigation.sprinkler_events.v1` | `{irint,irdept,nozzle,legacy_nozzle_default_applied}` + aliases | required for kernel stimulation without full run |
| furrow symbols (`surges,qspply,tstart,tend,tdepl`) | `irrigation.fixeddate.events[*].furrow` | `openwepp.boundary.irrigation.furrow_events.v1` | `{surges,qspply,tstart,tend,tdepl,legacy_line5_arity}` + aliases | explicit strict/compat arity provenance |
| derived diagnostics (`initial_dates_complete,event_stream_complete,legacy_ordering_warning_emitted`) | `irrigation.fixeddate.derived` | `openwepp.boundary.observability.parser_warnings.v1` | `{FDIR-W-001..FDIR-W-006}` with per-record flags | explicit warning bus for parser observability subsystem |

## 10. Compatibility Policy

- Strict mode:
  - requires explicit datver header with `datver=95.7`;
  - rejects no-datver probe branch;
  - rejects legacy sprinkler two-field line4 rows (missing `nozzle`);
  - requires furrow line5 canonical arity (`qspply,tstart,tend,tdepl`);
  - rejects ordering anomalies in initialization/event OFE sequences (`FDIR-E-010`);
  - rejects furrow fixed-date for contour/non-cropland contexts (`FDIR-E-009`).
- Compatibility mode:
  - may accept no-datver probe branch with `FDIR-W-001`;
  - may accept legacy explicit datver sets with `FDIR-W-002`;
  - may accept missing sprinkler `nozzle` with injected `nozzle=1.0` and `FDIR-W-003`;
  - may accept furrow line5 legacy three-field arity with `FDIR-W-004`;
  - may accept legacy ordering-warning branch with explicit `FDIR-W-006` and provenance flag;
  - disables furrow fixed-date in contour/non-cropland contexts with `FDIR-W-005`.
- Datver-floor enforcement remains provisional pending `FDIR-GAP-004`; strict/compat behavior above is not promoted to final authority until gap closure.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-FDIR-001` | datver policy gate | preamble parse | `FDIR-E-003` |
| `G-FDIR-002` | header domain (`itemp>0`, `jtemp in {1,2}`, `ktemp==2`) | header parse | `FDIR-E-004` |
| `G-FDIR-003` | initialization line3 closure (`itemp` rows, OFE ordering) with strict/compat ordering branch | parse finalize | strict: `FDIR-E-007`/`FDIR-E-010`; compat: `FDIR-W-006` |
| `G-FDIR-004` | event stream branch closure | event-stream parse/finalize | `FDIR-E-002`/`FDIR-E-008` |
| `G-FDIR-005` | date/year domain validity | record parse | `FDIR-E-005` |
| `G-FDIR-006` | sprinkler row field domain/arity | sprinkler event parse | `FDIR-E-002`/`FDIR-E-005` |
| `G-FDIR-007` | furrow `surges` range (`1..20`) | furrow line4 parse | `FDIR-E-005` |
| `G-FDIR-008` | furrow line5 field domain/arity | furrow line5 parse | `FDIR-E-002`/`FDIR-E-005` |
| `G-FDIR-009` | cross-file OFE-count closure | cross-surface validator | `FDIR-E-006` |
| `G-FDIR-010` | run-option/system/schedule coupling closure | cross-surface validator | `FDIR-E-006` |
| `G-FDIR-011` | `irday==0` schedule termination observability | parse finalize | `FDIR-E-008` |
| `G-FDIR-012` | compatibility branch warning emission and provenance fields | policy gate | `FDIR-W-001`/`FDIR-W-002`/`FDIR-W-003`/`FDIR-W-004`/`FDIR-W-005` |
| `G-FDIR-013` | contour/non-cropland furrow disallow policy | cross-file validator + policy gate | strict: `FDIR-E-009`; compat: `FDIR-W-005` |
| `G-FDIR-014` | unresolved `iryr` interpretation governance marker export | parse finalize | unresolved marker required; missing marker `FDIR-E-008` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`datver`, `itemp`, `jtemp`, `ktemp`, `ofeflg`, `irday`, `iryr`,
`irint`, `irdept`, `nozzle`, `surges`, `qspply`, `tstart`, `tend`, `tdepl`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `FDIR-GAP-001` | Usersum furrow line5 includes `tdepl`, while observed legacy furrow read path consumes only `qspply,tstart,tend`; canonical strict-vs-compat policy is adopted here but requires fixture verification. | `[DIRECT][E-SPEC-FDIR-01]`, `[DIRECT][E-WF-FDIR-01]` | `HOLD` |
| `FDIR-GAP-002` | `iryr` semantics conflict (calendar-year vs simulation-relative-year wording) needs explicit architecture-level runtime interpretation note. | `[DIRECT][E-SPEC-FDIR-01]`, `[DIRECT][E-WF-FDIR-01]` | `HOLD` |
| `FDIR-GAP-003` | Legacy ordering warning-only branches exist; strict reject + compat warning policy requires fixture-backed migration evidence. | `[DIRECT][E-SPEC-FDIR-01]`, `[DIRECT][E-WF-FDIR-01]` | `HOLD` |
| `FDIR-GAP-004` | Datver-floor authority conflict remains unresolved because fixed-date `verchk` enforcement is commented in legacy source while constants remain declared. | `[DIRECT][E-SPEC-FDIR-01]`, `[DIRECT][E-WF-FDIR-01]` | `HOLD` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-21` | `0.1.0` | Initial parser-contract draft authored for INFILE09. |
