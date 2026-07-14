# Independent Review A: Adjudicated CRAP Closure Gate

Evidence class: **Static + Ran**

Initial recommendation: **HOLD**

The terminal census reproduces the intended CQR semantics (`2` raw, `2`
adjudicated, `0` actionable), and the focused unit suite passes `8/8`.
However, four anti-evasion and audit-lifecycle defects remain closure-blocking.

## Findings

### A-GATE-001 — High, blocking: the fresh report is not bound to one immutable source snapshot

`run_adjudicated_crap_gate.sh` collects coverage and CRAP over a long-running
workspace build, then the checker reads the live worktree and Git status only
after collection
([run_adjudicated_crap_gate.sh](../../../../tools/release/run_adjudicated_crap_gate.sh):94,
[check_adjudicated_crap.py](../../../../tools/release/check_adjudicated_crap.py):568).
There is no pre/post production-source manifest, worktree fingerprint, or lock.
An uncommitted production edit made after compilation begins can therefore be
absent from `workspace-crap.json` while the terminal report still records the
post-edit worktree as its source state and may return `PASS`.

This is not hypothetical in the package history: the first delegated capture
overlapped the growth extraction and had to be rejected manually by comparing
timestamps. The checker protects the two adjudicated host files with hashes,
but it does not bind ordinary actionable functions to the measured source
snapshot. That leaves the principal closure set vulnerable to a concurrent
agent or human edit.

Required disposition: capture and compare a deterministic manifest of every
production Rust source (plus HEAD/index/worktree provenance) before and after
fresh measurement, fail when it changes, and record the manifest hash in both
reports. Add a test that simulates a source mutation between the two snapshots.

### A-GATE-002 — High, blocking: a package-local registry can manufacture an adjudication

The closure driver exposes `--adjudications <registry.json>` and passes that
arbitrary path directly to the checker
([run_adjudicated_crap_gate.sh](../../../../tools/release/run_adjudicated_crap_gate.sh):7,
[run_adjudicated_crap_gate.sh](../../../../tools/release/run_adjudicated_crap_gate.sh):48).
The checker requires authority path *strings* to match and evidence files to
exist, but does not prove that `adjudicated_at_commit` resolves, that the file
hash existed at that commit, or that either review artifact actually accepts
the registered symbol/classification/hash
([check_adjudicated_crap.py](../../../../tools/release/check_adjudicated_crap.py):238,
[check_adjudicated_crap.py](../../../../tools/release/check_adjudicated_crap.py):253).
The focused fixture demonstrates the weakness: a nonexistent all-`a` commit
and three one-line placeholder evidence files are sufficient for a synthetic
row to pass
([test_adjudicated_crap_gate.py](../../../../tests/python/test_adjudicated_crap_gate.py):47,
[test_adjudicated_crap_gate.py](../../../../tests/python/test_adjudicated_crap_gate.py):71).

This permits exactly the inline/package-local exception forbidden by ADR-0021:
point the closure command at a substitute registry, reuse any two existing
Markdown paths, and register the current file hash and CC.

Required disposition: closure mode must resolve and require the canonical
`tools/release/adjudicated_crap_exceptions.json`. If alternate registries remain
useful for diagnostics, they must produce an explicitly non-closure result.
Also validate commit existence and source-at-commit hash, and bind review
evidence to the exact id, file, symbol, classification, hash, and accepted
disposition. Add direct rejection tests for a substitute registry, nonexistent
commit, unrelated review files, and evidence that does not name the row.

### A-GATE-003 — Medium, blocking: failure and retained-artifact runs are not auditable fail-closed artifacts

The output directory is reused without clearing or run identity. With
`set -e`, any version, coverage, CRAP, or checker failure exits before the
checksum manifest is rewritten
([run_adjudicated_crap_gate.sh](../../../../tools/release/run_adjudicated_crap_gate.sh):77,
[run_adjudicated_crap_gate.sh](../../../../tools/release/run_adjudicated_crap_gate.sh):127).
A stale prior `PASS` report and checksum file can consequently survive a failed
rerun. In GitHub Actions, `release_dir` is written to step output only after the
release command succeeds, and the upload step has no `always()` condition, so
the artifacts needed to diagnose a blocking gate are not uploaded
([release-gates.yml](../../../../.github/workflows/release-gates.yml):98,
[release-gates.yml](../../../../.github/workflows/release-gates.yml):133).

The same report schema also does not record whether the driver generated fresh
LCOV/CRAP or accepted `--crap-json`. A retained file can produce indistinguishable
`PASS` output even though package governance says it cannot close current
implementation work. The checker calls a report "complete" after finding only
a nonempty production array and the two adjudicated symbols
([check_adjudicated_crap.py](../../../../tools/release/check_adjudicated_crap.py):141,
[check_adjudicated_crap.py](../../../../tools/release/check_adjudicated_crap.py):423);
a truncated synthetic report containing those symbols can therefore pass.

Required disposition: start each run from a clean/run-unique artifact set,
write a failure manifest and hashes on every exit path, publish CI artifacts
with `if: always()` after setting the output path before gate execution, and
record a machine-readable evidence mode. Retained mode must be visibly
assessment-only (not closure `PASS`), and completeness needs a current workspace
census/snapshot check rather than only two registered-symbol sentinels.

### A-GATE-004 — Medium, blocking: deleted production files are omitted from touched-file reporting

Touched discovery uses `--diff-filter=ACMR`, excluding `D`
([check_adjudicated_crap.py](../../../../tools/release/check_adjudicated_crap.py):340).
Deleting a production Rust file is an implementation touch, yet the generated
report claims to list every touched production file. The global actionable
ratchet still protects surviving functions, but `ACRAP-004` and ADR-0021's
audit statement are not met for deletion packages.

Required disposition: include deleted production paths (and make rename
old/new treatment explicit) in the touched-file inventory, with focused tests
for deletion and rename cases.

## Confirmed Correct Surfaces

- Ran: `.venv/bin/python -m unittest -v
  tests.python.test_adjudicated_crap_gate` passed `8/8`.
- Ran: independent checker execution over
  `/tmp/openwepp-acrap-terminal-20260713/workspace-crap.json` returned
  `raw=2 adjudicated=2 actionable=0 touched_files=1`.
- Ran: independent application of the campaign's literal jq filter and exact
  tuple deduplication produced only `MeteorologyError::fmt` and
  `SymbolAliasRegistryError::fmt`.
- Static: strict `row["crap"] > 30`, the six-field deduplication key, source
  hash/CC matching, global actionable blocking, and stale/missing adjudication
  failure are implemented as intended.
- Static: the canonical registry contains only the two campaign-approved
  `R-OBSERVABILITY` rows and their current whole-file hashes.

## Recommendation

Keep the gate package `ACTIVE/HOLD` until A-GATE-001 through A-GATE-004 are
accepted or otherwise explicitly dispositioned, fixed, and independently
reverified. No source edits were made by Reviewer A.
