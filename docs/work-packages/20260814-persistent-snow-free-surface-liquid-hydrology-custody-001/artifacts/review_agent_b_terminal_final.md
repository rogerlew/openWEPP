# Review Agent B Terminal Final — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `1313bf6bdc2913f719a36efbcd01c2b30ea4edce`

Verdict: `HOLD / one material error-precedence regression / no authority HOLD`.

This fresh review preserves every earlier review artifact and assesses the
exact post-remediation bytes. It specifically rechecked the prior terminal
arithmetic finding, public error precedence, persistent custody, immutable
snapshot arbitration, ingress and enthalpy routing, the shared WB14
continuation, receiver reconstruction, rollback, and production exclusion.

## Material finding

### B-TERMINAL-FINAL-HIGH-001 — independent closure now preempts the canonical E009 candidate-attribution failure

`DirectSurfaceLiquidIngressCandidate::validate()` now invokes
`validate_surface_liquid_closure_operands()` before reconstructing the producer
candidate from immutable inputs
(`direct_runtime/surface_liquid_ingress.rs:331-371`). This ordering corrected
the prior loss of E003 for arithmetic indeterminacy, but it also lets an
ordinary independent-closure mismatch preempt the earlier canonical candidate
failure.

The retained wrong-infiltration-recipient poison demonstrates the regression
directly. It changes the production-lane attribution on a receipt. The prior
test correctly expected E009; the remediation changes the assertion to E010
and `IndependentClosure`
(`direct_runtime/surface_liquid_ingress.rs:2220-2257`). The canonical branch
table is explicit:

```text
9 | capacity, attribution, routing, or parcel enthalpy mismatch
  | candidate closure | SURFACELIQUID-E-009
10| local/owner/soil join closure failure
  | independent closure | SURFACELIQUID-E-010
```

The wrong production-lane recipient is an attribution mismatch, so publishing
E010 violates both the named E009 boundary and the stated error precedence.
The same ordering can misclassify other producer-owned capacity, routing, or
parcel-enthalpy mismatches that are also visible to the independent
reconstruction.

Required correction:

1. Preserve E003 precedence without running all ordinary E010 comparisons
   before producer reconstruction. A bounded domain/arithmetic preflight may
   detect checked-arithmetic `None` first; finite producer/candidate
   reconstruction mismatches must then return E009; only an otherwise valid
   producer candidate may proceed to independent E010 closure.
2. Restore the wrong-recipient poison to exact E009 and
   `IngressCandidate`, retaining its transaction, owner and rollback hashes.
3. Add or retain separate poisons proving:
   - large-finite checked-comparison indeterminacy returns E003;
   - finite producer attribution/routing/enthalpy mismatch returns E009; and
   - a producer-valid but independently inconsistent join returns E010.

This is a bounded implementation-ordering defect. It requires no authority
amendment, new model identity, tolerance, clamp, fallback, or new package.

## Prior arithmetic finding closure

The earlier `A/B-TERMINAL-REREVIEW-HIGH-001` mechanics are otherwise closed:

- `require_close_mass()` and `require_close_enthalpy()` retain the checked
  tri-state: `Some(true)` passes, `Some(false)` returns E010, and `None`
  returns contextual E003.
- `require_receiver_close()` independently retains `None -> E003` and
  `Some(false) -> E011`, with exact transaction, hydrology owner, OFE/tile
  where available, and beginning/attempted hashes.
- production-lane infiltration depth, frozen OFE infiltration depth,
  infiltration enthalpy, and retained enthalpy accumulation now use checked
  division/addition. No cited raw receiver `+=` remains.
- public producer large-finite arithmetic and independent receiver
  overflow/underflow cases assert E003 and exact rollback hashes.

The focused suite does not directly force overflow inside the private
multi-receipt receiver aggregation maps, but static inspection confirms each
named division and accumulation is checked and returns contextual E003 before
candidate publication. This is a residual test-depth opportunity, not a
second material implementation finding.

## Retained hydrology and custody correctness

- One immutable beginning snapshot drives one authorization. Typed
  transaction/OFE/tile/surface/source identities remain exact and all normal
  branches retain `0 <= F <= A <= D`; finalized use alone debits storage and
  unused authorization remains.
- Signed condensation remains a resource-phase credit before capacity
  overflow. Rain, canopy release, runon and overflow remain post-authorization
  ingress and cannot fund the same authorization pass.
- Parcel mass and enthalpy preserve source, recipient, temperature, area and
  route identity through infiltration, retention, routed runoff and outlet
  runoff. Checked tile/OFE conversion and unequal-area routing remain intact.
- The shadow continues to use the single shared complete WB14 continuation;
  it does not introduce a second infiltration or runoff transition.
- Actual production-soil, soil-thermal and retained-LSE candidates remain
  clone-only and independently reconstruct ordered-layer water, aggregate
  residual/unfrozen water, infiltration enthalpy and retained enthalpy.
- Snow/frost/frozen/thaw entry remains contextual E004 before authorization or
  callback. Exact three-owner rollback and receiver-set identity remain
  fail-closed and byte-preserving.
- Normal production construction still selects no surface-liquid shadow.
  No runner selector, production scheduler path, default, output publication,
  runtime activation or cutover was introduced by the reviewed diff.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 37/37 selected; 507 skipped by filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

These passing gates confirm the current implementation and its encoded
expectations. They do not make the changed E010 expectation canonical when the
binding contract requires E009.

## Approval statement

`NO-GO`: exact commit `1313bf6bd` is not ready for dependency-package closure.
The prior E003 and receiver-aggregation implementation defect is closed, and
the persistent custody, D/A/F, signed condensation, ingress/enthalpy routing,
WB14, receiver, rollback and nonactivation behavior remains materially sound.
Correct the E009-before-E010 producer/independent-closure ordering in this
package, rerun its focused precedence poisons, and obtain fresh exact-byte Rust
and hydrology approval.
