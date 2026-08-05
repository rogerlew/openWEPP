# Terminal Verification B

Verdict: **PASS-WITH-NOTES**

Status: complete

Evidence class: **Static + Ran**

## Verification Result

The 21K implementation, authority amendment, exact-one real-consumer handoff,
offline alignment, conservation evidence, fixture custody, and required gate
matrix pass independent terminal verification. No new science, Rust,
security, anti-evasion, or regression blocker was found.

One closure-prose note remains. At this verifier's terminal freeze,
`verification_agent_a.md` still said `Status: queued` and `Evidence mode:
not-run`, while the package, disposition, catalogs, and roadmaps already
claimed dual-verification pass. Terminal Verification A completed during this
verifier's final lint pass with `PASS-WITH-NOTES`, so the lifecycle prerequisite
is now satisfied. The package's literal statement that both verifiers return
`PASS` should say that both terminal verifications passed, or otherwise
reconcile A's exact verdict. This is a wording issue, not a defect in the
verified 21K correction or a blocker to the dual-verification status.

## Authority, Chronology, and Exact-One Consumption

- Canonical `SC-SNOWFREEZE-001` v125 binds `INV-SNOWFREEZE-092`,
  `OBL-SNOWFREEZE-P-065`, and `TOL-SNOWFREEZE-017` to
  `sum(max(hourly melt_raw_m, 0)) + rain_retained + rain_released`, with
  interval-start snow contact and pre-runoff chronology.
- Production snapshots pack depth before hourly snow/melt/liquid advancement.
  A zero-pack mixed onset therefore leaves same-interval rain on the
  post-winter-rain path, while an existing interval-start pack classifies rain
  as retained or released snow-contact rain. The focused onset vector proves
  retained/released rain and the density operand remain zero for that onset.
- Daily finalization constructs the authoritative scalar before signed-melt
  redistribution, released-rain routing, bounded state loss, and Stage-3
  disposition. Signed negative melt contributes zero.
- One private `wet_compaction_liquid_input_m` field reaches the existing
  density runtime input. The bulk and multilayer consumers convert that value
  to mass once. The caller no longer supplies routed melt or snowpack state
  loss, and `09_snow_density.rs` is intentionally unchanged.
- The offline CoE boundary emits and requires three independent finite,
  nonnegative columns: gross positive generated melt, retained rain, and
  released rain. Their exact sum reaches the replay's real daily compaction
  consumer. Routed melt and state loss remain diagnostics; changing them does
  not change the result, while changing an authority column does.

## Conservation and Anti-Tautology

The receipt-bound materiality result reconstructs the current density operand
from hourly `coe_melt_applied_m` plus separately published retained/released
rain, rather than from the density operand itself. It independently
reconstructs the predecessor duplicate from snowpack state loss plus routed
melt and compares upstream mass fields between predecessor and current traces.
Acceptance is fail-closed and reports:

- operand reconstruction: `8.353e-17 m` maximum (`1e-12 m` limit);
- predecessor duplicate reconstruction: `2.776e-17 m` maximum;
- upstream mass delta: `2.443e-15 m` maximum (`1e-9 m` limit);
- Stage-3 liquid closure: `3e-17 m` maximum;
- density-process closure: `2.274e-13 kg m^-3` maximum (`1e-9 kg m^-3`
  limit);
- layer SWE/depth closure: `4.441e-16 m` / `8.882e-16 m` maximum; and
- `24,046` driver-changed days and `22,392` density-changed days.

The observed `0.002363 m` maximum Stage-3 disposition delta is correctly
reported as a downstream density-mediated response, not used to waive the
upstream mass-invariance gate. The materiality runner binds source, binary,
tool, predecessor receipts, input files, and traces before atomic result
publication. The reviewers' recommendation to add executable negative metric
injection remains useful nonblocking test-hardening debt; the existing source
marker test is not the sole evidence for acceptance.

## Regression and Gate Evidence

Retained exact-source receipts contain 18 required commands, each marked
`PASS` with exit code zero. The original unconstrained quick-profile timeout is
preserved rather than hidden; the same source subsequently passed low-
concurrency quick `2181/2181`, frost `358/358`, and Critical full workspace
`2270/2270`, followed by doctest, dependency policy, assurance, fixture,
anti-evasion, and AUTH11 gates. Receipt identities independently match:

- command log: `123291a0e067186a6f8278e67bb83831a1c7a702fb540f26871cdb59775d2a9f`;
- command summary: `1bf9e174dd777e811e3e1999d19355bd704c891aa2d650b4565aba76766ccb0e`;
- execution receipt: `1cd4aa5fb2110eb0445f57de846e2b65b224e7b0704e00a9d6cff1e3d4ca220a`;
- materiality result: `25c8150f95d1be81afa7597d93dc271f8df5d82e062c558b231dd1695afab05a`;
  and
- release CLI: `1934000cd3c2534350af7ab1678325906762798e94dbe245b3895b910bf1382a`.

Fresh bounded verification on the current worktree also passed:

- wet-compaction authority integration: `8/8`;
- offline required-column and exact-source replay: `2/2`;
- production operand helper: `1/1`;
- AUTH11 required-suite obligations: `3/3`;
- assurance `validate --all`;
- development fixture materializer `--check`; and
- authority-suite anti-evasion.

## Fixture, Prompt, Security, and Scope Custody

- Canonical Snowbird `p8.cli` remains
  `10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7`.
  The precipitation-only derivative is
  `c673145ee7fd41e71e3f2e21c529fba2d12691abd5f0f055444e621fb0b80afb`.
  Custody fixes `14,245` daily rows, `4,472` changed positive-precipitation
  rows, exact factor `1.2155576`, decimal half-up rounding, and totals
  `46,491.8`/`56,519.1 mm`.
- The scaled lane is consistently `DEVELOPMENT_ONLY`; its 39-water-year peak
  response supports input sensitivity only, not precipitation truth,
  wet-compaction validity, calibration, defaults, or transferability.
- The kickoff prompt moved from active to archived byte-identically at
  `a863c62df3b18bb82a7de9d5a38ecf4364d1cfbfb2ae591bbfb9480fa9f1f69e`.
- No dependency or lockfile change, secret, public schema, production
  `unwrap`/`expect`, broad boxed error, or undocumented `unsafe` was added.
  Fresh assurance, anti-evasion, and AUTH11 checks pass.
- Reviewed Rust line counts are `927`, `2579`, `2723`, `969`, `1209`, and
  `428`. No touched file reaches the `3000+` mandatory-refactor threshold;
  the two reconciliation modules remain documented warning-band debt.

## Reviews, Exact Diff, and Status

Both independent reviews return `GO`; Review B's historical validation HOLD
is explicitly retained and resolved. Every review finding is dispositioned as
accepted/resolved, deferred nonblocking debt, accepted line-count debt, or an
out-of-scope follow-up. There is no rejected finding requiring an omitted
rationale.

Against declared base `d41a67c7a2c8d199f9f05f5f309b9b85915e01e1`, the
terminal scope contains `86` intended paths: `77` tracked base-to-worktree
paths plus `9` untracked closure-generated paths. `git diff --check` passes.
All paths reconcile to the declared production/authority, tests, fixture,
package/prompt, and three roadmap/catalog surfaces. `Cargo.lock`, canonical
Snowbird climate, `09_snow_density.rs`, observations, and public output
surfaces are absent from the diff. The terminal campaign's source identity is
retained as scaffold HEAD `4a6948ddbcb652310f4ca063a6c57f9b206a3740` plus
binary diff SHA-256
`bd07523f8e0f566c52a152ff4ef6d8dd2c2deadfae5ab760c88c7d6d4d4e4119`;
later changes are review, verification, closure, prompt-archival, and status
prose only.

The scientific package/roadmap wording is otherwise bounded correctly: 21K
closes only the duplicate wet-compaction operand; it does not claim an
early-melt explanation or correction. 21L is conditioned on corrected-state
rebaselining, canonical lanes own acceptance, and scaled Snowbird owns
development sensitivity only. Terminal Verification A has satisfied the
sequencing condition above; 21K may remain closed and 21L may remain admitted.
