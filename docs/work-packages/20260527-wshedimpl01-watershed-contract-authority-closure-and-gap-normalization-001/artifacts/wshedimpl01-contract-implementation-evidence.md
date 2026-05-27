# WSHEDIMPL01 Contract Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- Canonical contract metadata updates:
  - `SC-ROUTE-001`: `contract_version 13 -> 14`, `last_reviewed -> 2026-05-27`.
  - `SC-IMPOUND-001`: `contract_version 5 -> 6`, `last_reviewed -> 2026-05-27`.
  - `SC-SED-001`: `contract_version 13 -> 14`, `last_reviewed -> 2026-05-27`.
  - `SC-SYSTEM-001`: `contract_version 27 -> 28`, `last_reviewed -> 2026-05-27`.
- `SC-ROUTE-001` authority-lineage closure:
  - Added `REF-ROUTE-CHRQIN-WAVE` for explicit `chrqin.for` migration anchor.
  - Added `GAP-ROUTE-008` (WS11 runtime branch migration incomplete).
  - Added `GAP-ROUTE-009` (channel sediment routing migration incomplete).
- `SC-IMPOUND-001` WS12 gap normalization:
  - Added `GAP-IMPOUND-005` (RK4/adaptive/regime-transition runtime closure open).
  - Added `GAP-IMPOUND-006` (parser-authoritative coefficient projection closure open).
- `SC-SED-001` cross-domain dependency normalization:
  - Added `GAP-SED-006` for unresolved companion watershed channel-sediment closure.
- `SC-SYSTEM-001` system-blocking normalization:
  - Added `GAP-SYSTEM-005` (missing end-to-end WS11/WS12 fixture vectors).
  - Added `GAP-SYSTEM-006` (`OWSOUT-E-004` publication blocker).
  - Added `GAP-SYSTEM-007` (WS12 seam projection dependency).
  - Added `GAP-SYSTEM-008` (channel-sediment integration dependency).
- Registry synchronization:
  - `docs/specifications/science-contracts/index.md` updated for `Last updated`,
    per-contract `last_reviewed`, and notes for all four target contracts.

## Ran
- Verification command set:
  - `rg -n "contract_version:|last_reviewed:|GAP-ROUTE-008|GAP-ROUTE-009|REF-ROUTE-CHRQIN-WAVE|WSHEDIMPL01 amendment" SC-ROUTE-001.md`
  - `rg -n "contract_version:|last_reviewed:|GAP-IMPOUND-005|GAP-IMPOUND-006|WSHEDIMPL01 amendment" SC-IMPOUND-001.md`
  - `rg -n "contract_version:|last_reviewed:|GAP-SED-006|WSHEDIMPL01 amendment" SC-SED-001.md`
  - `rg -n "contract_version:|last_reviewed:|GAP-SYSTEM-005|GAP-SYSTEM-006|GAP-SYSTEM-007|GAP-SYSTEM-008|WSHEDIMPL01 amendment" SC-SYSTEM-001.md`
  - `rg -n "Last updated:|SC-ROUTE-001|SC-IMPOUND-001|SC-SED-001|SC-SYSTEM-001" docs/specifications/science-contracts/index.md`
