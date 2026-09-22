# COLD-CANOPY-M1 final mechanical owner delta — correctness review 03

**Verdict: PASS for the mechanical vegetation-owner delta only; overall runtime remains HOLD.** The three lint-directed changes preserve the reviewed body02 physics, validation precedence, arithmetic order, transaction propagation and ledger construction. All substantive findings in `provider-parent-parent-body-correctness-02.md` remain open.

**Evidence class: Static.** Same-reviewer byte diff between live `cold_canopy_m1_owner.rs` SHA-256 `111ce8b548ff3174612e3ed61f5ba8b6b2e76ed86f0ac7443a578c3fde5f6937` and the preserved body02 reconstruction owner SHA-256 `b4234df1a40ce4b7961f6b1eb9e2bcedd921c17f98749152e4533759b0e87bb6`. No build, lint, test, or physical command was run by this reviewer.

## Findings

No new correctness blocker was found in the assigned mechanical delta.

- Removing `#[must_use]` from `M1OwnerReservoir::try_new` changes no executable behavior; the function already returns `Result`, whose unused value is independently `must_use`.
- `M1CandidateLineage` is a private `Clone + Copy` grouping of the same `TransactionId`, ending revision and single-support-validation boolean previously passed as separate arguments. Both callers populate the fields with their prior expressions. The shared core, ending-stratum construction, proposal binding, ledger transaction, ending revision and validation branch consume those same values in the same order.
- `validated_m1_carbon_operands` extracts the existing accepted-row validation and carbon/T10 map. The configuration/model/cardinality guard still runs first. Row finite/domain/net-bit/duplicate validation, exact configured-strata set equality, ordered `BTreeMap` iteration, `integrate_class_carbon`, beginning T10 lookup and `update_t10` all retain their prior expressions and error mappings. The persistent core still runs only after that map succeeds. No physical constant, unit conversion, clamp, numerical guard or nitrogen-arbitration order changed.

## Residual risk and missing validation

The body02 HOLD remains unchanged: staged BGC carries the high-bit support ID into persistent `last_transaction_id` and is incompatible with the canonical parent finalizer; successive solves still source mutable soil beginning temperatures from the original fixture; native receipt/forcing remains bootstrap-bound; and complete-parent/raw-outcome/finalization/replay APIs are absent. The last observed build was expected-red on those missing APIs, so there is no executable or physical release.

The final `Copy`-grouping/extraction cut was authored after the last reported formatter/Clippy check. It has **no final compile, formatter, Clippy, test, or physics rerun**. This static equivalence review does not substitute for those checks.

## Reviewed identities

- canonical source03: 760 entries, aggregate `e53295e289aa3b83cc4f3e069e81ca7ccda0dfcbef0d43fa3181194d2a77495f`
- source receipt SHA-256: `9368d1c32815f4437b0ae89a2e19c2846180a0311a9ede3a45836a6e1f8f4d86`
- observer-relative patch SHA-256: `c58b9183b84c9db40a9d60534107037ae7a362eaef11f1ba489fbeedbb50e153`
- fresh reconstruction receipt SHA-256: `8424d5fc7592a809968e725b2ed2e559a43af2708e5055b910a14933ce03e5e4`
- unchanged recorder SHA-256: `6781d00a9e42d45418993c522b5a7796ce55e6261834964b7452149140faf042`

Root reports the fresh reconstruction matches all 760 live entries and that the vegetation owner is the only body02-to-source03 source delta.

No body, complete-parent, or physical execution release is granted.
