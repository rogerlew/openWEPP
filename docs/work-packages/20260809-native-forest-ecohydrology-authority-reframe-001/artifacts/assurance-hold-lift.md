# Assurance Hold Lift

Evidence class: `Ran + governed reuse`

The exact lift condition recorded by this package is satisfied through
`20260809-assurance-draft-publication-defect-closure-001`.

## Diagnosis

The original disk-remediation command set
`TMPDIR=/home/workdir/openWEPP/target/task-tmp`. Assurance publication scratch
directories therefore became descendants of the repository. The publication
contract correctly rejected staging/repository root overlap before reading the
report lifecycle. No production confinement or lifecycle defect existed.

## Corrected Evidence

- Isolated DRAFT-publication case with external scratch: `PASS`.
- Reviewed test blob:
  `07e65f289049cfa6a96617a9922f70a06d8f5165`.
- Exact full workspace with external scratch: 2,325 passed, 0 failed, 33
  declared full-profile skips, 55 slow, 3,300.706 seconds.
- Canonical evidence:
  `../../20260809-assurance-draft-publication-defect-closure-001/artifacts/full-workspace-gate.md`.

## Disposition

Lift `executed-hold / exact-head full-workspace blocker` to
`complete / exact-head full-workspace pass`. This is a validation/lifecycle
lift only. It does not authorize production vegetation implementation,
defaults, calibration, validation, or cutover, and it does not reopen universal
site-value selection.
