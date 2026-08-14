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

| Finding | Disposition | Remediation status |
|---|---|---|
| B-FINAL-PASS-HIGH-001 / A-FINAL-PASS-HIGH-001: E011 substitutes the first configured receiver and hydrology owner for the actual offending receiver or rollback row | Accepted | Remediated; focused PASS |

The first canonical structural offender now supplies typed owner, OFE and tile
context. Missing rows use the exact expected owner/identity; malformed or extra
rows retain their actual identity. Two-row thermal and wrong LSE/soil-thermal
rollback poisons assert the payload rather than only the error code.

| Finding | Disposition | Remediation status |
|---|---|---|
| B-TERMINAL-HIGH-001 / A-TERMINAL-HIGH-001: later independent thermal expectation mismatch reports first LSE receiver | Accepted | Remediated; focused PASS |
| A-TERMINAL-HIGH-002: missing non-terminal rollback row reports shifted following owner | Accepted | Remediated; focused PASS |

Expectation preflight now reports the soil-thermal owner and the exact first
actual mismatch, or expected missing row. Rollback sequence validation detects
a deletion before treating the shifted row as malformed, so a missing first
LSE row identifies the expected LSE owner. Equal-length wrong rows continue to
identify their actual wrong owner.

| Finding | Disposition | Remediation status |
|---|---|---|
| B-RELEASE-HIGH-001: public bridge admits represented frozen/thawing and snow-retained-liquid-only state | Accepted | Remediated; focused PASS |
| A-RELEASE-HIGH-002: finite same-store demand accumulation can overflow and produce non-proportional authorization | Accepted | Remediated; focused PASS |

The public bridge now returns contextual E004 before authorization or callback
for snow runtime/carry, retained snow liquid, frost runtime/carry, or positive
production-layer frozen depth/water. The poison matrix proves callback
non-invocation and byte-identical production state. Demand accumulation and
every proportional-allocation intermediate now fail closed on nonfinite
results; retained arbitration reconstruction independently repeats the guard.
