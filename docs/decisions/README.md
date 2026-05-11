# Architecture Decision Records

Each ADR documents a decision that constrains future work. Format follows the lightweight Michael Nygard ADR convention.

| ID | Title | Status |
|---|---|---|
| [0001](0001-license-cc0.md) | License is CC0-1.0 | Accepted |
| [0002](0002-clean-room-model.md) | Clean-room model is kernel-mirror port | Accepted |
| [0003](0003-parity-semantic-not-bit.md) | Parity target is semantic, not bit-for-bit | Accepted |
| [0004](0004-subprocess-hillslope-orchestration.md) | Hillslope orchestration is subprocess-per-hillslope | Accepted |
| [0005](0005-parquet-via-wepppyo3-interchange.md) | Parquet schemas inherit from wepppy / wepppyo3 interchange | Accepted |
| [0006](0006-three-binaries-incl-replay.md) | Three production binaries including replay | Accepted |
| [0007](0007-openwepp-runner-and-release-governance.md) | openWEPP owns runner boundary and release metadata contract | Accepted |
| [0008](0008-routine-lifecycle-and-replacement.md) | Routine lifecycle states and replacement catalog | Proposed |
| [0009](0009-network-node-contract-and-extensibility.md) | Network node contract and extensibility policy | Proposed |

## ADR template

Use this shape for new ADRs:

- **Status** — Proposed / Accepted / Superseded by ADR-NNNN
- **Date** — YYYY-MM-DD UTC
- **Deciders** — names

Then three sections: **Context**, **Decision**, **Consequences**.

ADRs are short. If a decision needs more than ~1 page of justification, it belongs in a work-package artifact and the ADR cites it.
