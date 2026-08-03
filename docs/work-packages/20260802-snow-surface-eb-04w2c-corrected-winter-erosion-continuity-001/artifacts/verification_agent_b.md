# Terminal Verification B

Status: **HOLD — terminal governance and provenance closure incomplete**

Evidence mode: **Static + Ran**

## Findings

### HIGH — VB-01: the retained review history and disposition do not prove one-row-per-finding closure

`review_agent_a.md` is labeled as Review A's history but begins with the
**second** fresh re-review. The original `A-001` through `A-005` findings and
the first fresh Review A finding text are not retained there; later status
tables refer to those IDs but cannot recover their original severity, evidence,
or requested disposition. `review-disposition.md` then groups only the initial
`A-*`/`B-*` IDs and an unnamed scale risk into eight three-column rows. It does
not enumerate the fresh IDs (`FR-B-*`, `SFR-B-*`, `TFR-B-01`, `SF-A-*`, and
`SF3-A-001`) or provide the required source, severity, artifact reference, and
rationale fields.

This does not overturn the substantive corrections: the surviving re-review
tables and live tests support them. It does prevent exact verification that
every initial and fresh contract-review finding received the canonical
disposition. `science-contract-authoring-procedure.md:205-251` requires the
severity-ranked reviews to be retained and a disposition row for every finding;
missing rows block promotion.

Required correction: restore the missing Review A history from durable evidence
or explicitly record that it is unavailable, then replace/extend the disposition
with one complete row per finding ID, including source, severity, decision,
action, artifact reference, and rationale. Reverify the resulting complete
finding inventory before closing revision 57.

### HIGH — VB-02: exact terminal diff reconciliation and campaign status are stale

`terminal-diff-reconciliation.md:3-22` explicitly says reconciliation is
pending, reports the pre-correction `159` insertions / `45` deletions, describes
one contract-derived test, and says fresh review and terminal renewal have not
occurred. The current tracked diff is nine files with **600 insertions and 57
deletions**, plus the untracked package tree. The artifact has no current
tracked-diff/status/package-tree identity and therefore does not reconcile the
tree inspected by the terminal verifiers.

The same stale lifecycle state remains in `docs/ROADMAP.md:34`,
`docs/planning/snow-surface-energy-balance-roadmap.md:158,184-187`, and
`docs/work-packages/README.md:5080-5092`: they say fresh/independent review and
renewed broad gates remain pending. In contrast, `package.md`,
`disposition.md`, and `gate-results.md` correctly record dual fresh-review pass
and terminal gates 34-44 complete, with verification pending.

Required correction: after retaining both verifier outputs, recompute and
record the exact tracked diff, complete untracked inventory/package manifest,
base and dirty-state identities, final line counts, and protected-boundary
reconciliation. Advance the roadmap/catalog wording only to the actual state;
because this verifier returns HOLD, do not mark W2C complete or release W2B or
EB-04X yet.

### HIGH — VB-03: the kernel-profile compliance artifact omits mandatory dispositions

`kernel-profile-compliance-checklist.md:7-24` verifies sequencing, the mass/
diagnostic separation, tests, and broad gates, but it does not disposition the
mandatory profile items for all required schema sections, algorithm state
surfaces and branch table, guard/error mapping, unit governance, or calibration/
identifiability/readiness applicability. Static heading inspection of
`SC-SED-001.md` finds no explicit unit-governance map or calibration/
identifiability posture. The profile requires those items at
`kernel-process-contract-profile.md:60-103,128-145`, and the authoring procedure
keeps a noncompliant kernel contract in HOLD.

This leaves the initial `B-03` authority/profile finding incompletely evidenced
even though revision 57's changed Wave-1 invariant, algorithm, guard mapping,
tolerances, and test vectors are internally consistent.

Required correction: expand the compliance artifact to disposition every
mandatory item with an evidence path. Add missing canonical contract sections,
record explicit not-applicable rationale where permitted (including calibration
readiness for diagnostic numerics), or retain an approved risk-acceptance
exception. Re-review any canonical authority change.

### MEDIUM — VB-04: terminal Markdown gate 44 lacks exact invocation and document-root provenance

Gate 44's log says only `32 files validated, 0 errors, 0 warnings`, and
`gate-results.md:89` records only “scoped `markdown-doc lint`.” Neither artifact
retains the exact argv/path list or a documentation-root identity. This does not
meet the exact-command/input evidence minimum at
`testing-and-gate-strategy.md:292-307`. `gate-results.md` itself was also
updated after log 44, which is a normal self-referential closeout edit but means
the log alone cannot prove the current Markdown tree.

Required correction: record the actual exact argv and the 32-file scope if it
is recoverable; otherwise rerun the documented scope on the final reconciled
Markdown tree and retain exact argv, exit status, and file/root identity.

## Verified Technical Evidence

The result-affecting implementation is coherent and no new Rust correctness
finding was identified.

- Current base is `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`.
- The five-file result-affecting diff independently reproduces
  `a41615fc0a673ca23b70de45e09b6b8a8b2cdfa32e2ce1ba0ac5059c5d9fb176`.
- Every hash in `owned-file-manifest.md` reproduces, including
  `erosion_continuity.rs` `b95bb390...`, `erosion.rs` `869f9f33...`,
  `erosion_hb04.rs` `ae76c95e...`, EROD16 `c194c3d3...`, `SC-SED-001.md`
  `3ad30c32...`, and the partition CSV `4f70c91e...`.
- Nextest configuration and all five EROD16 fixture hashes reproduce the values
  in `gate-results.md`.
- Current line counts reproduce exactly as `2698 / 1284 / 969 / 660`.
  `erosion_continuity.rs` is correctly WARNed below the 3000-line hard boundary;
  the other touched Rust/test files pass.
- The complete tracked and untracked inventory is inside the declared write
  set. No manifest, lockfile, dependency, feature, forcing, observation,
  coefficient, assurance-source, or snow-runtime file changed. `cargo deny`
  is therefore not applicable.
- Read-only CSV reconstruction reproduces all eight transition counts, prior
  `37/227`, corrected `61/231`, and both retained refusal-cohort medians.

Behavioral anti-evasion is adequate on the current source. The seven focused
tests distinguish Simpson from trapezoid with curved one-through-seven interval
vectors; exercise production one- and seven-interval partitioning; prove seam,
clamp, region, and alignment-edge exclusion from both numerator and denominator;
inject a committed-load inconsistency; and prove the real hourly fold swallows
only `erosion.wave1.flux_closure`. The EROD16 fixture independently projects
both endpoints, reconstructs positive and negative cell deltas, compares all
four publication operands, and rejects the detachment-only alias.

Commands run independently from `/home/workdir/openWEPP`:

| Command | Result |
|---|---|
| `cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(eb04w2c)'` | PASS, `7/7`, 428 skipped |
| `cargo nextest run --test erod16_wave1_continuity_fixture_conservation --no-capture` | PASS, `1/1`; refusals exactly days 376, 715, 1036, and 1810; `227` depositing solves; `978601.7 kg` detachment and `124192.6 kg` deposition |
| `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `cargo test --doc --workspace --quiet` | PASS |
| `cargo run --quiet -p openwepp-assurance -- validate --all` | PASS, `3/3` reports |
| `git diff --check` | PASS |

## Terminal Gates 34-44

The retained logs are internally consistent with their claimed outcomes and
postdate the unchanged result-affecting sources:

| Gate | Verified retained result |
|---|---|
| 34 quick | PASS, `2156/2156`, 38 skipped, `2709.255 s` |
| 35 frost | PASS, `345/345`, 1903 skipped, `556.083 s` |
| 36 erosion | PASS, `377/377`, 1871 skipped, `148.987 s` |
| 37 Critical workspace | PASS, default `all()` profile, `2243/2243`, five skipped, `2727.948 s` |
| 38 owning crate | PASS, `435/435`, zero skipped, `147.905 s` |
| 39 Clippy | PASS with warnings denied |
| 40 formatting | PASS; empty stdout is consistent with success and was reproduced live |
| 41 workspace doctests | PASS and reproduced live |
| 42 assurance plan | PASS, zero public reports and three selected v2 reports; no `SC-SED-001`, W2C, Wave-1, or erosion-continuity dependency appears in the plan |
| 43 assurance validation | PASS, `3/3`, and reproduced live with source root `f8a9ba4c...` |
| 44 scoped Markdown lint | Outcome says PASS, 32 files and zero findings; exact argv/root provenance remains open as `VB-04` |

## Non-blocking Debt

- `erosion_continuity.rs:393-394` still calls the clamped-cell bookkeeping
  “trapezoid flux-residual accounting”; matched-order diagnostic terminology
  would be clearer.
- `review_agent_b.md` retains an initial `FINDINGS — QA HOLD` status at the top
  even though its appended final section is PASS. A top-level latest-status
  field, as used by Review A, would improve developer ergonomics.

## Hold And Verdict

`EB-04W2B` remains `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`, W2C's disposition
explicitly withholds its release, and roadmap row 21 keeps `EB-04X` held. That
posture is correct. Because Verification B is HOLD, the required dual terminal
verification gate has not passed even if Verification A returns PASS.

**HOLD.** The behavior, conservation anti-evasion, owned identities, and
terminal Rust/assurance results are technically acceptable, but formal W2C
closure is blocked by `VB-01` through `VB-04`. Do not resume W2B, rerun W2A, or
advance EB-04X until the findings are corrected and both terminal verifiers
accept the final reconciled tree.

## Revision-60 Terminal Re-verification — 2026-08-02

Status: **HOLD — one terminal Markdown provenance contradiction remains**

Evidence mode: **Static + Ran**

### Finding

#### MEDIUM — R60-VB-01: gate-results still labels final log 47 as pre-review

`artifacts/gate-results.md` gives log 47 two incompatible lifecycle identities.
Its summary table calls the revision-60 scoped Markdown lint “final
post-review” (line 50), while its exact-command table calls the same retained
log “pre-review” (line 101). `artifacts/terminal-markdown-scope.md` also binds
log 47 as the final post-review rerun and records the final self-excluding
content root. The retained log itself contains only the successful file count,
so it cannot independently resolve which lifecycle label is authoritative.

The bytes support the intended final-post-review claim: I independently reran
the exact six-path command, obtained 35 files with zero errors/warnings, and
reconstructed the stated 34-file self-excluding content root
`da27b4c9d60f3cc83e9f2f732cd2dd8fcf473fe6027b4b6257e545f396ca0e0e`.
The remaining defect is the contradictory terminal ledger, not lint content.
Correct the line-101 label to final post-review (or otherwise reconcile it to
the actual run), rerun the exact Markdown scope/content root if that correction
changes a selected file, and then request narrow re-verification. This finding
is distinct from historical `VB-04`: the required command, scope, count, log,
and content root now exist, but their terminal lifecycle description conflicts.

### VB-01 Through VB-04 Recheck

| Prior finding | Revision-60 result |
|---|---|
| `VB-01` review history/disposition | **PASS.** Review A explicitly records the unavailable verbatim history and conservatively reconstructs stable IDs/substance. `review-disposition.md` contains one independent row for every recovered initial, fresh, revision-58, revision-59, and revision-60 finding, including `R60-A-001`; each row has source, severity, accepted decision, action, artifact, and rationale. Both revision-60 reviews end in PASS with no unresolved review finding. |
| `VB-02` exact tree/lifecycle | **PASS except for `R60-VB-01`'s log-label contradiction.** HEAD, nine-file tracked status, 707/57 diff, complete/runtime/five-file diff identities, 80-path package identity, line counts, and declared write set all reproduce. Campaign surfaces consistently keep W2C at technical/review pass with re-verification pending and keep W2B/EB-04X held. |
| `VB-03` mandatory kernel profile | **PASS.** `SC-SED-001` revision 60 contains the required state surfaces, numbered algorithm and branch/guard map, constants/provenance, required unit-governance columns and conversion owners, explicit `CALIBRATION_NOT_APPLICABLE`, ten-row readiness disposition, and six-row Binding Exposure Index. The active EROD13 row includes `INV-SED-016`; normal retained and independently rerun strict BEI validation pass. |
| `VB-04` Markdown provenance | **TECHNICAL PASS / LEDGER HOLD.** Exact cwd/argv, six-path scope, exit, 35-file count, log47, and reproducible 34-file content root are present. Only the mutually exclusive pre-/post-review labels remain unresolved as `R60-VB-01`. |

### Exact Identity And Technical Reproduction

- HEAD: `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`.
- Tracked diff: nine files, 707 insertions, 57 deletions, SHA-256
  `203741f6dad955b2a18494c918ccf5830462e60f980deb961fd51965c7ac5137`.
- Four-file runtime/test diff:
  `ada609e061f5cc9eb91eaa249169ae0317548aeec71f0c57fc388d05bb1b64ee`.
- Five-file runtime/test/revision-60 contract diff:
  `2089324becad4b78809ed11c72830522c99ad73c37dacf1098bfc635807e0f80`.
- Runtime/test/contract/partition hashes reproduce exactly as
  `b95bb390...b5037`, `869f9f33...527a`, `ae76c95e...a52f2`,
  `c194c3d3...17c3`, `c0d73c88...c2c1`, and `4f70c91e...2d53`.
- Line counts reproduce as `2698 / 1284 / 969 / 660`; the sole 2,000+ file is
  correctly WARNed and remains below the 3,000-line hard boundary.
- The package has exactly 80 paths; its locale-stable sorted relative-path
  identity reproduces as
  `6bd84fb21c7fb6c5a4ca774124834ce286f7c6f4b388df3c614135a81c850b29`.
- Independently ran focused W2C: **PASS**, `7/7`, 428 skipped.
- Independently ran real EROD16: **PASS**, `1/1`, with explicit refusals on
  days 376, 715, 1036, and 1810; `4/231` refused and 227 clean/depositing.
- Independently ran strict Binding Exposure validation: **PASS**, six rows.
- Independently ran the exact scoped Markdown command: **PASS**, 35 files and
  zero findings; the selected-file content root reproduces as stated above.
- Independently ran `git diff --check`: **PASS**.

### Lifecycle Verdict

The Wave-1 matched-order correction, exact mass closure, explicit refusal
semantics, real-consumer evidence, profile/readiness/BEI authority, identities,
and W2C technical gates pass. No snow correction, process equation, solver,
grid, coefficient, tolerance, forcing, observation, or runtime/output schema
change was introduced by revisions 58–60.

Formal closure remains **HOLD** solely for `R60-VB-01`. W2B must remain
`HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`; its frozen rerun remains ineligible, and
EB-04X must remain held until this provenance contradiction is corrected and
both revision-60 terminal verifiers pass the reconciled tree.

## R60-VB-01 Narrow Final Re-verification — 2026-08-02

Status: **PASS — `R60-VB-01` corrected; no new findings**

Evidence mode: **Static + Ran**

- `gate-results.md` now identifies log 47 consistently as the **final
  post-review** lint in both the revision-60 summary row and the exact-command
  row. No `pre-review` label remains on that evidence.
- `verification-disposition.md` contains a distinct `R60-VB-01` row with
  source `verifier_b`, severity `MEDIUM`, decision `accepted`, the completed
  correction/renewal action, artifact references, and rationale.
- Independently reran the exact six-path Markdown command from
  `/home/workdir/openWEPP`: **PASS**, 35 files, zero errors, zero warnings.
- Immediately before this authorized verifier-history append, independently
  reconstructed the 34-file self-excluding Markdown content root as
  `f58b4415c83dc198f916f893d7580cca12b4a2ce8328d4bca689b092bf6743c9`,
  exactly matching `terminal-markdown-scope.md`.
- HEAD remains `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`; the nine-file tracked
  diff remains 707 insertions/57 deletions with complete diff
  `203741f6...5137`, runtime/test diff `ada609e0...4ee`, and revision-60
  runtime/test/contract diff `2089324b...f80`.
- All six owned source/contract/partition hashes remain unchanged. The package
  still contains exactly 80 paths with sorted-path identity
  `6bd84fb21c7fb6c5a4ca774124834ce286f7c6f4b388df3c614135a81c850b29`.
- `git diff --check` passes. Roadmap, catalog, package, and disposition surfaces
  continue to keep W2B at `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`, its frozen rerun
  ineligible, and EB-04X held pending formal W2C closeout.

`R60-VB-01` is closed. Verification B now **PASS**es the revision-60 terminal
tree with no remaining or newly discovered finding. This verifier does not
itself release W2B or EB-04X; the primary agent must first reconcile the dual
verification result and publish W2C's formal terminal disposition.
