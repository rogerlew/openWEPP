# Runner Subsystem (`RUNNER`)

Canonical specification home for openWEPP runner and CLI execution boundaries.

## Scope

This subsystem defines:
- openWEPP CLI invocation boundaries consumed by in-repo `open_wepp_runner`
  and wepppy.
- Declarative `.run` contract posture for hillslope execution with required
  core inputs, explicit metric-only units, optional sidecar overrides,
  required pass/loss outputs, and configurable optional parquet output paths.
- Required execution outputs and run-provenance artifacts.
- Binary release metadata sidecar requirements for build artifacts.

## Canonical Files

- [openwepp-hillslope-cli-specification.md](openwepp-hillslope-cli-specification.md)
- [../../../contracts/openwepp-hillslope-runfile-contract.md](../../../contracts/openwepp-hillslope-runfile-contract.md)

## Provenance

Initial canonicalization anchors:
- [docs/work-packages/20260511-openwepp-runner-bootstrap/](../../../work-packages/20260511-openwepp-runner-bootstrap/)
- [docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md](../../../work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md)

Promotion disposition:
- pending CLI02 package disposition artifact
