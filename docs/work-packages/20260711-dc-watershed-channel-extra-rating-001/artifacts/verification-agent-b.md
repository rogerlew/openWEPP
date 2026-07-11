# Verification agent B

Status: **PASS**
Evidence mode: Static and Ran as labeled
Verification date: 2026-07-11

## Verdict

**PASS.** FQ-03 has direct, current evidence for its correction authority,
typed semantics, focused and full validation, frame-consumer stability,
science-tier coverage, CRAP threshold, review disposition, and non-deferral
requirements. No blocking or advisory verification finding remains.

## Closure-gate log verification

Static/Ran:

- `gate-results.md` records PASS for the complete required loop:
  `cargo fmt --check`; workspace/all-target Clippy with warnings denied;
  full-profile workspace nextest; `cargo deny check`; scoped Markdown lint;
  and `git diff --check`.
- The full nextest log reports run
  `3d16d2bb-ad39-43ef-b37e-edb64778b023`, 1,765/1,765 passed, three skipped,
  171 binaries, and exit zero. The skips are explicitly counted and do not
  hide a failed current-scope test.
- Recomputed SHA-256 values exactly match `gate-results.md`:
  `cargo-fmt.log`
  `bb0d16c7a2bf23597420648cf208b5670787d1bc0e0f2278149e78118078964e`;
  `cargo-clippy.log`
  `ddf7620148c1b89b811a13510686ed5ab5e7372d86691dcdeccb4e18eb4a7aaa`;
  `cargo-nextest-full.log`
  `b61d320a7c89da8ae4fc0fb5c4d932b5df0a055613a5fe640b565cf410050ef2`;
  `cargo-deny.log`
  `e32d8d2a4a77942b5d58dd1384746881225c28b00f53e0fe317d95280ed084af`;
  `markdown-doc.log`
  `7e9f19b0bc2c25c66d36edec5f5e8e36ccccaacca64a1b6f3745e39caa005c1f`;
  and `git-diff-check.log`
  `01bee08d98cd0cf617c4c78eab914439e44338fde4e0363da1e8168531b387d5`.
- Production and focused-test mtimes precede the closure logs. Their live
  hashes still match terminal evidence, and a fresh combined focused run
  `2d83f85a-c82d-41d3-8f0f-c0ae22d904b7` passed all 58 parser and WSHED-W5
  tests with no skips.

## Authority, provenance, and semantic alignment

Static:

- Contract `0.1.2` hash
  `835facb44b2065f5c4505228d83d52200e8472e9826e54db75efe553850cdb0c`
  and spec `0.1.1` hash
  `2faf80f9285099711c8b0f169ff3d69ee16c252822a4abf7735a0424cf7e199f`
  match the package evidence and live files.
- Pinned legacy HEAD is exactly
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. The amended authority uses the
  exact direct anchors already independently reviewed: `inidat.for:1159` for
  `chnchk`; `wshinp.for:380-384`, `388-390`, `409-411`, `422-425`, and line
  425 for the named normalization/override/clamp operations; and `370-433`
  for the complete channel loop and conditional rating read. Searches found no
  stale exploratory legacy path or superseded challenged range.
- Contract, spec, code, and named tests agree that retained canonical suffix
  closure has priority; only one immediate full-domain rating candidate is
  examined after `icntrl != 4`; and E006 is emitted only when deleting that
  candidate uniquely closes all remaining declared channels plus EOF.
- `parse_channel_block` and `parse_rating_curve_line` are shared by ordinary
  parsing and probing. The implementation is therefore not a lexical
  “three floats means rating” classifier. Probe definitions/warnings are local,
  the real cursor/output is untouched, memoization is iterative and bounded by
  `(physical cursor, next channel_id)`, and no input is accepted by deletion.
- Exact precedence is preserved: two-/four-token and invalid-domain residuals
  remain E002; a valid retained numeric comment remains valid; neither-layout
  returns the ordinary retained E001 line/field/token unchanged; enabled-rating
  duplicates remain E002; and the sole recognized case returns the canonical
  E006 candidate line, disabled channel ID, exact reason, and no partial output.
  The fixed-arity proof soundly excludes simultaneous retained/deleted closure.

## Findings, obligations, and consumer proof

Static/Ran:

- `review-disposition.md` accepts and closes every initial A/B finding:
  both-layout proof, duplicate-enabled scope, exact E006 payload, ordinary
  precedence, exact obligations, bounded side-effect-free probe design, and
  pinned provenance. Final Reviews A and B both pass with no new finding; no
  finding is rejected, deferred, or moved to follow-up scope.
- The terminal A-H table binds exact test functions for every applicable
  family. A-F and H pass. G is correctly reviewed N/A for conservation because
  parsing/frame projection computes no conserved quantity, while its value-
  continuity concern is directly bound to the WSHED-W5 frame test.
- `watershed_channel_rating_projection_preserves_optional_fields` invokes the
  real unchanged `WatershedNetworkFrame::from_parsed_inputs` path.
  `build_channel_controls` consumes the parser rating record, and the test
  observes exact `1.25`, `1.50`, and `0.10` values for `icntrl=4` plus `None`
  for `icntrl=1`. This is downstream frame evidence, not a producer-only,
  skeleton, or shadow assertion.

## Safety-net chronology and terminal metrics

Static/Ran:

- Before decomposition, the expanded 38-test safety net passed against source
  hash `675ef55135e4f89d35f822cdbc836354a4215a6c10c5cf035cbefe51192635dc`
  at 99.511% lines, 99.346% regions, 28/28 functions, and minimum 96.970%.
  `parse_channel_block` was then the sole target row above CRAP 30 at 36.
  Only after that evidence was fixed did helper decomposition occur.
- Live terminal source/test hashes are
  `a2b18016361731f8f568857de4210f5e207b03683744ff42f53e41323d206b1d`
  and `7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`,
  matching recorded terminal evidence.
- Recomputed raw JSON metrics are 662/665 lines (99.549%), 793/798 regions
  (99.373%), 31/31 functions, and minimum named-function coverage 96.970%.
  Deduplicated LCOV-backed CRAP has 25 target rows, zero above 30, and maximum
  21.0005. No denominator exclusion was used.
- Touched Rust files are 956, 932, and 1,250 lines, below the 2,000-line warning
  and 3,000-line block thresholds.

## Non-deferral conclusion

All substantive package-scoped exit criteria audited by Verification B have
direct PASS evidence. There is no `FAIL`, `BLOCKED`, unjustified `NOT RUN`,
planned A-H item, accepted review deferral, safety/security exception, or
current-scope follow-up. The unrelated preexisting canonical gap register is
outside this package's explicitly bounded recognition/classification objective
and is not being used to waive a package gate. Verification B therefore
authorizes PASS; terminal disposition must still require the separately
assigned peer Verification A result rather than treating this artifact alone
as dual verification.
