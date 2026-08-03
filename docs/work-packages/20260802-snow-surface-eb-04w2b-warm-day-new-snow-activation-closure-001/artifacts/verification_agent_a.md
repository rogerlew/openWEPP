# Verification Agent A — Terminal-V2 Verification

Status: **PASS — terminal-v2 verification gate**

Evidence mode: **Ran + Static**

This PASS verifies the terminal-v2 correction and retained package evidence. It
does not itself advance EB-04X: the current roadmap HOLD remains truthful until
both named terminal verifiers and the package owner complete final finding,
exact-diff, prompt, and disposition reconciliation.

## Acceptance Criteria

| Criterion | Result | Independent verification |
|---|---|---|
| 1. Warm, zero-prior-SWE typed snowfall activates and conserves exact input | **PASS** | Static contract/implementation/test trace and retained focused green evidence cover the sufficient `>1e-12 m` typed-snow trigger and `snowfall_depth * 0.1` reconstruction. |
| 2. Mixed event closes; warm all-rain/no-pack remains inactive | **PASS** | Retained six-test contract target and focused shared/consumer tests cover mixed rain/snow closure, warm rain-only inactivity, exact threshold, and just-over threshold behavior. |
| 3. Independent daily SWE reconstruction fails closed | **PASS** | Shared partition and snowbench consumer independently use `SWE_before + typed_snowfall_SWE + rain_retained - snowpack_loss - sublimation - SWE_after`; retained exact/over/non-finite boundary tests pass with typed errors. |
| 4. Direct runtime and snowbench use the corrected public API | **PASS** | Real-consumer evidence reaches the production fixture, seed/frame, `DirectProductionDayInputBuilder`, shared partition, published storage, after-day SWE, and hydrology projection. No wrapper, shadow, skeleton, or compatibility-only path carries the claim. |
| 5. Protected phase/forcing/coefficient/selector/default/melt equations unchanged | **PASS** | Static exact-diff and source-manifest reconciliation finds only the contract-backed availability trigger and conservation/control work in W2B; terminal resumption changed no result-affecting W2B source. W2C is separately authorized and reviewed. |
| 6. Required validation/review/governance gates | **PASS** | Focused 11/11, owning-crate, frost, warnings-denied Clippy, formatting, assurance, documentation, and W2C quick/frost/erosion/Critical-full evidence are retained. Fresh Review Agents A and B both end in PASS with no open finding. Line-count, security, prompt, and exact-source checks pass as detailed below. |
| 7. Frozen rerun ordering and unchanged adjudication | **PASS** | W2C first released the prerequisite (`4/231` refusals); the isolated exact-source terminal-v2 run then executed 8/8 cells. Maximum mass closure is `2.220446049250313e-15 m`, maximum energy closure is `6.094342098e-08 J m^-2`, and no albedo promotion is admitted. Earlier generations remain ineligible historical evidence. |

## Ran Evidence

- Recomputed SHA-256 links independently: freeze -> receipt, freeze ->
  results, receipt -> results, and adjudication -> freeze/receipt/results all
  match. Actual hashes are freeze `943561dc...d44dbb`, receipt
  `02461508...a0ed`, and results `65b308db...d4cb`.
- Rehashed all 11 explicit result-affecting source files in `freeze.json`;
  every current file matches its frozen identity and the aggregate manifest is
  `e980170e...a4f938`.
- Rehashed the live `target/release/openwepp-snowbench`; it matches the release
  build receipt and freeze at `d6b2e824...9a54` (13,122,640 bytes). The receipt
  records the exact command, HEAD, dirty-source identity, path, size, and mtime.
- Parsed the execution receipt: exactly eight results are retained. Parsed the
  adjudication: the reported mass/energy maxima reproduce and all four frozen
  albedo-materiality decisions remain nonpromotion decisions.
- `git diff --quiet HEAD --` passed for the shared historical synthesis and all
  eight shared figure/sidecar files. Terminal-v2 has its own freeze, receipt,
  results, summary, adjudication, synthesis, four SVGs, and four sidecars.
- `git diff --check` passed on the terminal worktree.
- Current line count independently confirms `runoff_reconciliation.rs` is
  2,598 lines. The checklist also dispositions the 2,450-line day-input builder
  and 2,891-line runner test aggregate as WARN with split intent; no reviewed
  nonexempt Rust file reaches 3,000 lines.

## Static Lifecycle And Scope Checks

- EB-04W2C is unambiguously `COMPLETE / TECHNICAL_PASS / REVIEW_PASS /
  VERIFICATION_PASS`; its terminal evidence retains `4/231`, exact erosion
  mass closure, the `5e-3` diagnostic bound, quick `2156/2156`, frost
  `345/345`, erosion `377/377`, Critical full `2243/2243`, and review and
  verification PASS.
- Review findings `A-RT-001` through `A-RT-003`, `RB-01` through `RB-06`, and
  `RB-V2-01` are all accepted/corrected and closed. Neither fresh reviewer has
  a remaining finding; no finding was silently rejected, deferred, or waived.
- The combined tracked/untracked inventory is explained by W2B's declared
  package/evidence/tool/lifecycle set plus W2C's separately authorized
  production, contract, test, and package set. Result-bearing W2B identities
  have not changed since terminal-v2 generation. Review/verification artifacts
  are correctly recognized as self-referential terminal metadata.
- Historical prerequisite-ineligible JSON/CSV and shared synthesis/figures are
  preserved. The rejected terminal-v1 JSON chain is also retained without a
  closure claim. The wrapper's isolated terminal-v2 namespace and pre-existing
  destination guard prevent another overwrite.
- No manifest, lockfile, dependency-resolution, nextest-policy, `unsafe`, or
  production unwrap/expect change is present; `cargo deny check` is correctly
  not applicable. Required assurance adoption and current source locks pass.
- Prompt lifecycle is coherent: the kickoff is archived, there is no active
  prompt, and the active README says verification is in progress. The package
  contains the explicit delegation authorization used for this bounded report.
- `docs/ROADMAP.md`, the snow campaign roadmap, package catalog, package, and
  disposition keep EB-04X held. No stale `EB-04X may advance` statement remains
  in the two previously defective narratives.

## Blocking Findings And Verdict

No terminal-v2 technical, source-identity, evidence-integrity, acceptance, or
review-finding blocker remains. **PASS** for Verification Agent A.

Administrative package completion and EB-04X advancement remain **HOLD** until
Verification Agent B is freshly recorded and the owner performs the final
dual-verification disposition and exact-diff/lifecycle update. This is the
required current roadmap posture, not a technical failure of terminal-v2.
