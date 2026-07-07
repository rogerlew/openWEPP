# LANED-T3-AGG QA-M3 - Build-provenance guidance remainder

Status: **EXECUTED-COMPLETE** (2026-07-07). User-directed follow-on:
"scaffold and execute QA-M3 remainder".

Parent finding: `20260706-laned-router-t3-aggressive-deficit-carry-001`
`artifacts/review-disposition.md`, QA-M3. The T3-AGG package promoted the
stale-binary caution into the H2637 timing recipe, but deliberately left the
AGENTS.md durable-guidance remainder for Codex-owned follow-on work.

## Objective

Close the QA-M3 remainder by promoting the release-binary evidence build rule
into durable agent/workflow guidance so future timing, comparator, and H2637
evidence runs do not accidentally execute a stale `openwepp-cli-hill` or other
runner CLI binary.

## Scope

Included:

- Add durable release-binary provenance guidance to
  `docs/work-packages/AGENTS.md`.
- Add runner-CLI release-build guidance to `crates/AGENTS.md`.
- Add operator-facing local-CI command notes to `tools/local_ci/README.md`.
- Add package-local evidence, review, verification, disposition, and README
  catalog entry.

Excluded:

- Rust implementation changes.
- Science-contract changes.
- H2637 timing reruns, comparator reruns, or cargo workspace gates.
- Any change to external-authority suite posture, cohort fixtures, or required
  case bindings.

## Acceptance Criteria

- Durable guidance states that release-binary evidence must build the exact
  runner binary target, not rely on generic workspace `cargo build --release`.
- Guidance names the canonical runner command:
  `cargo build --release -p openwepp-runner --bins`.
- Guidance requires recording binary path, mtime/size or hash before using
  timing/comparator outputs as evidence.
- The local-CI README provides copyable commands for `stat` and `sha256sum`.
- Package evidence records QA-M3 closed and names the files that now carry the
  durable rule.
- `git diff --check` and scoped markdown lint pass.

## Intended Write Set

- `docs/work-packages/AGENTS.md`
- `crates/AGENTS.md`
- `tools/local_ci/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260707-laned-router-t3agg-qa-m3-build-provenance-guidance-001/`

## Security / Authority Impact

Docs/process-only. No runtime behavior, test fixture, required-case binding,
external-authority suite posture, or production output surface changes.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only review or verification subagents for this docs-only QA-M3 closure;
expected outputs are package-local review/verification artifacts; write access
is bounded to package artifacts unless an operator explicitly assigns an
implementation fix. No subagent was required for this small docs-only closure.
