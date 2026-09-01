# V48 fixed-point final-install authority correctness review

Status: `APPROVE`

Evidence mode: `Static + Ran`

## Findings

No correctness findings remain.

The prior medium-severity evidence finding is resolved. The expanded V48
suite now executes the production finalizer helper with literal source
transaction 42, target 43, predecessor 42, and support `1800..1980 s`; covers
authenticated same-ID and exact accepted-resident no-op behavior; and applies
distinct prepared support/receipt and accepted target/predecessor/support/
receipt/state/layer/seal substitutions. Every refusal checks byte-exact soil
rollback, unchanged vegetation/LSE/BGC source owners, and unchanged accepted
publication history.

## Correctness assessment

The builder at
`crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow/v10_soil_thermal_v2.rs:2223`
validates the authoritative native-V2 resident and exact prepared beginning,
derives the source solely from mutually equal vegetation/LSE/BGC owners, and
requires the prepared exact predecessor to equal source for a split. The
authenticated installer at `:2263` reconstructs the accepted resident and
receipt/orchestrator seals from that same beginning, admits only a candidate
resident equal to the authoritative beginning or exact accepted ending,
reconstructs and compares the explicit native-V2 authority, and delegates to
the unchanged V47 atomic posture before clone-and-replace mutation.

The generic/public installer continues to pass `None` and is same-ID-only.
Both ordinary native-V2 finalization fallbacks in
`crates/openwepp-hillslope-orchestrator/src/v11_covered/owner_finalization.rs`
use the single typed V48 helper; continuation finalization remains on its
separate authenticated path. No adjacency inference, source/target copying,
accepted-custody weakening, mutation before validation, private publication,
diagnostic persistence, arithmetic change, or serialization change was found.
Error mapping remains within the existing typed identity/owner-closure
taxonomy. The localized `clippy::result_large_err` allowance changes neither
error construction nor propagation. No substantial duplicated Rust algorithm
was introduced.

## Ran evidence

Ran independently after the HOLD fixes:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v48_/)'
```

Nextest run `3f2743f0-4cd5-4f05-9dc6-525f84fb54aa`: `7 passed; 0 failed`.

Reviewed current retained evidence:

- implementation V48 run `a9ee143a-b748-474e-bf17-e2fd523eed09`: `7/7`;
- retained V39/V46/V47/V48 behavior: `36/36`;
- complete snow source-contract target: `40/40`;
- persisted-restart crate: `40/40`;
- orchestrator all-target/all-feature check: `PASS`.

Current touched line counts are 2,468 for `v10_soil_thermal_v2.rs`, 2,956 for
its included test source, and 2,933 for `owner_finalization.rs`: all remain
below the 3,000-line closure threshold. The package records the required WARN
split disposition for the 2,000-line files.

## Residual risk and missing tests

No focused V48 test gap remains. Parent-owned canonical r123 is still required
to prove the real one-day consumer proceeds beyond the corrected final install
with unchanged ledger, receipt, rollback, and publication closure; that
pending qualification is not a defect in the reviewed increment.

## Disposition

`APPROVE`. The V48 implementation and focused/retained correctness evidence
conform to `SC-SNOWENERGY-001` v48 and the active WGHL package. No correctness
blocker prevents canonical r123.
