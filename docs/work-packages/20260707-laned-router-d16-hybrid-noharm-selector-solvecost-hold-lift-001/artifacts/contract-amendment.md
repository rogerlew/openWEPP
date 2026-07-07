# Contract Amendment

Status: COMPLETE. Evidence mode: Static + Ran.

## Touched Authority

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- Contract version: `4 -> 5`

## Amendment Summary

`OPENWEPP_LANED_ACTIVE_IMPLICIT=1` is now contractually a hybrid request, not
an unconditional hybrid execution flag.

`SC-OFEROUTE-002` rev 5 authorizes the deterministic exact-bare-skin no-harm
selector:

- Select hybrid only when the active lane-day cell operands are exact
  bare-skin eligible: no Manning override, no active roughness-element addend,
  and no active vegetation addend.
- Route every non-bare requested lane-day through the plain rev-27 active
  owner, preserving active ownership, no DC01 double feed, D13 routed
  hydrograph publication, and day closure.
- Decide only from run inputs already present at the active lane-day boundary:
  static friction/cover operands and post-growth daily LAI/canhgt.
- Prohibit wall time, host load, measured profile counters, or observed
  mid-run solve iteration counts as selector inputs.
- Publish requested/selected/fallback lane-day counters in the run manifest.

## New / Updated Bindings

- Added `INV-OFEHYB-011`: adaptive no-harm selector determinism.
- Updated `OBL-OFEHYB-P-003`: manifest records selector request provenance and
  requested/selected/fallback counters.
- Added selector-counter aliases to the symbol map.
- Added adaptive no-harm selector test-vector obligation.
- Added `GAP-OFEHYB-003` for selected-cohort timing no-harm and closed it as
  `RESOLVED-NOHARM-SELECTOR` after package evidence.

## Non-Changes

No promotion tolerance is ratified here. No default activation is made. No
non-bare implicit solve-cost authority is closed.

## Contract Gates

- Ran: `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
  - Result: PASS, no findings.
- Ran: `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
  - Result: PASS for this package gate. The tool emitted `PASS-DEFERRED`,
    meaning 4 BEI rows exist with the contract's existing
    `science-review-follow-on` posture and no missing row was detected.
- Ran: focused `ofe_routing` suite, including retained Case-4 hybrid ladder.
  - Result: 103/103 passed.
- Ran: full workspace nextest profile.
  - Result: 1442/1442 passed, 4 skipped.
