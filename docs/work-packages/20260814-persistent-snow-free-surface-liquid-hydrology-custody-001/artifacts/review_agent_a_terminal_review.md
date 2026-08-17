# Review Agent A — Terminal Rust Correctness Review

Evidence class: `Static exact-commit + reconciled retained Ran evidence`

Reviewed commit: `6c786ca6d07881697d64205fe44b09f12369034b`

Verdict: `HOLD / NO-GO`.

## Findings

### High — A missing non-terminal independent thermal expectation still reports the shifted row, not the missing expected receiver

`validate_receiver_expectations()` compares the configured tile sequence with
the caller-supplied independent thermal sequence through
`first_expected_identity_violation()` at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:952`
and again in the later receiver-topology check at line 1254. The shared helper
at lines 1355-1375 selects `actual[index]` whenever one exists and uses the
expected identity only for a missing suffix.

For configured identities `[tile-a, tile-b]`, a structurally valid
`UnifiedReceiverExpectations` containing only `[tile-b]` therefore fails closed
as `SURFACELIQUID-E-011` but names `tile-b`. The absent first receiver is
`tile-a`. This is the same deletion-versus-shift defect previously corrected
for the three-owner rollback sequence; it remains in both thermal-topology
call sites. `UnifiedReceiverExpectations::try_new()` accepts the shortened
nonempty unique sequence, so the path is public and reaches pre-callback
validation.

The terminal regression at
`tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs:1636`
covers a present wrong second row only. It does not remove a first or middle
expectation. The retained disposition claim that missing rows use the exact
expected identity is therefore not closed.

Required correction: make ordered identity validation membership-aware before
positional mismatch reporting, so a deleted unique expected receiver reports
that expected OFE/tile while a present replacement retains its actual typed
identity. Add two-or-more-row deletion poisons for each non-terminal position,
assert exact owner/OFE/tile and both rollback hashes, and prove the fixed-cap
callback is not invoked.

### High — Independent closure tolerances can overflow to infinity and accept materially wrong finite values

The mass and enthalpy predicates in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:737-768`
and the duplicated receiver predicates in
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:2083-2095`
compute:

```text
scale = abs(actual) + abs(expected)
tolerance = absolute_term + 64 * epsilon * scale
```

They check that `actual` and `expected` are finite, but not that `scale` or the
tolerance is finite. With finite values such as `actual = f64::MAX` and
`expected = f64::MAX / 2`, `scale` becomes positive infinity while the
difference remains finite; `difference <= infinity` is true. A materially
wrong mass or enthalpy equation is consequently accepted by the independent
closure guard.

No contract-authorized upper bound excludes this case: configuration capacity,
state mass, ingress mass, OFE area and receiver operands are bounded by
finiteness and sign, not by a smaller numerical ceiling. Related arithmetic
also remains unchecked before use, including `F/f_t` and `C/f_t` at
`surface_liquid_owner.rs:1614-1625`, parcel enthalpy construction and sums at
`surface_liquid_ingress.rs:869-883` and 951-989, and the OFE-area ratio at
lines 1169-1201. The closure predicate can therefore cease to be a meaningful
last guard on an admitted large-finite input.

This is duplicated contract-sensitive logic with a shared silent-false-pass
mode. It violates `INV-SURFACELIQUID-008`, the finite-domain posture, and the
requirement that numerical edge cases fail closed rather than being masked.

Required correction: centralize unit-specific checked closure comparison,
reject nonfinite scale/tolerance and nonfinite arithmetic intermediates with
the contract-prescribed precedence, and add large-finite overflow/underflow
poisons for resource conversion, enthalpy accumulation, routing area
conversion, and both independent closure surfaces. Expected values must remain
independently reconstructed rather than copied from producer residuals.

## Confirmed Correctness And Prior-Finding Disposition

Static inspection at the exact commit confirms that the following previously
accepted findings remain materially closed outside the two blockers above:

- configuration and state persistence are strict, canonical,
  configuration-bound and digest-sensitive; restart continuation and lineage
  combinations are validated before canonical emission;
- arbitration/resource/ingress/unified candidates are externally sealed;
  authorization is re-derived from immutable `W0 + D`, and the finite
  same-store demand, supply, numerator, division, remainder and allocation
  intermediates added at `93c46d3db` fail closed;
- exact request/authorization/use identity and `0 <= F <= A <= D` are retained,
  only finalized use is debited, and signed condensation is credited before
  capacity overflow becomes post-solve ingress;
- current rain, canopy release, runon and condensation overflow cannot satisfy
  same-interval withdrawal; open rain and covered canopy release are
  digest-bound mutually exclusive ingress paths;
- production and persistent continuation call one centralized complete WB14
  interval transition; no second Green-Ampt implementation remains;
- actual infiltration mutates only the cloned bound production lane, and the
  receiver reconstruction includes ordered layers, residual water over
  unfrozen depth, soil-thermal enthalpy and retained LSE enthalpy;
- rollback is limited to the exact three owners constructed by this bridge and
  preserves the caller's production frame on success and failure; and
- both normal `DirectRunFrame` constructors retain
  `surface_liquid_shadow=None`. Repository search found no runner selector,
  scheduler/default, publication or production-activation consumer for the
  unified bridge.

The dedicated surface closure is independent of producer-supplied residuals,
and the positive D/A/F, continuation, routing, receiver and rollback fixtures
separate relevant operands. Their normal-range coverage does not exercise the
two findings above.

## Heavy Evidence Reconciliation

The raw run-4 summary and logs bind commit `74d512f44`, the immediate parent of
the reviewed commit. They record:

- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS;
- workspace all-target/all-feature warnings-denied Clippy: PASS;
- full-workspace Nextest: PASS, 2,783/2,783 with 33 skipped;
- workspace doctests: PASS; and
- `cargo deny check`: PASS with only the documented non-failing unmatched
  `MIT-0` allowance warning.

Commit `6c786ca6d` adds only the retained gate logs and package narrative; it
does not change Rust source, tests, manifests, authority, selectors or runtime
inputs. Under the canonical evidence-reuse rule, run 4 resolves the earlier
workspace-Clippy and complete-full-suite evidence blockers. It does not resolve
the untested correctness findings in this review.

No additional Cargo gate was run for this terminal review. The retained heavy
run is already sufficient to establish the behaviors it executed, and rerunning
unchanged tests would not exercise either missing poison.

## Residual Risk And Missing Tests

- Add non-terminal deletion poisons for independent expectation, final LSE
  tile and final soil-thermal tile sequences; distinguish exact expected
  absence from an actual replacement or reorder.
- Add checked large-finite vectors proving closure tolerance construction
  cannot become infinite and that finite inputs cannot create nonfinite or
  zero-underflowed resource, enthalpy or routed-area intermediates.
- Retain the existing wrong-second receiver, per-position rollback deletion,
  nonzero-residual soil aggregate, D/A/F, canonical restart/digest, 48-step
  continuation, unequal-area routing, byte-identical rollback and production-
  exclusion vectors after remediation.
- The separate positive-frozen-depth-only and positive-frozen-water-only E004
  vectors remain a useful localization enhancement; the current disjunctive
  implementation is statically correct.

## Approval Statement

`NO-GO`: run 4 closes the prior heavy-evidence blockers, and the normal-range
custody, restart, D/A/F, WB14, receiver, rollback and production-exclusion
paths are otherwise materially sound. Exact commit `6c786ca6d` is not ready
for dependency-package closure because one public E011 deletion shape still
publishes the wrong offending receiver and duplicated independent closure
arithmetic can silently accept large finite mismatches.
