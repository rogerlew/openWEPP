# Worker Handoff

Status: `COMPLETE-PROMOTION-CANDIDATE`.

Next route: `SNOWDENSITY-05 Runtime Opt-In`.

## What SNOWDENSITY-04 Proved

- `dense_slow_melt_v1` is the best offline `physics_bulk` candidate under the
  v74/v75 SNOTEL profile.
- It beats both openWEPP as-built and legacy as-built by the package rule:
  lower robust fail count (`6` vs `9`) and higher robust ordinal score
  (`102` vs `84`).
- It uses global named constants only.
- It remains offline and no-site-tuned.

## What It Did Not Prove

- No runtime production coupling.
- No default activation.
- No legacy snow deletion.
- No frost attribution unblock.
- No production publication or consumer-path closure.

## SNOWDENSITY-05 First Actions

1. Scaffold runtime opt-in package for `snow_model = physics_bulk` using
   `dense_slow_melt_v1` as the opt-in candidate default.
2. Amend/confirm `SC-SNOWFREEZE-001` runtime opt-in obligations before
   production code.
3. Implement winter-column typed state coupling without changing
   `legacy_wepp` default behavior.
4. Prove independent SWE/depth/density/liquid/cold-content closure in runtime,
   not only snowbench.
5. Rerun SNOTEL and non-SNOTEL snow/frost gates after opt-in runtime output is
   available.
