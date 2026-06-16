# Artifacts

Status: F-A complete; F-B (frost-overflow double-count defect-closure) executing.

Evidence and disposition artifacts for
`20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001`.

| Artifact | Increment | Content |
|---|---|---|
| `fixture-and-baseline-evidence.md` | F-A | H2637 (19-OFE) fixture provenance + reproduce; clean `wepp_260606` legacy baseline; openWEPP run that surfaced Finding 1. |
| `dc-execplan-frost-overflow-double-count.md` | F-B | Defect-Closure ExecPlan: envelope, 7-gate bar, milestones, decision log for the `watbtm` double-count. |
| `disposition.md` | F-B | Terminal disposition + validation evidence. |
| `worker-handoff.md` | — | Defect-shaped handoff (`watpdg` branch-out + remaining demonstration scope). |

Finding 1 (the frost bottom-overflow `watbtm` double-count in the per-element
WB13 internal frost adjustment) is closed contract-first under F-B:
`SC-WATBAL-001` v162; `per_ofe_internal_wb13.rs` line-432 correction;
inline regression `farpoint01_internal_frost_adjustment_excludes_watbtm_lower_overflow`.
