# Rust Code Review

Status: `complete`

Evidence class: `Static` for exact commit
`7820953c1b5258564200bd167e0c4994a69b3065`; `Ran` checks are identified
separately and are not claimed as exact-commit evidence because other agents
modified the shared checkout during review.

Review range: `a65cc3973ddd04b07cad108fcb33d83a8c161abb..7820953c1b5258564200bd167e0c4994a69b3065`.

## Findings

### Critical -- the real pure-melt span can erase WB14 runoff by reclassifying it as infiltration

The runner now correctly sends routed melt to WB14 on its producer clock, and
WB14 computes infiltration, depression storage, and hourly excess from that
supply. The R4K span then overrides that result on a pure-melt day. When snow
coupling is active, routed melt is positive, and direct rain is absent,
`resolve_r4m_same_pass_infiltration_m` reconstructs infiltration as
`liquid_input_m - depression_storage_delta_m`, without the WB14 infiltration
capacity or interval solve
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:521-572`).
The caller promotes that daily reconstruction to cumulative infiltration and
removes the difference from the earliest WB14 excess bins
(`runoff.rs:234-249`). With zero WB14 storage capacity and positive routed
melt, WB14 produces zero infiltration and positive melt excess, but this second
operator promotes the entire post-depression melt depth to infiltration and
clears the hourly runoff series. Positive pure-melt runoff and its peak therefore
disappear even though WB14's source-complete ledger says they exist.

This violates `SC-WATBAL-001#INV-WATBAL-102/103`: melt-supply-only positive
runoff must retain its modeled hour, and daily scalar reconstruction may not
replace WB14's closing depths. It also contradicts the package's exactly-once
WB14 partition rule. The apparent melt-only peak test exercises the isolated
WB14 and peak helpers and never runs the R4K span
(`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:51-110`).
The real-span snowmelt test instead codifies promotion to the full liquid input
without a zero-capacity/excess assertion
(`direct_runtime_r7g_frost.rs:629-665`).

Remove the post-WB14 daily infiltration override, or prove an authoritative
hourly/source-tagged debit that respects WB14 capacity and interval custody.
Add a real R4K -> R4A -> R7D6 pure-melt vector with zero/limited storage that
proves positive excess, daily/hourly closure, and the producer peak hour.

### High -- Critical exact-head closure evidence is absent from the reviewed commit

The package requires exact-head full-workspace correctness and the complete
1,088-trial Topanga cohort, but the committed package state says the terminal
gates are pending (`package.md:219-222`,
`artifacts/implementation-test-evidence.md:1-27`,
`artifacts/gate-results.md:1-5`). The committed probe summary identifies anchor
`949349e7055c5d19277eeb708401c4614a52cd77`, and the committed full-cohort log's
external summary records binary SHA-256
`569f586516283c633cf4a2c99eb4c89725f8c57c476047b7b03a0b59e327ca88`, the same
949 anchor binary, not the reviewed 782 commit. Thus the cohort does not cover
the final source-custody, threshold, guard, publication-boundary, or census
resume changes.

This missing evidence is independently blocking for a Critical package, even
after the pure-melt defect is corrected. Rebuild from the terminal commit,
bind the binary hash to that commit, rerun the required focused/quick/full,
doctest, and complete cohort gates, and commit the reconciled summaries and
terminal disposition.

### Medium -- WB16 typed errors publish fabricated diagnostic values

The whole R7D6 span now reaches the shared WB16 code family, including missing
R4A/R4O, which resolves the earlier generic-code gap. However,
`map_wb16_peak_guard` manufactures `NaN`, `-1.0`, or `1.0` because the generic
intermediate variants discarded the observed operand
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1582-1599`).
The typed guard formatter then presents those sentinels as the observed flux
and reports closure/domain errors as `value=1 outside [None, None]`
(`hydrology/02_guard_errors.rs:301-350,446-479`). The tests assert only message
IDs, so this diagnostic corruption is not detected
(`direct_runtime_dc01.rs:188-228`).

Construct WB16 guard variants at validation sites with the actual operand and
meaningful bounds/residual context, or extend the intermediate error variants
to retain those values. Assert code, boundary class, symbol, value, and bounds
for missing, non-finite, negative, and closure routes through the actual R7D6
span.

### Medium -- the manifest claims a retired WB16 `ealpha` runtime producer still exists

The production APPMTH/`ealpha` peak branch and `DirectPeakRunoffInputs` are
correctly removed, and the contract says `ealpha` may remain only in an
explicitly historical diagnostic schema and cannot carry a runtime or
publication claim (`SC-WATBAL-001:1272-1275`). The execution manifest still
serializes `wb16_ealpha_seed_policy = "runtime_provided"`
(`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:545-568,632-635`),
and a contract-derived integration test explicitly requires that false runtime
claim (`tests/integration/cli03_runner_contract_derived_tests.rs:480-489`). No
WB16 `ealpha` producer or consumer remains.

Retire/version these provenance fields or serialize an explicit
`retired_not_applicable` historical status. Do not claim `runtime_provided`
after deleting the runtime producer.

## Residual Risk And Missing Tests

- Static: the prior mixed local/runon debit is now guarded, but the Critical
  local-only override above remains and masks the very pure-melt peak vector
  required by the contract.
- Static: every finite positive additional-supply bin is validated and admitted;
  positive runon without WB14, positive daily runoff with an empty ledger, and
  partial daily frost retention now fail closed. The earlier widened
  threshold/tolerance finding is resolved.
- Static: the public zero/nonzero basis guard is symmetric, area is applied once,
  and the production/test peak arithmetic shares
  `hourly_peak_runoff_from_closing_depths_m_s`. The prior public-boundary and
  duplicated-operator findings are resolved.
- Static: census receipts now bind the full expected calendar and row count,
  hash discovered hillslope sidecars, use atomic NPZ replacement, and reject an
  empty paired-event result. The prior empty/truncated resume finding is
  resolved. Final Parquet and summary writes are still non-atomic, so an
  interrupted terminal publication should be treated as incomplete evidence.
- Static: real p61 and p102 tests reconstruct HBP peak from `max(V_h)/3600` and
  join by year plus Julian day, but they are rain-event consumers and do not
  cover the pure-melt runtime override.
- Ran: `git diff --check a65cc3973..7820953c1b5258564200bd167e0c4994a69b3065`
  passed.
- Ran on the concurrently changing shared checkout, not admitted as immutable
  782 evidence: 29 focused orchestrator tests passed; six peak-contract/p61/p102
  tests passed; six census harness tests passed. Exact-commit heavy gates were
  not run by this reviewer.

## Verdict

`HOLD` -- commit `7820953c1b5258564200bd167e0c4994a69b3065` can erase
source-complete pure-melt runoff before WB16, retains two typed/provenance
integrity defects, and lacks Critical exact-head terminal evidence. No approval.
