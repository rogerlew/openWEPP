# Climate Parser and Architecture Integration Map

Status: `complete`
Evidence mode: `Ran + Static`

Static:
- Integration mapping synthesized from parser contract/spec docs, parser implementation, and current orchestrator seam architecture.

Ran:
- Executed line-level inspections for contract text, parser code, and orchestrator runtime-input seams.

## Parser Contract Linkage

### Canonical parser authority

1. Parser contract: `SC-INFILE-CLIMATE-001`.
2. Source grammar/spec authority: `climate-file.spec.md`.
3. Active registry entry: `infile-climate-cli` is `active`.

Evidence:
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/climate-file.spec.md`
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/input-surface-registry.md:17`

### Implemented parser behaviors (current)

| parser requirement | implementation status | evidence |
|---|---|---|
| `datver` allowlist (`0.0`, `4.0`, `4.3`, `5.3`) | implemented | `climate.rs:8`, `:334-342` |
| strict/compat `itemp=2` policy | implemented | `climate.rs:18-22`, `:358-361` |
| `ibrkpt` split between no-breakpoint and breakpoint records | implemented | `climate.rs:451-457`, `:532-697` |
| breakpoint cardinality guard | implemented (`<=50` strict; override in compat mode) | `climate.rs:9`, `:629-635` |
| date sequence, year coverage, monotone `pptcum` | implemented | `climate.rs:672-677`, `:720-761` |
| strict breakpoint `timem` monotonicity (`dtime>0` all intervals) | not yet implemented as parser/runtime guard | legacy behavior reference `/workdir/wepp-forest_260430_baseline/src/brkpt.for:76-83`, `:88-92` |
| typed error taxonomy | implemented | `climate.rs:143-205` |

## Runtime/Orchestrator Linkage

### Current architecture state

1. openWEPP has parser-to-runtime adapter seams for `soil` and `chaninp`, with typed runtime errors and integration tests.
2. No equivalent climate runtime adapter seam is currently implemented in orchestrator crates.
3. Therefore, parser output exists, but climate forcing promotion into scheduler/kernel request surfaces is not yet first-class.

Evidence:
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:98-183`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:86-170`
- `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:76-131`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs:311-485`

### Required climate seam boundary (to be implemented)

| seam_id | parser producer | adapter owner (target) | runtime consumer (target) | required symbols |
|---|---|---|---|---|
| `HS-CLIM-SEAM-001` | `parse_climate_file` | `openwepp-hillslope-orchestrator::runtime_inputs` | hillslope scheduler kernel requests | `prcp`, `stmdur`, `timep`, `ip`, `timem`, `intsty`, `tmax`, `tmin`, `rad`, `tdpt`, `vwind`, `iwind` |
| `WS-CLIM-SEAM-001` | `parse_climate_file` (+ hillslope climate assignment surface) | `openwepp-watershed-orchestrator::runtime_inputs` | watershed dispatch runtime surfaces | per-hillslope climate metadata, daily forcing, event forcing summaries |

### Lifecycle and ownership rules

1. Parser output is source-faithful and immutable.
2. Adapter layer owns conversion to runtime units/arrays (`mm->m`, `hr->s`) and explicit typed failure on invalid combinations.
3. Scheduler/kernel layers consume immutable request views only.
4. No compatibility-default parser outcomes may be silently promoted to runtime authority.

## Gaps and Required Follow-On Work

| gap_id | statement | impact | disposition |
|---|---|---|---|
| `CLIM-ARCH-GAP-001` | Missing climate parser-to-runtime adapter seams in both orchestrator crates. | Climate parser is not yet integrated into runtime execution surfaces. | `HOLD` |
| `CLIM-ARCH-GAP-002` | Decision ratified: align breakpoint cardinality target to legacy capacity (`1500`); parser/runtime implementation alignment is still pending. | Short-term parser/runtime mismatch risk remains until implementation lands. | `DECIDED-PENDING-IMPLEMENTATION` |
| `CLIM-ARCH-GAP-003` | Decision ratified: support explicit `datver=0.0` override (`iclig=0`) and `datver>=4.0` (`iclig=1`), and reject pre-4 nonzero correction branch requests (`iclig=2`). | Parser/runtime seam policy enforcement is still pending; until implemented, accepted parser versions may not be mapped with fully explicit runtime gating behavior. | `DECIDED-PENDING-IMPLEMENTATION` |
| `CLIM-ARCH-GAP-004` | No climate-specific end-to-end seam integration tests analogous to `parser_runtime_seam_integration`. | Missing closure evidence for parser-to-kernel climate symbol propagation. | `HOLD` |
| `CLIM-ARCH-GAP-005` | Decision ratified: treat legacy zero-drain non-increasing-time behavior as bug; require strict breakpoint `dtime>0` across all intervals. | Until parser/runtime guards are implemented, malformed duplicate/decreasing breakpoint times may still pass parser ingestion. | `DECIDED-PENDING-IMPLEMENTATION` |

## Integration Constraints

1. Preserve canonical WEPP climate symbols at seam boundary; aliases may be added but not substituted silently.
2. Enforce explicit mode policy for single-storm exclusion.
3. Keep parser-local validation separate from runtime closure checks, per parser-contract requirements document.
4. Maintain docs-first truthfulness: label static vs ran evidence in all follow-on climate seam artifacts.

Evidence:
- `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md:22-107`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for:156-219`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for:61-117`
