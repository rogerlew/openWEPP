# Review Disposition

Status: `EXECUTED-ALL-ACCEPTED-AND-FIXED`

Evidence mode: `Static + Ran` finding-to-fix audit.

| finding_id | source | severity | decision | action_taken | artifact_ref | rationale |
|---|---|---|---|---|---|---|
| A-H1 | A | High | accepted | distinct `qin`/whole-reach `qlat`; recurrence anti-alias | `hourly.rs`, focused tests | water source must enter once |
| A-H2 / B-H1 | A/B | High | accepted | DCAP gross detachment by solved span; continuity-derived deposition | routing core + conservation artifact | removes tautological net split |
| A-H3 / B-H2 | A/B | High | accepted | lower `depa`; distinct `wida/widb` and `wera/werb` | typed geometry + carry test | canonical six-field state |
| A-H4 / B-H4 | A/B | High | accepted | typed tillage-day authority and production carry/reseed | frame/hourly test | sole authorized reseed |
| A-H5 | A | High | accepted | production owner/core, water, shear/capacity, geometry, failure vectors | focused suite | binds real code paths |
| B-H3 | B | High | accepted | local baseflow separated from cumulative baseflow | two-channel reconstruction | external source once |
| B-H5 | B | High | accepted | partial dependency XOR hard-fail | guard regression | prevents scalar fallback |
| B-M1 | B | Medium | accepted | positive side-depth and six-field finite/domain guards | hourly guards | fail closed numerically |
| B-M2 | B | Medium | accepted | outlet metrics and metadata share outlet ID set | CLI asserts element 2 | prevents upstream/outlet mismatch |
| B-N1 | B | High | accepted | ENDDET returns and uses solved detachment span | ENDDET anti-alias | gross mass uses actual span |
| A-N1 | A | High | accepted | KW uses prior routed `q1`, tail and prior-day state | recurrence tests | pinned wave memory |
| B-N2 / V-B-H1 | B | High | accepted then superseded by pinned verification | signed interior MC state is retained; `1e-8 m3/s` normalization occurs only at the KW/MC outlet | `hourly.rs`, outlet-boundary anti-alias | matches `wshchr.for:447-448,567-571`; no interior clamp |
| B-N3 | B | Medium | accepted | prior-day `q1/qin/qlat` validation and malformed tests | hourly tests | blocks NaN/negative bypass |
| B-N4 | B | Medium | accepted | named `1e-8 m3/s` qref epsilon | constants/diagnostics | pinned MVPMC3 provenance |
| V-A-H1 | verification A | High | accepted | KW uses `qtmax`; MC uses `0.5*(qtmin+qtmax)` | branch-specific qref anti-alias | matches `wshchr.for:326-328` |
| V-B-H2 | verification B | High | accepted | exact zero inlet/lateral/detachment class produces exact zero egress | segment continuity + integration/P102 | prevents synthesized particle mass without relabeling a residual |
| V-B-H3 | verification B | High | accepted | required `pw0.sol` is projected through Rust `prtcmp` into channel-indexed `crfrac` | production CLI + P102 | matches `convrt.for:84-88`; no inlet-composition fallback |

No finding was rejected, deferred, left open, or assigned follow-up. Final
focused evidence: 23/23 W11B hourly/ENDDET, 18/18 typed integration, 2/2
runner, and 1/1 protected P102 tests passed.
