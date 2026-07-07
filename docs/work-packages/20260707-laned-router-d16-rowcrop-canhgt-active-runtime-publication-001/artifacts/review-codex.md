# Review - Codex

Status: COMPLETE

Static:

- Reviewed the final package diff for authority, consumer-path, guard, and
  validation risks.

Findings:

- None blocking.

Accepted corrections during review:

- The first implementation attempted to route daily post-growth canopy height
  into a Wave-1 erosion consumer. `erosion_single_ofe_p61_sediment` exposed a
  large p61 sediment change, so that broader change was reverted. The final
  package remains scoped to PL daily publication and Lane D routing operand
  consumption.
- Fixture/test drift from the new `bbb`, `hmax`, and `canopy_height_m` fields
  was fixed in R5D growth tests, projection fixtures, source guards, and
  direct-frame size ceilings.

Residual risk:

- Hybrid plain-vs-hybrid deltas remain a D16 default-promotion/tolerance issue.
  This package closes the row-crop active-run publication failure; it does not
  claim hybrid default readiness.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
  is 2996 lines after this package. It is below the 3000-line closure block but
  should be split before the next meaningful edit in that module.
