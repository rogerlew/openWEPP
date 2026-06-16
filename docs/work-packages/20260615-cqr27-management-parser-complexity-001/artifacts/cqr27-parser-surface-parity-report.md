# CQR27 Parser Surface Parity Report

Status: complete.

Static: parser surface parity holds.

Preserved public imports used by downstream code:

- `parse_management_from_path`
- `parse_management_from_str`
- `ManagementParseError`
- `ParseMode`
- `YearlyAnnualExtension`
- `YearlyAnnualFallowData`
- `YearlyCroplandBranch`
- `YearlyScenarioData`

Static: no public `pub` item was added, removed, renamed, or retyped for this
package. New helpers are private to `management.rs`.

Static: parser diagnostics remain stable for the characterized CQR27 branches:

| Condition | Error variant | Error ID |
| --- | --- | --- |
| legacy datver with annual `resmgt == 7` | `InvalidOptionDomain` | `MAN-E-004` |
| annual cut count `0` | `InvalidCount` | `MAN-E-005` |
| annual cut entry with fewer than two tokens | `RecordArityError` | `MAN-E-002` |

Ran: focused parser contract tests passed after the refactor.
