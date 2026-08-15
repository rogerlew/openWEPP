# Review Agent B Terminal Re-review — Hydrology, Custody, And Science

Evidence class: `Static exact-commit + Ran exact-commit`

Reviewed commit: `3bad2f2aab8cf2b1dbeb8754f0ff992fca89704e`

Verdict: `HOLD / one material checked-arithmetic error-precedence defect remains / no authority HOLD`.

This fresh review preserves the earlier hydrology PASS at `6c786ca6d` and
reassesses the exact post-Rust-review remediation bytes. It read the complete
version-5 `SC-SURFACELIQUID-001`, the LSE and WATBAL ownership rules, all
retained finding and gate history, the current owner/ingress/closure modules,
the unified receiver bridge, and the new large-finite and expectation-deletion
tests.

## Material finding

### B-TERMINAL-REREVIEW-HIGH-001 — checked closure arithmetic loses E003 precedence at its public consumers

The new centralized `checked_surface_liquid_close()` correctly returns `None`
when difference, scale, scaled tolerance, or final tolerance arithmetic is not
finite or when a nonzero multiplication/division underflows
(`surface_liquid_owner.rs:71-85`). The public closure consumers then collapse
that arithmetic failure into an ordinary closure mismatch:

- `require_close_mass()` and `require_close_enthalpy()` compare the result with
  `Some(true)` and map both `Some(false)` and `None` to
  `DirectSurfaceLiquidError::Closure` (`surface_liquid_closure.rs:942-970`).
  The public wrapper consequently reports `SURFACELIQUID-E-010`, not the
  contract-precedent `SURFACELIQUID-E-003` arithmetic/domain failure.
- `mass_m_close()` and `enthalpy_close()` make the same lossy conversion to
  `bool` (`land_surface_energy_shadow/mod.rs:2204-2215`). Their callers then
  report receiver/atomic `SURFACELIQUID-E-011`, again hiding the earlier E003
  arithmetic failure.

This is directly reachable with finite operands without first overflowing an
expected-value equation. For example,
`checked_surface_liquid_close(f64::MAX, f64::MAX / 2, ...)` returns `None`
because the scale overflows. The new unit test explicitly proves that `None`
result (`surface_liquid_owner_tests.rs:513-531`), but no public-call test proves
its canonical error code. A closure operand whose finite actual and expected
values are `MAX` and `MAX/2` therefore reaches E010/E011 instead of E003. Even
an exact pair at `MAX` cannot construct the admitted sum-of-absolute-operands
tolerance and is misclassified as an ordinary closure mismatch.

The receiver bridge also retains unchecked accumulation at
`land_surface_energy_shadow/mod.rs:1215` and `:1788-1802`. If those finite
receipt aggregates overflow, failure is deferred to a production bound or
receiver-envelope path instead of being emitted at the first arithmetic
offender as contextual E003. The ingress producer itself now checks its own
aggregates, so this does not permit a candidate to commit, but it leaves the
independent receiver arithmetic and failure precedence incomplete.

Required correction:

1. Preserve the tri-state result through every close consumer. `None` must
   become contextual `SURFACELIQUID-E-003` with transaction, owner/OFE/tile when
   available, and beginning/attempted hashes. Only `Some(false)` may become the
   applicable E010 or E011 closure mismatch.
2. Replace receiver-side raw `+=` aggregation with the same checked arithmetic
   and contextual E003 propagation before production or receiver validation.
3. Add public-call poisons in both independent closure surfaces where expected
   equation construction stays finite but tolerance scale construction fails;
   assert E003 rather than merely asserting the helper returns `None`.
4. Add a receiver-aggregation overflow poison and prove no candidate, exact
   rollback, and E003 first-offender context.

This is a bounded implementation defect. It requires no contract amendment,
new model identity, tolerance change, clamp, fallback, or new package.

## Confirmed closures

- The nonterminal thermal-expectation deletion is corrected. Membership-aware
  comparison reports the exact missing soil-thermal owner/OFE/tile before the
  fixed-cap callback, while present replacement/reorder rows retain their
  actual identity. The new two-tile deletion vector also binds both rollback
  hashes.
- Persistent state remains exact per run/OFE/tile/surface/class/source,
  configuration- and digest-bound, strict on restart combinations, and
  transaction-lineage preserving.
- One immutable beginning snapshot supplies one proportional authorization.
  Request, authorization, and finalized-use identities remain exact; all
  bounded branches preserve `0 <= F <= A <= D`; only finalized use is debited;
  and unused authorization remains in the owner.
- Signed condensation is credited before capacity overflow, retains its
  temperature/enthalpy and tile/source mapping, and enters only post-resource
  ingress. It cannot satisfy same-interval authorization.
- Ingress now checks tile/OFE conversion, parcel enthalpy, support slicing,
  mixed enthalpy, retention allocation, routing area conversion, and routed
  mass/enthalpy arithmetic before emitting receipts. One shared production
  WB14 continuation remains the infiltration authority.
- Store and parcel closure still reconstruct from explicit operands rather
  than producer residuals. The new checked arithmetic prevents the previous
  silent `difference <= infinity` acceptance; the remaining finding concerns
  typed precedence and uncovered receiver aggregation, not an accepted bad
  candidate.
- Actual production soil, soil-thermal, and retained LSE candidates remain
  clone-only and independently reconstruct ordered layers, aggregate residual
  water, infiltration enthalpy, and retained surface enthalpy.
- Snow/frost/frozen-state entry remains contextual E004 and precedes
  authorization/callback. Rollback remains the exact LSE, hydrology, and soil
  thermal owner set.
- Normal production constructors retain `surface_liquid_shadow=None`; no
  runner selector, default, production dispatch, output publication, runtime
  activation, or cutover path was introduced.

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

These runs confirm the retained normal and currently encoded poison cases.
They do not exercise public tolerance-construction failure or receiver-side
aggregate overflow with canonical E003 assertions.

## Approval statement

`NO-GO`: exact commit `3bad2f2aa` is not ready for dependency-package closure.
The scientific ownership, persistent custody, D/A/F, signed condensation,
ingress/routing, WB14, rollback, receiver identity, and production-exclusion
behavior reviewed here remains materially sound. The one remaining
checked-arithmetic defect is fail-closed but publishes the wrong canonical
error precedence on reachable large-finite closure operands and leaves two
independent receiver accumulations unchecked. Correct it in this package and
rerun fresh exact-byte Rust and hydrology reviews.
