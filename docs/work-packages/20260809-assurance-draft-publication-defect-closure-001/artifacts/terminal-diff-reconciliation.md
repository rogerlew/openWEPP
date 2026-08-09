# Terminal Diff Reconciliation

Evidence class: `Static + Ran`

## Intended Versus Actual Scope

- Production assurance Rust: unchanged; diagnosis proved no production defect.
- Assurance integration test: strengthened only in
  `draft_subject_root_is_stable_but_cannot_publish`.
- Package-local artifacts: scaffold, diagnosis, gates, reviews, disposition,
  and prompt lifecycle.
- Authorized downstream lifecycle: lifted the vegetation authority-reframe
  package's unrelated full-workspace hold and updated roadmap, work-package
  catalog, backlog note, and tracker consistently.
- Unrelated pre-existing/user changes: preserved and not reverted.

## Protected Invariants

- Root confinement remains prior to report-context/lifecycle loading.
- Production and test trust domains remain separate.
- DRAFT cannot publish.
- Rejection leaves the entire public byte tree unchanged and creates no
  snapshot or receipt.
- No report lifecycle, approval, transfer, or publication event was performed.

## Validation Identity

- HEAD: `4237552aa8dbc84a8baeff800014b23c7e75be9f`.
- Reviewed assurance test blob:
  `07e65f289049cfa6a96617a9922f70a06d8f5165`.
- Full-workspace log SHA-256:
  `d125d5ff4c5050e5068ac07bd698aa07ae2118ce46a204e1dfc679e159d9730d`.
- Full workspace: 2,325/2,325 selected passed; 33 declared skips; 55 slow.

Post-gate changes are documentation-only evidence, disposition, and lifecycle
updates. No Rust or science-contract byte changed after the terminal full run.

Dual terminal verification inspected the final lifecycle diff. Two stale
present-tense hold/conditional-review findings were accepted, fixed, and
rechecked. Verifiers A and B both record `PASS` with no open finding.
