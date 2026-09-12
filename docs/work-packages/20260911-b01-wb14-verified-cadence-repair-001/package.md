# B01 WB14 verified parent cadence repair

Status: HOLD — exact source recovery failed on 2 of 927 files. Owner adopted execution on 2026-09-11. Modeling/production/release remain HOLD.
Scaffold source: `17a1261fade4bb43d7b85c1fac6b42720fe4709a`.

## Objective and authorization

Deliver a recoverable, source-verified correction of WB14 cadence admission through
the entire authenticated inactive-prefix parent, with executed baseline/candidate
regressions, independent operand checks and required correctness/review evidence.
This is an isolated runtime correction; completion does not qualify the full B01
simulation, restart subsystem or production integration.

The owner requested this scaffold after adopting Astra orchestration. This request
authorizes package preparation, not reconstruction, builds, source correction or
simulation. The proposed execution envelope below is for owner adoption. An owner
instruction to execute this package adopts that envelope, including its explicit
replacement of the old checkpoint control limits; scaffolding alone changes none.

The [B01 integration record](../20260910-stage3-b01-snow-cycle-integration-001/package.md)
retains the previous `B01-WB14-PARENT-CADENCE-REPAIR` attempts and all wider modeling
obligations. This package becomes the single current cadence execution record on
adoption; the parent record remains the integration/backlog record, not a second
cadence ledger. Preserve finding `B01-CONT-005`: admission must work through the
entire prefix-backed parent, not just its first physical child.

## Adopted execution envelope

Astra orchestrates recovery, assignment, integration, validation, source custody
and disposition. Terra owns detailed implementation and focused verification;
Luna may perform narrow extraction. Use [the orchestration procedure](../role-orchestration.md)
and [bounded execution](../../standards/bounded-agent-execution.md). Reuse the
implementer and complementary independent reviewers through their fixes.

On execution adoption, package-wide orchestration replaces the earlier v2
30-active-minute/one-further-failed-cycle cap and automatic return on every newly
encountered failure. Two failed cycles or 60 active minutes trigger internal Astra
reassessment under the current standard. Continue only with a supported in-scope
next action. Stop for out-of-envelope changes, unavailable necessary primary
evidence after bounded recovery checks, unresolved science authority, or an
explicit new owner stop. No time/cycle allowance is renewed before adoption.

Carry history separately: at least one failed corrective edit/compilation; the
subsequent feature-correct inventory failed with disk exhaustion and executed zero
tests. Historical unrecorded active time remains UNOBSERVED; the prior diagnosis
used approximately 30 supplemental active minutes and zero correction cycles.
Do not reconstruct historical time from build durations or call a new package ID
a reset of those expenses. Start a new execution ledger only on adoption.

Permitted runtime changes are confined to the recovered candidate's
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/` files
`surface_liquid_ingress.rs` and `surface_liquid_ingress_preflight.rs`.
Contract-derived regression additions may use directly affected existing ingress,
context and native-prefix test modules or test-only sections in those two files.
Bind their exact write set after recovered-source inspection, before editing.
Local fixture/reconstruction/evidence scripts and this package record are allowed.
Adding an unrelated test harness or changing another production surface requires
new scope. Formatting the authorized files is allowed and tracked separately from
semantic changes. Preserve the original candidate patch before any correction.

Authentic-run allowance: **ZERO**. Focused real-entrypoint/fresh-process tests and
applicable correctness campaigns are allowed. No seven-day/warm/performance run,
saved cold-checkpoint resume, restart-subsystem repair, production integration,
working146 substitution or successor defect repair. No new science amendment,
solver/equation/tolerance change, feature/default/cfg or test-gating change,
dependency/toolchain/lockfile change, schema change, or primary-checkout Rust edit.
B01 thresholds remain 0.1/0.12 mm SWE and all prefix/accounting guards remain binding.
Scope does not authorize changing tests or numerical rules to make inherited debt
pass. Preserve a failing required check as a blocker when its correction is outside
this envelope.

Keep source and unique evidence under
`/workdir/openwepp-experiments/b01-wb14-cadence/`, with separate `working145`,
`baseline-red` and `candidate` directories. These are proposed paths, not created
trees. Inspect existing destinations before use; do not overwrite unique state.
Keep frozen working145 immutable. Use distinct disposable targets under
`/tmp/openwepp-b01-wb14-cadence-targets/`; never place the only source copy there.
Carry existing scoped commit/push permission for package evidence; no branch
change is authorized. Commit/publish the candidate and necessary recovery inputs,
verify remote retrieval including required LFS bytes, and record actual custody.

## Source recovery and configuration

Use the [retained recovery assessment](../20260910-stage3-b01-snow-cycle-integration-001/package.md#preservation-and-progress-assessment-2026-09-11)
and predecessor [reproduction instructions](../20260910-stage3-snow-accuracy-runtime-001/package.md#reproduction-and-retained-identity).
The exact construction order is:

1. Export Git base `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`.
2. Apply the predecessor's `artifacts/R0-composition.patch`, unpack
   `artifacts/R0-untracked-source.tar.gz`, and verify `artifacts/R0-source.json`.
3. Apply its `artifacts/PBC-final-build016.patch` and verify final016 identities.
4. Apply B01's [correction145.patch](../20260910-stage3-b01-snow-cycle-integration-001/artifacts/correction145.patch).
5. Extract `build145-prepare.py` and `build145-execution-identity.json` from B01's
   [raw-stop-point145.tar.gz](../20260910-stage3-b01-snow-cycle-integration-001/artifacts/raw-stop-point145.tar.gz).
   Verify all **927 actual file bytes** against that full manifest. The 717-file
   `correction145-source.json` is a subset, not complete source verification.
   Recompute SHA-256 of the sorted compact JSON path-to-file-hash map using the
   retained algorithm. It must equal
   `b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f`.
6. Apply the preserved [cadence patch](../20260910-stage3-b01-snow-cycle-integration-001/artifacts/wb14-parent-cadence-repair-candidate.patch)
   to the candidate copy. Patch SHA-256:
   `1b7f8a7b48f02d0aa117969081295325e7af0e11de0d914af6306c739df338bb`.
   Before edits, verify `surface_liquid_ingress.rs` against
   `45d6b2fb901adf7f37d14595ed088657886079027340adf71012cadb41e098de`,
   `surface_liquid_ingress_preflight.rs` against
   `249d4b1d4214381c762a9d7457f20a70a9997954a88e268717a63231e5cce141`,
   and workspace `Cargo.toml` against
   `e5271cfe2bca5132e263d6227264ebfd19c03e0275a9ff08cb0419c3f61bb676`.

Inspect the actual restored feature gates/definitions for the reported restart
methods, manifest and retained build145 command. Use existing Nix, both
`persisted-restart-v1` and `restart-authority-evidence`, and
`RUST_MIN_STACK=67108864`. Record actual cwd, manifest, target, tool versions,
features, fixture/source identities, command status, test counts and full-log path.
No `--all-features`, duplicate methods, default-feature repair or silent harness
substitution. A missing archived file or mismatch is a recovery finding; do not
fill it from HEAD and claim exact reconstruction.

## Authority and sequencing

Canonical [SC-SURFACELIQUID-001](../../specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md)
governs variables/units, persistent state, algorithm steps 5/6/8, ordered guard
precedence, parent/child receipt details and the native inactive-prefix amendment:
`INV-012/013/014/026/035`, `OBL-C-016/C-025`. Complete supplied parent validation
must precede alternate cadence admission. A zero cursor/ordinal, hash or absent
receipt alone is never authority. The published guard-map clarification from
`dafe138fe7022d59ba8ff7d685308eb434bba486` still needs independent review.

[SC-SNOWENERGY-001](../../specifications/science-contracts/contracts/SC-SNOWENERGY-001.md)
`INV-083/087`, `OBL-C-055` govern inactive physical custody and the exact terminal
transition. [SC-COUPLEDTIME-001](../../specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md)
algorithm steps 4/5/9, `INV-005/006/024/031` and `OBL-014` govern accepted-slab
identity, complete owner rollback, prefix authentication and finalization.
Follow affected scientific dependencies when recovered code reveals them; no
contract is waived by this reading selection. Pin governing authority separately
from runtime source; do not overwrite frozen runtime files to run authority checks
or claim a primary-HEAD check verifies the candidate.

Preserve the actual chronology: the retained candidate was implemented before the
required old-red execution. In `baseline-red`, add the exact regression fixture to
unchanged working145 production source and run a **retrospective baseline red**.
Run the same fixture/features on the candidate. This demonstrates regression
sensitivity, not retroactive compliance with pre-implementation sequencing.
Reviewers must disposition that unmet sequence explicitly. For further production
edits, confirm authority, add tests and record applicable pre-edit gate evidence.

## Acceptance and independent operands

All rows are required and currently NOT RUN on a reconstructed candidate.

| Proof | Required observation |
| --- | --- |
| Exact failing boundary | Real ingress at day 4 / interval 22 / transaction 255; parent `[385200,387000)` s, accepted inactive prefix `[385200,385920)` s, first physical child `[385920,385980)` s. Baseline rejects at the known cadence boundary; candidate accepts through actual ingress and WB14. |
| Prefix and first child | Prefix consumes zero physical WB14 calls, transitions, child receipts, routed parcels or cumulative changes; first physical ordinal is zero. The persistent cursor is not used as a universal zero classifier. |
| Subsequent and final children | Feed actual returned candidate and parent to contiguous children through exactly `387000 s`; for the fixed 60-second fixture this is 18 physical children per OFE, with zero WB14 calls for the 720-second prefix. Retain physical cumulatives and receipt history while nonfinal persistent cursor stays unchanged. Finalize exactly once to day 4 / next interval 23. No prefix reminting or reset to persistent-cursor admission after the first child. |
| Authentication and precedence | Missing/foreign/resealed-inconsistent prefix; wrong parent/owner/configuration/model/parameters/topology/lane; support/accepted-until mismatch; omitted scalar authority; duplicate/gapped/overlapping/replayed children or finalization all reject with canonical typed precedence before ingress/WB14 mutation. Include compound poisons that distinguish reordered errors. |
| Controls and rollback | Ordinary non-prefix parent and day-rollover controls. Before/after each refusal, compare the complete live/persistent and staged owner bytes for snow, litter V3/V4 and LSE, surface, hydrology, soil/soil-thermal, vegetation and coupled clock; include the parent candidate, provider/GSI/event cursors, complete accepted receipt/chronology/history state, publication buffers/receipts and physical-call counters. Every affected accepted/candidate custody surface must remain exact. Diagnostic `state_mutated=false` is not evidence. |
| Split/replay and real path | Exercise the required ignored fresh-process prefix/cadence regression explicitly through its real harness, plus uninterrupted/split state equivalence. Bind exact discovered test names before execution; skipped or zero-selected tests do not pass. Use real consumer/ingress receipt custody, not a helper-only substitute or authentic saved-checkpoint resume. |
| Scientific evidence | Reconstruct affected cumulative and transfer changes from independently obtained beginning/ending owners, immutable inputs and actual transfer receipts. Require nonzero distinguishing inputs and ordinary controls; do not use the producer's residual as the oracle. |

Freeze fixture inputs and affected operand lineage before further production edits.
Keep the exact failure fixture intact; add distinguishing nonzero/heterogeneous
controls separately rather than replacing the recorded failure inputs.
The following contract-level lineage must be bound to exact restored symbols and
fixture fields during preparation; it is not permission to invent new equations.

| Operand | Units / basis | Authority and independent check |
| --- | --- | --- |
| `cumulative_supply_m`, `cumulative_infiltration_m` | m water / OFE ground | Actual scalar authorities and physical receipt progression; independently sum support-sliced admitted supply and actual infiltration, using canonical `rho_w` once. Prefix deltas zero; subsequent child differences equal accumulated physical increments. |
| `W_0,k`, `W_1,k`, liquid/ice records | kg H2O / m² tile ground | Actual beginning/ending complete owners; separately reconstruct accepted withdrawals, condensation and retained credits, retaining exact inactive ice/custody. |
| `m_p`, `I_p`, retained/routed/outlet mass | kg H2O / m² named basis-OFE ground | Immutable admitted parcels and consumed transfer receipts; verify source partition and recipient soil-layer changes without counting a diagnostic aggregate twice. |
| `Q_p`, soil/litter/route energy credits | J / m² named ground basis | Accepted parcel enthalpy and authoritative recipient high/carry when present; independently reconstruct affected transfer closure using unchanged canonical equations/tolerances. No whole-system energy claim. |
| `f_t`, `A_o` | dimensionless tile fraction; m² OFE area | Immutable configuration/topology; convert tile/OFE basis once, reconstruct absolute mass where routing joins OFEs, and use unequal areas/nontrivial fractions for affected joins. |
| Calls, ordinals, support, owner bytes | counts; exact ns/s; bytes | Test instrumentation and complete serialized state, diagnostic only; demonstrate zero/one calls and exact chronology without using these counters as physical authority. |

No parameters are fitted: calibration and identifiability are NOT_APPLICABLE.
Fixtures are DIAGNOSTIC_ONLY; this verifies implementation/custody, not empirical
validation or whole-cycle scientific qualification. Keep existing physics and
tolerances; any new science requirement beyond this envelope is an escalation.

## Validation plan

Classify execution as critical chronology/state/guard work under
[testing strategy](../../standards/testing-and-gate-strategy.md), with
[kernel preparation](../../standards/kernel-work-package-preparation.md).
Use focused checks in edit loops, then immediate full correctness at the completed
semantic increment, before its closure. Full correctness is not deferred to a
future authentic cycle. No inherited-lint waiver or quick-profile substitution.

For either source copy, use its explicit `SRC`/`TARGET`, from cwd `$SRC`:

```sh
set -euo pipefail
SRC=/workdir/openwepp-experiments/b01-wb14-cadence/candidate
TARGET=/tmp/openwepp-b01-wb14-cadence-targets/candidate
cd "$SRC"
nix develop /workdir/openWEPP --command env \
  CARGO_TARGET_DIR="$TARGET" RUST_MIN_STACK=67108864 \
  cargo nextest list --manifest-path "$SRC/Cargo.toml" \
  -p openwepp-hillslope-orchestrator --lib --locked \
  --features persisted-restart-v1,restart-authority-evidence \
  -E 'test(surface_liquid_ingress)'
```

Then run that selection with `cargo nextest run`. Listing builds but does not run
tests. Inventory the acceptance fixtures explicitly: the initial filter is only a
configuration check. Expand selection by named required tests, including native
prefix, cadence precedence, ordinary/rollover controls and the actual ignored
fresh-process harness. Do not replace that harness if unavailable. Record exact
commands and nonzero selected/passed counts; preserve every failure.

Run affected-file rustfmt checks and warnings-denied owning-crate Clippy with
explicit manifest and both features, using `--all-targets --locked -- -D warnings`.
Select affected reverse dependents from the terminal diff; include their feature
posture. Run applicable docs/schema/serialization and source anti-tautology guards.

Run workspace full correctness against the final candidate:

```sh
nix develop /workdir/openWEPP --command env \
  CARGO_TARGET_DIR="$TARGET" RUST_MIN_STACK=67108864 \
  cargo nextest run --manifest-path "$SRC/Cargo.toml" --workspace \
  --profile full --locked \
  --features openwepp-runner/test-fixture-authority,openwepp-hillslope-orchestrator/persisted-restart-v1,openwepp-hillslope-orchestrator/restart-authority-evidence
```

A0 admission, affected A1 hard invariants and applicable A3 constitutive authority
remain mandatory. At recovered-source inventory, bind their actual registered
targets, execution/authority roots and obligation coverage here before edits;
the scaffold does not invent suite names or treat unspecified targets as passed.
Current discovery points include
`tests/integration/surface_liquid_hydrology_custody_authority_contract.rs` and
`tests/integration/coupled_time_authority_contract.rs`; their existence alone is
not A0/A1/A3 coverage. Preserve conservative escalation for uncertain impact.
External-authority suite posture, cohort fixtures and required-case bindings are
outside the intended edits; if touched, retain the root anti-evasion requirements
and first resolve whether the change is within scope.

## Review, evidence and disposition

Two independent execution reviews are required: correctness/authority owns full
parent chronology, prefix authentication, error precedence and the published SC
clarification; QA/evidence owns source/build/fixture identity, actual test coverage,
complete-byte/call-count proof and independent operand evidence. Both review the
relevant scientific acceptance. Neither author/implementer nor a material design
adviser may count. Same reviewers verify fixes; accepted unaffected evidence may
be reused. Scaffold review is not execution review or scientific acceptance.

Before handoff, retain a minimal working145-relative candidate patch, immutable
fixture/input identity, full recovery verification, raw commands/logs/counts,
independent calculations and attributable reviews here or in linked artifacts.
Reconcile the terminal diff; verify committed/remote recovery inputs, not just the
record. A completed repair requires every acceptance and applicable check to pass
and both reviews to accept their scope. Otherwise record HOLD with the first
failure, remaining obligations and exact next decision. Build success, preserved
evidence and scaffold completion cannot be labeled verified cadence repair.

Scaffold-time state: source recipe and manifest algorithm located in the predecessor;
the candidate patch/log are remotely preserved. Reconstruction, new tests,
baseline/candidate execution, full-parent proof, lint/full correctness/A0/A1/A3,
independent operands and execution reviews are all NOT RUN for this package.
Scaffold-time next action after owner adoption: assign Terra the recovery and exact inventory
deliverable, with no semantic source edit until source verification and baseline
fixture preparation are complete. Wider B01 work stays paused.

### Scaffold checks and review

Ran from `/workdir/openWEPP`: `git diff --check` PASS; an inline
`.venv/bin/python` reference/hash check resolved 63 local links across this record,
README and active locator and matched the retained correction145/candidate patch
hashes. Both shell blocks passed `bash -n`; neither was executed. Only this record
and the two locator/catalog rows changed. No runtime validation result is claimed.

Independent read-only scaffold reviews used `/root/governance_correctness`
(`rust_code_reviewer`, Sol/high) and `/root/governance_qa`
(`rust_qa_reviewer`, Terra/medium), distinct from the author and each other.
Correctness finding `WB14-SCAF-COR-001` (high) identified insufficiently explicit
rollback coverage. The acceptance row now enumerates the full live/staged owner
set, accepted history, publication custody and counters; both reviewers verified
that fix in their original sessions. Final attributable verdicts:

> Correctness: Static: `WB14-SCAF-COR-001` is resolved. The expanded rollback row now covers the complete accepted and candidate owner sets, clocks/cursors, receipts, chronology/history, staged state, publication buffers, and physical-call counters byte-for-byte.
>
> Final bounded scaffold verdict: PASS; no blocking findings remain. This approves the scaffold only, not execution or scientific acceptance.

> QA: Static: WB14-SCAF-COR-001 correction verified at `package.md:157`.
>
> The rollback row now requires complete live/persistent and staged custody comparisons across the named owner domains, plus parent candidate, provider/GSI/event cursors, accepted receipt chronology/history, publication buffers/receipts, and physical-call counters. It expressly rejects `state_mutated=false` as sufficient evidence. No affected QA finding.

Scaffold preparation is complete. The owner's instruction to execute this package
adopts the execution envelope; scaffold review remains historical evidence only.

## Execution ledger

Execution adoption recorded 2026-09-12 03:55 UTC (2026-09-11 owner timezone).
Primary source/authority root is `/workdir/openWEPP` at
`d3bbc104b2b69ac16e96acb7722b0a9795a89d94`, initially clean. Governing contracts
are pinned to that primary source, separately from reconstructed runtime source.
Intent is critical chronology/state/guard implementation with unchanged science,
features and tolerances. The package validation plan and all acceptance rows above
remain binding. Authentic runs consumed: 0 of 0; new corrective cycles: 0.
Historical consumption above is retained separately.

Astra assigned `/root/cadence_implementation` (`implementer`, configured
Terra/medium) the recovery/inventory checkpoint: reconstruct and hash-verify all
927 working145 files and the original candidate, inspect exact feature gates and
required harnesses/authority targets, and propose bound test paths and operand
symbols. No further semantic edits are authorized at that checkpoint. The agent
may write recovery/evidence scripts under the durable experiment root, but not
primary-checkout Rust or this record. Astra owns this record, integration,
validation decisions and subsequent independent reviews.

Checkpoints are exact recovery/inventory; contract-derived fixture preparation
and retrospective baseline red; in-envelope correction/focused verification;
full required validation and complementary independent review; recoverable
publication and disposition. The two-cycle/60-active-minute reassessment and
explicit stop conditions above apply throughout. All runtime acceptance remains
NOT RUN pending source-bound execution.

### Recovery stop and retained evidence

Ran: `/root/cadence_implementation` executed the retained recipe in
`/workdir/openwepp-experiments/b01-wb14-cadence/recovery-inputs`.
R0 verified 701/701 files; final016 verified 41/41 changed-file identities.
The correction145 result matched only 925/927 required files. Astra separately
read and hashed all 927 actual files with repo `.venv/bin/python`, confirming
the same two mismatches and actual map digest:
`c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e`.
The expected map remains
`b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f`.

The mismatching paths under
`tests/integration/land_surface_energy_real_hydrology_shadow_contract/` are
`precedence_tests.rs` and `raw_hash_tests.rs`. Full expected/actual hashes and
bounded search inventory are in [recovery-mismatch.json](artifacts/recovery-mismatch.json).
The implementer checked retained patches, archive membership, work-package
tarballs, reachable path history, 64 unreachable Git trees and old identity
manifests without locating the exact bytes. This is an unavailable-primary-source
stop under the adopted envelope, not a scientific impossibility claim.

Evidence retained:

- [full build145 identity](artifacts/build145-execution-identity.json) and
  [retained manifest algorithm](artifacts/build145-prepare.py), extracted from
  the existing raw-stop-point145 archive;
- [first recovery script](artifacts/reconstruct.sh) and
  [first failed log](artifacts/reconstruct-first-attempt.log): exit 1 after
  R0 verification because the script assumed the wrong final016 manifest key;
- [continuation script](artifacts/continue-reconstruct.sh) and
  [continued log](artifacts/reconstruct-continue.log): exit 1 at the two actual
  working145 byte mismatches after passing final016;
- [Astra actual-byte verification](artifacts/parent-actual-byte-verification.json):
  exit 0, 927 read, 925 matching; this is executor corroboration, not independent
  acceptance review.

These scripts are preserved failed-attempt evidence, not a corrected reusable
recovery tool. The continuation log labels the expected manifest-map digest as
`working145_map_sha256`; it does not represent the actual-byte map. The separate
actual-byte verification above resolves that distinction. Recovery scripts used
system `python3`, contrary to repo-local Python guidance; Astra's corroboration
used `.venv/bin/python`. No Rust compile or test ran; test count is zero.

The 3.2 GiB partial source remains durable at the path above. `working145`,
`baseline-red` and `candidate` were never created. The preserved cadence patch
was not applied, and no further runtime correction or fixture was authored.
No source cleanup occurred. Authentic runs remain 0/0. One failed recovery-script
correction/verification cycle occurred; no Rust corrective cycle occurred.
Execution ended at the recovery stop, before the 60-minute reassessment boundary;
precise active versus wait accounting is UNOBSERVED.

Static inventory from the partial tree suggested no matching required ignored
direct-ingress fresh-process harness in the authorized test paths. This is not a
verified candidate inventory or proof of global absence. Exact-source recovery
must precede binding the actual harness and A0/A1/A3 commands. No harness
substitution is authorized. All baseline/candidate, full-parent, negative,
rollback, split/replay, scientific operand, formatting, Clippy, full correctness
and authority acceptance remains NOT RUN.

### Execution review and disposition

Static: independent `/root/cadence_correctness` (`rust_code_reviewer`, Sol/high)
reviewed the clarification at `dafe138fe7022d59ba8ff7d685308eb434bba486`
against its parent `c98e731020e9611c7708f99b89e1c1fd036c7568` and applicable
coupled-time/snow authority. Contract SHA-256:
`0f7bfb46764f1db883988e0b34771be2e455435b213095fe61756d332e94931e`.
Final findings, attributable to that reviewer:

> HIGH — historical sequence remains RED. A retrospective baseline failure can
> establish regression sensitivity only. It cannot satisfy the preimplementation
> sequence or support a claim that OBL-SURFACELIQUID-C-025 fully passed.
>
> HIGH — exact recovery and runtime acceptance remain unmet. HOLD. Primary
> recovery logs were not independently verified by this reviewer.
>
> MEDIUM — clarification provenance is absent from the contract-local change log.
> The 2026-09-11 clarification is not recorded at SC-SURFACELIQUID-001 line 2205.
> This is an out-of-envelope owner follow-up.
>
> No semantic contradiction was found in the two changed guard-map rows.
> Complete-parent validation is a prerequisite for alternate cadence admission;
> it does not override compound-error precedence. Final bounded verdict: HOLD.

The reviewer withdrew an initial suspected precedence contradiction after
checking the distinction between successful validation and simultaneous-error
ordering. No source fix was made. The original final review is retained in
[correctness-review.txt](artifacts/correctness-review.txt).
A requested distinct QA/evidence reviewer could not be spawned: the tool returned
`agent thread limit reached`. No independent QA verdict is claimed. Both complete
execution reviews and runtime acceptance remain unmet; this static review does
not approve the repair. Contract change-log maintenance belongs to the owner’s
authority-maintenance scope, outside this package’s permitted runtime correction.

Disposition: HOLD, repair incomplete. First blocker is exact working145 recovery.
Next required input is the two exact file byte sequences matching the expected
hashes in recovery-mismatch.json (or owner-adopted revised source scope, which
cannot be labeled exact working145). Wider B01 work stays paused. After exact
recovery, all remaining acceptance/checks/reviews still apply.

Terminal diff is confined to this record, locator/catalog and recovery/review
artifacts. Ran from `/workdir/openWEPP`: `git diff --check`, both retained shell
scripts through `bash -n`, local-reference resolution and artifact JSON parsing
all PASS. These documentation/evidence checks do not validate runtime behavior.
Publication custody: evidence commit
`261daa8ec31a74a1f8f8d44c1fd19f0178cf0ca8` was pushed to existing `main`.
Astra fetched all ten package/artifact files directly from GitHub at that commit
and compared every byte to the local files: PASS. The verification result is
retained at the durable experiment root as `remote-evidence-verification.json`.
These new artifacts use ordinary Git bytes, not LFS pointers. This verifies the
new stop evidence only; prior large recovery archives were not remotely
re-downloaded in this execution. Exact working145/candidate custody remains
unavailable regardless of evidence publication. No complete recovered-source
claim is made.
