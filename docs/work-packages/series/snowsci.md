# SNOWSCI series — execution log

> Per-package log for the SNOWSCI (snow mass conservation / single-sourcing,
> Stage 1) series. Stage 2 (physics-magnitude) is deferred — see
> [../../ROADMAP.md](../../ROADMAP.md). Index: [../README.md](../README.md).

- `20260606-snowsci-stage1-snow-mass-conservation-closure-001/`
  - Purpose: close defect `SNOWSCI-S1-SNOW-MASS-NONCONSERVATION` by making
    routed snowpack melt and runtime SWE storage share the post-hourly
    depth/density store as the single accounting source, while preserving the
    protected boundary against snow physics-magnitude edits and silent clamps.
  - Status: closed-with-follow-up-postreview; contracts, red/green test, and production
    accounting were updated so mixed signed raw melt can no longer create an
    independent SWE debit. Release `p7`, `p11`, `p18`, and `p20` now publish;
    post-review gates ran, and WBVAL06 max annual R fell from `94.433070 mm`
    to `26.790809 mm` on the 18 WBVAL04 status-valid emitters. WBVAL06 annual
    residual attribution was closed by the WBVAL06 package as omitted
    interception publication.
