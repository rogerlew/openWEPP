# SIMIMPL27 Contract Authority Amendment Log

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL27 completed canonical contract-authority amendments for hourly
  snow/freeze migration boundary scope.
- Amendment summary:

| Contract | Version change | SIMIMPL27 authority closure added |
|---|---|---|
| `SC-SNOWFREEZE-001` | `5 -> 6` | Replaced provisional identity alias map with concrete typed/reserved boundary names; added SIMIMPL27 boundary-closure addendum and downstream contract-derived test requirements; reclassified `GAP-SNOWFREEZE-002/004/005` to migration-scope promotable-with-risk posture. |
| `science-contracts/index.md` | registry note update | Added SIMIMPL27 note documenting boundary/API alias finalization and closure of non-promotable naming ambiguity for migration scope. |

## Ran
- `rg -n "contract_version: 6|SIMIMPL27 Boundary/API Closure|snow.hourly.depth_before_m|frost.runtime_infcap_frz|GAP-SNOWFREEZE-002|GAP-SNOWFREEZE-004|GAP-SNOWFREEZE-005" docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `rg -n "SC-SNOWFREEZE-001.*SIMIMPL27" docs/specifications/science-contracts/index.md`
