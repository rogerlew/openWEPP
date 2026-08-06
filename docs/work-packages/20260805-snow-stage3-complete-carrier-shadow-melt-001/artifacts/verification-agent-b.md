# Terminal Verification B

Evidence class: Static + Ran on unchanged clean HEAD
`ffeecbaeaa3d104284007180ffb012bf5e2ec60c`.

Verdict: `PASS` for executed-HOLD closeout. No substantive blocker remains.
This is not a completion, promotion, or cutover verdict.

Independent streaming reconstruction recovered:

| Quantity | Result |
| --- | ---: |
| Windows | `35` |
| Shortwave | `223.2500438 MJ m^-2` |
| Complete carrier | `+170.2536089 MJ m^-2` |
| Cold-energy change | `-28.7523397 MJ m^-2` |
| Positive excess | `196.4732604 MJ m^-2` |
| Shadow melt | `588.9486222 kg m^-2` |
| Terminal unallocated | `1.72275354e-15 MJ m^-2` |
| CoE raw melt | `0.4100859994 m` |
| Maximum emitted allocation residual | `1.52067514e-9 J m^-2` |

Binary SHA `7e3cc80d...95f0`, sidecar SHA `a400ac25...daa5`, trace SHA
`621bd3f9...4716`, and source commit `2d035638...bddcf` match. The manifest
binds the direct executor, `14,245/14,245` executed days, and zero compatibility
edges.

Verified claim boundaries:

- carrier plausibility is `FAIL`;
- chronology and terminal meltout are `NOT EVALUABLE`;
- the residual covers pre-vapor-debit resolved-substep allocation only;
- cold-content export, full state closure, thin-pack termination, and receiving
  state remain HOLDs;
- CoE remains authoritative;
- generation `910ab3d3` remains DRAFT with public count zero;
- historical governance deviations are disclosed; and
- the `3,177`-line module blocks `complete` and further feature work.

Accepted exact-head heavy results: formatting, Clippy, doctests, quick
`2,189/2,189`, frost `360/360`, and full workspace `2,238/2,238` all pass.
