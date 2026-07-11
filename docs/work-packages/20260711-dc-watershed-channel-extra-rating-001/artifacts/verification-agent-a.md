# Verification agent A

Status: **PASS**
Evidence mode: Static and Ran as labeled
Verification date: 2026-07-11

## Independent verdict

**PASS.** FQ-03 closes `CHN-E006-EXTRA-RATING-ROW` within its declared
correction envelope. Canonical authority, implementation, exact vectors, real
consumer projection, A-H obligations, safety-net chronology, terminal metrics,
full gates, reviews, and finding disposition agree. No required current-scope
gate is failed, blocked, waived, deferred, or represented only by planned
evidence.

## Exact identity verification

Ran: live SHA-256 values are:

- parser source:
  `a2b18016361731f8f568857de4210f5e207b03683744ff42f53e41323d206b1d`
- focused parser test:
  `7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`
- WSHED-W5 consumer test:
  `eb236cf2149c9c42bbf1c58f47245b6508f5c0c9b01b28ed226858e03256e425`
- contract:
  `835facb44b2065f5c4505228d83d52200e8472e9826e54db75efe553850cdb0c`
- spec:
  `2faf80f9285099711c8b0f169ff3d69ee16c252822a4abf7735a0424cf7e199f`

These match the terminal package evidence. The focused-test hash also matches
the pre-decomposition safety-net identity, proving no test was weakened during
the behavior-preserving helper extraction.

## Initial finding-by-finding verification

| Finding | Independent closure evidence | Result |
| --- | --- | --- |
| `A-001` / both-layout ambiguity | Contract fixed-arity proof is valid: the one-record offset maps retained two-token geometry to deleted-layout one-token `flgout` before an optional rating position. Retained-only, deleted-only, and neither are the executable partition. | PASS |
| `A-002` / enabled duplicate | `duplicate_rating_after_enabled_branch_remains_generic_extra_input` loops over both modes and asserts exact `RecordClosure { context: "extra_records", expected: 15, found: 16 }` / `CHN-E-002`. Enabled blocks bypass recognition in production. | PASS |
| `A-003` / exact E006 payload | Contract and code agree on `RatingCurveClosure`, candidate physical line, preceding disabled channel ID, and exact reason `icntrl!=4 prohibits structurally recognized rating_curve_line`; both final and multi-channel vectors assert the payload via `expect_err`. | PASS |
| `A-004` / neither precedence | `neither_suffix_layout_preserves_the_ordinary_retained_error` asserts unchanged line 18, field `ishape`, token `channel 2 comment c`, and `CHN-E-001` in both modes. Recognition uses probe cursors and returns to the untouched real cursor. | PASS |
| `A-005` / A-H readiness | Terminal obligation map names exact test functions for A-F and H; G is reviewed `N/A` for conservation and binds frame projection separately. Every row is terminal PASS or reviewed N/A; none remains planned/intended-red. | PASS |
| `A-006` / bounded side-effect-free design | Ordinary/probe paths share block/rating validators. The invocation-local memo is keyed by `(physical cursor, next channel_id)`, iteration is non-recursive, only retained and one immediate deletion are explored, and probe definitions/warnings are dropped. | PASS |
| `B-provenance` / pinned citations | Contract/spec hashes match; all amended legacy citations use pinned baseline `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. Corrected anchors bind `chnchk`, enum reads/overrides, roughness clamp, control override, and rating read to the actual lines. | PASS |
| `B-rereview-bindings` | D/H shorthand and unnamed placeholders are gone. Exact domain, malformed, non-finite, ambiguity, generic residual, and no-partial-output tests are enumerated. | PASS |
| `B-rereview-citations` | Review disposition records accepted corrections; direct inspection in both final reviews found no stale exploratory path or incorrect challenged verb/range. | PASS |

Both final reviews returned PASS with no new finding, and
`review-disposition.md` accepts and closes every row above. Nothing is rejected,
deferred, or converted into follow-up scope.

## `INV-CHN-016` implementation verification

Static:

1. The ordinary loop parses a channel, then calls recognition only when
   `icntrl != 4`.
2. Recognition calls `canonical_suffix_closes` on the retained cursor first;
   success returns immediately, protecting numeric rating-shaped comments.
3. The candidate is parsed with the same `parse_rating_curve_line` used by an
   enabled canonical block, including exact arity, numeric, finite, and domain
   guards.
4. Only the candidate parser's one-record cursor advance is passed to the
   deleted suffix probe. E006 requires retained false and deleted true through
   all remaining declared blocks plus blank-tolerant EOF.
5. Candidate failure or deleted-suffix failure returns `Ok(())`; the ordinary
   parser then produces its existing retained-layout result. Input is never
   accepted by deletion.
6. The E006 return is an error before any public file value is returned, so no
   typed partial output escapes.

The memo is correct for invocation-constant input/options/nchan: cursor plus
next channel ID uniquely identifies the semantic suffix. Caching a successful
tail is exact EOF evidence; caching a failed later tail also proves every
visited prefix cannot close. The state set is bounded by reachable retained and
single-deleted layouts and does not branch into repair combinations.

## Exact vectors, real consumer, and A-H

Static/Ran:

- Strict and compatibility vectors bind final two-/three-/four-token and
  invalid-domain residuals, multi-channel deleted-only closure, retained exact
  numeric comments and numeric-leading prose, neither closure, and enabled
  duplicate scope.
- Nominal parser assertions bind all channel scalar fields and exact optional
  rating values. Missing, malformed/cardinality, domain-boundary, non-finite,
  sidecar, version, truncation, trailing-blank, warning, and public-error
  surfaces close D/E/F and supporting A/H obligations.
- WSHED-W5 calls the real unchanged
  `WatershedNetworkFrame::from_parsed_inputs` path. Its consumer assertion
  observes exact `1.25`, `1.50`, `0.10` values for `icntrl=4` and `None` for
  `icntrl=1`; `network_frame.rs` has no diff.
- G conservation is legitimately `N/A`: the parser and frame projection do not
  calculate a conserved quantity. This does not waive output identity, which
  the real frame test proves.

Ran independently:

```text
cargo nextest run \
  --test infile_watershed_channel_parser_contract \
  --test wshedw5_typed_watershed_runtime_contract
Nextest run ID: af899e61-8344-4696-85a7-a107f2da065a
58 tests run: 58 passed, 0 skipped
```

## Safety chronology and terminal metrics

Static/Ran:

- Production correction first reached 26/26.
- Test-only expansion then fixed the pre-decomposition safety net at source
  `675ef55135e4f89d35f822cdbc836354a4215a6c10c5cf035cbefe51192635dc`
  and current test
  `7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`:
  38/38, 99.511% lines, 99.346% regions, all function floors above 75%.
- Only after that capture was `parse_channel_block` decomposed from CC/CRAP 36
  into ordered helpers.
- Raw terminal coverage JSON confirms 662/665 lines (`99.549%`), 793/798
  regions (`99.373%`), and 31/31 functions. Minimum named-function coverage is
  `96.970%`; there is no denominator exclusion.
- Raw LCOV-backed CRAP JSON contains zero target rows above 30. Maximum target
  CRAP is `21.0005`; `parse_channel_block` is 9, and extracted parameter, enum,
  and effective-control helpers are 15, 13, and 5.

Terminal evidence hashes independently match `coverage-after.md`: `lcov.info`
`01d9f5eae6386403df67a33d49587fc315a7567262fd1de590fdc2126bce07f5`,
coverage JSON
`4cee3b1d6e5152668285fb7885c83086e03874fdd82ab7655822757f85252bf6`,
and CRAP JSON
`8e611c2990963c6f0917e4e83ccaa00dcd951e9714e9217f07c8a34787c9111e`.
Safety-net raw hashes likewise match their recorded values.

## Full-gate log verification

Ran/Static: every recorded raw-log SHA-256 matches `gate-results.md`, and log
contents show successful commands:

- `cargo fmt --check`: exit 0.
- workspace/all-target Clippy with `-D warnings`: exit 0.
- full-profile nextest run `3d16d2bb-ad39-43ef-b37e-edb64778b023`:
  1,765/1,765 passed across 171 binaries; 3 configured skips; exit 0.
- `cargo deny check`: advisories, bans, licenses, and sources pass; exit 0.
- scoped Markdown: 33 files, 0 errors/warnings; exit 0.
- `git diff --check`: exit 0.

Verification A also reran `cargo fmt --check` and whole-worktree
`git diff --check`; both passed.

## Numeric, line-count, and security verification

Static:

- Ordered execution remains comments → enums → geometry → erodibility and
  `chnn >= chnnbr` guard → control → optional rating → effective-control
  derivation. No formula, threshold, arithmetic grouping, accepted grammar,
  public field, output meaning, or floating comparison changed in the
  decomposition.
- Touched Rust files are 956, 932, and 1,250 lines, below both the 2,000-line
  warning and 3,000-line closure thresholds.
- The parser adds no `unsafe`, command execution, network access,
  authentication/authorization, secret handling, or broadened trust boundary.
  Recognition fails closed and bounds candidate exploration as described
  above.

## No-deferred-current-gate audit

All package exit criteria have direct terminal evidence: authority,
implementation, exact errors, valid-comment preservation, real consumer
identity, A-H, coverage/floors, CRAP, numeric behavior, line count, security,
dual final review/disposition, and the full closure loop. There is no accepted
waiver, exclusion, follow-up, or unresolved finding. Verification B, final
disposition, and worker handoff remain normal subsequent closure steps; they
must be completed, but they are not substitutes for or deferrals of evidence
required by this Verification A verdict.

## Verification verdict

**PASS.** No discrepancy was found between canonical authority, live code,
tests, real consumer behavior, raw evidence, review disposition, or full gate
logs. FQ-03 is technically and evidentially ready for independent Verification
B and terminal package disposition.
