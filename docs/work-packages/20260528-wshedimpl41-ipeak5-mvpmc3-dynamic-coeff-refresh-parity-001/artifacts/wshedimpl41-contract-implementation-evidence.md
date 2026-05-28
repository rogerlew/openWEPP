# WSHEDIMPL41 Contract Implementation Evidence

Status: complete  
Evidence mode: static  
Date: 2026-05-28

## Static
- Canonical contract amendments completed:
  - `SC-ROUTE-001`:
    - `contract_version` `42 -> 43`,
    - added WSHEDIMPL41 MVPMC3 addendum for `ipeak=5` dynamic-coefficient
      refresh requirements in the current WS10 lane,
    - dispositioned `GAP-ROUTE-011` to `closed`.
  - `SC-SYSTEM-001`:
    - `contract_version` `63 -> 64`,
    - added WS11 integration vector requiring `ipeak=5` dynamic refresh
      behavior (not static `ipeak=4` reuse),
    - dispositioned `GAP-SYSTEM-010` to `closed`.
  - `docs/specifications/science-contracts/index.md`:
    - updated `SC-ROUTE-001` and `SC-SYSTEM-001` summary notes to reflect
      WSHEDIMPL41 closure of the follow-on `ipeak=5` gaps.

## Ran
- not-applicable
