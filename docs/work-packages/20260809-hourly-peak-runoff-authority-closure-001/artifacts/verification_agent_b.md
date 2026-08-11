# Independent Terminal Verification B — Reopened Closure

Status: `executed`

Evidence class: `Static + Ran`

Verdict: `PASS`

Implementation/contract/runtime identity:
`33831787b7029b28b0716c8458f08a11899db446`.

Reopened ADR/source-guard identity:
`669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`.

Review/editorial and exact-source full-gate identity:
`a8a96498ee909c4305fbc0a4db562b72e45efd2b`.

Verifier independence: I did not read `verification_agent_a.md` and did not
search a path set containing it. Concurrent successor-package edits were
ignored but preserved. I initially questioned the exact-source provenance of
the warm full-gate target; the executor supplied the detached-worktree
location, and I independently confirmed the Git identity and compile/run-log
chain described below. No unresolved independence or provenance issue remains.

## Static Verification

- `669269ee4` amends ADR-0036 so the native WB16/public peak is the maximum
  modeled hourly mean and confines scalar/triangular treatment to explicit
  legacy-shard compatibility. Its integration guard requires the reconciled
  formulas and rejects all three former contradictory statements. The change
  adds no runtime, SC-* contract, schema, serialization, or frozen-input edit.
- Both hydrology/science re-reviews and the Rust correctness/QA reviews PASS
  the reopened authority/test identity. Descendant `a8a96498` resolves the
  sole LOW editorial fragment without changing that semantic target. Rust QA's
  remaining suggestions are explicitly maintainability debt, not closure
  blockers.
- The detached gate checkout remains at exact `a8a96498`. Its only untracked
  entries are the declared `.venv` symlink and one uniquely PID-suffixed stale
  tamper fixture from a non-admitted disk-full attempt. The failed build log
  `reopen-20260810T093947-full.log` records compilation from that detached path
  into `/home/workdir/openwepp-task-a8a96498-target2/full`; runlog entries
  16--18 and 31--33 bind the same target to the later successful receipt. Warm
  reuse therefore explains the successful log's 0.35-second build phase.
- The admitted reopened full receipt starts 2,346 tests and ends 2,346/2,346
  PASS, 33 ordinary skips, 46 slow, run ID
  `64cd5e97-d253-4da1-a3cf-3c4e16f83d22`, in 8,193.187 seconds. The retained
  quick/full inventories contain 2,297/2,346 identities; `comm -23` is empty,
  so the admitted full run executes every quick-selected identity.
- The interrupted `nextest-full-669269ee4.log`, three failed quick attempts,
  exit-137 full attempt, operator-stopped duplicate, and source-adjacent
  disk-full fixture failure are truthfully non-admitted. They do not replace or
  qualify the later complete receipt.
- Exact-source doctests, `cargo deny check`, formatting, authority anti-evasion,
  peak authority 4/4, and required-suite obligation guard 3/3 all have passing
  receipts. Unchanged production Clippy, the original exact-runtime full
  2,346/2,346 receipt (run ID
  `2a4b4f2c-d6c6-4bd6-a22f-e61bdb8f4576`), and the complete 280-baseline /
  1,088-mutation Topanga cohort remain correctly bound to `33831787b` and are
  labeled reused rather than newly run.
- The evidence-reuse boundary is valid: `33831787b..a8a96498` changes no
  production crate, Cargo manifest/lockfile, SC-* contract, output schema, or
  cohort input. The fresh full and focused receipts directly exercise the only
  reopened executable surface, the ADR source-reading guard.
- Exact write-set reconciliation PASS: 131 declared paths equal the union of
  the base-to-`a8a96498` predecessor diff and current predecessor lifecycle,
  backlog, catalog, and retained-log paths; both set differences are empty.
- Package, disposition, intent, handoff, backlog note/tracker, summary, and
  catalog consistently describe a closure candidate awaiting fresh terminal
  receipts. After both receipts are recorded, changing those lifecycle labels
  to terminal PASS/complete is an executor-owned archival step, not missing
  technical evidence.
- The accepted claim remains maximum hourly mean hillslope runoff flow. No
  instantaneous/subhourly peak, physical hydrograph duration, calibration,
  observed-flow validation, legacy parity, or routed watershed/channel-flow
  claim is admitted.

## Replayed Lightweight Checks

Ran from the shared checkout; no full workspace or Topanga workload was rerun.

```text
cargo fmt --all -- --check
  PASS
bash tools/release/check_authority_suite_antievasion.sh
  PASS
cargo nextest run --test peak_hourly_authority_contract
  PASS: 4/4, run ID 3c5b5fcb-e103-420c-aa74-a5f150068e98
cargo nextest run --test auth11_required_suite_obligation_guards_contract
  PASS: 3/3, run ID 37f9f135-7677-40fe-86f3-6908cf5129f5
markdown-doc lint --path <predecessor package> --format plain
  PASS: 28 files, 0 errors, 0 warnings
markdown-doc lint --path <ADR, backlog note/tracker, catalog> --format plain
  PASS: each target, 0 errors, 0 warnings
git diff --check -- <predecessor package, backlog note/tracker, catalog>
  PASS
jq -e . artifacts/command-log.json artifacts/summary.json
  PASS
wc -l artifacts/quick-list.sorted artifacts/full-list.sorted \
  artifacts/quick-only.identities
  PASS: 2297 / 2346 / 0
comm -23 artifacts/quick-list.sorted artifacts/full-list.sorted
  PASS: no output
git -C /tmp/openwepp-clean-a8a96498... rev-parse HEAD
  PASS: a8a96498ee909c4305fbc0a4db562b72e45efd2b
```

No closure-blocking finding remains. The reopened predecessor is eligible for
formal PASS closure once both independent terminal receipts are recorded and
the candidate lifecycle labels are archived.
