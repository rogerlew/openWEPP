# R6G Review Agent A

Status: complete.

Evidence mode: Static review of the R6G diff and focused gate evidence.

| Severity | Finding | Evidence | Required action | Disposition |
|---|---|---|---|---|
| High | Some direct WAT producer operands were initially defaulted instead of fail-closed. | The review flagged required scalar defaults in direct publication input construction, plus same-pass infiltration/outside-water operands that are not yet sourced from lane-dimensional direct authority. | Required operands must fail closed; optional or branch-conditional operands must be documented by contract; remaining lane-dimensional authority gaps must not be hidden under R6G completion. | Accepted. Required defaults for `solwpv`, `coca`, `rtd`, and `pltol` were replaced with required symbol reads. Optional PMET storage-return/frozen-depth operands remain contract-authorized optional inputs. Same-pass/hourly and lane-dimensional follow-up remains open under the R6G hold. |
| Medium | WAT `wepp_id` publication uses a compatibility-shaped constant. | `DIRECT_WAT_WEPP_ID` matches the current compatibility fixture's WAT output id, not an independently proven multi-OFE canonical id ledger. | Do not claim full direct WAT identity authority until WAT id semantics are proven for non-trivial OFE/lane cases. | Accepted follow-up. Current-fixture parity is recorded; full canonical multi-OFE WAT id authority remains blocked for R6 continuation. |
| Medium | Day input production is day-global and applied to every lane. | `direct_publication_day_inputs` builds one day vector from one execution lane context; direct runtime applies the input to each lane. | Add lane-dimensional direct day-input construction and anti-alias coverage before claiming full R6 publication closure. | Accepted follow-up. R6G fixture is single-lane/single-OFE; multi-lane coverage is required before full cutover. |
| Medium | The initial R6G hold marker predicate was field-set-only. | A marker based only on reduced field names could mask unrelated mechanisms with the same fields. | Make marker firing depend on row-level evidence: first row equal, later rows diverge exactly on the PMET/storage fields. | Accepted and fixed. `r6g_wat_pmet_day_state_carry_gap` now checks row counts, first-row equality, and later-row `Es`/storage divergence before emitting the R6G marker. |

## Verdict

Not approved for `COMPLETE-R6G-DIRECT-WAT-PRODUCER-AUTHORITY`. Approved for
the current `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` disposition
after required-default fixes, mechanism-aware marker hardening, and explicit
follow-up recording for WAT id and lane-dimensional authority.
