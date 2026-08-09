# Gate Results

Evidence class: `Ran unless labeled Static`

| Gate | Result | Evidence |
|---|---|---|
| failing-first focused contract test | `PASS AS RED` | 10 ran; 7 passed and 3 intended assertions failed after compile was redirected from full root `/tmp`. |
| post-amendment focused contract test | `PASS` | 10 passed, 0 skipped. |
| package Markdown lint | `PASS` | 34 files, 0 errors, 0 warnings after review disposition. |
| contract Markdown lint | `PASS` | 1 file, 0 errors, 0 warnings. |
| successor Markdown lint | `PASS` | 34 files, 0 errors, 0 warnings before terminal artifacts. |
| roadmap/backlog/tracker/catalog Markdown lint | `PASS` | 4 files, each 0 errors and 0 warnings. |
| `cargo fmt --all -- --check` | `PASS` | no output. |
| `git diff --check` | `PASS` | no output after review disposition. |
| Critical full workspace, initial | `FAIL / ENVIRONMENT` | 2325 ran: 2006 passed, 319 failed, 33 skipped; failures were `StorageFull`/`os error 28`. See `full-workspace-nextest.log` and `full-workspace-nextest-summary.json`. |
| Critical full workspace, disk-remediated | `FAIL / STABLE UNRELATED ASSERTION` | Exact command progressed beyond the prior disk failures but repeatedly failed `assurance_v2_publication_contract::draft_subject_root_is_stable_but_cannot_publish`. The redundant retry was stopped after recurrence; see `full-workspace-nextest-rerun.log`. |
| isolated assurance replay | `FAIL / STABLE UNRELATED ASSERTION` | 1 ran, 0 passed, 1 failed, 36 skipped; assertion at `tests/integration/assurance_v2_publication_contract.rs:541` expected an error containing `DRAFT`. |
| authorized assurance closure, isolated external scratch | `PASS` | The prior replay used an in-repository `TMPDIR` and correctly failed confinement. With `/home/workdir/openwepp-task-tmp`, the DRAFT case passed and created no public/snapshot/receipt mutation. |
| Critical full workspace, authorized external-scratch rerun | `PASS` | 2,325 passed, 0 failed, 33 declared full-profile skips, 55 slow, 3,300.706 s. Reviewed assurance test blob `07e65f289049cfa6a96617a9922f70a06d8f5165`. Reused from `20260809-assurance-draft-publication-defect-closure-001/artifacts/full-workspace-gate.md`. |

Commands:

```text
TMPDIR=/home/workdir/openWEPP/target/task-tmp cargo nextest run --workspace --profile full
TMPDIR=/home/workdir/openWEPP/target/task-tmp cargo nextest run \
  --test assurance_v2_publication_contract \
  draft_subject_root_is_stable_but_cannot_publish --profile quick
```

The historical failed invocations above remain evidence. The separately
authorized closure identified their scratch-topology defect and supplied a
passing exact-workspace rerun, so the full-workspace requirement is now met.
No failure in the focused vegetation contract test was observed.

Prompt lifecycle: direct source/destination SHA-256 both equal
`6d435d7f9e63ebf81559bf3d16ea03ff2981eeca528d844fd5ee487ee0a62b5d`.
The canonical `markdown-doc mv --no-backup` command was stopped after 14 minutes
of CPU-bound reference scanning with no filesystem change; a direct same-filesystem
rename then preserved bytes, and no active-path reference was found. This is a
confirmed local docs-tool performance painpoint, not a science or lifecycle gap.
