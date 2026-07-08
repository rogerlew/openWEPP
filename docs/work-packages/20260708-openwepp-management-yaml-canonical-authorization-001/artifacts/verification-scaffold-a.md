# Scaffold Verification A

Status: PASS.

Evidence mode: Static and local command execution.

Checks:

- Package scaffold exists.
- Draft `management-yaml.spec.md` exists.
- Extension policy is recorded for producer and consumer paths.
- `artifacts/crate-ownership-assessment.md` records the recommended long-term
  crate ownership path.
- `infile-management-yaml` is registered as a planned input surface.
- Roadmap and package catalog point to the package.
- Active execution prompt exists.
- `markdown-doc lint` passed for the package/spec/index write set.
- `git diff --check` passed.

Commands:

```bash
markdown-doc lint --path docs/work-packages/20260708-landuse-migration-cli-spec-implementation-001 --path docs/work-packages/20260708-openwepp-management-yaml-canonical-authorization-001 --path docs/specifications/wepp-input-files/specs/landuse-migration-cli.spec.md --path docs/specifications/wepp-input-files/specs/management-yaml.spec.md --path docs/specifications/wepp-input-files/specs/README.md --path docs/specifications/wepp-input-files/README.md --path docs/specifications/wepp-input-files/input-surface-registry.md --path docs/ROADMAP.md --path docs/work-packages/README.md
git diff --check
```

Result:

- `markdown-doc lint`: 33 files validated, 0 errors, 0 warnings.
- `git diff --check`: no output.
