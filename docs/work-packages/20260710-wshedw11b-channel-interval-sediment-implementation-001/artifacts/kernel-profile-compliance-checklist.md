# Kernel Profile Compliance Checklist

Status: `EXECUTED-PASS`

Evidence mode: `Static + Ran` implementation/test binding review.

| Requirement | Evidence | Result |
|---|---|---|
| Canonical authority | `SC-ROUTE-001` v53 and pinned baseline SHA; no amendment needed | PASS |
| Conversion rule | direct SI-to-English bridges precede legacy WS20/21/22/23/24/26 calls; class results return as kg | PASS |
| Covering grid | finite positive `dtchr`, integer `ntchr`, exact 86,400 s coverage guard | PASS |
| Exact projection | interval/hour overlap; source/projected magnitude closure | PASS |
| Wave branches | segmented pinned KW/MC equations; `mofapp=1`; KW `qref=qtmax`; MC `qref=0.5*(qtmin+qtmax)`; signed interior MC state and outlet-only epsilon; `ipeak=5` dynamic refresh | PASS |
| Hydraulic map | distinct `qt`, whole-reach `qlat`, `qe`, `leff`, `qu_top`, and `qe/leff`; anti-alias vector rejects event/raw/`qlat/lc` proxies | PASS |
| Exposure clocks | interval `t_exp=t_norm=dtchr`; event wrapper retains prior triangular exposure and normalization | PASS |
| Sediment family | complete existing WS20/21 segment family called per active nonzero interval; no surrogate process math | PASS |
| Geometry | separate `depa/depb`, `wida/widb`, `wera/werb`; lower and upper mutation carry; explicit no-primary/primary-tillage day authority; no refill/narrow; zero flow holds; corrected dcap terminals | PASS |
| Sediment state | class inlet/lateral/constructive-gross-detached/continuity-deposited/egress ledger; no suspended cross-interval/day pool | PASS |
| Zero-source class | exact inlet/lateral/detachment zero produces exact zero egress; nonzero negative deposition fails | PASS |
| Channel bed composition | required `pw0.sol` texture -> Rust `prtcmp` -> channel-indexed `crfrac` per pinned `convrt.for` | PASS |
| Dependency | downstream reads typed upstream `q1` and class egress at identical index | PASS |
| Publication | active terminal-channel aggregation prevents internal network double count; daily yield is kg egress | PASS |
| Typed guards | invalid grid, clocks, mixed authority, class/grid mismatch, aliases, geometry reset, negative/nonfinite operands fail with boundary symbols | PASS |
| Protected path | non-activated lane remains legacy event implementation; runner protected test passes | PASS |
| Vectors/gaps | all eleven W11A vectors plus both `GAP-ROUTE-014` terminals bound to production/core tests; active ledgers expose pinned hydraulic, shear, and transport-capacity observations | PASS |

No production `unwrap()` or `expect()` was introduced. `unreachable!` remains
only after a locally exhaustive `ws11_case_id` constraint in the pre-existing
legacy lane; it is not on the new interval path.
