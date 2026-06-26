# Worker Handoff

Evidence class: Static.

## Current State

SNOWDENSITY-05D is complete. The opt-in CoE melt path exists and is guarded,
but it is not activated by default and is not externally selectable through a
parser or CLI surface.

## What Changed

- `SC-SNOWFREEZE-001` v79 ratifies the opt-in production formula.
- The typed hydrology path can run `coe_shortwave_albedo_v1`.
- Direct runtime state carries albedo and melt-lineage operands.
- Runner production day input still selects `legacy_coe`.
- Focused reconstruction and fail-closed tests are in place.

## Next Package

Recommended next package: `SNOWDENSITY-05E Melt Rubric Adjudication`.

Expected scope:

- Run rubric profiles comparing legacy/default and the opt-in melt candidate.
- Use forcing-robust signatures from `INV-SNOWFREEZE-050`.
- Do not tune shared radiation forcing.
- Do not fit defaults to the five SNOTEL sites.
- Decide whether the opt-in candidate is promising, needs further contract work,
  or should remain parked behind the selector.

Do not default-activate the opt-in path without a separate contract amendment,
explicit rollback plan, and profile-level evidence.
