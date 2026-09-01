# V50 envelope-source transition authority implementation and validation

Status: `IMPLEMENTED; DUAL REVIEW APPROVED; CANONICAL R130 PENDING`

Evidence mode: `Static + Ran`

## Exact correction

R129 proved that the authenticated constitutive beginning is lawfully
heterogeneous: vegetation 41, LSE 40, BGC 41, and soil 41. It supplies exact
soil resident/prepared custody, not a complete ending-source transaction. The
V49 strengthening incorrectly applied the complete-source join to that
beginning and rejected the valid source-42 ending.

V50 adds opaque `DirectSoilThermalOuterOwnerTransitionAuthorityV2`. The real
finalizer reconstructs the complete non-soil ending from the authenticated
beginning plus the already validated `UncommittedCoveredV8OwnerEnvelope`, then
requires the candidate and reconstruction to be exactly equal outside the
independently owned soil resident and publication history. Both complete
endings must carry the envelope's exact transaction. The authority is retained
inside the prepared-install authority and independently reconstructed at
install time.

The V50 V4 prepared/install path validates candidate vegetation/LSE/BGC mutual
source identity, exact reconstructed non-soil state, exact authenticated soil
resident, prepared target/predecessor/support/state/receipt custody, accepted
ending/seals, and the existing atomic clone-and-swap posture. The heterogeneous
beginning is never normalized or treated as ending-source provenance. V3
retains the V49 beginning-equality posture for its retained tests; generic and
physical split installation guards are unchanged.

## Ran evidence

- Expected source red: `6686d134-b06e-41ca-92a4-032ce2b3120e`, 1/2 passed.
- Focused V50 behavior, including the exact r129 V4 install/no-op, executed
  native-V2 real finalizer, and direct envelope transaction/material-receipt
  poisons: `ddb75a3a-ea1a-43e3-8ccf-82cf23017a22`, 5/5 passed.
- Focused V50 source: `db174904-2ff2-4625-ba05-9404eebead97`, 2/2 passed.
- Retained V48 source plus V49/V50 runtime:
  `7b3647c7-0c36-4d08-9d5c-7bc75fbb96b3`, 8/8 passed.
- Retained V39/V46/V47/V48/V49/V50 runtime:
  `b66339ad-d4ed-43f9-93a5-1a2ef4f25d52`, 46/46 passed.
- Complete snow source-contract target:
  `e542ffd1-f70e-465a-b9ad-95d6116ca20e`, 44/44 passed.
- Persisted restart default: `57e81712-ceb4-49e7-a997-7d9e61cce42b`,
  40/40 passed.
- Persisted restart all features:
  `41aa86d4-d8ce-43eb-ad59-eccaf698e501`, 71/71 passed in 460.063 s.
- Orchestrator all-target/all-feature check: passed.
- Workspace formatting and scoped `git diff --check`: passed.
- Production diagnostic scan: no V50/R129 probe, `eprintln!`, or persistent
  microstepping diagnostic seam.
- Retained r129 log SHA-256:
  `60d4da094a0c96075b2b932250964474a30bf25c9c9be4680acefa96cee6142d`.

Warnings-denied Clippy was attempted twice. The dependency-inclusive run is
blocked by three unrelated concurrent land-surface-energy `too_many_lines`
findings. The orchestrator `--no-deps` run is blocked by broad pre-existing
crate lint debt outside the V50 diff. No V50-owned Clippy diagnostic was
reported; these attempts are not recorded as passes.

## Behavior evidence

The exact mixed-beginning unit vector carries vegetation 41/LSE 40/BGC 41,
soil resident 41, envelope/candidate source 42, and retains publication
history through the V50 V4 install. A separate native-V2 real-parent vector
executes `execute_real_parent`, the real V11 finalizer, and the opaque
validated-envelope transition mint; it proves reconstructed vegetation/LSE/BGC
owners equal the envelope source while the authenticated beginning remains
heterogeneous. The retained V49 vector separately proves
source42/resident43/target44.
Foreign envelope source, individual candidate-source divergence, concerted
candidate/envelope rebase, same-transaction foreign reconstructed ending, and
opaque transition substitution refuse. The runtime envelope validation seams
also execute exact envelope/vegetation/physical/BGC transaction substitutions
and material receipt transaction/owner/proposal/value substitutions. Retained V49 vectors cover independent
resident transaction/support/receipt/state/layer/latest-custody/seal,
prepared/accepted/seal/authority substitutions, exact no-op, rollback, and no
publication.

## Governance

Terminal line counts are `owner_finalization.rs` 2,951 (`WARN`), its V50
exact-move include 118 (`PASS`), `v10_soil_thermal_v2.rs` 2,882 (`WARN`), the
retained V10 test file 2,962 (`WARN`), the V49/V50 test split 813 (`PASS`), and
`covered_v8_owner.rs` 1,271 (`PASS`).
All files remain below the 3,000-line limit. Existing exact-move split plans
remain binding; no exception is requested. Independent Rust correctness and
QA re-reviews both approved the final V50 head. The parent-owned canonical
R130 rerun remains pending.
