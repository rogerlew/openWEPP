# Snow Hourly Energy-Balance Work-Package Queue

Status: complete
Evidence mode: static
Date: 2026-05-25

## Static
- Queue is dependency-ordered and sized to avoid single-package integration
  risk across contract, forcing, kernel, and parity surfaces.
- Baseline authority for migration remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Every code-authoring package below must follow internal contract sequencing:
  1. contract amendments,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.

## Proposed Queue
| order | wp_id | objective | depends_on | exit signal |
|---|---|---|---|---|
| 1 | `20260525-simimpl27-snowfreeze-contract-boundary-closure-for-hourly-energy-balance-001` | Close canonical authority gaps for hourly winter snow/melt boundaries and invariants (`SC-SNOWFREEZE-001`, companion `SC-*`) so migration-ready API/state aliases are promotable. | SNOWPLAN01 | `SC-SNOWFREEZE-001` no longer has non-promotable boundary/API gap for migration scope; contract-derived test requirements are explicit and accepted. |
| 2 | `20260525-simimpl28-hourly-winter-forcing-synthesis-port-001` | Port baseline hourly forcing synthesis chain (`sunmap`, `radcur`, `hr_tmp`, `stmtim`) into openWEPP runtime seams with typed guards and deterministic hourly surfaces for snow/frost consumers. | SIMIMPL27 | Hourly radiation/temperature/phase-partition forcing surfaces are produced from daily inputs with contract-derived tests and typed error posture. |
| 3 | `20260525-simimpl29-snowd-melt-energy-balance-kernel-port-and-coupling-001` | Implement baseline-authoritative snow energy-balance kernel migration (`snowd`, `melt`) including radiation, wind, canopy/residue coupling, and hourly meteorology linkage into hydrology runtime state/publication pathways. | SIMIMPL28 | Reduced `compute_active_snow_coupling` placeholder path is replaced by baseline-authoritative behavior for scoped winter snow terms with passing contract-derived tests. |
| 4 | `20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001` | Run winter-focused semantic parity lanes (single-OFE and MOFE where applicable), classify residuals by confidence tier, and publish GO/HOLD disposition for hourly snow closure wave. | SIMIMPL29 | Required parity lanes complete; residuals dispositioned with explicit ownership; closure recommendation published. |

## Sequencing Constraints
1. `SIMIMPL27` must complete before any hourly winter forcing or kernel edits.
2. `SIMIMPL28` must complete before snow energy-balance kernel migration.
3. `SIMIMPL29` must complete before parity/disposition reruns.
4. `SIMIMPL30` is final closure gate for this migration wave.

## Why Not One Package
- Baseline hourly winter behavior spans multiple coupled routines and broad
  state surfaces, not a single isolated function replacement.
- Contract authority currently carries boundary-readiness gaps for migration
  execution.
- Runtime seams currently emphasize daily forcing lanes and require explicit
  hourly synthesis closure before authoritative snow-kernel parity claims.

## Ran
- not run
