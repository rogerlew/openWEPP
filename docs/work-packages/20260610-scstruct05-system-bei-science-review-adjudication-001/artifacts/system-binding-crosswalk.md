# SCSTRUCT05 System Binding Crosswalk

Evidence: Static
Date: 2026-06-10

## Conservation

- Binding IDs removed: none.
- Binding IDs added: none.
- Binding IDs weakened: none.
- Kernel/runtime files edited: none.
- Comparator re-tiering: none.
- Narrative relocated: three historical HPHYS profile-lineage sections.
- Relocated rows map to existing binding IDs: `INV-SYSTEM-027`.

## Relocation Crosswalk

| Relocated entry | New sidecar location | Conserved binding ID |
|---|---|---|
| `HPHYS0202-WB13-PROFILE-FC-WP-PUBLICATION-LINEAGE-ADDENDUM-HISTORICAL` | `docs/specifications/science-contracts/contracts/provenance/SC-SYSTEM-001-provenance.md` | `INV-SYSTEM-027` |
| `HPHYS0205-CORRECTED-LAYER-PROJECTION-ADDENDUM-HISTORICAL` | `docs/specifications/science-contracts/contracts/provenance/SC-SYSTEM-001-provenance.md` | `INV-SYSTEM-027` |
| `HPHYS0206-NORMALIZED-LAYER-MAPPING-AND-FAIL-CLOSED-ADDENDUM-HISTORICAL` | `docs/specifications/science-contracts/contracts/provenance/SC-SYSTEM-001-provenance.md` | `INV-SYSTEM-027` |

## Retained Core Authority

Rows resolved as `map-in-core` remain in `SC-SYSTEM-001` because they carry active
guard, vector, schema, or integration narrative that should not be relocated
until exact binding exposure is complete.

## Remaining Work

Eleven rows remain deferred as narrower HOLDs and are listed in
`followon-queue.md`. They are not sidecar-eligible.
