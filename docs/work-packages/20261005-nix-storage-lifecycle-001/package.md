# Nix development source boundary and storage lifecycle

Status: IMPLEMENTED / CLOSURE HOLD. Storage remediation and recurring retention are operational; mandatory full-workspace correctness has failures. Owner authorized scaffolding and execution to completion on 2026-10-05.

## Objective and scope
Stop repository-sized Nix snapshots during ordinary development; preserve exact current toolchain and experiment custody; implement explicit environment retention and safe periodic cleanup. Reclaim historical Nix source snapshots only after verified durable recovery or proof of redundancy. No source/evidence deletion, numerical changes, toolchain upgrades, branch switching, or pushing. Preserve pre-existing dirty/untracked work.

Base: `2d1d6b0a4973f2fa76226dc3e3d6022ab3309c8d`; initial dirty inventory: [initial-status](artifacts/initial-status.txt).

Measured starting root usage: 438 GiB used, 4.5 GiB available after owner-authorized incremental cleanup. Nix store ~251 GiB, eligible unrooted paths ~249 GiB; this is eligibility, not disposal authorization. A sampled openWEPP snapshot is ~3.14 GiB. Runtime target: no repository-sized source import for environment entry; staged toolchain source below 1 MiB, unchanged identity for Rust/evidence changes. Existing numerical runtime targets and evidence remain unaffected.

## Intent and acceptance before edits
- Canonical small toolchain flake/lock/helper input, entered through tools/dev wrapper independent of source/evidence changes; preserve lock and tool versions, flags, CWD/target ownership and command exit behavior.
- Persistent environment GC roots and explicit package pins; release lifecycle, bounded cache retention, periodic safe cleanup without silently deleting evidence or arbitrary unrooted historical tools.
- Historical snapshot inventory and verified recovery before scoped Nix-managed removal; keep exact required toolchains and all unrelated data.
- Documentation/governance updated at owning guides, package locator/catalog updated.
- Execute focused positive/negative wrapper and lifecycle tests, shell/Python/Nix checks, real environment entry/tool comparison and source-size/reuse checks, root survival and safe deletion checks. No scientific experiment authorized.
- Two independent correctness and QA/evidence reviews, same-reviewer accepted-fix verification.

## Execution and validation bounds
Developer-infrastructure implementation with consequential custody/governance changes: dual review. Toolchain resolution, Cargo lock/manifests, compiler flags, kernel execution and validation-runner semantics must remain unchanged; prove equivalence rather than upgrade. If impact reaches a critical production boundary under testing-and-gate-strategy section 8, escalate applicable checks before disposition. Initial selection was focused infrastructure checks. Independent correctness review escalated custody/GC-root/governance changes under section 8 to Critical: full-workspace `cargo nextest run --workspace --profile full` is now required at increment closure, in addition to focused infrastructure checks. No waiver or retrospective deferral. No campaign or release qualification claim.

Internal checkpoints: implementation and focused checks; custody/integration and host activation; independent review and terminal reconciliation. Reassess after two failed correction cycles or 60 active minutes; no owner time/token cap supplied. No global garbage collection until needed roots are protected. No arbitrary removal by directory name.

Assignments: Astra owns integration, governance, host transition/custody and package record. Implementer owns tools/dev and flake implementation. Independent reviewers assigned after executable cut.

## Integration and custody progress

Ran inventory: 183 unrooted openWEPP source snapshots, 239.891 GiB allocated (summed per-path; actual reclamation measured separately). [Inventory](artifacts/legacy-snapshot-inventory.json). Preserve every snapshot as a compressed canonical NAR under `/workdir/openwepp-recovery/nix-snapshots-20261005`; [archive script](artifacts/archive_snapshots.py) restores each archive and compares the restored NAR SHA256 to Nix's registered original. This script does not delete Nix paths. Deletion is a separate, independently reviewed, exact-path Nix-managed action after recovery verification. Archives are durable recovery evidence, not an expiring cache. Symlinks remain symlinks; their existing external targets are not deleted by this migration.

Registered five persistent roots for the latest cold-canopy retained tool receipt: [pins and release condition](artifacts/legacy-toolchain-pins.json). The current main flake resolves different compiler versions from that frozen experiment; preserve both without upgrading either. No other legacy tool output is a cleanup target.

Original environment captured from `/nix/store/jpa2vlydn9q0ww27a6qs7675rf95i7m2-source`, whose flake, lock and three helpers byte-match the pre-edit checkout. `nix print-dev-env --option eval-cache false path:<snapshot> --json` PASS; [environment](artifacts/original-environment.json). First attempted snapshot had an external tools symlink and pure evaluation refused it; a direct store argument also failed to select a derivation. Correct explicit `path:` and self-contained snapshot resolve this infrastructure issue; no toolchain changes or impure fallback.

Protected pre-existing dirty Rust test files: [hashes](artifacts/protected-dirty-source.json). No Rust test edits belong to this package.

## Independent review (in progress)

Correctness reviewer: `/root/correctness_review` (repository-pinned independent correctness role). Initial custody review required complete one-to-one inventory/receipt binding, archive path/size/hash and original-NAR revalidation, live exact GC-root registration, literal Nix-managed deletion with liveness enforced, and post-delete restoration samples. Implemented in [removal script](artifacts/remove_verified_snapshots.py), which defaults to validation only and has not yet deleted anything. Reviewer found missing inventory-uniqueness and substring root matching; corrected to unique typed rows and exact root/target pairs. Same reviewer verified both fixes PASS, plus atomic per-path outcome receipt replacement. Full-workspace critical regression requirement accepted and added above.

Reviewer accepted the bounded symlink claim: [symlink inventory](artifacts/legacy-snapshot-symlinks.json) found 30,325 symlinks, no dangling links, and 32 distinct absolute external targets. Exact NAR recovery preserves symlink objects, not historical copies of their external referent bytes. External targets remain untouched; restored functional traversal can reflect subsequent target changes. No fully self-contained-checkout recovery claim is made.

Remaining independent review: final tools/governance correctness and complementary QA/evidence, including host activation and terminal diff.

Integration found an executable-mode mismatch in initial staged helper installation: helper bytes were identical but original Nix helper was executable and candidate was not, changing the shell derivation and its generated random-seed/rpath. Implementer is preserving canonical mode and bumping stage identity; terminal equivalence is required. The first full-profile run therefore remains diagnostic evidence only, not terminal closure. It also exposed existing assurance `SC-SNOWENERGY-001.md` identity mismatches, V10 binding drift and insufficient-stack aborts; no filters or authority receipts were changed. Final full-profile run will use established test stack allowance `RUST_MIN_STACK=67108864` on the corrected environment; required correctness failures remain blocking if present.

Correctness review also identified active/prune TOCTOU, exclusive toolchain lock preventing separate worktrees, uncoordinated pin creation, old README entry commands, and duplicated helper logic. These are being corrected by the same implementer and require same-reviewer final verification.

[Removal refusal controls](artifacts/removal-negative-controls.log): 8 PASS, covering duplicate/missing inventory receipts, unverified receipt, wrong archive path/size/hash, missing toolchain root and source outside store; no deletion invoked. First test-harness attempt incorrectly mocked read-only subprocess calls as forbidden; corrected the harness guard to reject only deletion and retained [initial harness failure](artifacts/removal-negative-controls-harness-failure.log). [Registered referrers](artifacts/legacy-snapshot-referrers.json): none for all 183 inventoried source snapshots at inspection time.

Internal reassessment after iterative reviewer corrections: continue within the authorized infrastructure envelope. Staging mode and identity now reproduce the entire original Nix environment JSON exactly ([equivalence](artifacts/environment-equivalence.json)); shared active locks and serialized retirement/pin mutation have focused passing evidence. Remaining concrete work is validation-before-mkdir for symlink ancestors and provenance-bound store-output retirement. These are bounded fixes with identified mechanisms, not repeated unchanged attempts. No scientific methods, source authority receipts, test filters or owner allowances changed.

Required complementary QA reviewer dispatch failed once with `agent thread limit reached` after the implementer completed. The correctness reviewer remains available; author/parent will not substitute for the missing independent QA reviewer, and unchanged-capacity retries are not used. Overall package closure must remain pending that required independence. Continue authorized implementation, real workflow checks and independently bounded recovery-backed cleanup; keep their outcomes separate from package closure.

## Executed storage outcome

Archive run COMPLETE: all 183 inventories restored individually and matched their original registered NAR SHA256. Compressed archives total 54,765,810,380 bytes (~51.0 GiB), durable at `/workdir/openwepp-recovery/nix-snapshots-20261005`. [Per-snapshot recovery receipts](artifacts/legacy-snapshot-recovery.json), [archive log](artifacts/archive-snapshots.log).

Removal validation PASS: [preflight](artifacts/removal-preflight.log) rehashed all archives, checked exact inventory/receipt sets, original source identities, and retained tool roots. Correctness reviewer approved this bounded recovery-backed action independently of overall package closure. Ran `.venv/bin/python artifacts/remove_verified_snapshots.py --apply` from repository root using the full package artifact path: 183/183 exact `nix-store --delete <inventoried source>` operations exit 0. No global GC or ignore-liveness. [Apply log](artifacts/removal-apply.log), [per-path outcomes](artifacts/legacy-snapshot-removal.json), [space measurement](artifacts/legacy-snapshot-space.json).

Recovered 239.899 GiB on `/`; root now 198 GiB used / 245 GiB available / 45% used. Nix store ~11 GiB. Actual usage can vary with concurrent test output.

Post-removal verification PASS: restored smallest/median/largest compressed archive cohorts, matched each NAR hash, confirmed all 183 original paths absent, and checked five cold-canopy tool roots plus executable SHA256 values unchanged. [Results](artifacts/post-cleanup-verification.json), [log](artifacts/post-cleanup-verification.log). To recover source, use `zstd -d -c <archive.nar.zst> | nix-store --restore <new-external-directory>` and compare its `nix-store --dump` SHA256 to the receipt; this restores exact NAR contents/symlink text, not historical external referent bytes or store registration. Preserve archives/receipts; no automatic expiry is assigned.

Real environment integration PASS: [two-shell/prune execution](artifacts/real-environment-concurrency.json) showed two distinct task shells concurrently sharing one staged identity, retained stage with two and then one live shell, and retired its isolated local profile/source only after both exited. Nix refused deleting the shared environment output while host profiles still rooted it. Source payload <1 MiB. The first harness counted a hidden lock file as a second stage; corrected directory-only counting and retained [initial diagnostic](artifacts/real-environment-concurrency-harness-failure.json).

## Terminal implementation and host activation

Ran: canonical environment inputs total 5,331 bytes; the original lock bytes and parsed complete development environment match. `tools/dev/develop` stages only the flake, lock and canonical helper outside Git; Rust/evidence edits do not alter that identity. Root flake files were replaced by `tools/dev/nix-toolchain/`. Shared active-use locks, explicit named pins and guarded retirement retain active or required environments. No numerical source, compiler selection, Cargo flags or validation filters changed.

The initial QA dispatch capacity issue above was resolved after the correctness reviewer completed its earlier turn. Independent `/root/qa_final_review` then reviewed the implementation and requested real pin/release evidence. That integration exposed Nix generation roots left behind when only a profile alias was removed. The corrected lifecycle prevalidates every numeric generation against the expected output before removal and uses `nix build --no-link` for pins. [Real final integration](artifacts/real-pin-release-final.json) PASS: release and prune remove owned generation roots, preserve nonnumeric siblings, refuse tampered pin generations intact, and skip ambiguous retirement candidates intact. The initial failing evidence is retained. Only the test-created checkout `result` link and isolated temporary fixtures were removed.

Focused lifecycle tests PASS, including refusal paths; [final log](artifacts/final-lifecycle-tests.log). The subsequent host ShellCheck invocation lacked the command; rerunning through the canonical environment PASS ([ShellCheck](artifacts/final-shellcheck.log)). Earlier Nix formatting and flake checks PASS, with actual cached check build output retained in `artifacts/nix-dev-tools-build.log`.

Installed and enabled the weekly persistent user timer `openwepp-nix-prune.timer`, keeping the two newest environments and retiring unpinned inactive environments older than 30 days. Configuration points to `/workdir/openWEPP/tools/dev/nix-lifecycle`. Initial activation and manual service execution PASS ([activation](artifacts/host-retention-activation.json)); manual execution after the generation-root fix also PASS ([final service](artifacts/host-retention-final-service.json)). User lingering is disabled: the user timer operates with the user manager/session and does not promise cleanup while logged out. Durable recovery archives and frozen experiment tool roots are outside its cleanup scope.

Terminal scope: tools/dev entry/lifecycle/tests/service templates and small manifest; root flake removal; repository, package and bounded-execution guidance; this package's evidence and locator/catalog. Original dirty Rust files retain their exact hashes and every original untracked path remains present ([protected work](artifacts/protected-work-check.json)). No branch switch, commit or push. The durable frozen-tool pins remain owned by the cold-canopy recovery obligation, with their release conditions in `artifacts/legacy-toolchain-pins.json`; recovery archives have no automatic expiry.

## Final regression and disposition

Ran: required full-workspace profile completed with exit 100 in 1,494.022 seconds: **4,204 tests run; 4,013 passed, 190 failed, 1 timed out; 76 skipped by the existing configuration**. [Result](artifacts/final-full-workspace-result.json), [complete log](artifacts/final-full-workspace-nextest.log). Failure classes include SC-SNOWENERGY source identity/assurance bindings, V10/V11 receipt bindings, E008 expectations and typed Stage-3 runtime guards. The runner Lane-D summary test timed out at its configured 720 seconds. These affected Rust/science surfaces were not edited by this package; a clean-baseline regression was not run, so this record does not establish that every failure predates the package. Exact environment equivalence and protected source hashes are established separately. No authority receipt, scientific code or test exclusion was changed to make the gate pass.

Storage remediation and its bounded recurring lifecycle are operational. Overall package remains **IMPLEMENTED / CLOSURE HOLD**, because mandatory full correctness failed; no COMPLETE or full-workspace green claim. Further closure requires disposition/correction of those failures by their owning scope and a passing required regression. Infrastructure-specific checks and independent review do not waive that requirement.

Recoverable scoped implementation, patch and evidence are also retained locally at `/workdir/openwepp-recovery/nix-storage-implementation-20261005` with a SHA256 file manifest. This is local custody, not remote publication. `artifacts/terminal-scoped-diff.patch` captures tracked implementation/governance changes; the recovery tar includes new tooling and this package's artifacts.

Independent terminal reviews: [correctness](artifacts/correctness-final-review.md) approves the scoped lifecycle implementation and same-reviewer fixes; [QA](artifacts/qa-final-review.md) finds no remaining in-scope QA defect. Both retain required-gate FAIL / package HOLD.

## Owner-authorized publication

On 2026-10-05 the owner directed commit and push with the package on HOLD. Publish the scoped infrastructure, governance and retained package evidence on existing `main`; preserve unrelated dirty/untracked work. This publication does not waive the failed correctness gate or change the disposition. Before this commit, local `main` was 41 commits ahead of `origin/main` with no remote divergence; pushing the existing branch also publishes that prior history. Large snapshot recovery archives remain in the documented local recovery directory.
