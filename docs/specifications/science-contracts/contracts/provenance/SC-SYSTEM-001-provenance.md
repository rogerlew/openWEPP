# SC-SYSTEM-001 Provenance Sidecar

Status: Active
Last updated: 2026-06-10
Scope: historical and superseded narrative relocated from
`SC-SYSTEM-001` after Binding Exposure Index adjudication.

Sidecar entries are non-binding except through the canonical binding IDs named
in `SC-SYSTEM-001`'s Binding Exposure Index.

## HPHYS0202-WB13-PROFILE-FC-WP-PUBLICATION-LINEAGE-ADDENDUM-HISTORICAL HPHYS0202 WB13 Profile FC/WP Publication-Lineage Addendum (Historical)

- status: historical
- source_package: HPHYS0202 WB13 Profile FC/WP Publication-Lineage Addendum (Historical)
- effective_date: 2026-06-10
- verdict: historical
- superseded_by: HPHYS0207 Normalized-Profile FC/WP Depth-Authority Addendum
- canonical_binding_ids: INV-SYSTEM-027
- migration_target: none
- provenance_anchors: SCSTRUCT05; SC-SYSTEM-001 Binding Exposure Index; REF-SYSTEM-LEGACY-WATBAL; REF-SYSTEM-LEGACY-OUTFIL

System publication authority for `ProfileFCStore` and `ProfileWPStore` was
previously described as simulation-owned layer aggregation from runtime WB13
symbols: `ProfileFCStore = Σ(thetfc_i * dg_i) * 1000` and
`ProfileWPStore = Σ(thetdr_i * dg_i) * 1000` (`mm`). Adapter-projected seed
symbols `wb13_profile_fc_store_mm` and `wb13_profile_wp_store_mm` were diagnostic
carry surfaces rather than authoritative WB13 publication values. Missing,
non-finite, or domain-invalid layer aggregation symbols remained typed WB13
publication failures and were not eligible for projection-side synthetic
reconstruction. This narrative is historical because retained HPHYS0207 and
`INV-SYSTEM-027` now expose live WB13 ET/soil-water/profile publication-lineage
authority.

## HPHYS0205-CORRECTED-LAYER-PROJECTION-ADDENDUM-HISTORICAL HPHYS0205 Corrected-Layer Projection Addendum (Historical)

- status: historical
- source_package: HPHYS0205 Corrected-Layer Projection Addendum (Historical)
- effective_date: 2026-06-10
- verdict: historical
- superseded_by: HPHYS0207 Normalized-Profile FC/WP Depth-Authority Addendum
- canonical_binding_ids: INV-SYSTEM-027
- migration_target: none
- provenance_anchors: SCSTRUCT05; SC-SYSTEM-001 Binding Exposure Index; REF-SYSTEM-LEGACY-WATBAL; REF-SYSTEM-LEGACY-OUTFIL

System-boundary authority for WB13 profile storage previously required
`thetfc_####` and `thetdr_####` publication-consumer symbols to be sourced from
baseline-corrected moisture lineage when correction lineage was available at the
runtime projection boundary. Adapter-projected FC/WP seed symbols were
diagnostic-only carry surfaces. When both corrected-layer aggregates and
diagnostic FC/WP seed surfaces were present, they had to reconcile; disagreement
was a typed boundary violation for HPHYS0205 closure governance. Missing,
non-finite, or domain-invalid corrected-layer symbol projections remained
fail-closed and were not eligible for projection-side surrogate replacement.
This narrative is historical because retained HPHYS0207 and `INV-SYSTEM-027`
now expose live WB13 profile publication-lineage authority.

## HPHYS0206-NORMALIZED-LAYER-MAPPING-AND-FAIL-CLOSED-ADDENDUM-HISTORICAL HPHYS0206 Normalized-Layer Mapping and Fail-Closed Addendum (Historical)

- status: historical
- source_package: HPHYS0206 Normalized-Layer Mapping and Fail-Closed Addendum (Historical)
- effective_date: 2026-06-10
- verdict: historical
- superseded_by: HPHYS0207 Normalized-Profile FC/WP Depth-Authority Addendum
- canonical_binding_ids: INV-SYSTEM-027
- migration_target: none
- provenance_anchors: SCSTRUCT05; SC-SYSTEM-001 Binding Exposure Index; REF-SYSTEM-LEGACY-WATBAL; REF-SYSTEM-LEGACY-OUTFIL

System-boundary publication authority for `ProfileFCStore` and `ProfileWPStore`
previously required authoritative `thetfc_####` and `thetdr_####` symbols to
originate from baseline-normalized corrected-layer lineage consistent with
`ProfileDepth` and `ProfilePorosityCap` authority surfaces. Mapping from
normalized corrected layers into emitted OFE authoritative layer symbols had to
be deterministic and depth-domain complete. Raw parser-theta substitution for
authoritative FC/WP publication symbols was prohibited when normalized
corrected-lineage projection was required. Missing normalized corrected-lineage
inputs, mapping incompleteness, or non-finite/domain-invalid mapped
authoritative symbols were typed fail-closed system-boundary violations. This
narrative is historical because retained HPHYS0207 and `INV-SYSTEM-027` now
expose live WB13 profile publication-lineage authority.
