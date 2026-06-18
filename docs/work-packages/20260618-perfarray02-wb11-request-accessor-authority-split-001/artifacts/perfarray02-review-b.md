# PERFARRAY02 Review B

Evidence: Static + Ran.

## Independent Review

Finding disposition:

| Finding | Disposition |
| --- | --- |
| Array reads must not silently fall back to logical maps in pilot mode | accepted, checked in `state_access.rs` |
| Runoff producer-published infiltration aliases must be array-visible | accepted, fixed through array-aware presence checks |
| `apply_kernel_writeback` samples in perf could be misread as dual-write | accepted, documented: samples are global non-runoff phases; static branch proves no dual-write for array payload |
| H2637 floor exceeds <=10x budget | accepted, package closes NO-GO |

## Residual Risk

The pilot is not Stage C: persistent scheduler authority remains logical and boundary
seed/materialize remains large. Any future attempt to pursue array authority should start
from a narrower writeback-payload/native-array kernel-output design or close the perf track
as not worth the architecture cost.

## Gate Legitimacy

Each package gate has current evidence:

- structural proof: static branch anchors plus perf report;
- identity: OFE5 and H2637 checksums/row equality;
- floor: H2637 timing split;
- determinism: static order invariants plus identity evidence;
- Rust gates: all required commands passed.
