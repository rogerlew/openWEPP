# Review Agent A — Terminal Closure 5 Rust Correctness Review

Evidence class: `Static exact-commit + Ran exact-commit + reconciled retained Ran evidence`

Reviewed commit: `cf6acd2f5aaf0b131cba5de77a83b186f29702a7`

Verdict: `HOLD / NO-GO`.

## Findings

### Critical — Independent closure collapses chronological subinterval mixtures into one whole-OFE mixture

The production correction restores the canonical constitutive calculation.
`advance_one_ofe()` constructs exact parcel boundaries and per-window
contributions at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs:1206-1267`,
checked-sums `X_b` and `Q_b`, and derives one `h_mix,b` at `:1276-1358`.
Every attributed infiltration child uses that value at `:1362-1458`, and the
same value is passed to retained-water and runoff partition at `:1461-1469`
and `:1542-1703`. Routing preserves the resulting parent temperature while
scaling mass and enthalpy exactly once at `:1753-1816`. No source-specific
temperature calculation remains in those production paths.

The independent projection implements a different equation. It first
aggregates every source over the entire 1,800-second transaction by
`(source_parcel_id, basis_ofe_id)` at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_closure.rs:931-1056`.
It then sums those whole-source values by OFE at `:1058-1075`, derives one
whole-OFE `h_mix` at `:1076-1098`, and overwrites every source's expected
enthalpy with `whole_source_mass * whole_OFE_h_mix` at `:1099-1115`.
Although source operands now contain `start_s` and `end_s`, neither field is
read by `project_parcel_arithmetic()` or any other arithmetic function.

That is not the binding equation in
`docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md:370-401`,
which defines `X_b`, `Q_b`, and `h_mix,b` for every exact chronological
subinterval `b`. The difference is observable on admitted inputs. For example,
two open tiles in one OFE may carry equal masses on disjoint half-interval
supports at 280 K and 290 K. Production correctly emits first-window children
at 280 K and second-window children at 290 K. The independent projection
expects both source totals at the whole-interval 285 K mixture, so
`capture_and_validate_surface_liquid_closure()` rejects the correctly produced
candidate with E010. Partially overlapping local precipitation or routed runon
has the same defect and cannot be independently reconstructed by this map.

Whole-interval aggregation can also manufacture E003: disjoint source
enthalpies that are individually finite in their active windows can overflow
when summed together even though production never combines them in one
`Q_b`. Thus the defect affects both finite closure and global arithmetic
precedence.

Required correction:

1. Independently construct the exact boundary set for every current basis OFE
   from frozen local, overflow, and routed-source support.
2. Reconstruct `x_p,b`, `q_p,b`, `X_b`, `Q_b`, and the exact zero-supply branch
   per subinterval, then accumulate each source/basis post-mix expected mass
   and enthalpy from those window rows.
3. Close raw-to-post-mix enthalpy for each window before accumulating the OFE
   transaction total; retain the contract's checked final-child mass and
   enthalpy remainder behavior rather than relying only on repeated
   `mass * h_mix` products.
4. Add disjoint and partially overlapping unequal-temperature vectors,
   unequal source durations, multi-tile mixtures, and multi-hop routed carry.
   Assert infiltration, retention, runoff, and downstream receipts against
   the correct per-window common temperature.

This is a material independent-ledger defect, not a new authority question.
The producer is now scientifically correct, but the public execution path
cannot accept the complete timed domain because independent closure remains
mandatory.

### High — Frozen source support is false and caller order becomes an unintended acceptance condition

Open raw precipitation explicitly admits partial interval support:
`DirectIngressAmount::validate(false)` accepts finite
`0 <= start_s < end_s <= 1800` at
`surface_liquid_ingress.rs:120-143`, and production copies the exact input bits
into `TimedParcel` at `:1092-1133`. The independent capture instead writes
every local source as `start_s=0` and `end_s=INTERVAL_S` at
`surface_liquid_closure.rs:763-800`. Its supposedly independent expected
identity reconstruction repeats those same constants at `:851-894` rather
than reading the immutable ingress input. A valid partial-support source is
therefore frozen with false timing, and a support poison cannot be detected
against the real input.

The same identity pass introduces an order regression. `capture_source_parcels()`
iterates `input.tile_ingress` in caller order at `:708-760`, while
`validate_frozen_source_identities()` constructs `expected` in canonical
configuration-record order and requires exact vector equality at `:856-903`.
The ingress validator accepts any complete unique tile set and production sorts
the resulting parcels by canonical parcel identity before arithmetic
(`surface_liquid_ingress.rs:869-980` and `:1163`). Reversing two otherwise
identical valid tile-ingress rows therefore leaves production semantics
unchanged but causes independent E010 solely because caller order differs.
The contract requires canonical source order to make floating remainders
deterministic; it does not authorize caller order to select acceptance.

Required correction: preserve exact input support in every frozen local source;
derive the expected local identity set from the validated immutable ingress
records; add resource-derived overflow identity; and canonicalize expected and
actual identities with the same admitted parcel ordering before sequence and
cardinality comparison. Exact-zero rows must remain present. Add partial-support,
support-bit poison, input-reorder, frozen-reorder, zero/nonzero deletion,
replacement, duplicate, re-key, and kind controls. Nonfinite or invalid frozen
support must retain global E003 domain precedence rather than becoming a late
generic identity mismatch.

## Prior Finding Closure And Retained Correctness

The latest remediation otherwise materially closes the three closure-4
findings on the full-interval domain:

- Production again assigns one common `h_mix,b` to every simultaneous source
  child across infiltration, retained water, runoff, outlet, and routed carry.
  The mixed 280/290 K canopy vector proves common temperature for
  infiltration, retention, and runoff receipts.
- Raw source mass/enthalpy operands remain separate from actual post-mix
  receipts. The shared parcel projection compares mass and post-mix enthalpy
  for every `(source_parcel_id,basis_ofe_id)` key, then performs the OFE
  enthalpy total and raw-to-post-mix total comparisons through the same
  arithmetic-preflight/final disposition. Its remaining defect is temporal
  projection, not a return to source-specific output temperature.
- Frozen source cardinality now includes exact-zero configured sources.
  Deletion, duplicate, source-ID re-key, and kind-swap controls fail E010 for
  canonical full-interval input. The support and caller-order gaps above keep
  the complete identity claim open.
- OFE aggregate comparison uses `contextual_ofe_comparison_failure()`, so
  owner/OFE remain present while tile/surface/source are typed absent.
  `projection_key_store()` no longer silently selects the first tile when a
  multi-tile destination cannot be uniquely proven.
- Store preflight and final validation still consume the one shared checked
  `W0-F+C-overflow+retained` projection. Routed area arithmetic retains exact
  destination context where a destination is known.
- Candidate validation order remains exhaustive arithmetic E003 preflight,
  immutable producer E009 reconstruction, then finite independent E010.
  Membership-aware producer deletion/replacement/reorder attribution and
  rollback hashes are unchanged.
- D/A/F, signed condensation, finalized-use-only debit, checked receivers,
  WB14 continuation, restart lineage, three-owner rollback, snow/frost
  exclusion, and default-off production selection are unchanged. No selector,
  scheduler, publication, activation, or cutover path changed in this
  increment.

The small comparison adapters remain parallel: per-store/per-key comparison
uses `compare_projected_value()` while OFE-only comparison uses the equivalent
`compare_ofe_value()`, and final store validation retains
`require_close_mass()`. No current code/tolerance drift was found, but
centralizing result disposition while parameterizing context shape would
reduce future E003/E010 divergence.

Line-count governance remains compliant. Current affected counts are 2,347
lines for `surface_liquid_owner.rs`, 876 for
`surface_liquid_owner_tests.rs`, 1,927 for `surface_liquid_ingress.rs`, 1,601
for `surface_liquid_ingress_tests.rs`, 1,662 for
`surface_liquid_closure.rs`, 2,881 for `land_surface_energy_shadow/mod.rs`,
2,852 for `direct_runtime/runoff.rs`, and 303 for
`surface_liquid_wb14.rs`. Every affected production file remains below the
mandatory 3,000-line threshold and every edited Rust file is below 2,000
lines.

All prior findings and failed-attempt artifacts remain preserved. This review
adds only the named Agent A artifact and does not modify the concurrent Review
B artifact.

## Exact-Commit Validation

Ran against exact commit
`cf6acd2f5aaf0b131cba5de77a83b186f29702a7`; the only preexisting worktree
addition was the concurrent Review B closure-5 artifact:

```text
cargo nextest run --profile quick \
  --test surface_liquid_hydrology_custody_authority_contract \
  --test land_surface_energy_real_hydrology_shadow_contract
PASS: 28/28; 0 skipped

cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
PASS: 554/554; 0 skipped; three known slow OFE-routing oracle tests completed

cargo clippy -p openwepp-hillslope-orchestrator \
  --all-targets --all-features -- -D warnings
PASS

cargo fmt --all -- --check
PASS

git diff --check
PASS before this Agent A artifact was added
```

These gates prove full-interval mixing, canonical-order zero-source controls,
and OFE-only aggregate context. They do not include a partial-support source,
disjoint or partly overlapping unequal-temperature sources, input tile reorder,
or timed downstream runon. Passing them cannot establish Section 6 temporal
parity.

The retained full-workspace, doctest, dependency-policy, AUTH11, anti-evasion,
and science-admission evidence predates the closure-5 implementation. Because
a material contract defect is already established, repeating the expensive
terminal campaign cannot make this commit releasable.

## Residual Risk And Missing Tests

- Add a valid one-source partial-support vector. It should freeze exact timing
  even when no mixed-temperature comparison makes the current lie observable.
- Add disjoint and overlapping multi-tile/open-plus-covered mixtures and assert
  per-window `h_mix,b`, per-source accumulated post-mix enthalpy, raw window
  totals, receiver totals, and exact rollback on poison.
- Route a partial-support source through unequal-area multi-hop topology and
  mix it downstream with local full-interval supply; assert exact support,
  area scaling, destination context, and raw/post-mix window closure.
- Add nonfinite/invalid frozen-support poisons and combined later-arithmetic,
  producer, and finite-closure defects to retain global precedence.
- After correction, rerun exact-final-byte full-workspace Nextest, strict
  workspace Clippy, doctests, dependency policy, AUTH11/anti-evasion, science
  admission, formatting, diff hygiene, and package Markdown lint.

## Approval Statement

`NO-GO`: commit `cf6acd2f5` restores canonical production `h_mix,b`, closes
full-interval zero-source identity and OFE-context findings, and retains D/A/F,
checked receivers, rollback/restart, and production exclusion. Dependency
closure remains blocked because the independent validator uses one
whole-transaction OFE mixture instead of the binding chronological
subintervals, erases admitted raw-precipitation support, and makes caller tile
order an unintended acceptance condition. Correct the temporal projection and
source freeze, add nondegenerate timed vectors, and rerun terminal gates before
another closure review.
