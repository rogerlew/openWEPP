# WEPP Input File Parser Survey

Date: 2026-05-20
Evidence class: `Static` (code inspection and document inspection only; no parser runtime execution in this survey)

## Goal

Survey existing WEPP input-file parser coverage in `wepppy`, `wepppyo3`, and `wepp-forest`, and define how openWEPP owns input-file specifications for architecture-first parser design.

## Source Repositories Surveyed

- `wepppy`: `/workdir/wepppy`
- `wepppyo3`: `/workdir/wepppyo3`
- `wepp-forest` active HEAD (exploratory): `/home/workdir/wepp-forest`
- `wepp-forest` pinned baseline (normative for new provenance/comparator work):
  `/workdir/wepp-forest_260430_baseline` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`)

## Coverage Matrix

| WEPP input surface | Spec source in openWEPP | wepppy coverage | wepppyo3 coverage | wepp-forest coverage | Notes for openWEPP |
|---|---|---|---|---|---|
| Climate `.cli` | `docs/specifications/wepp-input-files/specs/climate-file.spec.md` and `cligenparms.md` | `ClimateFile` parser (`wepppy/climates/cligen/cligen.py:778`) | Rust climate readers used for monthly/scaling transforms (`cli_revision/src/lib.rs:1400`, `:1452`, `:1515`) | Legacy runtime reader in `src/infile.for` (`open(13)` at `:1707`; climate header/state reads at `:1715`, `:1728`, `:1805`, `:1822`, `:1827`) with breakpoint consumption in `src/stmget.for` and `src/idat.for:1` | openWEPP needs a first-class typed climate parser; wepppyo3 coverage is functional but not a full AST-style input consumer. |
| Soil `.sol` | `docs/specifications/wepp-input-files/specs/soil-file.spec.md` | `WeppSoilUtil._parse_sol` (`wepppy/wepp/soils/utils/wepp_soil_util.py:166`) | No direct `.sol` parser entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(11)` at `:1874`; soil version parse at `:1877`; `solwpv` mapping at `:1949`) and datver-branch layer parsing in `src/input.for` (`:467`-`:472`, `:480`, `:542`, `:547`, `:551`, `:556`, `:641`, `:658`) | openWEPP should implement typed `.sol` parsing with datver-aware variants and explicit invariants/guards. |
| Plant/Management `.man` | `docs/specifications/wepp-input-files/specs/plant-file.spec.md` | `Management._parse` (`wepppy/wepp/management/managements.py:2182`) with loader `read_management` (`:2996`) | No direct `.man` parser entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(12)` at `:480`; bulk scenario/section reads across `:527`-`:1329`) with annual crop/tillage sequence consumption in `src/tilage.for` (`read (12,*)` at `:221`, `:230`, `:456`, `:458`) | openWEPP needs a section/scenario parser consistent with WEPP conventions and contract guards. |
| Slope `.slp` | Not yet present in this initial openWEPP spec set (gap) | `SlopeFile` parsing utilities (`wepppy/topo/watershed_abstraction/slope_file.py:73`) | Single-OFE slope parse/segment path (`wepp_interchange/src/mofe.rs:16`, `:206`) | Legacy runtime reader in `src/infile.for` (`open(10)` at `:1653`; datver read at `:1656`) and geometric point reads in `src/input.for` (`:394`, `:397`) with derived profile transforms in `src/profil.for:1` | openWEPP should retain backward compatibility with legacy slope text format; canonical slope specification still needs authoring/import. |
| Watershed structure `.str` | Not yet present in current openWEPP input spec corpus (gap) | Watershed prep emits `pw0.str` (`wepppy/nodb/core/wepp.py:2370`, `:2393`) | No direct `.str` parser/writer entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(17)` at `:380`; version checks at `:383`-`:391`) | openWEPP needs a typed watershed-structure parser/contract surface with explicit topology invariants. |
| Watershed channels `.chn` | Not yet present in current openWEPP input spec corpus (gap) | Watershed prep emits `pw0.chn` (`wepppy/nodb/core/wepp.py:2482`) | No direct `.chn` input parser/writer entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(18)` at `:409`; version checks at `:412`-`:420`) | openWEPP needs a channel-input contract including routing method gates and channel parameter validity constraints. |
| Watershed impoundment `.imp` | Not yet present in current openWEPP input spec corpus (gap) | Watershed prep emits `pw0.imp` (`wepppy/nodb/core/wepp.py:2508`, `:2510`) | No direct `.imp` input parser/writer entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(20)` at `:441`; version checks at `:444`-`:452`) | openWEPP needs an impoundment parser contract, even for `0`-impoundment baseline emission, to lock behavior and extension path. |
| Depletion irrigation sidecar (legacy unit `15`) | Not yet present in current openWEPP input spec corpus (gap) | No direct parser entry point identified in surveyed files | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(15)` at `:2060`; header checks at `:2069`, `:2110`) | openWEPP should define explicit acceptance policy for this sidecar, including whether initially supported or explicitly rejected with typed errors. |
| Fixed-date irrigation sidecar (legacy unit `14`) | Not yet present in current openWEPP input spec corpus (gap) | No direct parser entry point identified in surveyed files | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(14)` at `:2136`; header checks at `:2143`, `:2183`) | openWEPP should define explicit acceptance policy for this sidecar, including whether initially supported or explicitly rejected with typed errors. |
| PMET sidecar `pmetpara.txt` | Not yet present in current openWEPP input spec corpus (gap) | PMET writer path in `wepppy` (`wepppy/wepp/management/pmetpara.py:1`; orchestration in `wepppy/nodb/core/wepp.py:1855`, `:1895`) | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(unit=22,file='pmetpara.txt')` at `:1544`) | openWEPP should define parser/validator contract for PMET sidecar semantics and plant-loop mapping integrity. |
| Snow sidecar `snow.txt` | Not yet present in current openWEPP input spec corpus (gap) | Snow options writer in `wepppy` (`wepppy/nodb/core/wepp.py:320`, `:1965`) | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(22,file='snow.txt')` at `:1566`) | openWEPP should define explicit bounds/default behavior and typed validation errors for snow-sidecar values. |
| Frost sidecar `frost.txt` | Not yet present in current openWEPP input spec corpus (gap) | Frost options writer in `wepppy` (`wepppy/nodb/core/wepp.py:381`, `:1786`) | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/infile.for` (`open(unit=22,file='frost.txt')` at `:1612`) | openWEPP should define explicit bounds/default behavior and typed validation errors for frost-sidecar values. |
| Baseflow sidecar `gwcoeff.txt` | Not yet present in current openWEPP input spec corpus (gap) | Baseflow options writer in `wepppy` (`wepppy/nodb/core/wepp.py:2013`, `:2020`) | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/main.for` (`open(22,file='gwcoeff.txt')` at `:146`, `:476`) | openWEPP should define parse/validation contract for groundwater/baseflow coefficients and defaults when absent. |
| Phosphorus sidecar `phosphorus.txt` | Not yet present in current openWEPP input spec corpus (gap) | Phosphorus options writer in `wepppy` (`wepppy/nodb/core/wepp.py:1945`, `:1949`) | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/main.for` (`open(22,file='phosphorus.txt')` at `:170`, `:499`) | openWEPP should define parse/validation contract and explicit unsupported behavior if phosphorus surface is deferred. |
| Sentinel sidecar `wepp_ui.txt` | Not yet present in current openWEPP input spec corpus (gap) | Sentinel writer path in `wepppy` (`wepppy/nodb/core/wepp.py:1768`, `:1773`) | No direct parser entry point found in surveyed files | Legacy runtime probe in `src/main.for` (`open(unit=22,file='wepp_ui.txt')` at `:188`) | openWEPP should define semantics for this sentinel flag and typed behavior when present/missing. |
| Channel transport sidecar `tc.txt` | Not yet present in current openWEPP input spec corpus (gap) | Writer path in `wepppy` (`wepppy/nodb/core/wepp.py:1810`, `:1811`) | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/wshdrv.f90` (`open(unit=65,file='tc.txt')` at `:295`) | openWEPP should define whether this sidecar is supported in initial watershed parity and how it maps to channel output behavior. |
| Channel critical-shear sidecar `tcr.txt` | Not yet present in current openWEPP input spec corpus (gap) | Writer path in `wepppy` (`wepppy/nodb/core/wepp.py:1815`, `:1816`) | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/wshinp.for` (`open(unit=66,file='tcr.txt')` at `:183`) | openWEPP should define parse/validation bounds and fallback behavior for absent/invalid values. |
| Last-channel water-balance sidecar `lcwb.txt` | Not yet present in current openWEPP input spec corpus (gap) | No direct writer entry point found in surveyed files | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/wshinp.for` (`open(unit=67,file='lcwb.txt')` at `:199`) | openWEPP should define explicit support/defer/unsupported disposition for this optional flag sidecar. |
| Channel contrast input sidecar `chan.inp` | Not yet present in current openWEPP input spec corpus (gap) | Writer path in `wepppy` (`wepppy/nodb/core/wepp.py:2564`, `:2587`) | No direct parser entry point found in surveyed files | Legacy runtime reader in `src/wshinp.for` (`open(24, file='chan.inp')` at `:472`) | openWEPP should define parsing/validation and interaction with channel output/contrast modes. |
| Observability sidecar flags (`wepp_observe.on`, `wepp_observe_frost.on`) and probe target (`wepp_observe_wb05e_target.dat`) | Not part of parser-contract carry-forward (explicitly unsupported as parser sidecars) | No direct writer entry point found in surveyed files | No direct parser entry point found in surveyed files | Optional runtime probes in `src/wepp_observe.for` (`inquire(file='wepp_observe.on')` at `:26`; `inquire(file='wepp_observe_frost.on')` at `:32`) and `src/watbal_process_probe.f90` (`inquire(file='wepp_observe_wb05e_target.dat')` at `:108`) | openWEPP will replace these with a first-class observability subsystem focused on developer ergonomics and kernel stimulation without end-to-end runs. |

## Key Findings

1. `wepp-forest` contains the primary legacy runtime parser semantics for WEPP text inputs, watershed inputs, and operational sidecars, but they are tightly coupled to common blocks and simulation flow.
2. `wepppy` currently provides broad generation/prep coverage for many watershed and sidecar artifacts, but parser authority is uneven across surfaces.
3. `wepppyo3` currently provides targeted Rust parsing/transformation for climate and slope workflows, but not full coverage for soil/management/watershed/sidecar input surfaces.
4. openWEPP currently has owned specifications for climate/soil/plant domains only; watershed and sidecar specs remain contract gaps that must be authored.
5. Legacy text sidecar compatibility remains a required startup strategy, so acceptance/rejection behavior for each sidecar must be explicit in parser contracts (typed parse/validation error or supported surface).
6. A deeper scan confirms additional legacy/runtime sidecars beyond the initial list (`gwcoeff`, `phosphorus`, `wepp_ui`, `tc`, `tcr`, `lcwb`, `chan.inp`, observability flags), which must be explicitly governed.
7. `firedate.txt`, `cancov.txt`, and `simfire.txt` are wepp-forest-revegetation-specific and are explicitly out-of-scope for openWEPP parser sidecar carry-forward.
8. Sidecar surface count is broader than the initial matrix in many operational stacks; a dedicated sidecar-registry contract is required to enumerate and disposition every sidecar surface before parser implementation.

## Ownership Decision (Locked)

Canonical location for openWEPP-owned WEPP input-file specifications:
- `docs/specifications/wepp-input-files/specs/`

Current bootstrapped files:
- `cligenparms.md`
- `climate-file.spec.md`
- `plant-file.spec.md`
- `soil-file.spec.md`

Policy and provenance notes are maintained in:
- `docs/specifications/wepp-input-files/README.md`

## Input Surface Scope (Locked)

openWEPP parser-contract scope includes:

1. Hillslope core text inputs (`.cli`, `.sol`, `.man`, `.slp`).
2. Watershed WEPP text inputs (`.str`, `.chn`, `.imp`).
3. Operational sidecars required by legacy workflows where present
   (including irrigation sidecars, `pmetpara.txt`, `snow.txt`, `frost.txt`,
   `gwcoeff.txt`, `phosphorus.txt`, `wepp_ui.txt`, `tc.txt`, `tcr.txt`,
   `lcwb.txt`, `chan.inp`, and explicit debug sidecar dispositions).

Explicit parser-carry-forward exclusions:
- wepp-forest-revegetation sidecars (`firedate.txt`, `cancov.txt`,
  `simfire.txt`)

Canonical registry for surface completeness:
- `docs/specifications/wepp-input-files/input-surface-registry.md`

## Recommended Next Architecture Step

Define parser contracts before implementation for each input surface:
1. `SC-INFILE-CLIMATE-001`
2. `SC-INFILE-SOIL-001`
3. `SC-INFILE-MANAGEMENT-001`
4. `SC-INFILE-SLOPE-001`
5. `SC-INFILE-WATERSHED-STRUCTURE-001`
6. `SC-INFILE-WATERSHED-CHANNEL-001`
7. `SC-INFILE-WATERSHED-IMPOUNDMENT-001`
8. `SC-INFILE-IRRIGATION-DEPLETION-001`
9. `SC-INFILE-IRRIGATION-FIXEDDATE-001`
10. `SC-INFILE-PMETPARA-001`
11. `SC-INFILE-SNOW-001`
12. `SC-INFILE-FROST-001`
13. `SC-INFILE-GWCOEFF-001`
14. `SC-INFILE-PHOSPHORUS-001`
15. `SC-INFILE-WEPPUI-001`
16. `SC-INFILE-TC-001`
17. `SC-INFILE-TCR-001`
18. `SC-INFILE-LCWB-001`
19. `SC-INFILE-CHANINP-001`
20. `SC-INFILE-SIDECAR-REGISTRY-001`

First-class subsystem work (non-parser-contract track):
1. `OBS01` openWEPP observability subsystem to replace ad-hoc `wepp_observe*`
   flags and support kernel stimulation without end-to-end runs.
   - assessment: `docs/planning/openwepp-observability-subsystem-assessment.md`

Normative parser-contract requirements for data-model and propagation behavior:
- `docs/specifications/wepp-input-files/parser-contract-requirements.md`
- `docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`

Each contract should include:
- canonical legacy variable names (with alias table if openWEPP boundary names differ)
- required invariants and explicit guard behavior
- datver-specific branch semantics where applicable
- backward compatibility expectations for legacy WEPP text inputs
- explicit error taxonomy for malformed records
- a typed field specification table (symbol, alias, units, type, cardinality, datver applicability, default rule)
- a parse-to-simulation propagation map (source field -> parser model -> runtime state surface -> owning module -> phase -> consumers -> guards)
- explicit state ownership/mutability rules to prevent hidden cross-module mutation
