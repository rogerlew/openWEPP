# B01 WB14: finish feature compatibility under a prospective fixed allowance

DRAFT FOR ROGER'S ADOPTION. This proposal grants no authority until adopted.

## One active checkpoint, exact source, changes, acceptance, runs and stops

**Checkpoint:** `B01-WB14-CURRENT-CONTEXT-CAPTURE / A-001`, feature-build portion of `WB14-RQ1`, maintained in `docs/work-packages/20260911-b01-wb14-verified-cadence-repair-001/package.md`. Complete the existing compatibility repair: standalone library and required feature-specific test builds, preserved test membership, meaningful affected execution, and distinct independent reviews. Do not restart the accepted local promotion investigation or expand into general lint cleanup.

**Evidence revision:** `65d82e2fa6396deb76be0aff8eff06e50a104389` in `rogerlew/openWEPP`, especially `artifacts/feature-compatibility-20260917/`. Repository HEAD is not the executable experiment source.

**Starting feature source F:** `/home/roger/openwepp-experiments/b01-wb14-feature-compatibility-20260917`, tree SHA-256 `68600a01f8a90f2ef036fd966cd26dceb26c936a4efad3870dd3aa1096d1e12b`. Its selected two-feature test build passed; its no-default test check failed with 26 errors. No executed terminal-source behavioral tests or terminal approvals are established. The evidence-only PASS belongs to intermediate tree `8c88b0b16a7ec4d17e903bfb5b5b9a1adf7a3efb7db42e9207cb1b6220945e23`, not F.

**Recovery and preservation:** `stopped-from-frozen-integrated.patch`, SHA-256 `48a54e35dd7ddb4c77fa93d64e4e7c442bb2c07716bcc26b106a416bdb2b1abd`, is against frozen integrated source `e93175d626081d52219c5053ff7899521f7ce3010a720335bec8b462095ee1e7`, not T/U or an already-patched feature source. The stopped audit supplies F's 746-entry map and seven changed paths. Verify actual F and this recipe before further mutation; do not claim successful reconstruction from a manifest alone. Preserve the integrated baseline, its frozen binary `3ee09e2d800d145494dd38280e26a89fb95447284120112041bfef3e00b65169`, all 14-case evidence, T/U/v20, original inputs and failed feature cuts. Use the existing working copy, recovery tools and build cache; no new corpus or recovery framework.

**Permitted changes:** inherited and introduced feature-availability defects at the owning private diagnostic, test, import/helper and plain-state representation boundaries; targeted standalone-library diagnostic wiring; affected tests, inventory and recording tools. This explicitly includes the discovered inherited standalone-library configuration failure, only within these compatibility boundaries. Retain production APIs/wires, feature defaults and graph, dependencies/toolchain, capability traits, scientific semantics, physical inputs and runtime guards. A new public feature surface, changed admission contract, no-op recorder, disabled ordinary test or numerical workaround is not authorized. Broad inherited Clippy cleanup and unrelated refactoring remain excluded.

**Acceptance:** terminal-source PASS for both originally failing test-compilation commands and a separately selected standalone library check; preserved selected full-feature test build; source-justified membership in each supported mode; all 14 promotion selectors and required restoration/capability tests retained in their actual supported modes; executed ordinary recorder and affected representation/refusal controls; targeted formatting and attributable quality checks; distinct correctness and QA approvals. An intermediate PASS, a check, or a listing is not terminal execution. Preserve every remaining RQ1/A-001, strict-quality and scientific qualification gap separately.

**Run allowance:** bounded Nix builds/checks, exact listings, source/feature inspection, focused compatibility and representation tests, and justified affected local verification under existing scope. Keep automatic retries off. Existing small local physical controls may run only when the feature correction actually affects them; do not repeat the complete scientific matrix or corpus by default. No original-input model, captured day-4 successor, collector/acquisition, multi-day replay, original-regression repair, full coupled campaign, canonical promotion, production Rust adoption or cadence repair. Historical acquisition/witness allowances remain separately exhausted at 1/1.

**Prospective budget:** grant at most **14,400 additional charged seconds**. Carry **113,900.205 seconds**, including the entire historical overrun and publication/return reserve. Fixed new cumulative ceiling: **128,300.205 seconds**. Carry failure floor **at least 268**, preserving uncertainty. This is not retrospective approval of the overrun and does not restore the old 106,868.925-second ceiling or reclaim its former balance. Use the published `final-ledger.json` and any later actual preservation charge; greater established consumption reduces the remaining supplement. Concurrent wall time counts once for reading, edits, commands, coordination, reviews, preservation/publication and return; no new wait deductions or retrospective deductions for the previous clock gap. Anchor this new allowance only at actual owner-adopted continuation start.

**Stops/reassessment:** hard time ceiling, owner stop, uncontained integrity issue, indispensable unavailable input after bounded recovery, completed bounded disposition, or a necessary excluded change after useful in-scope work is exhausted. Two unsuccessful corrections or 60 charged minutes are internal Astra reassessment points, not owner handoffs or a new failure-count ceiling. Preserve and classify new failures; supported in-scope corrections stay with Astra. A genuinely out-of-domain failure returns to the owner without being tuned away.

## Preserve the clock evidence and enforce the next boundary

The previous final ledger exceeded its cap by 7,031.280 seconds. Its retained validation commands end before the old deadline; the ledger conservatively includes a later continuation gap. Neither infer continuous unauthorized engineering from that gap nor deduct it retroactively. Preserve the audit as recorded.

At adoption, put the new anchor, fixed deadline, cumulative charge and remaining balance in the existing ledger. The orchestrator checks them before dispatch or resumption after any interruption, not only when writing the final report. Give each worker the same deadline. Use the existing command recorder to refuse launches after expiry and cap subprocess timeouts by the remaining execution balance after a preservation reserve. Verify its expired-deadline refusal with a harmless local control. A small fix to that recorder is permitted; do not create a scheduler, gate planner, automation service or second governance system.

Reserve review and preservation time inside the allowance. Before returning or suspending, record the actual charge and worker/job state; do not silently reset a running session's deadline after a turn gap. Existing job-preservation rules still apply to legitimately running bounded jobs, but do not authorize new edits, builds or worker tasks after expiry. No need for repeated owner approval of routine corrections within the live allowance.

## Repair one consistent availability model

Read the retained compiler JSON locally and trace the remaining diagnostic dependencies, rather than broadening derives one error at a time. Establish the owning build contexts: ordinary non-test library; no-feature test harness; evidence-only tests; persisted-restart-specific tests; and the selected two-feature workflow. Inspect the persisted-only mode where the changed dependency path makes it applicable. A combined multi-package build that unifies features cannot establish standalone compatibility.

Keep feature-C1 and feature-C2 open until verified on the final cut:

- **C1:** no-feature recording must remain real recording. The rejected empty `record_pre_child_context` is not an acceptable fallback. Keep ordinary recorder controls active, preserve the independent raw expected projection, and execute a row-producing/control path without the restart features. Gate only tests that demonstrably require the missing restart capability; retain and execute them in their proper mode.
- **C2:** any legitimate plain-state test serializer must retain every applicable field/variant rename, tag, omission/default and numeric representation rule. Do not enable `Serialize` while leaving `rename_all = "snake_case"` disabled. Preserve the reverted growth-type experiment as history. Prefer existing complete private projections or narrowly scoped helpers when recursive derives would expose runtime state. No Serialize/Deserialize/Clone/Copy addition to a trusted capability.

`cfg(test)` applies to the crate's test-harness compilation, not its normal library compilation. Fix the standalone library at its actual diagnostic boundary; a test-only trait addition cannot alone establish that normal build. Keep required ordinary diagnostic behavior intact rather than gating away the failing caller.

Review widening of `cfg` conditions on `pub` checkpoint methods in `snow_stage3_v11_restart.rs` as an API-availability question, not automatically as private test wiring. Establish the enclosing visibility and existing feature contract. Where ordinary API availability would be expanded beyond authorization, retain the public gate and factor only the required private helper. Do not declare an unsupported public change acceptable because evidence-only compilation succeeds.

These are acceptance constraints, not instructions to redesign serialization or rewrite all seven files. Retain valid existing corrections and use the smallest complete dependency fix.

## Verify the final source, not the most favorable intermediate cut

Use the original command receipts, explicit detached manifest, selected targets, locked dependencies and pinned Nix route. In separate invocations retain:

1. `cargo check ... -p openwepp-hillslope-orchestrator --locked --tests --no-default-features --message-format=json`.
2. The same command plus `--features restart-authority-evidence`.
3. Standalone `cargo check ... -p openwepp-hillslope-orchestrator --locked --lib`, without co-selecting the persisted-restart package. Check other affected ordinary feature modes when source analysis requires them.
4. The retained selected test build with `--no-default-features --features persisted-restart-v1,restart-authority-evidence --no-run`, without narrowing its existing targets.

Do not enable a missing feature, change defaults, or rely on a previous multi-package PASS to make a different configuration appear repaired. Record exact command and source identity for every result.

Reconcile selectors by name and prerequisites, not total counts alone. The baseline's 1,518-test full-feature listing is a starting inventory, not evidence that all tests belong in every mode. Broken configurations require source-defined expected membership. Keep all 14 promotion cases and actual ordinary recorder/refusal/capability controls. No blanket module gates, `cfg(any())`, new ignores, assertion deletion or successful zero-test selection.

Run meaningful focused controls in the configurations whose compiled behavior changed, including exact diagnostic naming/representation and ordinary recorder output. Use preserved expected bytes or the existing independent oracle, not a second call to the candidate serializer as its own expected value. Reuse unaffected accepted physical evidence only with an explicit compiled-path comparison; affected projection semantics require affected parity verification. Do not relabel old binary executions as new-source results.

Finish exact-source targeted formatting, selected lint/compatibility comparisons and source recovery. Fix introduced defects. Existing strict Clippy and broader qualified-test failures remain FAIL until their own requirements are met; no warning cap or baseline-equivalence statement is a strict-quality PASS. This task need not clear unrelated inherited workspace debt.

## Review, publication and bounded disposition

Astra owns the debugging loop. Retain one source writer, at most two concurrent children and no nested spawning. Reuse `/root/feature_correctness` and `/root/feature_qa` if actually available; otherwise use attributable independent replacements without repeated capacity retries. Review the final feature conditions, ordinary/public availability, serializer semantics, source-declared inventory and primary test results. The author/orchestrator/adviser supplies neither independent review. Preserve C1/C2 findings and obtain same-reviewer fix verification where possible.

Use `package.md` as the single maintained narrative. Carry current-branch scoped evidence/patch commit/push and exact remote verification; no branch switch, source adoption or deletion of unique evidence. Apply the current root/nearest/package, Pro-role, bounded-execution and testing instructions and relevant private-state/capability authority. Preserve non-wire, consuming-use, ordinary-admission and exact-custody invariants.

Return separate outcomes for standalone library, no-default/evidence-only/selected-feature builds, named test membership, actual focused execution, feature-C1/C2, strict quality, independent reviews and source custody. Keep LP-C2/LP-C4 and WB14-RC1/RQ2 accepted only on their reviewed local frozen cuts; RQ1/A-001, original E008/regression, scientific/conservation/restart qualification and cadence remain independently dispositioned. End this continuation at a verified feature repair or an attributable remaining blocker, not another unverified intermediate-build summary.

## Adviser evidence boundary

Inspected at the pinned revision: Pro role; current stopped disposition; terminal seven-file patch; stopped command/source audit, including all command timestamps; final ledger; raw terminal no-default stderr; selected-feature build receipt; interim correctness findings; prior supplied authorization. Root/package/bounded/testing content was reused after current blob checks. Official Rust conditional-compilation and Serde attribute documentation support the general build/representation distinctions above. Complete detached source, binaries, terminal compressed compiler JSON and final external host state were not accessed here. Direct raw gzip download failed on DNS resolution; that is not evidence of executor-side source loss. No Rust, native probe, physical workflow or independent acceptance review was executed by this adviser.
