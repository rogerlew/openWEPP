# Review Agent A — Terminal Closure 8 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `298acedbb47455d5ce54ec0bac2b7382955b11ee`

Verdict: `HOLD / NO-GO`.

The review used a source archive created from the exact reviewed Git object.
Concurrent work began in the shared checkout during the review, so no result
from those later bytes is used as evidence here.

## Findings

### Critical — The receipt-free replay discards its final store and WB14 continuation instead of joining them to persistent ending state

The remediation independently replays the physical partition, but the replay's
authoritative final state does not survive into validation:

- The frozen partition operand contains only the beginning cumulative supply
  and infiltration (`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:140-147`).
- The projector initializes those values from the frozen operand at
  `surface_liquid_closure.rs:1204-1205` and advances them with the shared WB14
  transition at `:1281-1301`. Both final values leave scope at the end of the
  OFE loop without comparison.
- The independently advanced `store_liquid` map is initialized at
  `surface_liquid_closure.rs:1137-1159` and updated through retained children at
  `:1634-1647`, but is likewise discarded.
- `ParcelArithmeticProjection` contains only expected/actual parcel and OFE
  enthalpy maps (`surface_liquid_closure.rs:500-506`, `:1687-1696`). It exposes
  neither expected ending stores nor expected ending continuations.

The separate store equation is not an independent join to that replay. Its
`retained_excess_kg_m2_ofe_ground` operand is captured by summing producer
receipts at `surface_liquid_closure.rs:727-755`, and
`validate_store_equations()` reconstructs W1 from that producer-derived scalar
at `:1961-2013`. Thus the receipt-free final `store_liquid` is never compared
with the ending owner.

Production publishes the advanced cumulative supply and infiltration, day,
next interval, and transaction lineage into `ending.continuations` at
`surface_liquid_ingress.rs:704-717`. Candidate E009 validation reruns the same
producer and compares its complete ending state at `surface_liquid_ingress.rs:359-385`
and `:430-435`. That catches a post-construction mutation, but it cannot catch a
producer defect that consistently writes a wrong yet valid final store or
continuation. The owner validator at
`surface_liquid_owner.rs:1096-1152` checks cardinality, identity, lineage and
domain relations; it does not reconstruct the numerical WB14 transition.

A concrete accepted-defect class is a producer that emits the correct receipt
partition but writes a finite wrong cumulative supply or infiltration that
still satisfies `0 <= infiltration <= supply`, then recomputes its state
digest. Producer replay repeats the defect. The independent closure computes
the correct continuation, discards it, and therefore cannot reject it. The
same defect class exists for a wrong final tile store paired with a
producer-derived retained operand. Either error silently corrupts restart and
the next interval.

This violates Section 8 and `INV-SURFACELIQUID-002,006,008` of
`SC-SURFACELIQUID-001`: ending stores and the exact once-per-interval WB14
continuation must be independently reconstructed and bound to the strict
ending state and digest.

Required correction:

1. Return the replayed final store for every exact OFE/tile/surface/source key
   and the replayed cumulative supply/infiltration for every OFE.
2. Independently construct the expected continuation identity, day, next
   interval, transaction lineage and cardinality, including interval-48/day
   rollover.
3. Compare those values with the actual ending owner before accepting its
   digest; do not substitute a retained total captured from producer receipts.
4. Add wrong-but-valid ending-store, cumulative-supply, cumulative-
   infiltration, rollover, lineage, missing/duplicate/reordered/wrong-OFE and
   forged-self-consistent-operand poisons. Exercise nonterminal, terminal,
   multi-OFE and routed multi-hop intervals and assert rollback hashes.

### High — Partition-input identity/cardinality is misclassified as E003, and zero-supply paths evade complete operand-domain precedence

`preflight_surface_liquid_closure_arithmetic()` returns E003 for every frozen
partition-input cardinality mismatch at
`surface_liquid_closure.rs:1810-1820`, using the first configured OFE as
context. It also combines wrong OFE identity with numeric-domain predicates at
`:1822-1845` and reports the actual, potentially invalid OFE.

That preflight runs before immutable candidate reconstruction and immediately
returns every E003 (`surface_liquid_ingress.rs:337-358`). Consequently missing,
extra, reordered, and wrong-OFE partition rows cannot reach the structural E009
comparison at `surface_liquid_ingress.rs:451-460`. The latter uses the
positional `first_partition_input_mismatch()` implementation at
`surface_liquid_closure.rs:200-217`, which would also report the shifted row
rather than the exact missing member for a nonterminal deletion.

This is typed-taxonomy and diagnostic-contract drift. The contract's guard
table assigns E003 to numerical domain violations, identity to E002, and
candidate reconstruction mismatch to E009. A structurally changed frozen
partition row is not a nonfinite/out-of-domain number. The current precedence
also makes the dedicated E009 structural path unreachable for this class and
can attach an inexact OFE context.

The E003 preflight checks each cumulative value only for finiteness and
nonnegativity (`surface_liquid_closure.rs:1834-1837`). It does not check
`beginning cumulative infiltration <= beginning cumulative supply` or the
applicable infiltration-storage bound. An active window normally lets the
shared WB14 transition reject such input, but the projector skips that call
when supply is exactly zero (`surface_liquid_closure.rs:1267-1269`). A
coordinated malformed zero-supply operand can therefore fall through to E009
instead of the earlier domain error.

Required correction: separate structural identity/cardinality from numeric
domain validation; use a membership-aware mismatch that reports the exact
expected missing OFE; validate all cross-field beginning-continuation bounds
before the zero-supply branch; and add combined E003/E009/E010 precedence,
context and rollback tests for first/middle/last deletions, additions,
reordering, wrong OFE and zero-supply inconsistent carries.

### Medium — Intentional production/projector duplication still leaves shared constants and ordering seams exposed to science drift

The independent expected projector necessarily cannot call the production
allocator, but it now mirrors a substantial chronological allocation,
retention and routing algorithm. The production contribution order is defined
by `parcel_order()` at `surface_liquid_ingress.rs:1908-1915`; the projector
repeats the field comparator at `surface_liquid_closure.rs:1238-1246`.
Production uses named water-density and enthalpy constants at
`surface_liquid_ingress.rs:24-27`, while the projector repeats the raw
dimensional literals `1_000.0`, `273.15`, and `4_218.0` at
`surface_liquid_closure.rs:1272`, `:1303`, `:1860-1861`, and `:2202-2203`.

The high-level arithmetic duplication is justified by the required
anti-tautology boundary, but the duplicated canonical constants, identity key
construction and ordering definition are not independent science authorities.
They can drift silently, as the earlier routed-kind defect demonstrated.
Centralize those non-arithmetic definitions while retaining separate
production and expected allocation implementations, and record that boundary
as the explicit justification for the remaining duplication.

The current routed-kind poison mutates an already constructed receipt, and the
multi-hop test begins with only one raw source. Missing evidence remains for a
destination window containing multiple routed source kinds plus downstream-
local supply, unequal temperatures/amounts/areas, and order-sensitive parcel
IDs. Add that vector and prove canonical `UpstreamRunon` conversion, stable
chronological `h_mix,b` bits, proportional attribution, final-remainder
ownership and multi-hop area conversion.

## Closed Historical Findings And Retained Correctness

- The expected side is now receipt-free. Actual receipts are accumulated only
  into the comparison map at `surface_liquid_closure.rs:1101-1120`; expected
  construction from frozen sources begins at `:1122` and independently replays
  partition, retention and routing through `:1650`.
- Frozen partition inputs include exact OFE, conductivity, matric potential,
  storage capacity and beginning cumulative supply/infiltration. Each active
  chronological window calls the shared authoritative WB14 transition and
  updates the independent carry.
- `ParcelJoinKey` binds owner, source parcel, origin store, current/recipient
  store, the complete typed recipient, basis OFE, kind, exact support bits and
  disposition (`surface_liquid_closure.rs:445-456`). Coordinated owner/
  recipient swaps and cross-tile retargeting are covered.
- Independently routed descendants preserve source/origin lineage, re-key the
  destination/current store and basis OFE, and become canonical
  `UpstreamRunon` before downstream sorting
  (`surface_liquid_closure.rs:1588-1598`).
- Frozen raw source Q is bit-joined to mass times specific liquid enthalpy in
  the E003 preflight (`surface_liquid_closure.rs:1848-1883`). Source and receipt
  support, mass and temperature domains are also checked before closure
  comparison.
- Production `h_mix,b` arithmetic is unchanged by this remediation. The
  independent replay computes one checked chronological
  `supply_enthalpy / supply_mass` at
  `surface_liquid_closure.rs:1314-1323` and assigns all children from it.
- Exact D/A/F custody, signed condensation, clone-only candidate construction,
  rollback hashes, receiving soil/thermal/LSE identities, unequal-area
  routing, snow/frost rejection and default-off selection remain intact in the
  reviewed bytes. Focused failure tests retain byte-identical rollback.
- All historical HOLD artifacts remain preserved. This review adds only this
  closure-8 Agent A artifact.

## Line-Count Governance

Exact reviewed counts are:

| File | Lines | Disposition |
|---|---:|---|
| `direct_runtime/runoff.rs` | 2,852 | WARN, below mandatory stop. |
| `direct_runtime/00_core_frames.rs` | 2,783 | WARN, below mandatory stop. |
| `direct_runtime/surface_liquid_owner.rs` | 2,347 | WARN, below mandatory stop. |
| `direct_runtime/surface_liquid_owner_tests.rs` | 876 | PASS. |
| `direct_runtime/surface_liquid_ingress.rs` | 1,952 | PASS. |
| `direct_runtime/surface_liquid_ingress_tests.rs` | 1,983 | PASS. |
| `land_surface_energy_shadow/mod.rs` | 2,881 | WARN, below mandatory stop. |
| `direct_runtime/surface_liquid_closure.rs` | 2,300 | WARN, below mandatory stop. |
| `direct_runtime/surface_liquid_wb14.rs` | 303 | PASS. |
| `vegetation_real_hydrology_shadow.rs` | 2,157 | WARN, below mandatory stop. |

No affected file reaches the 3,000-line blocker. The package line-count
artifact records decomposition rationale and a future campaign split of frozen
operand projection from comparison/diagnostics for the newly enlarged closure
module. That satisfies WARN governance; it does not resolve the duplicated
definition risk above.

## Exact-Commit Validation

Ran from a source archive generated from
`298acedbb47455d5ce54ec0bac2b7382955b11ee`:

```text
CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo nextest run \
  -p openwepp-hillslope-orchestrator --profile quick
PASS: 559/559; 0 failed

CARGO_TARGET_DIR=/home/workdir/openWEPP/target cargo clippy \
  -p openwepp-hillslope-orchestrator --all-targets --all-features -- \
  -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check \
  bf7210ea1238ac12adf4aef77416141d7717570e...\
  298acedbb47455d5ce54ec0bac2b7382955b11ee
PASS
```

These gates prove the focused receipt partition, typed child identity,
rollback and compile/lint behavior present in the exact bytes. They do not
contain independent-ending-state poisons or the partition-input taxonomy and
multi-kind ordering vectors described above. Full-workspace, doctest,
dependency-policy and release/campaign gates were not rerun after the static
closure blockers were established.

## Residual Risk And Missing Tests

Until the critical join is added, successful acceptance does not prove that
the persistent W1 or restart continuation equals the independent physical
replay. Until the high finding is corrected, malformed frozen structural and
zero-supply continuation inputs can expose the wrong code, precedence and OFE
context. The duplicated definition seams remain comparator-sensitive without
the requested multi-kind ordering vector.

No other new arithmetic, clamp/guard-precedence, unit-conversion, serialization,
rollback or receiving-owner defect was found in the exact reviewed bytes.

## Approval Statement

`NO-GO`: commit `298acedbb47455d5ce54ec0bac2b7382955b11ee`
closes the accepted closure-7 receipt, identity, routed-kind, raw-Q and domain
findings, but it does not independently bind the replayed ending store and
WB14 continuation to persistent state. Correct that closure blocker and the
partition-input error taxonomy, add the named poison/order vectors, then rerun
focused restart, rollback, receiver and exact-byte terminal gates.
