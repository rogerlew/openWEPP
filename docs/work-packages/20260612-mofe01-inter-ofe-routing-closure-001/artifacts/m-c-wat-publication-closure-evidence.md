# M-C WAT publication closure evidence

Status: M-C executed-hold; implementation blocked at real per-OFE state boundary

Evidence mode: Ran + Static

## Ran

Operator override:
- Comparisons were run locally in the parent shell, without the
  `comparator_suite_runner` / comparator subagent, because the operator
  explicitly directed local comparison execution and stated that
  GPT-5.3-Codex-Spark weekly quota was exhausted.

| Command/check | Result | Notes |
| --- | --- | --- |
| Fresh H1-H36 CLI batch in `/tmp/openwepp_mofe01_mc` | PASS execution | 36/36 exit code `0`; outputs in `/tmp/openwepp_mofe01_mc/output`; manifests in `/tmp/openwepp_mofe01_mc/manifests`. |
| `tools/owcmp/owcmp batch h1-h39-semantic --baseline-dir /wc1/runs/ar/arboreal-dendrite/wepp/output --baseline-pattern 'H{h}.wat.dat' --candidate-dir /tmp/openwepp_mofe01_mc/output --candidate-pattern 'H{h}.wat.parquet' --candidate-year-offset 2012 --output-root /tmp/openwepp_mofe01_mc/owcmp --start 1 --end 36` | PASS execution, FAIL semantic | `summary.json`: `execution_verdict=PASS`, `semantic_verdict=FAIL`, `semantic_pass_count=0/36`, `structural_row_key_failures=350720`, first divergent H1 key `[1,1,2000]`. |
| Direct parquet publication audit over `/tmp/openwepp_mofe01_mc/output` | FAIL M-C structural gate | All 29 multi-OFE surfaces still have 2192 candidate WAT rows, unique `OFE=[1]`, `publication_ofe_policy=single-row-canonicalized-hillslope-aggregate`, `max(abs(UpStrmQ))=0`, and `max(abs(QOFE-Q))=0`. |
| Single-OFE anchor compare against `/tmp/openwepp_mofe01_mb/output` | PASS | H8/H15/H19/H20/H22/H23/H28 were byte-identical for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`. |

## Publication audit summary

The M-C red tests fail on the current M-B boundary:

- Multi-OFE WAT rows do not include one row per OFE per day. Each H surface
  publishes 2192 rows, while legacy row counts are `2192 * OFE_count`.
- H1 day 1 does not publish five OFE rows. The candidate publishes only
  `OFE=1`.
- Downstream `UpStrmQ` cannot be observed because no downstream OFE rows are
  emitted; the emitted aggregate row has `UpStrmQ=0` for every day.
- `QOFE` remains aliased to `Q` for every multi-OFE surface.

Observed multi-OFE failures:

- Row-shape failure: H1, H2, H3, H4, H5, H6, H7, H9, H10, H11, H12, H13,
  H14, H16, H17, H18, H21, H24, H25, H26, H27, H29, H30, H31, H32, H33, H34,
  H35, H36.
- Zero-`UpStrmQ` failure on the emitted row: same 29 multi-OFE surfaces.
- `QOFE=Q` alias failure: same 29 multi-OFE surfaces.

Local audit JSON was written to `/tmp/openwepp_mofe01_mc/m-c-publication-audit.json`
for this run and is not committed.

## Static implementation boundary

M-C was not implemented because the current production surfaces do not expose
real per-OFE daily hydrology state for WAT publication:

- `execute_scheduler_kernel_lifecycle` builds `TopologyGraph::new(1, 0, 0,
  Vec::new())` and executes one scheduler lifecycle per day, then builds one
  WB13 row from the resulting aggregate writeback surface
  (`scheduler_seed_and_runtime.rs:1469-1589`).
- `build_simulation_owned_wb13_row` hard-codes `UpStrmQ=0.0`, `QOFE=Q`, and
  row key `OFE=1` (`02_output_and_climate_helpers.rs:956-1014`).
- `build_hillslope_wat_rows` is a direct one-row-per-WB13-row projection, not
  a per-OFE expander (`02_output_and_climate_helpers.rs:536-543`).
- The summary accumulator rejects `QOFE != Q` (`openwepp-summary-accumulator/src/lib.rs:277-283`).
- Publication provenance still requires
  `publication_ofe_policy=single-row-canonicalized-hillslope-aggregate` and
  rejects non-`OFE=1` WB13 rows (`scheduler_publication.rs:151-203`).

Synthesizing per-OFE rows from the aggregate row would be surrogate physics.
That would violate the package and root instructions forbidding provisional or
heuristic process-physics math in production kernel/runtime publication paths.

## Disposition

M-C is executed to a hold boundary. The next implementation step is not a WAT
format patch; it is a contract and runtime design increment that introduces a
real per-OFE dynamic state/publication surface, or explicitly defines an
equivalent surface with enough real state to prove handoff identities without
inference.

## Claude review addendum — architectural finding + rung re-framing (2026-06-12)

Evidence mode: Ran (source read of the publication and carry seams) + Static.

**M-C's hold is correct and well-reasoned.** Refusing to synthesize per-OFE
WAT rows from aggregate state is exactly the right call — that would be the
surrogate-physics anti-pattern the whole project forbids. But the *reason*
it held is the load-bearing finding for the rung, and it re-frames what M-B
established.

What the code actually does (verified):

- The hillslope runtime executes **one scheduler lifecycle per day over a
  1-node topology** (`TopologyGraph::new(1, 0, 0, …)`,
  `scheduler_seed_and_runtime.rs:1469`).
- MOFE machinery is **real at the hourly-array level**: per-OFE input
  seeding (`for ofe_index in 1..=ofe_count`, alpha/slplen/cover/roughness/
  plant-growth/wave2-kinematic per OFE), and `mofe_hourly_carry_arrays`
  (upstream carryover, saturation carry, lateral carry) feeding
  `runon_input` in `hydrology_phase_runoff_reconciliation.rs:268-334`.
- But the **daily WB output writeback collapses to aggregate global
  scalars**: `build_simulation_owned_wb13_row` reads single `Q`/`Es`/
  `SubRIn`/… symbols, hardcodes `UpStrmQ=0`, `QOFE=Q`, and emits one
  `OFE=1` row (`02_output_and_climate_helpers.rs:956-1014`). No per-OFE
  daily WB output state is retained past the writeback.

**Consequence for the rung's status (honest re-framing):**

- M-B genuinely retired the multi-OFE execution blocker and made the
  **aggregate hillslope identity** close at the FDHP01 noise floor (Claude
  M-B audit, 1e-13-grade). That stands.
- But the **per-element and transfer identities — the actual definition of
  "routing closure" — remain unproven**, because the per-OFE daily state
  needed to evaluate them does not exist in the writeback. "M-B hydrology
  route closure" should be read as *execution-unblock + aggregate closure*,
  not as proven inter-OFE routing physics. (The M-B audit already flagged
  the per-element/transfer identities as pending M-C; this names why.)
- The real MOFE physics increment is therefore a **runtime-state increment**,
  not a publication patch: retain distinct per-OFE daily WB output state
  through the writeback (so the per-element balance and the sent≡received
  transfer can be measured), then publication and ladder acceptance follow.

**Open scoping question (M-D, not asserted here):** whether the existing
hourly carry arrays already carry enough genuine per-OFE flow that the
increment is "retain + expose per-OFE daily state through the writeback"
(narrower), or whether the per-OFE daily balances are currently coupled in a
way that needs distinct per-element accumulation (broader). The M-D scope
must answer this from the carry-array seam before any production edit —
read the lines (recorded FDHP01-Dh lesson).
