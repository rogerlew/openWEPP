# HPHYS0303 ADR-0016 Fixed Comparator Ratification

## Status

Executed.

## Objective

Complete the local work needed to ratify ADR-0016: preserve immutable legacy
archaeology refs, create and verify the fixed `wepp_260430` negative-melt
comparator anchor, rebuild/hash fixed binaries, regenerate baseline comparator
artifacts where feasible, amend ADR-0012/ADR-0016 and negative-melt provenance
citations, and keep the HPHYS0302 production-edit `HOLD` active until paired
term/state instrumentation is complete.

## Rationale

ADR-0016 required an executed local package to prove exact comparator commit,
binary hashes, observe-identity proof, baseline parquet manifest, ADR-0012
amendment, SC provenance updates, and SC lint evidence before ratification. The
prior HPHYS0302 audit also requires that aggregate/output residuals do not
authorize production forcing, snow, WB17, WB18, WB19, or WB13 patches.

## Included Scope

- Create local annotated archaeology tags for:
  - `dac3c950` as `wepp_260430_original_buggy_dac3c950`.
  - `924ab16d` as `kernel-rewrite-abandoned-20260605`.
- Fixed comparator branch/tag and exact commit SHA.
- Create a local fixed comparator branch/tag from `dac3c950` with only the
  `03fee45` negative-melt source patch plus generated fixed release binaries.
- Rebuild `release/wepp_260430` and `release/wepp_260430_hill` and record
  SHA256 hashes.
- Prove the source delta is limited to `src/winter.for` negative-melt lines,
  excluding generated binaries.
- Attempt observe-identity and baseline parquet regeneration; record hard
  blockers truthfully if unavailable in this environment.
- Amend ADR-0016 and ADR-0012 with completed local evidence or explicit HOLD
  blockers.
- Update negative-melt contract provenance/citation language only when the
  fixed comparator anchor is fully proven.
- Add contract/doc tests guarding ratification completeness and HPHYS0302 HOLD
  carry-forward.
- Complete dual review/disposition and dual verification.

## Excluded Scope

- Pushing tags, branches, or commits to any remote.
- Resetting or changing any remote default branch.
- Production openWEPP physics edits.
- Production melt/forcing/WB17/WB18/WB19/WB13 patches.
- Treating a partial local comparator build as ratified when any checklist item
  remains unproven.

## Dependencies

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/artifacts/disposition.md`
- `/workdir/wepp-forest_260430_baseline`
- `/workdir/wepp-forest`

## Intended Write Set

- `docs/work-packages/README.md`
- `docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/**`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `tests/integration/hphys0303_adr0016_comparator_ratification_contract.rs`
- `Cargo.toml`
- Local external git refs/worktree under `/workdir/wepp-forest_260430_baseline`
  and `/tmp/hphys0303_*` for comparator evidence only.

## Phase Plan

1. **Scaffold**: create package artifacts and kickoff prompt.
2. **Contract and ADR gates**: add tests for ADR-0016 ratification semantics and
   HPHYS0302 HOLD carry-forward.
3. **Comparator anchor**: create local tags, sparse worktree, fixed comparator
   commit/tag, and binary hashes.
4. **Evidence gates**: prove source delta, build hashes, smoke checks,
   observe-identity/parquet feasibility, and blockers.
5. **Docs update**: amend ADR-0012/0016 and contracts according to proven
   evidence; keep `HOLD` if any ratification item remains incomplete.
6. **Review and validation**: run focused/full gates and dual review with
   disposition.

## Progress

- [x] Scaffold package and required artifacts.
- [x] Add ADR/contract ratification tests.
- [x] Create comparator anchor evidence.
- [x] Run build/hash/source-delta gates.
- [x] Attempt observe/parquet gates.
- [x] Amend ADR/contracts from evidence.
- [x] Complete dual review/disposition/verification.

## Exit Criteria

- ADR-0016 is either ratified with all checklist items complete, or remains
  `Proposed-HOLD` with concrete unproven blockers.
- ADR-0012 is amended only if the fixed comparator commit and binary hashes are
  proven.
- Contracts collapse negative-melt dual-provenance only if the active fixed
  comparator anchor is proven.
- HPHYS0302 production-edit `HOLD` remains explicit.
- No remote refs are pushed.
- Dual review/verification artifacts have no undispositioned findings.

## Security Impact Gate

No credentials, network actions, or shell interpolation are in scope. Work is
limited to local flat-file reads/edits, local git refs/branches/tags, local
binary builds, local comparator artifact generation, and local tests.
