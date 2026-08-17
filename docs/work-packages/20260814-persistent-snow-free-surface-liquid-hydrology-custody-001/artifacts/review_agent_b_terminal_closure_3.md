# Review Agent B Terminal Closure 3 — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `e55bab15b84301b6bc8649dd2903c714da13726e`

Verdict: `HOLD / one material per-source enthalpy closure omission / no authority HOLD`.

This fresh exact-byte review preserves all prior artifacts. It audits the new
shared parcel projection against every arithmetic and finite-comparison surface
in the final validator, then rechecks global E003 precedence, E009
deletion/replacement attribution, E010 disposition, D/A/F, ingress, WB14,
receivers, rollback/restart, and production exclusion.

## Material finding

### B-TERMINAL-CLOSURE3-HIGH-001 — independent closure permits cross-parcel enthalpy substitution within one OFE

`project_parcel_arithmetic()` now correctly constructs shared expected and
actual `AmountPair` values keyed by `(source_parcel_id, basis_ofe_id)` and
shared expected/actual enthalpy totals keyed by OFE. The arithmetic preflight
uses the per-source projection for both mass and enthalpy comparison arithmetic
and now checks the formerly omitted OFE aggregate comparison.

The final finite validator does not apply the same per-source enthalpy join.
For each source key it calls only `require_close_mass()`
(`direct_runtime/surface_liquid_closure.rs:1088-1100`). It then sums all source
enthalpy within the OFE and calls one aggregate `require_close_enthalpy()` at
lines 1101-1119.

This is weaker than the binding requirement:

```text
For every source parcel independently reconstruct mass and enthalpy across
infiltration, retention, routed runoff, and outlet runoff.
```

Two same-OFE source parcels can retain exact individual mass and exact OFE
total enthalpy while exchanging equal-and-opposite enthalpy. Each receipt can
remain internally temperature/enthalpy-consistent, every typed recipient and
mass join can pass, and the OFE aggregate can close, yet neither source
parcel's enthalpy reconstructs from its frozen operand. The current independent
validator accepts that substitution because the per-key enthalpy
`Some(false)` result is ignored by the arithmetic-only preflight and never
evaluated by the final closure.

Producer reconstruction E009 is not a substitute for this independent
conservation check. It repeats the producer candidate and cannot establish the
anti-tautological source-parcel enthalpy ledger required by
`INV-SURFACELIQUID-008`.

Required correction:

1. Beside each per-key mass join, require
   `actual.enthalpy == expected.enthalpy` with the admitted enthalpy closure
   rule, exact source-parcel identity, and E010 for a finite mismatch.
2. Retain the OFE aggregate enthalpy join as the independent owner-total
   closure; it is additional evidence, not a replacement for per-source
   closure.
3. Add a poison with two same-OFE source parcels having distinguishable
   temperatures/enthalpies and compatible masses. Exchange their enthalpy
   custody while preserving receipt self-consistency and the exact OFE total.
   The independent validator must return E010 naming the first wrong source
   parcel; no candidate may publish and beginning bytes must remain exact.
4. Retain an arithmetic version proving a per-source enthalpy comparison
   `None` remains global E003 before E009/E010.

This is a bounded implementation/test defect in the existing shared
projection consumer. It requires no authority amendment, new model identity,
tolerance change, clamp, fallback, or new package.

## Correctly closed portions of the remediation

- The shared projection now supplies both the arithmetic preflight and final
  validator. Expected and actual OFE enthalpy totals are accumulated once and
  the preflight performs their checked comparison, closing
  `B-TERMINAL-CLOSURE2-HIGH-001`.
- The nondegenerate same-OFE aggregate poison proves that individual
  comparison scales can remain finite while aggregate comparison arithmetic
  returns contextual E003 before simultaneous producer E009 and finite E010
  defects.
- E009 localization is membership-aware for state records, continuation rows,
  receipt IDs, ledger OFEs, and WB14 map keys. Upper/middle deletion reports
  the missing expected identity; an equal-length replacement reports the
  actual first replacement rather than a shifted or first-record fallback.
- The wrong infiltration recipient remains E009 in `IngressCandidate` with
  exact recipient and rollback context.
- Finite independent store and aggregate mismatches remain last-stage E010;
  checked-comparison indeterminacy remains E003.

## Retained hydrology and custody correctness

- One immutable beginning snapshot supplies one authorization. Typed
  transaction/requester/OFE/tile/surface/source identity and normal
  `0 <= F <= A <= D` bounds remain exact; finalized use alone debits water and
  unused authorization remains.
- Signed condensation remains a resource-phase credit before capacity
  overflow. Rain, canopy release, runon, and overflow remain post-authorization
  ingress and cannot satisfy same-interval demand.
- Ingress and routing retain parcel time, temperature, mass, enthalpy, source,
  recipient, basis, and unequal-area conversion identity. The finding concerns
  the independent finite source-enthalpy join, not producer routing arithmetic.
- Receiver depth and enthalpy conversion/aggregation remain checked.
  Production-soil, soil-thermal and retained-LSE candidates remain clone-only
  and independently reconstruct their ending operands.
- The bridge retains the one shared complete WB14 continuation and introduces
  no second infiltration/runoff transition.
- Persistent state/configuration digests, exact transaction lineage, interval
  continuation, deterministic restart, exact three-owner rollback, and
  byte-identical failure isolation remain intact.
- Snow, retained snow liquid, frost, frozen-layer, and thawing entry remains
  contextual E004 before authorization or callback.
- No runner selector, default dispatch, production scheduler, output
  publication, runtime activation, or cutover path was introduced. Normal
  production constructors still disable the shadow.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 41/41 selected; 507 skipped by filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

These gates confirm the encoded aggregate-precedence and E009 identity cases.
They do not contain a cross-source enthalpy-substitution poison.

## Approval statement

`NO-GO`: exact commit `e55bab15b` closes the previously identified aggregate
comparison omission, global E003 example, and deletion attribution defects,
while retaining D/A/F, ingress, checked receivers, WB14, rollback/restart, and
production exclusion. Dependency-package closure remains blocked because the
independent ledger still closes enthalpy only after aggregating distinct source
parcels, contrary to the canonical per-source mass-and-enthalpy identity.
