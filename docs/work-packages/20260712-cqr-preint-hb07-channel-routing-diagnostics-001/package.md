# HB-07 Channel-Routing Diagnostics

Status: `ACTIVE`
Parent: `docs/work-packages/cqr-high-risk-b-execplan.md`

## Objective

Close the two fixed HB-07 CRAP rows in the WS10/WS11 channel-routing
diagnostics module by covering missing geometry and variable-parameter
Muskingum-Cunge regimes first, then mechanically decomposing only if the final
CRAP scores still exceed 30. Preserve equations, floating evaluation order,
typed guard priority, routing state, publication, and downstream behavior.

## Fixed Target And Start Metrics

Target:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
(620 lines; SHA-256
`4e4166a3f329e0ca1a8860645036cbc4e9ce3118d699d71346fd6d6302635117`).

| Fixed row | CC | Start coverage | Start CRAP |
| --- | ---: | ---: | ---: |
| `Ws10ChannelImpoundmentKernel::ws11_muskingum_geometry_from_depth` | 15 | 52.308% | 39.408 |
| `Ws10ChannelImpoundmentKernel::compute_variable_muskingum_cunge_state` | 32 | 70.455% | 58.410 |

Both are `E-SCIENCE` and currently miss the binding 75% function floor. No
other function is an HB-07 target unless the fresh same-source audit proves a
transitive helper below the floor or above CRAP 30.

## Authority And Provenance

- Canonical process authority: `SC-ROUTE-001`, especially `INV-ROUTE-006`,
  `INV-ROUTE-007`, `INV-ROUTE-021`, `INV-ROUTE-022`, the WS11 physics
  equivalence vectors, and the WSHEDIMPL40/41 Muskingum-Cunge addenda.
- System boundary authority: `SC-SYSTEM-001#WS11 Channel-Routing
  Physics-Equivalence Integration Addendum`, including
  `INV-SYSTEM-001`, `INV-SYSTEM-005`, and `INV-SYSTEM-006`.
- Pinned baseline: `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, chiefly `wshchr.for`
  (`MVPMC3`, geometry, coefficient refresh, recurrence and storage), with
  `wshcqi.for`, `wshdrv.for`, and `wshpek.for` for route-chain/branch context.
- Independent corroboration: HEC-HMS Muskingum-Cunge Eq. 6-15 and NEH 630
  Chapter 17 references already registered by `SC-ROUTE-001`.

## Bounded Write Set

- Production: the diagnostics target only, for behavior-preserving helper
  extraction after coverage closure.
- Tests: nearest private kernel tests (`direct_tests.rs` and/or
  `hourly_tests.rs`) for geometry, dynamic coefficients, prior-state memory,
  admissibility, and typed failures.
- Real consumer: existing runner
  `mt3_hbp_hourly_consumer_contract.rs` W11C vectors; update only if a missing
  downstream assertion is demonstrated.
- Package artifacts plus HB-07/High-B terminal records.

No parser, topology, output schema, writer, coefficient tolerance, routing
formula, state-memory policy, clamp/fallback, or sediment-process change is
authorized. A semantic defect requires the campaign defect-closure transition.

## A–H Obligations

| Family | Required evidence |
| --- | --- |
| A — nominal | Shapes 1–3, admissible `ipeak = 5`, finite refreshed `c0..c4`, routed closure and prior-state response. |
| B — boundaries | Positive depth/width/shape threshold, dynamic `qref` epsilon, bracket expansion, denominator and channel-length limits. |
| C — regimes | Triangular, rectangular, naturally eroded geometry; each `ckref` branch; fresh and carried `qin/q1`; 3600 s, 600 s and admissible fine grid. |
| D — invalid domain | Shape 0/out-of-domain, non-positive hydraulic geometry, roughness/slope/length, unbracketed depth, invalid `ckref`, `dencx`, or denominator. |
| E — missing seam | Missing prior state follows only the contract's explicit deterministic initialization; required runtime payload remains fail closed. |
| F — non-finite | NaN and positive/negative infinity at geometry, discharge, coefficient, and state boundaries with exact WS10 guard family. |
| G — conservation | Coefficient sum/monotonicity, passive maximum principle, `roff = qpo * durrof`, and `chvol = volint + sinit - sfnl`. |
| H — fail closed | Preserve `WKERNEL-WS10-CHANNEL-E-001..003`; no coefficient clamp, peak clip, empirical damping, static fallback, or synthetic state substitution. |

## Existing Tests And Real Consumer

- `direct_tests.rs` covers `ipeak = 5` dispatch and direct variable-Muskingum
  peak behavior.
- `hourly_tests.rs` covers shapes/branches, prior state, refreshed coefficients,
  static/variable MC behavior and admissibility.
- `openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` executes W11C
  static/variable (`ipeak = 4/5`) channel routing at 3600 s and 600 s, checks
  water/storage/peak response and sediment publication, and reads the
  downstream watershed outputs. This is the named real consumer.
- `watershed_cli_behavior_contract` has a static source guard proving the
  direct kernel calls `compute_variable_muskingum_cunge_state`; it is supporting
  evidence, not the downstream consumer.

## Execution And Acceptance

1. Capture same-source JSON/LCOV/CRAP and audit every eligible production
   function against the 75% region floor.
2. Add missing A–H characterization before any decomposition.
3. Re-measure. If either fixed row remains above CRAP 30, extract coherent
   geometry/branch/coefficient stages without changing operation order.
4. Run focused orchestrator tests and the named W11C runner consumer.
5. Record operand lineage, exact commands/counts/hashes/metrics, review and
   verification dispositions, line-count governance, and terminal status.

Minimum gates:

    cargo nextest run -p openwepp-watershed-orchestrator
    cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract
    cargo fmt --check
    cargo clippy -p openwepp-watershed-orchestrator -p openwepp-runner --all-targets -- -D warnings
    git diff --check

Acceptance requires both fixed rows at CRAP at most 30, zero eligible function
below 75%, preserved numeric/publication behavior, exact typed guards, the real
W11C consumer passing, and two independent final reviews/verifications under
the High-B delegation authorization.
