# Verification Agent A

Evidence: Ran
Date: 2026-06-11

## Commands

| Command | Result | Notes |
|---|---|---|
| `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | pass-deferred | 22 rows; 15 `science-review-follow-on` rows; exit `0`. |
| `python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | pass-deferred/strict-fail | Reports `PASS-DEFERRED`; strict exit `1` because deferred rows are intentionally not consolidated. |
| custom row/count check | pass | 22 addenda, 22 BEI rows, 7 mapped rows, 15 deferred rows, 0 `none`/`none` gate flips. |

## Result

No structural verification blocker remains. Strict-mode nonzero exit is the
expected not-consolidated signal for deferred rows.
