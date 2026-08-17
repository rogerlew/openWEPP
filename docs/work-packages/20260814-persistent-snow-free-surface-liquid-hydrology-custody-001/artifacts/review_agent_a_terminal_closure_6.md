# Review Agent A — Terminal Closure 6 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained evidence`

Reviewed commit: `bce6fa8300321a76ad4f2db36164eabbc57f5cc2`

Verdict: `HOLD / NO-GO`.

## Findings

### Critical — The independent join discards window identity and trusts routed producer support

The remediation now computes a checked chronological mixture in the independent
projection. `project_parcel_arithmetic()` forms the boundary union, slices raw
segments, and computes a distinct `h_mix,b` at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:1110-1206`.
It separately accumulates raw window enthalpy and post-mix attribution at
`:1207-1245`. That closes the closure-5 whole-OFE-average defect for an
unmodified positive-path candidate.

The comparison discards the chronology it just reconstructed. `ParcelJoinKey`
contains only `(source_parcel_id,basis_ofe_id)` at `:397-401`. Every actual
receipt, regardless of support or disposition, is accumulated into that map at
`:1029-1055`; every independently reconstructed window row is accumulated into
the same source/basis map at `:1220-1244`. Final comparison therefore proves
only the transaction-total mass and enthalpy for each source/basis, followed by
an OFE transaction total at `:1296-1383`. `validate_parcel_joins()` checks
recipient shape and each receipt's internal temperature/enthalpy relation, but
never compares a receipt's support or specific enthalpy with the independently
computed mixture for that exact window (`:1510-1585`).

This admits a direct constitutive counterexample. Give source A equal mass in
two consecutive windows, with source B entering only the second window so that
the two accepted `h_mix,b` values differ. Swap A's two window enthalpies and
temperatures while leaving each receipt internally consistent. A's aggregate
mass and enthalpy are unchanged, the OFE attributed total is unchanged, the raw
OFE total is unchanged, and every temperature/enthalpy join passes. The
independent validator therefore accepts outputs that violate the binding rule
that every attributed parcel uses its own exact-window `h_mix,b`. The immutable
E009 reconstruction catches a post-construction test mutation, but it reruns
the same production implementation and cannot detect that implementation
producing the swapped windows itself. It is not independent science evidence.

Routed timing is more circular. For every actual `RoutedRunoff` receipt, the
projection applies the checked area ratio and creates the destination raw
segment directly from that receipt's mass, enthalpy, `start_s`, and `end_s` at
`:1056-1107`. A producer defect that shifts routed support is consequently
adopted as the independent downstream expectation; the shifted interval then
controls the destination boundary set and mixture. The upstream source/basis
aggregate does not compare support, so neither side detects the loss of the
contract's support-preservation invariant. No new test routes a partial-support
parcel, overlaps it with destination-local supply, or carries it through more
than one unequal-area hop.

Canonical arithmetic order is also not shared. Production sorts active parcels
by support, origin, kind, and parcel ID in `surface_liquid_ingress.rs:1177-1184`
and `:1897-1904`. Frozen local operands are sorted by the derived field order of
`FrozenSourceIdentity` at `surface_liquid_closure.rs:403-421` and `:812`, while
routed segments retain producer receipt iteration order. The independent
window sums at `:1166-1185` therefore need not use the production canonical
source order. Besides preventing validation of final remainder ownership, this
can change floating summation, `h_mix,b` bits, or E003 overflow disposition on
large signed enthalpy operands. This duplicated ordering logic needs one shared
canonical identity key even though the arithmetic reconstruction itself must
remain independent.

Required correction:

1. Retain exact window identity in the independent projection. At minimum,
   compare every actual child's support and `Q_child/m_child` with the
   independently reconstructed `h_mix,b`; aggregate source/window mass and
   enthalpy before the existing source/basis and OFE totals.
2. Reconstruct the upstream routed child from the independent upstream window
   result, including canonical final mass and enthalpy remainders. Compare the
   routed receipt to that result, then apply `A_u/A_d` exactly once to construct
   destination runon. Do not use the actual routed receipt as its own timing or
   energy authority.
3. Centralize the canonical timed-source ordering key used by production and
   validation. Retain independent formulas but eliminate the conflicting sort
   definitions.
4. Add poisons that exchange equal-mass enthalpy/temperature between distinct
   windows and that shift routed support while keeping all transaction totals
   unchanged. Add disjoint, partially overlapping, unequal-area, and multi-hop
   positive vectors with explicit per-window and per-child expected values.

This is closure-blocking under `INV-SURFACELIQUID-007..008`: the current
validator reconstructs the correct window equation but does not prove that the
producer published that equation on the exact chronological and routed seams.

### High — Arithmetic preflight does not enforce support and receipt domains as E003

The public validator deliberately runs an exhaustive E003 preflight before
E009 producer reconstruction. The new parcel preflight validates frozen source
mass and enthalpy at `surface_liquid_closure.rs:1002-1015`, but it does not
validate finite ordered support within `[0,1800]`. It then puts raw
`start_s/end_s` values into the boundary set and uses comparisons and unchecked
`end_s-start_s` expressions at `:1116-1136`. With a NaN start, for example, the
source simply participates in no finite window; the finite expected/actual
mismatch is intentionally ignored during preflight, and later immutable
reconstruction returns E009. Reversed or out-of-range finite support similarly
becomes E009 rather than the contract's earlier E003 interval-domain failure.

The final identity pass cannot supply the missing domain check because it
copies each actual operand's support bits into its expected identity at
`:949-957`. E009 correctly detects a finite support mutation relative to the
immutable input, as the new test shows, but it does not preserve global error
precedence for malformed candidate operands.

Receipt preflight has the same taxonomy gap. `validate_receipt_enthalpy()` at
`:1674-1707` returns a generic closure error for negative mass or nonfinite
temperature, and it checks only temperature finiteness rather than the admitted
`200..=350 K` domain. `project_parcel_arithmetic()` propagates that result only
when it is already E003 at `:1030-1035`; otherwise it continues to a finite
comparison and the public path reports the later E009 producer mismatch.
Receipt support is not domain-checked at all. These are explicit branch-table
E003 conditions, not ordinary candidate equality failures.

Required correction: validate every frozen and receipt support as finite,
`0 <= start_s < end_s <= 1800`, before sorting or projection; validate receipt
mass and the full temperature domain in that same arithmetic/domain preflight;
and return contextual E003 directly. Add NaN, infinity, reversed, negative,
out-of-range, and routed-support poisons combined with later E009/E010 defects
to prove global precedence and typed context.

## Closed Prior Findings And Retained Correctness

- Local closure operands now copy the actual validated ingress support rather
  than hardcoding `[0,1800)`, and `capture_source_parcels()` sorts the complete
  frozen source vector. Reversing the complete unique caller tile-ingress set
  produces identical candidate bytes.
- Exact-zero configured sources remain present. Deletion, duplicate, re-key,
  kind swap, and finite support mutation are localized by immutable producer
  reconstruction. The domain-precedence gap above does not reopen their
  cardinality behavior.
- For each OFE, the positive-path projection now creates the correct union of
  local and routed segment boundaries, computes raw window mass/enthalpy and
  one nonzero-window mixture, and keeps raw OFE enthalpy separate from
  post-mix attribution. It no longer uses one transaction-wide OFE mixture.
- The destination area ratio remains a checked, once-only `A_u/A_d`
  conversion. Per-key and OFE comparisons retain destination or OFE-only
  context; multi-tile aggregates do not fabricate a tile.
- The closure-6 diff changes no production mixing arithmetic. The only change
  to `surface_liquid_ingress.rs` is the 14-line frozen-source mismatch context
  check at `:437-450`. Production still computes one `h_mix,b` and applies it
  across infiltration, retention, runoff, and routed carry.
- D/A/F, signed condensation, store projection, WB14 continuation, receiver
  reconstruction, restart serialization, rollback hashes, snow/frost
  exclusion, and default-off production selection are unchanged by this
  increment.

Line-count governance remains compliant. Current affected counts are 2,347
lines for `surface_liquid_owner.rs`, 876 for
`surface_liquid_owner_tests.rs`, 1,941 for `surface_liquid_ingress.rs`, 1,653
for `surface_liquid_ingress_tests.rs`, 1,793 for
`surface_liquid_closure.rs`, 2,881 for `land_surface_energy_shadow/mod.rs`,
2,852 for `direct_runtime/runoff.rs`, 303 for
`surface_liquid_wb14.rs`, 2,783 for `00_core_frames.rs`, and 2,157 for
`vegetation_real_hydrology_shadow.rs`. No affected file reaches the mandatory
3,000-line threshold; the existing 2,000-line WARN files retain their recorded
decomposition rationale. Every Rust file edited in closure 6 remains below
2,000 lines.

All prior failed reviews and HOLD artifacts remain preserved. This review adds
only the named Agent A artifact and does not modify the concurrent Agent B
closure-6 artifact.

## Exact-Commit Validation

Ran against exact commit
`bce6fa8300321a76ad4f2db36164eabbc57f5cc2`; the checkout was clean before the
concurrent Review B artifact appeared:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
PASS: 556/556; 0 skipped; three known slow routing-oracle tests completed

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this Agent A artifact was added

git diff --check cf6acd2f5..bce6fa830
PASS
```

The new partial-overlap test proves that one unmodified candidate traverses the
new boundary arithmetic, but it asserts no per-window expected temperature or
enthalpy. Its support poison proves E009 reconstruction only. The caller-order
test proves stable candidate bytes. Neither test exercises the aggregate-preserving
window swap, invalid-support E003 precedence, or partial routed/multi-hop
support. Passing gates therefore do not close the findings above.

The last full-workspace, doctest, dependency-policy, AUTH11, anti-evasion, and
science-admission campaign evidence predates this conservation-sensitive
implementation. It was not repeated after a material static defect was
established; a passing campaign cannot make the reviewed bytes releasable.

## Residual Risk And Missing Tests

- Assert independent expected values for every pre-overlap, overlap,
  post-overlap, and empty window, including each child receipt rather than only
  candidate acceptance.
- Add aggregate-preserving cross-window temperature/enthalpy swaps and
  within-window opposing child-temperature poisons.
- Route partial support across unequal-area multi-hop topology, mix it with
  destination-local supply, and independently assert parent support, area
  conversion, destination window boundaries, child temperatures, and outlet
  totals.
- Add zero-mass source support boundaries and exact canonical remainder-owner
  controls whose input order differs from support order.
- Add malformed frozen/receipt support, mass, and temperature poisons with
  combined later failures to prove E003 precedence and rollback hashes.
- After correction, rerun exact-head full-workspace Nextest, strict workspace
  Clippy, doctests, dependency policy, AUTH11/anti-evasion, science admission,
  formatting, diff hygiene, and package Markdown lint.

## Approval Statement

`NO-GO`: commit `bce6fa830` corrects the closure-5 whole-OFE averaging and raw
source-support capture defects on the unmodified positive path, while leaving
production `h_mix,b` unchanged. Dependency closure remains blocked because
independent comparison erases exact window identity, derives downstream timing
from the routed producer receipt, uses a different source ordering, and does
not preserve E003 precedence for malformed support or receipt domains. Retain
the chronological projection, close those validation seams with nondegenerate
poisons, and rerun terminal gates before another closure review.
