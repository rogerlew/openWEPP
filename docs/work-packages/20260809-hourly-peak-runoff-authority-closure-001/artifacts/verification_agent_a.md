# Independent Terminal Verification A — Reopened Closure

Status: `complete`

Evidence class: `Static + Ran lightweight verification`

Semantic authority/test identity:
`669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`

Review/editorial reconciliation identity:
`a8a96498ee909c4305fbc0a4db562b72e45efd2b`

Implementation/contract/runtime identity:
`33831787b7029b28b0716c8458f08a11899db446`

Verdict: `PASS`

I did not read the other terminal verifier's artifact and did not rerun the
full workspace or Topanga cohort. The unrelated dirty 20260810 successor
package was excluded from predecessor diff reconciliation and preserved.

## Authority And Review Closure

Static inspection confirms that `669269ee4` removes ADR-0036's contradictory
independent analytical WB16 peak and makes the native peak the maximum modeled
hourly mean. It preserves only explicit legacy-shard compatibility fallback,
matches the active SC-WATBAL/SC-SED unit and area boundary, and adds positive
and negative source assertions to `peak_hourly_authority_contract`. The narrow
increment changes no production Rust, canonical SC-* contract, serialization
schema, release binary, or frozen Topanga input.

Both independent science reviews, Rust correctness review, and Rust QA pass
the reopened authority/guard increment. The sole LOW editorial fragment is
resolved by `a8a96498`; all remaining QA suggestions are explicitly
non-blocking maintainability debt. Lifecycle evidence now binds `669269ee4`,
records the proportional reviews and guard receipt, and explicitly dispositions
reuse of unchanged runtime and campaign evidence. No open implementation,
authority, review, or gate finding remains.

## Workspace And Attempt Disposition

The admitted reopened exact-source full receipt is
`reopen-20260810T121200-full.log`: run ID
`64cd5e97-d253-4da1-a3cf-3c4e16f83d22`, 2,346/2,346 passed, 46 slow, 33
ordinary skips, 8,193.187 seconds. The retained inventory has 2,297 quick and
2,346 full identities with zero quick-only identities, so every quick-selected
test executed in the admitted full run. Exact-source doctests, `cargo deny`,
format, authority anti-evasion, the 4/4 peak authority target, and the 3/3
required-suite obligation guard also pass.

All other reopened attempts are preserved as non-admitted. They include the
interrupted `nextest-full-669269ee4.log`, `/tmp` capacity/setup failures,
quick-profile assurance interruption, an exit-137 orchestration attempt, the
operator-stopped duplicate, and the late source-adjacent fixture-write failure
caused by exhausted `/tmp`. None is relabeled as passing or used for semantic
admission; the unchanged exact-source full command subsequently passed.

Evidence reuse is legitimate and bounded. The complete runtime receipt remains
bound to `33831787b` (2,346/2,346), and the frozen Topanga receipt remains bound
to its original release binary and plan (280 baselines, 1,088/1,088 mutations,
1,913,199 event pairs, no unexplained volume-stable peak discontinuity). Since
the reopen changes only ADR prose, its source-reading guard, and lifecycle
evidence, those receipts are reused rather than described as newly run.

## Lifecycle, Manifest, And Ran Checks

The package, disposition, summary, intent, finding disposition, worker handoff,
backlog, tracker, and catalog consistently describe a closure candidate whose
only remaining step was fresh dual terminal verification. The owned-file
manifest exactly matches the base-to-current predecessor path set, including
all untracked reopened receipts, after excluding the separately owned 20260810
successor tree. The manifest has no actual-only or manifest-only path.

Ran from `/home/workdir/openWEPP` on the terminal lifecycle worktree:

```text
cargo nextest run --test peak_hourly_authority_contract
PASS: 4/4; run ID e5b3ef8f-5a75-4147-9414-98a93338db13

cargo fmt --all -- --check
PASS

bash tools/release/check_authority_suite_antievasion.sh
PASS

markdown-doc lint --path <predecessor-package> --format plain
PASS: 28 files, 0 errors, 0 warnings

markdown-doc lint --path <ADR-0036> --format plain
PASS: 1 file, 0 errors, 0 warnings

markdown-doc lint --path <promoted-backlog-note> --format plain
PASS: 1 file, 0 errors, 0 warnings

jq empty <command-log.json> <summary.json>
PASS

git diff --check
PASS
```

## Disposition

`PASS` — the reopened authority defect, source guard, reviews, exact-source
workspace evidence, non-admitted attempt record, bounded evidence reuse,
manifest, backlog/catalog state, and claim boundary are coherent. No unresolved
current-scope requirement prevents the parent from completing dual-verifier
reconciliation and changing the package lifecycle from closure candidate to
terminal PASS.
