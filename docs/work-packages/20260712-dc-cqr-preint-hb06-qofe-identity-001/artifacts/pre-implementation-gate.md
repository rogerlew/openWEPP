# Pre-Implementation Gate

Evidence class: **Static**, pending red-test reproduction.

Current code skips QOFE/Q identity validation when the per-OFE policy marker is
set. `INV-SYSTEM-031` explicitly says public `QOFE == Q` is the canonical
convention and that the former slplen-based rejection heuristic is superseded;
genuineness moves to independent lineage evidence, not to unequal public values.
`INV-WATBAL-098` and `INV-RUNOFFPART-032` agree.

The bounded correction removes only the marker-conditioned bypass and retains
the existing tolerance/error/priority. DC conversion criteria pass subject to
a red per-OFE mismatch regression before production correction.
