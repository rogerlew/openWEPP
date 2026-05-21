# INIMPL09 Management Typed Datamodel Decomposition

Static: parser implementation and contract/spec alignment inspected.

## Scope

This decomposition maps canonical `.man` symbols/sections to executable typed parser surfaces implemented in:
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs`

## Section-to-Type Mapping

| Canonical section | Canonical symbols | Typed surface |
| --- | --- | --- |
| Information | `datver`, `nofe/nchan`, total years | `ManagementParseOutput.datver`, `topology_count`, `declared_total_years` |
| Plant | `ncrop`, `crname`, `iplant`, cropland parameter rows | `ManagementScenarioRegistries.plants: Vec<PlantScenario>` |
| Operation | `nop`, `opname`, `iop`, `pcode`, operation effect rows | `ManagementScenarioRegistries.operations: Vec<OperationScenario>` |
| Initial | `nini`, `oname`, `lanuse`, `iresd`, `imngmt` | `ManagementScenarioRegistries.initials: Vec<InitialScenario>` |
| Surface Effects | `nseq`, `sname`, `ntill`, `mdate`, `op`, `tildep`, `typtil` | `ManagementScenarioRegistries.surfaces: Vec<SurfaceScenario>` |
| Contour | `ncnt`, `cname`, `icont`, `cntslp`, `rdghgt`, `rowlen`, `rowspc`, `contours_perm` | `ManagementScenarioRegistries.contours: Vec<ContourScenario>` |
| Drainage | `ndrain`, `dname`, `dcont`, `ddrain`, `drainc`, `drdiam`, `sdrain` | `ManagementScenarioRegistries.drains: Vec<DrainScenario>` |
| Yearly | `nscen`, `mname`, `iscen`, `itype`, `tilseq`, `conset`, `drset`, `imngmt`, branch-specific fields | `ManagementScenarioRegistries.yearlies: Vec<YearlyScenario>` |
| Management | `man` header, `nofes`, `ofeindx`, `nrots`, `nyears`, `nycrop`, `manindx` | `ManagementScenarioRegistries.management_meta` + `ManagementSchedule` |

## Alias Continuity

Canonical WEPP symbols are preserved in parser field naming where possible (`itype`, `tilseq`, `conset`, `drset`, `ofeindx`, `manindx`, `nrots`, `nyears`, `nycrop`). openWEPP runtime aliases remain boundary-level metadata and do not replace canonical contract naming.

## Executable Invariant Linkage

| Guard | Executable enforcement |
| --- | --- |
| `G-MAN-001` | datver allowlist enforcement (`95.7`, `98.4`, `2016.3`, `2017.1`) |
| `G-MAN-002` | positive `nofe_or_nchan` + management `nofes` closure |
| `G-MAN-003` | non-negative section counts (`ncrop`, `nop`, `nini`, `nseq`, `ncnt`, `ndrain`, `nscen`) |
| `G-MAN-004` | section-order parse and record-arity checks |
| `G-MAN-005` | cross-section reference closure (`iresd`, `op`, `itype`, `tilseq`, `conset`, `drset`) |
| `G-MAN-006` | schedule loop expansion closure (`nrots * nyears * nofe`) |
| `G-MAN-007` | declared total-year closure vs derived years |
| `G-MAN-008` | julian-day domain checks with explicit sentinel policy |

## Unsupported Policy Encoding

openWEPP does not execute rangeland `.man` behavior. Any `landuse=2` path encountered in parsed sections is rejected with typed unsupported behavior (`MAN-E-004`) rather than implicit partial support.
