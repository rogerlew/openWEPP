# Kernel-Profile Compliance Checklist

Status: `EXECUTED-PASS`

Evidence mode: `Static + Ran`

| Requirement | Evidence | Result |
|---|---|---|
| Contract-first authority | initial `SC-ROUTE-001` v55 / `SC-SYSTEM-001` v89 / `SC-INFILE-CHANINP-001` v0.1.3 preceded initial red tests; review corrections v56/v90/v0.1.4 preceded their additional red tests and production corrections | PASS |
| Pinned provenance | `wshchr.for` `sinit/sfnl/chvol`, MC recurrence; `wshinp.for` implied-DO zero count; pinned SHA in contracts | PASS |
| Volume operands | physical `volint` = local `qlat` interval volume + dependency daily `chvol`; `Σq1 dt` is diagnostic only | PASS |
| Recurrence grid | separate time-zero state; exactly `ntchr` routed terminals for `it=1..ntchr`; first/last and two-timestep consumer vectors | PASS |
| Hydraulic storage | fresh/MC boundary mean and KW all-terminal-spatial-node Manning mean; 101-segment independent rectangular-Manning reconstruction; finite/nonnegative typed guards | PASS |
| Daily closure | `chvol=volint+sinit-sfnl`; material negative available outlet fails `-E-003`; only TOL-ROUTE-009 roundoff canonicalizes to exact zero | PASS |
| Cross-day carry | prior terminal `q1/qin/qlat` validation plus prior `sfnl -> sinit`; first interval advances rather than aliasing seed; zero-peak storage retained | PASS |
| Channel balance | available `Inflow=volint+sinit`, `Outflow=chvol`, `Storage=sfnl`, zero loss; equivalent to physical-inflow/storage-change equation | PASS |
| MC coefficients | finite `c0..c4`; `c1+c2+c3` sum; `c1..c3 >= -1e-12`; static and dynamic refresh both guarded | PASS |
| Passive MC bound | output cannot exceed max three recurrence sources plus explicit `qlat*dx`, modulo declared roundoff | PASS |
| MC non-vacuity | matched 60-second static/dynamic full routes execute, publish 1,440 slots and convex coefficients, obey passive bound, and diverge under dynamic refresh | PASS |
| No repair path | no coefficient clamp, peak clip, damping, fallback recurrence, negative-storage clamp, or mass injection added | PASS |
| Terminal selection | channel consumed by another channel excluded; consumed impoundment traversed; explicit channel/impoundment/channel anti-alias; terminal-impoundment channel proxy and independent terminals preserved | PASS |
| Sediment units | interval state already kg; direct state rate integrated over channel-ancestry hourly span/direct event duration; impoundment is explicit unsupported ancestry boundary; no `kg s^-1` relabeled `kg` | PASS |
| Consumer proof | real watershed CLI reads corrected sidecar/routing/publication; EBE terminal identity/volume/mass and chanwb balance asserted | PASS |
| Parser closure | three-record strict/compat zero count, positive count, missing/extra record, topology and normalization suites | PASS |
| Protected paths | W11B interval sediment, typed runtime, single/two-channel KW, no-event/event fallback, and p102 HBP/publication/jobs-identity tests pass; the documented p102 wrapper now selects KW because its historical incidental MC grid is correctly inadmissible | PASS |
| Typed errors | existing `WKERNEL-WS10-CHANNEL-E-003`, `CHN-E-002`, and new typed `WSHEDFRAME-E-010` publication validation | PASS |

No production `.unwrap()` or `.expect()` and no `unsafe` block were introduced.
The only new `expect` calls are inside a `#[cfg(test)]` module.
