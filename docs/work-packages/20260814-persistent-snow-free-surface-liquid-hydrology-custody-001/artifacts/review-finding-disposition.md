# Review Finding Disposition

Evidence class: `Static + Ran`

All authority-review findings were accepted. None was rejected, deferred, or
moved to follow-up.

| Finding family | Disposition | Correction |
|---|---|---|
| LSE identity and condensation basis | Accepted / closed | Exact surface/source/source-ID mapping and OFE-ground DTO basis. |
| Kernel profile, guards, units, calibration | Accepted / closed | Obligations, invariant/guard/evidence table, machine registry, readiness matrix. |
| Actual WB14 and duplicate depression custody | Accepted / closed | Shared production transition, zero legacy depression capacity, persistent post-infiltration retention. |
| Cadence and restart continuation | Accepted / closed | Exact 48-step per-OFE continuation embedded in state/digest/lineage. |
| Precipitation/canopy duplication | Accepted / closed | Mutually exclusive open raw rain and covered accepted canopy release. |
| Enthalpy priority and retained LSE energy | Accepted / closed | Conservative mixing, exact tile/source attribution, tile-basis energy receipt. |
| Cross-tile redistribution | Accepted / closed | Excess retains only against its exact tile/source key. |
| Multi-OFE area basis | Accepted / closed | `basis_ofe_id` re-keying and once-only `A_u/A_d` mass/energy conversion. |
| Evidence overstatement | Accepted / closed | Independent cadence vector added; runtime parity reserved for implementation. |

Final contract/profile reviewer: `PASS`.

Final hydrology/ownership reviewer: `PASS`.

## Runtime implementation findings

| Finding | Disposition | Remediation status |
|---|---|---|
| Actual soil-liquid and soil-thermal infiltration recipients absent | Accepted | Remediated; focused PASS |
| Producer-self-referential and partial closure | Accepted | Remediated; focused PASS |
| Strict restart combinations under-validated | Accepted | Remediated; focused PASS |
| Production OFE/lane/area/day identity unbound | Accepted | Remediated; focused PASS |
| Canonical typed error identity and rollback context absent | Accepted | Remediated; focused PASS |
| Actual LSE/soil-thermal receiver candidates and rollback hashes discarded | Accepted | Remediated; focused PASS |
| Public mutable/forgeable resource candidate | Accepted | Remediated; focused PASS |
| Canonical persistence bytes differ from digest representation | Accepted | Remediated; focused PASS |
| Unified digest uses legacy 64-bit token rather than canonical soil bytes | Accepted | Remediated; focused PASS |
| `runoff.rs` exceeds mandatory line threshold; WARN files undispositioned | Accepted | Remediated; focused PASS |
| Complete WB14 interval transition duplicated | Accepted | Remediated; focused PASS |

No runtime finding is rejected, deferred, or moved to follow-up. Exact-byte
Rust and hydrology re-reviews and terminal verification remain pending.

## Re-review round 1

| Finding | Disposition | Remediation status |
|---|---|---|
| B-REMEDIATION-HIGH-001: actual receiver ending equations lack independent reconstruction | Accepted | Remediated; focused PASS |
| B-REMEDIATION-HIGH-002: canonical E001--E011 runtime payload is incomplete | Accepted | Remediated; focused PASS |
| A-REMEDIATION-HIGH-001: mutable arbitration can forge proportional authorization | Accepted | Remediated; focused PASS |
| A-REMEDIATION-MEDIUM-004: invalid public state can emit canonical bytes | Accepted | Remediated; focused PASS |
| A-REMEDIATION-MEDIUM-005: ingress/unified candidates remain mutable and duplicated | Accepted | Remediated; focused PASS |

All findings are in-scope implementation defects. Their corrections change no
authority, model identity, production selection, or package write envelope.
No finding is closed until final exact-byte re-review passes.

## Final re-review

| Finding | Disposition | Remediation status |
|---|---|---|
| B-FINAL-HIGH-001: extra/nonfinite thermal layers and forged rollback owner identity accepted | Accepted | Remediated; focused PASS |
| B-FINAL-HIGH-002: E004/E007/E011 omit available offending identity context | Accepted | Remediated; focused PASS |
| A-FINAL-HIGH-001: independent soil aggregate omits valid residual/frozen storage | Accepted | Remediated; focused PASS |

These are bounded implementation defects in the existing bridge and receiver
validator. The correction seals independent LSE and soil-thermal receiver
expectations, narrows the Child-3 rollback set to its actual three owners,
reconstructs production aggregate soil water with residual water over unfrozen
depth, and includes typed owner/OFE/tile context when known. No new authority or
package is indicated. Fresh exact-byte review remains pending.
