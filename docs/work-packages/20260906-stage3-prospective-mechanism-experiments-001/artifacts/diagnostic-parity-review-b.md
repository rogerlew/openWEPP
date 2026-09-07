# Natural-limit diagnostic parity — independent review B

Static: independently inspected the original natural-limit fixture and actual
solver failure, diagnostics conversion/validation, and audit interfaces before
disposition. Then read the complete proposed specification in
`diagnostic-parity-review-a.md`, SHA256
`c6bec31887d772e567bd4526e09334ce7acd959b03a60c62a88b9f5f2408537a`,
and its exact two-path write map in R-runtime-test-plan. No Rust execution or
source edits by B. Root/work-package guidance discovered for this artifact.

## Findings and authority limits

- **Medium — complete natural-limit parity remains unproved by the existing
  logs.** A/F printed residual vectors match, but the unchanged golden test
  aborts before later pivot/matrix/ordered-row checks. That observation is not
  full failure DTO, rollback, or Newton-state parity.
- **Medium — Debug or PartialEq alone is insufficient.** NumericalFailure's
  custom Debug omits `failed_solution`; float equality loses signed-zero
  distinction. The common observer must bind all eleven failure fields
  exhaustively and encode every float/Option float by bits.
- **Low — correct the descriptive DTO count, not its coverage.** The current
  NumericalDiagnostics declaration has twenty-four fields, not twenty-five.
  Complete typed binding and serialization, including all nested fields,
  controls coverage; a prose count cannot substitute for it.

These are evidence/QA obligations, not evidence that F or R caused the retained
release golden failure. This observer does not relax or replace that golden.

## Prospective source cut — GO to author

Only two common cfg(test) source paths:

- `crates/openwepp-land-surface-energy/src/covered_oracle_conformance_tests.rs`:
  child binding only; preserve the original natural golden and assertions.
- `crates/openwepp-land-surface-energy/src/covered_natural_limit_diagnostic_parity_tests.rs`:
  ignored diagnostic test and local observation helpers.

Main/F authoring is permitted now; R must wait for the parent's active full-gate
source freeze to end. No production, solver, public API, test-support runner,
science-contract, fixture authority or stopping-policy edit is needed.

The exact existing setup at covered_oracle_conformance_tests.rs:1150 supplies
the four hydraulic assignments and fifteen-entry initial vector; reuse its
private fixture()/column() factories. Preserve authority, occupancy count,
caps, forcing and Stage 3 absence. This is bounded duplication of test-vector
setup, not a second constitutive calculation or new parameter search.

### Binding requirements for the concrete test

1. Execute the actual solve once under the existing detailed audit. Require
   the actual `Ok(Rejected(IterationLimit))`, no candidate, and the still-bound
   iteration/shape/unit/finiteness conditions. A mathematical rejection is not
   a Result error: do not require an erroneous audit Solve failure count.
2. Exhaustively destructure all eleven NumericalFailure fields without `..`.
   Preserve kind, iteration/backtracking counts, occupancy, bounds and order;
   bit-encode both numeric vectors including failed solution, four numeric
   fields in each ordered residual, all five optional StepNorms, pivot and
   matrix norm. Do not replace the actual failure with a manufactured one.
3. Snapshot the actual input column by owned typed clone, complete derived
   structural Debug and explicit nested float bits before/after. Bind the
   traversal to the reviewed thirteen-type field map. Include initial-trial
   bits and assert caps/Stage 3 absence. Use no raw memory or padding bytes.
4. Pass this actual failure to the unchanged public
   `rejected_numerical_diagnostics` at transaction.rs:1173. Use a fixed declared
   test identity, validate the DTO and retain its complete serialization plus
   float-bit view. Include residual/order/unit, pass/solve/error code, caps,
   bracket/Option values and every owner-lineage entry. The converter's fixed
   test owner hashes prove diagnostic conversion lineage, not execution or
   rollback of external physical owners.
5. Retain the complete unfiltered audit and require zero dropped events and
   overflow, successful session finish, nonempty actual iteration/sweep/probe
   population and balanced lifecycle counts. `Session::finish` checks live
   scopes/overflow but does not reject dropped_events, so that assertion must
   be explicit. Cross-arm event/count comparison has no numeric normalization.
6. Observe ordered SweepBase bits and final failed-solution bits as the actual
   Newton-state trajectory. The existing audit does not expose all line-search
   trials, Jacobians or LU deltas; richer claims remain with R's separate trace
   corpus. Execute identical frozen test source/profile/flags in A/F/R and
   compare every retained input/failure/DTO/event field before declaring PASS.

## Zero-anchor/replay qualification

The original column factory sets HistoricalV8 at line470 and
`stage3_lower_boundary: None`, `stage3_optical: None` at lines495–496.
`ValidatedCoveredEvaluationInputs::stage3_identity_anchor_k` starts with
`let boundary = self.stage3_boundary?` at solver_covered_evaluation.rs:1919.
Absent Stage 3 therefore returns no anchor for every coordinate, before
ground/soil indexing. R component replay also requires its represented-snow
eligibility. Zero identity-anchor/component-replay starts on this authentic
historical route are appropriate mandatory observations, not a failed savings
gate. Do not alter the fixture to create replay participation.

## Disposition

**GO for the exact common test-only source scope above.** Concrete source
review and all three same-release executions remain pending. Complete
natural-limit differential evidence is HOLD until those records compare.
Retain the existing optimized golden FAIL, require separate default-debug
conformance and R arithmetic/custody evidence, and make no full-workspace PASS,
external-owner rollback, scientific waiver or production-promotion claim.

## Concrete main/F source confirmation

Static GO on the common child file SHA256
`88d8a16a433df8b25d16fab76ae1364743f724c72910ef431a4bfbc45125676c`,
verified identical in main and F. Existing covered conformance file changes
only by the child-module binding. No R copy or source unfreeze is implied.

Read the complete 429-line child. Independent read-only comparison confirms
its four hydraulic assignments and entire initial vector are byte-identical
to the original natural test. Its complete `column_float_bits` body is exactly
the previously reviewed R thirteen-type traversal, not an abbreviated proxy.

The concrete test exhaustively encodes all eleven failure fields and every
current residual/step/optional float by bits. It calls the actual public DTO
converter on this failure, validates, serializes the complete DTO, roundtrips
and compares both typed and full float-bit views. Actual column typed clone,
derived Debug and complete float bits are asserted before/after; initial-trial
and absent-cap/Stage 3 context remain explicit. Diagnostic identity hashes are
labeled conversion context only.

The detailed audit has explicit dropped_events/overflow checks, successful
finish, balanced count lifecycle and actual nonempty iteration/sweep/probe
population. Zero anchors/replay is required for the unchanged historical
fixture. Every full event and count is emitted, alongside ordered sweep bases
and actual final failed solution. No all-line-search/LU or external-owner
workflow claim was introduced. The actual mathematical rejection correctly
requires one completed, zero errored Result-level solve scope.

Nonblocking test-hardening follow-up: assert declared ordered-row metadata
length before zipping identity/unit checks. The current bound arrays both
contain fifteen rows; actual field/count serialization and cross-arm equality
are complete, so this does not block the reviewed cut's execution.

No source defect blocking the parent-controlled executions was found.
All three actual release records and exact comparison remain NOT RUN by B;
this source GO is not differential PASS or golden conformance.

Parent-requested cardinality hardening is now present before the metadata zip:
the declared ordered-row count must equal the actual ordered-residual count.
Confirmed identical formatted main/F child SHA256
`0d27a1e53cfa9c6ac719dea7cee6f713c88dd85fe97119a484386f37954f1467`.
This closes the nonblocking follow-up above; source GO continues on this cut.
No numeric equality, golden, production behavior, or R frozen source changed.

### Post-Clippy exact placement correction

The parent-authorized documentation backticks and statement-local expected
constant-assertion lint were reviewed prospectively. Actual Clippy rejected
the statement macro attribute as ignored; that attempt is not a passed gate.
The corrected common file places the same justified expectation on a tiny
private `require_release_posture` helper containing only the unchanged
`assert!(!cfg!(debug_assertions), ...)`; the ignored test calls it first.
Inspected this exact helper and verified all three arms now have SHA256
`2c219dbd3626195f735a9ee4db3a74b173462a1ef13fa290afdefa6a9e1dd156`.
Static GO continues: no numeric, golden, audit, DTO or rollback assertion
change; release posture remains fail-closed. Current execution is separate.
