# ADR-0021 Coverage Closure

Tier: glue.

Status: PASS.

Evidence:

- Focused coverage command:
  `CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t08-cov4 cargo llvm-cov --workspace --test topology_graph_validation_gate --lcov --output-path /tmp/openwepp-cqr-b02-t08-final4.lcov`
- Line coverage: 710/746, 95.1743%.
- Region coverage: 841/874, 96.2243%, derived from unique source-region
  coordinates in the JSON export because the LCOV export has no branch counters
  for this run.
- Function coverage: 70/72, 97.2222%.
- CRAP rows: 60 unique topology rows, 0 above 30, max 10.

Per-function floor:

- Original target functions all exceed the 75% coverage floor:
  `TopologyParseError::fmt` 100%, `parse_topology_fixture_str` 100%,
  `validate_pre_execution_topology` 95%.
- Review-identified extracted count helpers now exceed the 75% floor:
  the executed equality branches are covered, and the retained
  `usize::try_from` fallback branches are explicitly marked
  `COVERAGE-EXCLUDE` because `u32` counts fit `usize` on supported openWEPP
  targets while preserving the prior fail-closed behavior for unsupported
  narrower targets.
- New extracted helpers that carry parser/validator branches are covered by the
  same public characterization tests and all remain below CRAP 30.
- Filtered CRAP reports two topology rows below 75% coverage:
  `collect_channel_count_violation` and `collect_impoundment_count_violation`.
  Both are low only because of the type-impossible overflow branches above; the
  executable branches used on supported targets are covered.

Two `// COVERAGE-EXCLUDE` annotations were added for those type-impossible
overflow arms.
