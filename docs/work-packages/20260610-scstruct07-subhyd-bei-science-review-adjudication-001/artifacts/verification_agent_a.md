# Verification Agent A

Evidence: Ran
Date: 2026-06-11

## Commands

| Command | Result | Notes |
|---|---|---|
| `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | pass | 22 rows fully consolidated. |
| `python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` | pass | Strict exit `0`. |
| custom row-count guard | pass | 22 mapped rows, 0 deferred rows, 0 `none`/`none` rows. |
| `git diff --check` | pass | No whitespace errors. |

## Result

Binding exposure verification supports `executed-map-in-core` disposition.
