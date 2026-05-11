# 20260511-openwepp-runner-bootstrap

## Scope

Bootstrap governance and contracts required for a clean engine boundary:

- `openwepp_runner` ownership and launch contract.
- Binary naming contract (`openwepp_YYMMDD*`).
- Mandatory binary sidecar contract and schema validation policy.
- Blocking release lint gate definition.

## Deliverables

1. Runner boundary contract document.
2. Binary release + sidecar contract document.
3. ADR accepting the boundary/release governance decision.
4. Bootstrap package artifacts and closeout notes.

## Dependencies

- Existing architecture ADRs (`0002`..`0006`).
- Upstream HBP contract from wepp-palimpsest / wepp-forest.

## Exit criteria

- Contracts are documented and referenced from `docs/contracts/README.md`.
- ADR is accepted and indexed in `docs/decisions/README.md`.
- Package folder contains prompts/artifacts placeholders for subsequent
  implementation tracking.

## Out of scope

- Implementing `openwepp_runner` code.
- Running migration actions against external GitHub repositories.
- Changing kernel physics or oracle harness behavior.
