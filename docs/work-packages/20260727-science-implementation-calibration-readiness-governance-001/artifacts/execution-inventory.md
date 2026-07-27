# Terminal Execution Inventory

Evidence class: `Ran + Static`

Baseline: `5e3203a7`

Governance write set:

- `AGENTS.md`
- `docs/decisions/README.md`
- `docs/decisions/0042-science-implementation-and-calibration-readiness.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/science-contract-spec.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/work-packages/README.md`
- this package subtree.

Terminal commands:

```text
.venv/bin/python <package>/tools/validate_write_set.py
markdown-doc lint --path <package>
markdown-doc lint --path docs/decisions
markdown-doc lint --path docs/specifications
markdown-doc lint --path docs/standards
git diff --check
test "$(wc -l < AGENTS.md)" -le 160
```

No production code, test, fixture, canonical `SC-*` contract, dataset, or
calibration result is in the governance diff.
