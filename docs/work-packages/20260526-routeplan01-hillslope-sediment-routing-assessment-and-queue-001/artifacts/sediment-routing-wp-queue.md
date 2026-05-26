# Sediment Routing Work-Package Queue

Status: complete
Evidence mode: static+ran
Date: 2026-05-26

## Static
- Queue closes `route.for` segment-routing gaps identified in ROUTEPLAN01.
- Queue also closes sediment-routing magic-number debt by replacing raw
  literals with named constants and contract-anchored provenance.
- Baseline parity authority target remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Each code-authoring package must enforce contract-first sequence:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production edits.

## Proposed Queue
| order | wp_id | objective | depends_on | exit signal |
|---|---|---|---|---|
| 1 | `20260526-erod16-route-branch-contract-authority-and-routine-map-001` | Amend canonical `SC-SED-001` and `SC-ROUTE-001` to explicitly codify `route.for` segment-loop branch authority (`mshear` 1..5, upper-end deposition/detachment, `xcrit/depc/depend/depos/erod/enrich` lineage), and correct the `rtpart.for` misclassification in canonical provenance notes. | ROUTEPLAN01, EROD15 | Contracts contain explicit route branch map, invariants, and alias continuity for segment routing surfaces. |
| 2 | `20260526-erod17-route-branch-contract-derived-tests-and-preimplementation-gate-001` | Add contract-derived vectors for route branch families (mshear cases, deposition-end-in-segment vs extends-to-end, `ndep` follow-up path, zero/near-zero `qostar` threshold behavior) and record pre-implementation gate evidence. | EROD16 | Route-branch tests fail on current implementation where expected; gate evidence recorded before production edits. |
| 3 | `20260526-erod18-route-runtime-segment-state-topology-and-ingress-closure-001` | Add typed runtime state topology for segment routing (`nslpts`, `xu/xl`, `ainf/binf/cinf`, `ainftc/binftc/cinftc`, `xc1/xc2`, `mshear`, `xdbeg/xdend/xdetst`, `ldlast/lddend`) and typed ingress projection into the hillslope kernel boundary. | EROD17 | Required segment state surfaces and typed error guards are available with no silent defaults. |
| 4 | `20260526-erod19-route-mshear-segment-kernel-migration-001` | Implement baseline-authoritative `route.for` segment-loop algorithm in openWEPP (`du<0` and `du>=0` trees, MSHEAR 1..5 branch dispatch, deposition follow-up, enrichment end-of-OFE semantics). | EROD18 | Route branch logic executes with passing contract-derived tests and typed hard-fail posture for invalid domains. |
| 5 | `20260526-erod20-sediment-routing-constant-symbolization-and-magic-number-elimination-001` | Eliminate sediment-routing magic numbers by introducing named constants (including route thresholds and Wave-2 literals), wiring them through runtime code, and documenting each with canonical contract provenance. | EROD19 | No raw literal magic numbers remain in sediment-routing production paths without named constant provenance. |
| 6 | `20260526-erod21-route-parity-rerun-and-hold-lift-disposition-001` | Execute route-focused parity reruns and publish explicit GO/HOLD disposition for sediment-routing closure, including residual ownership. | EROD20 | Admissible parity evidence exists for route branch family or HOLD is retained with explicit blockers. |

## Sequencing Constraints
1. Do not edit route runtime kernel logic before EROD16+EROD17 contract/test gates.
2. EROD18 topology closure must land before EROD19 algorithm migration.
3. EROD20 is required for literal-cleanup closure even if EROD19 passes tests.
4. EROD21 is the final closure gate for sediment-routing parity disposition.

## Magic-Number Removal Scope (minimum)
- Wave-2 attenuation floor literal (`1.0e-8`).
- Wave-2 enrichment offset literal (`+0.005`).
- Case-range literals (`1..=4`) and structural payload-count literals
  (`5 + class_count*6`).
- Route near-zero `qostar` threshold (baseline `.0011`) when route migration
  lands.
- Additional raw literals introduced by EROD19 migration must be symbolized
  before EROD20 closeout.

## Ran
- `nl -ba /workdir/wepp-forest_260430_baseline/src/route.for | sed -n '140,420p'`
- `nl -ba /workdir/wepp-forest_260430_baseline/src/contin.for | sed -n '1190,1248p'`
- `nl -ba crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs | sed -n '5800,6660p'`
- `rg -n "0\.005|1\.0e-8|Vec::with_capacity\(5 \+ \(class_count \* 6\)\)|\(1\.\.=4\)" crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
