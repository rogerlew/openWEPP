# owcmp Specification

Status: implemented

Evidence mode: Static

## Purpose

`owcmp` is the openWEPP comparison and observation tooling surface. It owns
repeatable local execution for legacy WEPP versus openWEPP comparison lanes,
structured summaries for agent and package consumption, and normalization of
legacy observe output into explicit comparison artifacts.

The migration rolled the former `tools/legacy_comparison_suite` behavior into
`tools/owcmp` and retired the legacy directory. The long-term goal is a stable
comparator CLI that supports
manifest-driven suite execution without forcing parent agents to load raw
per-run artifacts, logs, or package-specific diagnostic scripts into context.

## Authority and Current Inputs

The first `owcmp` implementation must preserve these active contracts from
`tools/legacy_comparison_suite`:

- Semantic WAT report schema: `pl14s-semantic-wat-v2`.
- PL14S suite provenance schema: `pl14s-legacy-suite-v2`.
- Tolerance profile currently stored at
  `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`.
- Python dependency lock currently stored at
  `tools/legacy_comparison_suite/requirements.lock.txt`.
- Duplicate `(OFE, J, Y)` row-key rejection for semantic WAT comparisons.
- Candidate source classification:
  `native-runtime-dat`, `conversion-derived-dat`, or
  `native-runtime-parquet`.
- Strict comparator requirement for candidate `.dat` inputs.
- Strict-equivalent metadata and row-consistency checks for parquet and
  conversion-derived candidates.
- Mandatory input identity evidence for promotable comparison claims.
- Baseline-year policy and expected common-row-count enforcement for full-span
  replay comparability.
- Full keyed-span precipitation (`P`) parity readiness metadata.

During Package 1, the active tolerance config and Python dependency lock were
copied into `tools/owcmp`. During Package 2, canonical references were updated
to the new paths before `tools/legacy_comparison_suite` was deleted. The
migration proved the effective tolerance profile unchanged by recording
before/after hashes and byte-for-byte identity checks.

The cutover must update active tests and canonical references that currently
bind to `tools/legacy_comparison_suite`, including:

- `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`.
- Root README references to Python comparator setup.
- Any active package prompts that name the legacy suite for future work.

Historical work-package artifacts may keep old paths as evidence of what ran at
the time. They are not active API references and do not block directory removal.

## Non-Goals

- `owcmp` is not a production hydrology or erosion kernel.
- `owcmp` does not make legacy comparator agreement an acceptance oracle.
  ADR-0017 remains authoritative: comparator output is an investigation flag,
  not a target.
- `owcmp` does not reintroduce parser support for legacy `wepp_observe*`
  sidecars. Legacy observe activation can only be represented through explicit
  typed selectors or normalized evidence.
- `owcmp` does not require agents to inspect raw logs or full per-hillslope JSON
  reports for ordinary package handoff.

## CLI Surface

`owcmp` should be runnable as a repo-local Python CLI during the initial
migration. A Rust binary can replace or wrap it later if there is a concrete
integration reason.

Direct `tools/owcmp/owcmp` execution must prefer `.venv/bin/python` when the
repo-local environment exists, so comparator agents get the same dependency
surface as local tooling and parquet lanes can import `pyarrow`.

Initial implementation commands:

```text
tools/owcmp/owcmp wat semantic ...
tools/owcmp/owcmp pl14s run ...
tools/owcmp/owcmp batch h1-h39-semantic ...
tools/owcmp/owcmp summarize ...
tools/owcmp/owcmp manifest run ...
```

Deferred command:

```text
tools/owcmp/owcmp observe normalize ...
```

`observe normalize` is intentionally outside the first two cutover packages
unless a dedicated observability package adds the required typed selector,
schema, and migration-error contract.

### `owcmp wat semantic`

Runs a semantic comparison for one hillslope WAT surface.

Required behavior:

- Accept baseline `.dat`.
- Accept candidate `.dat` or parquet.
- Support candidate parquet filtering by partition column/value.
- Support candidate year-key offset.
- Emit a JSON report with schema `pl14s-semantic-wat-v2` until a replacement
  schema is deliberately versioned.
- Reject duplicate semantic row keys before computing deltas.
- Compute row-presence deltas, per-column tolerance verdicts, max absolute and
  relative deltas, and top divergent rows.

Required report payload keys:

- Top-level `report_schema_version`, `inputs`, `tolerances`, and `comparison`.
- `inputs.baseline_wat`, `inputs.candidate_wat`, `inputs.baseline_format`, and
  `inputs.candidate_format`.
- `inputs.row_key_fields` set to the semantic key fields.
- `inputs.width_diagnostic_mode`, `inputs.baseline_numeric_widths`, and
  `inputs.candidate_numeric_widths`.
- `inputs.baseline_column_alias_sources` and
  `inputs.candidate_column_alias_sources`.
- `inputs.candidate_partition_value`, `inputs.candidate_partition_column`, and
  `inputs.candidate_year_offset`.
- `inputs.baseline_sha256` and `inputs.candidate_sha256`.
- `comparison.semantic_pass`, `comparison.common_row_count`,
  `comparison.only_baseline_count`, and `comparison.only_candidate_count`.
- `comparison.column_stats`, `comparison.top_divergent_rows`,
  `comparison.investigation_columns_used`,
  `comparison.investigation_columns_missing`,
  `comparison.baseline_only_columns`, and
  `comparison.candidate_only_columns`.

Compatibility mapping:

- Replaces `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`.

### `owcmp pl14s run`

Runs the current PL14S replay plus strict and semantic comparison orchestration.

Required behavior:

- Replay the declared baseline run using the declared baseline binary and run
  file.
- Run strict raw comparison when candidate input is `.dat`.
- Use the pinned default strict comparator
  `/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py`
  unless an explicit package manifest names another comparator authority.
- Invoke strict raw comparison with zero tolerance (`--abs-tol 0 --rel-tol 0`)
  and persist the strict JSON artifact.
- Run semantic WAT comparison for all candidate source classes.
- Emit provenance schema `pl14s-legacy-suite-v2` until a replacement schema is
  deliberately versioned.
- Record input identity hashes and tool hashes.
- Preserve current promotability rules for native, conversion-derived, and
  parquet candidates.
- Preserve `--baseline-year-policy` and `--expected-common-row-count`.
- Fail closed when the declared expected common-row count does not match the
  semantic report.
- Emit `baseline_year_policy_materialization`,
  `strict_lane_policy.baseline_year_policy`,
  `strict_lane_policy.expected_common_row_count`,
  `strict_lane_policy.full_span_policy_ready`, and
  `strict_lane_policy.full_span_policy_blockers`.
- Emit `strict_lane_policy.conversion_source_row_consistency_ready` and
  `strict_lane_policy.conversion_source_row_consistency_blockers`.
- Preserve strict-equivalent semantic lane blockers for parquet candidates.
- Preserve full keyed-span `P` parity readiness metadata required by
  SIMIMPL18/PL14S replay comparability.

Required provenance payload keys:

- Top-level `suite_schema_version`, `baseline`, `candidate`,
  `strict_lane_policy`, `tooling`, `executions`, and `outputs`.
- `baseline.binary`, `baseline.binary_sha256`, `baseline.run_dir`,
  `baseline.run_file`, `baseline.source_runs_dir`, `baseline.baseline_lane_root`,
  `baseline.baseline_wat`, `baseline.baseline_wat_sha256`, and
  `baseline.baseline_year_policy_materialization`.
- `candidate.input_wat`, `candidate.input_wat_format`,
  `candidate.candidate_surface_source_class`, `candidate.input_wat_sha256`,
  `candidate.candidate_wat_for_compare`,
  `candidate.candidate_wat_for_compare_sha256`,
  `candidate.candidate_partition_value`,
  `candidate.candidate_partition_column`, and
  `candidate.candidate_year_offset`.
- `strict_lane_policy.mode`, `strict_lane_policy.strict_required`,
  `strict_lane_policy.strict_equivalent_lane`,
  `strict_lane_policy.strict_equivalent_ready`,
  `strict_lane_policy.strict_equivalent_blockers`,
  `strict_lane_policy.strict_source_promotable_for_final_tier_a_closeout`,
  `strict_lane_policy.baseline_year_policy`,
  `strict_lane_policy.expected_common_row_count`,
  `strict_lane_policy.full_span_policy_ready`,
  `strict_lane_policy.full_span_policy_blockers`,
  `strict_lane_policy.conversion_source_row_consistency_ready`, and
  `strict_lane_policy.conversion_source_row_consistency_blockers`.
- `tooling.legacy_comparator_tool`,
  `tooling.legacy_comparator_tool_sha256`, `tooling.semantic_script`,
  `tooling.semantic_script_sha256`, `tooling.tolerance_config`, and
  `tooling.tolerance_config_sha256`.
- `executions.baseline_replay`, `executions.strict_compare`, and
  `executions.semantic_compare`.
- `outputs.semantic_json`, `outputs.semantic_json_sha256`,
  `outputs.semantic_summary`, `outputs.strict_json`,
  `outputs.strict_json_sha256`, `outputs.baseline_stdout`, and
  `outputs.baseline_stderr`.

Compatibility mapping:

- Replaces `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`.

### `owcmp batch h1-h39-semantic`

Runs the repeated H1 through H39 semantic WAT comparison as one agent-safe
command.

Required behavior:

- Compose `owcmp wat semantic` behavior; do not duplicate comparison math.
- Default to `baseline_H{h}.parquet` under `--baseline-dir` and
  `H{h}.wat.parquet` under `--candidate-dir`.
- Accept `--start`, `--end`, `--baseline-pattern`, and `--candidate-pattern` so
  tests can run small local fixture ranges without parquet.
- Default `--candidate-year-offset` to `2012` for the HPHYS H1-H39 replay
  surface.
- Write per-H semantic reports under `reports/semantic/`.
- Redirect per-H stdout/stderr under `logs/`.
- Write `command-log.json`, `summary.json`, and `summary.md`.
- Fail closed on missing inputs or command failure with path-level evidence.
- Print one compact JSON line with execution verdict, semantic pass count, and
  summary paths.

Required summary payload keys:

- `source_type` set to `h1_h39_semantic_batch`.
- `execution_verdict`, separating command success from semantic comparator
  agreement.
- `semantic_verdict` and `semantic_pass_count`.
- `pass_hillslopes` and `failed_hillslopes`.
- `structural_row_key_failures`.
- `first_divergent` when available.
- `focus_columns`, including hillslope fail count, total fail count, mean
  absolute-difference mean, max absolute delta, max relative delta, and max key.
- `command_status`, `raw_reports`, `logs`, `summary_json`, `summary_md`, and
  `command_log`.

This command is the preferred H1-H39 surface for `comparator_suite_runner`.
Parent agents should consume `summary.json` or `summary.md` and leave raw reports
and logs on disk.

### `owcmp summarize`

Produces compact, parent-agent-safe summaries from one or more raw comparator
reports.

Required output:

- Command or manifest identifier.
- Exit code and `PASS`/`FAIL` per command.
- Suite pass count, for example `0/39` or `39/39`.
- Structural row/key failure count.
- First divergent unit/window/key where available.
- Focus-column metrics: hillslope fail count, total fail count, mean absolute
  delta mean, max absolute delta, and max relative delta when available.
- Absolute paths to raw logs and reports.
- One-line verdict.

This command is the default surface for Codex parent agents and subagents.
Raw logs, per-row dumps, and per-hillslope reports must remain file artifacts,
not chat output.

### `owcmp manifest run`

Runs a declared comparison manifest.

Manifest responsibilities:

- Name suite lane and schema version.
- Name baseline source, commit or binary identity, run root, and run file.
- Name candidate source, candidate HEAD or binary identity, and output surface.
- Name tolerance profile.
- Name output root.
- Declare whether the run is promotable, investigation-only, or blocked until
  identity evidence is supplied.
- Declare input identity evidence for soil (`*.sol`), management/landuse
  (`*.man`), slope (`*.slp`), climate (`*.cli`), `pmetpara.txt`, `snow.txt`,
  `wepp_ui.txt`, and any lane-required sidecar.
- Define whether missing or unequal input identity evidence is a hard failure or
  an explicit downgrade to investigation-only. Promotable evidence must not be
  emitted when required identity evidence is missing or unequal.
- Declare baseline-year policy and expected common-row count when the lane
  requires full-span comparability.

Initial implementation may support only the PL14S WAT lane. In Package 1,
`manifest run` is limited to a PL14S lane plus an explicit `args` list that is
forwarded to `owcmp pl14s run`; it does not yet validate the full manifest
responsibility list above. The manifest format should not assume that all future
lanes are WAT, daily, single-OFE, or PL14S.

### Deferred: `owcmp observe normalize`

Normalizes legacy observe evidence into structured records for comparison
ledgers. This command is not part of the initial PL14S cutover package.

Required posture:

- Accept explicit input paths and selectors only.
- Do not scan the current working directory for `wepp_observe*` sentinels.
- Do not provide parser compatibility for observe sidecars.
- Emit JSONL or JSON records with explicit symbol, unit, source file/line when
  known, selector, and value fields.
- Emit a schema ID, typed migration errors, and explicit migration notices.
- Label outputs as diagnostic evidence unless a later canonical contract grants
  stronger authority.

This command is the future bridge for old `wepp.observe` evidence into `owcmp`,
not a revival of observe sidecar activation. It should be implemented under a
separate observability work package, not hidden inside the PL14S comparator
cutover.

## Artifact Layout

Default output roots should use this shape:

```text
<output-root>/
  manifest.json
  summary.json
  summary.md
  logs/
  reports/
  raw/
```

Rules:

- `summary.json` and `summary.md` are compact and intended for package handoff.
- `reports/` contains structured comparator reports.
- `raw/` contains large per-hillslope, per-row, and intermediate artifacts.
- `logs/` contains redirected command stdout/stderr.
- The CLI must print the summary path and a compact verdict, not full logs.

## Context Discipline

`owcmp` exists partly to reduce Codex context waste. Every command that can
produce large output must write that output to files and provide a compact
summary. The expected subagent contract is:

- Run exact commands or manifests declared by the package/operator.
- Do not read canonical science contracts unless the package explicitly asks.
- Do not interpret physics or authorize production edits.
- Return command status, compact metrics, verdict, and artifact paths.
- Prefer `tools/owcmp/owcmp batch h1-h39-semantic` for H1-H39 semantic WAT
  comparisons instead of historical package-local scripts.

## Compatibility and Deprecation

The migration should avoid a long-lived compatibility layer.

Allowed temporary compatibility:

- A short-lived legacy path wrapper may exist for one cutover package if needed
  to produce clearer failure messages.
- The wrapper must print or raise a direct moved-path error naming `tools/owcmp`.
- The wrapper must not silently delegate forever.

Cutover completion requires:

- Active tests reference `tools/owcmp`.
- Active docs reference `tools/owcmp`.
- Active package templates/prompts reference `tools/owcmp`.
- `tools/legacy_comparison_suite` is removed.
- `__pycache__` artifacts are absent.

## Work Package Plan

### Package 1: Implement `owcmp`

Scope:

- Create `tools/owcmp` CLI and package-local docs.
- Port PL14S WAT semantic comparison behavior.
- Port PL14S suite runner behavior.
- Add compact summary generation.
- Preserve existing schema markers and tolerance behavior.
- Move or copy PL14S tolerance config and Python dependency lock into
  `tools/owcmp`; prove effective tolerance identity against the legacy file.
- Preserve strict comparator default path, zero-tolerance invocation, artifact
  naming, and hash capture.
- Preserve baseline-year policy, expected common-row-count, full-span readiness,
  strict-equivalent blockers, and conversion row-consistency metadata.
- Add or update focused tests for the new paths.
- Do not implement `owcmp observe normalize` in this package.

Exit criteria:

- Existing PL14S contract behavior passes through `tools/owcmp`.
- Focused `owcmp` contract tests pass while the existing
  `pl14s_tier_a_candidate_emission_and_replay_contract` remains intact.
- Compact summaries are generated without loading raw reports into model
  context.
- No active code path depends on a new long-lived legacy wrapper.
- Tolerance config hash/effective profile is proven unchanged.

### Package 2: Cut Over and Remove Legacy Suite

Status: complete by OWCMP02.

Scope:

- Update canonical docs and tests from `tools/legacy_comparison_suite` to
  `tools/owcmp`.
- Retarget `pl14s_tier_a_candidate_emission_and_replay_contract` or its
  successor to the `tools/owcmp` paths.
- Update active prompt/package references where they define future execution.
- Delete `tools/legacy_comparison_suite`.
- Run relevant tests and path checks.

Exit criteria:

- `rg legacy_comparison_suite` returns only historical work-package artifact
  references or no references, depending on the selected cleanup policy.
- `cargo test --test pl14s_tier_a_candidate_emission_and_replay_contract`
  passes.
- The root README no longer documents the legacy suite as active tooling.

### Package 3: Agent Runner H1-H39 Validation

Status: active by OWCMP03.

Scope:

- Add `owcmp batch h1-h39-semantic`.
- Update `.codex` comparator runner configuration to name `tools/owcmp/owcmp`.
- Add config compliance tests so active runner examples do not regress to the
  retired legacy suite path.
- Validate the H1-H39 batch through `comparator_suite_runner`.

Exit criteria:

- Focused `owcmp` batch and config tests pass.
- The comparator runner returns compact H1-H39 metrics and artifact paths, or a
  hard blocker with command-level evidence.
- No active `.codex` runner config references `tools/legacy_comparison_suite`.

### Future Package: Observe Normalization

Scope:

- Add `owcmp observe normalize` only after the observability migration contract
  has a concrete CLI UX, schema ID, selector validation, and typed migration
  error contract.
- Keep parser-sidecar compatibility unsupported.
- Treat normalized legacy observe evidence as diagnostic unless a canonical
  contract grants stronger authority.

## Review Questions

- Does this spec preserve all behavior currently required by PL14S contract
  tests?
- Are the CLI commands narrow enough for a first implementation package?
- Is observe normalization scoped tightly enough to avoid reintroducing
  sidecar activation?
- Are compatibility wrappers constrained enough to prevent another permanent
  legacy surface?
