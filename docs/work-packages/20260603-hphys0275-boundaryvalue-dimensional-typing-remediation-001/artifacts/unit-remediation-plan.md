# Unit Remediation Plan

Status: completed
Evidence mode: static

Static: Recommended continuation order:

1. Add direction-specific boundary typing for `wind` direction and decide the
   domain contract (`0..=360`, circular degrees, or legacy direction class).
2. Migrate watershed-prefixed climate aliases to typed `BoundaryValue`
   producers or keep them explicitly split from hillslope rows.
3. Migrate snow runtime state and retained snow trace rows.
4. Continue with HPHYS0276 named conversion-helper/raw-literal governance.
5. Continue with HPHYS0278 output metadata alignment so publication schema
   units use the same authority as runtime symbols.

Ran: not-run; static remediation plan.
