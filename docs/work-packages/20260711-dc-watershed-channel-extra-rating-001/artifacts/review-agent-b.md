# Review agent B

Status: **PREIMPLEMENTATION HOLD**
Evidence mode: Static and Ran

## Authority and intended-red evidence

Static: pinned baseline HEAD is
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. Exact direct anchors are:

- `infile.for:401-419` for unit-18 open, initial datver positioning, and
  `verchk` dispatch;
- `wshinp.for:347-368` for declared-count/header reads and cross-file count
  closure;
- `wshinp.for:370-433` for the declared channel loop, arbitrary comments at
  376-378, fixed records through control line 418, and rating read only under
  `icntrl==4` at 429-433;
- `inidat.for:1157-1160` for `chnchk=94.301` at line 1159; and
- `verchk.for:19-31` for version reread/rejection.

The baseline directly supports conditional record consumption, not exact EOF,
leftover, `CHN-E-006`, or unique-repair diagnostics. The amended contract
correctly labels `INV-CHN-016` and its typed-error policy as openWEPP
inference.

Ran: `cargo nextest run --test infile_watershed_channel_parser_contract`
produced the intended-red state: `22/24` passed and only
`final_no_rating_residuals_use_structural_rating_classification` and
`multi_channel_extra_rating_is_recognized_only_by_unique_suffix_closure`
failed (run `390df0f4-7015-4a94-b7ef-987ed14d813f`). Static/Ran: target parser
and network-frame production diffs are empty. Generic final residuals and the
numeric-comment retained layout already pass.

## Findings

1. **High — required ambiguity/error-precedence vectors are incomplete.**
   Contract family C explicitly requires retained-success, unique
   removed-success, **both**, and **neither** layouts, but the new tests bind
   only retained-success and unique removed-success. Add constructible both/
   neither vectors, or prove one class structurally impossible and remove it
   from normative obligations. For neither, assert the exact ordinary error
   from the retained layout. Also bind the newly explicit duplicate residual
   after an enabled `icntrl==4` rating record to generic `CHN-E-002`. These are
   central to proving the recognizer is deterministic rather than a repair
   heuristic.
2. **High — “ordinary parser/error precedence” is not exact enough to
   authorize implementation.** `INV-CHN-016` defines retained-layout success
   precedence, but when neither or both layouts close it does not name which
   diagnostic wins. Ratify that the unmodified retained-layout parse runs with
   existing field/record order and its exact typed error is preserved; probing
   may only replace that result with `CHN-E-006` for the single unique case.
   Tests must bind line, channel, variant, and reason/payload for both modes.
3. **Medium — canonical spec provenance remains mixed and partly incorrect.**
   The new recognition paragraphs are pinned, but the same amended spec still
   contains many `/workdir/wepp-forest/src/...` citations. In particular its
   version evidence cites `inidat.for:1160-1163`, which misses `chnchk` at
   pinned line 1159. Re-anchor the touched surface's legacy citations to
   `/workdir/wepp-forest_260430_baseline` and the pinned commit; use the exact
   anchors listed above. Do not let exploratory HEAD remain co-authority beside
   the pinned evidence.
4. **Medium — A-H binding is not yet reviewable.** The contract now has useful
   A-H obligations, but `obligation-to-test-map.md` is still queued. Before
   production authorization, at minimum bind every `INV-CHN-016` B/C/H clause
   to exact intended-red or passing test functions, including strict/compat
   parity, valid rating-shaped and numeric-leading comments, valid next-channel
   blocks, invalid-domain residuals, duplicate-enabled-rating residual, and no
   partial output. The terminal package must additionally bind all D/E/F
   domain, missing, and non-finite families plus valid frame projection.
5. **Medium — implementation must have an explicit bounded, side-effect-free
   probe design.** A final trailing-record check cannot recognize the
   intermediate case because the extra record is greedily consumed as the
   next channel's comment. The preimplementation plan should require a pure
   suffix/layout probe or raw-block parse that shares the canonical rating
   validator, emits no warnings/output, and never accepts input by deletion.
   “One candidate at each boundary” alone does not prevent quadratic suffix
   reparsing when `nchan` is unbounded; require memoized/linear reuse or an
   explicit canonical channel-count bound and resource test.

## Passing points

- The proposed rule is not a lexical “three floats means rating” heuristic:
  the candidate must satisfy rating arity/numeric/finite/domain rules and be
  the unique single-record removal that restores full declared suffix/EOF
  closure.
- Canonical retained-layout success has priority, and the exact numeric triple
  `comment_1` plus numeric-leading `comment_2` vector proves valid comments are
  not reclassified.
- Final two-/four-token and invalid-domain residuals remain `CHN-E-002`; only
  the valid three-token unique case is intended `CHN-E-006`.
- The rule changes diagnostic classification only. It broadens no accepted
  grammar, changes no rating physics, and cannot canonicalize-and-proceed.
- Contract/spec hashes match the pre-review artifact:
  `998e4c99acfac5a392f664964bd5da693bb77a8089560c972ff0806fb52130bd`
  and `1738434625f9778bcee872f3d5885eccee26662a37cfddbf3d383298afab9d56`.

## Verdict

**PREIMPLEMENTATION HOLD.** The core unique-layout authority is ratifiable and
the intended-red state is correctly isolated, but ambiguity/error precedence,
pinned spec provenance, and exact target-obligation binding must be closed
before production edits. All findings are in-envelope and are not legitimate
deferred boundaries.

## RE-REVIEW — accepted preimplementation fixes

Verdict: **PREIMPLEMENTATION HOLD**.

Evidence mode: Static and Ran.

Closed findings:

- Exact ordinary precedence: PASS. `INV-CHN-016` now requires canonical
  retained-layout success first and returns the unmodified retained-layout
  line/field/context/error when neither layout closes. The strict/compat
  `neither_suffix_layout_preserves_the_ordinary_retained_error` vector binds
  exact `CHN-E-001`, line 18, field `ishape`, and token text.
- Both-layout case: PASS by static proof. With fixed channel-block arities, a
  one-record offset necessarily presents the ordinary two-token geometry
  record to a single-token enum slot before any optional rating position, so
  retained and deleted suffixes cannot both close. The obligation now records
  the proof rather than requiring an impossible runtime vector.
- Duplicate enabled-rating scope: PASS. Contract and spec explicitly leave a
  residual after an already consumed `icntrl==4` rating record outside
  `INV-CHN-016`; the named strict/compat vector asserts exact generic
  `CHN-E-002` payload.
- Probe design: PASS for implementation authorization. The package predeclares
  a shared canonical block/rating validator, local warning/output state,
  canonical-first order, immediate single-candidate scope, memoized suffix
  states keyed by cursor/remaining channel, untouched-error fallback, and no
  acceptance by deletion.
- Canonical prohibited-extra payload: PASS. Contract and intended-red tests
  agree on `RatingCurveClosure`, candidate physical line, owning channel ID,
  exact reason
  `icntrl!=4 prohibits structurally recognized rating_curve_line`, both-mode
  identity, and no typed partial output.
- Intended-red isolation: PASS. Ran
  `cargo nextest run --test infile_watershed_channel_parser_contract`; `24/26`
  passed and only the final and multi-channel exact `CHN-E-006` vectors failed
  (run `1eb6713a-7c7e-4cf4-90e9-40406973c7b2`). Target parser, network-frame
  production, and WSHED-W5 test diffs remain empty.

Remaining findings:

1. **High — the claimed exact A-H map still contains non-exact placeholders.**
   Family D binds `G-CHN-001` through `G-CHN-009` and `G-CHN-013` to “existing
   named enum/roughness/rating tests,” not actual function names. Family H
   binds to “all B/C functions” and a planned unnamed ambiguity/no-partial
   assertion rather than enumerating exact functions. Replace these shorthands
   with every concrete test-function name/contract clause. Planned terminal
   tests may remain labeled planned, but their stable names and exact clauses
   must be explicit before the map can claim preimplementation completeness.
2. **Medium — paths are pinned, but several line citations still do not prove
   their stated verbs.** Examples in the amended spec:
   - control override is at `wshinp.for:422-425`, not `428-431`;
   - roughness clamp is at `409-411`, not `415-417`;
   - `ishape` read/normalization is at `380-384`, not `386-390`;
   - `flgout` override is at `388-390`, not `394-397`;
   - `CHN-GAP-004` cites `inidat.for:1162`, but `chnchk=94.301` is line 1159;
   - `CHN-GAP-005` cites `wshinp.for:431`, but `ctlslp <- slplst` is line 425.
   Reconcile every cited range with the source operation it claims. Replacing
   the directory prefix alone does not satisfy exact provenance truthfulness.

Current contract/spec hashes match `contract-and-provenance.md`:
`835facb44b2065f5c4505228d83d52200e8472e9826e54db75efe553850cdb0c`
and `09b510d62b74ba234c906e56fd329a30b251800dd3f6ca140872d198ab3e14c5`.
Pinned baseline HEAD remains
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Final re-review disposition: **PREIMPLEMENTATION HOLD**. The authority and
implementation shape are now technically adequate, but exact obligation
bindings and citation truthfulness remain current preimplementation gates.
Production edits remain prohibited until they are fixed and reverified.

## FINAL PREIMPLEMENTATION RE-REVIEW

Verdict: **PREIMPLEMENTATION PASS**.

Evidence mode: Static and Ran.

The two remaining preimplementation findings are closed:

- The A-H map now gives exact Family D and Family H test-function bindings.
  Family D enumerates the current datver, enum, roughness, rating-domain, and
  structural-rating functions and gives the planned expansion the stable name
  `all_channel_real_domain_families_are_exact`. Family H enumerates all six
  fail-closed functions directly; its error-return assertions prove that no
  typed partial output is returned. No `all B/C functions`, `existing named
  tests`, or unnamed planned-test placeholder remains in either family.
- Every challenged legacy verb is now anchored to the pinned baseline operation
  that performs it: `chnchk=94.301` at `inidat.for:1159`; `ishape` read and
  normalization at `wshinp.for:380-384`; `flgout` read and override at
  `388-390`; the roughness clamp at `409-411`; the no-control-section override
  at `422-425`; `ctlslp <- slplst` specifically at line 425; and the complete
  channel loop/conditional rating-read context at `370-433`. Direct inspection
  confirmed baseline HEAD
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Current hashes agree with `contract-and-provenance.md`: contract
`835facb44b2065f5c4505228d83d52200e8472e9826e54db75efe553850cdb0c`;
spec `2faf80f9285099711c8b0f169ff3d69ee16c252822a4abf7735a0424cf7e199f`.

Ran `cargo nextest run --test infile_watershed_channel_parser_contract` (run
`b4b63eeb-880b-4251-bc33-5126b68de8bb`): 26 tests ran, 24 passed, and exactly
the two intended `CHN-E-006` tests failed. The parser, network-frame, and
WSHED-W5 production-target diffs remain empty.

Final disposition: **PREIMPLEMENTATION PASS**. The contract authority, exact
error precedence, ambiguity proof, bounded side-effect-free probe design,
canonical diagnostic payload, exact obligation bindings, and pinned provenance
are adequate to authorize the package's production implementation phase.

## FINAL REVIEW

Status: **FINAL GOVERNANCE/PROVENANCE PASS — full workspace gates pending**

Evidence mode: Static and Ran as labeled.

### Findings

No blocking or non-blocking review finding remains.

### Correction-envelope and authority audit

Static:

- The changed semantic surfaces remain inside the declared write envelope:
  canonical contract/spec, parser, contract vectors/fixtures, unchanged-frame
  consumer assertion, and package evidence. `network_frame.rs` is unchanged;
  no routing, rating physics, serialization, public field meaning, or unrelated
  policy implementation changed.
- Contract `0.1.2` and spec `0.1.1` remain aligned with the code and exact
  vectors. Their live SHA-256 hashes are respectively
  `835facb44b2065f5c4505228d83d52200e8472e9826e54db75efe553850cdb0c`
  and `2faf80f9285099711c8b0f169ff3d69ee16c252822a4abf7735a0424cf7e199f`.
- The legacy repository is still pinned at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. Direct inspection confirms
  `chnchk=94.301` at `inidat.for:1159`; `ishape` read/normalization at
  `wshinp.for:380-384`; `flgout` read/override at `388-390`; roughness clamp at
  `409-411`; no-control override at `422-425`; `ctlslp <- slplst` at line 425;
  and complete channel-loop/conditional-rating context at `370-433`. No stale
  exploratory-path or superseded verb-to-line citation remains on the amended
  authority surface.

### Structural-recognition and fail-closed audit

Static:

- Ordinary and probe parsing share `parse_channel_block` and
  `parse_rating_curve_line`; the candidate therefore uses canonical arity,
  numeric, finite, domain, mode, option, and warning semantics rather than a
  shadow or lexical classifier.
- Retained canonical suffix closure is tested first. Only the immediate record
  is considered after `icntrl != 4`, and E006 is returned only when that record
  is a fully valid rating row and advancing by exactly one record closes every
  remaining declared channel plus EOF. Numeric three-token comments and
  numeric-leading prose remain accepted only through the ordinary retained
  layout.
- Candidate failure or deleted-suffix failure leaves the real cursor untouched
  and resumes ordinary parsing, preserving the exact neither-layout
  line/field/token/E001 payload. Enabled-rating duplicates bypass recognition
  and retain generic E002. The exact E006 line/channel/reason payload matches
  contract and tests, and returning `Err` exposes no typed partial output.
- Probe definitions and warnings are invocation-local and discarded. The real
  channel vector, cursor, and warning list are not mutated by probing. The
  iterative memo keyed by `(physical cursor, next channel_id)` bounds work to
  reachable retained/one-record-deleted suffix states; it performs no recursive
  repair or accept-by-deletion path.
- The fixed-record arity proof still excludes simultaneous retained/deleted
  closure. Thus no ambiguity branch is hidden by the implementation.

### Tests, consumer proof, and initial-finding disposition

Ran:

- `cargo nextest run --test infile_watershed_channel_parser_contract`:
  run `3970609d-cc23-4153-a6fe-b7b3fac7d36c`, 38/38 passed.
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`:
  run `1fabc8a8-f462-42fc-9a3e-74f677657b1e`, 20/20 passed.
- `cargo fmt --check` and targeted `git diff --check` passed.

Static/Ran:

- Strict/compat vectors cover final two-/three-/four-token and invalid-domain
  residuals, deleted-only multi-channel closure, retained numeric comments,
  neither-layout precedence, duplicate-enabled-rating scope, missing/domain/
  non-finite families, and fail-closed output behavior. The terminal A-H map
  contains exact function bindings for every applicable family; G is correctly
  reviewed N/A for conservation and separately binds unchanged frame
  projection.
- WSHED-W5 calls the real unchanged `WatershedNetworkFrame::from_parsed_inputs`
  path. `build_channel_controls` reads the parser's optional rating record, and
  the test observes exact `1.25/1.50/0.10` frame values for `icntrl=4` and
  `None` for `icntrl=1`. This is consumer-path evidence, not a producer-only
  or shadow assertion.
- Every initial A/B finding is accepted and closed: both-layout proof,
  enabled-duplicate scope, canonical E006 payload, ordinary-error precedence,
  exact A-H bindings, bounded side-effect-free probing, and pinned citation
  truthfulness. None is rejected, deferred, or converted into follow-up scope.

### Safety-net chronology, metrics, and closure posture

Static/Ran:

- The recorded pre-decomposition safety net used production source hash
  `675ef55135e4f89d35f822cdbc836354a4215a6c10c5cf035cbefe51192635dc`
  and the current focused-test hash
  `7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`;
  38/38 passed at 99.511% lines and 99.346% regions before the CRAP-36 block
  was decomposed. Helper extraction therefore followed, rather than preceded,
  the required safety net.
- Live terminal source/test hashes match the recorded evidence:
  `a2b18016361731f8f568857de4210f5e207b03683744ff42f53e41323d206b1d`
  and `7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`.
  Raw JSON independently confirms 662/665 lines (99.549%), 793/798 regions
  (99.373%), 31/31 functions, and minimum named-function coverage 96.970%.
  LCOV-backed target CRAP has no row above 30; the maximum is
  `parse_watershed_channel_from_str` at 21.0005.
- Touched Rust files are 956, 932, and 1,250 lines, below both governance
  thresholds. No denominator exclusion, deferred A-H item, safety/security
  exception, or current-scope follow-up is used for acceptance. The unrelated
  preexisting contract gap register remains outside this defect's expressly
  bounded recognition/classification objective and is not a deferred package
  gate.

### Pending external gate

`artifacts/gate-results.md` remained `queued` / `not-run` at review time because
the root closure loop was running concurrently. This review does not represent
those commands as passed. Any failure of formatting, workspace Clippy,
full-profile nextest, deny, Markdown, or another required closure command must
block final package disposition until corrected and rerun. This is pending
execution evidence, not an accepted deferral or waiver.

### Final verdict

**FINAL GOVERNANCE/PROVENANCE PASS.** FQ-03 satisfies its correction envelope,
canonical and pinned authority, exact typed semantics, structural non-heuristic
recognition, real frame-consumer proof, A-H obligations, safety-net chronology,
coverage/floor/CRAP gates, line-count governance, and initial-finding
disposition. The only unavailable review evidence is the concurrently running
root closure-loop result; terminal package disposition must consume its recorded
PASS rather than infer it from this review.
