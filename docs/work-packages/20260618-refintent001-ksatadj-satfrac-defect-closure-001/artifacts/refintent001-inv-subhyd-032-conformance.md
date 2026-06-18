# REFINTENT001 INV-SUBHYD-032 Conformance

Evidence class: Static + Ran

## Checklist

| Requirement | Status | Evidence |
|---|---:|---|
| Top-two tillage weighting from `dg` / `tillay(2)` | PASS | `tillage_depth` accumulates top-two `dg`; all averages divide by it |
| `avpor` from porosity and layer thickness | PASS | `por_depth += por * dg` |
| `avcpm` from coarse-particle multiplier and layer thickness | PASS | `cpm_depth += cpm * dg`; `cpm_####` is required |
| `avsm15` from residual water (`thetdr`) and thickness | PASS | `thetdr_depth += thetdr * dg`; used in `avsat` |
| `avsat = (st1 + st2) / tillay(2) + avsm15` | PASS | `theta_storage / tillage_depth + avsm15` |
| cap 1: `avpor * 0.98` | PASS | applied before denominator cap |
| cap 2: `avpor * avcpm * 0.99` | PASS | applied after cap 1 |
| `sat_frac = min(avsat / (avpor * avcpm), 1)` | PASS | implemented after denominator guard |
| `avthetafc` / `avthetadr` from direct source-intent terms | PASS | no FC/WP reconstruction fallback remains |
| 9001/9002+/9003 branch formulas and unit conversion | PASS | no branch-formula changes in this package |
| Missing/non-finite/out-of-range operands fail typed | PASS | unit guard test plus existing WB14 contract tests |

## Ran evidence

- `cargo test -p openwepp-hillslope-orchestrator wb14_ksatadj -- --nocapture`
  passed with 2 tests.
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture`
  passed with 16 tests.
- `cargo test --workspace` passed.

Disposition: `INV-SUBHYD-032` is satisfied by implementation and tests. No
contract ambiguity was exposed, so no `SC-*` amendment was made.
