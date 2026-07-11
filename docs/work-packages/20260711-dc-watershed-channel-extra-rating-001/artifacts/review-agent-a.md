# Review agent A — preimplementation technical review

Status: PREIMPLEMENTATION HOLD
Evidence mode: Static and Ran as labeled
Review role: independent technical/authority reviewer
Review date: 2026-07-11

## Disposition

**PREIMPLEMENTATION HOLD.** `INV-CHN-016` establishes the correct
canonical-first, unique full-suffix-closure direction, and the intended-red
suite fails only at the two absent production classifications. Production
parser, network-frame, and WSHED-W5 consumer-test diffs are empty. Production
implementation is not yet authorized because the contract-mandated
both/neither, enabled-rating, and exact-diagnostic obligations are not fully
bound, and the A-H map remains queued.

## Findings

### A-001 — Closure-blocking: both-layout ambiguity is required but untested

Static: Contract family C explicitly requires a vector in which both retained
and skipped layouts close, and Sections 2.1/6.1 require ordinary canonical
retained-layout precedence. No added or existing fixture constructs that
variable-record-count ambiguity. `strict_multi_numeric_comment.chn` proves only
retained-valid/skipped-invalid precedence. Before implementation, add a fixture
whose option-dependent channel structure permits both layouts to close and
assert unchanged canonical retained parsing in strict and compatibility modes.
This is the principal guard against a recognizer that treats any successful
single-record deletion as sufficient.

### A-002 — Closure-blocking: duplicate enabled-rating exclusion is unbound

Static: The contract and spec explicitly exclude a duplicate residual after an
already consumed `icntrl == 4` rating record from `INV-CHN-016`; it must remain
generic `CHN-E-002`. Existing nominal enabled-rating coverage proves only one
required rating row, and no fixture appends a second valid rating-shaped row.
Add final- and, if structurally relevant, multi-channel duplicate-enabled
vectors in both modes so an EOF-oriented or globally lexical implementation
cannot misclassify them as `CHN-E-006`.

### A-003 — Closure-blocking: exact diagnostic payload lacks canonical authority

Static: The new tests require exact
`RatingCurveClosure { line: 15, channel_id: 1, reason:
"icntrl!=4 prohibits structurally recognized rating_curve_line" }`, but the
contract/spec authorize only the `CHN-E-006` class. They do not define which
physical line is reported, which channel owns the prohibited row, or the exact
stable reason string. Contract-derived tests must not create authority. Amend
canonical authority to specify candidate physical line, preceding disabled
channel ID, and the exact reason text (or deliberately relax the tests to the
payload fields that canonical authority makes stable) before production edits.

### A-004 — Closure-blocking: neither-layout and ordinary-error precedence are
only partially bound

Static: The final invalid-domain triple demonstrates one non-recognized
candidate ending in generic `extra_records`, but it does not exercise a
multi-channel suffix for which retaining and skipping both fail with different
typed errors. Because the invariant says "ordinary parser/error precedence"
for neither-layout cases, at least one such vector must pin which ordinary
error wins and prove no speculative-probe error or warning escapes. Bind it in
both modes.

### A-005 — Closure-blocking readiness gap: A-H obligations are not mapped

Static: `artifacts/obligation-to-test-map.md` is still queued, while the amended
contract makes A-H current-scope obligations. Current coverage is visibly
partial: A lacks an explicit unchanged frame projection binding for the new
vectors; C lacks both-layout and enabled-duplicate branches; E does not bind a
wrong-arity required `icntrl == 4` row; F does not bind non-finite values across
the declared real-token families; and H lacks the ambiguity/fail-closed cases
from A-001 through A-004. G is legitimately `N/A`. Complete the map with exact
test names and either add missing contract-derived cases or narrowly amend
over-broad obligations before implementation review is rerun.

### A-006 — Non-blocking implementation constraint: keep probing zero-repair,
side-effect-free, and resource-bounded

Static: The authority correctly requires canonical retained closure first,
then at most the one immediate post-control candidate, with exact EOF after all
remaining declared blocks. Implementation must use a zero-anomaly suffix probe;
it must not recursively apply `INV-CHN-016` inside the suffix. Probe execution
must not append compatibility warnings, mutate parsed channels/output, or emit
an option-derived side effect. The declared one-candidate-per-boundary bound
prevents combinatorial repair search, though a repeated suffix scan may still
be quadratic in `nchan`; record the actual complexity/resource bound in the
implementation evidence and keep recursion bounded or absent.

## Confirmed authority and provenance

Static:

- Baseline repository HEAD is exactly
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Pinned `wshinp.for:370-433` directly proves three arbitrary comment reads,
  fixed channel records through control at line 418, and the conditional rating
  read only under `icntrl == 4` at lines 429-430.
- Contract `0.1.2` and spec `0.1.1` correctly label prohibited-row recognition
  as openWEPP inference rather than legacy behavior.
- Canonical retained-layout success takes precedence; only a valid rating
  candidate whose removal alone yields full semantic suffix and EOF closure may
  become `CHN-E-006`.
- Two-/four-token and invalid-domain final residuals remain `CHN-E-002`.
- Numeric three-token comments and numeric-leading prose remain unrestricted
  comment records when the canonical retained layout closes.

Reviewed hashes:

- contract:
  `998e4c99acfac5a392f664964bd5da693bb77a8089560c972ff0806fb52130bd`
- spec:
  `1738434625f9778bcee872f3d5885eccee26662a37cfddbf3d383298afab9d56`

## Diff-boundary audit

Ran:

- `git diff --quiet -- crates/openwepp-input-contract/src/parsers/watershed_channel.rs`
  returned `0`.
- `git diff --quiet -- crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
  returned `0`.
- `git diff --quiet -- tests/integration/wshedw5_typed_watershed_runtime_contract.rs`
  returned `0`.
- `git status --short` showed no untracked or modified file at any of those
  paths.

The production parser, real network-frame consumer, and consumer contract test
therefore remain untouched at this preimplementation gate.

## Intended-red execution

Ran:

```text
cargo nextest run --test infile_watershed_channel_parser_contract
Nextest run ID: 0d1ee8bf-9de5-4719-aed7-2d710e7ea7eb
24 tests run: 22 passed, 2 failed, 0 skipped
```

The only failures were:

- `final_no_rating_residuals_use_structural_rating_classification`
- `multi_channel_extra_rating_is_recognized_only_by_unique_suffix_closure`

Both fail because the current production parser returns its existing generic
closure behavior instead of the newly asserted exact `RatingCurveClosure`.
Generic residual and numeric-comment non-regression assertions passed. This is
the intended red state, but it is insufficient to lift A-001 through A-005.

## Re-review lift conditions

1. Canonically authorize or deliberately narrow the asserted exact diagnostic
   payload.
2. Add and pass intended-red bindings for both-layout ambiguity,
   multi-channel neither/error precedence, and duplicate enabled-rating scope
   in both modes.
3. Complete the A-H obligation-to-test map and resolve its missing bindings.
4. Reconfirm the same production/network-frame diff boundary remains empty.
5. Rerun the focused suite and record that only the complete, intended
   production-absent behavior set is red.

## Re-review after accepted corrections

Status: **PREIMPLEMENTATION PASS**
Evidence mode: Static and Ran as labeled
Re-review date: 2026-07-11

### Finding disposition

Static: All initial findings are corrected in-envelope and none is deferred.

- `A-001` is closed. The canonical fixed-arity proof is sound: with retained
  and deleted suffixes offset by exactly one record, the first remaining
  channel's retained two-token geometry record necessarily occupies the
  deleted layout's single-token `flgout` position. Both layouts therefore
  cannot close before the later optional rating position can alter record
  count. Retained-only, deleted-only, and neither cases are the complete
  executable partition.
- `A-002` is closed by
  `duplicate_rating_after_enabled_branch_remains_generic_extra_input`, which
  binds exact `CHN-E-002` residual closure in both modes after a canonical
  `icntrl == 4` rating row.
- `A-003` is closed. Canonical contract Section 2.1 now authorizes the candidate
  physical line, preceding disabled channel ID, exact stable reason string,
  and no-partial-output behavior asserted by the E006 tests.
- `A-004` is closed by
  `neither_suffix_layout_preserves_the_ordinary_retained_error`, which binds
  the unchanged retained-layout `TokenParse` line, field, token, and
  `CHN-E-001` ID in both modes.
- `A-005` is closed for the preimplementation gate. The A-H map gives exact
  current test-function bindings for the changed B/C/H behavior, identifies
  the static both-layout proof, reviews G as `N/A`, and names the terminal
  D/E/F and frame-consumer tests required after implementation. The explicit
  rule that no planned or intended-red binding may remain at terminal
  disposition prevents gate deferral.
- `A-006` is closed as a design gate. The predeclared implementation extracts a
  shared canonical channel/rating validator, isolates probe warnings/output,
  preserves retained-error fallback, considers only the immediate candidate,
  and memoizes suffix states by physical cursor and remaining channel ID. This
  is a bounded zero-repair recognizer, not recursive repair search or repeated
  unbounded suffix parsing.

### Authority and provenance re-check

Static:

- Contract `0.1.2` hash is
  `835facb44b2065f5c4505228d83d52200e8472e9826e54db75efe553850cdb0c`.
- Spec `0.1.1` hash is
  `09b510d62b74ba234c906e56fd329a30b251800dd3f6ca140872d198ab3e14c5`.
- The pinned baseline repository remains exactly
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No `/workdir/wepp-forest/src/...` citation remains in the amended contract or
  spec. Legacy citations consistently use the pinned baseline, including the
  corrected `inidat.for:1157-1160` version anchor and exact
  `wshinp.for:370-433` channel-loop/rating-read anchor.
- Direct legacy evidence remains limited to conditional record consumption.
  Unique closure, EOF/error precedence, and typed E006 payload are truthfully
  labeled openWEPP inference.

### Intended-red and diff-boundary re-check

Ran:

```text
cargo nextest run --test infile_watershed_channel_parser_contract
Nextest run ID: e552aa17-8f6b-4b3a-9c95-3d70c5d0a470
26 tests run: 24 passed, 2 failed, 0 skipped
```

The only failures remain the intended production-absent classifications:

- `final_no_rating_residuals_use_structural_rating_classification`
- `multi_channel_extra_rating_is_recognized_only_by_unique_suffix_closure`

The retained numeric-comment, neither-layout, duplicate-enabled-rating, and
generic residual vectors pass in strict and compatibility modes.

Ran: `git diff --quiet` returned `0` for the production parser,
`network_frame.rs`, and the WSHED-W5 consumer contract test. Targeted
`git status --short` was empty for the same paths. Production and real-consumer
surfaces therefore remain untouched.

### Re-review verdict

**PREIMPLEMENTATION PASS.** The contract-first safety net now uniquely and
exactly defines the correction, the focused red state is isolated to the two
missing production classifications, provenance is pinned, and the planned
probe is shared-validator, side-effect-free, memoized, and resource-bounded.
Production parser implementation may begin within the declared package
envelope. Terminal PASS still requires completing every planned A-H binding,
the WSHED-W5 frame projection assertion, coverage/CRAP gates, full validation,
dual final review, and dual verification.

## Final technical review

Status: **FINAL TECHNICAL PASS — full workspace gates pending**
Evidence mode: Static and Ran as labeled
Review date: 2026-07-11

### Findings

No closure-blocking or non-blocking technical finding remains.

### `INV-CHN-016` implementation audit

Static:

- Ordinary parsing and probing share `parse_channel_block` and
  `parse_rating_curve_line`; candidate arity, numeric, finite, domain, mode,
  option, guard, and warning semantics do not have a shadow validator.
- At each successfully parsed `icntrl != 4` boundary, retained canonical suffix
  closure is checked first. A retained success returns without inspecting or
  deleting the candidate, preserving numeric three-token comments and
  numeric-leading prose.
- Only the immediate physical record is parsed as a candidate. Exact E006 is
  emitted only when that full-domain rating record is valid, retained closure
  is false, and advancing by exactly that one record makes every remaining
  declared channel plus EOF close.
- The emitted payload is exactly the candidate physical line, preceding
  disabled channel ID, and canonical reason
  `icntrl!=4 prohibits structurally recognized rating_curve_line`. Returning
  `Err` exposes no typed partial output.
- If the candidate is invalid or the deleted suffix also fails, recognition
  returns `Ok(())` and the ordinary parser proceeds from the untouched cursor;
  the neither-layout fixture proves its original line/field/token and
  `CHN-E-001` priority are preserved.
- Enabled `icntrl == 4` blocks never call prohibited-record recognition, so a
  duplicate enabled rating row reaches the ordinary extra-record closure and
  remains exact `CHN-E-002`.
- The contract's fixed-arity proof correctly makes simultaneous retained and
  deleted closure unreachable: the one-record offset presents retained
  two-token geometry to the deleted layout's one-token `flgout` slot before an
  optional rating position can change record count.

### Memoization, side effects, and resource bound

Static: The memo is local to one parse and keyed by `(physical cursor, next
channel_id)`, which fully identifies a suffix under invocation-constant
`nchan` and parse options. Each visited canonical state is parsed at most once;
a successful state means its complete tail reaches EOF, while a failed later
state also makes every recorded prefix unable to close. The probe is iterative,
not recursive, and explores only retained and one-record-deleted states. Parsed
probe definitions and compatibility warnings remain local and are dropped;
the real cursor, channel vector, and public warnings are not mutated. This is a
bounded recognizer rather than repair search.

### Exact semantic vectors and accepted-output identity

Static and Ran:

- Both strict and compatibility modes bind final two-/three-/four-token and
  invalid-domain residuals, multi-channel unique deletion, retained numeric
  comment, neither-layout error priority, and duplicate-enabled-rating scope.
- `expect_err` plus exact variant/payload matching proves prohibited input is
  never accepted or partially returned.
- Nominal parser assertions cover every channel field and exact rating values.
- WSHED-W5 `watershed_channel_rating_projection_preserves_optional_fields`
  proves the real unchanged `WatershedNetworkFrame` consumer receives exact
  `rccoef=1.25`, `rcexp=1.50`, and `rcoset=0.10` for `icntrl=4`, and `None` for
  `icntrl=1`. `network_frame.rs` has no diff.

Ran:

```text
cargo nextest run \
  --test infile_watershed_channel_parser_contract \
  --test wshedw5_typed_watershed_runtime_contract
Nextest run ID: ef5f5f18-1dff-49cf-ba38-4b01ca175baf
58 tests run: 58 passed, 0 skipped
```

### A-H, coverage, CRAP, and chronology

Static/Ran:

- The obligation map binds A through F and H to exact test functions; G is
  correctly reviewed `N/A` because parsing/projection computes no conserved
  quantity, while its output-identity concern is bound through WSHED-W5.
- Raw terminal JSON confirms 662/665 lines (`99.549%`), 793/798 regions
  (`99.373%`), and 31/31 functions. The minimum named-function coverage is
  `canonical_suffix_closes` at `96.970%`; no denominator exclusion is used.
- Raw LCOV-backed CRAP JSON has zero target rows above 30. The maximum is
  `parse_watershed_channel_from_str` at CC 21 / CRAP 21.0005;
  `parse_channel_block` fell from CC/CRAP 36 to 9, and extracted helpers are
  independently below 30.
- Chronology is valid: production correction first passed 26/26; test-only
  expansion then fixed the safety net at source
  `675ef55135e4f89d35f822cdbc836354a4215a6c10c5cf035cbefe51192635dc`
  and test
  `7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`
  with 38/38 and 99% coverage; only then was the CRAP-36 block decomposed.
- Live terminal identities match the artifacts: source
  `a2b18016361731f8f568857de4210f5e207b03683744ff42f53e41323d206b1d`
  and unchanged focused test
  `7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`.
  Recorded LCOV, coverage JSON, CRAP JSON, contract, and spec hashes also match
  their live files.

### Numeric, line-count, and security disposition

Static and Ran:

- The helper extraction preserves the exact ordered sequence
  comments → enums → geometry → erodibility/cross-field guard → control →
  rating → effective-control derivation. No formula, threshold, arithmetic
  grouping, field meaning, accepted grammar, or floating comparison changed.
- Touched Rust files are 956, 932, and 1,250 lines, all below the 2,000-line
  warning and 3,000-line closure thresholds.
- No `unsafe`, subprocess, network, authentication, secret, or trust-boundary
  mechanism was added. Candidate exploration remains one immediate record per
  disabled boundary with invocation-local memoized state.
- `cargo fmt --check` and targeted `git diff --check` passed in this review.

### Pending external gate

`artifacts/gate-results.md` was still queued when this review was written
because the root closure loop was running concurrently. This review does not
claim those commands passed. A failure in formatting, workspace Clippy,
full-profile nextest, deny, Markdown, or another package-required gate must
block terminal disposition until corrected and revalidated.

### Final verdict

**FINAL TECHNICAL PASS.** The implementation, exact semantics, consumer path,
A-H bindings, safety-net chronology, coverage/floors, CRAP, numeric behavior,
line count, and security posture satisfy the FQ-03 technical envelope. Package
closure remains contingent only on successful recorded completion of the
concurrently running full gates, the second independent final review, review
disposition, and dual verification.
