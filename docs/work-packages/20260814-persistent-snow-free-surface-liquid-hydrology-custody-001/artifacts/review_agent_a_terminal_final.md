# Review Agent A — Terminal Final Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained Ran evidence`

Reviewed commit: `1313bf6bdc2913f719a36efbcd01c2b30ea4edce`

Verdict: `HOLD / NO-GO`.

## Findings

### High — Independent E010 closure now preempts the contract-precedent E009 candidate-attribution failure

`DirectSurfaceLiquidIngressCandidate::validate()` now calls
`validate_surface_liquid_closure_operands()` before reconstructing the expected
producer candidate from immutable configuration, resource, and ingress inputs
at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:331-371`.
This ordering preserves E003 when the retained closure operands contain
indeterminate arithmetic, but it also lets an ordinary independent-closure
mismatch return E010 before candidate reconstruction can return E009.

The retained wrong-infiltration-recipient poison demonstrates the regression
at `surface_liquid_ingress.rs:2220-2257`. It changes the typed production-lane
recipient on a sealed receipt. Before this remediation, the immutable producer
reconstruction detected that forged attribution and the test asserted E009 in
the ingress-candidate phase. The remediation changes the encoded expectation
to E010 in the independent-closure phase.

SC-SURFACELIQUID-001's branch table is explicit and ordered: capacity,
attribution, routing, or parcel-enthalpy candidate mismatch is E009; only the
later local/owner/soil independent join closure is E010. A wrong production
lane on a producer receipt is an attribution mismatch. Running all independent
finite comparisons before reconstructing the producer reverses the required
E009-before-E010 precedence and can similarly preempt producer-owned routing
or parcel-enthalpy mismatch classification.

Required correction: perform only a bounded arithmetic/domain preflight before
producer reconstruction so checked indeterminacy remains E003. Reconstruct and
compare the complete candidate next so finite producer attribution, routing,
capacity, and enthalpy mismatch remains E009. Run ordinary independent E010
join closure only after the producer candidate matches. Restore the wrong
recipient poison to exact E009 and `IngressCandidate`, with its transaction,
owner, beginning hash, and attempted hash. Retain separate controls proving
arithmetic indeterminacy is E003 and a producer-valid independent mismatch is
E010.

## Prior Finding Closure And Retained Correctness

The prior A/B terminal arithmetic-precedence finding is otherwise materially
closed:

- Repository-wide Rust search finds exactly two production consumers of
  `checked_surface_liquid_close()`. `require_close_mass()` and
  `require_close_enthalpy()` at
  `direct_runtime/surface_liquid_closure.rs:976-1038` preserve
  `Some(true) -> success`, `Some(false) -> contextual E010`, and
  `None -> contextual E003`.
- `require_receiver_close()` at
  `land_surface_energy_shadow/mod.rs:2342-2359` independently preserves
  `Some(false) -> E011` and `None -> E003`; both paths retain transaction,
  hydrology owner, OFE/tile where available, and exact beginning/attempted
  hashes.
- The previously cited raw production-lane accumulation at
  `land_surface_energy_shadow/mod.rs:1224-1241` now uses checked addition and
  returns contextual E003 before production infiltration.
- The previously cited independent OFE-depth and tile-enthalpy reconstruction
  at `land_surface_energy_shadow/mod.rs:1847-1928` now uses checked
  mass-to-depth division and checked accumulation throughout. Repository
  search finds no remaining raw `+=` or water-density division in the receiver
  bridge.
- Public producer and receiver large-finite overflow/nonzero-underflow poisons
  assert E003. The producer poison also asserts exact transaction,
  owner/OFE/tile, beginning-state hash, and attempted-state hash; receiver
  poisons assert the exact unified beginning hash and recomputed attempted
  operand hash. Finite mismatch controls retain E010/E011.

Static inspection confirms that the arithmetic remediation changes no
tolerance, unit, source operand, authority, model identity, or production
selector. The previous nonterminal expectation-deletion fix also remains
correct: membership detects the missing expected identity, while equal-length
replacement/reorder retains the actual first mismatch.

The remaining custody domains are unchanged and materially sound:

- strict canonical configuration/state bytes, digests, continuation, restart
  combinations, predecessor lineage, and sealed candidates remain intact;
- authorization is reconstructed from one immutable beginning snapshot with
  exact request/authorization/use identity and `0 <= F <= A <= D`; same-store
  demand and proportional-allocation arithmetic remains checked;
- signed condensation remains a resource credit before capacity overflow, and
  current rain, canopy release, runon, and overflow remain unavailable to the
  same authorization pass;
- parcel mass/enthalpy, tile/source custody, retention, outlet runoff, and
  once-only unequal-area routing remain independently reconstructed;
- production and persistence still use the one centralized complete WB14
  continuation, with no parallel Green-Ampt implementation;
- actual production-soil, soil-thermal, and retained-LSE mutation remains
  clone-only and independently reconstructs ordered layers, residual/unfrozen
  aggregate water, infiltration enthalpy, and retained enthalpy;
- snow/frost/frozen-state E004 preflight, exact three-owner rollback, and
  contextual structural E011 identity remain fail-closed; and
- ordinary production constructors still select no surface-liquid shadow, and
  no runner, scheduler, publication, activation, or cutover consumer was added.

Line-count governance remains compliant: `surface_liquid_owner.rs` is 2,347
lines, `surface_liquid_ingress.rs` 2,547, `surface_liquid_closure.rs` 1,038,
`land_surface_energy_shadow/mod.rs` 2,881, and `runoff.rs` 2,852. No affected
production file exceeds 3,000 lines and no lint suppression was added. The
shared checked arithmetic is centralized; intentional producer/independent
reconstruction remains separate to avoid self-referential closure.

## Exact-Commit Validation

Ran at `1313bf6bdc2913f719a36efbcd01c2b30ea4edce`:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 37/37 selected; 507 skipped by filter

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
PASS: 544/544; 0 skipped; three known slow OFE-routing oracle tests completed

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS
```

These gates confirm the current implementation and its encoded poisons. They
cannot make the newly changed E010 assertion authoritative when the canonical
contract requires E009 to precede E010.

## Residual Risk And Missing Tests

- Restore a three-way public precedence matrix: checked-comparison
  indeterminacy -> E003, finite producer attribution mismatch -> E009, and
  producer-valid independent join mismatch -> E010.
- Add a public comparison-scale poison whose expected equation stays finite;
  the current helper test and static call-site audit prove the tri-state path,
  while the public overflow poisons primarily fail during expected-equation
  construction.
- A focused private aggregation poison would strengthen localization for the
  newly checked multi-receipt receiver maps. Static inspection establishes
  checked failure before candidate publication.
- The retained full-workspace 2,783/2,783 run at `74d512f44` predates the two
  arithmetic remediation commits. Exact-head full-workspace, doctest, and
  dependency-policy gates remain terminal closure evidence after correction.

## Approval Statement

`NO-GO`: exact commit `1313bf6bd` fully closes the previously reported
tri-state and raw receiver-arithmetic mechanics, and the retained science,
custody, rollback, serialization, and production-exclusion domains remain
sound. Dependency-package closure is still blocked because candidate
validation now publishes E010 before the contract-precedent E009 for a finite
producer attribution mismatch.
