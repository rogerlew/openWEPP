# Runner Subsystem (`RUNNER`)

Canonical specification home for openWEPP runner and CLI execution boundaries.

## Scope

This subsystem defines:
- openWEPP CLI invocation boundaries consumed by in-repo `open_wepp_runner`
  and wepppy.
- Legacy-compatible run-directory sidecar discovery policy for current
  hillslope execution (`.run` does not enumerate sidecar paths in this
  version).
- Required execution outputs and run-provenance artifacts.
- Binary release metadata sidecar requirements for build artifacts.

## Canonical Files

- [openwepp-hillslope-cli-specification.md](openwepp-hillslope-cli-specification.md)

## Provenance

Initial canonicalization anchors:
- [docs/work-packages/20260511-openwepp-runner-bootstrap/](../../../work-packages/20260511-openwepp-runner-bootstrap/)
- [docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md](../../../work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md)

Promotion disposition:
- pending CLI01 package disposition artifact
