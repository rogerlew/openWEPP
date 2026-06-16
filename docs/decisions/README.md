# Architecture Decision Records

Each ADR documents a decision that constrains future work. Format follows the lightweight Michael Nygard ADR convention.

| ID | Title | Status |
|---|---|---|
| [0001](0001-license-cc0.md) | License is CC0-1.0 | Superseded by 0015 |
| [0002](0002-clean-room-model.md) | Clean-room model is kernel-mirror port | Superseded by 0011 |
| [0003](0003-parity-semantic-not-bit.md) | Parity target is semantic, not bit-for-bit | Accepted |
| [0004](0004-subprocess-hillslope-orchestration.md) | Hillslope orchestration is subprocess-per-hillslope | Accepted |
| [0005](0005-parquet-via-wepppyo3-interchange.md) | Parquet schemas inherit from wepppy / wepppyo3 interchange | Superseded by 0019 |
| [0006](0006-three-binaries-incl-replay.md) | Three production binaries including replay | Accepted (amended by 0020) |
| [0007](0007-openwepp-runner-and-release-governance.md) | openWEPP owns runner boundary and release metadata contract | Accepted |
| [0008](0008-routine-lifecycle-and-replacement.md) | Routine lifecycle states and replacement catalog | Proposed |
| [0009](0009-network-node-contract-and-extensibility.md) | Network node contract and extensibility policy | Proposed |
| [0010](0010-non-clean-room-direct-port-policy.md) | Provenance model is explicitly non-clean-room direct port | Superseded by 0011 |
| [0011](0011-architecture-first-top-down-science-contracts.md) | Architecture-first delivery with top-down science contracts | Accepted |
| [0012](0012-legacy-wepp-260430-baseline-anchor.md) | Legacy provenance/comparator baseline is pinned to wepp_260430 hotfix snapshot | Accepted |
| [0013](0013-climate-forcing-ownership-boundary.md) | Climate forcing ownership boundary across hillslope and watershed surfaces | Accepted |
| [0014](0014-snow-drift-routine-non-implementation.md) | Do not implement legacy snow drift routine (`sndrft.for`) | Accepted |
| [0015](0015-relicense-to-apache-2.md) | Relicense openWEPP to Apache-2.0 (supersedes ADR-0001) | Accepted |
| [0016](0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md) | Promote wepp_260430 + negative-melt fix as canonical comparator; abandon kernel rewrite | Accepted (amended by 0017) |
| [0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) | Re-pin operational distrust — the fixed comparator is a flag, not a target | Accepted (operationalized by 0018) |
| [0018](0018-defect-closure-execplans-conversion-rule.md) | Defect-Closure ExecPlans — diagnosis must convert to correction | Accepted |
| [0019](0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md) | openWEPP owns its output surface; wepppyo3 interchange stays wepp-legacy-only | Accepted (supersedes 0005) |
| [0020](0020-totalwatsed3-dedicated-output-aggregation-cli.md) | totalwatsed3 is a dedicated output-aggregation CLI | Accepted (amends 0006) |
| [0021](0021-module-coverage-closure-thresholds.md) | Module coverage and complexity-risk closure thresholds are binding (90% science / 85% glue region+line; per-function CRAP ≤ 30; obligation binding non-waivable) | Accepted |
| [0022](0022-indexed-runtime-surface-representation.md) | Indexed runtime-surface representation | Accepted |

## ADR template

Use this shape for new ADRs:

- **Status** — Proposed / Accepted / Superseded by ADR-NNNN
- **Date** — YYYY-MM-DD UTC
- **Deciders** — names

Then three sections: **Context**, **Decision**, **Consequences**.

ADRs are short. If a decision needs more than ~1 page of justification, it belongs in a work-package artifact and the ADR cites it.
