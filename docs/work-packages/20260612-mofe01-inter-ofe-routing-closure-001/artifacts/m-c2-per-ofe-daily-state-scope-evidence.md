# M-C2 per-OFE daily state scope evidence

Status: M-C2 executed-hold; runtime-state implementation blocked by missing
per-OFE daily WB state surface

Evidence mode: Ran + Static

## Operator override

Comparisons were run locally without the comparator subagent. The operator
explicitly directed this because GPT-5.3-Codex-Spark weekly quota was
exhausted.

## Scoping verdict

M-C2 answered the required seam question before production edits:

- Existing MOFE hourly carry arrays are real hourly transfer/copy-forward
  state, but they are not genuine per-OFE daily water-balance state.
- Current scheduler/kernel writeback carries one aggregate state surface and
  one aggregate flux surface through one phase lifecycle.
- Daily WB13/WAT publication still reads aggregate symbols and emits one
  `OFE=1` row/day with `UpStrmQ=0` and `QOFE=Q`.

Implementing M-C2 within this increment would require a broader architecture
change: an authoritative per-OFE dynamic state model and sequential OFE
execution/handoff, or an equivalent contracted state surface. Any narrower
patch that splits aggregate daily rows or infers missing OFE balances from
hourly carry totals would be surrogate physics, so production code and science
contracts were not edited.

## Static seam evidence

| Seam | Evidence | M-C2 finding |
| --- | --- | --- |
| Orchestrator writeback shape | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:240` | `HillslopeWritebackSurface` owns one `state_surface` map and one `flux_surface` map. No OFE-keyed writeback collection exists. |
| Scheduler kernel dispatch | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:501` | `execute_with_kernel` runs the phase graph once and lends the same state/flux maps to each phase request. Topology gates execution but does not create OFE-local scheduler lanes. |
| Kernel request/writeback contract | `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs:982`, `:1018`, `:1384` | `WritebackField` is one scalar field; `KernelWritebackPayload` is lists of scalar field updates; `HillslopeKernelRequest` borrows the same aggregate maps. |
| MOFE hourly carry activation | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:30` | Multi-OFE contributors activate 24-slot carry arrays and force hourly substeps, but this is not per-OFE daily accumulation. |
| MOFE carry seed names | `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:1098` | Required arrays are `ui_SUrunf_0001..0024`, `ui_SCrunf_0001..0024`, `ui_LfUrf_0001..0024`, and `ui_LfCrf_0001..0024`: hour-indexed roots, not OFE-indexed row state. |
| Lateral carry producer | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs:92`, `:232`, `:713` | Lateral drainage records current hourly lateral and saturation carry values when MOFE arrays are enabled. It does not create independent daily WB rows for each OFE. |
| Runoff carry consumer/copy-forward | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs:268`, `:1147` | Reconciliation sums upstream hourly carry to `runon_input`, then copy-forwards current carry into upstream carry arrays and exposes aggregate `UpStrmQ`/`SubRIn`. |
| M-B contract tests | `tests/integration/wb11_hydrology_kernel_contract.rs:735`, `:795` | Tests prove array-derived carryover, copy-forward, and separated aggregate `UpStrmQ`/`SubRIn`; they do not prove per-OFE daily WB state. |
| WAT row construction | `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:536`, `:980` | One WAT row is built per WB13 row; WB13 row surface hardcodes `UpStrmQ=0.0`, `QOFE=q`, and `OFE=1`. |
| Summary accumulator guard | `crates/openwepp-summary-accumulator/src/lib.rs:277` | Current WB13 row guard rejects `QOFE != Q`, which is the opposite of the downstream per-OFE publication requirement. |
| Publication provenance | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs:214` | MOFE provenance sums hourly carry totals only; publication policy remains `single-row-canonicalized-hillslope-aggregate`. |

## Ran

| Command/gate | Result | Evidence |
| --- | --- | --- |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Built current hillslope CLI for M-C2 replay. |
| Fresh H1-H36 CLI batch | PASS | `/tmp/openwepp_mofe01_mc2/exit-codes.tsv`: 36/36 exit code `0`; 36 WAT files; 36 manifests. |
| Direct WAT publication audit | FAIL | `/tmp/openwepp_mofe01_mc2/m-c2-publication-audit.json`: all 29 multi-OFE surfaces publish only `OFE=[1]`, `UpStrmQ=0`, and `QOFE=Q`. |
| Single-OFE byte anchors vs M-B | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`. |
| Local owcmp H1-H36 command execution | PASS | `/tmp/openwepp_mofe01_mc2/owcmp/summary.json`: `execution_verdict=PASS`. |
| Local owcmp H1-H36 semantic comparison | FAIL | `/tmp/openwepp_mofe01_mc2/owcmp/summary.json`: `semantic_verdict=FAIL`, `semantic_pass_count=0/36`, `structural_row_key_failures=350720`, first divergent H1 key `[1,1,2000]`. |
| `cargo test --test wb11_hydrology_kernel_contract mofe01_mb -- --nocapture` | PASS | 1 passed, 0 failed. |
| `cargo test -p openwepp-runner mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays -- --nocapture` | PASS | 1 runner seed test passed; filtered tests did not run. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | 28 files validated, 0 errors, 0 warnings. |

## Gate classification

| M-C2 gate | Result | Notes |
| --- | --- | --- |
| Scoping first: decide narrow retain/expose vs broader per-element accumulation | PASS | Static seams show broader architecture is required. |
| Contract-first per-OFE daily state semantics | BLOCKED | No contract edit was made because there is no implementable real state surface in the current architecture. |
| Implement retained per-OFE daily WB state through writeback | BLOCKED | Existing writeback is aggregate scalar maps; no OFE-keyed state collection exists. |
| Per-element identity measurable on 1-5 ladder | BLOCKED | Per-OFE daily WB state is absent. |
| Transfer identity measurable on 1-5 ladder | BLOCKED | Runtime has hourly carry totals, but WAT/daily state does not expose OFE-to-OFE received/sent terms. |
| Aggregate identity unchanged | PASS | No production code changed; current full H1-H36 execution stayed green and single-OFE anchors stayed byte-identical. |
| Single-OFE anchor | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-B. |
| Documentation lint | PASS | Package markdown lint passed after M-C2 artifact updates. |
| Full Rust closure loop | NOT RUN | No production Rust, contract, test, or dependency edits were made in M-C2. Focused existing M-B tests were run. |

## Hold disposition

M-C2 is held at the contract/architecture gate. The next lawful increment must
design and contract the real per-OFE daily state surface before any WAT
publication change. The M-E/M-F path remains blocked until a real state surface
can support publication.
