# CQR27 Coverage Closure

Status: complete.

Ran: target-file LCOV improved during the package:

| Metric | Before | After | Result |
| --- | ---: | ---: | --- |
| Line coverage | `749/1114` (`67.24%`) | `816/1147` (`71.14%`) | improved |
| Function coverage | `40/49` (`81.63%`) | `45/54` (`83.33%`) | improved |

Ran: target closure:

| Function | Before CRAP | After CRAP | Result |
| --- | ---: | ---: | --- |
| `parse_yearly_annual_fallow` | `290.7314769280208` | `4.0` | closed |

Ran: helper closure:

| Helper | After CRAP | Result |
| --- | ---: | --- |
| `parse_yearly_annual_fallow_header` | `5.0` | closed |
| `parse_yearly_annual_extension` | `19.045125` | closed |
| `parse_yearly_annual_cut_records` | `6.0` | closed |
| `parse_yearly_annual_cut_entry` | `6.0` | closed |

Static: remaining target-file CRAP rows over `30` are outside the CQR27 target
and were not modified except for incidental line-number stability around the
new helpers.
