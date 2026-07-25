# Evidence Currency Matrix

Evidence class: Ran and Static.

| Evidence state | Disposition | Selection | Recollection |
| --- | --- | --- | --- |
| Exact canonical publication/control/admission; current source and controls | `CURRENT` | Candidate ranking emitted; dual review still required | Forbidden |
| Internally verified historical subject with current identity drift | `STALE` | None | Requires deterministic receipt replay plus explicit CQR directive |
| Missing, malformed, unsafe, digest-corrupt, internally inconsistent, or unsupported topology | `INVALID` | None | Requires deterministic receipt replay plus explicit CQR directive |

The real 11-file fixture passed `CURRENT`; a bound artifact mutation produced
`INVALID`; and an internally valid historical subject produced `STALE`.
Current verification independently enumerated the repository inventory.
