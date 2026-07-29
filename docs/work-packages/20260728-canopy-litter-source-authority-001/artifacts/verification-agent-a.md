# Terminal Verification A

Status: `FAIL — CORRECTION REQUIRED`

Evidence class: `Static + Ran`

Verification scope: exact current package files, canonical contract
amendments, management schema/parser, runtime projection, native-forest
consumer test, package gate evidence, terminal documentation, and line-count
disposition. This verification does not rely on either reviewer conclusion.

## Independent execution

Ran on the current tree:

```text
cargo nextest run --test canopy_litter_external_boundary_contract
```

Result: `6 passed, 0 failed`.

```text
cargo nextest run -p openwepp-runner \
  native_forest_yaml_executes_through_the_direct_production_consumer
```

Result: `1 passed, 0 failed`.

```text
cargo nextest run -p openwepp-runner \
  canopy_phenology_02_real_consumers_share_the_typed_native_state
```

Result: `1 passed, 0 failed`.

These runs independently confirm the covered digest/CSV/exhaustive-daily
failures, the real management-to-runner handoff, the published
leaf/needle/fine-woody source operands, and reconstruction of the three
parallel residue recurrences on the authenticated native fixture. They do not
exercise the provenance contradictions below.

## Findings

1. `VA-001 — BLOCKING — identity payload support is not internally bound.`
   `SC-PLANT-001#INV-PLANT-039` and
   `authority-law-and-operand-lineage.md` define an original/identity payload
   as one authenticated source/executable object with one exact-daily support.
   `validate_support_and_provenance` parses both
   `payload.support_start/support_end` and
   `original_observation.support_start/support_end`, but never requires them
   to be equal when `derivation` is absent. An identity file can therefore
   execute dates under a support different from the claimed immutable source
   support while still passing the same-path/same-digest check. Add the
   identity-support equality guard and a rejection vector.

2. `VA-002 — BLOCKING — derived-object provenance is weaker than the reviewed
   contract.` The reviewed law requires every derived payload to identify all
   input objects and digests, and states that source and executable objects
   remain distinct. `ForestLitterDerivation.inputs` is only `Vec<String>`;
   validation proves only that each string is nonempty. It neither requires a
   typed object identity plus SHA-256 digest nor requires the source and
   executable path/digest pairs to differ when `derivation` is present.
   Consequently arbitrary labels, or a derivation attached to one unchanged
   object, pass structural validation. Introduce typed input identities and
   digests, enforce the distinct derived-object relationship, and add negative
   tests.

3. `VA-003 — BLOCKING — an incomplete oven-dry method is accepted.` The
   prospective authority correction requires drying temperature plus duration
   or a constant-mass criterion. `validate_mass_basis` requires a criterion
   only for `dry_to_constant_mass`; `oven_dry` with a finite temperature,
   absent duration, and absent criterion passes. That does not preserve the
   admitted dry-mass method. Require positive duration or an explicit
   constant-mass criterion for `oven_dry` and cover the failure.

4. `VA-004 — BLOCKING FOR TERMINAL DISPOSITION — terminal records contradict
   the implemented tree.` At verification time:

   - `final-disposition.md` still says the package stopped before contract and
     production edits and that the interface is not implemented;
   - the work-package catalog and canopy roadmap still describe the package as
     a historical implementation hold;
   - `gate-results.md` leaves Markdown and dual terminal review/verification
     pending; and
   - the two review artifacts contain prospective review only, while
     `verification-agent-b.md` still says implementation was not authorized.

   These records cannot support a completed final status until terminal review,
   both verifications, documentation validation, catalogs, and final
   disposition are reconciled to the exact implementation result.

5. `VA-005 — MAJOR EVIDENCE DRIFT — the recorded terminal test count and full
   gate identity are not yet auditable as exact-terminal evidence.`
   `contract-test-implementation-evidence.md` records a terminal `5/5` result,
   while the current contract test binary contains and passes six tests.
   `gate-results.md` records a 2,106-pass full profile but does not retain the
   exact command or a source-snapshot identity, and no package-local full-run
   log is present. Record the current six-test result and bind the full-profile
   result to the unchanged terminal source snapshot (or rerun the applicable
   exact-terminal gate if that identity cannot be established).

## Non-blocking checks

- Contract-first ordering is supported by the canonical amendment timestamps,
  pre-implementation gate, recorded expected-red run, and later production
  file timestamps. No contrary ordering evidence was found.
- The real consumer test reconstructs
  `Q = L_leaf + N_ext + W_ext` and each surface/interrill/rill recurrence from
  published operands; it also follows the resulting states into the existing
  depth/frost and cover/erosion consumers.
- Current relevant line counts agree with the package disposition: the new
  schema file is 797 lines, the extracted forest-data file is 25, the contract
  test is 184, and no touched nonexempt Rust file reaches 3,000 lines. Existing
  2,000-line files correctly remain `WARN`.
- The implementation diff is confined to the declared litter-boundary
  write-set. Numerous unrelated CAL-04B/CAL-05 changes coexist in the dirty
  worktree, but the package does not claim them as this increment's work.

## Verification disposition

`FAIL`. The runtime handoff and covered conservation behavior pass, but
`VA-001` through `VA-003` leave authenticated source authority less strict
than the reviewed canonical law. `VA-004` and `VA-005` also prevent truthful
terminal closure on the exact current artifact set. Correct the findings and
request re-verification; predictive needle and fine-woody biology must remain
`AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`.

## Final Terminal Re-verification — 2026-07-28

Status: `PASS`

Evidence class: `Static + Ran`

This final re-verification preserves the immutable initial `FAIL` above and
supersedes its disposition only for the corrected exact tree.

### Independent execution

Ran:

```text
cargo nextest run --test canopy_litter_external_boundary_contract
```

Result: `16 passed, 0 failed`.

Ran:

```text
cargo nextest run -p openwepp-runner -E \
  'test(native_forest_yaml_executes_through_the_direct_production_consumer) \
  | test(canopy_phenology_02_real_consumers_share_the_typed_native_state)'
```

Result: `2 passed, 0 failed`.

Ran:

```text
cargo nextest run -p openwepp-hillslope-orchestrator \
  r7b_constructor_type_size_layout_is_bounded
```

Result: `1 passed, 0 failed`.

Ran:

```text
cargo clippy --workspace --all-targets -- -D warnings
```

Result: `PASS`.

The package records the post-correction exact-head full profile as
`2,117 passed, 29 profile-declared skips, 757.402 seconds`. Production/test
file timestamps and the reconciled terminal artifacts place that run after
the last implementation correction; no later production edit was found.

### Finding closure

1. `VA-001 — CLOSED.` Identity inputs now require exact equality between
   original and executable support in addition to the existing identical
   path/digest requirement. The contract suite directly rejects contradictory
   support.

2. `VA-002 — CLOSED BY SCOPE NARROWING.` This increment is explicitly
   identity-only. Every derived forcing, including an interval-derived daily
   object, fails closed before execution. The implementation no longer claims
   the unimplemented derived-object route, and a direct rejection test binds
   that boundary.

3. `VA-003 — CLOSED.` Every accepted dry-mass record now requires a positive
   drying duration or a nonempty constant-mass criterion. The suite directly
   rejects an incomplete `oven_dry` method.

4. `VA-004 — CLOSED FOR RE-VERIFICATION.` The final-disposition artifact,
   catalog, and canopy roadmap now describe the identity-only implementation
   and retain predictive biology as authority-missing. Their current
   `PENDING` language is truthful: package completion still awaits the other
   independent re-review/re-verification results and final Markdown
   validation. This verification does not prematurely convert those pending
   package gates to `PASS`.

5. `VA-005 — CLOSED.` The contract evidence and gate table now record the
   current 16-test suite, corrected full-profile result, type-size guard, and
   terminal line counts. The exact counts independently observed are:
   `forest_litter.rs` 948, `management.rs` 2,984, `forest_data.rs` 25,
   contract test 397, runner tests 2,890, builder authority 2,844, trace
   builder 2,067, and runtime authority 1,514 lines. No touched nonexempt Rust
   file reaches 3,000 lines.

### Consumer and authority determination

The corrected tree authenticates the vegetation classification and identity
forcing bytes, enforces tissue/class compatibility, publishes
`not_represented` as a null operand with explicit incomplete-ledger status,
and preserves a numeric zero only for supported complete or inapplicable
arithmetic. The native fixture independently reconstructs the three-source
sum, all three parallel residue recurrences, weighted ground mass,
interrill/rill/composite cover, residue depth, frost depth, and the exact
interrill/rill cover operands consumed by active erosion. The source guard
proves one decomposition handoff and no downstream external-source
re-addition.

Final verification A disposition: `PASS`. No `VA-*` blocker remains on the
corrected exact tree. Predictive needle and fine-woody deposition correctly
remain `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`.
