# Review Agent A — Terminal Closure 4 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained Ran evidence`

Reviewed commit: `9339b55c637c266c765b7e461ab03d89dae04317`

Verdict: `HOLD / NO-GO`.

## Findings

### Critical — The remediation replaces mandatory chronological mixing with source-specific enthalpy

The production change removes the interval enthalpy sum and the single
`h_mix = Q_b / X_b`. `advance_one_ofe()` now reconstructs
`specific_enthalpy = enthalpy / mass` separately for every source contribution
and uses that source-specific value for its infiltration receipt at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:1267-1451`.
`retain_excess_proportionally()` repeats the source-specific division and uses
it for retained water and runoff at `:1598-1725`.

This directly contradicts the canonical constitutive rule in
`docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md:370-401`
and `:403-422`. The contract requires all parcels present in one chronological
subinterval to be mixed before WB14 partition:

```text
X_b = sum_p(x_p,b)
Q_b = sum_p(q_p,b)
h_mix,b = Q_b / X_b
Q_infiltration,b = I_b * h_mix,b
```

Every attributed infiltration, excess, retained, and runoff child must carry
that same `h_mix,b`. With two equal-mass 280 K and 290 K canopy sources, the
admitted receipts must all carry the common 285 K mixture. The current code
instead emits 280 K receipts for one source and 290 K receipts for the other.
The mass partition remains proportional and the OFE energy total will often
still close, so the aggregate join and passing tests do not detect the wrong
per-receipt thermal custody. Floating-remainder assignment also makes this a
real exact-byte arithmetic-path change, not a harmless relabeling.

The new independent join encodes the same unauthorized behavior. Frozen source
operands capture raw pre-mix enthalpy at
`surface_liquid_closure.rs:603-723`, and
`project_parcel_arithmetic()` treats that raw value as the expected enthalpy of
the same source after partition at `:742-781`. A canonical well-mixed child is
instead expected to carry its attributed mass times `h_mix,b`. The frozen
source operand at `:264-270` does not even retain `start_s` or `end_s`, so the
independent validator lacks the operands needed to reconstruct different
chronological overlap windows. The same-OFE swap test at
`surface_liquid_ingress_tests.rs:800-843` therefore proves the new raw-source
invariant, not the binding post-mix invariant.

Required correction:

1. Restore one checked `Q_b` sum, `h_mix,b`, and mixed temperature per exact
   chronological subinterval, then use it for every attributed child.
2. Preserve source mass provenance and final-source floating remainders, but
   do not preserve pre-mix source-specific temperature after mixing.
3. Make the independent operands sufficient to reconstruct chronological
   mixing, including source timing or an equivalently independent frozen
   per-window projection. Compare each post-mix attributed source enthalpy and
   separately close raw input `Q_b` to the mixed output total.
4. Add unequal-temperature positive and poison vectors covering infiltration,
   retention, runoff, downstream routing, source-order independence, and
   mixed enthalpy applied exactly once.

This is a production science-contract regression. The existing authority is
complete enough to fix it; changing the contract after the implementation
would require separate authority adjudication and is not an admissible closure
of this finding.

### High — Independent source identity and cardinality fail open, especially for exact-zero parcels

The independent validator has no exact source-operand identity pass.
`DirectSurfaceLiquidParcelClosureOperands.kind` is captured and exposed at
`surface_liquid_closure.rs:264-302`, but it is never consumed by projection or
validation. Source operands are aggregated only by
`(source_parcel_id, basis_ofe_id)` at `:747-781`; duplicate source IDs are not
rejected. Final comparison unions expected and actual arithmetic keys and
defaults a missing side to `AmountPair::default()` at `:920-955`.

Exact-zero parcels are normal, not hypothetical. `capture_amount()` always
pushes an operand at `:658-694`, while `advance_one_ofe()` creates a contribution
only when interval mass is positive at
`surface_liquid_ingress.rs:1212-1264`. Covered-canopy fixtures routinely
contain zero second-drainage and stemflow sources, including
`surface_liquid_ingress_tests.rs:805-820`.

Consequently, deleting, replacing, re-keying, or duplicating one of those zero
frozen sources changes source identity/cardinality but leaves every projected
mass and enthalpy equal to zero. The union comparison accepts it. A nonzero
source's frozen `kind` can likewise be changed without affecting any validator
branch. The crate-local helper
`add_downstream_projection_poison_for_test()` at
`surface_liquid_closure.rs:191-204` makes the zero-duplicate case explicit:
called with zero enthalpy, it adds an invalid duplicate/re-keyed frozen source
without changing any checked amount. Producer E009 reconstruction deliberately
excludes closure operands, so it cannot close this seam.

This violates the contract's exact source/basis identity and duplicate/missing
parcel obligations. Required correction: validate the complete frozen source
identity set and cardinality from immutable configuration/resource/input
structure, including source ID, origin key, basis OFE, kind, uniqueness, and
expected zero-valued members, independently of the amount equations. Add
zero-source deletion, equal-length replacement, basis re-key, duplicate, kind
poison, and reorder controls with the canonical applicable error taxonomy and
exact context.

### High — OFE aggregate errors fabricate the first configured tile identity

`projection_store_key()` resolves an OFE by calling `.find()` over
configuration records and returns the first matching store at
`surface_liquid_closure.rs:370-381`. Every OFE-total comparison uses that store
at `:958-985`; `projection_key_store()` also falls back to it when exact parcel
routing identity cannot be resolved at `:384-424`.

Multi-tile OFEs are explicitly admitted. Configuration validation accumulates
tile fractions by OFE and permits multiple unique tile records at
`surface_liquid_owner.rs:567-608`. An OFE aggregate has an exact OFE identity
but no unique tile/surface/source identity. Selecting the lexically first
configured tile fabricates context and may name a different tile than the
configured route destination. SC-SURFACELIQUID-001 at `:477-481` requires exact
available identity and typed absence for unavailable identity; a convenient
first-record fallback is prohibited.

The new routed-context test uses `routed_configuration()`, which has only one
tile per OFE (`surface_liquid_ingress_tests.rs:70-96`), so it cannot expose this
failure. Required correction: construct aggregate failures from partial
context with OFE present and tile/surface/source absent, and resolve per-key
routed context from the exact origin/current/destination identity without a
first-record fallback. Add multi-tile destination controls for both E003 and
E010, including the case where the route destination is not the first tile.

## Prior Finding Closure And Retained Correctness

Apart from the findings above, the closure-3 structural corrections are
present and internally consistent:

- `project_store_arithmetic()` is the single checked implementation of
  `W0-F+C-overflow+retained`; preflight and final store validation consume its
  two projected values.
- `project_parcel_arithmetic()` is shared by arithmetic preflight and final
  validation. `compare_parcel_projection()` applies the same disposition to
  mass and enthalpy for every `(source_parcel_id, basis_ofe_id)` key and then
  to the OFE enthalpy total. `None` maps to E003; finite `Some(false)` is
  deferred in preflight and maps to E010 in final closure.
- The same-OFE enthalpy swap now fails per key before the unchanged OFE total,
  and per-source comparison-scale overflow remains E003. As described in the
  critical finding, the expected enthalpy operand is scientifically wrong.
- Routed area conversion still scales mass and enthalpy exactly once and the
  shared projection uses the configured destination for direct route
  arithmetic. The two-OFE destination tests pass with exact rollback hashes.
- Producer reconstruction remains ordered after exhaustive arithmetic
  preflight and before finite independent closure. Membership-aware sequence
  localization retains the missing expected identity for deletion and the
  actual first mismatching row for equal-length replacement/reorder. WB14 map
  identity remains membership-aware; map reorder is nonsemantic.
- The prior checked-close tri-state, receiver division/accumulation, D/A/F,
  signed-condensation, rollback/restart, snow/frost exclusion, and default-off
  production selection changes remain intact. No new runtime selector,
  publication path, activation, or cutover appears in this increment.

The shared store equation closes the prior substantial duplication finding.
Final store comparison still uses `require_close_mass()` while preflight uses
`compare_projected_value()`; their current tri-state mappings are equivalent,
but consolidating those small parallel adapters would reduce future precedence
drift.

Line-count governance remains within the mandatory threshold. Current affected
counts are 2,347 lines for `surface_liquid_owner.rs`, 876 for
`surface_liquid_owner_tests.rs`, 1,954 for `surface_liquid_ingress.rs`, 1,453
for `surface_liquid_ingress_tests.rs`, 1,396 for
`surface_liquid_closure.rs`, 2,881 for `land_surface_energy_shadow/mod.rs`,
2,852 for `direct_runtime/runoff.rs`, and 303 for
`surface_liquid_wb14.rs`. Every affected production file remains below 3,000
lines; edited Rust files remain below 2,000 lines.

All prior review and failed-attempt artifacts are preserved. This review adds
only the named Agent A artifact and does not alter the concurrent Review B
artifact.

## Exact-Commit Validation

Ran against exact commit
`9339b55c637c266c765b7e461ab03d89dae04317`; the only pre-existing worktree
addition was the concurrent Review B terminal artifact:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
PASS: 552/552; 0 skipped; three known slow OFE-routing oracle tests completed

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this Agent A artifact was added
```

These gates prove the encoded source-specific implementation, shared
projection mechanics, and current one-tile context tests. They cannot override
the canonical well-mixed equations and do not exercise zero-valued frozen
identity mutation or multi-tile aggregate failure context.

The retained exact-head full-workspace, doctest, dependency-policy, AUTH11,
anti-evasion, and science-admission evidence predates this production arithmetic
change. Because a material science regression remains, rerunning the expensive
terminal campaign cannot make this commit releasable.

## Residual Risk And Missing Tests

- Add a real mixed-temperature runtime vector with overlapping and disjoint
  timing windows. Assert common `h_mix,b` only within each overlap window and
  assert exact raw-to-mixed total enthalpy closure.
- Add multi-tile OFE aggregate and multi-hop routed failure-context vectors;
  the current two-OFE/one-tile poison is insufficient evidence for exact
  context across the supported configuration domain.
- Add frozen-source identity controls for positive and zero sources. Current
  producer deletion/replacement/reorder controls do not exercise the excluded
  independent operand collection.
- After correction, rerun exact-final-byte full-workspace Nextest, strict
  workspace Clippy, doctests, dependency policy, AUTH11/anti-evasion, science
  admission, formatting, diff hygiene, and package Markdown lint.

## Approval Statement

`NO-GO`: commit `9339b55c6` closes the earlier shared-comparison and direct
routed-context omissions, but it does so by replacing the binding
chronological well-mixed enthalpy rule with source-specific production
thermodynamics. Independent source identity also fails open for normal zero
parcels, and OFE-total failures fabricate a first-tile identity in admitted
multi-tile configurations. Restore canonical mixing, make the frozen ledger
sufficient for post-mix reconstruction and exact identity, remove positional
context fallback, and rerun terminal gates before dependency-package closure.
