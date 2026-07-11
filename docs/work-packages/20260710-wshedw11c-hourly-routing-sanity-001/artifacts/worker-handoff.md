# Worker Handoff

Status: `EXECUTED-HOLD-SANITY-FAIL`

Evidence mode: `Static + Ran`

Do not treat the current hourly watershed routing path as physically
sanity-approved for production use. The real CLI executes all tested branches,
but W11C-F001 through F004 remain open production defects.

Resume with queued package
`20260710-wshedw11d-hourly-routing-numerical-defect-closure-001`. Its bounded
contract-first scope owns:

1. KW/static-MC negative storage and generated terminal volume;
2. static/variable-MC peak amplification and timestep sensitivity;
3. legacy CREAMS terminal-outlet volume/sediment publication; and
4. canonical `chan.inp nchnum=0` parsing without timestep aliasing.

Use W11C corrected debug run `f695f3db-0627-4c28-8d97-8e5c5d023158` and exact
release run `29024159-9f78-4506-9918-09c7f007af0d` as the starting regression
matrix. Preserve W11D's prohibition on clamps, peak clipping, surrogate
physics, silent defaults, and publication-only masking.
