# Codex QA Verification

Static: read required governance/context files, package artifacts, current
`SC-OFEROUTE-001`, package-local proof artifacts, and current implementation
diffs. Ran: lightweight JSON consistency checks, tracked `git diff --check`,
and a direct trailing-whitespace scan over the package directory. Full
workspace gates were not run per review instruction.

Status: `QA-HOLD`

## Findings

### BLOCKING - Contract metadata still advertises rev 43

Path: `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`

The contract body and revision history claim rev 45 authority for the active
production `dx5` mesh policy, but the YAML front matter still says
`contract_version: 43` at line 7. The rev 45 body claims appear in the active
mesh policy rows and invariant text (`SC-OFEROUTE-001.md:127`,
`:232`, `:250`) and the revision-history row (`:586`).

This blocks closure of the package's contract/profile gate because tools or
reviewers reading canonical contract metadata will still see rev 43 while the
package claims rev 45 authorization.

Required disposition: update the canonical contract metadata to match the rev
45 body, and rerun/record the required contract/profile/BEI checks.

### BLOCKING - Closure artifacts and gate record are incomplete

Path:
`docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/package.md`

`package.md:141-151` requires `gate-results.md`, `review-*.md`,
`disposition.md`, `final-disposition.md`, and `worker-handoff.md`. The current
artifact directory has `verification-comparator.md` plus implementation,
contract, consumer-path, and evidence summaries, but it does not contain those
required closure/disposition/handoff artifacts. `package.md:157-172` also
requires focused tests, doc lint, contract/profile/BEI checks, `cargo fmt`,
`clippy -D warnings`, full nextest, and `cargo deny`; no `gate-results.md`
records them. `package.md:192` requires gates, review, verification, and
finding disposition before completion.

This blocks `EXECUTED-COMPLETE-DX5-PRODUCTION-MESH-POLICY` and leaves the
required follow-on handoff unclosed.

Required disposition: add a truthful `gate-results.md` with PASS/FAIL/BLOCKED
/NOT RUN classifications, complete review/finding disposition, final
disposition, and `worker-handoff.md`. Full workspace gates may be delegated or
recorded as not run only if the package is held rather than completed.

### LOW - Required-reading artifact is stale for rev 45

Path:
`docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/required-reading-map.md`

`required-reading-map.md:15` describes `SC-OFEROUTE-001` as "Rev-44 active mesh
policy" even though this package and the current contract body are rev 45
production-promotion work. This is not the primary closure blocker, but it is
artifact drift in the required-reading/provenance map.

### LOW - Package-local whitespace scan found one trailing-space line

Path:
`docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/verification-comparator.md`

`rg -n '[ \t]+$' docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001`
reported `verification-comparator.md:41`. Tracked `git diff --check` returned
clean, but the package is untracked, so that command does not cover these new
files until they are staged or otherwise checked directly.

## Verified Passing Evidence

- Promotion matrix JSON passes the lightweight invariant check:
  `status=DX5_PRODUCTION_RATIFIED_BY_EVIDENCE`, `row_count=21`,
  `blockers=0`, `missing_annual_count=0`, and `ratified_by_evidence=true`.
- Runtime evidence JSON passes the lightweight invariant check:
  `status=PASS`, `members=3`, `runs=12`, six identity comparisons, zero
  identity mismatches, zero mesh-policy assertion failures, and zero closure
  assertion failures.
- Active default/no-env evidence resolves only to `target_dx_m=5.0`.
- `default-dx5-evidence.md:22-33` records active default, explicit dx5,
  off-default, and off mesh-env-control runs for the three selected real-cohort
  members.
- `default-dx5-evidence.md:39-44` records active default-vs-explicit dx5 and
  off default-vs-mesh-env-control identity comparisons as `PASS` with zero
  mismatches.
- `verification-comparator.md:40-52` records closure thresholds and observed
  maxima; the observed active residuals are well below the stated package
  thresholds.
- `contract-disposition.md:37-49`, `implementation.md:66-72`, and
  `consumer-path-proof.md:81-89` record the shadow mesh decision as unchanged
  and separate from the rev45 active production default.
- `consumer-path-proof.md:6-31` gives a static consumer path from runner env
  resolution through `DirectLanedActiveConfig.mesh_policy` into the active
  executor, plus runtime no-env dx5 proof.
- `consumer-path-proof.md:51-79` records the DC01-disable/double-feed guard
  and routed-hydrograph erosion consumer path.

## Lightweight Commands

```text
jq -r '[...] | .[]' artifacts/rev44-promotion-matrix.json
jq -r '[...] | .[]' artifacts/default-dx5-evidence.json
jq -e '.status == "PASS" and ...' artifacts/default-dx5-evidence.json
jq -e '.ratified_by_evidence == true and ...' artifacts/rev44-promotion-matrix.json
jq -r '[.runs[] | select(.mode == "active_default_dx5") | ...] | unique' artifacts/default-dx5-evidence.json
git diff --check
rg -n '[ \t]+$' docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001
```

## Verification Status

The core runtime/provenance artifacts are internally consistent for dx5
promotion evidence, protected off/default identity, active default-vs-explicit
dx5 identity, closure residuals, unchanged shadow mesh policy, DC01
double-feed posture, and routed erosion consumer path.

The package is not ready for completion because canonical contract metadata is
still rev 43 and required gate/review/disposition/handoff artifacts are absent.
