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
| [0023](0023-array-authoritative-hot-path-state.md) | Array-authoritative hot-path state | Accepted (incremental application superseded by 0025; dense-authority principle retained) |
| [0024](0024-reference-implementation-intent-authority.md) | Reference-implementation intent can anchor empirical model authority | Accepted |
| [0025](0025-array-native-hillslope-day-frame.md) | Array-native HillslopeDayFrame hot-path runtime (comprehensive re-architecture; completes 0023, narrows 0022) | Accepted (ratified 2026-06-18) |
| [0026](0026-stateful-winter-column-sub-solver.md) | Stateful winter-column sub-solver for snow/frost | Accepted |
| [0027](0027-opt-in-physics-bulk-snow-model.md) | Opt-in physics-bulk snow model for snow-density remediation | Accepted |
| [0028](0028-observed-data-admission-authority.md) | Observed-data admission authority when scientific authority is lacking (domain-general; extends 0011, reaffirms 0017) | Accepted (ratified 2026-06-28) |
| [0029](0029-commit-paradigm-2-multilayer-snow.md) | Commit to Paradigm 2 — staged multilayer snow physics (foundation for frost insulation, winter water temperature, runoff dynamics; admitted under 0028, homed in 0026) | Accepted (ratified 2026-06-28) |
| [0030](0030-r7-terminal-contract-and-compatibility-runtime-deletion.md) | R7 terminal contract and compatibility runtime deletion | Accepted |
| [0031](0031-delete-compatibility-runtime-single-authority-terminal.md) | Delete the compatibility runtime - single-authority terminal state | Accepted |
| [0032](0032-watershed-runtime-ratification.md) | Watershed runtime entrypoint, job default, and canonical benchmark mode | Accepted |
| [0033](0033-ofe-by-ofe-overland-flow-routing.md) | OFE-by-OFE overland-flow routing (Papanicolaou 2018) | Accepted |
| [0034](0034-management-file-lanuse-input-authority.md) | Management-file `lanuse` input authority (first-class landuse modes) | Accepted |
| [0035](0035-hillslope-erosion-sediment-continuity-port.md) | Hillslope erosion sediment-continuity direct-runtime port | Accepted |
| [0036](0036-hydrograph-resolved-sediment-transport-and-routing.md) | Hydrograph-resolved sediment transport and channel routing | Accepted |

ADR-0025 was ratified 2026-06-18 and is the accepted hot-path runtime authority. ADR-0023's dense-authority
principle is retained; its incremental symbol/phase migration application is superseded — no further
writeback-only or materialization-retirement rungs. Execution is the staged `PERFDEEP0N` series.
ADR-0026 ratifies the coupled winter-column snow/frost sub-solver exception to ADR-0025's ordinary
pure-phase direct-frame model.
ADR-0027 authorizes only an opt-in `physics_bulk` snow-density remediation lane;
`legacy_wepp` remains default until a later package and contract amendment
ratify runtime promotion.
ADR-0028 adds a domain-general third authority tier: when established scientific
authority is insufficient to derive a contract and a defensible observed-data
corpus + forcing-robust rubric exist, a physically-defensible mechanism may be
admitted on measurable rubric improvement (no calibration to the set, comparators
stay flags, conservation non-negotiable). It sits below derivable contracts
(ADR-0011) and above reference comparators (ADR-0017); the snow/frost rubric is
the first instance. Ratified by
`docs/work-packages/20260628-adr0029-paradigm-2-ratification-001/`.
ADR-0029 commits to staged multilayer snow physics (Paradigm 2) after both the
bulk family and climate-class specialization failed to resolve the structural
densification-trajectory residual. It is the shared foundation for frost
insulation, winter water temperature, and runoff dynamics; admitted under ADR-0028,
homed in the ADR-0026 winter-column sub-solver (whose variable-layer Vec exception
de-risks it), opt-in and staged with the bulk default as rollback. Stage 0 (the
openwepp-meteorology surface energy balance) is done. Ratified by
`docs/work-packages/20260628-adr0029-paradigm-2-ratification-001/`.
ADR-0030 amends the R7 terminal contract after frost ratification/default
activation: compatibility frost bit-parity is no longer the acceptance target,
production direct mode must not silently fall back to compatibility, obsolete
transition modes may be deleted under no-regression/static-proof gates, and the
explicit `--compatibility-runtime` seam remains only as deprecated diagnostic
replay until a later full-deletion package.
ADR-0031 supersedes ADR-0030's seam-retention clause and authorizes deletion of
the explicit `--compatibility-runtime` seam with rollback by release/git
history. The ratification package removed the public selector but held before
full `scheduler.rs`/carrier deletion because the remaining symbol-keyed support
surface is still compiled and test-backed as a unit.
ADR-0032 ratifies the watershed runtime public entrypoint and benchmark posture:
the full watershed supervisor remains under `openwepp-cli-watershed`,
`--jobs` defaults to deterministic serial `1`, CPU scaling is explicit through
`--jobs N`, and canonical benchmark/ratification evidence uses
`strict-committed-fixture` mode with legacy sidecar discovery disabled.
ADR-0033 accepts the OFE-by-OFE routing representation and opt-in activation
policy for Papanicolaou-style hillslope overland-flow routing. Its ratification
does not authorize D4/D5 solver or cascade implementation; those stages remain
gated on authoring and ratifying `SC-OFEROUTE-001`.
ADR-0034 accepts that the management-file `lanuse` block — not the `.run` — is
the opt-in authority for first-class landuse-physics operands, quarantines
cropland-encoded forest/range fixtures as compatibility inputs, and disallows
inferring new-physics operands from legacy cropland fields without a bridge
contract. By the contract-boundary test (coherent invariants + multi-consumer +
distinct concern-layer) the input authority is governed by a **standalone
interface contract** (`docs/contracts/openwepp-management-lanuse-authority-contract.md`),
which routing/soil/canopy science contracts reference for provenance as concrete
operands are bound — not folded into a physics contract. Ratification covers the
authority model and `LANUSE-AUTH-1..6`, not a concrete `lanuse` operand schema;
WS-1 populates and promotes `openwepp-management-lanuse-v1`.
ADR-0035 accepts the direct-runtime port of the `SC-SED-001` hillslope erosion
sediment-continuity **source physics**, on the finding that the spatial
detachment/deposition solve never existed in openWEPP (only a reduced,
disabled Wave-1 detachment check plus multi-OFE routing). Because SC-SED-001
already specifies the full model, this is a contract-exists/implement port
(WS-2 `ksatadj` shape) with the legacy `.for` chain as source-intent authority
(ADR-0024), not a magnitude oracle (ADR-0017). Staged single-OFE Wave-1 first,
shadow-state + conservation-gated per increment; Claude Code executes as an
operator-authorized exception to the Codex-authors-code default. Unblocks the
held WS-3 sediment ordering law.

## ADR template

Use this shape for new ADRs:

- **Status** — Proposed / Accepted / Superseded by ADR-NNNN
- **Date** — YYYY-MM-DD UTC
- **Deciders** — names

Then three sections: **Context**, **Decision**, **Consequences**.

ADRs are short. If a decision needs more than ~1 page of justification, it belongs in a work-package artifact and the ADR cites it.
