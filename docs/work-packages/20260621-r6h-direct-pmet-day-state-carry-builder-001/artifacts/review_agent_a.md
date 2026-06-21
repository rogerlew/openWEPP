# R6H Review Agent A

Status: complete.

Source: Newton delegated read-only code review.

Evidence class: Static review plus focused gate context.

| Severity | Finding | Evidence | Required action | Disposition |
|---|---|---|---|---|
| High | Later-day PMET seed construction could silently fall back to static layer symbols when no committed direct layers existed. | `overlay_direct_publication_lane_state` returned `Ok(())` for empty `lane.subsurface_layers` regardless of day. | Fail closed after day zero when committed direct-carried layers are absent. | Accepted and fixed. `overlay_direct_publication_lane_state` now receives `day_index` and errors after day zero; `r6h_day_input_overlay_requires_committed_layers_after_day_zero` covers the guard. |
| Medium | R6H hold predicate could over-classify a broader WAT mismatch as the narrow PMET ULP blocker. | `r6h_wat_pmet_layer_carry_ulp_gap` accepted any later `Es` delta <= `1.0e-12` without first-row identity or all-row ULP bounds. | Require first-row identity and reject any later non-ULP `Es` delta. | Accepted and fixed. Predicate now requires first-row equality, `Es` as the only reduced field, and all later mismatches to be ULP-scale; tests cover first-row and mixed-later-row negatives. |

## Verdict

Approved after accepted fixes and focused/full gate reruns.
