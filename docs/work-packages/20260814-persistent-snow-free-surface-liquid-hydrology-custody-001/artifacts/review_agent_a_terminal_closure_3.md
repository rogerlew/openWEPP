# Review Agent A — Terminal Closure 3 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained Ran evidence`

Reviewed commit: `e55bab15b84301b6bc8649dd2903c714da13726e`

Verdict: `HOLD / NO-GO`.

## Findings

### High — Final closure omits the per-source-parcel enthalpy join

`project_parcel_arithmetic()` now provides one shared expected/actual
projection keyed by `(source_parcel_id, basis_ofe_id)` and one shared pair of
OFE enthalpy aggregates at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:577-766`.
The arithmetic preflight consumes both the projected mass and enthalpy for
each parcel key at `:861-898`, then consumes the projected OFE enthalpy totals
at `:899-925`.

The final finite validator does not apply the same comparison set. It calls
`require_close_mass()` for every parcel key at `:1088-1100`, but performs no
per-key `require_close_enthalpy()`. It compares enthalpy only after aggregation
to the OFE total at `:1101-1119`.

This permits silent cross-parcel substitution. In the existing covered-canopy
fixture, swap the independently frozen enthalpy values of two nonzero source
parcels while leaving their masses and all producer-owned receipts unchanged.
Each preflight per-key enthalpy comparison returns finite `Some(false)` and is
correctly deferred; the swapped two-term OFE sum is unchanged; immutable
producer reconstruction matches because closure operands are deliberately
excluded; per-key mass passes; and OFE-total enthalpy passes. The poisoned
candidate is therefore accepted even though neither source parcel reconstructs
its own enthalpy custody.

That contradicts
`docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md:451-455`,
which requires mass and enthalpy to be independently reconstructed for every
source parcel across infiltration, retention, routed runoff, and outlet
runoff. Producer E009 reconstruction is not a substitute for this independent
anti-tautological join.

Required correction: apply the admitted enthalpy close rule beside the mass
close for every projected parcel key, returning E010 with the exact current
basis OFE and source-parcel identity for finite mismatch. Retain the OFE-total
enthalpy join as an additional owner-total check. Add a two-source same-OFE
swap poison that preserves each mass, receipt self-consistency, and exact OFE
enthalpy total but must fail per-source closure. Retain a per-source
comparison-arithmetic poison proving `None` remains E003 before E009/E010.

### High — Routed projection arithmetic reports origin identity instead of current basis OFE

The shared projection correctly keys routed values by their current
`basis_ofe_id`, but its contextualization does not consistently use that key.
While accumulating expected OFE enthalpy at
`surface_liquid_closure.rs:704-735`, it first searches
`source_parcels` by source ID and therefore selects the parcel's original
store even for a downstream routed key. Actual OFE accumulation at `:736-759`
likewise selects the receipt's `origin_store_key`. The per-key preflight
comparison at `:866-895` also reports the original source store.

For an upper parcel routed into middle or lower, an overflow/nonzero-underflow
while accumulating or comparing the downstream-basis amount consequently
reports upper OFE/tile/surface/source. The arithmetic bucket's exact
`key.basis_ofe_id` and its configured store are already available. The former
final validator reported the current `route_record.key`, and the retained
finite per-key and per-OFE final comparisons still use the current basis at
`:1080-1119`.

This is canonical failure-payload drift: SC-SURFACELIQUID-001 requires the
exact applicable OFE/tile/surface/source, not a convenient related origin.
Required correction: resolve projection arithmetic and comparison context from
the current `basis_ofe_id` record, retaining the source-parcel/receipt ID
separately. Add an unequal-area multi-hop poison whose arithmetic failure
occurs only after re-keying downstream and assert the downstream basis store,
transaction, owner, parcel, and both rollback hashes.

### Medium — Store closure arithmetic remains substantially duplicated

The parcel/routing projection is now centralized, closing the prior 300-line
parallel-transcription defect. Store arithmetic is still implemented twice:
the preflight repeats tile-fraction division, `W0-F+C-overflow`, retained
addition, and both checked comparisons at
`surface_liquid_closure.rs:780-858`; `validate_store_equations()` repeats the
same equation and comparisons at `:930-1007`.

No current formula or tolerance difference was found for correctly identified
rows. The duplicated checked science equation is nevertheless substantial and
can silently recreate the same E003/E010 precedence drift when either copy is
changed. Repository review policy requires at least a medium finding for this
class of duplicated Rust logic.

Required correction: factor one store-arithmetic projection returning the
checked pre-ingress and ending values. The preflight should inspect the shared
comparison outcomes only for arithmetic `None`; final closure should consume
the same values and map finite `Some(false)` to E010. Preserve the dedicated
identity handling and exact store context around that shared projection.

## Prior Finding Closure And Retained Correctness

The remediation otherwise materially closes both closure-2 findings:

- Routed additions now update the same projected `expected` map consumed by
  final validation, and expected/actual per-OFE enthalpy totals are accumulated
  once in canonical key order. The preflight now performs the formerly omitted
  aggregate checked comparison.
- The combined same-OFE aggregate poison at
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_tests.rs:723-797`
  has finite per-key comparison scales but an indeterminate aggregate scale.
  It also carries a producer mismatch and an earlier finite store mismatch,
  and correctly returns E003 with exact transaction, owner/OFE/tile, and
  rollback hashes before E009/E010.
- `DirectSurfaceLiquidIngressCandidate::validate()` retains the required
  visible order: arithmetic preflight, one immutable producer reconstruction
  and E009 comparison, then final independent E010.
- `first_identity_aware_mismatch()` and
  `first_map_identity_mismatch()` at `surface_liquid_ingress.rs:483-516`
  are membership-aware for shorter actual collections. State records,
  continuations, receipts, ledgers, and WB14 upper/middle deletions at
  `surface_liquid_ingress_tests.rs:884-1005` correctly name the missing
  expected identity with exact hashes.
- For equal-length replacement or reorder, the sequence helper retains the
  actual first mismatching row; the map helper retains the actual wrong key or
  value key. Static search finds no remaining producer positional helper or
  first-configuration-record fallback. WB14 map order itself is not semantic.
- The earlier checked-close tri-state, receiver division/accumulation,
  independent receiver/rollback deletion attribution, and exact three-owner
  rollback corrections remain unchanged. No cited raw receiver `+=` or
  water-density division has returned.
- No clamp, tolerance, unit conversion, WB14 transition, source operand,
  science authority, model identity, production selector, publication path,
  activation, or cutover changed in this increment.

Line-count governance improves and remains compliant. Current affected counts
are 2,347 lines for `surface_liquid_owner.rs`, 876 for
`surface_liquid_owner_tests.rs`, 1,927 for `surface_liquid_ingress.rs`, 1,227
for the mechanically extracted `surface_liquid_ingress_tests.rs`, 1,362 for
`surface_liquid_closure.rs`, 2,881 for `land_surface_energy_shadow/mod.rs`,
2,852 for `direct_runtime/runoff.rs`, and 303 for
`surface_liquid_wb14.rs`. Every edited Rust file is below 2,000 lines and every
affected production file remains below the mandatory 3,000-line threshold.
The retained 2,000-line WARN dispositions remain explicit; no mandatory
line-count exception was added.

All earlier review and failed-attempt artifacts are preserved. This review
adds only this terminal artifact and does not alter concurrent Review B work.

## Exact-Commit Validation

Ran against the clean exact commit
`e55bab15b84301b6bc8649dd2903c714da13726e`:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 41/41 selected; 507 skipped by filter

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
PASS: 548/548; 0 skipped; three known slow OFE-routing oracle tests completed

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before review artifacts were added
```

These gates verify the shared aggregate projection, combined precedence
poison, and upper/middle deletion controls. They do not contain a
cross-source enthalpy-substitution poison, a downstream-only arithmetic-context
poison, or replacement/reorder poisons for every producer collection.

The retained heavy evidence at `74d512f44` remains truthful for its exact
bytes: workspace strict Clippy, 2,783/2,783 full Nextest, workspace doctests,
dependency policy, formatting, and diff hygiene passed. It predates the later
arithmetic/projection changes and does not replace exact-final-byte terminal
qualification after correction.

## Residual Risk And Missing Tests

- Add equal-length identity replacement and reorder poisons for state records,
  continuations, receipts, and ledgers. Static mechanics are correct, but the
  package claim currently exceeds its direct producer-context evidence.
- Add WB14 missing-key, extra-key, and wrong-value controls; map reorder is not
  a meaningful case.
- Add store-projection parity controls for arithmetic `None`, finite
  `Some(false)`, and order-independent later-store precedence once the
  duplicated equation is centralized.
- Exact-final-byte full-workspace, doctest, dependency-policy, anti-evasion,
  and any package-required comparator gates remain terminal evidence after the
  findings are corrected.

## Approval Statement

`NO-GO`: commit `e55bab15b` closes the aggregate-precedence and shifted-row
deletion findings and retains final E010, rollback, prior arithmetic, science,
and line-count corrections. Dependency-package closure remains blocked because
the final independent validator still accepts cross-source parcel enthalpy
substitution, routed arithmetic can publish the wrong basis identity, and the
store equation remains duplicated across preflight and final closure.
