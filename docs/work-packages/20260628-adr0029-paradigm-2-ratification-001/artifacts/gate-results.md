# Gate Results

Evidence class: Static + Ran.

Ran:

- `git diff --check`
  - Result: pass.
- `markdown-doc lint --path docs/decisions/0028-observed-data-admission-authority.md --path docs/decisions/0029-commit-paradigm-2-multilayer-snow.md --path docs/decisions/README.md --path docs/work-packages/README.md --path docs/work-packages/20260628-snow-density-paradigm-assessment-001/artifacts/adr-candidate-snow-density-paradigm.md --path docs/work-packages/20260628-adr0029-paradigm-2-ratification-001`
  - Result: pass, `14` files validated, `0` errors, `0` warnings.
- `markdown-doc validate --path docs/decisions/0028-observed-data-admission-authority.md --path docs/decisions/0029-commit-paradigm-2-multilayer-snow.md --path docs/decisions/README.md --path docs/work-packages/README.md --path docs/work-packages/20260628-snow-density-paradigm-assessment-001/artifacts/adr-candidate-snow-density-paradigm.md --path docs/work-packages/20260628-adr0029-paradigm-2-ratification-001`
  - Result: pass, `14` files validated, `0` errors.

No cargo/runtime gate was run for this package because the write set is docs-only
and no code, science-contract, fixture, schema, or runtime behavior changed.
