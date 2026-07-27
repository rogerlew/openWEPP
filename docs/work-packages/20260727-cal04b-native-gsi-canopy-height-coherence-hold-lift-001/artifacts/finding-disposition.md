# Finding Disposition

Status: `PASS / ALL FINDINGS CLOSED`

Evidence class: `Static + Ran`

| Finding | Disposition | Closure |
|---|---|---|
| A-01 `Bt` publication could replace WB15 foliar physics | accepted | `Bt` is internal to height; `Bf` remains foliar/interception state |
| A-02 missing native branch/guard row | accepted | added `BR-PL-NATIVE-GSI-HEIGHT` |
| A-03 primary equation citation wrong | accepted | Chapter 8 Eq. 8.2.8 is primary; historical source label disclosed |
| A-04 arithmetic guards/vectors incomplete | accepted | checked sum/product/result, overflow, underflow, and saturation bound |
| A-05 YAML versus PL projection enforcement stale | accepted | `bb/xmxlai` remain YAML/runtime; `bbb/hmax` are selected-crop runtime projections |
| A-06 BEI omitted obligations | accepted | all GSI producer/consumer obligations are canonical IDs in the BEI |
| A-R1 structural-only underflow unguarded | accepted | `Bt>0 => Hc>0` or typed failure everywhere |
| A-R2 top-level `Bt` consumer wording stale | accepted | sole `Bt` consumer is internal height projection |
| B-01 consumer height overclaim | accepted | all consumers share the realization; ET/erosion/Lane D specifically share `Hc` |
| B-02 BEI checker self-authenticated missing IDs | accepted | core IDs exclude BEI text; three regression vectors pass |
| R-01 active erosion used optional PMET height with a zero fallback | accepted | erosion now reads post-growth `growth.canopy_height_m`; runtime trace and negative source guard pass |
| R-02 frost thermal input retained management-seed height | accepted | post-growth height is the explicit typed frost override; exact builder/consumer trace passes |
| R-03 legacy `hmax=0` parity could evaluate overflowing exponent operands | accepted | exact-zero short circuit precedes exponent arithmetic and returns canonical `+0.0`; regression passes |
| R-04 height tests named transitions without explicit before/after vectors | accepted | explicit deciduous leaf-on/leaf-off, structural leaf-off, and evergreen-floor state vectors added |
| R-05 typed negative tests asserted only generic failure | accepted | parameter-domain, checked-sum/product overflow, and underflow assert exact error variant and field |
| R-06 WARN-size owning aggregates require explicit disposition | accepted | line-count artifact records exact counts, bounded additions, and retained structural follow-up |
| R-07 active/shadow Lane D exact height was not traced | accepted | active routing records its consumed height and the real shadow operand seam is executed from the same native day frame; exact equality is asserted |
| R-08 structural-only underflow lacked behavioral atomicity proof | accepted | real `ForestCanopyState` candidate advances, fails height with `Bs>0/Bf=LAI=0`, and the retained state remains exactly equal to its snapshot |
| T-01 native erosion correction changed legacy non-native sediment behavior | accepted | post-growth height is selected only for `TypedStateOverride`; legacy PMET semantics and the P61 sediment regression pass |
| T-02 frozen invalid case rejected canonical typed parser wording | accepted | verifier recognizes both canonical intake and runtime wording; regression and unchanged 12-case replay pass |
| T-03 full workspace blocked by stale assurance identity | accepted and closed externally | typed assurance adoption plus TESTGATE historical-root decoupling closed the blocker; exact full profile passes 2,301/2,301 |
| H-01 HOLD legitimacy audit remained queued | accepted | populated boundary, evidence, in-envelope routes, and the separate assurance authority requirement |
| H-02 exact diff/write-set reconciliation was incomplete | accepted | intended write set now names every orchestrator/runner surface and the exact owned-file manifest is populated |
| H-03 full-profile evidence mixed pre-fix and exact-head failures | accepted | gate chronology distinguishes the earlier frame/P61 failures, their fixes, the 2,180-test pass, and the exact-head unfiltered rerun |
| H-04 defect-shaped handoff was stale | accepted | first item now closes named defect `ASSURANCE-CANOPY-README-IDENTITY-001` with authority, evidence, acceptance, and forbidden routes |
| H-05 required-case verifier edit lacked anti-evasion gates | accepted | source anti-evasion guard and three-test AUTH11 obligation suite pass |
| H-06 queued review/verification artifacts were ambiguous | accepted | review findings are recorded; terminal verification is explicitly blocked/not run until the full gate is repaired |

No contract, implementation, review, or terminal finding remains open.
