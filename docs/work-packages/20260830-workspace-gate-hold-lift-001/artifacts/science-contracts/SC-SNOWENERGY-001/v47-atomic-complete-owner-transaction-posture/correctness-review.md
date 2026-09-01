# V47 independent correctness review

Disposition: `APPROVE`

Evidence mode: `Ran + Static`

## Findings

No blocking correctness, science-contract, typed-error, or duplication finding.

Static review confirmed that the ordinary/public
`install_soil_thermal_accepted_v2` and
`install_soil_thermal_accepted_v2_from_beginning` paths pass no split
authority and therefore require exact `source == soil target`. The only split
admission is the unpublished-continuation install. It requires an explicit
native-V2 `PhysicalSoilEnergyTransactionAuthorityV2`, reconstructs and
byte-compares the expected source/target authority from the exact continuation
prepared beginning, and then requires the accepted owner's exact predecessor
to equal the mutually equal vegetation/LSE/BGC source transaction. No V47
admission path performs transaction arithmetic or infers adjacency.

The accepted resident is validated before installation, including its owner,
state/layer target seals, predecessor custody, accepted operand receipt, and
orchestrator seals. Continuation validation retains exact prepared support,
physical trial, accumulated operands, receipt support, predecessor receipt
chain, and physical-ending joins. Complete posture validation precedes the
clone; only a fully validated clone replaces the resident. Refusal therefore
leaves the original owner set unchanged, and the exact accepted no-op still
runs the same posture validation. All three production continuation call sites
construct and pass the explicit authority; the generic call sites remain
unchanged and strict.

Independent focused execution passed retained V39/V46/V47 behavior `29/29`
(Nextest `b2da9b37-5e02-4532-b064-f6e77b97b993`) and V47 contract/source
obligations `2/2` (Nextest `1afb2185-2cf0-43a6-8618-00197f4ce011`). The
reviewer also inspected the same-ID, explicit-successor, foreign/swapped/
missing-predecessor, three-source-disagreement, generic-split rollback, and
composed second-child success vectors.

## Residual risk and missing evidence

Canonical R122 is still the required real-consumer proof that the retained
r121 `source 42 / target 43 / predecessor 42` support now completes with exact
ledger closure; this review does not substitute for that run. Target-state,
support, receipt, seal, and exact-no-op poisons are enforced by retained V39
accepted-owner/continuation validation rather than duplicated in every V47-
named test.

The touched `v10_soil_thermal_v2.rs` and its included test file are 2,382 and
2,581 lines respectively (`WARN`, both below the 3,000-line blocking limit).
The existing exact-move sibling-module split intent remains necessary before
either reaches 3,000 lines; V47 centralizes the new admission logic rather than
adding a parallel validator.

## Approval

`APPROVE`: V47 is consistent with `INV-SNOWENERGY-071` and
`OBL-SNOWENERGY-C-039`, preserves V39/V46 semantics, and has no correctness
blocker to canonical R122 execution.
