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
| Poisons/invariants were prose-only | accepted/corrected | Numeric wrong-formula/alias/pore cases execute; rejected vectors traverse typed guards; configuration/receipt validators execute ordering, topology, nested digest, source identity, geometry, frozen, and caller-reseal poisons while proving source bytes unchanged. |
| Gravity sign opposite cited CLM | accepted/corrected | Receipt now stores signed `-1000*z_node`, which existing V10 adds; `z3` remains independently positive. Exact source anchor added. |
| Conductivity source locator inaccurate | accepted/corrected | Hydrology equation 2.7.47 is named for the intrinsic factor; vertical-interface averaging is explicitly excluded; immutable CTSM source anchors layer-local current `hk_l` consumption. |
| Contract profile/readiness incomplete | accepted/corrected | Canonical serialization, topology joins, precondition priority, parameters, readiness matrix, and Binding Exposure Index added. |
| Source-owner projection copied receipt fields and coupled one occupancy to one hydrology layer | accepted/corrected | Independent hydrology-layer and vegetation-root-binding projections now have separate canonical digests; multiple occupancies/strata can bind one layer and executable evidence validates that topology. |
| `dxroot`, predecessor depth, frozen, and accessibility were not independently source-owned | accepted/corrected | `dxroot` comes from the exact root binding, predecessor depth from ordered hydrology rows, an independently frozen source rejects, and inaccessible bindings cannot mint receipts. |
| Poison atomicity snapshotted only the receipt and matrix evidence was generic prose | accepted/corrected | Rejections preserve the combined receipt/configuration/source byte snapshot; every generated matrix category names its executable Rust or schema gate, and the integration test requires the complete inventory. |
| Root-binding digest was not bound to immutable vegetation authority | accepted/corrected | The expected static context binds the canonical vegetation root-binding digest; source admission recomputes it, enforces canonical unique order, and rejects a coordinated source/receipt/digest mutation. Root operands use the native `lateral_root_length_m` name. |
| Receipt accessibility/frozen posture could diverge from source | accepted/corrected | Both fields are joined exactly before posture disposition; receipt-only mutations return `OwnerJoin`, independently frozen sources return `FrozenRootedLayerUnsupported`, and matching inaccessible sources return `InaccessibleRootedLayer`. |
| Source shape and named poison execution were insufficiently bound | accepted/corrected | A closed source-owner schema rejects unknown/WB14 fields; explicit wrong-clamp, lawful-path default, coordinated caller-reseal, root-order, and complete digest poisons execute. |
| Scalar-domain precedence was behind frozen/accessibility posture | accepted/corrected | Source fields join first, scalar domains execute next, posture follows, then pore/equations and receipt digest; a synchronized invalid-thickness plus frozen-source case proves `Domain` precedence. |

All three required reviewers independently returned PASS with no material
finding on exact frozen authority commit
`b30f42de67136bca37f888fa62e8f1145537a230`. No finding was waived or deferred.
