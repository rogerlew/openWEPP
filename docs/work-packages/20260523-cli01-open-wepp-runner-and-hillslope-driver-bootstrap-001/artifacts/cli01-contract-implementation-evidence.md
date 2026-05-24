# CLI01 Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Static
- Updated canonical runner authority in `docs/contracts/openwepp-runner-contract.md` with:
  - explicit CLI01 command surface (`run-hillslope`, `release lint`),
  - required launch/release validation behaviors,
  - stable typed runner failure IDs (`RUNNER-E-001..006`).
- Updated canonical hillslope CLI specification in
  `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` with:
  - CLI01 required sidecar IDs and file bindings,
  - explicit `.run` posture for this revision,
  - manifest schema ID (`openwepp-hillslope-run-manifest-v1`) and determinism requirements,
  - additional invariant for required sidecar resolution.
- Updated release-sidecar contract in
  `docs/contracts/openwepp-binary-release-contract.md` with required
  `validation` object fields (`schema_valid`, `release_lint_level`,
  `validated_utc`).

## Ran
- None in this phase.
