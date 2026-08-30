# Soil-thermal exact-carry contract gate evidence

Evidence mode: `Static + Ran`

## Scope and authority

The version-15 amendment is contract/test-only. It owns no production edit.
It advances `SC-SURFACELIQUID-001` with `INV-SURFACELIQUID-022` and
`SC-LANDSURFACEENERGY-001` with the next free stable ID,
`INV-LANDSURFACEENERGY-150`. The parent package prospectively corrected the
initial requested `INV-LANDSURFACEENERGY-140`, which was already occupied by
the active frozen-litter V3 identity invariant.

The amendment defines a receiver-owned exact total, canonical signed-dyadic
wire form, exact accepted operand aggregation, one correctly rounded finite
binary64 high term, exact remainder, V1-to-V2 zero-carry migration, downgrade
refusal, versioned owner/receipt/restart/checkpoint identity, independent
reconstruction, and exact rollback. It retains v14 litter phase chronology,
physics, tolerances, custody, event/topology rules, and the exact 60-second
fallback floor.

## Contract/vector gate

Ran: the final isolated expected-red run
`f2d09db4-cf59-4736-817e-e2fe1dfff57b` executed both version-15 contract/vector
tests and both production-binding tests. The contract/vector tests passed. The
only failures were the intended missing `ExactDyadicEnthalpy` production
symbol; see `pre-implementation-red.md`.

The bound vector families are:

- canonical WAT5 high term `-34315.42154113602 J m^-2`, credit
  `-8.0670339832330148e-19 J m^-2`, and exact carry
  `(-1,"1dc319224e55f",-109)`;
- positive/negative, halfway even/odd, adjacent-high crossing, cancellation,
  order, subnormal, normal/subnormal-boundary, largest-finite, and overflow;
- every noncanonical signed-dyadic schema form and nonfinite operand;
- schema/definition/configuration/state/version/owner/transaction/predecessor/
  support/OFE/layer/source/ordinal/digest substitution;
- receipt omission, duplication, reorder, substitution, and replay;
- V1 byte lock, exact-zero migration, downgrade refusal, before/after-credit
  restart split, checkpoint equivalence, and byte-exact rollback; and
- canonical WAT5 plus unchanged `p61` and native-forest real consumers.

## Documentation and hygiene gates

Ran:

- `bash tools/release/check_sc_unit_compliance.sh --path
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
  — `PASS`, no findings;
- `bash tools/release/check_sc_unit_compliance.sh --path
  docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`
  — `PASS`, no findings; and
- `git diff --check` on the exact owned file set — `PASS`.

The surface-liquid unit lint exposed the already registered
`surface_liquid.interval_s` alias during amendment; the materially amended
contract now exposes that alias in its canonical symbol map. No unit
conversion, boundary value, publication metadata, or scalar exception changed.

## Disposition

`CONTRACT-FIRST RED READY`. Production promotion remains blocked until the
missing version-15 symbols, exact vector implementation, restart/rollback, and
three real-consumer gates pass. Dual independent contract review, finding
disposition, and dual verification remain package-level gates after this
authoring handoff; this artifact does not self-review or waive them.
