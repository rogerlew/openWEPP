# Terminal Verification A

Status: **HOLD — technical correction passes; terminal governance is not
reconciled**

Evidence mode: **Static + Ran**

Verification point: `/home/workdir/openWEPP`, base commit
`a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`.

## Findings

### HIGH — VA-001: exact terminal-diff and repository-status reconciliation is still explicitly pending

The package requires exact-diff reconciliation before completion, but
`artifacts/terminal-diff-reconciliation.md:3-22` still identifies itself as
superseded/pending, reports the pre-review-correction `159` insertions and `45`
deletions, describes one contract-derived test, and says its counts must be
recomputed after fresh review and renewed terminal gates.

Independent inspection of the current tree instead finds:

- nine tracked files with `600` insertions and `57` deletions;
- tracked-diff SHA-256
  `5f73eac70ea0b1f560b64d4a122ae7c9c44480654abd273175c65350ac1a5628`;
- seven focused W2C tests, an internal real-hourly-consumer solver seam, and
  the independently reconstructed EROD16 ledger in the terminal diff; and
- `73` files in the untracked package tree, all within the declared write set.

The same stale lifecycle remains on repository-facing surfaces:

- `docs/ROADMAP.md:34` says fresh review and broad renewal are pending;
- `docs/planning/snow-surface-energy-balance-roadmap.md:158,184-187` says
  fresh review and renewed gates are pending; and
- `docs/work-packages/README.md:5080-5092` says independent review is pending
  and contains the stale grammar “revisions 56–57 separates”.

Both reviews have in fact reached final PASS and terminal logs 34–44 contain
the renewed broad gates. These understatements preserve the W2B/EB-04X hold,
but they do not satisfy the package's exact terminal reconciliation and
truthful catalog/status gates. In addition, the “Exact argv” table in
`artifacts/gate-results.md:69-89` records only “scoped `markdown-doc lint`” for
log 44; the log itself contains only the 32-file summary, so the exact command
and selected paths cannot be reproduced from the retained evidence.

Required correction: reconcile the complete terminal diff and current status
surfaces, record the exact Markdown-lint argv/path scope, rerun the affected
documentation checks and `git diff --check`, disposition this finding, and
obtain fresh terminal verification. Broad Rust evidence may be reused only if
the result-affecting identity remains unchanged and that exclusion is recorded.

### MEDIUM — VA-002: mandatory kernel-profile conformance is not demonstrated

`docs/specifications/science-contracts/kernel-process-contract-profile.md:60-103`
requires every applicable `SC-*` file to include algorithm-state surfaces, a
branch/guard table, a constants/parameters table with provenance, a
unit-governance map, and a calibration/identifiability posture or explicit
`CALIBRATION_NOT_APPLICABLE` rationale. Its checklist rule at lines 128–140
requires the package artifact to disposition those items, and lines 142–146
make noncompliance a package HOLD.

`SC-SED-001` revision 57 now carries the W2C invariant, tolerance, algorithm,
guard, and test-vector authority, but it still has no explicit
constants/parameters table, unit-governance map, or calibration/
identifiability-not-applicable section. The package's
`artifacts/kernel-profile-compliance-checklist.md:7-24` checks the W2C behavior
and validation but does not disposition the profile's required schema,
unit-governance, or calibration/readiness items. No approved risk-acceptance
exception is recorded.

Required correction: complete or explicitly disposition every mandatory
profile item in canonical authority and the package checklist. At minimum,
bind the fixed dimensionless `32 * f64::EPSILON` alignment tolerance and the
unchanged `5e-3` diagnostic tolerance in the constants/provenance posture,
record the touched unit-governance applicability, and state why empirical
calibration/identifiability is not applicable to this diagnostic-numerics
correction. Re-review any canonical authority change before re-verification.

## Verified Technical Evidence

### Source identity and write set

- Current owned hashes exactly match `artifacts/owned-file-manifest.md`:
  `erosion_continuity.rs` `b95bb390...b5037`, `erosion.rs`
  `869f9f33...527a`, `erosion_hb04.rs` `ae76c95e...a52f2`, EROD16
  `c194c3d3...17c3`, `SC-SED-001.md` `3ad30c32...b8b6`, and the storm
  partition `4f70c91e...2d53`.
- The five-file result-affecting source/contract diff independently reproduces
  as `a41615fc0a673ca23b70de45e09b6b8a8b2cdfa32e2ce1ba0ac5059c5d9fb176`.
- Nextest and all five disturbed-burn fixture hashes reproduce the identities
  in `gate-results.md`.
- The tracked/untracked inventory contains no path outside the package's
  intended write set. No manifest, lockfile, dependency, fixture forcing,
  observation, assurance source, or snow-runtime file changed.

### `SC-SED-001` and numerical mechanics

- Revision 57 corrects `INV-SED-016(f)` in place and binds separate
  `TOL-SED-007` exact publication closure and `TOL-SED-008` discretization
  consistency. The tolerances remain `1e-9` and `5e-3`, respectively.
- `wave1_totals` evaluates the denormalized boundary/per-cell telescoping mass
  identity before invoking the matched-order diagnostic. A publication
  failure remains the typed `erosion.wave1.publication_closure` hard error.
- Every analytic-deposition or RK4 sub-march receives a distinct diagnostic
  zone. Its first interval is eligible only when the normalized sub-march
  boundary lies within `32 * f64::EPSILON` of the preceding grid point;
  otherwise that straddling interval receives zone zero.
- The diagnostic admits only contiguous, nonzero-zone, same-region, unclamped
  intervals. Residual and scale traverse the same accepted slice, so an
  excluded seam, region transition, or clamp cannot dilute the denominator.
- Nonoverlapping Simpson `1/3` pairs and a final Simpson `3/8` triple are used;
  trapezoid remains only for a single eligible interval. No Wave-1 process
  equation, RK4/analytic solution, grid, coefficient, unit conversion, snow
  correction, or tolerance changed. The implementation centralizes rather
  than duplicates this logic.

### Error and consumer disposition

- The production hourly fold still calls the original quantum solver through
  its internal seam. It converts only
  `DirectClosureToleranceExceeded { field:
  "erosion.wave1.flux_closure" }` into zero sediment plus one surfaced
  `flux_refused_quanta`; every other typed error propagates.
- The focused consumer test proves
  `erosion.wave1.publication_closure` cannot be swallowed. No new error enum,
  code/message drift, fallback, serialization change, or unreachable variant
  was introduced.

### Independent ledger and review closure

- The EROD16 test reconstructs inflow, export, positive cell deltas, and
  negative cell deltas from the published normalized load trajectory plus
  input denormalization. It compares all four published operands and rejects
  the detachment-only alias. This is independent produced-operand accounting,
  not an external process-model or observation claim.
- Live EROD16 execution passes with 231 storms, 227 accepted/depositing solves,
  four explicit refusals on days `376`, `715`, `1036`, and `1810`, aggregate
  detachment `978601.7 kg`, and deposition `124192.6 kg`.
- Independent CSV reconstruction reproduces every retained transition and the
  prior/current diagnostic populations: prior `37/227`; corrected old
  diagnostic `61/231`.
- Review families `A-001` through `A-005`, `B-01` through `B-05`,
  `FR-B-01` through `FR-B-03`, `SF-A-001/002`, `SFR-B-01/02`,
  `TFR-B-01`, and the eligible-scale risk all have accepted corrections and
  final reviewer PASS. VA-001 and VA-002 above are new terminal findings.

### Retained terminal logs 34–44

- 34 quick: `2156/2156` passed, 38 skipped.
- 35 frost: `345/345` passed, 1903 skipped.
- 36 erosion: `377/377` passed, 1871 skipped.
- 37 full workspace: `2243/2243` passed, five skipped.
- 38 owning crate: `435/435` passed, zero skipped.
- 39 warnings-denied clippy: clean completion.
- 40 formatting: empty success log, with exit `0` recorded in gate evidence.
- 41 workspace doctests: every listed target reports `ok`, zero failures.
- 42 assurance plan: `PASS`, three selected v2 reports.
- 43 assurance validation: `PASS`, `3/3` reports.
- 44 scoped Markdown lint: 32 files, zero errors/warnings, subject to
  VA-001's missing exact argv/path provenance.

The result-affecting source and contract files predate logs 31–44 and retain
the manifest hashes. No contrary failure appears in logs 34–44.

### Line count and campaign hold posture

- Current counts reproduce the checklist: `erosion_continuity.rs` 2698
  (`WARN`, below 3000), `erosion.rs` 1284, `erosion_hb04.rs` 969, and EROD16
  660. The bounded-module rationale is reasonable; no new substantial
  duplicated Rust logic was found.
- W2B remains truthfully `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`: its terminal
  full profile and frozen W2A rerun have not been resumed. EB-04X remains held
  behind W2B. W2C technical success removes the erosion prerequisite only
  after W2C itself satisfies terminal governance; it does not automatically
  complete W2B or admit EB-04X.

## Commands Run Independently

From `/home/workdir/openWEPP` on the current tree:

| Command | Result |
|---|---|
| `cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(eb04w2c)'` | PASS, `7/7`, 428 skipped |
| `cargo nextest run --test erod16_wave1_continuity_fixture_conservation --no-capture` | PASS, `1/1`; `4/231`, 227 depositing |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `git diff --check` | PASS |
| SHA-256/file-count/diff/write-set reconstruction | PASS, with VA-001's stale terminal artifact confirmed |
| Independent `storm-partition.csv` aggregation | PASS, prior `37/227`, corrected `61/231` |

## Residual Risk And Verdict

The four retained diagnostic refusals remain an explicit under-estimate and
the conservation reconstruction proves produced-operand accounting rather
than empirical erosion accuracy. Those limitations are correctly scoped and
are not new blockers. Broad terminal Rust gates were inspected and identity-
bound rather than rerun; the live focused, real-fixture, clippy, format, and
diff checks agree with them.

**HOLD.** No unresolved arithmetic, seam, unit, typed-error, consumer,
serialization, conservation-ledger, duplication, or review-correction defect
was found. Formal package acceptance is blocked by VA-001 and VA-002. Keep W2B
and EB-04X held until both findings are corrected, dispositioned, and freshly
verified.

## Revision-60 Terminal Re-verification — 2026-08-02

Status: **PASS — VA-001 and VA-002 are closed; no findings**

This section supersedes the earlier terminal verdict while preserving its
history. Evidence mode: **Static + Ran**.

### Prior-verification finding disposition

| Finding | Revision-60 result | Evidence |
|---|---|---|
| `VA-001` | **PASS / CLOSED** | `verification-disposition.md` accepts the finding. Terminal reconciliation now reproduces nine tracked paths, `707` insertions, `57` deletions, complete tracked-diff SHA-256 `203741f6dad955b2a18494c918ccf5830462e60f980deb961fd51965c7ac5137`, and the complete 80-path package identity. Repository-facing status and exact Markdown provenance are current. |
| `VA-002` | **PASS / CLOSED** | `SC-SED-001` revision 60 now supplies the mandatory algorithm-state, step/branch/guard, constants/provenance, unit-governance, and `CALIBRATION_NOT_APPLICABLE` surfaces. The package checklist and readiness matrix map the profile requirements without weakening sediment or snow authority. |

### Review-history and authority verification

- The retained review history exposes the overwritten early Review A prose,
  conservatively reconstructs its stable finding inventory, and keeps every
  A/B, fresh-review, seam, evidence, revision-58, revision-59, and revision-60
  finding individually dispositioned. `review-disposition.md` accepts every
  finding; none is rejected, waived, deferred, or silently grouped.
- Revision-60 Review A's `R60-A-001` is accepted and its narrative recheck is
  **PASS**. Revision-60 Review B is **PASS** with no findings. The operative
  package, review disposition, gate results, terminal reconciliation, profile
  checklist, and handoff consistently state dual revision-60 review PASS and
  terminal re-verification pending.
- `SC-SED-001` is version 60 with SHA-256
  `c0d73c88858959ce481f4de579d07a495945323156169f7b916ef5d62072c2c1`.
  Its profile separates exact publication closure under `TOL-SED-007` from the
  recoverable diagnostic refusal under `TOL-SED-008`, binds sub-march zones and
  zero-/one-interval behavior, fixes all numerical constants and conversion
  owners, and explicitly makes calibration inapplicable.
- The six-row Binding Exposure Index is structurally and semantically complete.
  In particular the active EROD13 row now maps its `TOL-SED-007/008`
  sub-march/refusal residue to `INV-SED-016`. Both the normal and `--strict`
  canonical checkers independently pass with six consolidated rows. The
  science-contract index and revision history identify revision 60.

### Exact identity and evidence verification

- HEAD/base remains `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`.
  The current tracked inventory contains exactly the declared nine paths and no
  W2C-owned path outside the intended write set. `git diff` independently
  reproduces `707/57` and SHA-256 `203741f6...137`.
- The four runtime/test files reproduce diff SHA-256
  `ada609e061f5cc9eb91eaa249169ae0317548aeec71f0c57fc388d05bb1b64ee`;
  adding revision-60 `SC-SED-001` reproduces
  `2089324becad4b78809ed11c72830522c99ad73c37dacf1098bfc635807e0f80`.
  File hashes reproduce `b95bb390...b5037`, `869f9f33...527a`,
  `ae76c95e...a52f2`, and `c194c3d3...17c3`. Thus revisions 58–60 are
  documentation-authority changes and do not invalidate logs 31–46 or the
  renewed broad-gate evidence bound to the unchanged runtime/test identity.
- The package contains exactly 80 files. Its locale-stable sorted relative-path
  list independently hashes to
  `6bd84fb21c7fb6c5a4ca774124834ce286f7c6f4b388df3c614135a81c850b29`.
  This required append changes no path identity.
- Log 47 postdates both final revision-60 review appends and records `35 files
  validated, 0 errors, 0 warnings`. The exact six-path `markdown-doc lint`
  command in `terminal-markdown-scope.md` independently passes with the same
  count. The locale-sorted `sha256sum <path>` ledger over those Markdown files,
  excluding only the self-referential scope artifact, contains 34 files and
  independently reproduces content root
  `da27b4c9d60f3cc83e9f2f732cd2dd8fcf473fe6027b4b6257e545f396ca0e0e`.
  This is correctly labeled a final **post-review** snapshot; terminal verifier
  appends are the expected evidence output and do not retroactively alter the
  reviewed authority/runtime identity.
- `git diff --check` passes. No manifest, lockfile, dependency, feature,
  fixture, observation, assurance source, coefficient, tolerance, snow-runtime,
  or runtime/output-schema input changed; `cargo deny check` remains not
  applicable.

### Retained executable evidence and campaign posture

Logs 34–43 retain current-identity quick `2156/2156`, frost `345/345`, erosion
`377/377`, full workspace `2243/2243`, owning crate `435/435`, clippy, format,
doctest, assurance plan, and assurance validation passes. Logs 45–46 retain
focused W2C `7/7` and EROD16 `1/1`, with 227 accepted/depositing storms and
four explicit refusals out of 231. No broad gate was rerun for this
documentation-only re-verification; exact runtime/test identity justifies
reuse.

The roadmap, catalog, package, and disposition remain deliberately
non-promotional: W2C is a technical/review pass awaiting the second terminal
re-verification; W2B remains `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`; W2A's first
rerun remains prerequisite-ineligible; and EB-04X remains held after W2B. This
verification does not itself resume W2B, authorize a W2A rerun, or advance
EB-04X.

### Revision-60 verdict

**PASS.** No unresolved revision-60 finding remains. Verifier A accepts the
corrected exact tree and closes `VA-001` and `VA-002`. Formal W2C completion
still requires verifier B's independent PASS and primary-agent reconciliation;
until then the existing W2B and EB-04X holds remain binding.

## R60-VB-01 Narrow Re-verification — 2026-08-02

Status: **PASS — no new finding**

Evidence mode: **Static + Ran**. This narrow addendum supersedes only the
pre-correction terminal-lint identity from the preceding section; all earlier
review and verification history remains authoritative evidence of chronology.

- `gate-results.md` now identifies log 47 consistently in both its summary and
  exact-command tables as the **final post-review** revision-60 scoped Markdown
  lint. No pre-review label remains on that retained log.
- `verification-disposition.md` contains a distinct `R60-VB-01` row with
  source `verifier_b`, severity `MEDIUM`, decision `accepted`, the exact
  correction, artifact references, and rationale. It is not grouped into
  historical `VB-04` or silently waived.
- The exact six-path `markdown-doc lint` command independently passes with 35
  selected files, zero errors, and zero warnings.
- At narrow re-verification intake, the locale-sorted `sha256sum <path>` ledger
  over that exact Markdown selection, excluding only the self-referential
  `terminal-markdown-scope.md`, contains 34 files and independently reproduces
  the renewed post-review content root
  `f58b4415c83dc198f916f893d7580cca12b4a2ce8328d4bca689b092bf6743c9`.
  This verifier append is the expected evidence output after that snapshot.
- The package remains exactly 80 paths with sorted-path SHA-256
  `6bd84fb21c7fb6c5a4ca774124834ce286f7c6f4b388df3c614135a81c850b29`.
  Tracked reconciliation remains nine paths, `707/57`, complete diff
  `203741f6...137`; the runtime/test and runtime/test/contract identities remain
  `ada609e0...4ee` and `2089324b...f80`. `git diff --check` passes.
- Status remains deliberately unreleased while narrow dual re-verification is
  reconciled: W2C is a technical/review pass with re-verification pending, W2B
  remains `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`, W2A's first rerun remains
  prerequisite-ineligible, and EB-04X remains held after W2B.

**PASS.** `R60-VB-01` is accepted and corrected, the renewed documentation
evidence is reproducible, and no new verifier-A finding exists. This PASS does
not itself resume W2B, authorize the W2A rerun, or advance EB-04X.
