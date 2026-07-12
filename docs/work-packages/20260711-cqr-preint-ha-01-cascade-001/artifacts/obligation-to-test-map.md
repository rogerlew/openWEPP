# Obligation-To-Test Map

Evidence class: **Ran**

| Family / authority | Tests | Status |
| --- | --- | --- |
| A nominal interpolation and cascade | `interpolate_unit_discharge_characterizes_search_endpoints_and_clamp`; `two_ofe_cascade_conserves_and_hands_off` | Bound |
| B endpoint/range and width boundary | `interpolate_unit_discharge_characterizes_search_endpoints_and_clamp`; `width_change_scales_handoff_for_discharge_continuity` | Bound |
| C empty/singleton/search/clamp, direct point-fallback `Some`/`None`, and multi-OFE branches | `interpolate_unit_discharge_characterizes_search_endpoints_and_clamp`; `two_ofe_cascade_conserves_and_hands_off`; `three_ofe_cascade_accumulates_downslope` | Bound |
| D invalid domain | `degenerate_cascade_fails_closed` zero/negative widths and empty cascade | Bound |
| E missing required state | N/A: this typed/private seam has no optional required symbol; upstream absence is the valid summit branch | Bound by applicability |
| F non-finite input | `degenerate_cascade_fails_closed` `NaN` and both infinities | Bound |
| G conservation/handoff | `two_ofe_cascade_conserves_and_hands_off`; `width_change_scales_handoff_for_discharge_continuity`; `solver_ledger_books_scheme_actual_boundary_fluxes`; `handoff_injection_is_flux_integral_conservative`; `runon_only_ofe_handoff_is_nonnegative_and_conservative`; `partial_final_bin_handoff_is_exact` | Bound |
| H exact fail closed | `degenerate_cascade_fails_closed` exact typed error assertions | Bound |
| `INV-OFEROUTE-008`, D5 multi-OFE vectors | `two_ofe_cascade_conserves_and_hands_off`; `three_ofe_cascade_accumulates_downslope`; `width_change_scales_handoff_for_discharge_continuity`; four exact D10B handoff/ledger tests above | Bound |
| `INV-OFEROUTE-006` scheme ledger and cascade conservation | `solver_ledger_books_scheme_actual_boundary_fluxes`; `handoff_injection_is_flux_integral_conservative`; `runon_only_ofe_handoff_is_nonnegative_and_conservative`; `partial_final_bin_handoff_is_exact` | Bound |
| `OBL-OFEROUTE-P-003` | `solver_ledger_books_scheme_actual_boundary_fluxes`; `handoff_injection_is_flux_integral_conservative`; `runon_only_ofe_handoff_is_nonnegative_and_conservative` | Bound |
| `OBL-OFEROUTE-C-002` | `two_ofe_cascade_conserves_and_hands_off`; `width_change_scales_handoff_for_discharge_continuity`; `partial_final_bin_handoff_is_exact` | Bound |

The current real upstream consumer selects `integrate_bin_series`; the point
fallback is characterized directly and is not represented as consumer-path
proof.
