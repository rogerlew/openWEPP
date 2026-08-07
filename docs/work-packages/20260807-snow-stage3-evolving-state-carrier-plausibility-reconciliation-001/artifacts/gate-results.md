# Gate Results And Frozen Command Inventory

Static: commands selected before implementation/result execution. Results are
recorded after each run with `Ran:` evidence.

| Phase | Command | Expected inventory / claim |
| --- | --- | --- |
| A/C | `.venv/bin/python -m pytest -q docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/tools/test_analyze_evolving_carrier.py` | exact analyzer malformed/alias/aggregation tests |
| B | `cargo nextest run --test snow_stage3_turbulent_operator_reconciliation_contract --test snow_stage3_legacy_predecessor_bridge_contract --test snow_surface_eb03_contract` | all three affected registered contract targets |
| B | `cargo fmt --all --check` | Rust integration-test formatting |
| B/F | `cargo clippy --test snow_stage3_turbulent_operator_reconciliation_contract --test snow_stage3_legacy_predecessor_bridge_contract --test snow_surface_eb03_contract -- -D warnings` | warnings-denied affected Rust test targets |
| B/F | `.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` and the same command for `SC-SNOWENERGY-001.md` | both amended contracts have fully consolidated binding exposure |
| B/F | `markdown-doc lint --path <changed-contract-or-index-path>` for each of `SC-SNOWFREEZE-001.md`, `SC-SNOWENERGY-001.md`, and the science-contract index | canonical Markdown validation of each amended authority document |
| B | `.venv/bin/python -m json.tool docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/artifacts/protocol-freeze.json >/dev/null` | frozen protocol remains valid JSON |
| B/F | `git diff --check` | whitespace and conflict-marker hygiene |
| C | `.venv/bin/python docs/work-packages/20260807-snow-stage3-evolving-state-carrier-plausibility-reconciliation-001/tools/analyze_evolving_carrier.py --verify-retained --receipt target/snow_stage3_operator_reconciliation_v3/execution-receipt.json --results target/snow_stage3_operator_reconciliation_v3/results/operator-reconciliation-results.json --output target/snow_stage3_evolving_state_carrier_plausibility_reconciliation/attempt-004` | exact schema-v6 independent reconstruction and immutable output; attempts 001-003 are retained invalid-execution evidence |
| D/E | comparator runner executes the exact Phase-C analyzer command | 4 sites; eligible WY counts 34/44/41/35; WY2025 censoring; one immutable attempt |
| F | `cargo nextest run --workspace --profile quick` | exact terminal diff fast-workspace correctness |
| F | comparator runner executes `cargo nextest run --workspace --profile full` | critical package/full correctness |
| F | scoped Markdown validation/local links and `git diff --check` | documentation/diff hygiene |
| F | line counts and exact-diff/write-set reconciliation | package governance |

No release CLI rebuild is selected because no Rust production or schema change
is intended. Manifest/dependency files remain unchanged, so `cargo deny check`
is not selected. If the terminal diff changes either fact, the gate set
escalates before disposition.

## Phase-B Pre-Implementation Result

Ran at exact clean `5e353b8c8bc56c9d36301743119dbe1c76a0e9a0`:

- focused contract targets: `27/27 PASS`;
- formatting and warnings-denied affected-target Clippy: `PASS`;
- strict Binding Exposure: Snow/Freeze `13` rows and Snow Energy `9` rows,
  fully consolidated;
- exact Markdown lint for both contracts and the index: three files separately
  validated with zero errors/warnings;
- protocol JSON validation and `git diff --check`: `PASS`;
- clean worktree before and after the command set: `PASS`.

Independent Phase-B reviews: science `GO`, Rust `GO`, and consumer `GO`.

## Phase-D/E Four-Site Result

Ran by the required comparator runner at clean `e07cdbdf9`: attempt 004 exited
`0` after `1,969.55 s` per the retained timing log, maximum RSS `76,812 KiB`.
Exact output hashes are
recorded in `implementation-test-evidence.md`.

Inventory: Mica Creek `34`, Niwot `44`, Paradise `41`, Snowbird `35` eligible
water years; total `154`; WY2025 remained right-censored. All trace, receipt,
result, climate, observation, cross-lane, fixed-field, tuple, energy, mass,
endpoint, and N/A checks completed. No evidence hard-fail occurred.

Scientific gates: zero capacity-truncated tuples and zero active-state failure
tuples at every site. Mica Creek, Niwot, and Snowbird pass per-WY support
materiality. Paradise fails one year, WY2015, at `0.0621730192` versus the
frozen `0.05` threshold. Wind exposure remains `UNKNOWN`; geometry and physical
magnitude envelopes remain `NOT_EVALUABLE`. Terminal classes are
`WIND_FORCING_EXPOSURE_UNRESOLVED` and `MULTIFACTOR_OR_INCONCLUSIVE`;
persistence is `HOLD`.

## Phase-F Closure Candidate

Ran on the reconciled closure candidate:

- analyzer pytest: `29/29 PASS`;
- focused Rust contract targets: `27/27 PASS`;
- workspace all-target/all-feature Clippy with warnings denied: `PASS`;
- `cargo fmt --all -- --check`: `PASS`;
- Binding Exposure: Snow/Freeze `13` and Snow Energy `9`, both `PASS`;
- DRAFT assurance validation and 98-file review-draft renderer check: `PASS`;
- all changed Markdown files via `markdown-doc lint`: `PASS`;
- `git diff --check`: `PASS`.

The first workspace quick run stopped after `916` passes on one roadmap
string-contract failure caused by the closure update. The roadmap was corrected
to preserve the required conditional phrase, and the exact failed test then
passed `1/1`.

The required comparator runner then executed both exact clean-head workspace
profiles at `fae6f8a187c475a0213ef6d83cbedc14c5c5baa5`:

- full: `2,288/2,288 PASS`, 33 skipped, `2,332.11 s`, maximum RSS
  `868,016 KiB`;
- quick: `2,239/2,239 PASS`, 40 skipped, `2,327.97 s`, maximum RSS
  `867,172 KiB`.

Both exit codes were zero. The complete quick rerun supersedes the interrupted
attempt; no selected correctness gate remains incomplete.
