# SCSTRUCT05 Follow-On Queue

Evidence: Static
Date: 2026-06-10

## Remaining Deferral Count

`tools/check_sc_binding_exposure.py` reports 11 `science-review-follow-on` rows.
These are narrower HOLDs, not unmapped generic deferrals.

## Queue

| Owner / gate | Entry | Required action |
|---|---|---|
| `SCSTRUCT05-ARCH22-BEI-PROMOTION` | `ARCH22-TYPED-PRODUCTION-SURFACE-ADDENDUM` | Promote/map typed production-surface accessor/signature authority. |
| `SCSTRUCT05-CROSSDOMAIN-BEI-PROMOTION` | `EROD12-CROSS-DOMAIN-OWNERSHIP-AND-GUARD-CLOSURE-ADDENDUM` | Map cross-domain producer/consumer guard ownership to exact SYSTEM/SED/HYDRAULICS/ROUTE IDs. |
| `SCSTRUCT05-HPHYS0203-BEI-PROMOTION` | `HPHYS0203-WB13-ROBUSTNESS-GOVERNANCE-ADDENDUM` | Promote/map robustness vectors, including subsurface-loss families. |
| `SCSTRUCT05-HPHYS0208-BEI-PROMOTION` | `HPHYS0208-COUPLED-WB13-PUBLICATION-LINEAGE-ADDENDUM` | Promote/map coupled WB13 lineage across profile, `Dp`, `latqcc`, soil-water, and threshold seeds. |
| `SCSTRUCT05-WB19-THRESHOLD-BEI-PROMOTION` | `HPHYS0218-WB19-DRFC-THRESHOLD-GOVERNANCE-ADDENDUM` | Promote/map WB19 `drfc` threshold and `coca` guard authority. |
| `SCSTRUCT05-WB19-SATDEP-BEI-PROMOTION` | `HPHYS0221-WB19-COUPLED-SATURATED-DEPTH-GOVERNANCE-ADDENDUM` | Promote/map WB19 `solwpv`, saturated-depth, and lateral writeback coupling authority. |
| `SCSTRUCT05-MOFE04-BEI-PROMOTION` | `MOFE04-MULTI-OFE-WB13-WAT-PUBLICATION-BOUNDARY-CARRY-ADDENDUM` | Promote/map MOFE row identity, contributor cardinality, and area-policy authority. |
| `SCSTRUCT05-MOFE05-BEI-PROMOTION` | `MOFE05-WATERSHED-CONTRIBUTOR-METADATA-INTAKE-VALIDATION-ADDENDUM` | Promote/map contributor metadata shape, consistency, and test-vector authority. |
| `SCSTRUCT05-EROD13-BEI-PROMOTION` | `EROD13-WAVE-1-ACTIVE-BOUNDARY-CARRY-ADDENDUM` | Promote/map Wave-1 hydrology-to-erosion boundary-carry authority. |
| `SCSTRUCT05-EROD14-BEI-PROMOTION` | `EROD14-WAVE-2-ACTIVE-BOUNDARY-CARRY-ADDENDUM` | Promote/map Wave-2 sediment enrichment/class-conservation boundary-carry authority. |
| `SCSTRUCT05-EROD15-BEI-PROMOTION` | `EROD15-WAVE-3-HBP-BOUNDARY-CARRY-ADDENDUM` | Promote/map Wave-3 HBP routing-boundary payload authority. |
