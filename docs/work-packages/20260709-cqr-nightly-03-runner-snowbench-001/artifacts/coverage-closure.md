# Coverage Closure

Evidence label: Static.

Status: `QUEUED`

ADR-0021 tier: `glue` because the target is a diagnostic CLI argument
parsing/dispatch entrypoint, not a kernel math module.

Closure rule:

- If characterization tests are added or materially changed, record line and
  region coverage status for the target, per-function 75% region-floor status or
  disposition, and obligation-to-test binding before completion.
- If only module-local CLI parse tests are added, the obligation map is
  CLI/API behavior, with no new `SC-*` obligation unless contract-derived tests
  are edited.

Current status: pending implementation and after-coverage measurement.
