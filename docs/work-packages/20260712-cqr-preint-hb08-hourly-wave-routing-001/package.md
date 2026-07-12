# HB-08 Hourly Baseline Wave Routing

Status: `MODULE-PASS`
Parent: `docs/work-packages/cqr-high-risk-b-execplan.md`

## Objective

Close the fixed HB-08 row
`Ws10ChannelImpoundmentKernel::ws11_route_baseline_wave_series` by first
covering all same-source function floors, then mechanically decomposing the
time/space routing orchestration only where required for CRAP at most 30.
Preserve WS11 equations, state order, exact floating grouping, guards,
coefficient admissibility, storage closure and downstream publication.

## Target And Start State

- Source: `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs`.
- SHA-256: `c2c7ba5e00e662522b2e559fe8df4c61b7c87a3cbae1b9cd928417378bba065e`.
- Lines: `2,064`, WARN but below the 3,000-line blocker.
- Fixed row: CC `44`, coverage `85.106%`, CRAP `50.396`, `E-SCIENCE`.

The fresh same-source audit must bind region floors. The available campaign
LCOV identifies these additional production functions below 75% line coverage:

| Function | Coverage | CRAP |
| --- | ---: | ---: |
| `ws11_interval_lane_active` | 64.000% | 27.944 |
| `ws11_wave_celerity_and_top_width` | 63.636% | 14.808 |
| `ws11_validate_interval_mass_closure` | 61.538% | 11.641 |
| `ws11_project_hourly_totals` | 72.973% | 10.599 |
| `ws11_close_daily_outlet_volume` | 66.667% | 8.815 |
| `ws11_kinematic_terminal_storage_m3` | 72.222% | 8.050 |
| `ws11_ntchr` | 37.500% | 7.906 |
| `ws11_wave_reference_flow` | 70.588% | 4.407 |
| `ws11_geometry_detachment_mass` | 73.684% | 3.164 |
| `ws11_zero_flow_interval` | 72.222% | 2.086 |
| `ws11_small_count_as_f64` | 54.545% | 1.094 |

These are transitive floor obligations, not additional CQR target rows.

## Authority And Provenance

- `SC-ROUTE-001`: `INV-ROUTE-006/007/016/017/019/020/021/022`, WS11
  physics-equivalence vectors, WSHEDIMPL40/41 MC memory/dynamic-coefficient
  addenda, and TOL-ROUTE-009/010.
- `SC-SYSTEM-001` WS11 integration addendum and
  `INV-SYSTEM-001/005/006/036` downstream ownership.
- Pinned baseline `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`: `wshchr.for` time/space
  recurrence, KW/MC dispatch, epsilon outlet, terminal storage and daily
  volume; `wshcqi.for`, `wshdrv.for`, and `wshpek.for` route-chain context.
- Registered HEC-HMS/NEH Muskingum-Cunge stability references corroborate
  recurrence admissibility but do not replace pinned baseline formulas.

## Bounded Write Set

- `hourly.rs`: behavior-preserving wave-series decomposition and same-source
  floor closure only.
- Nearest private `hourly_tests.rs` and, only where private access requires,
  `direct_tests.rs`.
- Existing W11C runner integration only if a demonstrated consumer assertion
  is missing.
- Package evidence and HB-08/High-B terminal records.

No parser, topology, schema/writer, output key/order, tolerance, clamp,
coefficient repair, routing formula, sediment law or state-memory policy change
is authorized. A semantic defect must transition to a committed DC package.

## A–H Obligations

| Family | Required evidence |
| --- | --- |
| A — nominal | KW, static MC and variable MC over complete grids with finite terminal state, representative coefficients and storage. |
| B — boundaries | Empty/mismatched grid, one/101 segment bounds, first/last interval and spatial node, qref zero, outlet epsilon and volume roundoff. |
| C — regimes | Fresh/carried state; dry/active MC; shape/celerity families; early/late pulses; 3600/600/admissible fine timesteps. |
| D — invalid domain | Non-wave branch, malformed prior/grid/profile/count, invalid geometry/celerity, segment count, storage and negative outlet volume. |
| E — missing seam | Missing profile and required prior-grid fields fail exactly; absent prior state uses only pinned deterministic initialization. |
| F — non-finite | Grid, prior state, geometry, coefficient, storage, mass-closure and projection non-finite paths preserve exact symbol priority. |
| G — conservation | Interval storage changes, KW spatial-mean versus MC boundary-mean storage, daily volume, coefficient sum/passive maximum and sediment mass closure. |
| H — fail closed | Preserve `WKERNEL-WS10-CHANNEL-E-001..003`; no clamp, damping, peak clip, static fallback, skipped interval or synthetic repair. |

## Existing Tests And Real Consumer

Private hourly tests already cover time-zero separation, first/last slot,
early/late pulse, KW terminal spatial storage, fresh/carried storage, static and
dynamic MC admissibility, coefficient maximum principle, timestep variants,
zero peak, prior-day rejection, two-channel carry, geometry carry and sediment
interval closure. Cover-first work must audit each branch rather than duplicate
nominal vectors.

The real downstream consumer is
`openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs`: seven W11C tests
execute static/variable routing across timestep/scenario variants and consume
water, storage, peak and sediment publication. Private helper tests alone
cannot close HB-08.

## Execution And Acceptance

1. Capture fresh same-source JSON/LCOV/CRAP and audit every production function
   against the 75% region floor.
2. Add missing A–H characterization before decomposition.
3. Re-measure, then extract coherent initialization, grid construction,
   interval/segment update, representative selection and finalization stages as
   needed, without reordering arithmetic or errors.
4. Run the full orchestrator and W11C real consumer.
5. Record exact metrics/hashes/counts, lineage, reviews, verification and line
   governance before terminal disposition.

Minimum gates:

    cargo nextest run -p openwepp-watershed-orchestrator
    cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract
    cargo fmt --check
    cargo clippy -p openwepp-watershed-orchestrator -p openwepp-runner --all-targets -- -D warnings
    git diff --check

Acceptance requires the fixed row at CRAP at most 30, zero eligible same-source
function below 75%, exact behavioral/typed-error preservation, W11C consumer
PASS, and two independent final reviews/verifications under the High-B plan.

## Terminal Outcome

Cover-first tests close eligible same-source floors. Mechanical extraction
reduces CC 44/CRAP 50.396 to CC 28/CRAP 28.344 while preserving recurrence
arithmetic and errors. Full orchestrator and W11C gates pass. Disposition:
`MODULE-PASS`.
