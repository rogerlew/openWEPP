# Shared C/N State Authority Hold-Legitimacy Audit

Status: `BLOCKED / affected E20--E22 shared-state transition only`

Evidence class: `Static` authority audit plus `Ran` unaffected focused gates.

## Exact Blocked Surface

The accepted V3 authority does not define two state identities precisely enough
to implement the persistent shared-stratum transition without inference:

1. `SC-VEGETATION-001` defines `LAI = leaf_C * SLA`, but does not identify
   `leaf_C` as the display, storage, transfer, or summed leaf-carbon subpool.
   The ordered onset/offset equations and independent calculator strongly imply
   display carbon, but implication is not executable constitutive authority.
2. `previous_leaf_offset_flux` and `previous_root_offset_flux` are mandatory
   persisted fields, but no canonical source defines their units, amount basis,
   update equation, cadence, initial semantics, or later consumer.

## Evidence Proving the Boundary

- `SC-VEGETATION-001.md` lines 512--548 separately create display/storage
  pools, move transfer carbon to display during onset, remove displayed donors
  during offset, and then use the undefined `leaf_C` symbol for LAI.
- `equation-authority-ledger.md` E19--E21 repeats the LAI expression without
  defining the owning leaf subpool.
- `reference_calculator.py` lines 511--533 tracks displayed and transfer pools
  independently but contains neither persisted previous-offset field.
- `parameter-and-configuration-manifest.md` requires both previous-offset
  fields without their numerical semantics.

The current Rust `tissue_carbon()` sum of display, storage, and transfer is an
implementation inference. Historical code that assigned donor transfer carbon
divided by `dt` to the previous-offset fields was likewise inferred and never
consumed. Neither route is admissible production science.

## In-Scope Correction Routes Attempted

- Complete search of the V3 contract, immutable V1/V2/V3 model definitions,
  equation ledger, parameter/state manifest, state-ownership ledger, and
  independent reference calculator.
- Cross-check of onset, offset, turnover, and LAI ordering for an exact implied
  identity.
- Historical implementation inspection to determine whether the unexplained
  fields had an authority-backed consumer.

These routes establish intended display-only LAI behavior but do not supply the
missing normative alias or previous-offset semantics. Implementing either
behavior in V3 would change constitutive state meaning without digest-bound
authority.

## Containment

- Do not implement or claim the affected shared-stratum E20--E22 finalizer.
- Keep the public transaction fail-closed.
- Continue independent in-scope work that does not consume the missing
  identities: occupancy radiation, E04 routing, E07--E15 coupled solves, typed
  water/N protocols, C/N pure-kernel hardening, and independent keyed ledgers.
- Preserve V1, V2, and V3 definitions and digests unchanged.

## First Concrete Lift Action

Admit a narrow successor model identity that explicitly selects the accepted
LAI carbon subpool and either fully defines both previous-offset fields or
removes them from the state schema. Bind the correction to independent
phenology/state vectors, dual science review, and immutable definition digest;
then resume this same implementation package against that authority.
