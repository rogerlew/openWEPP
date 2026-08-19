# Review Finding Disposition

Evidence class: `Static + Ran`

Frozen candidate `469b0ef00e8d233ff72b1606db1787e7eb834b54` received three
independent `FAIL` reviews. No finding was waived or deferred.

| Finding | Disposition | Correction/evidence |
|---|---|---|
| Python `math.pow` was not exact Rust authority | accepted/corrected | Every accepted vector is now recomputed and exact-bit compared by Rust `libm 0.2.16`; the one observed host-libm ULP difference is recorded and emitted with the Rust-authoritative bit. |
| Exact domains and pore tolerance incomplete | accepted/corrected | Contract gives all inequalities and exact `capacity.to_bits()+1` predicate; one-bit acceptance/two-bit rejection execute. |
| Exact saturation and signed-zero contradictions | accepted/corrected | Inputs use binary-exact capacity; signed zero normalizes all zero saturation intermediates. |
| Schema/digest/manifest incomplete | accepted/corrected | Closed configuration and receipt schemas, canonical digest vector, runtime descriptor, and manifest are generator-owned and checked. |
| Poisons/invariants were prose-only | accepted/corrected in authority scope | Numeric wrong-formula/alias/pore cases execute; identity/schema/digest cases are closed by typed schemas and canonical digest checks. Live-owner atomicity remains an implementation-phase obligation and is no longer falsely claimed by the authority inventory. |
| Gravity sign opposite cited CLM | accepted/corrected | Receipt now stores signed `-1000*z_node`, which existing V10 adds; `z3` remains independently positive. Exact source anchor added. |
| Conductivity source locator inaccurate | accepted/corrected | Hydrology equation 2.7.47 is named for the intrinsic factor; vertical-interface averaging is explicitly excluded; immutable CTSM source anchors layer-local current `hk_l` consumption. |
| Contract profile/readiness incomplete | accepted/corrected | Canonical serialization, topology joins, precondition priority, parameters, readiness matrix, and Binding Exposure Index added. |

All three reviewers must re-review the next frozen candidate commit. This file
does not convert the failed reviews into PASS.
