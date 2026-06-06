# ADR-0016: Promote wepp_260430 plus the negative-melt fix as the canonical comparator reference

**Status:** Accepted
**Date:** 2026-06-05
**Deciders:** Roger Lew, Codex
**Author of draft:** Claude Code (disposition; deciders ratify Accepted)
**Amends:** [ADR-0012](0012-legacy-wepp-260430-baseline-anchor.md)
**Amended By:** [ADR-0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) (proposed) — re-pins operational distrust; comparator is a flag, not a target

## Context

Both legacy trees are clones of `github.com/wepp-in-the-woods/wepp-forest`
(shared root `05507637`):

- `/workdir/wepp-forest` — branch `master` @ `924ab16d` (the ongoing
  "kernel-rewrite" / WB-* remediation line). Carries the negative-melt fix
  (`ngtvML/pstvML`, `winter.for:441-460`, commit `03fee45`).
- `/workdir/wepp-forest_260430_baseline` — `dac3c950`, the pinned `wepp_260430`
  release that ADR-0012 anchors the comparator to. Predates the negative-melt
  fix, so it still contains the negative-melt **sign bug**.

This split has produced a persistent dual-authority problem across the HPHYS
snow/`RM` arc: openWEPP implements the **corrected** negative-melt math while the
comparator baseline carries the **buggy** version. That ambiguity is the root of
the HPHYS0296 "magic vs science" acceptance question — whether a residual is an
openWEPP defect or openWEPP correctly diverging from a buggy baseline — and the
whole acceptance-criteria/defect-ledger apparatus (HPHYS0296-0302) exists to
adjudicate it. The comparator-surface artifacts from HPHYS0298 (`hrsnow`) and
HPHYS0301 (H39 residual rain/release) compounded the confusion.

This ADR does not make legacy behavior a universal correctness oracle. ADR-0011
still governs: comparator deltas are confidence-tiered investigation evidence
unless source-line, contract, and conservation evidence proves defect ownership.

## Decision

1. **Preserve immutable archaeology.**
   - Create an annotated tag `wepp_260430_original_buggy_dac3c950` at
     `/workdir/wepp-forest_260430_baseline` commit `dac3c950`.
   - Create an annotated tag `kernel-rewrite-abandoned-20260605` at
     `/workdir/wepp-forest` commit `924ab16d`.
   - These tags are references for reviewability; no history is deleted.
2. **Do not force-reset canonical `main`/`master` as part of this ADR.**
   openWEPP comparator authority should move by exact commit/tag reference, not
   by assuming a mutable default branch points at the intended comparator.
   Moving a remote default branch is a separate repository-maintenance action,
   not a prerequisite for this ADR.
3. **Create a fixed 260430 comparator anchor.** HPHYS0303 created local branch
   `wepp_260430_negmeltfix_comparator` from `dac3c950` and ported only the
   negative-melt source patch from commit `03fee45` (`winter.for:431-447`,
   `ngtvML/pstvML` math) plus rebuilt release binaries. The fixed comparator
   commit is `47ac4c32faeea81bb99081f955a14c38b815ef4d`, with annotated tag
   `wepp_260430_negmeltfix_comparator_47ac4c32faee`. Pushes require separate
   repository-maintenance authorization.
4. **Ratified exact identity.** HPHYS0303 records the accepted local comparator
   identity:
   - fixed comparator commit SHA:
     `47ac4c32faeea81bb99081f955a14c38b815ef4d`
   - fixed `release/wepp_260430` SHA256:
     `cd56985bf1575d8d82d4b4f943ca29a4fc2865448d7c308c0220c565a8955e87`
   - fixed `release/wepp_260430_hill` SHA256:
     `b9337f1db714ef3d4ae45633b88249ccdc5416fbb7f5614a45fb688126eb45cd`
   - regenerated H1..H39 baseline parquet set:
     `/tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions`
   - regenerated baseline parquet hash manifest:
     `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-baseline-parquet-manifest.json`
   - H1/H7/H39 observe-identity proof artifact:
     `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/observe-identity-fixed-comparator.json`
   - ADR-0012 amendment:
     `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
   - SC provenance/reference update:
     `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
     `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`, and
     `docs/specifications/science-contracts/index.md`
   - SC unit/provenance lint result artifact:
     `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/sc-unit-provenance-lint.json`
5. **Then instrument.** Only after the fixed comparator anchor, binary hashes,
   observe identity, and regenerated baseline parquets exist do the paired
   baseline/openWEPP melt-term instrumentation package proceed for
   `amelt`/`bmelt`/`cmelt`/`dmelt` and state/forcing surfaces.

## Rationale

- **One corrected negative-melt comparator.** Folding the negative-melt fix into
  the canonical comparator collapses the corrected-vs-buggy split for this
  defect class. There is one comparator reference for negative melt, with the
  original buggy baseline still preserved for archaeology and delta proof.
- **Restores 39/39 as a legitimate goal.** With the baseline corrected, openWEPP
  must match it on negative-melt rows. The prior bind ("cannot reach 39/39
  without reintroducing the legacy bug") disappears.
- **Narrows the acceptance escape hatch.** Once both sides use the corrected
  negative-melt branch, negative-melt-specific divergence can no longer be
  accepted merely as "corrected openWEPP versus buggy baseline." The HPHYS0296
  acceptance gates (`INV-SNOWFREEZE-027`, `INV-WATBAL-071`,
  `INV-RUNOFFPART-024`) can be narrowed for the negative-melt class; their
  broader comparator-surface and defect-ledger discipline remains useful.
- **Cleaner root-causing.** Instrumenting against a reference that is already
  corrected on negative melt makes residuals less confounded. Residuals still
  require ADR-0011-style contract/source/conservation evidence before they are
  labeled openWEPP defects.

### Why fix before instrument (ordering is load-bearing)

If instrumentation precedes the fix, every paired comparison is taken against a
reference that is about to change, and must be redone. Fix first, instrument once,
against the final reference.

## Preconditions and implications (these break the plan if missed)

1. **Baseline surgery must be auditable and reversible.** The executed package
   must prove:
   - the old buggy baseline tag points to `dac3c950`;
   - the abandoned kernel-rewrite tag/branch points to `924ab16d`;
   - the fixed comparator branch starts from `dac3c950`;
   - the only source change on the fixed comparator branch is the
     `03fee45` negative-melt patch in `winter.for:441-460`;
   - the new commit, binary hashes, and generated comparator artifact manifest
     are recorded before this ADR is marked Accepted.
2. **Port faithfully, do not re-derive.** Commit `03fee45` is the fixed
   comparator patch provenance for the negative-melt correction, not production
   kernel authority. Canonical openWEPP behavior remains governed by `SC-*`
   contracts. Verify the ported `winter.for` reproduces the abandoned line's
   negative-melt behavior on at least one known case rather than
   re-implementing it.
3. **Regenerate ALL baseline comparator parquets — highest-risk gotcha.** Every
   `baseline_H*.parquet` the H1..H39 semantic suite compares against (e.g. the
   `/tmp/unpalatable_parity_*` set) was generated from the **buggy** `dac3c950`
   binary. After the fix they are stale. The full baseline set must be
   regenerated from the fixed binary before any new comparison is meaningful.
   Comparing the fixed openWEPP against stale buggy-baseline parquets would
   produce garbage and is the worst failure mode here.
4. **Re-establish observe-identity for the fixed (and later instrumented)
   baseline.** The HPHYS0298 byte-identical-SHA proof (release = observe-off =
   observe-on) was for `dac3c950`. Re-prove it for the fixed binary on the
   H1/H7/H39 target windows. For ratification, verify the fixed comparator
   source delta is limited to the negative-melt `winter.for` patch and record
   the fixed-vs-original output delta manifest. Row-level expected-magnitude
   melt-term proof is deferred to paired instrumentation and is not used to
   authorize production physics changes.
5. **Amend ADR-0012 and update provenance citations.** ADR-0012 pins the
   comparator to `dac3c950`. The new anchor is "wepp_260430 + negative-melt fix"
   at the new fixed comparator commit. The fix is localized to
   `winter.for:441-460`, so most `REF-*` line citations should hold, but the
   commit hash changes everywhere; update ADR-0012, contract `REF-*` rows, and
   the binary hash evidence, then re-run the SC unit/provenance lint.
6. **Collapse the corrected-vs-buggy dual citations for negative melt.** Contracts
   that cite both
   `wepp-forest_260430_baseline` (buggy) and `/workdir/wepp-forest` `03fee45`
   (corrected) for negative melt — `REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX`
   and the buggy-baseline refs, plus `INV-SNOWFREEZE-019` and the
   former "corrected vs pinned-baseline negative-melt" distinctions — now
   reconcile to one source for the active comparator. The original buggy
   baseline remains an archived reference, but it ceases to be the active
   negative-melt comparator.
7. **Re-run and re-classify the HPHYS0296-0302 snow/`RM` windows.** That analysis
   was predicated on "baseline buggy, openWEPP corrected." Against the fixed
   baseline, negative-melt-attributable divergence should either disappear or
   become a less-confounded residual requiring contract/source/conservation
   proof. The H7 first-2013 "post-raw routing without baseline-negative-melt"
   window in particular must be re-evaluated.
8. **Carry HPHYS0302 surface-audit rigor into instrumentation.** Each new paired
   melt-term surface must be verified like-for-like (units + lineage stage) at the
   time it is added, or a third comparator artifact after `hrsnow` and H39 is
   likely.

## What this does NOT change

- 260430 remains the baseline lineage; this **refines** ADR-0012's anchor (adds
  the negative-melt fix, formalizes abandonment of the divergent master line), it
  does not reverse the decision to anchor on wepp_260430.
- The fixed 260430 anchor becomes the single active comparator for regenerated
  H1..H39 baseline artifacts after ratification. The original `dac3c950`
  baseline remains preserved for archaeology and delta proof only.
- openWEPP's own negative-melt implementation is unaffected (it is now matched on
  both sides).
- HPHYS0302's surface semantics remain valid: `RM`/`Snow-Water` are daily
  output surfaces, while raw/post-raw melt comparisons are aggregate cut-points
  only. Their numeric metrics must still be regenerated against the fixed
  comparator before new residual decisions are made.
- HPHYS0302's `HOLD` remains active. This ADR does not authorize production
  melt, forcing, WB17, WB18, WB19, or WB13 patches before paired term/state
  instrumentation and disposition complete.
- ADR-0011 remains active: legacy comparator evidence is confidence-tiered
  investigation evidence, not a universal correctness oracle.

## Ratification Checklist

HPHYS0303 completed the local ratification checklist:

- Annotated original-baseline tag at `dac3c950`:
  `wepp_260430_original_buggy_dac3c950`.
- Annotated abandoned-line tag at `924ab16d`:
  `kernel-rewrite-abandoned-20260605`.
- Fixed comparator branch/tag and exact commit SHA:
  `wepp_260430_negmeltfix_comparator`,
  `wepp_260430_negmeltfix_comparator_47ac4c32faee`, and
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
- Fixed `release/wepp_260430` SHA256:
  `cd56985bf1575d8d82d4b4f943ca29a4fc2865448d7c308c0220c565a8955e87`.
- Fixed `release/wepp_260430_hill` SHA256:
  `b9337f1db714ef3d4ae45633b88249ccdc5416fbb7f5614a45fb688126eb45cd`.
- H1/H7/H39 observe-identity proof for the fixed comparator binary:
  `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/observe-identity-fixed-comparator.json`.
- Fixed-vs-original source-limited output delta manifest:
  `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-vs-original-output-delta.json`.
- Regenerated H1..H39 baseline parquet set and hash manifest:
  `/tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions`
  and
  `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/fixed-baseline-parquet-manifest.json`.
- ADR-0012 amendment carrying the new comparator commit and binary hashes:
  `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`.
- Contract `REF-*`/negative-melt citation update:
  `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`, and
  `docs/specifications/science-contracts/index.md`.
- SC unit/provenance lint result:
  `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/sc-unit-provenance-lint.json`.

## Executed Ratification Order

HPHYS0303 executed the authorized local baseline-comparator work package that
completed the ratification checklist. Remote pushes remain out of scope.

1. In `/workdir/wepp-forest_260430_baseline`, create annotated tag
   `wepp_260430_original_buggy_dac3c950` at `dac3c950`.
2. In `/workdir/wepp-forest`, create annotated tag
   `kernel-rewrite-abandoned-20260605` at `924ab16d`.
3. In a dedicated comparator worktree, create branch
   `wepp_260430_negmeltfix_comparator` from `dac3c950`.
4. Port only the `03fee45` `winter.for` negative-melt source patch onto that
   branch; verify behavior parity with the abandoned line.
5. Rebuild the baseline release binaries; record fixed commit SHA and binary
   SHA256s in this ADR and ADR-0012 before ratification.
6. Re-prove H1/H7/H39 observe-identity and record source-limited fixed-vs-
   original output deltas.
7. **Regenerate all H1..H39 baseline comparator parquets** from the fixed binary
   and record the artifact manifest/hash path.
8. Amend ADR-0012; update contract `REF-*` commit citations; collapse
   corrected-vs-buggy negative-melt dual-citations; rerun SC lint.

## Required Continuation Order

1. Re-run the H1..H39 semantic suite (openWEPP vs fixed baseline) and
   re-classify the snow/`RM` windows under ADR-0011 confidence-tier rules.
2. Then scaffold the paired melt-term instrumentation package against the fixed
   baseline.

## Consequences

- Eliminates the dual-authority confusion and narrows the need for
  negative-melt acceptance adjudication; after ratification, the fixed 260430
  anchor becomes the active H1..H39 comparator artifact source, while ADR-0011
  still governs broader correctness decisions.
- One-time cost: full baseline regeneration, observe-identity re-proof, and
  citation/commit updates across contracts.
- After the fixed-baseline semantic rerun, remaining H1..H39 residuals are less
  confounded and better suited for paired instrumentation. They are not labeled
  openWEPP defects until contract/source/conservation evidence supports that
  classification.
