# Disposition

Status: `EXECUTED-COMPLETE-WSHED-W3`

Final disposition: `EXECUTED-COMPLETE-WSHED-W3`.

W3 implemented bounded worker-pool orchestration for generated hillslope jobs,
accepted positive `--jobs N`, preserved invalid `--jobs` rejection, and added
focused public CLI tests for output identity, artifact isolation, child-failure
policy, and missing generated pass inventory failure before routing.

The prior canonical fixture blocker was resolved by user-authorized,
fixture-only normalization of over-bound daily `radly` values in
`tests/fixtures/watershed/carnivorous-adobo/`. Production runtime guard
semantics remain unchanged: openWEPP still fails closed rather than clipping
over-bound radiation in production code.

Completion evidence:

- Release scaling matrix passed `18/18` canonical runs: job counts
  `1/2/4/8/16/32`, three repeats each.
- All canonical runs emitted the required watershed outputs and matched
  `jobs1-rep1` by Parquet row content and order.
- Average wall time improved from `36.96 s` at `--jobs 1` to `2.04 s` at
  `--jobs 32` on the recorded 48-logical-CPU host.
- Final gates passed: `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full` (`1283` passed, `1` skipped),
  and `cargo deny check`.
