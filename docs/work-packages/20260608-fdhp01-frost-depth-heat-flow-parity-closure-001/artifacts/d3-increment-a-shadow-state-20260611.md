# D3 Increment A Shadow Fine-State Evidence

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Scope

Increment A landed the fine-sublayer state shape and `frwatc` handoff proof
surface with the state driving nothing. Active depth progression, WAT `frdp`,
WAT `frozwt`, frozen-soil conductivity, and freeze/thaw physics remain the
pre-increment production behavior. D3 is still open for increments B and C.

Static:

- `SC-SNOWFREEZE-001` is now v58. `INV-SNOWFREEZE-012` pins
  `frwatc(1)` to active-day hour-1 ingress (`frostn.for:335-337`,
  `hour.eq.1`) instead of an ambiguous every-hour entry.
- The contract now authorizes Increment A shadow aliases for
  `fgfrst`, `slfsd`, `slsic`, `slsw`, `sltime`, `yst`, and `nwfrzz`, plus an
  internal residual identity for the handoff.
- `FrostCouplingOutcome` carries shadow fine-layer diagnostics and aggregate
  residuals; runoff reconciliation writes those symbols back to runtime state.
- `compute_active_frost_coupling` computes the shadow state from the current
  layer water state before existing active scalar/depth logic. The shadow
  state is not read by active depth, publication, conductivity, or water-store
  mutation paths.
- WAT parquet writing now emits a deterministic `ARROW:schema` footer by
  sorting Arrow field/schema metadata before encoding it. This preserves
  required field metadata (`units`, `description`) while making the package's
  byte-identical output gate enforceable.

## Local Gates

Ran:

| Command | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo test -p openwepp-hillslope-output hillslope_wat -- --nocapture` | Pass, 4 tests; includes WAT byte-stability and file field-metadata preservation |
| `cargo clippy -p openwepp-hillslope-output --all-targets -- -D warnings` | Pass |
| `cargo test --test cli04_runner_wat_parquet_contract_derived_tests -- --nocapture` | Pass, 2 tests |
| `cargo test --test sim_contract_boundary_unit_registry -- --nocapture` | Pass, 14 tests |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 22 tests; includes Increment A round-trip, seam identity, and non-driving shadow-state tests |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v58 |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v58 |

## Cohort Gate

Comparator-suite reruns used copied runfiles and daily fallback behavior
matching the pre baseline, not `--run-dir` against `/wc1`.

Ran:

| Gate | Result |
|---|---|
| Clean pre baseline root | `/tmp/fdhp01_increment_a_pre_20260611T164115Z` (`20a1e91f`) |
| Latest current root | `/tmp/fdhp01_increment_a_current_pre_like_pre_1_20260611T181018Z` |
| Current release binary SHA | `cd3a2318550e641c94d3c54a8dc7bf5dacf42cc80d57178e521b4215ae75c12b` |
| Current cohort execution | Pass, `43/43` clean exits |
| Pre vs current `H.hbp` physical byte equality | Pass, `43/43` |
| Pre vs current `H.loss.json` physical byte equality | Pass, `43/43` |
| Pre vs current `H.wat.parquet` decoded rows/columns | Pass, `43/43` |
| Pre vs current daily fallback warning counts | Pass, `43` vs `43`, mismatch runs `0` |
| Pre vs current `H.wat.parquet` physical byte equality | Fail, `0/43`; first diff `p1`, pre SHA `c62c2918d6543c865c6d1ce238637217184f8b7704110f37f8a4776250b0bf40`, current SHA `bb00af3f54453222d4b78f6120551a8929e5207731cbfde5787a0c5c89bf9ca2` |
| Pre WAT `ARROW:schema` footer uniqueness | `43` unique footer hashes across `43` files |
| Current WAT `ARROW:schema` footer uniqueness | `1` unique footer hash across `43` files |

The original physical-WAT pre/post gate cannot be satisfied against the clean
pre baseline because that baseline was produced by the prior nondeterministic
Arrow footer path. The physical mismatch is isolated to parquet footer bytes:
decoded WAT payloads match, and the non-WAT binary outputs are byte-identical.

To prove the new gate is enforceable after this precondition, the required
subagent ran a latest-source current-vs-current pair:

| Gate | Result |
|---|---|
| Current root 1 | `/tmp/fdhp01_increment_a_current_pre_like_pre_1_20260611T181018Z` |
| Current root 2 | `/tmp/fdhp01_increment_a_current_pre_like_pre_2_20260611T181018Z` |
| Current-vs-current execution | Pass, `43/43` clean exits in both roots |
| Current-vs-current `H.hbp` physical byte equality | Pass, `43/43` |
| Current-vs-current `H.loss.json` physical byte equality | Pass, `43/43` |
| Current-vs-current `H.wat.parquet` physical byte equality | Pass, `43/43` |
| Current-vs-current decoded WAT equality | Pass, `43/43` |
| Current-vs-current fallback warning counts | Pass, `43` vs `43`, mismatch runs `0` |
| Compact summary | `fdhp01_increment_a_current_pair_comparison_20260611.json` |

Latest-source full Rust gates after final WAT footer minimization:

| Command | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| Compact summary | `fdhp01_increment_a_gates_latest_20260611.json` |

## Disposition

Increment A is not a D3 physics fix. It is a behavior-preserving seam landing
that makes the legacy fine-state shape observable and testable before it is
allowed to drive depth or publication. The only failed literal output gate is
the old-pre physical WAT byte check, and that failure is attributable to
preexisting nondeterministic parquet `ARROW:schema` footers rather than changed
WAT rows or shadow-state wiring. Increments B and C remain responsible for
deriving depth from the fine state, freeze energy consumption, thaw arms,
sandwich geometry, and final depth/duration acceptance.
