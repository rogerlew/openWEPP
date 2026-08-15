# Review Agent A — Terminal Rust Correctness Re-review

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained Ran evidence`

Reviewed commit: `3bad2f2aab8cf2b1dbeb8754f0ff992fca89704e`

Verdict: `HOLD / NO-GO`.

## Findings

### High — Checked closure arithmetic discards E003 precedence, and receiver aggregation is not fully checked

The new shared arithmetic surface in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_owner.rs:27-85`
correctly rejects nonfinite arithmetic, exact nonzero underflow from
multiplication/division, and nonfinite closure difference, scale, scaled
tolerance, or final tolerance. In particular,
`checked_surface_liquid_close(f64::MAX, f64::MAX / 2.0, ...)` now returns
`None`; the former `difference <= infinity` false pass is closed.

Both public closure consumers erase the distinction between an arithmetic
failure and an ordinary finite mismatch:

- `require_close_mass()` and `require_close_enthalpy()` at
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:942-970`
  accept only `Some(true)` and map both `Some(false)` and `None` to
  `DirectSurfaceLiquidError::Closure`, whose canonical code is E010.
- `mass_m_close()` and `enthalpy_close()` at
  `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:2204-2215`
  likewise collapse the tri-state result to `bool`; their callers publish the
  receiver/atomic E011 path.

This is reachable using finite public receiver operands while keeping the
expected-value equation finite. For example, an LSE operand with beginning
enthalpy `f64::MAX / 2.0`, zero retained enthalpy, unit tile fraction, and
ending enthalpy `f64::MAX` constructs the finite expected value
`f64::MAX / 2.0`; only the comparison scale overflows. The result is E011,
although SC-SURFACELIQUID-001 requires the earlier numerical-domain E003
precedence. The crate-local poison at
`surface_liquid_owner_tests.rs:513-531` proves the helper returns `None`, but
no public closure poison asserts the canonical code.

The unified receiver bridge also retains raw arithmetic at
`land_surface_energy_shadow/mod.rs:1210-1218` and `:1777-1803`: production-lane
infiltration depths and the independently frozen OFE/tile mass and enthalpy
maps use `/` and `+=` rather than the checked conversion and accumulation
helpers. Upstream ingress checks bound several of these shapes, and downstream
closure rejects a resulting nonfinite operand, so this does not restore the
former silent acceptance. It nevertheless leaves the claimed producer and
independent-receiver arithmetic surface incomplete and can defer the first
typed arithmetic offender into a bound or receiver-envelope path.

Required correction: preserve `Option<bool>` through both closure surfaces so
`None` becomes contextual E003 with the available transaction, owner/OFE/tile,
and rollback hashes, while only `Some(false)` becomes E010/E011. Replace the
receiver-side raw division/addition with checked operations that return the
same contextual E003 before receiver mutation/validation. Add public poisons
for tolerance-scale overflow in both closure surfaces and for receiver
aggregation; assert exact code, context, no candidate publication, and
byte-identical rollback.

## Prior Finding Closure And Retained Correctness

The identity finding is materially closed. The shared comparison at
`land_surface_energy_shadow/mod.rs:1389-1418` first searches membership when
the unique actual sequence is shorter, so a nonterminal deletion identifies
the missing expected soil-thermal OFE/tile. Equal-length replacement or
reorder shapes instead retain the actual first mismatching identity. Both the
pre-callback expectation check and final receiver-topology checks use this
rule. The two-tile deletion poison at
`tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs:1748-1824`
asserts the expected owner/OFE/tile, callback non-invocation, and both rollback
hashes; the wrong-second-row poison retains the actual replacement identity.

The numerical false-accept finding is also closed as an acceptance defect:
checked difference, scale, tolerance, unit conversion, sums, products, and
division cannot yield an infinite tolerance that admits a wrong finite value.
Its remediation remains incomplete because the public error taxonomy and the
raw receiver aggregations above do not preserve the required arithmetic
failure boundary.

Static inspection confirms no regression in the previously accepted custody
domains:

- strict canonical configuration/state bytes, digests, restart continuation,
  predecessor lineage, and sealed resource/ingress/unified candidates remain
  intact;
- authorization is reconstructed from immutable beginning state and preserves
  exact request/authorization/use identity and `0 <= F <= A <= D`; checked
  same-store demand, supply, proportional numerator/share/remainder, and
  allocation arithmetic remains in place;
- `F/f_t`, `C/f_t`, raw resource state, condensation overflow, tile/OFE parcel
  construction, ingress/enthalpy sums, retention partition, and unequal-area
  routing now fail closed on their checked arithmetic boundaries;
- signed condensation remains resource-phase credit before capacity overflow,
  while rain, canopy release, runon, and overflow remain post-authorization
  ingress and cannot satisfy same-interval withdrawal;
- production and persistence still call the single shared complete WB14
  continuation; no parallel Green-Ampt transition was introduced;
- actual production-soil, soil-thermal, and retained-LSE receiver mutation is
  clone-only, with independent ordered-layer, residual/unfrozen aggregate,
  infiltration-enthalpy, retained-enthalpy, parcel, and store reconstruction;
- snow/frost/frozen-state E004 preflight, exact three-owner rollback, contextual
  E007/E011 identity, and byte-identical failure rollback remain intact; and
- normal production constructors still set `surface_liquid_shadow=None` and no
  production selector, runner, scheduler, publication, or cutover consumer was
  added.

The test extraction is cohesive: `surface_liquid_owner.rs` is 2,347 lines and
`surface_liquid_owner_tests.rs` is 876 lines. All affected Rust source files
remain below the mandatory 3,000-line threshold; the 2,852-line `runoff.rs`
disposition is unchanged. The shared checked primitives remove the former
duplicated tolerance formula. The separate producer and independent
reconstruction algorithms remain intentionally distinct to preserve
independent science closure rather than copying producer residuals.

## Exact-Commit Validation

Ran at `3bad2f2aab8cf2b1dbeb8754f0ff992fca89704e`:

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
```

The retained heavy run at `74d512f44` remains valid for its exact bytes
(workspace Clippy, 2,783/2,783 full Nextest, doctests, dependency policy,
formatting, and diff hygiene). It predates the arithmetic remediation and does
not substitute for an exact-head full-workspace closure run after the remaining
finding is corrected.

## Residual Risk And Missing Tests

- No public-call poison distinguishes closure-arithmetic `None` (required E003)
  from a finite closure mismatch (E010/E011) in either independent closure
  surface.
- No receiver-aggregation poison exercises the raw production-lane and frozen
  per-OFE/tile accumulation seams with exact E003 context and rollback.
- Add an explicit equal-length reorder poison. Static membership/positional
  behavior is correct, and the existing deletion/replacement poisons cover the
  two branches, but a public reorder regression would improve localization.
- Exact-head full-workspace correctness, doctest, and dependency-policy gates
  have not been rerun after `82bfdc3a0`; they remain terminal closure evidence,
  not a substitute for correcting the finding.

## Approval Statement

`NO-GO`: exact commit `3bad2f2aa` closes the wrong-receiver deletion and silent
infinite-tolerance acceptance defects, and the retained custody, D/A/F,
condensation, WB14, receiver, rollback, serialization, and production-exclusion
domains remain materially sound. Dependency-package closure is still blocked
because reachable checked-comparison arithmetic is published with E010/E011
instead of E003 and two receiver aggregation seams remain unchecked.
