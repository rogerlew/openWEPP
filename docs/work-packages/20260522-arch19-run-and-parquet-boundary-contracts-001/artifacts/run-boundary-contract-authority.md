# Run Boundary Contract Authority

Static: `.run` boundary authority authored from openWEPP contracts, ADRs, and implemented adapter/parser seams.
Ran: none.
Status: `complete-with-hold`.
Boundary ID: `openwepp.boundary.run.v1`.

## Scope

This contract defines top-level run-boundary authority for openWEPP orchestration
inputs and compatibility behavior.

In scope:
- run-level control-plane intent (engine selection, run context, compatibility policy),
- required input-surface closure requirements for hillslope/watershed execution,
- strict/compat sidecar boundary behavior,
- ownership map from run boundary to parser/runtime consumers.

Out of scope:
- full canonical `.run` text grammar authoring,
- implementation of a dedicated `.run` parser module,
- runtime execution code changes.

## Authority Stack

| precedence | authority | role in this contract |
|---|---|---|
| 1 | `/home/workdir/openWEPP/docs/contracts/README.md` | states openWEPP-owned `.run` contract intent and strict no-fallback posture |
| 2 | `/home/workdir/openWEPP/docs/contracts/openwepp-runner-contract.md` | defines explicit engine selector requirements (`legacy_wepp`, `openwepp`) |
| 3 | `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md` | ratifies explicit legacy `.run` + sidecar compatibility bridge with hard-fail rules |
| 4 | `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/sidecar.rs` | implemented strict/compat sidecar normalization contract and typed error/warning codes |
| 5 | `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md` | requires `.run` cross-file consistency closure and boundary-export mapping |

External sources inform implementation detail but do not override openWEPP
boundary authority statements.

## Canonical Boundary Statements

| clause_id | authority statement |
|---|---|
| `RUN-B-001` | Run boundary must carry explicit engine selector; missing selector is a typed configuration failure. |
| `RUN-B-002` | Run boundary must resolve to one unambiguous execution context (`hillslope`, `watershed`, or `replay`) before parser/runtime dispatch. |
| `RUN-B-003` | Compatibility behavior is explicit and policy-gated; strict mode rejects alias-only or unknown sidecars. |
| `RUN-B-004` | Required primary parser surfaces (`.cli`, `.sol`, `.man`, `.slp`, watershed files, and contracted sidecars) must be declared or derivable from run boundary intent. |
| `RUN-B-005` | No silent fallback/defaulting is allowed for missing required sidecars or ambiguous mixed input modes. |
| `RUN-B-006` | Run boundary must preserve canonical WEPP symbol continuity by explicit alias mapping when runtime/API names differ. |

## Minimum Canonical Run Control Plane (v1)

| canonical control field | boundary/API alias examples | consumer surfaces | source |
|---|---|---|---|
| `engine_selector` | `engine`, `runner.engine_selector` | runner launch boundary | `openwepp-runner-contract.md` |
| `run_context` | `tc.mode.run_context`, `lcwb.mode.run_context` | parser applicability and runtime adapter gating | `SC-INFILE-TC-001`, `SC-INFILE-LCWB-001` |
| `compatibility_policy` | `strict`, `compatibility` | sidecar/HBP adapter normalization | `openwepp-legacy-bridge` contracts |
| `primary_surface_set` | contracted input-surface IDs | parser contract closure checks | `input-surface-registry.md` |
| `sidecar_contract_set` | `SidecarContract[]` + discovery set | sidecar binding adapter | `sidecar.rs` |

## Strict vs Compatibility Policy

| policy | required behavior | forbidden behavior |
|---|---|---|
| `strict` | canonical sidecar names only; unknown files and alias-only matches are typed errors | implicit alias acceptance, implicit unknown-file ignore |
| `compat` | configured legacy aliases may bind with deterministic warning IDs; unknowns may be ignored with warnings | silent acceptance without warning IDs, fallback default synthesis |

## Canonical Symbol Continuity and Alias Mapping

| canonical symbol intent | boundary alias in current surfaces | note |
|---|---|---|
| `run_context` | `TcRunContext`, `LcwbRunContext` | parser applicability is explicit, not inferred |
| `ui_run` mode sentinel | `wepp_ui.ui_run` / `ui_run_requested` | sidecar-driven mode choice remains explicit |
| `hillslope_id` pass identity | `H<hillslope_id>.hbp` naming family | pass-family identity must stay deterministic |

## Ownership and Closure Map

| surface | owner | closure status |
|---|---|---|
| run control-plane contract text | openWEPP contract authors (ARCH19) | `closed` for governance-level authority |
| full `.run` grammar/spec | parser-contract/spec owners (follow-on) | `open` |
| `.run` parser implementation | `openwepp-input-contract` owners (follow-on) | `open` |
| parser-to-runtime run ingestion wiring | orchestrator owners (follow-on) | `open` |

## HOLD Register

| hold_id | ambiguity/gap | impact | required closure owner |
|---|---|---|---|
| `RUN-HOLD-001` | Canonical `.run` text grammar and versioned schema are not yet authored in `SC-INFILE-RUN-*` + spec files. | top-level boundary not fully promotable | parser-contract/spec authors |
| `RUN-HOLD-002` | No implemented `openwepp-input-contract` `.run` parser surface exists in current codebase. | run contract not executable end-to-end | input-contract implementation owners |
| `RUN-HOLD-003` | Input-surface registry currently omits explicit `.run` entry/disposition. | registry completeness requirement not met for `.run` | parser governance owners |

Promotion state for this boundary remains `HOLD` until all `RUN-HOLD-*` items are
closed or explicitly risk-accepted by disposition.
