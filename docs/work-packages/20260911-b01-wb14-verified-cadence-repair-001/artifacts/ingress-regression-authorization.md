# B01-WB14-INGRESS-RED-REGRESSION

DRAFT FOR OWNER ADOPTION. No execution or source adoption is authorized until the owner adopts this document.

## One active checkpoint

**Owning record:** `docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/package.md`. Add one explicitly bounded test-only checkpoint here; do not create another work package or reopen the completed witness package.

**Deliverable:** a retained, compiled Rust regression that enters the existing parent-bound surface-liquid ingress path with the captured day-4 request and demonstrates the exact baseline cadence refusal. Establish the fixture's source-defined parent/context preconditions separately. The test must express the intended admissible-parent behavior so it can later be used unchanged for a red/green correction comparison. Production cadence repair is NOT authorized in this checkpoint.

**Evidence/governance revision:** `e49266c312d44cf3390e1728bd68bc8399328b35` in `rogerlew/openWEPP`. The completed `20260912-b01-wb14-available-source-red-witness-001` package, its final reviews and source-bound current packet are accepted predecessor evidence within their stated scopes.

**Actual executable-source baseline:** unchanged `reconciled-available145-r1`, retained at `/workdir/openwepp-experiments/b01-wb14-source-reconciliation/reconstructed-available145`. Its 927-entry actual-map SHA-256 is `c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e`. Its separate whole-source regular-file digest is `8b29e888b96c223f46d8c2432c49560bbf68c15871491cfdaa68b1221cd373c6`; symlink-target digest is `6c50ef172d22672ba98f0b66f3a45da1849b0c47409f771affd079430196b58f`. Reuse accepted reconstruction evidence and verify actual inputs touched/consumed by this increment. These inventories are not interchangeable.

**Explicit source-use decision on adoption:** permit that available source in a separate diagnostic test copy for THIS checkpoint, notwithstanding the older cadence package's exact-historical-working145 admission requirement. This is NOT a claim that working145 was recovered, that the two substituted tests are semantically equivalent, or that a qualification/correction baseline has been adopted. The older exact-source recovery failure remains historical fact. No new archive/Git recovery search is requested.

**Permitted changes:** new regression tests and fixture-loading helpers in the existing orchestrator crate-test surfaces, minimal immutable test fixture material extracted from the accepted current capture, the owning record, necessary evidence and locator status. Preserve all existing assertions and the available 18 precedence/four raw-hash tests. Production Rust functions, the witness checker/tests, accepted packet, collector, runtime inputs, scientific contracts, features/defaults/cfg gating, manifests, lockfile, toolchain and numerical settings are read-only. Minimal registration of a new test inside an already existing test module is allowed; changing production visibility or feature gates is not.

**Acceptance/checks:** actual compiled test selection; authentic fixture lineage and source-defined setup/context checks; real-entrypoint expected-red with exact error and location; appropriate existing local ordinary/non-prefix control; complete command/source/fixture identities and raw results; independent correctness and QA of the NEW regression scope; normal applicable test-only/document checks. A build failure, zero selected tests, setup panic or unrelated refusal cannot satisfy expected-red.

**Run allowance:** builds, compiled listings, and focused Rust unit/regression tests for the declared fixture and local controls are allowed. They may execute the bounded real ingress/kernel path under test. ZERO new runner/collector invocations, full-tail/warm/performance simulations, saved-checkpoint operations or baseline/candidate authentic comparisons. The witness's original 1/1 runtime allowance remains exhausted. No replay of days 0–4 to regenerate the packet.

**Proposed hard allowance on adoption:** 60 active minutes or two unsuccessful corrective-edit/verification cycles, whichever comes first, for this new test-only checkpoint. Keep the completed witness ledger separate: at the final published timestamp it records 99m02s cumulative active time, eight unsuccessful cycles against ceiling nine, and 1/1 authentic invocations. Any already-recorded later custody time remains recorded; do not invent it. No historical allowance is reset, transferred or borrowed. This test-only checkpoint is not a resumption of an unfinished witness correction.

**Stops:** exhausted allowance; missing/corrupt necessary current source or capture; unexplained input drift; a required production/authority/visibility change; an unanticipated setup/context/consumer failure beyond the captured cadence refusal; or inability to establish the fixture's required parent context. Preserve and return the specific result. Do not fix the new downstream defect or expand into another checkpoint. Required independent review must be complete for acceptance; inability to obtain it leaves acceptance incomplete.

## Accepted evidence stays closed

Do not rerun the 75 Python controls, re-export the packet, rehash the entire multi-GB corpus, rebuild the frozen witness executable, or repeat accepted witness reviews without an affected-source/evidence change. B01-CONT-011 and B01-CONT-012 are resolved on their accepted checker cut.

Current packet:
`docs/work-packages/20260912-b01-wb14-available-source-red-witness-001/artifacts/current-run-boundary-packet.json`, SHA-256 `d8f1fe07d73ac09a6eb002b1a49a714cad7a0af1bb58ffab7612282ea002cc76`.

Current observation SHA-256:
`7a944817acf43be2ba018f670efbd3490a0cf1f3629b0011ae26449de2332ba9`.
Frozen witness executable SHA-256:
`958e61764d373a2c183764e7f7ff41607a32ef74a2af5cd7b3f299bf1123906f`.
These remain immutable reference evidence, not a test binary for the new Rust test.

The witness establishes the captured refusal and relevant supplied relationships. It does not establish independent complete-prefix authentication or prove that reordering cadence validation alone is a correct repair. The earlier proposed Rust cadence patch remains unapplied reference material.

## Regression boundary and fixture construction

Read applicable instructions for the actual write paths and affected canonical sections: SC-SURFACELIQUID-001 INV-012/013/014/026/035, its ordered guards and OBL-C-025; SC-COUPLEDTIME-001 INV-031/OBL-014; SC-SNOWENERGY-001 INV-083/087 and OBL-C-055. Keep their obligations intact. This checkpoint adds no science amendment.

Use the existing crate-private `execute_surface_liquid_ingress_with_parent_state_and_coupled_binding` entrypoint, not a reimplemented Python predicate, a direct call to `validate_cadence` alone, or a new public runner facade. Known local test surfaces are `surface_liquid_ingress_context_tests.rs` and `surface_liquid_wb14_native_prefix_tests.rs`; inspect their registration and actual constructors before choosing the smallest test addition.

Derive immutable ingress input, beginning/working state, supplied parent and coupled binding from the CURRENT packet's typed bytes. Obtain any additionally required configuration/constructor input from retained current-run material or an existing source-defined fixture only with an explicit identity/compatibility join. Do not fabricate configuration hashes, remint a proof, reset a cursor, edit an ordinal or install a convenient fabricated clock to make setup pass. Preserve original bytes; label decoded/extracted views and any deliberately synthetic control separately.

Captured target:

- day 4, interval 22, transaction sequence 255 in the full u128 domain;
- parent support `[385200000000000, 387000000000000)` ns;
- inactive prefix ending at `385920000000000` ns;
- attempted child `[385920000000000, 385980000000000)` ns;
- nonfinal parent-child posture, physical ordinal zero and no prior WB14 physical receipts;
- the actual beginning cumulative bits and source-defined distinct owner identity groups.

Before claiming the regression represents wrongful rejection of an admissible parent, exercise the existing nested/context validators separately with their actual required inputs. In particular, inspect `DirectWb14ParentWorkingState::validate_nested` and source-defined configuration, parameter, lane, support and owner joins. Do not silently bypass a failing validator because the offline witness passed. Distinguish internal nested validation from independent authentication against the complete accepted coupled history; a successful check is evidence only for what it actually checks. If required context is missing or invalid, return that concrete finding instead of repairing/synthesizing it or claiming a valid-parent regression.

The regression should assert the contract-required admission of the first physical child after the valid inactive prefix. On the unchanged baseline it must report the precise known E008 continuation mismatch at the real ingress entrypoint. Retain the failed assertion and typed error as EXPECTED_RED; do not invert the assertion merely to obtain a green test. An ordinary existing local control should confirm that the setup is capable of entering the intended ingress path.

A planned, correctly observed expected-red is a successful baseline experiment, not an unsuccessful correction cycle. An unintended compilation, setup, selection or verification failure is not expected-red. Corrective edits followed by failing verification count under the governing cycle definition; retain each failed cut/result before another edit.

## Isolation and commands

Inspect destinations before use. Create one new durable test copy under `/workdir/openwepp-experiments/b01-wb14-cadence/` without overwriting any unique source or prior candidate. Keep `reconstructed-available145` immutable. Use a separate disposable build target and retain test changes as a patch relative to the exact available-source recipe, plus any new fixture bytes. Do not duplicate or archive the entire tree for every edit.

Use the retained Nix environment and the actual test copy's explicit manifest/cwd. Owning-crate feature selection is `persisted-restart-v1,restart-authority-evidence`, not defaults alone or `--all-features`. Retain `RUST_MIN_STACK=67108864` and actual build settings. Discover and record the real fully qualified test names from a compiled listing; do not guess a passing selection from a source symbol.

Prefer canonical Nextest selection for ordinary focused checks. A fresh-process libtest invocation for an isolated global B01-policy test is permitted where needed. Freeze the selected regression and local-control commands before execution, require nonzero exact selection, retain individual outcomes and full logs, and distinguish build, listing and test results. No hidden collector invocation is permitted inside the new test.

## Review, disposition and next decision

Astra/medium orchestrates; Terra/medium owns detailed test work. Use one writer and no nested spawning. Assign bounded work with the remaining allowance; an unsuccessful verification returns to Astra before another corrective edit. Reuse independent reviewers when accessible. On owner adoption, one explicitly identified replacement per inaccessible required scope is permitted, preserving prior findings and never claiming same-conversation continuity. Do not repeatedly spawn on capacity errors; sequence supported slots. Correctness and QA remain distinct from each other and the author/adviser.

Review only new fixture/test semantics, real-entrypoint reachability, setup validity, exact expected-red and evidence. Existing witness approval is not approval of this new regression, and a new regression review must not reopen unchanged witness claims.

Report separately: test delivery/expected-red result, context-validation result, and every unmet wider cadence obligation. This checkpoint cannot satisfy B01-CONT-005's full first/subsequent/final-child repair, 18-child progression, complete rollback, independent transfer accounting, actual runner coverage or split restart. All older cadence-package acceptance rows remain required and unfulfilled; none is deleted or waived. The broader cadence package remains HOLD even if this test-only checkpoint is completed.

Carry scoped evidence/test-patch commit-and-push permission, preserving the current branch and unrelated work. Publish the small new test/fixture patch and results with actual remote verification; the full raw corpus remains local unless separately authorized. Do not claim remote source recoverability from a hash alone.

Return the executable regression, expected-red transcript, actual source/fixture/command identities, validator results, both scoped reviews and consumed allowance. The next owner decision is whether that evidence supports the separately bounded production correction. Do not begin it automatically.
