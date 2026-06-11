# SCSTRUCT05 System Row Adjudication Ledger

Evidence: Static
Date: 2026-06-10
Target: `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`

## Summary

SCSTRUCT05 adjudicated all 27 SCSTRUCT04-routed BEI rows. Sixteen rows resolved
to existing binding IDs or historical relocation. Eleven rows remain in
`science-review-follow-on` as narrower HOLDs with owner/gate names.

| Outcome | Count |
|---|---:|
| `maps-to-existing-INV`, retained in core | 13 |
| `historical-or-superseded`, relocated to sidecar | 3 |
| narrower HOLD, retained in core | 11 |
| promoted new `INV-*` / `OBL-*` | 0 |

## Resolved Rows

| Entry | Outcome | Binding IDs | Authority / rationale |
|---|---|---|---|
| `WS11-CHANNEL-ROUTING-PHYSICS-EQUIVALENCE-INTEGRATION-ADDENDUM` | map-in-core | `INV-SYSTEM-001`, `INV-SYSTEM-005`, `INV-SYSTEM-006` | Pass-file completeness, hydrograph merge/routed gating, and outlet branch identity cover the row; detailed guard/vector text remains core. |
| `WS12-IMPOUNDMENT-PHYSICS-EQUIVALENCE-INTEGRATION-ADDENDUM` | map-in-core | `INV-SYSTEM-007`, `INV-SYSTEM-008` | Impoundment continuity/adaptive-step and outflow aggregation cover the row; detailed vectors remain core. |
| `SIMIMPL03-PRODUCTION-RUNNER-AND-PUBLICATION-PROVENANCE-ADDENDUM` | map-in-core | `INV-SYSTEM-018..021` | Runner execution ownership, mode propagation, simulation-owned surfaces, and selective consolidated intake are existing system invariants. |
| `SIMIMPL14-CONTINUOUS-REPLAY-SPAN-AND-KEY-DOMAIN-ADDENDUM` | map-in-core | `INV-SYSTEM-022` | Continuous replay span, row closure, monotonic key progression, and simulation-year key semantics are existing authority. |
| `SIMIMPL15-REPLAY-COMPARATOR-TOOLING-ALIGNMENT-ADDENDUM` | map-in-core | `INV-SYSTEM-023`, `INV-SYSTEM-024` | Strict/parquet policy, candidate-source classification, alias continuity, and row-width diagnostics are existing authority. |
| `SIMIMPL16-REPLAY-CONTRACT-DERIVED-TEST-COVERAGE-CLOSURE-ADDENDUM` | map-in-core | `INV-SYSTEM-025` | Replay contract-derived closure-test coverage is existing authority. |
| `SIMIMPL18-BASELINE-YEAR-POLICY-AND-PRECIPITATION-SPAN-CLOSURE-ADDENDUM` | map-in-core | `INV-SYSTEM-026` | Baseline-year policy and full-span precipitation parity are existing authority. |
| `SIMIMPL21-WB13-ET-SOIL-WATER-PUBLICATION-LINEAGE-ADDENDUM` | map-in-core | `INV-SYSTEM-027` | WB13 ET, soil-water, and profile-capacity simulation-owned lineage is existing authority. |
| `HPHYS0202-WB13-PROFILE-FC-WP-PUBLICATION-LINEAGE-ADDENDUM-HISTORICAL` | historical relocated | `INV-SYSTEM-027` | Superseded by retained HPHYS0207 and live WB13 profile publication-lineage invariant. |
| `HPHYS0205-CORRECTED-LAYER-PROJECTION-ADDENDUM-HISTORICAL` | historical relocated | `INV-SYSTEM-027` | Superseded by retained HPHYS0207 and live WB13 profile publication-lineage invariant. |
| `HPHYS0206-NORMALIZED-LAYER-MAPPING-AND-FAIL-CLOSED-ADDENDUM-HISTORICAL` | historical relocated | `INV-SYSTEM-027` | Superseded by retained HPHYS0207 and live WB13 profile publication-lineage invariant. |
| `HPHYS0207-NORMALIZED-PROFILE-FC-WP-DEPTH-AUTHORITY-ADDENDUM` | map-in-core | `INV-SYSTEM-027` | Normalized-profile FC/WP and ordering continuity are exposed by `INV-SYSTEM-027`; detailed depth authority remains core. |
| `HPHYS0216D-PROFILEFC-LAYER-TAIL-BOUNDARY-AUTHORITY-ADDENDUM` | map-in-core | `INV-SYSTEM-027` | ProfileFC layer+tail publication authority maps to `INV-SYSTEM-027`; detailed tail rule remains core. |
| `HPHYS0209-PROFILEWP-NEAR-CLOSED-PUBLICATION-ADJUDICATION-ADDENDUM` | map-in-core | `INV-SYSTEM-027` | ProfileWP publication lineage and profile geometry non-regression map to `INV-SYSTEM-027`; adjudication caveats remain core. |
| `HPHYS0255-MOFE-STORAGE-LINEAGE-PUBLICATION-ADDENDUM` | map-in-core | `INV-SYSTEM-029` | MOFE storage-lineage policy and area/storage separation are existing authority. |
| `HPHYS0241-MOFE-HOURLY-CARRY-METADATA-ADDENDUM` | map-in-core | `INV-SYSTEM-028` | Active 24-slot carry metadata and watershed intake rejection posture are existing authority. |

## Narrower HOLD Rows

| Entry | Owner / next gate |
|---|---|
| `ARCH22-TYPED-PRODUCTION-SURFACE-ADDENDUM` | `SCSTRUCT05-ARCH22-BEI-PROMOTION` |
| `EROD12-CROSS-DOMAIN-OWNERSHIP-AND-GUARD-CLOSURE-ADDENDUM` | `SCSTRUCT05-CROSSDOMAIN-BEI-PROMOTION` |
| `HPHYS0203-WB13-ROBUSTNESS-GOVERNANCE-ADDENDUM` | `SCSTRUCT05-HPHYS0203-BEI-PROMOTION` |
| `HPHYS0208-COUPLED-WB13-PUBLICATION-LINEAGE-ADDENDUM` | `SCSTRUCT05-HPHYS0208-BEI-PROMOTION` |
| `HPHYS0218-WB19-DRFC-THRESHOLD-GOVERNANCE-ADDENDUM` | `SCSTRUCT05-WB19-THRESHOLD-BEI-PROMOTION` |
| `HPHYS0221-WB19-COUPLED-SATURATED-DEPTH-GOVERNANCE-ADDENDUM` | `SCSTRUCT05-WB19-SATDEP-BEI-PROMOTION` |
| `MOFE04-MULTI-OFE-WB13-WAT-PUBLICATION-BOUNDARY-CARRY-ADDENDUM` | `SCSTRUCT05-MOFE04-BEI-PROMOTION` |
| `MOFE05-WATERSHED-CONTRIBUTOR-METADATA-INTAKE-VALIDATION-ADDENDUM` | `SCSTRUCT05-MOFE05-BEI-PROMOTION` |
| `EROD13-WAVE-1-ACTIVE-BOUNDARY-CARRY-ADDENDUM` | `SCSTRUCT05-EROD13-BEI-PROMOTION` |
| `EROD14-WAVE-2-ACTIVE-BOUNDARY-CARRY-ADDENDUM` | `SCSTRUCT05-EROD14-BEI-PROMOTION` |
| `EROD15-WAVE-3-HBP-BOUNDARY-CARRY-ADDENDUM` | `SCSTRUCT05-EROD15-BEI-PROMOTION` |
