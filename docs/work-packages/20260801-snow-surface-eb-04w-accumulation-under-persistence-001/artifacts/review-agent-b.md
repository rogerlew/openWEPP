# Independent QA Review B

Evidence class: **Static + Ran**.

Reviewed the current uncommitted EB-04W tree independently. I did not read
`review-agent-a.md`.

Ran from `/home/workdir/openWEPP`:

- `git diff --check` — pass;
- `cargo fmt --all -- --check` — pass;
- `cargo nextest run --test snow_surface_eb04w_accumulation_melt_diagnostics_contract`
  — `2/2` pass;
- `cargo nextest run -p openwepp-hillslope-orchestrator` — `427/427`
  pass;
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner
  --all-targets -- -D warnings` — pass; and
- `cargo deny check` — advisories, bans, licenses, and sources pass, with the
  repository's unmatched `MIT-0` allowance warning.

Static checks also confirmed that the release binary hash matches the receipt,
all 16 receipt provenance hashes match their retained files, all 16 return
codes are zero, the four-lane/16-cell/five-operator population hashes to the
frozen source, the current JSONL is schema v3 with 24 hourly diagnostic rows,
the three SVG/Markdown stems are paired, assurance source/draft hashes are
internally aligned, no touched Rust file reaches 3,000 lines, and the exact
diff is confined to the package's reconciled write set.

## Findings

### B-01 — High (closure-blocking): the exact frozen observation operators were not re-evaluated

Paths: `tools/run_accumulation_diagnostics.py:263-327`,
`artifacts/accumulation-mechanics-results.json:315-355`,
`artifacts/scientific-synthesis.md:10-18`,
`artifacts/scientific-disposition.md:11-20`, and
`artifacts/figures/eb04w-chronology-offsets.md:13-22`.

`operator_rows` analyzes only `cells[f"{lane_id}/B"]` and implements a new
first-zero-after-model-peak rule. It does not consume the frozen observed-date
phase frame or the accepted persistent-disappearance/tie rule. The retained
protocol requires candidates to be sampled on the frozen observed dates and
forbids model dates from replacing the primary frame. The divergence is
material and visible: current recomputation gives Mica Creek `-38` versus
frozen `-35`, Paradise `-60` versus `-37`, and Snowbird `-46.5` versus `-44.5`.
The generated synthesis then prints the inherited offsets under the heading
"Median offset" instead of printing or disposing the recomputed values. The
chronology sidecar acknowledges the disagreement but does not close it.

The same B-only implementation computes every modeled/observed peak ratio,
while `scientific-disposition.md` says the result holds "Across B/L/S/LS".
Executing 16 cells is not evidence that the five exact operators or peak ratios
were evaluated across them. Acceptance criterion 6 and the result-bearing
attribution therefore remain unmet. Reuse the frozen operator implementation
or reproduce its exact observed-date/persistent-disappearance rules, report
cell coverage explicitly, and fail on any unexplained mismatch instead of
substituting inherited values.

Disposition: **accepted; correction required before closure**.

### B-02 — High (closure-blocking): ledger acceptance is self-consistency, not the required independent anti-alias proof

Paths: `tests/integration/snow_surface_eb04w_accumulation_melt_diagnostics_contract.rs:31-72`,
`tools/run_accumulation_diagnostics.py:198-239`, and
`crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs:1167-1194`.

The new integration test only searches source text for field names; it never
executes or parses the real consumer. The cohort does parse the real JSONL, but
its reconstruction omits contract-required relations:

- it never checks `coe_melt_uncapped_m == amelt + bmelt + cmelt + dmelt`;
- it checks applied melt against components plus cap, so a wrong or aliased
  published uncapped field is invisible;
- it defines precipitation as `rain_m + snowfall_swe_m` and then checks only
  that fractions sum to one, rather than reconstructing rain and snowfall
  amounts from an independently published/source total;
- it never checks `snowfall_swe_m == 0.1 * snowfall_depth_m` or explicitly
  rejects the depth/SWE alias; and
- it does not isolate numerical expectations for all four CoE terms or reject
  adjacent/pure-energy/wind-residual aliases.

The producer-side guard uses the same rain and snowfall operands to derive its
own total, so it cannot replace consumer-side anti-alias evidence. This falls
short of `OBL-SNOWFREEZE-P-062`, package acceptance criteria 2-4, and the
conservation/publication acceptance rule. Add numeric, non-aliasing fixtures
whose rejected formulas differ, parse an actual v3 line, and independently
close both the uncapped and applied stages plus the phase amounts.

Disposition: **accepted; correction required before closure**.

### B-03 — High (closure-blocking): behavior neutrality is not bound to the exact terminal implementation

Paths: `artifacts/behavior-neutrality.md:5-20` and
`artifacts/gate-results.md:14,27-37`.

The retained comparison explicitly used the first complete EB-04W cohort
before the terminal rebuild/helper extraction. It compares only 90 fields from
the pre-EB-04V common schema; it does not demonstrate that every EB-04V v2
density field survived the v3 formatter refactor unchanged. The later terminal
cohort proves current execution and EB-04W closure, but it is not compared with
the retained reference. No exact comparison command, input identities, output
hash/report, or source identity is retained.

Consequently the zero-difference claim cannot close acceptance criterion 5 or
the no-arithmetic/no-state-mutation portion of criterion 8 for the exact tree.
Run the decoded WAT and all prior-v2-field comparison against the current
binary/tree and retain an auditable receipt.

Disposition: **accepted; correction required before closure**.

### B-04 — High (closure-blocking): current-scope terminal gates and auditable execution provenance are incomplete

Paths: `artifacts/gate-results.md:23-25`,
`artifacts/implementation-test-evidence.md:25-26`,
`artifacts/logs/terminal-frost.log`, `artifacts/logs/terminal-full.log`, and
`artifacts/execution-receipt.json:1-9`.

The governing gate table truthfully marks quick, frost, and full exact-head
suites pending. The retained frost log is the pre-fix failure and the retained
full log is an interrupted nonterminal run; neither is current passing
evidence. This alone forces `HOLD` under the non-deferral rule until the
independent suite runner finishes successfully.

In addition, most passed gate rows record summaries rather than the required
exact argv, working directory, source/dirty identity, duration/exit status, and
log/output path. The release receipt records the binary path/hash and per-cell
provenance, but not its build command, size/mtime, source identity, or evidence
run command. Assurance adoption likewise has no retained command-level result.
Successful later suites do not repair these other provenance omissions.

Disposition: **accepted; terminal suites and provenance record required before
closure**.

### B-05 — High (closure-blocking): the calibration-readiness matrix does not implement the mandatory governance schema

Path: `artifacts/calibration-readiness-matrix.md:1-16`.

The package declares `science diagnostic implementation +
calibration-readiness`, but the matrix is a six-question yes/no narrative. It
does not report the required orthogonal
`science_implementation_status`, `calibration_evidence_status`, and
`identifiability_status`, and it does not disposition every applicable
`science-contract-spec.md` readiness obligation as `PASS`, `BLOCKED`, or
`NOT_APPLICABLE` with evidence and rationale. It also uses
`CALIBRATION_HOLD` without reconciling the package's declared
`calibration evidence = NOT_APPLICABLE` and initial
`PARTIALLY_IDENTIFIABLE` posture.

This is a mandatory package artifact, not optional narrative. Rebuild it in the
governed form and make the final disposition match those three independent
axes.

Disposition: **accepted; correction required before closure**.

## Acceptance Disposition

| Criterion | Disposition | QA basis |
|---|---|---|
| Contract-first authority | `PASS` | v121 binding and red-before-production evidence are present; current contract test passes. |
| Phase/accumulation reconstruction | `FAIL` | Daily accumulation is checked; independent phase amount and snowfall depth/SWE anti-alias closure are not. |
| Four-term/cap reconstruction | `FAIL` | Applied-stage self-consistency passes; uncapped-stage reconstruction and non-alias term fixtures are absent. |
| Real JSONL consumer | `PASS` | Exact cohort v3 output contains all 24 hourly fields and analysis reads it. |
| Exact-terminal behavior neutrality | `FAIL` | Comparison predates the terminal implementation and omits prior v2 density fields. |
| Four lanes/16 cells/five frozen operators | `FAIL` | Four lanes and 16 runs are valid; exact frozen operators are not reproduced and result analysis is B-only. |
| Interpretation boundaries | `FAIL` | Data-role/pure-energy/redistribution cautions are good, but B-only evidence is overstated across B/L/S/LS and mismatched operators are hidden by inherited values. |
| No behavior/authority-boundary change | `NOT PROVEN` | Static diff is additive, but exact-terminal prior-output comparison is missing. |
| Required validation/review/verification | `FAIL` | Quick/frost/full, dual disposition, and dual verification are pending; several retained passes lack command/source provenance. |

## Non-blocking Debt / Follow-ups

- `infiltration_reconciliation.rs:544-572` converts a fixed four-element array
  through `Vec` allocation and an impossible-length error mapped to `NaN`.
  Directly destructuring the four terms and using the original explicit sum
  would be clearer, cheaper in the hourly hot path, and easier to audit for the
  promised arithmetic-order neutrality.
- The three SVGs have complete same-stem Markdown explanations, captions,
  methods, limits, and textual accessibility guidance. Their embedded
  Matplotlib timestamps make byte-for-byte regeneration nondeterministic;
  consider fixed metadata plus figure hashes in the scientific receipt.
- The 2,224-line infiltration reconciliation, 2,501-line runoff
  reconciliation, and 2,454-line runner formatter remain correctly
  dispositioned warning-band debt. The 51-line trace suffix extraction is a
  cohesive improvement.
- The active kickoff prompt omits the canonical tiered required-reading,
  explicit autonomy, real-consumer, and conservation/output-acceptance lines.
  Bring future kernel diagnostic prompts back to the preparation template.

## Verdict

**HOLD — QA rejection of `DIAGNOSTIC_COMPLETE` on the current tree.**

The production diagnostic path is readable, typed, real-consumer-connected,
format/lint clean, and locally testable. The release cohort and figure pairs
are present and the scientific caution against tuning is appropriate. However,
the exact frozen operator, independent anti-alias closure, exact-terminal
neutrality, calibration-readiness governance, and required terminal evidence
must all close before this package can truthfully claim diagnostic completion.
