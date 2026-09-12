# B01 WB14 bounded source reconciliation

Status: ASSESSMENT COMPLETE — recommendation HOLD. Cadence repair, wider B01 modeling,
production and release remain HOLD.
Preparation source: `16cc53dbb17a0fad371ebeea36939d98c65ad36a`.

## Objective and authorization

Produce an independently reviewed source-provenance and test-scope decision for
resuming [verified cadence repair](../20260911-b01-wb14-verified-cadence-repair-001/package.md).
The predecessor stopped because its exact working145 recipe reproduced only
925 of 927 required file hashes. This package assesses the available source and
prepares a concrete revised-baseline proposal if exact recovery remains unavailable.
It does not adopt that proposal or resume runtime correction.

The owner requested this scaffold following Astra's recommendation to reconcile
source provenance and harness scope before further cadence work. Current authority
covers preparation of this record and locator/catalog updates only. An instruction
to execute this package adopts the bounded assessment below. Even after execution,
adopting a revised baseline or expanding cadence test-write scope is a separate
owner decision over the completed, reviewable proposal. The predecessor's exact
working145 requirement and historical HOLD are not rewritten by this package.

## Evidence and authority

Read the predecessor's recovery stop, [mismatch evidence](../20260911-b01-wb14-verified-cadence-repair-001/artifacts/recovery-mismatch.json),
[full build145 manifest](../20260911-b01-wb14-verified-cadence-repair-001/artifacts/build145-execution-identity.json),
[actual-byte verification](../20260911-b01-wb14-verified-cadence-repair-001/artifacts/parent-actual-byte-verification.json)
and its linked reconstruction recipe/logs. Retain these identities without revision:

| Item | SHA-256 |
| --- | --- |
| Expected working145 927-file map | `b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f` |
| Available partial-source 927-file map | `c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e` |
| Expected `precedence_tests.rs` | `730f9e36c83837251b080ede62069acabef5e19b828c59d1a5fad2a087ca4b10` |
| Available `precedence_tests.rs` | `982dac38c5b45b60eeb6db22a9ad839037359ff3fbc38fae11c10b069d434d76` |
| Expected `raw_hash_tests.rs` | `9b0c40bcada9706a100714fe7b9913bc016124b68be8f5e0f8716efae857d2ad` |
| Available `raw_hash_tests.rs` | `041c0df1739385995e5cc6c241510523e0e4c74bedfba34bf152c501461e53fd` |

Both test paths are under
`tests/integration/land_surface_energy_real_hydrology_shadow_contract/`.
Partial source is retained at
`/workdir/openwepp-experiments/b01-wb14-cadence/recovery-inputs`.
Neither an expected hash nor agreement on the other 925 entries reveals the
missing bytes or proves their semantic equivalence. Compare available variants
only against explicitly identified available variants; never claim an exact
missing-versus-available diff without recovering both versions.

Pin governing authority separately from runtime source at execution adoption.
For interpretation of the affected guards and acceptance, read applicable
science-contract instructions and the predecessor's selected canonical sections:
SC-SURFACELIQUID-001 `INV-012/013/014/026/035`, `OBL-C-016/C-025`, ordered guard
table and parent/child schema; SC-COUPLEDTIME-001 `INV-005/006/024/031`,
`OBL-014`; SC-SNOWENERGY-001 `INV-083/087`, `OBL-C-055`. Expand dependencies when
the two tests or consumer/harness inventory expose another affected obligation.
Package evidence does not replace canonical authority. The predecessor's
contract-local change-log finding remains an authority-maintenance follow-up;
this package does not amend science or adjudicate it away.

## Proposed execution envelope

Astra owns assessment, integration, custody and disposition under
[bounded execution](../../standards/bounded-agent-execution.md). An assigned
implementer owns detailed source/test inspection and deterministic evidence
scripts. Use explicit read/write bounds and no nested delegation. Retain two-cycle
or 60-active-minute internal reassessment; continue only with a concrete supported
in-scope action. Historical recovery failures/time remain in the predecessor.
New package accounting never makes exact working145 recoverable or resets its
consumed allowance.

Allowed writes on execution: this package's record and artifacts, locator/catalog
status, and new source copies/evidence scripts under the proposed durable root
`/workdir/openwepp-experiments/b01-wb14-source-reconciliation/`. Inspect that
destination before creating anything; preserve existing unique state. Use repo
`.venv/bin/python`. Keep the predecessor's partial source, manifest, original
cadence patch and logs immutable. Do not create/change branches. Carry scoped
evidence commit/push permission from the predecessor and verify publication
before claiming remote custody; this grants no production-source publication.

Source snapshots may be reconstructed or copied into new directories for
inspection, with exact input identities, safe paths and logged operations.
An independently named revised-source proposal may retain the available two
files verbatim. Do not author replacement Rust, alter test assertions, apply
the cadence candidate, format source, or manufacture bytes to meet a hash.
No test harness may be added or replaced here.

No Rust builds, test execution, authentic runs, saved-checkpoint resume,
performance/warm/seven-day runs, restart repair or production integration.
No equations, solver, thresholds (including B01 0.1/0.12 mm SWE), tolerances,
features/defaults/cfg/test gating, manifests, lockfiles, dependencies, toolchains,
schemas, required-case bindings or canonical contracts may change. Static Cargo
target/module inspection is allowed; inventory is not a compiled test listing.

Stop dependent assessment on unexplained source drift beyond the two known
paths, inaccessible necessary primary evidence, contradictory science authority,
or a need to edit a forbidden surface. Preserve results and return the specific
decision. The already known absence of the two historical files does not prevent
the authorized revised-source assessment; new missing evidence required to
support that assessment does. Do not repeat the exhausted archive/Git search
without a newly identified source and a specific recovery hypothesis.

## Checkpoints and required deliverables

1. **Reconcile available bytes.** Rehash all 927 actual entries against the
   original manifest and reproduce the two mismatches. Verify retained R0/final016
   and correction145 inputs by their identities and evidence; distinguish
   re-executed reconstruction from reused historical stage checks. Record
   symlinks, missing/extra paths and build/test inputs outside the 927-file map.
   A match on that map alone is not a complete workspace/input identity.
   If fresh evidence recovers the historical files, verify their exact hashes
   and the entire reconstructed map before proposing the exact-recovery route.
2. **Assess the two tests.** Trace each available file's provenance through
   retained patches and available Git versions. Bind actual test names, enclosing
   modules, Cargo registrations, feature/ignore conditions, fixtures, assertions,
   error-precedence and raw-hash obligations, and production paths exercised.
   Assess risks to cadence, complete-owner rollback, receipt authentication,
   serialization and independent operands. Label unavailable historical changes
   UNKNOWN, not formatting-only or behavior-preserving. Explain whether and how
   fresh contract-derived tests could supply sufficient prospective coverage;
   do not claim they restore historical evidence or implement them here.
3. **Resolve harness scope prospectively.** Inventory the real ignored
   fresh-process prefix/cadence harness across reconstructed source, exact module
   registration and retained references. Give its source, fully qualified test
   name, process entrypoint, fixture lineage, features and proposed explicit
   ignored-test command. Separate the exact day-4/interval-22/transaction-255
   boundary from generic cadence tests. Trace actual ingress/WB14 and receipt
   custody, split/replay and complete-parent coverage. If missing or outside
   prior write bounds, identify the evidence gap and propose the minimum exact
   test-path/scope amendment for owner adoption. Do not substitute the unrelated
   terminal-event cadence test or claim a static name executes successfully.
4. **Prepare one source decision.** Recommend either verified exact recovery,
   a distinctly named revised baseline, or rejection/HOLD. A revised proposal
   must contain recoverable source bytes/base-plus-patches, full actual input
   identity, the two explicit substitutions, unchanged production-file proof
   relative to the retained recipe, unknowns and their impact, and a concrete
   amendment to the predecessor's source/test envelope recorded here only.
   Include any required expanded harness paths and commands. If unchanged
   production or sufficient test authority cannot be established, recommend HOLD.
5. **Review and preserve.** Correct findings within these bounds, retain raw
   comparisons and attributable review in this record, reconcile the terminal
   diff, and verify committed/remote evidence and necessary recovery bytes
   (including any required LFS content). Report local-only or missing custody
   explicitly. Return the concrete owner decision; do not resume cadence work.

## Acceptance and validation

Assessment success means a supported, independently reviewed recommendation,
including a negative recommendation if warranted. It never means cadence repair
PASS, source adoption, production readiness or empirical validation.

| Requirement | Acceptance evidence |
| --- | --- |
| Source identity | Actual-byte 927-file comparison, complete relevant workspace/input inventory, preserved historical identities and source-custody proof; no unnamed substitution. |
| Semantic claims | Primary source locations and available-version diffs; unavailable historical semantics explicitly UNKNOWN with consequence assessed. No unsupported equivalence claim. |
| Test authority | Registered target/module/feature/fixture/assertion lineage for the two files, touched canonical obligations and concrete coverage gaps. No weakened or removed requirement. |
| Harness | Verified static entrypoint/registration and proposed command, or a precise absence/out-of-scope finding plus proposed amendment. No helper-only or authentic-resume substitute. |
| Decision | One exact-source/revised-source/HOLD recommendation; proposed revised identity and scope are reviewable, but not adopted. Every material unknown receives an explicit disposition. |
| Preservation | Any exact/revised source proposal can be reconstructed from retained available inputs. A negative HOLD recommendation instead preserves the inspected input identities, negative evidence and custody needed to reproduce its assessment; it need not fabricate a new source proposal. Original partial source and hashes remain intact. Remote claims follow actual retrieval. |

Apply [testing strategy](../../standards/testing-and-gate-strategy.md) directly.
This package changes no executable behavior or authority: run diff/whitespace,
local-link, JSON/schema as applicable, deterministic actual-byte/hash and
comparison checks, script syntax and focused script checks for any added evidence
tool. Record exact argv/cwd, source/input identities, exit status and logs; use
`Static:` versus `Ran:` accurately. No Rust validation is claimed or required
solely for this assessment. If work would require changing executable behavior,
stop for scope instead of silently narrowing its validation.

All predecessor runtime acceptance remains required at any subsequently adopted
cadence execution: matched retrospective red/candidate fixtures, all 18 physical
children through 387000 s, ordinary/rollover and compound-poison controls, exact
complete-owner rollback, real fresh-process/split path, independent operand
reconstruction, warnings-denied Clippy, full correctness and applicable A0/A1/A3.
Baseline replacement does not turn historical failed/NOT RUN checks into passes.
The original preimplementation sequencing violation remains explicit; future
tests and gate evidence must precede further production edits. No numerical
calibration is involved; calibration and identifiability are NOT_APPLICABLE.

Two independent execution reviewers are required because a proposed source/test
trust change has consequential unknowns. Correctness/authority owns semantic
claims, guard/coverage authority, historical sequencing and admissible next scope.
QA/evidence owns actual bytes, reconstruction/input identity, harness registration,
publication and anti-evasion. Neither may have authored the reconciliation or
materially designed the revised-source proposal; a prior recovery author cannot
independently accept their own reused recovery claims. Same reviewers verify
fixes. Missing required review prevents assessment acceptance. Scaffold review
does not substitute for these execution reviews or future cadence reviews.

## Current state and next action

The source/test assessment now recommends HOLD. The inspected revised-source
candidate is recoverable, but the complete real-runner test bridge remains
unbound. Both independent execution reviews accept this negative assessment. Evidence
publication is the final custody step below. Cadence
repair remains HOLD; the proposed baseline and expanded test scope are not adopted.

### Scaffold checks and review

Ran from `/workdir/openWEPP`: `git diff --check`, explicit whitespace/conflict
checks on the new untracked record, and `.venv/bin/python` checks of six local
references and all six retained source/file hashes: PASS. Terminal changes are
this record and the two locator/catalog additions only. No runtime or source
reconciliation workflow ran. At scaffold review the changes were local and
uncommitted; the owner subsequently authorized committing and pushing them.

Independent scaffold reviews reused `/root/cadence_correctness` (Sol/high) and
`/root/cadence_implementation` (Terra/medium, scaffold wording QA only), neither
of whom authored this scaffold. The latter explicitly excluded their own prior
recovery evidence and cannot independently accept that evidence during execution.
Correctness identified one medium acceptance ambiguity: an unconditional
reconstructable-proposal requirement could prevent an otherwise valid negative
HOLD assessment. The preservation row now distinguishes those outcomes; both
same reviewers verified the fix. Attributable final responses:

> Correctness: Fix verified. Static source: untracked package blob
> 261d32309cfc8e3450bc7985beae6df412319385, SHA-256
> 61994dfd03db009db806636e0dc861f6cfc25f1e036a9d0834ee70ff59d2d557
> over HEAD 16cc53db.... Line 158 now makes reconstructability conditional for
> exact/revised proposals and gives negative HOLD an explicit preservation/evidence
> posture without fabricating a proposal. Finding closed; final scaffold verdict
> PASS, no blocking findings. Scaffold only; no assessment execution/source
> adoption/runtime approval.

> QA: Static QA: PASS. The corrected preservation row properly makes
> reconstructability conditional on an exact/revised proposal and allows a
> negative HOLD to preserve identities, negative evidence, and custody without
> fabricating source. Prior recovery evidence remains excluded.

These reviews bind the substantive scaffold before this checks/review appendix;
they do not approve the assessment's unexecuted claims. Preparation is complete;
execution at that scaffold boundary remained NOT STARTED.


## Execution ledger

Owner execution adoption: 2026-09-11 local date; work began 2026-09-12 05:40 UTC.
Primary checkout and governing authority are pinned separately from runtime bytes
to `1497c078cfa524315763e17bdb225594480be0b4`, initially clean. Runtime inspection
uses the immutable predecessor partial source and retained recipe named above.
The proposed durable destination was absent before assignment.

Intent: source-provenance and static test/harness assessment only. Allowed writes
are this record/artifacts, locator/catalog, and new evidence/source copies under
`/workdir/openwepp-experiments/b01-wb14-source-reconciliation/`. No executable
behavior, science authority or test assertions change. Checks selected before
measurement: deterministic actual-file hashes and complete input inventory,
available-version comparisons, static module/target/feature/fixture tracing,
script syntax/focused checks, JSON parsing, local links and diff whitespace.
No Rust build/test or authentic run is authorized (allowance 0); no empirical
or runtime PASS will be inferred. Stop conditions and two-cycle/60-active-minute
internal reassessment above remain binding.

Assigned `/root/reconciliation_implementation` (implementer, Terra/medium) the
bounded detailed checkpoints 1–4, with artifact/durable-root writes only and no
nested delegation. Astra owns this record, authority interpretation, integration,
two complementary independent execution reviews, and evidence publication under
carried permission. Reviewers will inspect a stable substantive assessment and
verify their own accepted fixes. Current next action: collect actual-byte and
static coverage evidence, then formulate the concrete recommendation for review.


### Governing authority and claim boundaries

Static: authority remains the adoption commit above, independently of the older
runtime recipe. Read SC-SURFACELIQUID-001's ordered guard table (lines 659–684),
parent/child schema and chronology (1430–1548), inactive custody (1702–1742), and
native-prefix amendment (2148–2206); SC-COUPLEDTIME-001's complete owner/commit
and receipt invariants (582–601) and prefix obligation (1009–1040); and
SC-SNOWENERGY-001's represented-snow and prefix companions (3157–3206,
3358–3396). Their selected invariant/obligation IDs remain those in this record.
The explicit isolated B01 qualifications remain binding; this assessment changes
no regime classification, threshold or physics.

The authority requires authenticated complete accepted-prefix chronology before
parent-local cadence admission, zero inactive-prefix physical work, unchanged
beginning cumulative bits, physical ordinal zero, full-parent partition and
finalization, replay/restart identity, and complete accepted/staged-owner rollback.
Successful complete-parent validation is a prerequisite for alternate admission;
it does not override simultaneous-error precedence. Presence or inequality of
failure hashes cannot prove the complete rollback owner set. Fresh future tests
could establish prospective coverage under those obligations, but cannot recover
unavailable historical semantics or cure the predecessor's preimplementation
sequencing violation. The contract-local change-log follow-up remains outside
this package; it is neither amended nor dismissed here.


## Assessment results and source decision

Ran: deterministic identity comparisons and fresh retained-recipe replay.
Static: test semantics, registration and prospective coverage assessment.
No Rust build/test or authentic run occurred. Results below integrate the
implementer return; raw evidence is retained in [artifacts](artifacts/).

### Frozen predicates and reproducibility

`/workdir/openwepp-experiments/b01-wb14-source-reconciliation/protocol.md`
freezes the byte and harness predicates before measurement.  `audit.py` is the
deterministic read-only collector; it was run with repo `.venv/bin/python`.
Its JSON result and command log are retained beside it.

### Checkpoint 1 — retained bytes and inputs

The retained `recovery-inputs` root is not a Git worktree.  Recalculation of
the canonical 927-entry expected manifest map is
`b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f`.
The complete actual path-to-hash map is retained in
`actual-byte-inventory.json`; its canonical actual-map digest is
`c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e`.
It has 925 matches and exactly two mismatches, with no missing map paths:

| path | expected SHA-256 | available SHA-256 |
| --- | --- | --- |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs` | `730f9e36c83837251b080ede62069acabef5e19b828c59d1a5fad2a087ca4b10` | `982dac38c5b45b60eeb6db22a9ad839037359ff3fbc38fae11c10b069d434d76` |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract/raw_hash_tests.rs` | `9b0c40bcada9706a100714fe7b9913bc016124b68be8f5e0f8716efae857d2ad` | `041c0df1739385995e5cc6c241510523e0e4c74bedfba34bf152c501461e53fd` |

The two inspected available files are preserved verbatim in the durable root.
All other mapped paths match, which is a production-file unchanged proof within
the retained 927-file recipe map; it does not prove complete workspace identity
or the unknown historical test semantics.  Beyond the map the root has 35,130
regular files, 1,318 file symlinks and zero directory symlinks; their explicit
paths/targets are in
`actual-byte-inventory.json`, including configuration, Cargo, build/include and
fixture surfaces.  Relevant beyond-map test/config/build inputs are separately
classified and hashed there; direct source `#[path]`/`include!` resolution is
also recorded. `actual-byte-inventory-first-attempt.json` preserves the first
incomplete inventory before the corrected rerun. The retained historical R0 (701 files) and final016 (41
changed files) checks are reused historical evidence, not re-executed stages.

**Ran reconstruction:** `reconstruct_available_source.sh` replayed the retained
base commit plus R0, final016 and correction145 inputs into the new durable
`reconstructed-available145` root.  A whole-source fresh-versus-retained census
then rehashed all 36,057 regular files and compared all 1,318 symlink targets:
zero differences, regular-file digest
`8b29e888b96c223f46d8c2432c49560bbf68c15871491cfdaa68b1221cd373c6`,
symlink-target digest
`6c50ef172d22672ba98f0b66f3a45da1849b0c47409f771affd079430196b58f`.
No symlink resolves outside the source root. Direct `#[path]`/`include!`
references from the test parent all resolve inside the root and are hashed in
the inventory. This proves recipe replay equals the retained available source;
it does not manufacture the two unavailable expected test bytes.

The exact reconstruction input list is in that JSON.  Key identities are
R0 composition patch `5db3436348bdaae05cfe458d33b97075c6a4e820d4fb474d506b0da17e22d943`,
PBC final016 patch `91878772522b22a733401600dc7c5b1442f627ab2357d6679ad0d030e2a69e88`,
correction145 patch `52e53716f3a640a1f90984b2294557f42fe629e91dcd444728db0a93590cdc1d`,
and raw-stop-point145 archive
`5365a6e47b67d23663f668cbe5c1d6c1b063cf599abcd8be0c37115135c38985`.

### Checkpoint 2 — available test scope

Both files are reachable only through the `#[cfg(test)]` crate module
`raw_boundary_contract_tests` at
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:77`,
whose parent is `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs`.
That parent registers `raw_hash_tests` at lines 36–37 and `precedence_tests` at
39–40. No Cargo `[[test]]` path names this parent: the similarly named Cargo
target at `Cargo.toml:850–851` instead points to
`land_surface_energy_strict_v8_public_contract.rs`. Thus these are available
crate-test modules, not a separately registered integration target.

The machine inventory lists every actual test name, line, error code, direct
production call, and assertion form: 18 `precedence_tests` plus four
`raw_hash_tests`, all nonignored.  The former cover E002 identity precedence,
E003 domain checks, E007-over-finite-cadence and E003-nonfinite-interval
poisons (not a direct E008 assertion), protocol cases, callback
non-entry and rollback/hash checks; the latter require raw NaN payload bits in
attempted configuration/state/snapshot hashes and assert E002 where applicable.
Fixture lineage is `production_frame:55`, `owner:77`,
`surface_configuration:203`, `configured_surface_frame:240`,
`surface_potential_batch:313`, and poison/final protocol helpers at 1385/1741.
The direct production calls are
`execute_unified_real_hydrology_shadow` (real_hydrology_execution.rs:5),
`configure_surface_liquid_shadow` (00_core_frames.rs:677), and
`unified_beginning_hydrology_snapshot_sha256` (mod.rs:866).

The B01 positive-origin native-drain exception (SC-SURFACELIQUID-001 around
line 2208) is a separate, pending physical-posture topic.  It neither supplies
nor weakens the required zero-WB14 inactive-prefix rule for this parent; this
assessment makes no claim that the two are semantically equivalent.

These assertions bear on SC-SURFACELIQUID-001 ordered guards and exact custody
(notably INV-012/013/014/026/035 and OBL-C-016/C-025), coupled-time ordering
(SC-COUPLEDTIME-001 INV-005/006/024/031 and OBL-014), and snow receipt boundary
constraints (SC-SNOWENERGY-001 INV-083/087 and OBL-C-055).  They do not by
static inspection establish the required full day-4 ingress lifecycle,
complete-owner rollback, receipt authentication, serialization, split/replay,
or independent operands.

Available Git history provides 13 `precedence_tests` and five `raw_hash_tests`
versions; current HEAD is exactly the available pair (`982d…` and `041c…`).
Their per-commit hashes are in `static-test-harness-inventory.json`. The
[complete available-version trace](artifacts/available-version-provenance.json)
now binds every parent-to-commit diff, parent/commit identity and subject for
all 13/5 available versions under [version-diffs](artifacts/version-diffs/).
The retained newest available transition patch diff (0edf0457 to 96c46c88) is
`logs/available-git-head-diff.patch` (`324a6215…`, 35 lines). The three retained
reconstruction patches and the two relevant tar member lists contain no direct
path occurrence, so they do not identify the source of either expected byte
version. Neither available Git history nor retained patch trace supplies the
unavailable expected versions. Their semantic delta is therefore **UNKNOWN**,
not formatting-only or behavior-preserving. Contract-derived
prospective tests could cover a future source, but cannot recreate historical
evidence.

### Checkpoint 3 — harness inventory

The machine inventory provides registration, features, entrypoints and limits
for each nearby harness. The available ignored B01 test is
`openwepp_hillslope_orchestrator::direct_runtime::stage3_committed_publication_tests::actual_day_four_thin_snow_publication_preserves_owner`
(`stage3_committed_publication_tests.rs:747`).  It installs B01 policy and
checks a thin-snow publication owner; its retained excerpt is preserved.  It
does not show the required day-4/interval-22/transaction-255 parent, real
surface-liquid ingress/WB14 path, receipt custody, or split/replay.

Native-prefix unit tests and an ingress-context cadence unit test exist, but
they are not the required ignored fresh-process harness.  Searches of the
retained source found no source-visible test binding all required boundary
values with an ignored fresh-process registration.  Static absence is limited
to this retained partial source; it is not an assertion of global absence.

The assessed candidate routes have two test layers and a named diagnostic
observation seam. They remain incomplete pending the outer-transaction bridge
identified below; no sufficient scope amendment is recommended. No harness or
observer is implemented here. The inspection
source remains immutable: on later owner adoption, create `baseline-red` and
`candidate` copies under the predecessor durable root from
`reconciled-available145-r1`. Add the identical tests/observations to both before
any further cadence correction. The candidate command below is prospective; use
the same command with `baseline-red` source/target for its matched retrospective
red. No current static name implies a selected or passing test.

#### Execution identity

The existing real-consumer ignored entry is
`hillslope::tests::stage3_snow_accuracy_case`, included at
`crates/openwepp-runner/src/hillslope/03_tests.rs:49` from
`tests03/snow_accuracy_experiment.rs:4`. It calls
`execute_hillslope_run_with_runtime_policy` at line 155 using the direct
production executor. This is the closest retained ignored real-consumer runner test;
it has no WB14 day-4 boundary. `controlled_mechanism_experiments.rs:3` is also
an ignored experiment without a source-visible self-spawn or this boundary.
Neither existing experiment proves fresh-worker isolation; the self-spawn
pattern below comes from the separate runner qualification test.

The new test belongs in the existing runner include surface, not the synthetic
orchestrator `raw_boundary_contract_tests` module:

* write path: `crates/openwepp-runner/src/hillslope/tests03/snow_accuracy_experiment.rs`;
  add `wb14_day4_interval22_transaction255_fresh_process_prefix_cadence_replay`
  and its non-test worker helper in that file;
* registration: existing `03_tests.rs:49 include!` already compiles the file;
  no new `[[test]]` target;
* no Cargo manifest change is proposed. Runner directly depends on the
  orchestrator, so the workspace command selects the existing dependency
  features `openwepp-hillslope-orchestrator/persisted-restart-v1` and
  `openwepp-hillslope-orchestrator/restart-authority-evidence`.

Proposed driver command, not run:

```text
cd /workdir/openwepp-experiments/b01-wb14-cadence/candidate
nix develop /workdir/openWEPP --command env \
  CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-cadence-targets/candidate \
  RUST_MIN_STACK=67108864 cargo test --manifest-path /workdir/openwepp-experiments/b01-wb14-cadence/candidate/Cargo.toml \
  -p openwepp-runner --lib --locked --release \
  --features test-fixture-authority,openwepp-hillslope-orchestrator/persisted-restart-v1,openwepp-hillslope-orchestrator/restart-authority-evidence \
  hillslope::tests::wb14_day4_interval22_transaction255_fresh_process_prefix_cadence_replay \
  -- --ignored --exact --nocapture
```

The driver must spawn `std::env::current_exe()` with `--exact`, the same FQ
test name, `--ignored`, and `--nocapture`, set
`OPENWEPP_WB14_PREFIX_CADENCE_WORKER=1`, retain inherited feature/cwd identity,
and require child success plus evidence that exactly one named worker test
executed. A skipped or zero-selected child process cannot pass. The worker branch is the only
branch allowed to construct/run the fixture. This is source-derived: the
retained `accepted_stage3_real_runner_routes_lane_d_and_publishes_summary`
driver at `stage3_runner_qualification.rs:661–692` self-spawns under an env
guard and its worker then builds a native fixture and calls the real runner;
`cqr_laned_active_configure_wrapper_builds_authoritative_active_config_without_routing`
at `cqr_laned_active_outputs.rs:1168–1205` similarly self-spawns per isolated
environment case. The actual worker process entrypoint is
`execute_hillslope_run_with_runtime_policy` as above, not a direct helper.

#### Two required layers and visibility boundary

The runner worker cannot construct `DirectWb14ParentWorkingState`,
`DirectWb14CoupledChildBindingV1`, `DirectWb14ParentIntervalV1`, or
`ValidatedNativeInactiveWb14PrefixV1`: their ingress/parent APIs are
`pub(crate)` in the orchestrator. The future scope therefore has two layers.

1. The runner test above supplies the fresh self-spawned real-runner transition
   and actual accepted publications. It must not call private ingress APIs.
2. Existing orchestrator crate-test modules supply private construction, poison,
   replay, complete-owner and independent-operand vectors:
   `crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_context_tests.rs`
   (registered by `surface_liquid_ingress.rs:2996`) and
   `surface_liquid_wb14_native_prefix_tests.rs` (included by
   `surface_liquid_wb14.rs:2973`), plus their existing test-only sections.

The existing public `snow_accuracy_observer` is used by
`snow_accuracy_experiment.rs:139–178` to spool observations, and the runner
qualification audit exports a committed snapshot. Neither currently records a
successful surface-liquid/WB14 child, receipt count, parent support, or child
ordinal: `surface_liquid_ingress.rs:1070–1085` records only a caller failure.
Consequently current exports cannot prove the actual runner 18-child boundary.
The minimum additional observation seam is a feature-gated, non-public,
test-only diagnostic record in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress.rs`,
immediately after a successful parent-bound ingress result, emitted through the
existing `snow_accuracy_observer::record` sink. It must expose parent support,
child support/ordinal, transaction/day/interval, receipt count, WB14-call delta,
beginning/ending owner hashes and immutable receipt bytes/digest. It must be
gated by the existing `restart-authority-evidence` feature, introduce no public production
API, and be bound by a future contract-derived test before the runner harness.

Without that exact private observation seam (or a subsequently identified
existing public receipt export with the same fields), runner evidence remains
insufficient and the revised route is HOLD for cadence execution.

#### Required private-vector API binding

Private-vector fixtures must set day 4, interval 22, transaction 255; authenticate the
parent `[385200,387000)` s and prefix `[385200,385920)` s, then submit 18
60-second physical children/OFE through `387000`. They must reach
`execute_surface_liquid_ingress_with_parent_state_and_coupled_binding`
(`surface_liquid_ingress.rs:969`) using a `DirectWb14ParentWorkingState` and
`DirectWb14CoupledChildBindingV1`; the ordinary public ingress function at 896
is only a control, not the target path. Parent setup/replay uses
`DirectWb14ParentIntervalV1`, `ValidatedNativeInactiveWb14PrefixV1`, and the
native prefix validator (`surface_liquid_wb14.rs:639`).

Private split/replay vectors must use the complete coordinator methods
`DirectWb14ParentWorkingState::restart_bytes(configuration)` and
`DirectWb14ParentWorkingState::from_restart_bytes(configuration, bytes)` at
`surface_liquid_ingress_coordinator.rs:714/723`; both validate nested custody.
Retain `wb14_parent_replay_bytes`/`wb14_child_replay_bytes`
(`surface_liquid_ingress.rs:401–411`) and compare uninterrupted/reconstructed
complete coordinator, candidate-owner and receipt bytes exactly. The per-OFE
serde roundtrip in `surface_liquid_wb14_native_prefix_tests.rs:31–45` is a
supporting scalar test, not complete coordinator or runner restart proof.
No unspecified runner wire is claimed tested; the outer consumer restart seam
must also be bound before future execution acceptance.

#### Proposed private-vector commands

The two new orchestrator cases are proposed in their existing registered
modules as
`direct_runtime::surface_liquid_ingress::context_tests::wb14_prefix_cadence_complete_parent_and_precedence`
and
`direct_runtime::surface_liquid_wb14::tests::wb14_prefix_cadence_restart_and_poison_matrix`.
Runner dependency compilation does not execute these crate unit tests. Run each
explicitly for both matched sources after owner adoption and test preparation:

```sh
for variant in baseline-red candidate; do
  SRC=/workdir/openwepp-experiments/b01-wb14-cadence/$variant
  TARGET=/tmp/openwepp-b01-wb14-cadence-targets/$variant
  cd "$SRC"
  for test_name in \
    direct_runtime::surface_liquid_ingress::context_tests::wb14_prefix_cadence_complete_parent_and_precedence \
    direct_runtime::surface_liquid_wb14::tests::wb14_prefix_cadence_restart_and_poison_matrix
  do
    nix develop /workdir/openWEPP --command env \
      CARGO_TARGET_DIR="$TARGET" RUST_MIN_STACK=67108864 \
      cargo test --manifest-path "$SRC/Cargo.toml" \
      -p openwepp-hillslope-orchestrator --lib --locked \
      --features persisted-restart-v1,restart-authority-evidence \
      "$test_name" -- --exact --nocapture
  done
 done
```

Execute the two adopted available test modules as well on both sources:

```sh
for variant in baseline-red candidate; do
  SRC=/workdir/openwepp-experiments/b01-wb14-cadence/$variant
  TARGET=/tmp/openwepp-b01-wb14-cadence-targets/$variant
  cd "$SRC"
  for test_module in \
    land_surface_energy_shadow::raw_boundary_contract_tests::precedence_tests:: \
    land_surface_energy_shadow::raw_boundary_contract_tests::raw_hash_tests::
  do
    nix develop /workdir/openWEPP --command env \
      CARGO_TARGET_DIR="$TARGET" RUST_MIN_STACK=67108864 \
      cargo test --manifest-path "$SRC/Cargo.toml" \
      -p openwepp-hillslope-orchestrator --lib --locked \
      --features persisted-restart-v1,restart-authority-evidence \
      "$test_module" -- --nocapture
  done
done
```

These module selections must execute all 18 precedence and four raw-hash tests,
respectively, on both baseline and candidate; all 22 remain required, with no
assertion weakening or inherited-failure waiver. Their execution still cannot
restore the unavailable historical versions or prove full-parent coverage.

Each exact selection must execute one named case. Retain the expected baseline
failure and all ordinary/negative baseline controls; require the candidate
cases and controls to pass. The same baseline/candidate substitution applies
to the ignored runner command above, with child processes inheriting that
source cwd. These commands supplement, not replace, the predecessor's full
correctness, Clippy, A0/A1/A3 and all other acceptance.

#### Required observations

For every private-vector refusal, compare before/after complete live, persistent and staged
owner bytes for snow, litter V3/V4, LSE, surface, hydrology, soil/soil-thermal,
vegetation and coupled clock; parent candidate; provider/GSI/event cursors;
complete accepted receipt/chronology/history; publication buffers/receipts; and
physical-call counters. Positive rows require zero prefix WB14 calls, receipts,
routed parcels and cumulative **deltas** while preserving unchanged beginning
bits; they do not require zero absolute beginning cumulatives. Physical children
then require nonzero distinguishing inputs/receipts/counters. Independent operands must be reconstructed from
beginning/ending owner snapshots plus immutable input and actual transfer
receipts, with test-owned calculations over separately read operands. The production
validator at `surface_liquid_owner/v3_exact_enthalpy.rs:972,1079` is a consumer
under test, not the independent oracle; its residual/carry cannot supply the
expected answer.

#### Poison and precedence matrix

Each listed row must reject at its canonical phase and preserve the complete
accepted state; preflight poisons require zero physical calls, while later
closure/finalization poisons may discard unpublished candidate work. Retain the full
rollback set: missing/foreign/resealed prefix; wrong parent, owner,
configuration, model, parameters, topology or lane; support/accepted-until
mismatch; omitted scalar authority; duplicate, gapped, overlapping or replayed
child; duplicate/replayed finalization; nonfinite/domain poison; and compound
identity-vs-domain, custody-vs-cadence, protocol-vs-receiver poisons. Assert
canonical order: E002 identity before E003 domain; E007 exact-one custody
before E008 cadence; and in `finalization_applies_precedence_across_protocol_and_all_receiver_sets`,
E002 identity wins while otherwise receiver E003 wins over protocol-cardinality
E005. Preserve E010 independent-closure reporting under the canonical ordered
guard table; this proposal adds no error-precedence rule. Existing static seams are
`precedence_tests.rs:116–1134`, ingress context `:284–330`, and WB14 chronology
poisons `surface_liquid_wb14.rs:2321,2740–2878`; they inform vectors but do not
substitute for the runner path.

### Complete outer-transaction evidence and remaining gap


The retained source has an actual complete-owner transaction test seam; it is
not the local LSE integration fixture. `DirectV10RealConsumerShadow` owns
`inner` (hydrology/surface), vegetation, LSE, GSI, provider cursor, accepted
publication history, and frozen-litter V3/V4 residents
(`v9_real_consumer_shadow.rs:408–430`). Its complete-owner projection is
`canonical_v11_parent_owner_state_bytes` and the physical comparison adds snow
(`canonical_owner_bytes.rs:1062–1090`).

The real covered consumer path is
`execute_covered_real_v11_parent`. The existing private helper
`exercise_complete_wb14_cadence` in
`v9_real_consumer_shadow_wb14_tests.rs:32–2110` constructs a `V11ParentTransaction`,
`CoupledClockStateV1`, `DirectV10RealConsumerShadow`, and Stage-3 lane state;
it saves the parent checkpoint JSON, complete owner state, litter publication
history/WB14 restart bytes, coupled-clock JSON, and Stage-3 owner bytes at
`:1270–1319`. For each existing injected failure it calls the real covered
consumer and requires equality of the parent, consumer, clock, and Stage-3
state (`:1320–1340`); the canonical parity-poison branch repeats byte-level
comparisons through those same outer owners (`:1993–2060`). This is the
appropriate orchestrator-crate private test surface for a refusal after an
actual covered parent transaction. The existing native-prefix test
`native_inactive_prefix_validator_consumes_real_ordered_terminal_chronology`
in `snow_stage3_v11_adaptive_production_tests.rs:1325–1405` derives its prefix
from that real complete-owner cadence, rather than an unrelated fixture.

The existing real runner seam is
`duplicate_configured_mapping_rejects_real_stage3_day_without_owner_or_clock_mutation`
in `crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs:1490–1607`.
It calls `execute_hillslope_run_with_runtime_policy`, takes the direct-production
path, and asserts no attachment adoption/no accepted-publication history plus
the `Stage3RejectedDayRollbackAudit`. That audit clones the complete
`DirectRunFrame` and serializes the V11 parent checkpoint and coupled clock
before execution (`05_runner_execution_and_outputs.rs:52–75`); on error it
compares all three after the real prepare callback (`:77–112, 413–445`).
`DirectRunFrame` contains lanes, publication, transfer ledger/operands,
groundwater, surface-liquid owner, Stage-3 attachment, and routing state
(`direct_runtime/00_core_frames.rs:580–609`). Runner output and archive paths
remain in `HillslopeOutputTransaction` private/staged spools until success;
its `Drop` rolls back an uncommitted transaction
(`output_transaction.rs:31–41, 410–419`).

Accordingly the future scope must add the WB14 refusal case to this existing
runner qualification surface (or a fresh-process worker in that surface), and
must retain the existing complete-frame plus parent/clock comparisons. It must
also assert absence of final output, manifest, and Stage-3 evidence paths after
the expected error, because the retained audit currently does not inspect
those transaction files. This is source-backed for a complete outer
transaction rollback; it does **not** make the current duplicate-configured
mapping poison evidence of the day-4/interval-22 native-prefix refusal.

There is no retained cross-crate way for a runner test to install the existing
phase-specific injection. `Stage3V11FailureInjection` and
`DirectSnowStage3V11ShadowAttachment::{inject_failure_after_subslab,
inject_failure_after_outcome_ledger,inject_failure_after_final_owner_join}`
are `pub(crate)` and `#[cfg(test)]` in the orchestrator
(`snow_stage3_v11_attachment.rs:2809–2820`,
`snow_stage3_v11_attachment_runtime.rs:502–520`); the runner cannot access
them. A viable future amendment therefore needs an explicit test/evidence-only
runner-to-orchestrator injection/observation seam with no public production
API, or a contract-derived external input poison demonstrated to reach the
same post-prefix point. No such seam is present in the retained source. Until
one is added and tested, the runner can prove generic full-frame refusal but
cannot nonvacuously prove that the complete runner transaction rolls back on
the required native-prefix refusal.

The zero-prefix physics condition has the same visibility limit. The only
retained counter is `record_native_wb14_physics_entry_v1`, defined and exported
only under `#[cfg(test)]` in
`v9_real_consumer_shadow/frozen_litter_v4_adoption.rs:289–295` and
`v9_real_consumer_shadow.rs:117–123`; its sole recorded ingress call is also
`#[cfg(test)]` at `direct_runtime/surface_liquid_ingress.rs:1203–1205`.
It is unavailable in a runner dependency build, and success-only child
observations cannot detect an unwanted prefix construction call. The minimum
additional evidence-only scope must make a total counter observable across the
prefix-construction through suffix interval (using this existing counter call
site and its underlying audit) and bind it to the new runner worker. The
counter must report zero for the inactive prefix and a nonzero suffix delta;
without that added feature-gated evidence, cadence execution remains HOLD.

`execute_unified_real_hydrology_shadow` is not this outer seam: it is
`pub(crate)` at `land_surface_energy_shadow/real_hydrology_execution.rs:5` and
the two recovered test files call it through a local adapter. Those tests can
provide receiver/error semantics, but do not own the V11 parent, Stage-3
clock, runner transaction spools, or final publication boundary.


Historical boundary fixture lineage is
[gradual-warm-tail7-cases.json](../20260910-stage3-b01-snow-cycle-integration-001/artifacts/gradual-warm-tail7-cases.json),
SHA-256 `c48cc560dff31102085ebcf852225b2063a1629ad96cfa9f7283364df44fbf3f`,
case `gradual_warm_tail7`, one OFE with policy B01. Its
[identity](artifacts/historical-boundary-input.json) is retained separately
from runtime-source identity. Boundary numbers alone do not bind a future
worker to this fixture. No tail7 run, authentic saved-checkpoint resume or
replacement forcing is authorized here. Any later bounded contract-derived
fixture must bind its derivation to this original input, retain the exact
failure case and add separate heterogeneous/nonzero controls.

### Checkpoint 4 — decision

**Recommendation: HOLD.** The retained recipe reproducibly yields
`reconciled-available145-r1`, with the two explicitly substituted tests and
whole-source identities above. That inspected candidate is recoverable, but
source adoption is not recommended yet. Sufficient prospective test authority
has not been established: the real runner lacks a demonstrated late-phase
failure-injection and total-physics-count bridge, and its exact bounded fixture
and complete restart/rollback observation must be bound to the original case.
The private test routes and proposed commands above are necessary candidate
components; they are not a complete, accepted test-scope amendment.

The [assessed source candidate](artifacts/revised-baseline-proposal.json) now
records `ASSESSED_NOT_RECOMMENDED_HOLD`. It is retained as inspected evidence,
not a second source recommendation or authorization. The missing historical
file semantics stay UNKNOWN and the predecessor's sequence stays RED. Fresh
contract-derived tests can supply prospective coverage only after the missing
real-consumer bridge is concretely specified, owner-adopted and validated;
they cannot restore historical evidence.

The next owner decision is whether to authorize preparation of that narrow
evidence/test-interface scope before reconsidering source adoption. Its exact
known surfaces are the runner `tests03/snow_accuracy_experiment.rs` and
`tests03/stage3_runner_qualification.rs`; orchestrator
`v9_real_consumer_shadow_wb14_tests.rs` and the two ingress/native-prefix test
modules; and the existing injection/counter interfaces in
`snow_stage3_v11_attachment.rs`, `snow_stage3_v11_attachment_runtime.rs`,
`v9_real_consumer_shadow.rs`, `v9_real_consumer_shadow/frozen_litter_v4_adoption.rs`
and `direct_runtime/surface_liquid_ingress.rs`. The interface/visibility and
lifecycle design is unresolved: this assessment grants no blanket edit or new
public production API. Existing feature/default/science/threshold/schema and
all predecessor runtime requirements remain binding. Cadence repair, wider B01
modeling, production and release remain HOLD.

### Evidence paths and checks

The durable reconstructed source is
`/workdir/openwepp-experiments/b01-wb14-source-reconciliation/reconstructed-available145`.
It remains an inspection/reconstruction copy, with no cadence patch applied.
The reproducible [source proposal](artifacts/revised-baseline-proposal.json),
[actual map/input inventory](artifacts/actual-byte-inventory.json),
[whole-source recipe comparison](artifacts/fresh-recipe-whole-source-comparison.json),
[static test inventory](artifacts/static-test-harness-inventory.json) and
[available-version diff](artifacts/logs/available-git-head-diff.patch) are the
primary machine evidence. The [measurement protocol](artifacts/protocol.md) preserves the frozen
scientific/hash/harness predicates, with a review correction to the execution-
method statement; its [initial text](artifacts/protocol-initial.txt) is retained.

Ran from `/workdir/openWEPP`, with scripts at the durable root above:
`.venv/bin/python <root>/audit.py`, `bash <root>/reconstruct_available_source.sh`,
`.venv/bin/python <root>/compare_reconstructed_source.py`,
`.venv/bin/python <root>/compare_whole_source.py`, and
`.venv/bin/python <root>/static_test_inventory.py`: exit 0. Raw outputs are in
[logs](artifacts/logs/). The initial audit omitted the actual-map digest and
full beyond-map input hashes; its original JSON/log is retained separately,
and the corrected rerun supplies those required claims. Script syntax/JSON and
focused result checks are assessment validation only, with zero Rust tests.
No failed Rust correction or authentic run occurred. Astra reassessed after two evidence/proposal correction rounds; no owner
allowance was renewed.

[Authority identity](artifacts/authority-identity.json) pins the governing files
at adoption. [Remote recipe checks](artifacts/recipe-remote-verification.json)
retrieved and matched eight recipe input files at the adoption commit. One `gh`
binary download failed with `transform: short source buffer`; direct raw `curl`
retrieval matched the archive. [Base metadata](artifacts/base-remote-metadata.json)
confirms the remote base commit/tree identity only. Full old-base blob retrieval
and full raw-stop-point145 LFS archive retrieval were NOT RUN. The archive is
locally hash-verified; no complete remote reconstructed-source claim is made.
The source proposal is locally recoverable from the retained base/inputs and
fresh copy. New assessment artifact publication is pending.


## Independent execution review and corrections

Assigned distinct `/root/reconciliation_correctness` (Sol/high) and
`/root/reconciliation_qa` (Terra/medium), neither the current nor predecessor
recovery author, to the integrated assessment and primary evidence. Reviews
were read-only; same reviewers verified accepted fixes. Both final verdicts
accept the negative assessment as recorded below.

QA identified a medium evidence defect: the test-inventory regex used a word
boundary after `!`, omitting assertion macros while the narrative claimed their
forms were inventoried. Astra corrected it to a call lookahead and regenerated
the inventory; all `assert!`, `assert_eq!`, `assert_ne!`, and `expect_err` forms
are now captured. The script also labels registration/fixture/harness findings
as manual static inspection and hashes their twelve primary source files.
Original pre-fix script/JSON are retained with `.before-qa-fix` suffixes at the
durable root. Same-reviewer verification passed. Implementer `stable-cut`
and `final-artifacts` hash logs describe the earlier implementation return,
not the terminal post-review artifact identity; the original return is retained
as `implementation-return.txt` at the durable root and integrated here.


QA final bounded verdict from `/root/reconciliation_qa`: **PASS**, with the
medium extraction finding corrected and independently verified. Their
[attributable final response](artifacts/qa-review.txt) records independent
927-map and full-source rehashes, source-copy identity checks and static
registration/feature/fixture/harness verification. Publication was still pending
at that review; full remote historical source retrieval remains NOT RUN.


Correctness review identified two blocking gaps in this assessment cut: the
available-version diff was mislabeled as oldest-to-HEAD although it contains
only the newest adjacent transition, and the proposed future harness did not
yet bind the complete source/feature/process/API/rollback/operand scope. The
factual label is corrected; the same implementer is completing the full
available-version trace and concrete harness amendment. Correctness acceptance remained pending at that cut; final negative-assessment
acceptance below resolves its disposition. These are evidence/proposal
corrections, with no Rust edits.


Correctness also found incomplete explicit classification of build/lint inputs
outside the 927-map and an overbroad “no patch application” statement. The
collector now identifies `.config/nextest.toml`, `clippy.toml`, `flake.nix` and
`flake.lock` with hashes; `rust-toolchain.toml` was already in the original map.
The initial no-patch statement is preserved with the initial protocol, and
corrected to “no cadence candidate patch application”. Fresh recipe replay was
already authorized and was directed before its comparison ran; the frozen hash,
semantic-unknown and harness-sufficiency predicates were not changed. Original
source bytes stayed immutable. The same correctness reviewer verified these
fixes before accepting the negative assessment.


Internal reassessment: continue within the adopted assessment. The fresh recipe
and full-source equality are independently verified, while the remaining gaps
are bounded evidence wording, available-version trace and exact runner-harness
specification. The supported next action is to bind the existing runner include
and real ingress/receipt APIs with existing dependency features, correct the
counter/precedence wording against canonical guards, and have the same reviewers
verify those changes. No new scientific method, production edit, authentic run,
source adoption or owner budget is authorized by this decision.


QA affected-scope continuation: **PASS** after verifying the corrected ignored-
runner wording, complete 13/5 available-version trace, lossless compressed raw
diffs, expanded relevant input classification, initial/corrected protocol and
historical fixture identity. QA accepts the evidence-backed negative HOLD
assessment and its explicit publication limits. Their original and continuation
responses are retained in [qa-review.txt](artifacts/qa-review.txt).

Raw unified diff context contains required spaces. The first staged whitespace
check therefore returned exit 2, preserved in
[staged-whitespace-first-check.json](artifacts/staged-whitespace-first-check.json).
The original patches remain durable; the repository retains deterministic gzip
wrappers with both compressed and original raw hashes. All 18 decompressed
patches were independently verified against Git. Full terminal whitespace checks
then passed without path exclusions or changed raw bytes. This packaging change
has no source or test semantics.


### Final review and assessment disposition

Static: `/root/reconciliation_correctness` verified the accepted evidence and
scope corrections, including the retained-source citation fix, and found no
blocking findings against **HOLD**. Their [final attributable response](artifacts/correctness-review.txt)
approves publication of the negative assessment, explicitly excluding source,
runtime, cadence or test-scope adoption. `/root/reconciliation_qa` independently
accepts the same bounded outcome; no additional verifier wave is required.
The incomplete runner bridge is an assessed high-risk reason for HOLD, not a
waived runtime requirement or an unperformed assessment checkpoint.

Assessment acceptance: **PASS — completed negative source/test decision**.
Exact working145 recovery: **FAIL (925/927)**. Fresh retained-recipe source
reproduction: **PASS (36,057 regular files and 1,318 symlinks)**. Historical
missing-test semantics: **UNKNOWN, blocks equivalence claims**. Test scope and
source adoption: **HOLD**. Rust builds/tests, authentic runs and all cadence
runtime gates: **NOT RUN here**. Numerical calibration/identifiability remains
NOT_APPLICABLE. The original source, candidate cadence patch and predecessor
manifest/logs were not modified; no cleanup or branch change occurred.

Terminal diff is confined to this record, its raw evidence/scripts/source copies
and the locator/catalog row. No primary Rust, contract, fixture, manifest,
feature, required-case, dependency or toolchain bytes changed. Source-level
anti-evasion and Rust suites are not triggered by this assessment-only diff;
no suite/obligation binding was edited. The source/test proposal was evaluated
and withheld, not applied.

Ran `.venv/bin/python docs/work-packages/20260911-b01-wb14-source-reconciliation-001/artifacts/validate_assessment.py`
from `/workdir/openWEPP`: JSON parsing, Python syntax, shell/proposed-command
syntax only, owned local links and full staged/unstaged whitespace checks PASS;
exact argv/results are in [executor-checks.json](artifacts/executor-checks.json).
The raw outer-owner excerpt is also preserved losslessly in
[outer-rollback-seam.log.gz](artifacts/logs/outer-rollback-seam.log.gz), with
[identity](artifacts/outer-seam-log-identity.json), after its intentional terminal
blank line flagged the text whitespace checker. Its durable original remains.
The [historical fixture remote check](artifacts/historical-fixture-remote-verification.json)
retrieved the exact frozen input bytes at the adoption commit; no fixture run
is implied. Publication of the new assessment evidence is pending below.
