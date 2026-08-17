# Review Agent B Terminal Closure — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained Ran evidence`

Reviewed commit: `63d3edde677409d9bb9170129528a9941513653a`

Verdict: `PASS / GO / no unresolved material hydrology, custody, or science finding`.

This fresh review preserves all prior review and failed-attempt artifacts. It
reassesses the exact post-precedence-remediation bytes against the complete
surface-liquid custody contract, with particular attention to the previously
accepted E003/E009/E010 ordering defect, immutable-snapshot D/A/F, checked
receiver arithmetic, ingress and enthalpy routing, WB14 continuation,
independent receivers, rollback, and production exclusion.

## Prior finding closure

`B-TERMINAL-FINAL-HIGH-001` is materially closed.

`DirectSurfaceLiquidIngressCandidate::validate()` now implements the required
three-stage distinction:

1. The independent operand pass is used first only to surface checked
   arithmetic indeterminacy as contextual E003.
2. Producer-owned candidate fields are reconstructed from immutable inputs.
   A finite capacity, attribution, routing, receipt, state, ledger, or WB14-call
   mismatch returns E009 in `IngressCandidate`.
3. Only after producer fields match does the complete independent validation
   publish a finite E010 join mismatch.

The producer equality deliberately excludes the independently frozen closure
operands. Consequently a producer-valid poison in those independent operands
still reaches E010, while it cannot disguise a receipt or producer-state
mismatch as independent closure.

The wrong-infiltration-recipient poison is restored to exact E009 and
`IngressCandidate`. It asserts the transaction, surface-liquid owner, actual
recipient OFE/tile, parcel ID, beginning-owner hash, and attempted-owner hash.
Separate public poisons retain:

- large-finite comparison/arithmetic failure as E003;
- wrong producer attribution as E009; and
- finite independent source-operand mismatch as E010.

No tolerance, operand, constitutive equation, WB14 behavior, model identity,
or owner boundary changed in this correction.

## Checked arithmetic and receiver closure

- Mass and enthalpy comparison callers preserve the complete tri-state:
  `Some(true)` passes, `Some(false)` returns the applicable E010/E011, and
  `None` returns contextual E003.
- Production-lane and OFE infiltration depth, infiltration enthalpy, retained
  enthalpy, mixed debit, and authorization-rate conversion use checked
  arithmetic. The receiver bridge contains no cited raw depth/enthalpy
  accumulation from the earlier finding.
- Receiver E003 failures carry transaction, hydrology owner, OFE/tile where
  available, and exact beginning/attempted hashes. Finite receiver equation
  mismatches remain E011.
- Store, parcel, routing, production-layer, aggregate-water, soil-thermal, and
  retained-LSE equations are reconstructed from operands rather than a
  producer-supplied residual.

No arithmetic failure can construct an infinite tolerance and accept a wrong
finite value. No representation tolerance repairs an identity, owner, basis,
request/authorization/use, routing, or receiver mismatch.

## Hydrology and custody assessment

- Persistent state remains strict and restart-representable per
  run/OFE/tile/surface/class/source, with exact configuration/state digests,
  accepted transaction lineage, interval index, and WB14 continuation state.
- One immutable beginning owner snapshot supplies one authorization batch.
  Typed transaction/requester/OFE/tile/source identity is preserved and the
  accepted branches enforce `0 <= F <= A <= D`. Only finalized use debits the
  store; unused authorization remains.
- Signed condensation is credited during the resource phase before capacity
  overflow. Rain, canopy liquid release, runon, and overflow remain ingress
  after authorization and cannot satisfy same-interval demand.
- Parcel mass and enthalpy preserve temperature, source, recipient, area and
  route identity through infiltration, retention, routed runoff, and outlet
  runoff. Tile/OFE conversion occurs exactly once, including unequal-area
  routing.
- The bridge retains the single shared complete WB14 continuation for the
  production infiltration/runoff transition. No copied or parallel hydrology
  partition was introduced.
- Production-soil, soil-thermal and retained-LSE work remains clone-only.
  Their ending candidates independently reconstruct ordered layers,
  residual/unfrozen aggregate water, infiltration enthalpy, and retained
  surface enthalpy.
- Snow, retained snow liquid, frost, frozen-layer and thawing entry remains
  contextual E004 before authorization or callback.
- The receiver envelope retains the exact LSE, hydrology and soil-thermal
  owner set. Failure preserves beginning owner bytes and reports exact
  receiver/rollback identity.

## Production exclusion

The reviewed production diff is confined to ingress candidate validation and
package evidence. Normal production construction still sets the optional
surface-liquid shadow to absence. No new reference from the runner, selector,
default dispatch, production scheduler, output publication, activation, or
cutover path exists. The dependency lift remains default-off shadow authority
only.

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

The retained exact-head heavy evidence at `74d512f44` remains truthful for its
bytes: workspace strict all-target/all-feature Clippy, 2,783/2,783 full
Nextest, workspace doctests, dependency policy, formatting, and diff hygiene
passed. Later corrections are bounded to checked custody arithmetic,
diagnostic precedence, tests, and package evidence; campaign terminal closure
must still run any exact-final-byte gates required by package governance.

## Approval statement

`GO`: exact commit `63d3edde6` closes the accepted ingress precedence defect
without weakening E003 arithmetic guards or E010 independent closure. The
persistent hydrology custody, D/A/F, signed condensation, ingress and enthalpy
routing, WB14 continuation, independent receiving owners, rollback, restart,
and production-exclusion claims reviewed here have no unresolved material
finding. This review approves the dependency package for terminal verification
and subsequent truthful lifecycle disposition; it authorizes no production
activation or cutover.
