# Scaffold Verification A

Status: PASS.

Evidence mode: Static and local command execution.

Checks:

- Package scaffold exists.
- CLI specification draft exists.
- CLI specification now requires YAML-only output.
- CLI specification requires lowercase `.yaml` producer output and defaults
  flat `.man` inputs to `.man.yaml` when `--output` is omitted.
- CLI specification includes `--validate`.
- Roadmap, work-package catalog, input registry, and spec indexes include the
  amended package/spec dependencies.
- YAML authorization package exists and is named as an implementation closure
  dependency.
- Earlier read-only subagent review was recorded.
- Review and YAML amendment dispositions were incorporated.
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
