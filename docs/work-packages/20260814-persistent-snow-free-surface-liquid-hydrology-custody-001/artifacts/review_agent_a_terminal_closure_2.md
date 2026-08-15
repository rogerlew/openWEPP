# Review Agent A — Terminal Closure 2 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained Ran evidence`

Reviewed commit: `1b76bd12ec5bb98b031c9fbeed3cd35d93afd597`

Verdict: `HOLD / NO-GO`.

## Findings

### High — The duplicated arithmetic preflight omits OFE aggregate comparison arithmetic

The new dedicated preflight fixes the demonstrated store-order defect: it
evaluates every store equation without returning a finite mismatch, so its new
two-store poison correctly reports the later store's E003. It is not exhaustive
over every arithmetic operation performed by the final independent validator.

The preflight constructs `expected_ofe_enthalpy` and
`actual_ofe_enthalpy` maps at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:785-821`,
but never supplies the paired totals to `checked_surface_liquid_close()` before
returning at `:858`. The final validator does perform this operation through
`require_close_enthalpy()` at `:1121-1129`. Therefore tolerance difference,
scale, scaled tolerance, and tolerance addition for the OFE aggregate remain
outside the purported E003 preflight.

This is reachable with finite operands. Two parcel keys in one OFE can each
have an actual/expected enthalpy pair whose individual comparison scale is
finite, while the separately finite actual and expected OFE totals overflow
only when their absolute values are added for the aggregate comparison. The
preflight observes only finite per-key mismatches and intentionally ignores
them. A simultaneous producer-field mismatch then publishes E009 before the
final aggregate comparison can publish the contract-precedent E003. With no
producer mismatch, an earlier finite E010 in the short-circuiting final
validator can likewise hide that later aggregate E003.

The divergence is a consequence of substantial mirrored science-sensitive
logic. The 300-line preflight separately rebuilds store, source-parcel,
receipt, routing, and aggregate arithmetic already implemented by the final
validator. It has already drifted in a second place: routed additions are
accumulated into a new `routed_expected` map at `:683-781`, whereas final
closure adds them to the evolving `expected` join map at `:1085-1099`.
Consequently the preflight does not exercise the same existing-plus-routed
addition and association order as the authoritative final equation. This
duplication can silently diverge again as either path changes and is therefore
itself closure-blocking under the repository's duplication policy.

Required correction: centralize the shared arithmetic projection so the
preflight and final closure consume the same checked intermediates while only
their finite-comparison disposition differs. The arithmetic stage must run the
same per-OFE aggregate comparison and routed join additions, returning E003 on
`None` while retaining `Some(false)` for the later E010 stage. Add a
multi-parcel same-OFE comparison-scale poison, separately combined with E009
and with an earlier finite E010, and assert exact OFE identity plus beginning
and attempted hashes.

### High — Positional E009 localization reintroduces shifted-row deletion attribution

The new producer comparison removes the unconditional first-record fallback
and correctly localizes the added same-length second-row mutations. Its generic
`first_positional_mismatch()` helper at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:487-494`
returns the actual row at the first unequal position whenever one exists.
`state_mismatch_context()` and the receipt and ledger branches use that helper
at `:423-460` and `:496-511`.

For a nonterminal deletion this names the shifted following row, not the
missing expected row. With expected records `[upper, lower]` and actual records
`[lower]`, index zero differs and the helper returns actual `lower`; the
producer E009 therefore reports lower OFE/tile/surface/source even though the
missing offender is upper. The same error shape applies to continuation,
receipt, and ledger deletion. The attempted ending-state digest remains
computable, so typed absence does not justify substituting the shifted row.

This repeats the deletion-context defect previously corrected for independent
receiver and rollback sequences. The retained membership-aware implementation
at `land_surface_energy_shadow/mod.rs:1407-1435` first finds an expected member
absent from a shorter actual sequence, while preserving actual identity for
equal-length replacement or reorder. The new producer helper does not reuse
that established rule.

The added E009 test at `surface_liquid_ingress.rs:2591-2671` mutates the value
of an existing second ending record, ledger, and WB14 entry. Those controls
correctly prove later-row localization and rollback hashes for equal-cardinality
data, but they do not cover missing nonterminal rows.

Required correction: use a centralized membership-aware sequence comparison
for unique ordered producer collections. A shorter sequence must report the
first expected identity absent from actual; equal-length malformed/reordered
rows must retain the actual first offender. Add first and nonterminal deletion
poisons for state records/continuations, receipts, and ledgers, asserting exact
E009 phase, owner, applicable OFE/tile/surface/source/parcel, and both rollback
hashes. Retain the current later-value and WB14-map controls.

## Prior Finding Closure And Retained Correctness

The remediation materially closes the exact cases it exercises:

- `DirectSurfaceLiquidIngressCandidate::validate()` now invokes a dedicated
  arithmetic-only entry point before one immutable producer reconstruction at
  `surface_liquid_ingress.rs:337-373`. The full independent validator remains
  after producer E009 at `:386-405`, so finite E010 is the final stage.
- The two-store poison at `surface_liquid_ingress.rs:2539-2588` proves that an
  earlier finite store E010 no longer hides a later store-equation E003. It
  asserts the later store, phase, transaction, owner, and exact rollback
  hashes.
- Direct second ending-store, ledger-OFE, and WB14-map value mismatches now
  return E009 without any first-record fallback. The wrong infiltration
  recipient remains exact E009, and a producer-valid frozen-operand mismatch
  remains final E010.
- The original mass/enthalpy close consumers still distinguish
  `Some(true)`, finite `Some(false)` as E010/E011, and arithmetic `None` as
  E003. The earlier checked receiver division and accumulation correction is
  unchanged, and no raw receiver water-density division or cited `+=`
  accumulation has returned.
- The existing membership-aware independent receiver/rollback deletion
  implementation remains unchanged. The second finding concerns only the new
  producer-context helper.
- No tolerance, clamp, unit conversion, WB14 transition, source operand,
  science authority, model identity, production selector, publication path,
  or cutover was changed by the reviewed Rust delta.

Line-count governance remains numerically compliant and explicitly
dispositioned: `surface_liquid_owner.rs` is 2,347 lines,
`surface_liquid_owner_tests.rs` 876, `surface_liquid_ingress.rs` 2,894,
`surface_liquid_closure.rs` 1,356, `land_surface_energy_shadow/mod.rs` 2,881,
`direct_runtime/runoff.rs` 2,852, and `surface_liquid_wb14.rs` 303. No affected
production file reaches 3,000 lines and no lint suppression was added. The
2,894-line ingress module remains a documented WARN with future split intent;
the duplicated closure algorithm above is a correctness finding independent
of the numeric line threshold.

All earlier review and failed-attempt artifacts are preserved. This review
adds only this terminal artifact and does not alter concurrent Review B work.

## Exact-Commit Validation

Ran against the clean exact commit
`1b76bd12ec5bb98b031c9fbeed3cd35d93afd597`:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 39/39 selected; 507 skipped by filter

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
PASS: 546/546; 0 skipped; three known slow OFE-routing oracle tests completed

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before review artifacts were added
```

These gates verify the implemented later-store and same-cardinality
later-record controls. They contain neither an OFE aggregate comparison-scale
poison nor a producer collection deletion poison, so they cannot close the two
findings.

The retained heavy evidence at `74d512f44` remains truthful for its exact
bytes: workspace strict Clippy, 2,783/2,783 full Nextest, workspace doctests,
dependency policy, formatting, and diff hygiene passed. It predates the later
arithmetic/preflight changes and does not replace exact-final-byte terminal
qualification after correction.

## Residual Risk And Missing Tests

- Add route-aggregate overflow and nonzero-underflow poisons that distinguish
  adding into an existing expected destination entry from aggregating routed
  additions in a separate empty map.
- Add an order-reversed store poison as a control and combined aggregate
  E003/E009/E010 poisons so global precedence is independent of iteration
  order and arithmetic surface.
- Add producer deletion/cardinality poisons; direct second-row mutation proves
  neither missing-member attribution nor reorder semantics.
- Exact-final-byte full-workspace, doctest, dependency-policy, anti-evasion,
  and any package-required comparator gates remain terminal evidence after the
  findings are corrected.

## Approval Statement

`NO-GO`: commit `1b76bd12e` closes the demonstrated later-store E003 and direct
later-record E009 cases, retains final E010 and the earlier arithmetic,
receiver-deletion, rollback, science, and line-count corrections. Dependency
package closure remains blocked because the parallel preflight omits and
diverges from arithmetic evaluated by final closure, and its new positional
producer helper can again name a shifted row instead of the missing offender.
