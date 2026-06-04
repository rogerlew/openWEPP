# Unit Remediation Plan

Status: completed/HOLD
Evidence mode: Static

Static: recommended continuation packages:

1. Soil runtime conversion cluster:
   - migrate `runtime_inputs/02_soil_slope.rs` `mm <-> m`,
     `mm h^-1 <-> m s^-1`, `g cm^-3 -> kg m^-3`, and percent/fraction seams;
   - add helper tests for soil-specific density and conductivity conversions;
   - classify true percentage constants separately from dimensional conversions.
2. Runner publication conversion cluster:
   - migrate hillslope publication `m -> mm` conversions through named helpers;
   - align with HPHYS0278 output metadata authority.
3. Shared climate adapter cluster:
   - migrate parser/runtime `hours -> seconds` and `mm -> m` conversions;
   - avoid duplicating helper constants inside adapter.
4. Guard expansion:
   - after the above clusters close, change
     `check_raw_unit_conversions.py` default from first-wave files to all
     production roots.

Ran: not-run; this is static planning based on HPHYS0276 inventory.
