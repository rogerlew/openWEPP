# Kernel Profile Compliance

Status: complete / terminal gates pass

Evidence mode: Static + Ran

| Profile concern | Disposition |
| --- | --- |
| Authority/provenance | PySnobal 0.2.3 supplies wet-compaction chronology and melt-plus-rain meaning; Anderson 1976 supplies the physical family; pinned WEPP supplies signed melt, rain-contact, storage, and later-routing cut points. |
| Algorithm order | Exact hourly positive applied melt and snow-contact rain are finalized before signed daily redistribution/runoff and passed once to density. |
| Branch map | Active coupling computes and guards the value; inactive coupling supplies zero; the selected bulk or multilayer density consumer receives the same private field. |
| Units | Source components and private handoff are `m water equivalent`; the existing density boundary converts once to `kg m^-2`. |
| Guard map | The existing typed boundary helper rejects non-finite or negative `snow.wet_compaction_liquid_input_m`; no canonicalize-and-proceed behavior was added. |
| Alias map | Duplicate state-loss-plus-routed, routed-only, state-loss-plus-rain, raw rain, retained-store level/change, signed melt, and redistributed melt are rejected. |
| State and conservation | Upstream SWE/melt/routed-liquid mass, compact ledgers, Stage-3 incoming liquid, and every retained closure remain protected. Density/depth changes may legitimately alter Stage-3 routed/retained/refrozen disposition; those deltas are reported separately and are not misclassified as upstream mass drift. |
| Numerics | Existing Anderson/SNOBAL wet-compaction formula, constants, half-saturation, density cap, and closure tolerances are unchanged. Operand reconstruction uses `1e-12 m`. |
| Calibration | `NOT_APPLICABLE`; no parameter was fitted. The Snowbird scaled CLI is `DEVELOPMENT_ONLY`, not physics authority. |
| Test vectors | A deliberately non-equal scalar vector and a real consumer vector distinguish every prohibited alias and reconstruct the accepted input independently. |
| Publication/API | Private production field only; no public result, public replay CSV, or runtime trace schema changed. The internal CoE melt-to-density CSV carries the exact additive lineage and intentionally rejects historical files that cannot prove it. |
| Validation | Pre-fix red gate and post-fix focused green gate are recorded. Canonical/materiality and terminal profiles are delegated and recorded separately. |
