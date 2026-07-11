# Coverage Closure

Ran: science tier applies because the module admits and mutates kernel state and
flux writeback surfaces. Production source remained byte-identical throughout;
no decomposition was needed after characterization.

| Metric | Before | After | Science floor |
|---|---:|---:|---:|
| Lines | `77/302` (`25.497%`) | `297/302` (`98.344%`) | `90%` |
| Regions | not captured in fresh batch summary | `397/407` (`97.543%`) | `90%` |

Every logical source function clears the `75%` region floor. The lowest primary
functions are logical and indexed evaluation at `33/36` (`91.667%`); apply
functions are `40/41` and `96/98`, and all helper/error functions exceed those
floors.

Obligation-to-test binding:

| Family | Binding |
|---|---|
| A — nominal | Logical and indexed finite payloads accept with exact `WRITEBACK_ACCEPT_MESSAGE_ID`; accepted application emits exact apply ID and values. |
| B — boundaries | Indexed accepted values sit exactly on inclusive min/max; tests cover valid range, below-min, above-max, and invalid bound ordering. |
| C — state/order | Tests distinguish state/flux lanes, unsorted inputs, sorted application order, logical/indexed mirrors, and resolution-before-mutation. |
| D — domain rejects | Range/lower/upper/invalid-bounds cases assert ordered `INV-WRITEBACK-002..004`, violation kinds, and message IDs. |
| E — missing dependency | A late unknown registry ID returns typed `SymbolRegistryError::UnknownSymbolId` after a known ID resolves. |
| F — non-finite | Bounded NaN, positive infinity, and negative infinity assert `INV-WRITEBACK-001`, mixed range violation, and non-finite status priority. |
| G — conservation/continuity | Residual calculation is not applicable. Writeback continuity is separately bound by exact values across indexed authority and logical compatibility surfaces. |
| H — fail-closed | Non-accept decisions and late ID-resolution failure reject; the latter proves no partial mutation on all four indexed/logical state/flux surfaces. |
| Output/mutation | Direct surface-value assertions, exact applied symbol order, and exact status IDs bind mutation results; no readiness/cutover claim is made. |

No exclusion annotation or denominator shrink was introduced.
