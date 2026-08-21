# Publication boundary

Status: `DEFAULT-OFF SCHEDULER SEAM / CONSUMER CLOSURE BLOCKED`.

`Static:` The ordinary scheduler keeps the existing invocation point. The new
attachment is explicitly configured and prepared; the production
`stage_snow_stage3_shadow` bridge does not derive physical state from a
completed `DirectDayFrame`, and the legacy caller-built handoff is retained
only under `cfg(test)` in that bridge. No selector, default, or production
output surface was changed.

`Static:` The runner does not currently construct the prepared 48-support
capability or attach the new committed owner from actual run ownership. The
downstream consumer therefore does not read the new path for a real run, so
publication/endpoint closure is not claimed.
