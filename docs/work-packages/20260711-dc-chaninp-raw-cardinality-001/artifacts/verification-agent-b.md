# Verification agent B

Status: **PASS after reverification**
Evidence mode: Static and Ran

## Finding-by-finding verification

1. `B-FINAL-001` arity-before-token priority: **PASS**.
   Static: `parse_conditional_ichnum` compares raw expected cardinality with
   token count before `parse_ichnum_tokens`. The named regression
   `line4_wrong_arity_precedes_invalid_id_token_in_both_modes` asserts exact
   line `4`, field `line4`, expected `3`, found `2` in strict and compatibility
   modes, and receives no typed/defaulted output.
2. `A-FINAL-001 / B-FINAL-002` six non-finite vectors: **PASS for test
   implementation**. Static: `token_parse_and_nonfinite_failures_are_field_specific`
   covers `NaN`, positive infinity, and negative infinity independently for
   both `dtchr` and `cbase`, each as exact `CHN-E-003` field failure.
3. Exact named A-H binding: **FAIL**. The map is green but is not exact at the
   contract-obligation level. Family C uses the placeholder “strict/compat
   missing/open tests” instead of naming
   `strict_mode_missing_required_file_is_chn_e_009`,
   `compatibility_missing_file_defaults_with_chn_w_001`,
   `strict_mode_non_enoent_open_error_is_chn_e_000`, and
   `compatibility_non_enoent_open_error_collapses_with_chn_w_002`; it also does
   not name its topology-warning binding. Family D omits named count-domain
   bindings despite requiring invalid count/topology domains. Family H refers
   to an unnamed “raw 99+2 strict/compat loop” and does not explicitly bind the
   full B-F fail-closed set. Replace every shorthand/category reference with
   exact test-function names and bind each clause in the contract row.
4. `A-FINAL-002` count-consumer claim narrowing: **PASS**. Static: package,
   contract, and implementation evidence now claim only that
   `network_frame.rs` consumes `nchnum_norm`; they explicitly identify
   `ichnum_norm` as parser projection without a proved downstream list
   consumer. The WSHED-W5 test proves raw `99`, normalized count `2`, and frame
   `routing_globals.nchnum=2` without making a list-consumer readiness claim.
5. `B-FINAL-004` pinned citations: **PASS**. Static: the package cites
   `wshinp.for:467-475`, including raw implied-DO read line `470` and clamp
   lines `473-475`; `chnrt.for:773-774` proves post-clamp count consumption.
   Ran: pinned baseline HEAD is
   `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
6. `B-FINAL-005` touched Rust line counts: **PASS**. Static/current counts are
   `chaninp.rs` 1,018, parser integration test 931, and WSHED-W5 integration
   test 1,186. All are recorded and below the 2,000-line warning threshold.
7. `A-FINAL-003` cover-first provenance: **FAIL**. The accepted finding is not
   implemented. `coverage-before.md` still supplies no exact corrected-source
   hash, snapshot, commit/worktree identity, timing, or byte sizes. It also
   states that LCOV/CRAP preceded two tests while JSON is the final safety-net
   capture, without the promised labeled reconstruction. The preserved output
   hashes verify files, not which source was decomposed. Produce the requested
   contemporaneous identity or an explicit reproducible reconstruction; this
   cover-first gate cannot be inferred from terminal coverage.
8. Final artifact truthfulness: **FAIL**. `contract-and-provenance.md` labels
   its hashes final, but current contract/spec SHA-256 values are
   `55d0e9985f2d5610d9ce9ca38e32eecbd8a19db9d4741320289ee1e265a76783`
   and `23299cba5478c3ba8d29b78161c8efa5d5c4aa38d18e51026e7ae272f5fcd9d4`,
   not the recorded `3eb6...` and `1c63...`. Refresh the artifact after the
   claim-narrowing edits and reverify it.
9. Final coverage/CRAP and closure runs: **PASS as executed evidence**. Raw
   hashes and sizes match. Target JSON reports `687/741` lines (`92.713%`) and
   `738/763` regions (`96.723%`); deduplicated maximum target CRAP is `16.352`.
   Recorded gate-log hashes match; formatting, workspace clippy, full nextest
   (`1,747/1,747`, three skipped), deny, Markdown lint, and diff check exited
   zero. These passes do not waive findings 3, 7, or 8.

## Independent runs

- Ran: `cargo nextest run --test infile_chaninp_parser_contract` passed
  `36/36`, run ID `8e3e185f-9d85-466f-aff6-bff0e420cc11`.
- Ran: `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`
  passed `19/19`, run ID `f3f0b6ae-1e16-4870-8b20-3ff968084a26`.
- Ran: scoped `markdown-doc lint` passed 31 files with zero errors/warnings.
- Ran: `git diff --check` exited zero before this artifact edit.

## Verdict

**HOLD.** The semantic correction, narrowed consumer claim, current coverage/
CRAP, and heavy gates pass. Exact obligation binding, independently
attributable cover-first evidence, and current contract/spec hash truthfulness
remain unresolved current-package gates. They must be corrected and
independently reverified; none is a legitimate deferred boundary.

## REVERIFICATION — final HOLD fixes

Verdict: **PASS**.

Evidence mode: Static and Ran.

Finding closure:

1. Exact clause-to-named-test A-H map: **PASS**. Static: the completed map now
   names canonical invariant/guard clauses and exact Rust test functions for
   every applicable family. Family C names all four missing/open strict/compat
   functions plus applicability, normalization, and the WSHED-W5 count
   consumer. Family D binds `G-CHN-004` through `G-CHN-008` to named enum,
   timestep, baseflow, topology-ID, and negative-count tests. Families E/F/H
   bind exact arity-priority, six non-finite, runtime-guard, and non-collapse
   tests. Family G is explicitly reviewed N/A because no conserved quantity or
   continuous state exists in scope. Cross-family reuse does not leave a
   clause unbound; the named B-F tests collectively provide H fail-closed
   evidence.
2. Review-corrected monolith reconstruction: **PASS**. Static/Ran evidence is
   source-snapshotted and isolated from the terminal worktree. Raw JSON names
   `/tmp/openwepp-fq02-reconstruct/crates/openwepp-input-contract/src/parsers/chaninp.rs`;
   the retained snapshot contains one monolithic `parse_required_branch` and
   hashes to
   `63f700ff562fd9fb351ee9ce6cc95faf89db055d7981fb4561a5149b3f7f2dbd`
   (28,745 bytes, 965 lines). The exact terminal focused test hash is
   `bb7475b308acd1364e7c8037fc4495a321ec1de8d46abb78a0fdf4c62f620c9e`.
   Recorded reconstruction LCOV/JSON/CRAP hashes and sizes match; timing files
   record zero exits at 0.38/37.76/1.04/1.16 seconds. The reconstruction passed
   36/36 at 92.351% lines and 96.589% regions, while monolithic
   `parse_required_branch` remained an eligible CC 42 / CRAP 49.283 target.
   Current terminal workspace source remains
   `f7857a4cbd5a0bdb5f7ade1bf4e2d8871811988791f79dcb77fe5af33b59646d`,
   matching terminal coverage evidence; reconstruction did not replace it.
3. Canonical consumer wording: **PASS**. Static: contract Section 2.2,
   propagation and boundary maps, test obligations, spec, package, and
   implementation evidence consistently limit the real consumer proof to
   network-frame consumption of `nchnum_norm`. `ichnum_norm` is labeled a
   parser projection with no proved downstream ID-list consumer or readiness
   claim.
4. Contract/spec identities: **PASS**. Static current SHA-256 values exactly
   match `contract-and-provenance.md`: contract
   `da94093ff009be0e8ee618783c799d1ed70c0377b398e546fecd7e5c6c605be6`
   and spec
   `1d21069f186876faf19dfc4b2f300fdd17bad825cc54381d8c0c609b60c37ae2`.
5. Previously passing terminal evidence remains current: source/test hashes,
   all touched Rust line counts, 92.713% terminal lines, 96.723% terminal
   regions, CRAP maximum 16.352, and recorded delegated closure-gate log hashes
   all match their artifacts.

Independent Ran evidence:

- `cargo nextest run --test infile_chaninp_parser_contract`: PASS `36/36`, run
  `9a03902f-be41-4b10-8705-841520488c64`.
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`: PASS
  `19/19`, run `2ffebb76-0a0a-4c1a-b679-12e535f9f35b`.
- Scoped `markdown-doc lint`: PASS, 31 files, zero errors/warnings.
- `git diff --check`: PASS before this append.

Final Verification B disposition: **PASS**. All prior HOLD findings have direct
current-package closure evidence, and no current gate is deferred.
