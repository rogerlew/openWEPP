# Review Agent A — Terminal Closure Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained Ran evidence`

Reviewed commit: `63d3edde677409d9bb9170129528a9941513653a`

Verdict: `HOLD / NO-GO`.

## Findings

### High — The filtered closure call is not a complete E003 preflight

`DirectSurfaceLiquidIngressCandidate::validate()` calls the complete,
short-circuiting independent validator first and returns its result only when
the first error happens to be E003 at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:337-356`.
It ignores an initial E010, reconstructs the producer once, and invokes the
same complete validator again after the E009 comparison.

That shape does not preserve E003 precedence across a multi-store or
multi-parcel candidate. `validate_store_equations()` visits stores in order at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:553-620`
and returns immediately from the first failed finite `require_close_mass()`.
A reachable crate-level poison can therefore give the first routed store a
finite beginning-state mismatch and the second store the existing
large-finite beginning-plus-retained arithmetic overflow. The first pass
returns E010 for store one and discards it. Producer fields still compare equal
because the deliberately independent closure operands are excluded. The
second complete pass returns the same store-one E010, so the later store-two
E003 is never evaluated.

This violates the ordered branch table in
`docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md:463-477`,
which assigns nonfinite/out-of-domain arithmetic to E003 before E009 and E010.
It also means the prior final finding is not fully closed: the implementation
has the intended three visible stages, but its first stage does not establish
that no E003 exists.

The new poison matrix cannot expose this defect. Both the finite E010 poison
at `surface_liquid_ingress.rs:2351-2410` and the E003 overflow poison at
`:2413-2473` use `one_tile_configuration()` and exercise only one defect at a
time. Their exact code, phase, identity, and rollback assertions are valid for
those isolated cases.

Required correction: make the first stage an actual arithmetic/domain pass
that cannot stop on an ordinary finite closure mismatch, then retain exactly
one immutable producer reconstruction/field comparison for E009 and one
independent finite-closure disposition for E010. Centralize the shared
arithmetic evaluation rather than copying the store/parcel equations into a
parallel validator. Add a two-store combined poison with an earlier finite
E010 and later E003, asserting E003 plus the later store's exact identity and
beginning/attempted hashes. A companion combination with a producer E009
poison would prove that E003 also precedes E009.

### High — Non-receipt E009 mismatches are attributed to the first store

The new `producer_mismatch_context()` localizes only receipt-vector
differences. If receipts compare equal, it unconditionally falls back to
`configuration.records.first()` at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:417-451`.
However, `producer_fields_equal()` also rejects beginning state, ending state,
ledger, and per-OFE WB14-call mismatches at `:408-415`.

On the existing two-store routed configuration, poisoning only the second
ending-state row or second ledger therefore returns E009 with the first
store's OFE/tile/surface/source. This repeats the exact-offender substitution
class corrected earlier for E011 and conflicts with the contract requirement
that applicable public identities name the actual offender. The sealed
candidate forgery control at `surface_liquid_ingress.rs:2636-2663` mutates only
the first ending-state row, so the fallback happens to look correct and no
two-row E009 context test detects the error.

Required correction: compare each producer-owned field with field-specific
localization and use the first actual/expected mismatching row or OFE. Preserve
typed absence only when an identity is genuinely unavailable. Add second-row
ending-state, ledger, and WB14-call poisons asserting exact E009 phase,
transaction, owner, OFE/tile where applicable, and both rollback hashes.

## Prior Finding Closure And Retained Correctness

The bounded remediation otherwise preserves the intended mechanics:

- The producer is reconstructed exactly once from immutable configuration,
  resource, and ingress inputs. Producer-owned fields exclude the independent
  closure operands, so an isolated wrong recipient is E009 and an isolated
  producer-valid closure-operand mismatch reaches E010.
- The wrong-infiltration-recipient poison now asserts E009 in
  `IngressCandidate`, including the actual receipt's transaction, owner,
  OFE/tile, parcel, beginning hash, and attempted hash.
- The isolated large-finite closure poison asserts E003 with exact available
  store identity and rollback hashes. The isolated finite closure poison
  remains E010 in `IndependentClosure` with the same rollback coverage.
- Repository-wide production consumers of the checked close primitive remain
  tri-state: finite equality succeeds, finite mismatch returns E010/E011, and
  arithmetic indeterminacy returns E003. The earlier raw receiver division and
  accumulation findings remain closed.
- The membership-aware expectation/rollback deletion correction remains
  unchanged. Missing rows use expected identity, while equal-length malformed
  rows retain actual identity.
- No tolerance, unit conversion, WB14 transition, science operand, owner
  boundary, model identity, selector, publication path, or production cutover
  changed in the reviewed Rust delta.

Line-count governance remains closed. Current affected counts are 2,347 lines
for `surface_liquid_owner.rs`, 876 for `surface_liquid_owner_tests.rs`, 2,664
for `surface_liquid_ingress.rs`, 1,038 for `surface_liquid_closure.rs`, 2,881
for `land_surface_energy_shadow/mod.rs`, 2,852 for `direct_runtime/runoff.rs`,
and 303 for `surface_liquid_wb14.rs`. No affected production file exceeds the
mandatory 3,000-line threshold, and no lint suppression was added.

All earlier review and failed-attempt artifacts were preserved. This review
adds only this terminal artifact; concurrent package-disposition and Review B
work was not altered.

## Exact-Commit Validation

The reviewed Rust files are byte-identical to commit
`63d3edde677409d9bb9170129528a9941513653a`. Ran against those bytes:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator \
  surface_liquid --profile quick
PASS: 37/37 selected; 507 skipped by filter

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
PASS: 544/544; 0 skipped; three known slow OFE-routing oracle tests completed

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS
```

These gates verify the implementation and its isolated poison expectations.
They do not exercise the ordered multi-fault counterexample or second-store
E009 localization and therefore cannot close either finding.

The retained heavy evidence at `74d512f44` remains truthful for its exact
bytes: workspace strict Clippy, 2,783/2,783 full Nextest, workspace doctests,
dependency policy, formatting, and diff hygiene passed. It predates the later
arithmetic and ingress-precedence changes and does not replace exact-final-byte
terminal qualification after correction.

## Residual Risk And Missing Tests

- Add an order-reversed combined poison as a control so E003 classification is
  independent of record iteration order.
- Add a combined receipt E009 plus later arithmetic E003 poison to prove the
  complete E003-before-E009-before-E010 ordering, not merely three isolated
  outcomes.
- Add second-row E009 localization for every non-receipt producer-owned
  collection. Exact rollback hashes are already covered; exact offender
  identity is not.
- Exact-final-byte full-workspace, doctest, dependency-policy, anti-evasion,
  and any package-required comparator gates remain terminal evidence after the
  findings are corrected.

## Approval Statement

`NO-GO`: commit `63d3edde6` restores the isolated wrong-recipient E009 case,
retains isolated E003/E010 behavior and rollback hashes, and leaves the prior
arithmetic, deletion, science, and line-count corrections intact. Dependency
package closure remains blocked because the claimed E003 preflight can publish
an earlier E010/E009 while a later E003 exists, and non-receipt E009 failures
can name the wrong store.
