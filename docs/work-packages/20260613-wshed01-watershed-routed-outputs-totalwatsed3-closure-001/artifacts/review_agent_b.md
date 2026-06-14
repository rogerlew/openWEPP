# Review Agent B

Status: T-B2-REDO2 post-review complete

Evidence mode: Static

## Findings

No blocking W-A findings.

Checks:

1. W-A did not edit production source.
2. The artifact set distinguishes current-scope W-A characterization from
   W-B/W-C implementation gates; it does not claim totalwatsed3 closure.
3. The next-increment red tests include the governance lessons from MOFE01:
   independent operands, anti-placeholder publication, and no exact-zero
   acceptance on the real routed run.

## Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | accepted | W-A is complete; full package closure remains blocked on W-B-WD. |

## W-D QA Review

Evidence mode: Static + Ran

Blocking finding:

1. The independent conservation proof is not present because `runvol` is still
   WAT-derived. This keeps W-D at `executed-hold` even though the publication
   defects found during the audit were fixed.

QA findings addressed during W-D:

1. Optional WAT numeric columns with mixed null/value rows initially risked
   silent zero coercion. The ingestion helper now treats all-null optional
   columns as absent-equivalent and rejects mixed null/value columns as typed
   null failures.
2. Final gates were pending during review. They were run after artifact
   updates: fmt, clippy, workspace tests, deny, diff check, and scoped markdown
   lint all pass.

Residual risk:

- The optional-column null test is in-memory. The real CLI/audit path exercises
  production parquet ingestion on the arboreal-dendrite fixture, but no
  dedicated parquet fixture for mixed-null optional WAT columns was added in
  W-D.

## W-D Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| 1 | Closure proof still WAT-self-consistent | accepted / blocking | T-B is queued for independent PASS runoff lineage in the dedicated CLI. |
| 2 | Optional mixed nulls risk silent coercion | fixed | Added typed rejection path and focused unit coverage. |
| 3 | Final gates pending at review time | fixed | Full gate suite ran and passed after artifact updates. |
| 4 | Mixed-null parquet fixture absent | deferred | Real CLI/audit parquet path is covered; dedicated edge fixture remains follow-on hardening. |

## T-A QA Review

Evidence mode: Static + Ran

Findings:

No blocking T-A findings.

QA observations:

1. T-A did not edit production source and therefore does not need the Rust
   closure loop for completion.
2. The scope artifact defines measurable T-B red tests instead of leaving a
   diagnostic-only handoff.
3. The artifact uses current source evidence for the PASS-lineage gap and a
   pyarrow schema sample for the arboreal-dendrite interchange shape.
4. The live next increment is T-B; W-D-REDO remains historical/superseded
   watershed-CLI framing.

Residual risk:

- T-B may require a new or amended output contract for PASS parquet emission.
  T-A flags this but does not amend `SC-*` authority because it is design-only.

## T-A Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | accepted | T-A design gates are met; T-B owns contract-first implementation. |

## T-B QA Review

Evidence mode: Static + Ran

Findings:

No blocking T-B findings.

QA observations:

1. The T-B red test was real: the binary target was absent before production
   implementation.
2. The focused CLI contract covers typed missing-input failure and the
   load-bearing PASS-vs-WAT runoff independence.
3. The unit registry now prevents a future regression where
   `watershed_totalwatsed3.Runoff` is recast as `hillslope_wat.Q` lineage.
4. Full Rust gates passed after implementation.

Residual risk:

- The audit residual is still `57.409871 mm`. That is correctly carried into
  T-C rather than waived.
- `openwepp-cli-watershed.rs` and `writers.rs` remain line-count WARN files.

## T-B Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | accepted | T-B implementation and verification gates are met; package closure remains T-C. |

## T-B2 QA Review

Evidence mode: Static + Ran

Findings:

No blocking T-B2 findings.

QA observations:

1. T-B2 closed the governance gap identified in the T-B review: the real-run
   input is now openWEPP-native PASS/WAT output, not Jun-7 legacy interchange.
2. The red tests exercised missing native PASS writer/row builder and missing
   per-hillslope input discovery.
3. `hillslope_pass` unit metadata is covered by the canonical output registry
   test, preventing ungoverned schema drift.
4. Exact-zero closure is not claimed. The PASS identity audit closes at
   numerical noise on a direct publication identity; T-C still owns the full
   water-balance closure.

Residual risk:

- T-B2 does not run the wepppy conservation audit; that is explicitly T-C
  scope.

## T-B2 Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | superseded | This local review is superseded by the later T-B2 runvol area defect review and T-B2-REDO acceptance record. |

## T-B2-REDO QA Review

Evidence mode: Static + Ran

Findings:

No blocking T-B2-REDO findings.

QA observations:

1. The T-B2 area defect is not waived. The artifacts mark T-B2 output as
   superseded and route T-C to `/tmp/openwepp_wshed01_tb2_redo_qarea`.
2. The first attempted redo, `QOFE * record.area`, was rejected by direct
   evidence before acceptance, which closes the governance gap where a
   same-surface audit could pass.
3. The accepted PASS audit checks `runvol` against `Q * Area / 1000`; the
   deleted T-B2 `QOFE * publication area` surface now differs by up to
   `21766.4323911278 m3`.
4. Full gates are recorded, and no comparator-suite subagent was used.

Residual risk:

- The remaining `6948.564523 mm` closure residual is still live T-C work.
- T-C must not use the defective `/tmp/openwepp_wshed01_tb2` output.

## T-B2-REDO Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| - | None | accepted | T-B2-REDO has direct focused, real-run, anchor, annual-bound, totalwatsed3, and full-gate evidence. |

## T-B2-REDO2 QA Review

Evidence mode: Static + Ran

Findings:

1. REDO2 evidence was not yet recorded in the canonical gate table at review
   time.
2. Earlier implementation/test evidence sections still described the
   defective REDO `Q * Area` correction as accepted authority.

Disposition:

- Fixed `gate-results.md` with REDO2 red/focused tests, real rerun, HBP/WAT
  anchor check, PASS-vs-WAT `QOFE * Area`, totalwatsed3 closure audit, full
  gates, and the post-review metadata/fixture fix.
- Amended implementation and contract-test evidence so T-B2-REDO is explicitly
  superseded by T-B2-REDO2.
- Also tightened the focused fixture to distinguish outlet WAT row area from
  the publication-area argument.

## T-B2-REDO2 Finding Disposition

| # | Finding | Disposition | Rationale |
|---|---|---|---|
| 1 | Canonical gate table stale | fixed | REDO2 gate rows and post-review fix note are recorded. |
| 2 | Stale REDO evidence authority | fixed | Evidence artifacts now mark REDO as superseded and route T-C to REDO2 output. |
