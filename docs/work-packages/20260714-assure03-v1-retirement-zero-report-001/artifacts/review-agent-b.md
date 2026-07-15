# ASSURE-03 Review B — Build, Release, And Security Integrity

Review class: internal coding-agent review; not external scientific peer review

Evidence class: Static + Ran

Verdict: **HOLD**

## Findings

### B-001 — High — Failed workflows upload candidate-named artifacts

The release workflow created the release directory before its explicit
preflight, then uploaded it as `openwepp-release-candidate-*` under `always()`.
A failed preflight or later gate could therefore emit a candidate-named
artifact.

Required correction: run preflight first, upload failure logs under a
non-candidate name, condition candidate upload on full success, and add a
workflow failure-route contract test.

### B-002 — High — Release preflight is not symlink-fail-closed

The marker check missed a dangling marker symlink, and retired-directory scans
missed nested symlinks. Both evasions returned success in temporary
reproductions.

Required correction: reject existing or symlink markers, recursively reject
symlink/special retired entries, and prove the real aggregate route leaves no
release directory for each case.

### B-003 — High — Snapshot-ID symlink can escape its root

Snapshot creation joined the safe lexical ID to the root, then followed an
existing target without rejecting a symlink or confining its canonical target.
A temporary reproduction confirmed an external snapshot through an ID symlink.

Required correction: reject symlinked targets, canonicalize/confine existing
targets, reject descendant symlinks before reading, and add both cases to the
integration test.

### B-004 — High — Initial assembly evidence is not release qualification

The workflow forced `--skip-stability` and did not depend on its separate
stability job. The initial heavy run also skipped stability and deleted its
temporary 28-file assembly tree, while the release runbook requires stability
and an archived evidence bundle.

Required correction: either produce and retain a conformant full candidate or
narrow the package claim to transition-route verification. A candidate-named
workflow upload must depend on successful stability.

### B-005 — Medium — Recovery actions disagree with terminal disposition

This independently duplicates A-001: eight deleted compiler modules were
misclassified as `preserve-or-revise`.

### B-006 — Medium — Gate metadata contradicts retained evidence

This independently duplicates A-002: queued/pending metadata contradicted the
initial heavy-run PASS record.

## Ran

- Focused Nextest passed 10/10 before remediation.
- Shell syntax, `git diff --check`, and the manifest recovery test passed.
- Retained CRAP evidence recorded zero actionable rows and no touched-file row
  above 30.

The reviewer made no workspace edits.
