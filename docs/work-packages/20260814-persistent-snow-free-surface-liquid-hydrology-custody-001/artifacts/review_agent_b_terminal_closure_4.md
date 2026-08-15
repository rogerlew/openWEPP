# Review Agent B Terminal Closure 4 — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `9339b55c637c266c765b7e461ab03d89dae04317`

Verdict: `HOLD / one critical canonical mixing regression / no new authority package indicated`.

This comprehensive exact-byte review preserves all prior artifacts. It traces
the source-parcel, OFE aggregate, routed destination, store, producer,
receiver, and rollback joins through the public candidate and compares the
latest production arithmetic directly with the complete canonical
surface-liquid contract.

## Material finding

### B-TERMINAL-CLOSURE4-CRITICAL-001 — the parcel-ledger remediation replaces mandatory well-mixed enthalpy with source-specific temperatures

The latest production change removes the interval `supply_enthalpy` sum,
`h_mix = Q_b/X_b`, and one mixed interval temperature. It now computes
`specific_enthalpy = enthalpy/mass` separately for each source contribution
and uses that source-specific value for infiltration
(`direct_runtime/surface_liquid_ingress.rs:1327-1451`).
`retain_excess_proportionally()` repeats the source-specific calculation and
uses it for retained water and runoff at lines 1598-1725.

That is the exact opposite of the admitted Section 6 and Section 7 equations:

```text
X_b = sum_p(x_p,b)
Q_b = sum_p(q_p,b)
h_mix,b = Q_b / X_b
Q_infiltration,b = I_b * h_mix,b

Every attributed parcel uses h_mix,b, so source kind cannot select which
temperature infiltrates or remains.

Every retained receipt uses h_mix,b.
Q_runoff,k = m_runoff,k * h_mix,b.
```

The changed code makes a warm source contribute warm infiltration/retention
and a cold source contribute cold infiltration/retention. Source kind and
canonical source order therefore select thermal custody even though WB14
operates on their one mixed mass supply. This changes accepted infiltration
enthalpy, soil-thermal credit, retained-LSE enthalpy, runoff enthalpy, parcel
temperatures, and downstream runon energy for any nondegenerate
mixed-temperature interval.

The new per-source enthalpy closure is internally consistent only because it
also treats each raw pre-mix source enthalpy as if it must remain attached to
that source after mixing. The contract instead preserves source **mass**
provenance while assigning every attributed child the common `h_mix,b`; raw
input enthalpy is conserved at the mixed interval/OFE control volume. The
Section 8 per-source reconstruction must therefore close the post-mix
attributed parcel (`m_child * h_mix,b`), not bypass the explicit mixing
equations by retaining `q_p/x_p`.

This is a constitutive production regression, not merely a diagnostic or test
gap. The current focused tests do not contain a positive assertion that two
simultaneous unequal-temperature sources produce the same admitted mixed
temperature across infiltration, retained water, and runoff.

Required correction:

1. Restore the one checked chronological-subinterval calculation of
   `Q_b=sum(q_p,b)`, `h_mix,b=Q_b/X_b`, and the corresponding mixed
   temperature.
2. Use that exact `h_mix,b` for every attributed infiltration, excess,
   retained, runoff, and downstream-routed child in the subinterval. Preserve
   source mass provenance and canonical floating remainder allocation.
3. Retain the shared independent projection, but freeze or reconstruct the
   post-mix expected enthalpy for each `(source_parcel_id,basis_ofe_id)` as the
   attributed mass times the admitted `h_mix,b`. Separately close total raw
   input `Q_b` against total mixed output enthalpy so mixing cannot create or
   lose energy.
4. Add a nondegenerate positive vector with equal-mass warm and cold sources.
   Prove every same-subinterval attributed receipt has the common mass-weighted
   temperature, source order/kind cannot change it, total enthalpy closes, and
   per-source post-mix mass/enthalpy joins close.
5. Add poisons for source-specific temperature retention, source-priority
   infiltration temperature, pre-mix `q_p` used as post-mix expected custody,
   and mixed enthalpy applied twice.

The explicit canonical equations are sufficient to correct this defect; no
new model identity or authority package is indicated unless maintainers intend
to change those equations. Do not normalize the current implementation into
authority after the fact.

## Independent-ledger audit

Apart from the constitutive mixing regression, the latest closure architecture
materially closes the previously identified structural omissions:

- One shared store projection supplies preflight and final `W0-F+C-overflow`
  plus retained-ending arithmetic.
- One shared parcel projection supplies actual and expected mass and enthalpy
  keyed by `(source_parcel_id,basis_ofe_id)` and the OFE enthalpy totals.
- Final closure now compares both mass and enthalpy per projected key, followed
  by the independent OFE aggregate enthalpy join.
- Routed projections retain the current downstream `basis_ofe_id`; failure
  context resolves the destination store rather than the immutable origin.
  Typed routed recipients independently validate source OFE, destination OFE,
  and destination store.
- The arithmetic preflight consumes the same projections and comparison helper
  as final closure. `None` is global E003, producer mismatch is E009, and a
  finite independent mismatch is last-stage E010.
- Membership-aware producer attribution reports missing upper/middle expected
  rows for deletions and the actual first row for equal-length replacement or
  reorder, including exact rollback hashes.

The receipt DTO also contains producer-owned redundant kind, interval, and
current-store fields. Public immutable producer reconstruction covers those
fields through E009; no additional material independent-ledger omission was
found beyond the incorrect pre-mix versus post-mix enthalpy authority used by
the new projection.

## Retained hydrology and ownership behavior

- One immutable beginning snapshot supplies one authorization. Typed
  transaction/requester/OFE/tile/source identity and normal
  `0 <= F <= A <= D` bounds remain exact; only finalized use debits water and
  unused authorization remains.
- Signed condensation remains resource-phase credit before capacity overflow.
  Rain, canopy release, runon, and overflow remain post-authorization ingress.
- WB14 is still invoked once per OFE/subinterval through the shared complete
  continuation. The finding changes energy attribution around its mass result,
  not WB14 infiltration mass or continuation cadence.
- Receiver depth/enthalpy conversions remain checked. Production-soil,
  soil-thermal and retained-LSE candidates remain clone-only and independently
  reconstruct their received amounts, but the current energy amounts are
  scientifically wrong whenever source temperatures differ.
- Configuration/state digests, transaction lineage, interval continuation,
  deterministic restart, exact three-owner rollback, and failure isolation
  remain intact.
- Snow/frost/frozen/thaw entry remains contextual E004 before authorization or
  callback.
- Normal production constructors still disable the shadow. No runner selector,
  default dispatch, production scheduler, output publication, runtime
  activation, or cutover was introduced.

## Commands run at the exact reviewed commit

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 45/45 selected; 507 skipped by filter

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this review artifact was added
```

These gates prove the encoded source-specific implementation and its closure.
They cannot override the canonical well-mixed equations that implementation
replaced.

## Approval statement

`NO-GO`: exact commit `9339b55c6` closes the structural per-source and routed
projection gaps but does so by changing admitted production physics. Restore
the exact chronological well-mixed enthalpy rule, bind per-source closure to
post-mix attributed parcels, and rerun mixed-temperature, global-precedence,
receiver, rollback, and restart gates before another exact-byte closure review.
D/A/F, WB14 mass custody, checked receivers, rollback/restart, and production
exclusion remain materially sound; the accepted thermal state and energy
receipts do not.
