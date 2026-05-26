# SIMIMPL31 Contract Authority Amendment Log

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- SIMIMPL31 completed canonical frost routine-authority amendments for
  migration-scope snow/freeze contract governance.
- Amendment summary:

| Contract | Version change | SIMIMPL31 authority closure added |
|---|---|---|
| `SC-SNOWFREEZE-001` | `8 -> 9` | Added SIMIMPL31 frost routine-chain authority addendum (`winter`/`frostN`/`frwatc`/`frzng`/`frznw`/`frsoil`/`getFreezeCond`/`winthd`), added invariants `INV-SNOWFREEZE-012/013`, updated producer/guard/disposition obligations, and re-scoped frost-hourly gap ownership to SIMIMPL32+ follow-on execution. |
| `science-contracts/index.md` | registry note update | Updated `SC-SNOWFREEZE-001` `last_reviewed` to `2026-05-26` and added SIMIMPL31 routine-authority closure note with downstream SIMIMPL32 test ownership. |

## Ran
- `rg -n "contract_version: 9|SIMIMPL31 Frost Routine-Chain Authority|INV-SNOWFREEZE-012|INV-SNOWFREEZE-013|SIMIMPL32 Contract-Derived Test Requirements" docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `rg -n "Last updated: 2026-05-26|SC-SNOWFREEZE-001.*SIMIMPL31" docs/specifications/science-contracts/index.md`
