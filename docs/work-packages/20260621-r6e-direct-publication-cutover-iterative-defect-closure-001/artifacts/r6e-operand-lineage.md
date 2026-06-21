# R6E Operand Lineage

Evidence mode: Static + Ran.

Status: executed-held.

## Accepted Retained Operand Families

| Family | Source authority | Direct producer | Disposition |
|---|---|---|---|
| run/lane/day identity | parsed runner execution context and `DirectRunIdentity` | direct publication execution | Authoritative for identity. |
| calendar year / Julian day / day index | parsed climate span | `DirectPublicationDayInput.calendar` | Authoritative for calendar. |
| precipitation | parsed climate day projection | `DirectPublicationDayInput.precipitation_m` -> direct normalization/liquid-input spans -> publication row | Authoritative as direct input. |
| effective daily temperature | parsed `tmax/tmin` average | `DirectPublicationDayInput.effective_temperature_c` -> direct normalization inputs | Authoritative as direct input. |
| lane area and upstream area ratio | parsed static OFE lane geometry | direct lane frames | Authoritative for lane geometry. |

## Diagnostic Direct Process Operands

R6E now reaches direct span-produced hydrology operands and HBP byte comparison.
Those operands are not accepted as parity-grade public output authority because
HBP byte identity fails at
`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

## Missing Direct Operand Families

The following families remain blocked for public cutover:

- HBP parity-grade runoff/OFE runoff volumes;
- infiltration, storage, snow/frost, evapotranspiration, interception, and
  profile terms;
- subsurface/lateral/tile terms;
- PASS row volumes and erosion/sediment terms;
- loss JSON run summary values beyond parsed/static fields;
- manifest direct runtime provenance and output checksum authority.

## Rejected Aliases

R6E rejects these aliases for future direct publication closure:

- compatibility `SimulationOwnedWb13Row` values;
- compatibility `HillslopeWritebackSurface` publication symbols;
- `KernelWritebackPayload` values;
- stale logical state retained outside direct phase mutation;
- zero/default skeleton direct frame capture.
