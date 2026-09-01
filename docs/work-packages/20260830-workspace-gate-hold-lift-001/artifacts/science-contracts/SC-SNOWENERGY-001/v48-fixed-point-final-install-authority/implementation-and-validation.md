# V48 fixed-point final-install authority implementation and validation

Status: `IMPLEMENTED; INDEPENDENT REVIEWS APPROVE; CANONICAL R123 PENDING`

Evidence mode: `Static + Ran`

## Exact correction

R122 did not traverse any authenticated-continuation branch corrected by V47.
The real ordinary fixed-point completion called
`finalize_v11_imported_segment` without a precomputed physical ending, soil
candidate, or continuation. `owner_finalization.rs` reconstructed the lawful
prepared target 43 with predecessor/source 42 and then erased that posture by
calling the strict generic installer with no explicit authority.

V48 adds a distinct authenticated-prepared-beginning install path. It validates
the authoritative native-V2 resident and its complete prepared beginning,
including model, run, state, receipt chain, exact predecessor, target, and
support; validates the accepted result and orchestrator seals against the same
prepared beginning; derives the mutually equal outer source transaction;
constructs an explicit native-V2 V39 source/soil-target authority; and only then
uses unchanged V47 atomic posture validation and clone-then-replace install.

Both non-continuation native-V2 finalization fallbacks call one shared typed
helper. The generic/public `install_soil_thermal_accepted_v2` still passes no
split authority and remains exact same-ID-only. No transaction is inferred,
incremented, copied, or repaired. Refusal occurs before mutation and leaves
soil and all source owners unchanged.

## Validation

Ran focused V48 behavior and real-callsite vectors:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v48_/)'
```

Result: Nextest run `a9ee143a-b748-474e-bf17-e2fd523eed09`, `7 passed; 0
failed`.

The seven executed behaviors include the literal retained-r122 chain source
42 -> target 43 with predecessor 42 on support `1800..1980 s`, direct execution
of the production finalizer install helper, same-ID and exact accepted no-op,
no publication, and independently substituted prepared support/receipt plus
accepted target/predecessor/support/receipt/state/layer/seal and explicit
authority. Every refusal retains byte-exact soil and source-owner custody.

Ran focused V48 contract/source obligations:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract -E 'test(/v48_/)'
```

Result: Nextest run `7d003f17-053e-4074-abb8-6179479b5621`, `2 passed; 0
failed`.

Ran retained V39/V46/V47/V48 behavior regressions:

```text
nix develop -c cargo nextest run -p openwepp-hillslope-orchestrator \
  -E 'test(/v39_/) | test(/v46_/) | test(/v47_/) | test(/v48_/)'
```

Result: terminal exact-head Nextest run `846cf99b-d673-4a8e-a8c0-e89c62a1e59f`, `36 passed; 0
failed`.

Ran the complete source-contract target: Nextest run
`92ac046c-79db-4974-bd49-19e053f8debe`, `40 passed; 0 failed`.

Ran the retained persisted-restart crate: Nextest run
`cbc2d948-ad54-4773-b660-003d2224b09f`, `40 passed; 0 failed`.

The orchestrator all-target/all-feature check passes. The authority anti-evasion
script reports `PASS`; required-suite guard run
`c6cfd6b7-49e3-43fe-b62d-27e7ac7afb57` reports `3 passed; 0 failed`.
Workspace format and `git diff --check` pass, and an exact production scan finds
no V48/R122/R123 diagnostic seam. A warnings-denied whole-crate Clippy attempt
remains blocked by extensive pre-existing shared-head lint debt; the one new
V48 `result_large_err` finding is resolved by the repository's existing scoped
allow posture. Touched line counts remain below 3,000:
`v10_soil_thermal_v2.rs` 2,468, `v10_soil_thermal_v2_tests.rs` 2,956, and
`owner_finalization.rs` 2,933. Exact-move split plans are recorded in the
line-count artifact. Independent Rust correctness review and QA re-review both
record `APPROVE`; their artifacts retain the independent focused run IDs and
dispositions. The parent-owned canonical rerun remains pending.

## Remaining qualification

The implementation agent did not run the canonical fixture. Parent-owned r123
must verify the real fixed-point final install now retains the exact split
authority and proceeds through unchanged ledger, receipt, rollback, and
publication closure.
